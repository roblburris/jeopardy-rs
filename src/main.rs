mod questions;
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut a = questions::open_questions("./static/questions.json");
    let mut guess = String::new();
    let mut score: u32 = 0;
    a.question_buffer.shuffle(&mut thread_rng());
    a.question_buffer.iter().for_each(|x| {
        println!("question: {}", x.question);
        io::stdin().read_line(&mut guess).expect("failed to read line");
        if guess.trim().to_lowercase().eq(&x.answer.trim().to_lowercase()) {
            score += x.value.parse::<u32>().unwrap();
            print!("correct! ");
        } else {
            print!("incorrect! correct answer is {}. ", x.answer);
        }
        println!("your score is: {}\n", score);
        guess.clear();
    });
}
