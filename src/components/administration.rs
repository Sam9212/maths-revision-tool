use crate::components::{
    inputs::{AccessPicker, Button, DateInput, UserPicker, ValidatedInput},
    layout::{Column, Row},
    theme_ctx::use_theme,
};
use chrono::Utc;
use shared::{AccessLevel, User};
use stylist::yew::styled_component;
use tauri_sys::dialog::*;
use yew::{platform::spawn_local, prelude::*};

#[styled_component(AddAccountForm)]
pub fn add_account_form() -> Html {
    // Store the values from the inputs in state vars so we can send
    // them to the backend later with a query
    let username = use_state_eq(|| AttrValue::from(String::new()));
    let username_valid = use_state_eq(|| false);
    let password = use_state_eq(|| AttrValue::from(String::new()));
    let password_valid = use_state_eq(|| false);
    let dob = use_state_eq(|| AttrValue::from(Utc::now().format("%Y-%m-%d").to_string()));
    let handle = use_state_eq(|| AccessLevel::USER);

    let onclick = {
        let username = username.clone();
        let password = password.clone();
        let dob = dob.clone();
        let handle = handle.clone();

        move |_| {
            let user = User::new(
                (*username).to_string(),
                (*password).to_string(),
                (*dob).to_string(),
                *handle,
            );
            spawn_local(async move {
                let (kind, message) = match crate::commands::invoke_add_user(user).await {
                    Ok(_) => (
                        MessageDialogKind::Info,
                        "account was added successfully".to_owned(),
                    ),
                    Err(why) => (MessageDialogKind::Error, why.message),
                };

                let _ = MessageDialogBuilder::new()
                    .set_title("Add Account")
                    .set_kind(kind)
                    .message(&message)
                    .await;
            });
        }
    };

    let theme = use_theme();
    let class = css!(
        r#"
            background-color: ${bgs};
            padding: calc( 0.5 * ${fs} );

            h1 {
                font-size: calc( 1.5 * ${fs} );
                text-align: center;
            }

            > div > * {
                margin-left: calc( 0.125 * ${fs});
                margin-right: calc( 0.125 * ${fs});
            }
        "#,
        bgs = theme.bg_shade,
        fs = theme.font_size,
    );

    html! {
        <Column {class} align_items={"center"}>
            <h1>{ "Add New Account" }</h1>
            <br />
            <Row>
                <ValidatedInput shaded={true} id={"username"} text_handle={username} validity_handle={username_valid}>{ "Username" }</ValidatedInput>
                <ValidatedInput shaded={true} hidden={true} minl={8} id={"password"} text_handle={password} validity_handle={password_valid}>{ "Password" }</ValidatedInput>
            </Row>
            <br />
            <Row>
                <DateInput shaded={true} id={"dob"} handle={dob}>{ "DOB" }</DateInput>
                <AccessPicker id={"access"} {handle}>{ "Access Level" }</AccessPicker>
            </Row>
            <br />
            <Button {onclick}>{ "Create" }</Button>
        </Column>
    }
}

#[styled_component(DeleteAccountForm)]
pub fn delete_account_form() -> Html {
    let to_delete = use_state_eq(|| AttrValue::from(String::new()));
    let dependency = use_state_eq(|| false);

    let onclick = {
        let to_delete = to_delete.clone();
        let dependency = dependency.clone();
        move |_| {
            let to_delete = to_delete.clone();
            let dependency = dependency.clone();
            spawn_local(async move {
                match crate::commands::invoke_delete_user(to_delete.to_string()).await {
                    Ok(_) => {
                        let _ = MessageDialogBuilder::new()
                            .set_title("Delete Account")
                            .set_kind(MessageDialogKind::Info)
                            .message("Deleting Account was Successful!!")
                            .await;
                    }
                    Err(why) => {
                        let _ = MessageDialogBuilder::new()
                            .set_title("Delete Account")
                            .set_kind(MessageDialogKind::Error)
                            .message(&why.message)
                            .await;
                    }
                }
                dependency.set(!*dependency);
            });
        }
    };

    let theme = use_theme();
    let class = css!(
        r#"
            background-color: ${bgs};
            padding: calc( 0.5 * ${fs} );

            > div > * {
                margin-left: calc( 0.125 * ${fs});
                margin-right: calc( 0.125 * ${fs});
            }
        "#,
        bgs = theme.bg_shade,
        fs = theme.font_size,
    );

    html! {
        <Column {class} align_items={"center"}>
            <h1>{ "Delete Existing Account" }</h1>
            <Row align_items={"center"}>
                <UserPicker {dependency} id={"deleter"} user_handle={to_delete}>{ "Pick a user" }</UserPicker>
                <Button {onclick}>{ "Delete" }</Button>
            </Row>
        </Column>
    }
}
