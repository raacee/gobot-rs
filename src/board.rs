use std::fmt::{Display, Formatter};
use crate::stones::{Stone, EMPTY};
use std::ops::{Index, IndexMut};
use std::path::Iter;
use crate::game::Coordinates;

pub type BoardSideLength = usize;
pub type BoardSize = (usize, usize);
pub type BoardArray = Vec<Vec<Stone>>;


const SMALL: BoardSideLength = 9;
const MEDIUM: BoardSideLength = 13;
const BIG: BoardSideLength = 19;

const SMALL_SIZE: BoardSize = (SMALL, SMALL);
const MEDIUM_SIZE: BoardSize = (MEDIUM, MEDIUM);
const BIG_SIZE: BoardSize = (BIG, BIG);

const SMALL_BOARD: [[Stone; SMALL]; SMALL] = [[None; SMALL]; SMALL];
const MEDIUM_BOARD: [[Stone; MEDIUM]; MEDIUM] = [[None; MEDIUM]; MEDIUM];
const BIG_BOARD: [[Stone; BIG]; BIG] =  [[None; BIG]; BIG];

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

    pub fn get(&self, (x, y): (usize, usize)) -> &Stone {
        &self.data[x][y]
    }
    
    pub fn get_board_side_length(&self) -> BoardSideLength {
        self.board_side_length.clone()
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut new_data: BoardArray = Vec::new();
        for row in &self.data {
            let mut new_row: Vec<Stone> = Vec::new();
            for cell in row {
                new_row.push(cell.clone());
            }
            new_data.push(new_row)
        }
        let new_shape = (self.shape.0, self.shape.1);
        Board {
            data:new_data,
            board_side_length: self.board_side_length,
            shape: new_shape
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

pub struct BoardCoordsIterator<'a> {
    side: &'a BoardSideLength,
    counter: usize,
}

impl<'a> BoardCoordsIterator<'a> {
    pub fn new(side: &BoardSideLength) -> BoardCoordsIterator {
        BoardCoordsIterator {
            side,
            counter:0
        }
    }
}

impl<'a> Iterator for BoardCoordsIterator<'a> {
    type Item = Coordinates;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > self.side * self.side {
            None
        }
        else {
            Some((self.counter / self.side, self.counter % self.side))
        }
    }
}