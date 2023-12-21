use log::info;
use yew::{prelude::*, platform::spawn_local};
use yew_router::prelude::*;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew_router::hooks::use_navigator;
use tauri_sys::{
    tauri::invoke,
    dialog::{
        MessageDialogBuilder,
        MessageDialogKind,
    },
    Error,
};
use db_manager::{
    requests::UserReqError,
    commands::ValidateLoginArgs,
    User,
};
use crate::{
    app::Route,
    components::{
        inputs::LengthValidationInput, 
        user_ctx::UserCtx
    },
};

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let navigator = use_navigator().expect("Couldn't find the navigator handle!");
    let user_ctx: UseReducerHandle<UserCtx> = use_context().expect("Couldn't find the user context!");

    let onchange_username = {
        let username = username.clone();
        move |e: Event| {
            username.set(
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .expect("Input Element Failed To Cast")
                    .value()
            );
        }
    };

    let onchange_password = {
        let password = password.clone();
        move |e: Event| {
            password.set(
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .expect("Input Element Failed To Cast")
                    .value()
            );
        }
    };

    let onclick = move |_| {

        let username = username.clone();
        let password = password.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        spawn_local(async move {

            let username = (*username).clone();
            let password = (*password).clone();

            let result: Result<User, _> = invoke("validate_login", &ValidateLoginArgs { username, password }).await;
            match result {
                Ok(user) => {
                    user_ctx.dispatch(Some(user));
                    navigator.push(&Route::Dashboard)
                },
                Err(why) => {
                    if let Error::Command(s) = why {
                        let why: UserReqError = s.into();
                        let dialog = MessageDialogBuilder::new()
                            .set_title("Login")
                            .set_kind(MessageDialogKind::Error)
                            .message(&why.message)
                            .await;

                        user_ctx.dispatch(None);
                    }
                }
            }
        });
    };

    spawn_local(async move {
        let _: () = invoke("debug_fetch_all", &()).await.unwrap();
    });

    html!{
        <div class={classes!("margin-1rem")}>
            <h1 class={classes!("margin-bottom-1rem")}>{ "Maths Login" }</h1>
            <LengthValidationInput class={classes!("margin-bottom-1rem")} onchange={onchange_username} id={"username"} min_length={3} max_length={20} required={true}>
                { "Username" }
            </LengthValidationInput>
            <LengthValidationInput input_type={"password"} class={classes!("margin-bottom-1rem")} onchange={onchange_password} id={"password"} min_length={8} max_length={20} required={true}>
                { "Password" }
            </LengthValidationInput>
            <button class={classes!("margin-bottom-1rem")} type="submit" {onclick}>{ "Login" }</button>
            <p>{"New User? "}<Link<Route> to={Route::Register}>{ "Register" }</Link<Route>></p>
        </div>
    }
}