use crate::contexts::user_context::use_user;
use i18nrs::yew::use_translation;
use yew::prelude::*;

const CAS_LOGIN_URL: &str = "https://portail-ovh.isep.fr/cas/login";
const FRONTEND_CALLBACK_URL: &str = "http://127.0.0.1:8080/auth/callback";

#[function_component(Home)]
pub fn home() -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();

    if let Some(user) = &user_context.user 
    {
        let welcome_message = i18n.t("dashboard.welcome").replace("{name}", &user.name);

        html! 
        {
            <div>
                <h1>{ i18n.t("nav.dashboard") }</h1>
                <div class="card">
                    <h2>{ welcome_message }</h2>
                    <p>{ i18n.t("dashboard.description") }</p>
                </div>
            </div>
        }
    } 
    else 
    {
        let login_url = format!("{}?service={}", CAS_LOGIN_URL, FRONTEND_CALLBACK_URL);

        html! 
        {
            <div class="home-page" style="text-align: center; margin-top: 10vh; display: flex; flex-direction: column; align-items: center; gap: var(--spacing-lg);">
                <img src="/assets/logo-garageisep-white.svg" alt="GarageIsep Logo" style="height: 80px;" />
                <h1>{ i18n.t("home.title") }</h1>
                <p style="max-width: 500px;">{ i18n.t("home.description") }</p>
                <a href={login_url} class="button-gradient">{ i18n.t("home.login_button") }</a>
            </div>
        }
    }
}