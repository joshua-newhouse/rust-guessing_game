pub(crate) trait Model {
    fn check_guess(&mut self, guess: u32) -> bool;

    fn guesses_remaining(&self) -> bool;

    fn num_guesses_remaining(&self) -> u32;

    fn reset(&mut self);

    fn secret_number(&self) -> u32;
}
