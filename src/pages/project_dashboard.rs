use std::pin::Pin;

use crate::
{
    contexts::user_context::use_user, models::project::Project, router::AppRoute,
    services::project_service,
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

#[function_component(ProjectDashboard)]
pub fn project_dashboard(props: &ProjectDashboardProps) -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();
    let navigator = use_navigator().unwrap();
    let project = use_state(|| None::<Project>);
    let error = use_state(|| None::<String>); 
    let deletion_error = use_state(|| None::<String>);

    let status = use_state(|| None::<String>);
    let is_controlling = use_state(|| false);

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
            fetch_status(); // Appel initial
            let interval = Interval::new(5000, fetch_status); // Toutes les 5 secondes
            
            move || drop(interval)
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

        html! 
        {
            <div>
                <h1>{ i18n.t("project_dashboard.title") }{ format!(": {}", p.name) }</h1>
                <div class="card">
                    <h2>{ i18n.t("project_dashboard.card_title_info") }</h2>
                    <p>{ "Status: " }
                        <span class={classes!("status-badge", status.as_ref().map(|s| format!("status-{}", s)).unwrap_or("status-unknown".to_string()))}>
                            { status.as_ref().cloned().unwrap_or_else(|| i18n.t("common.loading")) }
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
                        <h2>{ "Controls" }</h2>
                        <div style="display: flex; gap: var(--spacing-md);">
                            <button class="button-primary" onclick={on_start} disabled={*is_controlling || *status == Some("running".to_string())}>
                                { "Start" }
                            </button>
                             <button class="button-danger" onclick={on_stop} disabled={*is_controlling || *status != Some("running".to_string())}>
                                { "Stop" }
                            </button>
                             <button style="background-color: var(--color-primary-accent-hover)" class="button-primary" onclick={on_restart} disabled={*is_controlling || *status != Some("running".to_string())}>
                                { "Restart" }
                            </button>
                        </div>
                    </div>
                }

                <div class="card" style="margin-top: var(--spacing-lg);">
                    <h2>{ i18n.t("project_dashboard.card_title_logs") }</h2>
                    <pre style="background: #000; color: #0f0; padding: 1em; border-radius: 5px;">{ i18n.t("project_dashboard.logs_placeholder") }</pre>
                </div>

                <div class="card" style="margin-top: var(--spacing-lg);">
                    <h2>{ i18n.t("project_dashboard.card_title_metrics") }</h2>
                    <p>{ i18n.t("project_dashboard.metrics_placeholder") }</p>
                </div>

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