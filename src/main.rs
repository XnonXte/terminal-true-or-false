use htmlentity::entity::{decode, ICodedDataTrait};
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

const TRIVIA_URL_ENDPOINT: &str = "https://opentdb.com/api.php?type=boolean";
const DIFFICULTIES: [&str; 4] = ["easy", "medium", "hard", "all"];
const MAX_AMOUNT: i32 = 50;

#[derive(Debug, Deserialize, Serialize)]
struct Trivia {
    r#type: String,
    difficulty: String,
    category: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TriviaResult {
    response_code: i32,
    results: Vec<Trivia>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Welcome to Terminal True or False V0.1");
    println!("Copyright (c) 2024 XnonXte");
    println!();

    let (amount, difficulty) = prompt_amount_and_difficulty();
    let url = format!(
        "{}&amount={}&difficulty={}",
        TRIVIA_URL_ENDPOINT,
        amount,
        if difficulty == "all" {
            String::new()
        } else {
            difficulty
        }
    );
    let trivia_result: TriviaResult = reqwest::Client::new().get(url).send().await?.json().await?;
    let (correct_count, incorrect_count) = prompt_questions(&trivia_result.results);
    println!(
        "Game finished with {} correct and {} incorrect answer(s)!",
        correct_count, incorrect_count
    );
    println!("Thanks for playing my game!");

    Ok(())
}

fn capitalize(string: &str) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn decode_html_entities(content: &str) -> String {
    decode(content.as_bytes()).to_string().unwrap()
}

fn input(message: &str) -> String {
    print!("{}", message);
    stdout().flush().expect("Failed to flush stdout");
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).expect("Failed to read line.");

    tmp.trim_end().to_string()
}

fn process_question(correct_answer: &str) -> Result<bool, &'static str> {
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

fn prompt_questions(trivia_vec: &Vec<Trivia>) -> (i32, i32) {
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

fn prompt_amount_and_difficulty() -> (i32, String) {
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
