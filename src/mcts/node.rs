use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::IndexedRandom;
use crate::game::{Game, Move};
use crate::mcts::{MoveNodeRef, tree::Tree};
use crate::players::player::Player;

#[derive(Clone, Default)]
pub struct Node {
    pub(crate) state: Game,
    pub(crate) children: Vec<MoveNodeRef>,
    eval: f32,
    pub(crate) won: f32,
    pub(crate) total: f32
}

impl Node {
    pub fn new() -> Node {
        let state= Game::default();
        let score = state.calculate_scores_difference();
        Node { state, children:vec![], eval: score, won: 0.0, total: 0.0 }
    }

    pub fn from(state: Game, children: Vec<MoveNodeRef>, eval: f32, won:f32, total:f32) -> Node {
        Node {
            state,
            children,
            eval,
            won,
            total
        }
    }

    pub(crate) fn expand(&mut self) {
        let random_child = self.generate_random_child();
        self.add_child(Rc::new(RefCell::new(random_child.unwrap())));
    }

    fn generate_random_choice(&self) -> Move {
        let mut rng = rand::rng();
        let available_cases = self.state.available_cases(self.state.get_current_player());
        let chosen_move = available_cases.choose(&mut rng);
        match chosen_move {
            Some(c) => *c,
            None => None
        }
    }

    fn generate_random_child(&self) -> Option<(Move, Node)> {
        if !self.is_over() {
            let random_choice = self.generate_random_choice();
            let mut state_clone = self.state.clone();
            let mut result_step = state_clone.step(random_choice);
            while result_step.is_err() {
                if state_clone.is_over() {
                    break;
                } else {
                    let random_choice = self.generate_random_choice();
                    state_clone = self.state.clone();
                    result_step = state_clone.step(random_choice);
                }
            }
            let eval = state_clone.calculate_scores_difference();
            return Some((random_choice, Self::from(state_clone, vec![], eval, 0.0, 0.0)))
        }
        None
    }

    pub(crate) fn run_simulation(&self) -> Option<Rc<Box<dyn Player>>> {
        let mut current_node = self.clone();
        while !current_node.state.is_over() {
            let child = current_node.generate_random_child();
            (_, current_node) = child.unwrap();
        }

        current_node.state.winner()
    }

    fn add_child(&mut self, child: MoveNodeRef) {
        if !self.is_over() {
            self.children.push(child);
        } else {
            panic!("Tried to add a child to a finished game");
        }
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn is_over(&self) -> bool { self.state.is_over() }

    fn increment_total(&mut self) { self.total += 1.0 }

    pub(crate) fn increment_won(&mut self) {
        self.won += 1.0;
        self.increment_total();
    }

    pub(crate) fn increment_draw(&mut self) {
        self.won += 0.5;
        self.increment_total();
    }

    pub(crate) fn increment_lost(&mut self) { self.increment_total(); }

    pub(crate) fn most_suited_child_selection(&self) -> MoveNodeRef {
        let mut best_child = Rc::clone(&self.children[0]);
        let mut best_selection_eval = Tree::calculate_selection_criteria(&best_child.borrow().1, self);
        for child in &self.children {
            let selection_eval = Tree::calculate_selection_criteria(&child.borrow().1, self);
            if selection_eval > best_selection_eval {
                best_child = Rc::clone(child);
                best_selection_eval = selection_eval;
            }
        }
        best_child
    }

    pub(crate) fn least_winning_child(&self) -> MoveNodeRef {
        let mut worst_child = Rc::clone(&self.children[0]);
        let mut least_winning_probability = worst_child.borrow().1.won / worst_child.borrow().1.total;
        for child in &self.children {
            let winning_probability = Tree::calculate_selection_criteria(&child.borrow().1, self);
            if winning_probability < least_winning_probability {
                worst_child = Rc::clone(child);
                least_winning_probability = winning_probability;
            }
        }
        worst_child
    }

}