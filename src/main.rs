mod controller;
mod game;
mod model;
mod view;

use crate::controller::KeyboardController;
use crate::game::Game;
use crate::model::GameModel;
use crate::view::ConsoleView;

fn main() {
    let game_components = (
        &ConsoleView::new(),
        &mut GameModel::new(10),
        &KeyboardController::new(),
    );

    let mut game = Game::new(game_components.0, game_components.1, game_components.2);

    game.run();
}
