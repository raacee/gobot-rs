use std::cell::RefCell;
use std::collections::VecDeque;
use std::f32::consts::SQRT_2;
use std::rc::Rc;
use crate::board::Board;
use crate::game::{Game, Move};
use crate::mcts::{MoveNodeRef, node::Node};



#[derive(Default)]
pub struct Tree {
    pub(crate) root: MoveNodeRef,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Rc::new(RefCell::new((None, Node::new())))
        }
    }

    fn from(root: MoveNodeRef) -> Tree {
        Tree {
            root
        }
    }

    fn selection(&self) -> Vec<MoveNodeRef> {
        let mut path = Vec::new();
        let mut current_node = Rc::clone(&self.root);
        path.push(Rc::clone(&current_node));

        while !current_node.borrow().1.is_leaf() {
            let best_child = current_node.borrow().1.most_suited_child_selection();
            path.push(Rc::clone(&best_child));
            current_node = best_child;
        }
        path
    }
    pub fn mcts_step(&self) {
        // Selection
        let leaf_node_path = self.selection();
        let end_child = leaf_node_path.last().unwrap();
        // Expansion
        end_child.borrow_mut().1.expand();
        // Simulation
        let winner_opt = end_child.borrow().1.run_simulation();
        let leaf_node_path_reverse = leaf_node_path.iter().rev();
        // Backpropagation
        match winner_opt {
            Some(winner) => {
                for node in leaf_node_path_reverse {
                    if winner == node.borrow().1.state.get_current_player() {
                        node.borrow_mut().1.increment_won()
                    }
                    else {
                        node.borrow_mut().1.increment_lost()
                    }
                }
            }
            None => {
                for node in leaf_node_path_reverse {
                    node.borrow_mut().1.increment_draw()
                }
            }
        }
    }

    fn get_leaves(&self) -> Vec<MoveNodeRef> {
        // Breadth first search
        let mut res: Vec<MoveNodeRef> = vec![];
        let mut queue: VecDeque<MoveNodeRef> = VecDeque::new();
        queue.push_back(Rc::clone(&self.root));
        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap();
            if current_node.borrow().1.is_leaf() {
                res.push(current_node);
            } else {
                for child in &current_node.borrow().1.children {
                    queue.push_back(Rc::clone(child));
                }
            }
        }
        res
    }

    pub fn best_move(&self) -> MoveNodeRef {
        // Select the child of root for which the move MINIMIZE prob of winning for the opponent
        self.root.borrow().1.least_winning_child()
    }

    pub fn replace_root(&mut self, move_node_ref: MoveNodeRef) {
        self.root = move_node_ref
    }

    fn find_state_in_root_children(&self, game: &Game) -> Option<MoveNodeRef> {
        for child in self.root.borrow().1.children.iter() {
            if child.borrow().1.state == *game {
                return Some(Rc::clone(child));
            }
        }
        None
    }

    pub fn update_root_after_move(&mut self, game: &Game) {
        let game_in_children_option = self.find_state_in_root_children(game);

        match game_in_children_option {
            Some(child) => {
                self.replace_root(child);
            }
            None => {
                let new_root = Node::from(game.clone(), vec![], game.calculate_scores_difference(), 0.0, 0.0);
                let new_root_ref = Rc::new(RefCell::new((None, new_root)));
                self.replace_root(new_root_ref);
            }
        }
    }

    pub(crate) fn calculate_selection_criteria(node: &Node, parent: &Node) -> f32 {
        let w = node.won;
        let n = node.total;
        let c = SQRT_2;

        w/n + c*(parent.total.ln()/n).sqrt()
    }
}