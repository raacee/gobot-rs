use std::f32::consts::SQRT_2;
use rand::prelude::*;
use crate::game::{Coordinates, Game, Move};

struct Node {
    state: Game,
    children: Vec<Node>,
    won: f32,
    total: f32
}

impl Node {
    fn new(state:Game) -> Node {
        let children = Vec::new();
        Node { state, children, won: 0.0, total: 0.0 }
    }

    fn from(state: Game, children: Vec<Node>, won:f32, total:f32) -> Node {
        Node {
            state,
            children,
            won,
            total
        }
    }

    fn expand(&self) {

    }

    fn generate_random_choice(&self) -> Move {
        let mut rng = rand::rng();
        *self.state.available_cases(self.state.get_current_player()).choose(&mut rng).unwrap()
    }

    fn generate_random_child(&mut self) -> Node {
        let random_choice = self.generate_random_choice();
        let mut state_clone = self.state.clone();
        let result_step = state_clone.step(random_choice);
        match result_step {
            Ok(_) => {
                Self::new(state_clone)
            },
            Err(signal) => self.generate_random_child()
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
    
    fn is_over(&self) -> bool { self.state.is_over() }
}

pub struct Tree {
    root: Node,
}

impl Tree {
    fn new(state: Game) -> Tree {
        Tree {
            root: Node::new(state),
        }
    }

    fn selection(&self) -> &Node {
        let mut node = &self.root;
        while !node.is_leaf() {
            let children = &node.children;
            let mut best_child = &children[0];
            let mut best_eval = Self::calculate_selection_criteria(best_child, node);
            for child in children {
                let child_eval = Self::calculate_selection_criteria(child, node);
                if child_eval > best_eval {
                    best_child = child;
                    best_eval = child_eval;
                }
            }
            node = best_child;
        }
        node
    }
    fn expansion(&mut self) {todo!()}
    fn simulation(&mut self) {todo!()}
    fn backpropagation(&mut self) {todo!()}

    fn calculate_selection_criteria(node: &Node, parent: &Node) -> f32 {
        let w = node.won;
        let n = node.total;
        let c = SQRT_2;

        w/n + c*(parent.total.ln()/n).sqrt()
    }
}