use crate::contexts::user_context::use_user;
use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html 
{
    let user_context = use_user();
    let (i18n, _) = use_translation();

    let welcome_message = if let Some(user) = &user_context.user 
    {
        i18n.t("dashboard.welcome").replace("{name}", &user.login)
    } 
    else 
    {
        "Welcome!".to_string()
    };

    html! 
    {
        <div>
            <h1>{ i18n.t("nav.dashboard") }</h1>
            <div class="card">
                <h2>{ welcome_message }</h2>
                <p>{"C'est ici que vous trouverez les informations relatives à vos projets."}</p>
            </div>
        </div>
    }
}