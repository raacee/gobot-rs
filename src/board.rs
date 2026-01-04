use crate::stones::{Stone};
use std::ops::{Index, IndexMut};

pub type BoardSize = (usize, usize);
pub const SIDE: usize = 19;
pub type BoardArray = [[Stone; SIDE]; SIDE];

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Board {
    pub data:BoardArray,
    pub board_side_length: usize,
    pub shape:BoardSize
}

impl Board {
    pub fn new() -> Board {
        let data: [[Stone;SIDE];SIDE] = [[None; SIDE]; SIDE];
        Board {
            data,
            board_side_length: SIDE,
            shape: (SIDE, SIDE)
        }
    }
}

impl Index<(usize, usize)> for Board
{
    type Output = Stone;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x][y]
    }
}

impl IndexMut<(usize, usize)> for Board{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x][y]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
