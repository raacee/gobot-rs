use crate::stones::{get_stone_name_from_stone, Stone};
use std::fmt::{Display, Error, Formatter, Result};
use crate::board::Board;
use crate::game::{Coordinates, Move};
use std::io::stdin;

pub trait Player {
    fn choose_case(&self, board: &Board) -> Move;
}

pub struct Human {
    pub stone: Stone,
    pub name: &'static str,
}

impl Human {
    pub fn new(stone: Stone) -> Human {
        Human {
            stone,
            name: get_stone_name_from_stone(stone),
        }
    }
}

impl Player for Human {
    fn choose_case(&self, board: &Board) -> Move {
        let mut input = String::new();
        print!("{} player choice", self.name);
        let stdin = stdin();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let player_choice = input
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>();
                match player_choice.len() {
                    0 => None,
                    2 => {
                        let mut coords: (usize, usize) = (0, 0);
                        let player_choice_coords: Vec<usize> = player_choice
                        .into_iter()
                        .map(|c| c.parse::<usize>().unwrap()).collect();
                        (coords.0, coords.1) = (player_choice_coords[0], player_choice_coords[1]);
                        Some(coords)
                    }
                    _ => panic!("You must provide a valid move")
                }

            }
            Err(_) => panic!("Failed to read input"),
        }

    }
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

