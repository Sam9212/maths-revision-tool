use crate::{
    app::Route,
    components::inputs::{DateInput, LengthValidationInput},
};
use chrono::Utc;
use db_manager::{commands::AddUserArgs, requests::UserReqError, AccessLevel, User};
use tauri_sys::{
    dialog::{MessageDialogBuilder, MessageDialogKind},
    tauri::invoke,
    Error,
};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;

#[function_component(Register)]
pub fn register() -> Html {
    let username = use_state_eq(|| String::new());

    let password = use_state_eq(|| String::new());

    let confirmation = use_state_eq(|| String::new());
    let confirmation_valid = use_state_eq(|| true);
    let confirmation_reason = use_state_eq(|| String::new());

    let date = use_state_eq(|| Utc::now().format("%Y-%m-%d").to_string());

    let nav = use_navigator().unwrap();

    let onchange_username = {
        let username = username.clone();
        move |e: Event| {
            username.set(
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .expect("Input Element Failed To Cast")
                    .value(),
            );
        }
    };

    let onchange_password = {
        let password = password.clone();
        let confirmation = confirmation.clone();
        let confirmation_valid = confirmation_valid.clone();
        let confirmation_reason = confirmation_reason.clone();

        move |e: Event| {
            let new_val = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .expect("Input Element Failed To Cast")
                .value();

            let validity = new_val == *confirmation;
            confirmation_valid.set(validity);
            confirmation_reason.set(if !validity {
                "Passwords don't match!".to_string()
            } else {
                String::new()
            });
            password.set(new_val);
        }
    };

    let onchange_confirmation = {
        let password = password.clone();
        let confirmation = confirmation.clone();
        let confirmation_valid = confirmation_valid.clone();
        let confirmation_reason = confirmation_reason.clone();

        move |e: Event| {
            let new_val = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .expect("Input Element Failed To Cast")
                .value();

            // info!("*password = {}, new_val = {}", *password, new_val);
            // info!("*password == new_val = {}", *password == new_val);

            let validity = *password == new_val;
            confirmation_reason.set(if !validity {
                "Passwords don't match!".to_string()
            } else {
                String::new()
            });
            confirmation_valid.set(validity);
            confirmation.set(new_val);
        }
    };

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
                username.clone(),
                password.clone(),
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

    html! {
        // Every page needs this outer div. It does not necessarily need any classes,
        // however I find it appropriate to add some spacing around the edges of the
        // screen. While Yew does allow me to have more than 1 root
        // element in an `html! macro, I still need to nest other components inside
        // of this 1 root component so that the flex spacing of the UserContext bar
        // does not mess up the layout.
        <div class={classes!("margin-1rem")}>
            <h1 class={classes!("margin-bottom-1rem")}>{ "Register" }</h1>
            <LengthValidationInput onchange={onchange_username} class={classes!("margin-bottom-1qrem")} id={"username"} min_length={3} max_length={20} required={true}>
                { "Username" }
            </LengthValidationInput>
            <p style={"font-size: 0.75rem;"} class={classes!("margin-bottom-1rem")}>{ "*A valid username is between 3 and 20 characters" }</p>

            <LengthValidationInput input_type={"password"} onchange={onchange_password} class={classes!("margin-bottom-1qrem")} id={"password"} min_length={8} max_length={20} required={true}>
                { "Password" }
            </LengthValidationInput>
            <p style={"font-size: 0.75rem;"} class={classes!("margin-bottom-1rem")}>{ "*A valid password is between 8 and 20 characters" }</p>

            <LengthValidationInput valid={confirmation_valid} valid_reason={confirmation_reason} input_type={"password"} onchange={onchange_confirmation} class={classes!("margin-bottom-1rem")} id={"confirmation"} min_length={8} max_length={20} required={true}>
                { "Confirm Password" }
            </LengthValidationInput>

            <DateInput onchange={onchange_date} class={classes!("margin-bottom-1rem")} id={"dob"}>
                { "Enter Your DOB" }
            </DateInput>

            <button {onclick}>
                { "Register" }
            </button>
        </div>
    }
}
