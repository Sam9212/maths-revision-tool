use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizReview {
    pub username: String,
    pub responses: Vec<Response>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Response {
    question: String,
    is_correct: bool,
    submitted: String,
    answer: String,
}

impl Response {
    pub fn new(question: String, submitted: String, answer: String) -> Response {
        Response {
            question,
            is_correct: submitted == answer,
            submitted,
            answer,
        }
    }

    pub fn is_correct(&self) -> bool {
        self.is_correct
    }

    pub fn question(&self) -> &String {
        &self.question
    }

    pub fn submitted(&self) -> &String {
        &self.submitted
    }

    pub fn answer(&self) -> &String {
        &self.answer
    }
}

impl QuizReview {
    pub fn new(username: String, responses: Vec<Response>) -> Self {
        QuizReview {
            username,
            responses,
        }
    }
}
