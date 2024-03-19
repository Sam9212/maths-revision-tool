use crate::{
    app::Route,
    components::{
        inputs::{Button, ValidatedInput},
        layout::Column,
        theme_ctx::use_theme,
        user_ctx::use_user,
    },
};
use stylist::yew::styled_component;
use tauri_sys::{
    dialog::{MessageDialogBuilder, MessageDialogKind},
    tauri::invoke,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;
use yew_router::prelude::*;

#[styled_component(Login)]
pub fn login() -> Html {
    let username = use_state_eq(|| AttrValue::from(String::new()));
    let username_valid = use_state_eq(|| false);
    let password = use_state_eq(|| AttrValue::from(String::new()));
    let password_valid = use_state_eq(|| false);

    let nav = use_navigator().expect("Couldn't find the navigator handle!");
    let user_ctx = use_user();

    let onclick = {
        let username = username.clone();
        let password = password.clone();

        move |_| {
            let username = (*username).clone();
            let password = (*password).clone();
            let user_ctx = user_ctx.clone();
            let nav = nav.clone();

            spawn_local(async move {
                match crate::commands::invoke_validate_login(
                    username.to_string(),
                    password.to_string(),
                )
                .await
                {
                    Ok(user) => {
                        user_ctx.dispatch(Some(user));
                        nav.push(&Route::Dashboard);
                    }
                    Err(why) => {
                        let _ = MessageDialogBuilder::new()
                            .set_title("Login")
                            .set_kind(MessageDialogKind::Error)
                            .message(&why.message)
                            .await;

                        user_ctx.dispatch(None);
                    }
                }
            });
        }
    };

    spawn_local(async move {
        let _: () = invoke("debug_fetch_all", &()).await.unwrap();
    });

    let theme = use_theme();
    let class = css!(
        r#"
            background-color: ${bg};
            padding: ${fs};

            h1 {
                margin-bottom: calc( 0.5 * ${fs} );
            }
        "#,
        bg = theme.bg_color,
        fs = theme.font_size,
    );

    html! {
        <Column justify_content={"center"} align_items={"center"} wfill={true} hfill={true}>
            <Column justify_content={"center"} align_items={"center"} {class}>
                <h1>{ "Maths Login" }</h1>
                <ValidatedInput text_handle={username} validity_handle={username_valid.clone()} id={"username"}>{ "Username" }</ValidatedInput>
                <ValidatedInput text_handle={password} validity_handle={password_valid.clone()} id={"password"} minl={8} hidden={true}>{ "Password" }</ValidatedInput>
                <br />
                <Button {onclick} clickable={*username_valid && *password_valid}>{ "Login" }</Button>
                <p>{"New User? "}<Link<Route> to={Route::Register}>{ "Register" }</Link<Route>></p>
            </Column>
        </Column>
    }
}
