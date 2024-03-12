use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;
use db_manager::User;

#[derive(PartialEq, Clone)]
pub struct UserCtx {
    pub inner: Option<User>,
}

impl Reducible for UserCtx {
    type Action = Option<User>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        UserCtx { inner: action }.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct UserContextProviderProps {
    pub children: Children,
}

/// The UserContextProvider is a component which will be used
/// to act as a global storage of which user is logged into the system
/// at a given time. Additionally, it will provide a div at the bottom
/// of the screen which tells the user what account they are logged into,
/// and allows them to quickly log out.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &UserContextProviderProps) -> Html {
    let user = use_reducer(|| UserCtx { 
        inner: None 
    });

    let class = classes!(
        "flex", "flex-column",
        "justify-content-space-between",
        "height-100vh"
    );

    html!{
        <ContextProvider<UseReducerHandle<UserCtx>> context={user}>
            <div {class}>
                { props.children.clone() }
                <ContextBar />
            </div>
        </ContextProvider<UseReducerHandle<UserCtx>>>
    }
}

#[function_component(ContextBar)]
pub fn context_bar() -> Html {
    let class = classes!(
        "padding-1hrem", "background-color-bg-dark",
        "flex", "justify-content-space-between",
    );

    let ctx = use_context::<UseReducerHandle<UserCtx>>().expect("Couldn't Get User Context");
    let user = (*ctx).clone();
    let nav = use_navigator().expect("Couldn't get the nav handle");

    let onclick = {
        let ctx = ctx.clone();
        let nav = nav.clone();

        move |e: MouseEvent| {
            e.prevent_default();
            ctx.dispatch(None);
            nav.push(&Route::Login);
        }
    };

    html!{
        <div {class}>
            if let Some(user) = user.inner {
                <p>{ "Logged in as " }{ user.username() }</p>
                <a {onclick}>{ "Log out" }</a>
            } else {
                <p>{ "Not Logged In!" }</p>
                <Link<Route> to={Route::Login}>{ "Log in" }</Link<Route>>
            }
        </div>
    }
}