use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::app::Route;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let nav = use_navigator().expect("Could not get hook to navigator");

    let onclick = move |_| {
        nav.clone().push(&Route::Admin)
    };

    html!{
        <div class={classes!("padding-1rem")}>
            <h1 class={classes!("margin-bottom-1rem")}>{ "Dashboard" }</h1>
            <button {onclick}>{ "Go To Admin" }</button>
            <p>{ "Shows for everyone instead of just admins for now but it's not hard" }</p>
        </div>
    }
}