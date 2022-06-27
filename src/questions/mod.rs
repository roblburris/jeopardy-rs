use std::fs;
use serde_json;
use serde::{Deserialize};
use std::vec;
use std::env;

#[derive(Deserialize)]
pub struct QuestionOpt {
    category: Option<String>,
    value: Option<String>,
    question: Option<String>,
    answer: Option<String>,
    round: Option<String>,
    show_number: Option<String>,
    air_date: Option<String>,
}

#[derive(Debug)]
pub struct Question {
    pub category: String,
    pub question: String,
    pub answer: String,
    pub value: String
}


pub struct QuestionWrapper {
    pub question_buffer: Vec<Question>
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct QuestionWrapperOpt {
    pub question_buffer: Vec<QuestionOpt>
}

pub fn open_questions(file_path: &str) -> Box<QuestionWrapper> {
    let questions_src = fs::read_to_string(file_path).expect("unable to open file");
    let qwo: QuestionWrapperOpt = serde_json::from_str(questions_src.as_str()).expect("error deserializing json file");
    return Box::new(QuestionWrapper{question_buffer: qwo.question_buffer.into_iter().
        filter(|cur| check_question_opt_fields(cur)).
        map(|c| Question{question: c.question.unwrap(),category: c.category.unwrap(),
            answer: c.answer.unwrap(), value: c.value.unwrap()[1..].to_string()}).
        collect() });
}

fn check_question_opt_fields(q: &QuestionOpt) -> bool {
    q.question.is_some() && q.answer.is_some() && q.round.is_some() && q.value.is_some()
}