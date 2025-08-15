use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/auth/callback")]
    AuthCallback,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html 
{
    match route 
    {
        AppRoute::Home => html! { <pages::home::Home /> },
        AppRoute::AuthCallback => html! { <pages::auth_callback::AuthCallback /> },
        AppRoute::NotFound => html! { <pages::not_found::NotFound /> },
    }
}