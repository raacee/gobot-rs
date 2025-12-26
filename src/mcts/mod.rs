use std::cmp::PartialEq;
use std::f32::consts::SQRT_2;
use std::rc::Rc;
use rand::prelude::*;
use crate::game;
use crate::game::{Coordinates, Game, Move};
use crate::players::bot::Bot;
use crate::players::human::Human;
use crate::players::player::Player;
use crate::signals::Signals;
use crate::stones::{BLACK_STONE, WHITE_STONE};

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

    fn expand(&mut self) {
        let random_child = self.generate_random_child();
        self.add_child(random_child);
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

    fn generate_random_child(& self) -> Node {
        let random_choice = self.generate_random_choice();
        let mut state_clone = self.state.clone();
        let mut result_step = state_clone.step(random_choice);
        while result_step.is_err() {
            let result_step_err = result_step.err().unwrap();
            if result_step_err == Signals::GameOver || result_step_err == Signals::DoublePass {
                break
            }
            else {
                let random_choice = self.generate_random_choice();
                state_clone = self.state.clone();
                result_step = state_clone.step(random_choice);
            }

        }
        Self::new(state_clone)

    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
    
    fn is_over(&self) -> bool { self.state.is_over() }
}

pub fn run_simulation() {
    let black_bot = Bot::new(BLACK_STONE);
    let white_bot = Bot::new(WHITE_STONE);
    let black_player = Box::new(black_bot);
    let white_player = Box::new(white_bot);
    let players: [Rc<Box<dyn Player>>; 2] = [Rc::new(black_player), Rc::new(white_player)];
    let initial_game = game::Game::new(
        players,
        false,
        7.5
    );

    let mut node = Node::new(initial_game);
    while !node.state.is_over() {
        let child = node.generate_random_child();
        node = child;
    }

    let winner = node.state.winner();
    match winner {
        Some(p) => println!("Simulation ended with winner : {}", *p),
        None => println!("Simulation ended with draw")
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