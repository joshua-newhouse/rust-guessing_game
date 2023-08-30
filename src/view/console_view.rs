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
        println!("Guess the number, it is in the range [1, 100].");
    }

    fn display_low_guess(&self) {
        println!("Your guess was too low, try a higher number.");
    }

    fn display_correct_guess(&self) {
        println!("Congratulations, you correctly guessed the secret number!");
    }

    fn display_high_guess(&self) {
        println!("Your guess was too high, try a lower number.");
    }

    fn display_begin_round(&self) {
        println!("The round begins now.");
    }

    fn display_end_round(&self, guessed_correctly: bool, secret_number: u32) {
        if !guessed_correctly {
            println!("You didn't win this round, the secret number was {secret_number}");
        }
    }

    fn display_exit(&self) {
        println!("Thanks for playing; see you again next time!");
    }
}
