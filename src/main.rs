use std::rc::Rc;
use crate::game::Game;
use crate::players::bot::Bot;
use crate::players::human::Human;
use crate::players::player::Player;
use crate::stones::{BLACK_STONE, WHITE_STONE};

mod board;
mod game;
mod stones;
mod signals;
mod mcts;
mod players;

fn main() {
    let players: [Rc<Box<dyn Player>>; 2] = [Rc::new(Box::new(Human::new(BLACK_STONE))), Rc::new(Box::new(Bot::new(WHITE_STONE)))];
    let mut game = Game::new(players, true, 7.5);
    game.game()
}