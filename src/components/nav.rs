use crate::contexts::user_context::use_user;
use crate::router::AppRoute;
use crate::services::auth_service;
use crate::components::language_switcher::LanguageSwitcher;
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let user_context = use_user();
    let navigator = use_navigator().unwrap();
    let (i18n, _) = use_translation();

    let is_menu_open = use_state(|| false);

    let on_logout = {
        let user_context = user_context.clone();
        Callback::from(move |_| {
            let user_context = user_context.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if auth_service::logout().await.is_ok() {
                    user_context.dispatch(None);
                    navigator.push(&AppRoute::Home);
                } else {
                    gloo_console::error!("Logout failed");
                }
            });
        })
    };
    
    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let nav_links_class = classes!("nav-links", is_menu_open.then_some("active"));

    html! {
        <nav>
            <Link<AppRoute> to={AppRoute::Home} classes="logo-link">
                <img src="/assets/logo-garageisep-white.svg" alt="GarageIsep Logo" />
            </Link<AppRoute>>
            
            <div class="nav-right-section">
                <div class="burger-menu" onclick={toggle_menu}>
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
                
                <ul class={nav_links_class}>
                    <li><Link<AppRoute> to={AppRoute::Home}>{ i18n.t("nav.home") }</Link<AppRoute>></li>
                    {
                        if user_context.user.is_some() 
                        {
                            html! 
                            {
                                <>
                                    <li><button class="button-danger" onclick={on_logout}>{ i18n.t("nav.logout") }</button></li>
                                </>
                            }
                        } 
                        else 
                        {
                            html!{}
                        }
                    }
                </ul>
                <LanguageSwitcher />
            </div>
        </nav>
    }
}