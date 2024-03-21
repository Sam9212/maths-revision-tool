#![allow(non_camel_case_types)]

use crate::{
    app::Route,
    components::{
        inputs::{Button, ValidatedInput},
        layout::{Column, Row},
        theme_ctx::use_theme,
        user_ctx::use_user,
    },
};
use shared::{
    questions::QuestionSet,
    requests::UserReqError,
    responses::{QuizReview, Response},
};
use stylist::yew::styled_component;
use tauri_sys::dialog::{MessageDialogBuilder, MessageDialogKind};
use yew::{
    platform::spawn_local,
    prelude::*,
    suspense::{use_future, Suspension, UseFutureHandle},
};
use yew_autoprops::autoprops;
use yew_router::{components::Redirect, hooks::use_navigator};

#[hook]
pub fn use_question_set(
    name: AttrValue,
) -> Result<UseFutureHandle<Result<QuestionSet, UserReqError>>, Suspension> {
    let name = name.to_string();
    use_future(|| async { crate::commands::invoke_get_question_set(name).await })
}

#[autoprops]
#[styled_component(Quiz)]
pub fn quiz(set_name: AttrValue) -> Html {
    let theme = use_theme();
    let nav = use_navigator().unwrap();

    let user = use_user();
    let user = match (*user).clone().inner {
        Some(user) => user,
        None => {
            return html! { <Redirect<Route> to={Route::Login}/>};
        }
    };

    let current_question = use_state_eq(|| 0usize);
    let responses = use_state_eq(|| vec![]);
    let answer = use_state_eq(|| AttrValue::from(String::new()));
    let valid = use_state_eq(|| true);

    let set = match use_question_set(set_name) {
        Ok(res) => match res.clone() {
            Ok(qs) => qs,
            Err(why) => {
                spawn_local(async move {
                    let _ = MessageDialogBuilder::new()
                        .set_title("Quiz")
                        .set_kind(MessageDialogKind::Error)
                        .message(&why.message)
                        .await;
                });
                return html! { <Redirect<Route> to={Route::Browser}/> };
            }
        },
        Err(_) => return html! { "Loading..." },
    };

    let onquit = {
        let nav = nav.clone();
        move |_| {
            nav.push(&Route::Browser);
        }
    };

    let total = set.questions().len();
    let current = set.questions()[*current_question].clone();

    let onclick = {
        let user = user.clone();
        let handle = answer.clone();
        let submitted = answer.to_string();
        let question = format!("<p>{}</p><p>{}</p>", current.title(), current.markup());
        let nav = nav.clone();
        let answer = current.answer().clone();
        let responses = responses.clone();

        let current_question = current_question.clone();
        move |_| {
            let response = Response::new(question.clone(), submitted.clone(), answer.clone());
            let mut tmp_rsp = (*responses).clone();
            tmp_rsp.push(response);
            log::info!("{:?}", tmp_rsp);
            responses.set(tmp_rsp.clone());

            handle.set(AttrValue::from(String::new()));

            if *current_question + 1 == total {
                let quiz_review = QuizReview::new(user.username().clone(), tmp_rsp);
                spawn_local(async move {
                    match crate::commands::invoke_add_quiz_review(quiz_review).await {
                        Ok(_) => {
                            let _ = MessageDialogBuilder::new()
                                .set_title("Quiz")
                                .set_kind(MessageDialogKind::Info)
                                .message("The quiz was completed and a review was uploaded!")
                                .await;
                        }
                        Err(_) => {
                            let _ = MessageDialogBuilder::new()
                                .set_title("Quiz")
                                .set_kind(MessageDialogKind::Error)
                                .message("The review of this quiz failed to upload!")
                                .await;
                        }
                    };
                });
                nav.push(&Route::Review);
            } else {
                current_question.set(*current_question + 1);
            }
        }
    };

    let class = css!(
        r#"
            background-color: ${bg};
            p { display: none; }
            h1 { font-size: calc( 5 * ${fs} ); }
            h2 { font-size: calc( 2.5 * ${fs}); }
            h3 { font-size: calc( 1.5 * ${fs}); }
            input { margin-right: calc( 0.5 * ${fs}); }
            > * > button {
                margin: calc( 0.5 * ${fs});
                background-color: ${ec};
                border-color: ${ec};
            }
        "#,
        ec = theme.fail_color,
        fs = theme.font_size,
        bg = theme.bg_color,
    );
    html! {
        <Column {class} hfill={true}>
            <Row wfill={true} justify_content={"space-between"} align_items={"center"}>
                <h3>{ "Question " }{ *current_question + 1 }{ " of " }{ total }</h3>
                <Button onclick={onquit}>{ "Exit" }</Button>
            </Row>
            <Column hfill={true} wfill={true} align_items={"center"} justify_content={"center"}>
                <h1>{ current.title() }</h1>
                <h2>{ current.markup() }</h2>
                <br />
                <Row align_items={"center"} justify_content={"center"}>
                    <ValidatedInput id={"answer"} minl={0} maxl={30} validity_handle={valid} text_handle={answer}>{ "Enter Answer" }</ValidatedInput>
                    <Button {onclick}>{ "⏎" }</Button>
                </Row>
            </Column>
        </Column>
    }
}
