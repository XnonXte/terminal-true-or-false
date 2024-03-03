use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Trivia {
    pub r#type: String,
    pub difficulty: String,
    pub category: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TriviaResult {
    pub response_code: i32,
    pub results: Vec<Trivia>,
}
