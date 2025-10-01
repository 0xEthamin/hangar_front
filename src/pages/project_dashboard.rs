use std::pin::Pin;

use crate::
{
    components::gauge::Gauge, contexts::user_context::use_user, models::project::{Project, ProjectMetrics}, router::AppRoute, services::project_service::{self, ApiError}
};
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_timers::callback::Interval;

#[derive(Properties, PartialEq)]
pub struct ProjectDashboardProps 
{
    pub project_id: i32,
}

type LocalBoxFutureAction<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

fn render_log_line(line: &str) -> Html 
{
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let (timestamp, message) = if parts.len() == 2 && parts[0].ends_with('Z') 
    {
        (parts[0], parts[1].trim())
    } 
    else 
    {
        ("", line)
    };

    let message_upper = message.to_uppercase();
    let log_level_class = if message_upper.contains("ERROR") || message_upper.contains("FAILED") 
    {
        "log-error"
    } 
    else if message_upper.contains("WARN") || message_upper.contains("WARNING") 
    {
        "log-warn"
    } 
    else 
    {
        "log-info"
    };

    html! 
    {
        <div class="log-line">
            <span class="log-timestamp">{ timestamp.split('.').next().unwrap_or(timestamp) }</span>
            <span class={classes!("log-message", log_level_class)}>{ message }</span>
        </div>
    }
}

fn translate_status(status_str: &str, i18n: &i18nrs::I18n) -> String 
{
    let key = format!("common.status_{}", status_str);
    let translation = i18n.t(&key);

    if translation.starts_with("Key '")  && translation.contains(" not found for language ") 
    {
        i18n.t("common.status_unknown")
    } 
    else 
    {
        translation
    }
}

#[derive(Properties, PartialEq)]
struct UpdateImageFormProps 
{
    project_id: i32,
    project_name: String,
}

#[function_component(UpdateImageForm)]
fn update_image_form(props: &UpdateImageFormProps) -> Html 
{
    let (i18n, _) = use_translation();

    let new_image_url = use_state(String::new);
    let is_updating = use_state(|| false);
    let update_error = use_state(|| None::<ApiError>);

    let on_submit = 
    {
        let project_id = props.project_id;
        let project_name = props.project_name.clone();
        let new_image_url_state = new_image_url.clone();
        let is_updating = is_updating.clone();
        let update_error = update_error.clone();
        let i18n = i18n.clone();

        Callback::from(move |e: SubmitEvent| 
        {
            e.prevent_default();
            let confirm_message = i18n.t("project_dashboard.confirm_update_image")
                .replace("{name}", &project_name);

            if web_sys::window().unwrap().confirm_with_message(&confirm_message).unwrap() 
            {
                let new_image_url = (*new_image_url_state).clone();
                let is_updating_async = is_updating.clone();
                let update_error_async = update_error.clone();
                let new_image_url_state_async = new_image_url_state.clone();
            
                is_updating.set(true);
                update_error.set(None);

                wasm_bindgen_futures::spawn_local(async move 
                {
                    match project_service::update_project_image(project_id, &new_image_url).await 
                    {
                        Ok(_) => 
                        {
                            new_image_url_state_async.set(String::new());
                            is_updating_async.set(false);
                            web_sys::window().unwrap().location().reload().ok();
                        }
                        Err(api_error) => 
                        {
                            update_error_async.set(Some(api_error));
                            is_updating_async.set(false);
                        }
                    }
                });
            }
        })
    };

    html! 
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("project_dashboard.card_title_update_image") }</h2>
            <p style="color: var(--color-text-secondary); margin-bottom: var(--spacing-md);">
                { i18n.t("project_dashboard.update_image_description") }
            </p>
            <form onsubmit={on_submit}>
                <div class="form-group">
                    <label for="new_image_url">{ i18n.t("create_project.image_label") }</label>
                    <input type="text" id="new_image_url" class="text-input"
                           placeholder="mon-registre/mon-image:2.0"
                           value={(*new_image_url).clone()}
                           onchange=
                           {
                                let new_image_url = new_image_url.clone();
                                Callback::from(move |e: Event| 
                                {
                                    let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                    new_image_url.set(value);
                                })
                           }
                           required=true />
                </div>
                
                {
                    if let Some(err) = &*update_error 
                    {
                        let error_key = format!("errors.{}", err.error_code);
                        let message = i18n.t(&error_key);
                        let display_message = if message.starts_with("Key '") && message.contains(" not found for language ")
                        {
                            i18n.t("errors.DEFAULT")
                        } 
                        else 
                        {
                            message
                        };
                        html!{ <p class="error">{ display_message }</p> }
                    } 
                    else 
                    {
                        html!{}
                    }
                }

                <button type="submit" class="button-primary" disabled={*is_updating}>
                    { if *is_updating { i18n.t("project_dashboard.update_image_button_loading") } else { i18n.t("project_dashboard.update_image_button") } }
                </button>
            </form>
        </div>
    }
}

#[function_component(ProjectDashboard)]
pub fn project_dashboard(props: &ProjectDashboardProps) -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();
    let navigator = use_navigator().unwrap();
    let project = use_state(|| None::<Project>);
    let error = use_state(|| None::<String>); 
    let deletion_error = use_state(|| None::<String>);
    
    let logs = use_state(|| None::<String>);
    let logs_error = use_state(|| None::<String>);
    let are_logs_loading = use_state(|| false);

    let status = use_state(|| None::<String>);
    let is_controlling = use_state(|| false);

    let metrics = use_state(|| None::<ProjectMetrics>);
    let _metrics_interval = use_state(|| None::<Interval>);

    {
        let project = project.clone();
        let error = error.clone();
        let project_id = props.project_id;
        use_effect_with(project_id, move |_| 
        {
            wasm_bindgen_futures::spawn_local(async move 
            {
                match project_service::get_project_details(project_id).await 
                {
                    Ok(p) => project.set(Some(p)),
                    Err(e) => { error.set(Some(e)); }
                }
            });
            || ()
        });
    }

    {
        let status = status.clone();
        let project_id = props.project_id;
        use_effect_with(project_id, move |_| 
        {
            let fetch_status = move || 
            {
                let status = status.clone();
                wasm_bindgen_futures::spawn_local(async move 
                {
                    if let Ok(s) = project_service::get_project_status(project_id).await 
                    {
                        status.set(s);
                    }
                });
            };
            fetch_status();
            let interval = Interval::new(5000, fetch_status);
            
            move || drop(interval)
        });
    }

    {
        let metrics = metrics.clone();
        let _metrics_interval = _metrics_interval.clone();
        let project_id = props.project_id;
        
        use_effect_with(project_id, move |_| 
        {
            let fetch_metrics = move || 
            {
                let metrics = metrics.clone();
                wasm_bindgen_futures::spawn_local(async move 
                {
                    if let Ok(m) = project_service::get_project_metrics(project_id).await 
                    {
                        metrics.set(Some(m));
                    } 
                    else 
                    {
                        metrics.set(None);
                    }
                });
            };

            fetch_metrics();
            let interval = Interval::new(3000, fetch_metrics);
            _metrics_interval.set(Some(interval));

            || ()
        });
    }

    if let Some(e) = &*error 
    {
        let error_message = i18n.t("project_dashboard.load_error_message").replace("{error}", e);
        return html! 
        {
            <div class="card error">
                <h2>{ i18n.t("project_dashboard.access_error_title") }</h2>
                <p>{ error_message }</p>
                <Link<AppRoute> to={AppRoute::Home} classes="button-primary">
                    { i18n.t("common.back_to_home") }
                </Link<AppRoute>>
            </div>
        }
    }

    if let Some(p) = &*project 
    {
        let is_owner = user_context.user.as_ref().map_or(false, |u| u.login == p.owner);
        let created_at_formatted = p.created_at.split('T').next().unwrap_or("").to_string();
        let created_on_message = i18n.t("common.created_on").replace("{date}", &created_at_formatted);

        let create_control_callback = |action: fn(i32) -> LocalBoxFutureAction<Result<(), String>>| 
        {
            let is_controlling = is_controlling.clone();
            let status = status.clone();
            let project_id = props.project_id;
            Callback::from(move |_| 
            {
                let is_controlling = is_controlling.clone();
                let status = status.clone();
                is_controlling.set(true);
                wasm_bindgen_futures::spawn_local(async move 
                {
                    if action(project_id).await.is_ok() 
                    {
                        gloo_timers::callback::Timeout::new(1000, move || 
                        {
                            wasm_bindgen_futures::spawn_local(async move 
                            {
                                if let Ok(s) = project_service::get_project_status(project_id).await 
                                {
                                    status.set(s);
                                }
                            });
                        }).forget();
                    } 
                    else 
                    {
                        gloo_console::error!("Control action failed");
                    }
                    is_controlling.set(false);
                });
            })
        };

        let on_start = create_control_callback(|id| Box::pin(project_service::start_project(id)));
        let on_stop = create_control_callback(|id| Box::pin(project_service::stop_project(id)));
        let on_restart = create_control_callback(|id| Box::pin(project_service::restart_project(id)));

        let on_delete = 
        {
            let project_name = p.name.clone();
            let project_id = props.project_id;
            let navigator = navigator.clone();
            let i18n = i18n.clone();
            let deletion_error = deletion_error.clone();

            Callback::from(move |_| 
            {
                let confirm_message = i18n.t("project_dashboard.confirm_delete").replace("{name}", &project_name);
                if web_sys::window().unwrap().confirm_with_message(&confirm_message).unwrap()
                {
                    let navigator = navigator.clone();
                    let deletion_error = deletion_error.clone();
                    let i18n = i18n.clone();
                    wasm_bindgen_futures::spawn_local(async move 
                    {
                        if project_service::purge_project(project_id).await.is_ok() 
                        {
                            navigator.push(&AppRoute::Home);
                        } 
                        else 
                        {
                            deletion_error.set(Some(i18n.t("errors.DELETE_FAILED")));
                            gloo_console::error!("Failed to delete project");
                        }
                    });
                }
            })
        };

        let on_fetch_logs = 
        {
            let logs = logs.clone();
            let logs_error = logs_error.clone();
            let are_logs_loading = are_logs_loading.clone();
            let project_id = props.project_id;
            let i18n = i18n.clone();

            Callback::from(move |_| 
            {
                let logs = logs.clone();
                let logs_error = logs_error.clone();
                let are_logs_loading = are_logs_loading.clone();
                let i18n = i18n.clone();
                
                are_logs_loading.set(true);
                logs_error.set(None);
                
                wasm_bindgen_futures::spawn_local(async move 
                {
                    match project_service::get_project_logs(project_id).await 
                    {
                        Ok(log_data) => 
                        {
                            logs.set(Some(log_data));
                        },
                        Err(e) => 
                        {
                            gloo_console::error!("Failed to fetch logs:", e.clone());
                            let error_message = i18n.t("project_dashboard.logs_error").replace("{error}", &e);
                            logs_error.set(Some(error_message));
                            logs.set(None);
                        }
                    }
                    are_logs_loading.set(false);
                });
            })
        };

        html! 
        {
            <div>
                <h1>{ i18n.t("project_dashboard.title") }{ format!(": {}", p.name) }</h1>
                <div class="card">
                    <h2>{ i18n.t("project_dashboard.card_title_info") }</h2>
                    <p>{ i18n.t("common.status") }{ ": " }
                        <span class={classes!("status-badge", status.as_ref().map(|s| format!("status-{}", s)).unwrap_or("status-unknown".to_string()))}>
                            { status.as_ref().map(|s| translate_status(s, &i18n)).unwrap_or_else(|| i18n.t("common.loading")) }
                        </span>
                    </p>
                    <p>{ format!("{}: {}", i18n.t("common.owner"), p.owner) }</p>
                    if let Some(participants) = &p.participants 
                    {
                        if !participants.is_empty() 
                        {
                            <p>{ "Participants: " } { participants.join(", ") }</p>
                        }
                    }
                    <p>{ created_on_message }</p>
                    <p style="word-break: break-all;">{ format!("{}: {}", i18n.t("common.image"), p.image_url) }</p>
                </div>

                if is_owner 
                {
                     <div class="card" style="margin-top: var(--spacing-lg);">
                        <h2>{ i18n.t("project_dashboard.card_title_controls") }</h2>
                        <div style="display: flex; gap: var(--spacing-md);">
                            <button class="button-primary" onclick={on_start} disabled={*is_controlling || *status == Some("running".to_string())}>
                                { i18n.t("project_dashboard.start_button") }
                            </button>
                             <button class="button-danger" onclick={on_stop} disabled={*is_controlling || *status != Some("running".to_string())}>
                                { i18n.t("project_dashboard.stop_button") }
                            </button>
                             <button style="background-color: var(--color-primary-accent-hover)" class="button-primary" onclick={on_restart} disabled={*is_controlling || *status != Some("running".to_string())}>
                                { i18n.t("project_dashboard.restart_button") }
                            </button>
                        </div>
                    </div>
                }

                <div class="card" style="margin-top: var(--spacing-lg);">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--spacing-md);">
                        <h2>{ i18n.t("project_dashboard.card_title_logs") }</h2>
                        <button class="button-primary" onclick={on_fetch_logs} disabled={*are_logs_loading}>
                            { if *are_logs_loading { i18n.t("project_dashboard.fetch_logs_loading") } else { i18n.t("project_dashboard.fetch_logs_button") } }
                        </button>
                    </div>

                    <div class="logs-container">
                    {
                        if let Some(err_msg) = &*logs_error 
                        {
                             html! { <p class="error">{ err_msg }</p> }
                        }
                        else if let Some(log_data) = &*logs 
                        {
                            if log_data.is_empty() 
                            {
                                html!{ <div class="placeholder">{ i18n.t("project_dashboard.logs_empty") }</div> }
                            } 
                            else 
                            {
                                log_data.lines().map(render_log_line).collect::<Html>()
                            }
                        } 
                        else 
                        {
                            html!{ <div class="placeholder">{ i18n.t("project_dashboard.logs_placeholder") }</div> }
                        }
                    }
                    </div>
                </div>

                <div class="card" style="margin-top: var(--spacing-lg);">
                    <h2>{ i18n.t("project_dashboard.card_title_metrics") }</h2>
                    <div class="metrics-grid">
                    {
                        if let Some(m) = &*metrics 
                        {
                            html! 
                            {
                                <>
                                    <Gauge
                                        label="CPU"
                                        value={m.cpu_usage}
                                        max_value={100.0} // Le CPU est déjà en %
                                        unit="%"
                                    />
                                    <Gauge
                                        label="RAM"
                                        value={m.memory_usage}
                                        max_value={m.memory_limit}
                                        unit="MiB" // L'unité sera affichée en MiB
                                    />
                                </>
                            }
                        } 
                        else 
                        {
                            html! { <p>{ i18n.t("common.loading") }</p> }
                        }
                    }
                    </div>
                </div>

                if is_owner 
                {
                    <UpdateImageForm project_id={p.id} project_name={p.name.clone()} />
                }

                if is_owner 
                {
                    <div class="card" style="margin-top: var(--spacing-lg); border-color: var(--color-danger);">
                        <h2>{ i18n.t("project_dashboard.card_title_danger") }</h2>
                        {
                            if let Some(error_msg) = &*deletion_error 
                            {
                                html! { <p class="error" style="margin-top: var(--spacing-md)">{ error_msg }</p> }
                            } 
                            else 
                            {
                                html! {}
                            }
                        }
                        <button class="button-danger" onclick={on_delete}>
                            { i18n.t("project_dashboard.delete_button") }
                        </button>
                    </div>
                }
            </div>
        }
    } 
    else 
    {
        html! { <div class="loading-spinner">{ i18n.t("common.loading") }</div> }
    }
}