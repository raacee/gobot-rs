use crate::mcts::run_simulation;

mod board;
mod game;
mod stones;
mod signals;
mod mcts;
mod players;

fn main() {
    run_simulation()
}