#![allow(non_camel_case_types)]

use super::inputs::{RadioToggle, ValidatedInput};
use crate::commands::invoke_get_question_sets;
use shared::{
    questions::{QuestionBuilder, QuestionSet},
    requests::UserReqError,
};
use stylist::yew::styled_component;
use yew::{
    prelude::*,
    suspense::{use_future, Suspension, UseFutureHandle},
};
use yew_autoprops::autoprops;

#[autoprops]
#[styled_component(QuestionForm)]
pub fn question_form(
    page_no: &UseStateHandle<usize>,
    pg_changed: &UseStateHandle<bool>,
    questions: &UseStateHandle<QuestionBuilder>,
) -> Html {
    let qtitle = use_state_eq(|| AttrValue::from(String::new()));
    let qtitle_valid = use_state_eq(|| false);
    let markup = use_state_eq(|| AttrValue::from(String::new()));
    let markup_valid = use_state_eq(|| false);
    let answer = use_state_eq(|| AttrValue::from(String::new()));
    let answer_valid = use_state_eq(|| false);
    let calculator = use_state_eq(|| true);

    let mut questions_now = (**questions).clone();
    let mut question = questions_now.get(**page_no);
    log::info!("{:?}", question);

    if **pg_changed {
        pg_changed.set(false);
        qtitle.set(if let Some(s) = question.title.clone() {
            qtitle_valid.set(true);
            AttrValue::from(s)
        } else {
            qtitle_valid.set(false);
            AttrValue::from(String::new())
        });

        markup.set(if let Some(s) = question.markup.clone() {
            markup_valid.set(true);
            AttrValue::from(s)
        } else {
            markup_valid.set(false);
            AttrValue::from(String::new())
        });

        answer.set(if let Some(s) = question.answer.clone() {
            answer_valid.set(true);
            AttrValue::from(s)
        } else {
            answer_valid.set(false);
            AttrValue::from(String::new())
        });
    }

    if *qtitle_valid {
        question.title = Some((*qtitle).to_string());
    } else {
        question.title = None;
    }

    if *markup_valid {
        question.markup = Some((*markup).to_string());
    } else {
        question.markup = None;
    }

    if *answer_valid {
        question.answer = Some((*answer).to_string());
    } else {
        question.answer = None;
    }

    question.calculator_allowed = Some(*calculator);

    questions_now.set(**page_no, question);
    questions.set(questions_now);

    html! {
        <>
            <ValidatedInput id={"qtitle"} maxl={80} text_handle={qtitle} validity_handle={qtitle_valid}>{ "Enter Title" }</ValidatedInput>
            <ValidatedInput id={"markup"} maxl={50} text_handle={markup} validity_handle={markup_valid}>{ "Enter Markup" }</ValidatedInput>
            <ValidatedInput id={"answer"} minl={1} maxl={50} text_handle={answer} validity_handle={answer_valid}>{ "Enter Answer" }</ValidatedInput>
            <br />
            <p>{ "Is this a set for calculator use?" }</p>
            <br />
            <RadioToggle name={"calc"} handle={calculator}>{ "" }</RadioToggle>
        </>
    }
}

#[hook]
pub fn use_question_sets(
) -> Result<UseFutureHandle<Result<Vec<QuestionSet>, UserReqError>>, Suspension> {
    use_future(|| async { invoke_get_question_sets().await })
}
