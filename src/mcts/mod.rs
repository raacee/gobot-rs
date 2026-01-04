use std::cell::RefCell;
use std::rc::Rc;
use crate::game::Move;
use crate::mcts::node::Node;

pub mod tree;
mod node;

type MoveNodeRef = Rc<RefCell<(Move, Node)>>;