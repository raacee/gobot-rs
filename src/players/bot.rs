use crate::board::Board;
use crate::game::Move;
use crate::players::player::Player;
use crate::stones::Stone;

pub struct Bot {
    name: String,
    stone: Stone
}

impl Bot {
    fn next_best_move(&self, board: &Board) -> Move {
        todo!()
    }
}

impl Player for Bot {
    fn get_stone(&self) -> &Stone { &self.stone }
    fn get_name(&self) -> &str { &self.name }
    fn choose_case(&self, board: &Board) -> Move {
        self.next_best_move(board)
    }
}