use crate::stones::{get_stone_name_from_stone, Stone};
use std::fmt::{Display, Error, Formatter, Result};
use std::hash::Hash;
use crate::board::Board;
use crate::game::{Coordinates, Move};
use std::io::stdin;

pub trait Player {
    fn get_stone(&self) -> &Stone;
    fn get_name(&self) -> &str {
        get_stone_name_from_stone(*self.get_stone())
    }
    fn choose_case(&self, board: &Board) -> Move;
}

impl Display for dyn Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", self.get_name())
        }
}

impl Hash for dyn Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_name().hash(state);
    }
}

impl PartialEq for dyn Player {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl Eq for dyn Player {}
