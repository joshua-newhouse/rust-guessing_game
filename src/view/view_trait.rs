use std::num::ParseIntError;

pub(crate) trait View {
    fn display_intro(&self, guesses_remaining: u32);

    fn display_input_format_error(&self, err: ParseIntError);

    fn display_status(&self, guesses_remaining: u32);

    fn display_result(&self, guess: &str, correct: bool);

    fn display_inter_round(&self);

    fn display_exit(&self);
}
