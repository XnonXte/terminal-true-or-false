use crate::shared_constants::{DIFFICULTIES, MAX_AMOUNT};
use crate::utils::{capitalize, decode_html_entities, input};
use models::Trivia;

pub fn process_question(correct_answer: &str) -> Result<bool, &'static str> {
    let user_answer = match input("Your answer (Y/n): ").to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            return Err("Invalid answer, must be either 'y' or 'n'!");
        }
    };
    let answer = if correct_answer == "True" {
        true
    } else {
        false
    };

    Ok(user_answer == answer)
}

pub fn prompt_questions(trivia_vec: &Vec<Trivia>) -> (i32, i32) {
    let mut correct_count = 0;
    let mut incorrect_count = 0;
    let mut trivia_index: usize = 0;

    while trivia_index < trivia_vec.len() {
        let trivia = trivia_vec.get(trivia_index).unwrap();

        println!("Trivia {} out of {}", trivia_index + 1, trivia_vec.len());
        println!("Difficulty: {}", capitalize(&trivia.difficulty));
        println!("Category: {}", decode_html_entities(&trivia.category));
        println!("{}", decode_html_entities(&trivia.question));

        match process_question(&trivia.correct_answer) {
            Ok(is_correct) => {
                if is_correct {
                    println!("You're correct!");
                    correct_count += 1
                } else {
                    println!(
                        "Sorry, but the correct answer is {}",
                        if trivia.correct_answer == "True" {
                            "yes"
                        } else {
                            "no"
                        }
                    );
                    incorrect_count += 1
                }

                trivia_index += 1
            }
            Err(error) => {
                println!("{:?}", error);
                continue;
            }
        }
    }

    (correct_count, incorrect_count)
}

pub fn prompt_amount_and_difficulty() -> (i32, String) {
    loop {
        let amount = input("Amount of trivia: ")
            .parse::<i32>()
            .expect("Expected a digit value!");
        if amount > MAX_AMOUNT {
            println!("Can't be higher than 50!");
            continue;
        }
        let difficulty = input("Difficulty (easy, medium, hard, all): ").to_lowercase();
        if !DIFFICULTIES.contains(&difficulty.as_str()) {
            println!("Invalid difficulty!");
            continue;
        }

        return (amount, difficulty);
    }
}

pub mod models;
