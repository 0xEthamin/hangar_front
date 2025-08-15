use crate::{contexts::user_context::use_user, router::AppRoute};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProtectedRouteProps 
{
    pub children: Children,
}

#[function_component(ProtectedRoute)]
pub fn protected_route(props: &ProtectedRouteProps) -> Html 
{
    let user_context = use_user();
    let navigator = use_navigator().unwrap();

    if user_context.loading 
    {
        html! { <div>{"Loading..."}</div> }
    } 
    else if user_context.user.is_some() 
    {
        html! { for props.children.iter() }
    } 
    else 
    {
        navigator.push(&AppRoute::Home);
        html! {}
    }
}