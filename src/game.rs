use crate::controller::Controller;
use crate::model::Model;
use crate::view::View;
use std::cmp::Ordering;
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
            self.view.display_begin_round();

            // loop until round ends
            let mut round_in_progress = true;
            let mut won_round = false;
            while round_in_progress {
                self.view.display_status(self.model.num_guesses_remaining());

                let guess: String = match self.controller.receive_user_input() {
                    Ok(input) => input,
                    Err(e) => {
                        println!("Failed getting input; terminating\n{e}");
                        exit(1);
                    }
                };

                if guess.eq_ignore_ascii_case("q") {
                    quit = true;
                    break;
                }

                let guess_result = match guess.parse::<u32>() {
                    Ok(number) => self.model.check_guess(number),
                    Err(e) => {
                        self.view.display_input_format_error(e);
                        self.model.decrement_guesses_remaining();
                        continue;
                    }
                };

                match guess_result {
                    Ordering::Less => self.view.display_low_guess(),
                    Ordering::Equal => self.view.display_correct_guess(),
                    Ordering::Greater => self.view.display_high_guess(),
                }

                round_in_progress = guess_result.is_ne() && self.model.guesses_remaining();
                won_round = guess_result.is_eq();
            }

            self.view
                .display_end_round(won_round, self.model.secret_number());

            self.model.reset();
        }

        self.view.display_exit();
    }
}
