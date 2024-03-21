use crate::{
    app::Route,
    components::{
        inputs::Button,
        layout::{Column, Row},
        questions::use_question_sets_with,
        theme_ctx::use_theme,
        user_ctx::use_user,
    },
};
use shared::AccessLevel;
use stylist::yew::styled_component;
use tauri_sys::dialog::{MessageDialogBuilder, MessageDialogKind};
use yew::{platform::spawn_local, prelude::*};
use yew_router::{components::Redirect, hooks::use_navigator};

#[styled_component(Browser)]
pub fn browser() -> Html {
    let nav = use_navigator().unwrap();
    let user = use_user();
    let user = match (*user).clone().inner {
        Some(user) => user,
        None => {
            return html! { <Redirect<Route> to={Route::Login}/>};
        }
    };

    let create_button = match user.access_level() {
        AccessLevel::USER => html! {},
        _ => {
            let nav = nav.clone();
            let onclick = move |_| nav.push(&Route::Creator);
            html! { <Button {onclick}>{ "+ New" }</Button> }
        }
    };

    let theme = use_theme();
    let class = css!(
        r#"
            margin: ${fs};
            background-color: ${bg};

            > div > div {
                display: inline-flex;
                justify-content: center;
                flex: 1.5;
            }

            > div > div:first-child {
                justify-content: start;
                flex: 1;
            }

            > div > div:last-child {
                justify-content: end;
                flex: 1;
            }

            h1 {
                font-size: calc( 1.5 * ${fs} );
                margin-right: calc( 0.5 * ${fs} );
                text-align: center;
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
    let dependency = use_state(|| false);
    let thingy = use_question_sets_with(dependency.clone());
    let navc = nav.clone();
    let set_bars = match thingy {
        Ok(sets) => sets
            .clone()
            .unwrap()
            .into_iter()
            .map(move |set| {
                let sc = set.clone();
                let class = if alternate {
                    css!("background-color: ${bg};", bg = theme.bg_shade)
                } else {
                    css!()
                };

                alternate = !alternate;
                let onclick = {
                    let nav = navc.clone();
                    let set_name = AttrValue::from(set.name().clone());
                    move |_| {
                        nav.push(&Route::Quiz { set_name: set_name.clone() })
                    }
                };

                match user.access_level() {
                    AccessLevel::USER => html! {
                        <Row {class} wfill={true} justify_content={"space-between"} align_items={"center"}>
                            <p><b>{ set.name() }</b></p>
                            <p>{ set.author() }</p>
                            <Button {onclick}>{ "> Go!" }</Button>
                        </Row>
                    },
                    _ => {
                        let dependency = dependency.clone();
                        let onclick = move |_| {
                            let dependency = dependency.clone();
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
                                            .set_kind(MessageDialogKind::Error)
                                            .message(&why.message)
                                            .await;
                                    }
                                }
                                dependency.set(!*dependency);
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
            }).collect::<Html>(),
        Err(_) => html! { "Loading..." },
    };

    html! {
        <Column hfill={true} {class}>
                <Row wfill={true} justify_content={"space-between"} align_items={"center"}>
                    <div><Button onclick={move |_| nav.push(&Route::Dashboard)}>{ "← Back" }</Button></div>
                    <div><h1>{ "Question Browser" }</h1></div>
                    <div>{create_button}</div>
                </Row>
            { set_bars }
        </Column>
    }
}
