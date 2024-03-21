use shared::{requests::UserReqError, responses::QuizReview};
use stylist::yew::styled_component;
use yew::{
    prelude::*,
    suspense::{use_future_with, Suspension, UseFutureHandle},
};
use yew_router::components::Redirect;

use crate::{
    app::Route,
    commands::invoke_get_quiz_reviews,
    components::{
        layout::{Column, Row},
        theme_ctx::use_theme,
        user_ctx::use_user,
    },
};

#[hook]
pub fn use_quiz_reviews(
    update: UseStateHandle<bool>,
) -> Result<UseFutureHandle<Result<Vec<QuizReview>, UserReqError>>, Suspension> {
    use_future_with(update, |_| async { invoke_get_quiz_reviews().await })
}

#[styled_component(Review)]
pub fn review() -> Html {
    let theme = use_theme();
    let dependency = use_state_eq(|| false);

    let user = use_user();
    let user = match (*user).clone().inner {
        Some(user) => user,
        None => {
            return html! { <Redirect<Route> to={Route::Login}/>};
        }
    };

    let quiz_reviews = match use_quiz_reviews(dependency) {
        Ok(revs) => revs
            .clone()
            .unwrap()
            .into_iter()
            .filter(|review| &review.username == user.username())
            .map(|review| {
                let answers = review.responses.iter().map(|resp| {
                    html! {
                        <>
                            <Row wfill={true} justify_content={"space-between"} align_items={"center"}>
                                { Html::from_html_unchecked(AttrValue::from(resp.question().clone())) }
                                <p>{ resp.is_correct() }</p>
                                <p>{ resp.submitted() }</p>
                                <p>{ resp.answer() }</p>
                            </Row>
                            <br />
                        </>
                    }
                }).collect::<Html>();
                html! {
                    <Column>
                        { answers }
                    </Column>
                }
            })
            .collect::<Html>(),
        Err(_) => return html! { "Loading..." },
    };

    let class = css!(
        r#"
            background-color: ${bg};

            > div {
                margin: ${fs};
                background-color: ${bgs};
            }

            p {
                flex: 1;
                text-align: center;
            }
        "#,
        fs = theme.font_size,
        bg = theme.bg_color,
        bgs = theme.bg_shade,
    );
    html! {
        <div {class}>
            <Row justify_content={"space-between"} align_items={"center"}>
                <p>{ "Question" }</p>
                <p>{ "Equation" }</p>
                <p>{ "Answer Correct?" }</p>
                <p>{ "Submitted Answer" }</p>
                <p>{ "Correct Answer" }</p>
            </Row>
            { quiz_reviews }
        </div>
    }
}
