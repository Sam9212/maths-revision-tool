use crate::{
    app::Route,
    components::{
        inputs::{Button, ValidatedInput},
        layout::{Column, Row},
        questions::QuestionForm,
        theme_ctx::use_theme,
        user_ctx::use_user,
    },
};
use shared::questions::{QuestionBuilder, QuestionSet};
use stylist::yew::styled_component;
use tauri_sys::dialog::{MessageDialogBuilder, MessageDialogKind};
use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;

#[styled_component(Creator)]
pub fn creator() -> Html {
    let user_ctx = use_user();
    let nav = use_navigator().unwrap();

    if user_ctx.inner.is_none() {
        nav.push(&Route::Login);
    }

    let name = use_state_eq(|| AttrValue::from(String::new()));
    let name_v = use_state_eq(|| false);

    let theme = use_theme();
    let class = css!(
        r#"
            margin-top: ${fs};
            margin-left: ${fs};
            margin-right: ${fs};

            > div {
                margin-bottom: ${fs};
                background-color: ${bg};
            }

            h1 {
                font-size: calc( 1.5 * ${fs} );
                text-align: center;
            }
        "#,
        bg = theme.bg_color,
        fs = theme.font_size,
    );

    let questions = use_state_eq(|| QuestionBuilder::new());
    let page_no = use_state_eq(|| 0);
    let pg_changed = use_state_eq(|| false);

    let cl = {
        let page_no = page_no.clone();
        let pg_changed = pg_changed.clone();
        move |_| {
            if *page_no > 0 {
                pg_changed.set(true);
                page_no.set(*page_no - 1);
            }
        }
    };

    let cr = {
        let page_no = page_no.clone();
        let pg_changed = pg_changed.clone();
        move |_| {
            pg_changed.set(true);
            page_no.set(*page_no + 1);
        }
    };

    let onclick = {
        let user_ctx = user_ctx.clone();
        let questions = questions.clone();
        let name = name.clone();
        let name_v = name_v.clone();

        move |_| {
            let user = (*user_ctx).clone();
            let user = user.inner.unwrap();
            let questions = (*questions).clone();
            log::info!("{:?}", questions);
            let questions = questions.build();

            log::info!("{:?}", questions);
            log::info!("{}", *name_v);

            if !*name_v {
                spawn_local(async move {
                    let _ = MessageDialogBuilder::new()
                        .set_title("Quiz Creator")
                        .set_kind(MessageDialogKind::Error)
                        .message("quiz name is invalid")
                        .await;
                });
            } else if let Some(qlist) = questions {
                let qset = QuestionSet::new((*name).to_string(), user.username().clone(), qlist);
                let nav = nav.clone();
                spawn_local(async move {
                    match crate::commands::invoke_add_question_set(qset).await {
                        Ok(_) => {
                            let _ = MessageDialogBuilder::new()
                                .set_title("Quiz Creator")
                                .set_kind(MessageDialogKind::Info)
                                .message("succesfully uploaded question set")
                                .await;
                        }
                        Err(why) => {
                            let _ = MessageDialogBuilder::new()
                                .set_title("Quiz Creator")
                                .set_kind(MessageDialogKind::Error)
                                .message(&why.message)
                                .await;
                        }
                    }

                    nav.push(&Route::Browser);
                });
            } else {
                spawn_local(async move {
                    let _ = MessageDialogBuilder::new()
                        .set_title("Quiz Creator")
                        .set_kind(MessageDialogKind::Error)
                        .message("question form was not all valid, try again")
                        .await;
                });
            }
        }
    };

    html! {
        <Column {class} hfill={true}>
            <Row wfill={true} justify_content={"space-between"} align_items={"center"}>
                <ValidatedInput id={"name"} maxl={30} text_handle={name} validity_handle={name_v}>{ "Q-Set Name" }</ValidatedInput>
                <div class={classes!("expander")}></div>
                <h1>{ "Question Writer" }</h1>
                <div class={classes!("expander")}></div>
                <Button {onclick}>{ "Publish" }</Button>
            </Row>
            <Column hfill={true} wfill={true} justify_content={"center"} align_items={"center"}>
                <QuestionForm page_no={page_no.clone()} {questions} {pg_changed}/>
            </Column>
            <Row wfill={true} justify_content={"space-between"} align_items={"center"}>
                <Button onclick={cl}>{ "<" }</Button>
                <h1><b>{ "Editing Question " }{ *page_no + 1 }</b></h1>
                <Button onclick={cr}>{ ">" }</Button>
            </Row>
        </Column>
    }
}
