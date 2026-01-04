use std::cell::RefCell;
use crate::board::Board;
use crate::game::{Game, Move};
use crate::mcts::tree::Tree;
use crate::players::player::Player;
use crate::stones::{get_stone_name_from_stone, Stone};

const N_ITER: u32 = 10;


pub struct Bot {
    name: String,
    stone: Stone,
    tree: RefCell<Tree>
}

impl Bot {
    pub fn new(stone: Stone) -> Bot {
        Bot {
            name: get_stone_name_from_stone(stone).to_string(),
            tree: RefCell::new(Tree::new()),
            stone
        }
    }

    /*
        With area scoring, the bots should continue playing unless playing leads to a situation where a group loses an eye
        In that case, the bot should pass
        So, the bots consolidate their position unless they lose an alive group if they play any move

        Or

        We can use the calculate_scores function as a heuristic to find
        if any move leads to a losing situation

        If previous player has passed and the bot has a better score than the player, the bot should pass to win the game
    */

    fn next_best_move(&self, game: &Game) -> Move {
        self.refresh_state(game);
        self.think();
        let best_move_ref = self.tree.borrow().best_move();
        let best_move = best_move_ref.borrow().0.clone();
        self.tree.borrow_mut().replace_root(best_move_ref);
        best_move
    }
    
    fn refresh_state(&self, game: &Game) {
        // find a node in children of root which game state represents the current state of the game
        self.tree.borrow_mut().update_root_after_move(game)
    }
    
    fn think(&self) {
        let tree_borrow_mut = self.tree.borrow_mut();
        for _ in 0..N_ITER {
            tree_borrow_mut.mcts_step();
        }
    }
}

impl Player for Bot {
    fn get_stone(&self) -> Stone { self.stone }
    fn get_name(&self) -> &str { &self.name }
    fn choose_case(&self, game: &Game) -> Move {
        self.next_best_move(game)
    }
}