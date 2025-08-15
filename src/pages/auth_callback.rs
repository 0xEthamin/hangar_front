use crate::{contexts::user_context::use_user, router::AppRoute, services::auth_service};
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AuthCallback)]
pub fn auth_callback() -> Html 
{
    let user_context = use_user();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let (i18n, _) = use_translation();
    let error_message = use_state(|| None::<String>);

    use_effect_with((),
    {
        let user_context = user_context.clone();
        let navigator = navigator.clone();
        let error_message = error_message.clone();
        let i18n = i18n.clone();
        
        move |_| 
        {
            if let Some(ticket) = location.query_str()
                .trim_start_matches('?')
                .split('&')
                .find_map(|p| p.strip_prefix("ticket=")) 
            {
                let ticket = ticket.to_string();
                wasm_bindgen_futures::spawn_local(async move 
                {
                    match auth_service::validate_ticket(&ticket).await 
                    {
                        Ok(user) => 
                        {
                            user_context.dispatch(Some(user));
                            navigator.push(&AppRoute::Home);
                        }
                        Err(e) => 
                        {
                            log::error!("Login failed: {}", e);
                            error_message.set(Some(i18n.t("auth.login_failed")));
                        }
                    }
                });
            } 
            else 
            {
                 error_message.set(Some(i18n.t("auth.ticket_missing")));
            }
            || ()
        }
    });

    if let Some(error) = &*error_message 
    {
        html! { <p class="error">{error.clone()}</p> }
    } 
    else 
    {
        html! { <p>{ i18n.t("auth.logging_in") }</p> }
    }
}