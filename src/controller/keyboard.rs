use crate::controller::Controller;
use std::io;
use std::io::Result;

pub struct KeyboardController {}

impl KeyboardController {
    pub fn new() -> KeyboardController {
        KeyboardController {}
    }
}

impl Controller for KeyboardController {
    fn receive_user_input(&self) -> Result<String> {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .map(|_| String::from(input.trim()))
    }
}
