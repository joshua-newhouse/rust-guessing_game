use crate::model::Model;
use rand::Rng;
use std::cmp::Ordering;

pub(crate) struct GameModel {
    guesses_remaining: u32,
    secret_number: u32,
}

impl GameModel {
    pub fn new() -> GameModel {
        GameModel {
            guesses_remaining: 5,
            secret_number: rand::thread_rng().gen_range(1..=100),
        }
    }
}

impl Model for GameModel {
    fn check_guess(&mut self, guess: u32) -> Ordering {
        self.guesses_remaining -= 1;
        guess.cmp(&self.secret_number)
    }

    fn guesses_remaining(&self) -> bool {
        self.guesses_remaining > 0
    }

    fn num_guesses_remaining(&self) -> u32 {
        self.guesses_remaining
    }

    fn reset(&mut self) {
        let new_model = GameModel::new();

        self.guesses_remaining = new_model.guesses_remaining;
        self.secret_number = new_model.secret_number;
    }

    fn secret_number(&self) -> u32 {
        self.secret_number
    }

    fn decrement_guesses_remaining(&mut self) {
        self.guesses_remaining -= 1;
    }
}
