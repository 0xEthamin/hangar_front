use std::collections::HashSet;

use crate::{contexts::user_context::use_user, router::AppRoute, services::project_service::{self, ApiError}};
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(CreateProject)]
pub fn create_project() -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();
    let project_name = use_state(String::new);
    let image_url = use_state(String::new);
    let participants_str = use_state(String::new);
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<ApiError>);
    let navigator = use_navigator().unwrap();

    let on_submit = 
    {
        let project_name = project_name.clone();
        let image_url = image_url.clone();
        let participants_str = participants_str.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        let user_login = user_context.user.as_ref().map(|u| u.login.clone());

        Callback::from(move |e: SubmitEvent| 
        {
            e.prevent_default();
            let project_name = project_name.clone();
            let image_url = image_url.clone();
            let participants_str = participants_str.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let navigator = navigator.clone();
            let user_login = user_login.clone();

            let participants_set: HashSet<String> = (*participants_str)
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            if let Some(login) = &user_login 
            {
                if participants_set.contains(login) 
                {
                    error.set(Some(ApiError 
                    {
                        error_code: "OWNER_CANNOT_BE_PARTICIPANT".to_string(),
                        details: None,
                    }));
                    return;
                }
            }
            let participants: Vec<String> = participants_set.into_iter().collect();

            wasm_bindgen_futures::spawn_local(async move 
            {
                is_loading.set(true);
                error.set(None);

                let result = project_service::deploy_project(&project_name, &image_url, participants).await;
                is_loading.set(false);

                match result 
                {
                    Ok(project) => 
                    {
                        navigator.push(&AppRoute::ProjectDashboard { id: project.id });
                    }
                    Err(api_error) => 
                    {
                        error.set(Some(api_error));
                    }
                }
            });
        })
    };

    let handle_change = |state: UseStateHandle<String>| 
    {
        Callback::from(move |e: Event| 
        {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            state.set(value);
        })
    };

    html! 
    {
        <div class="create-project-page" style="max-width: 700px; margin: auto;">
            <h1>{ i18n.t("create_project.title") }</h1>
            <p>{ i18n.t("create_project.description") }</p>

            <form onsubmit={on_submit} class="card">
                
                <div class="form-group">
                    <label for="project_name">{ i18n.t("create_project.name_label") }</label>
                    <input type="text" id="project_name" class="text-input"
                           placeholder={i18n.t("create_project.name_placeholder")}
                           value={(*project_name).clone()}
                           onchange={handle_change(project_name.clone())}
                           required=true />
                    <small style="color: var(--color-text-secondary)">{ i18n.t("create_project.name_help") }</small>
                </div>

                <div class="form-group">
                    <label for="image_url">{ i18n.t("create_project.image_label") }</label>
                    <input type="text" id="image_url" class="text-input"
                           placeholder={i18n.t("create_project.image_placeholder")}
                           value={(*image_url).clone()}
                           onchange={handle_change(image_url.clone())}
                           required=true />
                </div>

                <div class="form-group">
                    <label for="participants">{ i18n.t("create_project.participants_label") }</label>
                    <input type="text" id="participants" class="text-input"
                           placeholder={i18n.t("create_project.participants_placeholder")}
                           value={(*participants_str).clone()}
                           onchange={handle_change(participants_str.clone())} />
                     <small style="color: var(--color-text-secondary)">{ i18n.t("create_project.participants_help") }</small>
                </div>

                {
                    if let Some(err) = &*error 
                    {
                        let error_key = format!("errors.{}", err.error_code);
                        let main_message = i18n.t(&error_key);
                        let display_message = if main_message.starts_with("Key '") && main_message.contains(" not found for language ") 
                        {
                            i18n.t("errors.DEFAULT")
                        } 
                        else 
                        {
                            main_message
                        };

                        html! 
                        {
                            <>
                                <p class="error">{ display_message }</p>
                                {
                                    if err.error_code == "IMAGE_SCAN_FAILED" 
                                    {
                                        if let Some(details) = &err.details 
                                        {
                                            html! 
                                            {
                                                <div class="error-details-box">
                                                    <strong>{ "Grype Security Report:" }</strong>
                                                    <pre><code>{ details.clone() }</code></pre>
                                                </div>
                                            }
                                        } 
                                        else 
                                        {
                                            html! {}
                                        }
                                    } 
                                    else 
                                    {
                                        html! {}
                                    }
                                }
                            </>
                        }
                    } 
                    else 
                    {
                        html! {}
                    }
                }

                <button type="submit" class="button-primary" disabled={*is_loading}>
                    { if *is_loading { i18n.t("create_project.submit_button_loading") } else { i18n.t("create_project.submit_button") } }
                </button>
            </form>
        </div>
    }
}