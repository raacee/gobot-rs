use std::cell::RefCell;
use std::io::Write;
use crate::board::Board;
use crate::game::Move;
use crate::players::player::Player;
use crate::stones::{get_stone_name_from_stone, Stone};

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

impl  Human {
    fn get_user_input(&self) -> String {
        let mut input = String::new();
        let stdin = std::io::stdin();
        let choice_result = stdin.read_line(&mut input);
        match choice_result {
            Ok(_) => {
                input
            }
            Err(_) => {
                panic!("failed to read input");
            }
        }

    }
}

impl Player for Human {
    fn get_stone(&self) -> Stone {
        self.stone
    }

    fn choose_case(&self, board: &Board) -> Move {
        let mut user_input = self.get_user_input();
        let choice = user_input
            .trim()
            .split(' ')
            .map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        match choice.len() {
            0 =>{
                let res = None;
                user_input.clear();
                res
            }
            2 => {
                let res = Some((choice[0], choice[1]));
                user_input.clear();
                res
            }
            _ => panic!("Input is not of length 0 or 2")
        }
    }
}
