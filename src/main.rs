mod game;
mod shared_constants;
mod utils;

use game::models::TriviaResult;
use game::{prompt_amount_and_difficulty, prompt_questions};
use shared_constants::TRIVIA_URL_ENDPOINT;

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
