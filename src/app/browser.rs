use crate::{
    app::Route,
    components::{
        inputs::Button,
        layout::{Column, Row},
        questions::use_question_sets,
        theme_ctx::use_theme,
        user_ctx::use_user,
    },
};
use shared::AccessLevel;
use stylist::yew::styled_component;
use tauri_sys::dialog::{MessageDialogBuilder, MessageDialogKind};
use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;

#[styled_component(Browser)]
pub fn browser() -> Html {
    let nav = use_navigator().unwrap();
    let user = use_user();
    let user = match (*user).clone().inner {
        Some(user) => user,
        None => {
            nav.push(&Route::Login);
            return html! {};
        }
    };

    let create_button = match user.access_level() {
        AccessLevel::USER => html! {},
        _ => {
            let onclick = move |_| nav.push(&Route::Creator);
            html! { <Button {onclick}>{ "+ Create" }</Button> }
        }
    };

    let theme = use_theme();
    let class = css!(
        r#"
            margin: ${fs};
            background-color: ${bg};

            h1 {
                font-size: calc( 1.5 * ${fs} );
                margin-right: calc( 0.5 * ${fs} );
            }

            p {
                margin-left: calc( 0.5 * ${fs} );
                flex: 1;
            }

            button {
                margin: calc( 0.25 * ${fs} );
            }
        "#,
        fs = theme.font_size,
        bg = theme.bg_color,
    );

    let mut alternate = true;
    let set_bars = match use_question_sets() {
        Ok(sets) => sets
            .clone()
            .unwrap()
            .into_iter()
            .map(|set| {
                let sc = set.clone();
                let class = if alternate {
                    css!("background-color: ${bg};", bg = theme.bg_shade)
                } else {
                    css!()
                };

                alternate = !alternate;

                match user.access_level() {
                    AccessLevel::USER => html! {
                        <Row {class} wfill={true} justify_content={"space-between"} align_items={"center"}>
                            <p><b>{ set.name() }</b></p>
                            <p>{ set.author() }</p>
                            <Button onclick={|_| {}}>{ "> Go!" }</Button>
                        </Row>
                    },
                    _ => {
                        let onclick = move |_| {
                            let set = set.clone();
                            log::info!("{:?}", set);
                            spawn_local(async move {
                                match crate::commands::invoke_delete_question_set(set.name().clone()).await {
                                    Ok(_) => {
                                        let _ = MessageDialogBuilder::new()
                                            .set_title("Question Browser")
                                            .set_kind(MessageDialogKind::Info)
                                            .message("The Question Set Deleted Successfully!")
                                            .await;
                                    }
                                    Err(why) => {
                                        let _ = MessageDialogBuilder::new()
                                            .set_title("Question Browser")
                                            .set_kind(MessageDialogKind::Info)
                                            .message(&why.message)
                                            .await;
                                    }
                                }
                            });
                        };
                        html! {
                            <Row {class} wfill={true} justify_content={"space-between"} align_items={"center"}>
                                <p><b>{ sc.name() }</b></p>
                                <p>{ sc.author() }</p>
                                <Button {onclick}>{ "Delete" }</Button>
                            </Row>
                        }
                    },
                }
            })
            .collect::<Html>(),
        Err(_) => html! { "Loading..." },
    };

    html! {
        <Column hfill={true} {class}>
            <Row wfill={true} justify_content={"center"} align_items={"center"}>
                <h1>{ "Question Browser" }</h1>
                {create_button}
            </Row>
            { set_bars }
        </Column>
    }
}
