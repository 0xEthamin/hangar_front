use crate::{models::user::User, services::auth_service};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UserState 
{
    pub user: Option<User>,
    pub loading: bool,
}

pub type UserContext = UseReducerHandle<UserState>;

#[derive(Properties, PartialEq)]
pub struct UserProviderProps 
{
    pub children: Children,
}


impl Reducible for UserState 
{
    type Action = Option<User>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> 
    {
        UserState 
        {
            user: action,
            loading: false,
        }.into()
    }
}

#[function_component(UserProvider)]
pub fn user_provider(props: &UserProviderProps) -> Html 
{
    let user_state = use_reducer(|| UserState 
    {
        user: None,
        loading: true,
    });


    {
        let user_state = user_state.clone();
        use_effect_with((), move |_| 
        {
            wasm_bindgen_futures::spawn_local(async move 
            {
                let current_user = auth_service::get_current_user().await.ok();
                user_state.dispatch(current_user);
            });
            || ()
        });
    }

    html! 
    {
        <ContextProvider<UserContext> context={user_state}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}

#[hook]
pub fn use_user() -> UserContext 
{
    use_context::<UserContext>().expect("use_user must be used within a UserProvider")
}