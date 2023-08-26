use crate::model::Model;

pub(crate) struct GameModel {
    guesses_remaining: u32,
    secret_number: u32,
}

impl GameModel {
    pub fn new(secret_number: u32) -> GameModel {
        GameModel {
            guesses_remaining: 10,
            secret_number,
        }
    }
}

impl Model for GameModel {
    fn check_guess(&mut self, guess: u32) -> bool {
        self.guesses_remaining -= 1;
        self.secret_number.eq(&guess)
    }

    fn guesses_remaining(&self) -> bool {
        self.guesses_remaining > 0
    }

    fn num_guesses_remaining(&self) -> u32 {
        self.guesses_remaining
    }

    fn reset(&mut self, secret_number: u32) {
        self.guesses_remaining = 10;
        self.secret_number = secret_number;
    }
}