use rand::prelude::*;
use crate::game::{Coordinates, Game, Move};

struct Node {
    state: Game,
    children: Vec<Node>,
}

impl Node {
    fn new(state:Game) -> Node {
        let children = Vec::new();
        Node { state, children }
    }

    fn from(state: Game, children: Vec<Node>) -> Node {
        Node {
            state,
            children
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

    fn selection(&mut self) {todo!()}
    fn expansion(&mut self) {todo!()}
    fn simulation(&mut self) {todo!()}
    fn backpropagation(&mut self) {todo!()}
}