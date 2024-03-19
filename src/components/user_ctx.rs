#![allow(non_camel_case_types)]

use crate::{app::Route, components::theme_ctx::use_theme};
use shared::User;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

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

/// The UserContextProvider is a component which will be used
/// to act as a global storage of which user is logged into the system
/// at a given time. Additionally, it will provide a div at the bottom
/// of the screen which tells the user what account they are logged into,
/// and allows them to quickly log out.
#[autoprops]
#[styled_component(UserContextProvider)]
pub fn user_context_provider(children: &Children) -> Html {
    let user = use_reducer(|| UserCtx { inner: None });

    let class = css!(
        r#"
            display: flex;
            flex-flow: column nowrap;
            justify-content: space-between;
            height: 100vh;
        "#,
    );

    html! {
        <ContextProvider<UseReducerHandle<UserCtx>> context={user}>
            <div {class}>
                { children }
                <ContextBar />
            </div>
        </ContextProvider<UseReducerHandle<UserCtx>>>
    }
}

#[styled_component(ContextBar)]
pub fn context_bar() -> Html {
    let theme = use_theme();
    let class = css!(
        r#"
            padding: calc( 0.5 * ${fs} );
            background-color: ${bg};
            display: flex;
            justify-content: space-between;
        "#,
        fs = theme.font_size,
        bg = theme.bg_color,
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

    html! {
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

#[hook]
pub fn use_user() -> UseReducerHandle<UserCtx> {
    use_context().expect("couldn't get user context")
}
