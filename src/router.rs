use crate::{components::protected_route::ProtectedRoute, pages::{self, admin, create_project, project_dashboard}};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute 
{
    #[at("/")]
    Home,
    #[at("/auth/callback")]
    AuthCallback,
    #[at("/projects/create")]
    CreateProject,
    #[at("/projects/:id")]
    ProjectDashboard { id: i32 },
    #[at("/admin")]
    Admin,
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
        AppRoute::CreateProject => html! 
        {
            <ProtectedRoute>
                <create_project::CreateProject />
            </ProtectedRoute>
        },
        AppRoute::ProjectDashboard { id } => html! 
        {
            <ProtectedRoute>
                <project_dashboard::ProjectDashboard project_id={id} />
            </ProtectedRoute>
        },
        AppRoute::Admin => html! 
        {
            <ProtectedRoute>
                <admin::Admin />
            </ProtectedRoute>
        },
        AppRoute::NotFound => html! { <pages::not_found::NotFound /> },
    }
}
