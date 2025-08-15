use yew::prelude::*;
use crate::router::AppRoute;
use yew_router::prelude::Link;

#[function_component(NotFound)]
pub fn not_found() -> Html 
{
    html! 
    {
        <div>
            <h1>{ "404 - Not Found" }</h1>
            <p>{ "The page you are looking for does not exist." }</p>
            <Link<AppRoute> to={AppRoute::Home}>{ "Go to Home" }</Link<AppRoute>>
        </div>
    }
}