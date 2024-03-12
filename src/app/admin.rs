use chrono::Utc;
use db_manager::{
    commands::{AddUserArgs, DeleteUserArgs},
    requests::UserReqError,
    AccessLevel, User,
};
use stylist::yew::styled_component;
use tauri_sys::{
    dialog::{MessageDialogBuilder, MessageDialogKind},
    tauri::invoke,
    Error,
};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, HtmlSelectElement};
// use db_manager::AccessLevel;
use crate::components::{
    inputs::{DateInput, Dropdown, LengthValidationInput, UserPicker},
    tabs::{Tab, TabController},
    theme_ctx::use_theme,
};
use yew::{platform::spawn_local, prelude::*};
// use yew_router::hooks::{use_navigator, use_route};
// use crate::{components::user_ctx::UserCtx, app::Route};

#[styled_component(Admin)]
pub fn admin() -> Html {
    // Grab the theme from the theme CTX manager (ThemeProvider).
    let theme = use_theme();
    let class = css!(
        r#"
            padding: 16px;

            > * h1 {
                text-align: center;
                font-size: calc(2 * ${fs});
            }
        "#,
        fs = theme.font_size,
    );

    // Store the values from the inputs in state vars so we can send them to the
    // backend later in a query
    let username = use_state(|| String::new());
    let onchange_username = {
        let username = username.clone();

        move |e: Event| {
            let value = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .expect("Input Element Failed To Cast")
                .value();
            log::info!("{}", value);
            username.set(value);
        }
    };

    let password = use_state(|| String::new());
    let onchange_password = {
        let password = password.clone();

        move |e: Event| {
            password.set(
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .expect("Input Element Failed To Cast")
                    .value(),
            );
        }
    };

    let dob = use_state(|| Utc::now().format("%Y-%m-%d").to_string());
    let onchange_dob = {
        let dob = dob.clone();

        move |e: Event| {
            dob.set(
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .expect("Input Element Failed To Cast")
                    .value(),
            );
        }
    };

    let selected = use_state(|| AccessLevel::USER);

    let onchange = {
        let selected = selected.clone();
        move |e: Event| {
            let v = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                .expect("Input Element Failed To Cast")
                .value();

            let v = match &v[..] {
                "User" => AccessLevel::USER,
                "Teacher" => AccessLevel::TEACHER,
                "Admin" => AccessLevel::ADMIN,
                _ => {
                    unreachable!()
                }
            };

            selected.set(v);
        }
    };

    let onclick = {
        let username = username.clone();
        let password = password.clone();
        let dob = dob.clone();
        let selected = selected.clone();

        move |_| {
            let user = User::new(
                (*username).clone(),
                (*password).clone(),
                (*dob).clone(),
                *selected,
            );
            spawn_local(async move {
                let result: Result<(), _> =
                    invoke("add_user", &AddUserArgs { newUser: user }).await;
                match result {
                    Ok(_) => {
                        let _ = MessageDialogBuilder::new()
                            .set_title("Add Account")
                            .set_kind(MessageDialogKind::Info)
                            .message("Adding Account was Successful!!")
                            .await;
                    }
                    Err(why) => {
                        if let Error::Command(s) = why {
                            let why: UserReqError = s.into();
                            let _ = MessageDialogBuilder::new()
                                .set_title("Add Account")
                                .set_kind(MessageDialogKind::Error)
                                .message(&why.message)
                                .await;
                        }
                    }
                }
            });
        }
    };

    let to_delete = use_state(|| String::new());
    let onchange_deleter = {
        let to_delete = to_delete.clone();
        move |e: Event| {
            let v = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                .expect("Input Element Failed To Cast")
                .value();

            to_delete.set(v);
        }
    };

    let delacc = {
        let to_delete = to_delete.clone();
        move |_| {
            let to_delete = (*to_delete).clone();
            spawn_local(async move {
                let result: Result<(), _> = invoke(
                    "delete_user",
                    &DeleteUserArgs {
                        username: to_delete,
                    },
                )
                .await;

                match result {
                    Ok(_) => {
                        let _ = MessageDialogBuilder::new()
                            .set_title("Delete Account")
                            .set_kind(MessageDialogKind::Info)
                            .message("Deleting Account was Successful!!")
                            .await;
                    }
                    Err(why) => {
                        if let Error::Command(s) = why {
                            let why: UserReqError = s.into();
                            let _ = MessageDialogBuilder::new()
                                .set_title("Delete Account")
                                .set_kind(MessageDialogKind::Error)
                                .message(&why.message)
                                .await;
                        }
                    }
                }
            });
        }
    };

    html! {
        <div {class}>
            <h1 class={classes!("margin-bottom-1rem")}>{ "Admin" }</h1>
            <TabController name={"add-edit-delete"}>
                <Tab _id={"Add"}>
                    <h1>{ "Add New Account" }</h1>
                    <LengthValidationInput min_length={3} max_length={20} required={true} id={"username"} onchange={onchange_username}>{ "Username" }</LengthValidationInput>
                    <LengthValidationInput input_type={"password"} min_length={8} max_length={20} required={true} id={"password"} onchange={onchange_password}>{ "Password" }</LengthValidationInput>
                    <DateInput onchange={onchange_dob} id={"dob"}>{ "DOB" }</DateInput>
                    <Dropdown id={"access"} {onchange}>{ "Select Access" }</Dropdown>
                    <button {onclick}>{ "Create" }</button>
                </Tab>

                <Tab _id={"Delete"}>
                    <h1>{ "Delete Existing Account" }</h1>
                    <UserPicker id={"deleter"} onchange={onchange_deleter}>{ "Pick a user" }</UserPicker>
                    <button onclick={delacc}>{ "Delete" }</button>
                </Tab>
            </TabController>
        </div>
    }
}
