use crate::board::Board;
use crate::player::Human;
use crate::stones::{BLACK_STONE, WHITE_STONE};
use std::collections::{VecDeque};

mod board;
mod game;
mod player;
mod stones;
mod signals;
mod bot;

fn main() {
    let black_player = Human::new(BLACK_STONE);
    let white_player = Human::new(WHITE_STONE);
    let players: VecDeque<&Human> = [&black_player, &white_player].into();
    let game = game::Game::new(
        9,
        false,
        players,
        true,
        false,
        7.5);
}
