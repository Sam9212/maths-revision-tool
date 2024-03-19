use crate::{
    app::Route,
    components::{
        inputs::{Button, DateInput, ValidatedInput},
        layout::Column,
        theme_ctx::use_theme,
    },
};
use chrono::Utc;
use shared::{commands::AddUserArgs, requests::UserReqError, AccessLevel, User};
use stylist::yew::styled_component;
use tauri_sys::{
    dialog::{MessageDialogBuilder, MessageDialogKind},
    tauri::invoke,
    Error,
};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::{components::Link, hooks::use_navigator};

#[styled_component(Register)]
pub fn register() -> Html {
    let username = use_state_eq(|| AttrValue::from(String::new()));
    let username_valid = use_state_eq(|| false);

    let password = use_state_eq(|| AttrValue::from(String::new()));
    let password_valid = use_state_eq(|| false);

    let confirmation = use_state_eq(|| AttrValue::from(String::new()));
    let confirmation_valid = use_state_eq(|| false);
    let confirmation_verif = use_state_eq(|| false);

    let date = use_state_eq(|| Utc::now().format("%Y-%m-%d").to_string());

    let nav = use_navigator().unwrap();

    let onchange_date = {
        let date = date.clone();
        move |e: Event| {
            date.set(
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .expect("Input Element Failed to Cast")
                    .value(),
            )
        }
    };

    let onclick = {
        let username = (*username).clone();
        let password = (*password).clone();
        let confirmation = (*confirmation).clone();
        let date = (*date).clone();
        let nav = nav.clone();

        move |_| {
            if confirmation != password {
                return;
            }

            let user = User::new(
                username.clone().to_string(),
                password.clone().to_string(),
                date.clone(),
                AccessLevel::USER,
            );
            spawn_local(async move {
                let res: Result<(), _> = invoke("add_user", &AddUserArgs { newUser: user }).await;
                match res {
                    Ok(_) => {}
                    Err(why) => {
                        if let Error::Command(s) = why {
                            let why: UserReqError = s.into();
                            let _ = MessageDialogBuilder::new()
                                .set_title("Registration")
                                .set_kind(MessageDialogKind::Error)
                                .message(&why.message)
                                .await;
                        }
                    }
                }
            });
            nav.push(&Route::Login);
        }
    };

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
        // Every page needs this outer div. It does not necessarily need any classes,
        // however I find it appropriate to add some spacing around the edges of the
        // screen. While Yew does allow me to have more than 1 root
        // element in an `html! macro, I still need to nest other components inside
        // of this 1 root component so that the flex spacing of the UserContext bar
        // does not mess up the layout.
        <Column justify_content={"center"} align_items={"center"} wfill={true} hfill={true}>
            <Column justify_content={"center"} align_items={"center"} {class}>
                <h1>{ "Registration" }</h1>
                <ValidatedInput text_handle={username} validity_handle={username_valid.clone()} id={"username"}>{ "Username" }</ValidatedInput>
                <ValidatedInput text_handle={password.clone()} validity_handle={password_valid.clone()} id={"password"} minl={8} hidden={true}>{ "Password" }</ValidatedInput>
                <ValidatedInput text_handle={confirmation} secondary_handle={Some(password)} verif_handle={confirmation_verif.clone()} validity_handle={confirmation_valid.clone()} id={"confirmation"} minl={8} hidden={true}>{ "Confirm Password" }</ValidatedInput>
                <DateInput onchange={onchange_date} class={classes!("margin-bottom-1rem")} id={"dob"}>
                    { "Enter Your DOB" }
                </DateInput>
                <br />
                <Button {onclick} clickable={*username_valid && *password_valid && *confirmation_valid && *confirmation_verif}>{ "Register" }</Button>
                <p>{"Returning User? "}<Link<Route> to={Route::Login}>{ "Login" }</Link<Route>></p>
            </Column>
        </Column>
    }
}
