use crate::stones::{Stone, EMPTY};
use std::ops::{Index, IndexMut};
use crate::game::Coordinates;

pub type BoardSideLength = usize;
pub type BoardSize = (usize, usize);
pub type BoardArray = Vec<Vec<Stone>>;


const SMALL: BoardSideLength = 9;
const MEDIUM: BoardSideLength = 13;
const BIG: BoardSideLength = 19;

#[derive(Eq, Hash, PartialEq)]
pub struct Board {
    pub data:BoardArray,
    pub board_side_length: BoardSideLength,
    pub shape:BoardSize
}

impl Board {
    pub fn new(mut side_length:BoardSideLength) -> Board {
        if ![SMALL, MEDIUM, BIG].contains(&side_length) {
            side_length = BIG;
        }
        let mut data = Vec::new();
        for _ in 0..side_length {
            let row = vec![EMPTY; side_length];
            data.push(row);
        }
        Board {
            data,
            board_side_length: side_length,
            shape: (side_length, side_length)
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut new_data: BoardArray = Vec::with_capacity(self.board_side_length);
        for row in &self.data {
            let mut new_row: Vec<Stone> = Vec::with_capacity(self.board_side_length);
            for cell in row {
                new_row.push(cell.clone());
            }
            new_data.push(new_row)
        }

        Board {
            data:new_data,
            board_side_length: self.board_side_length.clone(),
            shape: self.shape.clone()
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
