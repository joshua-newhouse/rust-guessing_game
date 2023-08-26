use crate::controller::Controller;
use crate::model::Model;
use crate::view::View;
use std::process::exit;

pub(crate) struct Game<'a> {
    pub(crate) view: &'a dyn View,
    pub(crate) model: &'a mut dyn Model,
    pub(crate) controller: &'a dyn Controller,
}

impl<'a> Game<'a> {
    pub(crate) fn new(
        view: &'a impl View,
        model: &'a mut impl Model,
        controller: &'a impl Controller,
    ) -> Game<'a> {
        Game {
            view,
            model,
            controller,
        }
    }

    pub(crate) fn run(&mut self) {
        let mut quit = false;

        self.view.display_intro(self.model.num_guesses_remaining());

        // loop until user quits
        while !quit {
            self.view.display_inter_round();

            // loop until round ends
            let mut round_in_progress = true;
            while round_in_progress {
                self.view.display_status(self.model.num_guesses_remaining());

                let guess: String;
                match self.controller.receive_user_input() {
                    Ok(input) => guess = input,
                    Err(e) => {
                        println!("Failed getting input; terminating\n{e}");
                        exit(1);
                    }
                }

                if guess.eq_ignore_ascii_case("q") {
                    quit = true;
                    break;
                }

                let mut is_correct = false;
                match guess.parse::<u32>() {
                    Ok(number) => is_correct = self.model.check_guess(number),
                    Err(e) => {
                        self.view.display_input_format_error(e);
                        self.model.check_guess(0);
                    }
                };

                self.view.display_result(&guess, is_correct);

                round_in_progress = !is_correct && self.model.guesses_remaining();
            }

            self.model.reset(99);
        }

        self.view.display_exit();
    }
}
