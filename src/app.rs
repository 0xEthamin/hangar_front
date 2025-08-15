use crate::
{
    components::nav::Nav,
    contexts::user_context::UserProvider,
    router::{switch, AppRoute},
};
use i18nrs::yew::{I18nProvider, I18nProviderConfig};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::window;

#[function_component(App)]
pub fn app() -> Html 
{

    let translations = HashMap::from(
    [
        (
            "en",
            r#"{
                "home": { "title": "Welcome", "login_button": "Login with Moodle", "description": "Easily host your projects with GarageISEP" },
                "nav": { "dashboard": "Dashboard", "logout": "Logout", "home": "Home" },
                "auth": { "logging_in": "Connecting, please wait...", "login_failed": "Authentication failed. Please try again." },
                "dashboard": { "welcome": "Welcome, {name}!", "description": "Here you will find information about your projects." },
                "ticket_missing": "Authentication ticket is missing. Please try logging in again."
            }"#,
        ),
        (
            "fr",
            r#"{
                "home": { "title": "Bienvenue", "login_button": "Connexion avec Moodle", "description": "Hébergez facilement vos projets avec GarageISEP" },
                "nav": { "dashboard": "Tableau de bord", "logout": "Déconnexion", "home": "Accueil" },
                "auth": { "logging_in": "Connexion en cours, veuillez patienter...", "login_failed": "L'authentification a échoué. Veuillez réessayer." },
                "dashboard": { "welcome": "Bienvenue, {name} !", "description": "Ici, vous trouverez des informations sur vos projets." },
                "ticket_missing": "Le ticket d'authentification est manquant. Veuillez retenter la connexion."
            }"#,
        ),
    ]);

    let default_language = window()
        .and_then(|w| w.navigator().language())
        .map(|lang| if lang.starts_with("fr") { "fr".to_string() } else { "en".to_string() })
        .unwrap_or_else(|| "en".to_string());
    

    let config = I18nProviderConfig 
    {
        translations,
        default_language,
        ..Default::default()
    };

    html! 
    {
        <I18nProvider ..config>
            <UserProvider>
                <BrowserRouter>
                    <Nav />
                    <main class="container">
                        <Switch<AppRoute> render={switch} />
                    </main>
                </BrowserRouter>
            </UserProvider>
        </I18nProvider>
    }
}