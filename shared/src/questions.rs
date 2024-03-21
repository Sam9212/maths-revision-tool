use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestionSet {
    name: String,
    author: String,
    questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuestionBuilder {
    inner: Vec<PartialQuestion>,
}

impl QuestionBuilder {
    pub fn new() -> Self {
        QuestionBuilder { inner: vec![] }
    }

    pub fn get(&mut self, i: usize) -> PartialQuestion {
        if self.inner.len() == i {
            self.inner.push(PartialQuestion::new());
        }
        self.inner[i].clone()
    }

    pub fn set(&mut self, i: usize, v: PartialQuestion) {
        self.inner[i] = v;
    }

    pub fn build(self) -> Option<Vec<Question>> {
        let mut out = Vec::with_capacity(self.inner.len());
        for v in self.inner {
            if let Some(v) = v.build() {
                out.push(v);
            } else {
                return None;
            }
        }
        Some(out)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartialQuestion {
    pub title: Option<String>,
    pub markup: Option<String>,
    pub calculator_allowed: Option<bool>,
    pub answer: Option<String>,
}

impl PartialQuestion {
    pub fn new() -> Self {
        PartialQuestion {
            title: None,
            markup: None,
            calculator_allowed: None,
            answer: None,
        }
    }

    pub fn build(self) -> Option<Question> {
        if let Some(title) = self.title {
            if let Some(markup) = self.markup {
                if let Some(calculator_allowed) = self.calculator_allowed {
                    if let Some(answer) = self.answer {
                        return Some(Question {
                            title,
                            markup,
                            calculator_allowed,
                            answer,
                        });
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Question {
    title: String,
    markup: String,
    calculator_allowed: bool,
    answer: String,
}

impl QuestionSet {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn questions(&self) -> &Vec<Question> {
        &self.questions
    }

    pub fn new(name: String, author: String, questions: Vec<Question>) -> Self {
        QuestionSet {
            name,
            author,
            questions,
        }
    }
}

impl Question {
    // Ref-getter for the title field
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn markup(&self) -> &String {
        &self.markup
    }

    pub fn calculator_allowed(&self) -> bool {
        self.calculator_allowed
    }

    pub fn answer(&self) -> &String {
        &self.answer
    }
}
