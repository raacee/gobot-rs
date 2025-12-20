use crate::board::Board;
use crate::game::Move;
use crate::players::player::Player;
use crate::stones::Stone;

pub struct Bot {
    name: String,
    stone: Stone
}

impl Bot {
    /*
        With area scoring, the bots should continue playing,
        unless playing leads to a situation where a group loses an eye
        In that case, the bot should pass
        So, the bots consolidate their position unless they lose an alive group if they play any move

        Or

        We can use the calculate_scores function as a heuristic to find
        if any move leads to a losing situation
    */
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