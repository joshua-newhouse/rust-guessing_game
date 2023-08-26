use crate::view::View;
use std::num::ParseIntError;

pub(crate) struct ConsoleView {}

impl ConsoleView {
    pub fn new() -> ConsoleView {
        ConsoleView {}
    }
}

impl View for ConsoleView {
    fn display_intro(&self, guesses_remaining: u32) {
        println!("Welcome to the number guessing game.");
        println!(
            "You have {guesses_remaining} attempts to correctly guess the secret number in each round.");
        println!("You may exit the game at any time by entering 'q'.");
    }

    fn display_input_format_error(&self, err: ParseIntError) {
        println!("Your guess is not a number; you must enter a number. {err}");
    }

    fn display_status(&self, guesses_remaining: u32) {
        println!("You have {guesses_remaining} guesses remaining this round.");
        println!("Guess the number!");
    }

    fn display_result(&self, guess: &str, correct: bool) {
        if correct {
            println!("Congratulations, {guess} was the secret number!");
        } else {
            println!("{guess} is not the secret number.")
        }
    }

    fn display_inter_round(&self) {
        println!("The round begins now.");
    }

    fn display_exit(&self) {
        println!("Thanks for playing; see you again next time!");
    }
}
