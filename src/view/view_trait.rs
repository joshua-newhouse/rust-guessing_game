use std::num::ParseIntError;

pub(crate) trait View {
    fn display_intro(&self, guesses_remaining: u32);

    fn display_input_format_error(&self, err: ParseIntError);

    fn display_status(&self, guesses_remaining: u32);

    fn display_result(&self, guess: &str, correct: bool);

    fn display_begin_round(&self);

    fn display_end_round(&self, guessed_correctly: bool, secret_number: u32);

    fn display_exit(&self);
}
