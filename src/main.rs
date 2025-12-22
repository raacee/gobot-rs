use players::human::Human;
use crate::stones::{BLACK_STONE, WHITE_STONE};
use std::rc::Rc;
use crate::players::player::Player;

mod board;
mod game;
mod stones;
mod signals;
mod mcts;
mod players;

fn main() {
    let black_human = Human::new(BLACK_STONE);
    let white_human = Human::new(WHITE_STONE);
    let black_player = Box::new(black_human);
    let white_player = Box::new(white_human);
    let players: [Rc<Box<dyn Player>>; 2] = [Rc::new(black_player), Rc::new(white_player)];
    let mut game = game::Game::new(
        9,
        false,
        players,
        true,
        false,
        7.5);

    game.game()
}