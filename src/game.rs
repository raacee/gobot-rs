use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;
use crate::board::{Board, BoardCoordsIterator, BoardSideLength};
use crate::players::player::Player;
use crate::signals::*;
use crate::stones::{
    Stone, BLACK_STONE, BLACK_STONE_CHAR, EMPTY, EMPTY_CHAR, WHITE_STONE, WHITE_STONE_CHAR,
};


pub type Coordinates = (usize, usize);
pub type Move = Option<Coordinates>;

pub type Group<'a> = HashMap<Coordinates, &'a Stone>;
pub type GroupDict<'a> = HashMap<&'static str, Group<'a>>;

#[derive(Clone)]
pub struct Game {
    board_size: BoardSideLength,
    board: Board,
    super_ko: bool,
    last_boards: Vec<Board>,
    players: [Rc<Box<dyn Player>>; 2],
    current_player: usize,
    display: bool,
    last_turned_passed: bool,
    komi: f32,
}

impl Game{
    pub fn new(
        board_size: BoardSideLength,
        super_ko: bool,
        players:  [Rc<Box<dyn Player>>; 2],
        display: bool,
        last_turned_passed: bool,
        komi: f32,
    ) -> Self {
        let board = Board::new(board_size);
        let last_boards = Vec::new();
        Game {
            board_size,
            board,
            super_ko,
            last_boards,
            players,
            current_player: 0,
            display,
            last_turned_passed,
            komi,
        }
    }

    pub fn from(
        board_size: BoardSideLength,
        board: Board,
        super_ko: bool,
        last_boards: Vec<Board>,
        players:  [Rc<Box<dyn Player>>; 2],
        display: bool,
        current_player: usize,
        last_turned_passed: bool,
        komi: f32,
    ) -> Self {
        Game {
            board_size,
            board,
            super_ko,
            last_boards,
            players,
            current_player,
            display,
            last_turned_passed,
            komi,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Stone {
        &self.board[(x, y)]
    }

    pub fn width(&self) -> &usize {
        &self.board.shape.0
    }

    pub fn height(&self) -> &usize {
        &self.board.shape.1
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_current_player(&self) -> &Box<dyn Player> {
        self.players.get(0).unwrap()
    }

    fn is_case_occupied(&self, (x, y): Coordinates) -> bool {
        self.get_board()[(x, y)] != EMPTY
    }

    fn check_ko(&self, board: &Board) -> bool {
        self.last_boards.contains(board)
    }

    pub fn game(&mut self) {
        loop {
            let current_player = self.get_current_player();
            let step_result = self.step(current_player.choose_case(&self.board));
            match step_result {
                Ok(()) => continue,
                Err(e) => {
                    match e {
                        Signals::InducesSuicide |
                        Signals::OccupiedCase |
                        Signals::BreakingKo |
                        Signals::BreakingSuperKo |
                        Signals::OutsideBounds => println!("{}", e),

                        Signals::GameOver |
                        Signals::DoublePass => {
                            println!("{}", e);
                            break
                        },
                    }
                }
            }
        }

        let winner = self.winner();
    }

    pub fn step(&mut self, player_choice: Move) -> Result<(), Signals> {
        if self.display {
            let current_player = self.get_current_player();
            println!("{}", self);
            println!("Player {current_player}'s turn");
        }

        match player_choice {
            None => {
                if self.last_turned_passed {
                    Err(Signals::DoublePass)
                } else {
                    self.last_turned_passed = true;
                    match self.current_player {
                        0 => self.current_player = 1,
                        1 => self.current_player = 0,
                        _ => panic!("Current player count is outside of bounds"),
                    }
                    Ok(())
                }
            }

            Some (chosen_coords) => {
                let turn_result = self.verify_player_choice(chosen_coords);
                match turn_result {
                    Ok(new_board) => {
                        self.board = new_board;
                        self.last_turned_passed = false;
                        Ok(())
                    }
                    Err(signal) => {
                        Err(signal)
                    }
                }
            }
        }

    }

    fn verify_player_choice(&mut self, player_choice: Coordinates) -> Result<Board, Signals> {
        let (x, y) = player_choice;
        if x >= self.board_size || y >= self.board_size {
            return Err(Signals::OutsideBounds);
        }
        if self.is_case_occupied(player_choice) {
            return Err(Signals::OccupiedCase);
        }

        let induces_suicide = self.induces_suicide(player_choice);
        let (induces_capture, new_board_option) = self.induces_capture(player_choice);

        if induces_suicide {
            if !induces_capture {
                return Err(Signals::InducesSuicide);
            }
        }
        let new_board = new_board_option.unwrap();
        if induces_capture {
            if self.last_boards.contains(&new_board){
                return if self.super_ko {
                    Err(Signals::BreakingSuperKo)
                } else {
                    Err(Signals::BreakingKo)
                }
            }
            else {
                Ok(new_board)
            }
        }
        else {
            Ok(new_board)
        }
    }

    fn neighbors_indices((x, y): Coordinates, board: &Board) -> Vec<Coordinates> {
        let mut neighbors: Vec<Coordinates> = vec![];
        let (height, width) = board.shape;
        if x != 0 {
            neighbors.push((x - 1, y))
        }
        if x != height - 1 {
            neighbors.push((x + 1, y))
        }
        if y != 0 {
            neighbors.push((x, y - 1))
        }
        if y != width - 1 {
            neighbors.push((x, y + 1))
        }
        neighbors
    }

    fn flood_fill(coords: Coordinates, board: &Board, add_border: bool, add_empty: bool) -> GroupDict {
        let group: HashMap<Coordinates, &Stone> = HashMap::new();
        let border: HashMap<Coordinates, &Stone> = HashMap::new();
        let mut group_dict: GroupDict = HashMap::new();
        group_dict.insert("group", group);
        group_dict.insert("border", border);
        let current_stone = board.get(coords);

        let mut queue: VecDeque<Coordinates> = VecDeque::new();
        queue.push_back(coords);

        while !queue.is_empty() {
            let coords = queue.pop_front().unwrap();
            if !group_dict["group"].contains_key(&coords)
                || !group_dict["border"].contains_key(&coords)
            {
                match board.get(coords) {
                    None => {
                        if add_empty {
                            queue.push_back(coords);
                        }
                    }
                    some_stone => {
                        if some_stone == current_stone {
                            group_dict.get_mut("group").unwrap().insert(coords, some_stone);
                            queue.push_back(coords);
                        }
                        else {
                            if add_border {
                                group_dict.get_mut("group").unwrap().insert(coords, some_stone);
                            }
                        }
                    }
                }
            }
        }
        group_dict
    }

    fn mark_capture(group_dict: &mut GroupDict) {
        if group_dict.get("border").unwrap().values().collect::<Vec<&&Stone>>().contains(&&&None){
            group_dict.insert("captured", HashMap::new());
        }
        else{
            group_dict.insert("captured", HashMap::new());
        }
    }

    fn induces_suicide(&self, player_choice: Coordinates) -> bool {
        let mut test_board = self.board.clone();
        test_board[player_choice] = *self.get_current_player().get_stone();
        let same_stone_group_dicts = Self::flood_fill(player_choice, &test_board, true, false);
        same_stone_group_dicts
            .get("border")
            .unwrap()
            .values()
            .all(|stone| {
                match **stone {
                    None => true,
                    Some(_) => false,
                }
            })
    }

    fn induces_capture(&self, player_choice: Coordinates) -> (bool, Option<Board>) {
        let neighbors = Self::neighbors_indices(player_choice, &self.board);
        let opposite_stone_color = self.get_current_player().get_stone().unwrap() * -1;
        let opposite_stone_neighbors_coords: Vec<Coordinates> = neighbors
            .into_iter()
            .filter(|&coordinates| self.board.get(coordinates).unwrap() == opposite_stone_color)
            .collect();
        let mut opposite_stone_neighbors_group_dicts: Vec<GroupDict> = opposite_stone_neighbors_coords
            .into_iter()
            .map(|coordinates| Self::flood_fill(coordinates, &self.board, true, false))
            .collect();

        opposite_stone_neighbors_group_dicts
            .iter_mut()
            .for_each(|group_dict| {Self::mark_capture(group_dict)});

        let opposite_stone_captured_groups = opposite_stone_neighbors_group_dicts
            .into_iter()
            .filter(|group_dict| {group_dict.contains_key("captured")})
            .collect::<Vec<GroupDict>>();

        let induces_capture = opposite_stone_captured_groups.len() > 0;

        if induces_capture {
            let new_board = self.capture_groups(opposite_stone_captured_groups);
            (induces_capture, Some(new_board))
        }
        else {
            (induces_capture, None)
        }
    }

    fn capture_groups(&self, opposite_stone_neighbors_group_dicts: Vec<GroupDict>) -> Board{
        let mut new_board = self.board.clone();
        for group_dict in opposite_stone_neighbors_group_dicts {
            let captured_group = group_dict.get("group").unwrap();
            let captured_group_coords = captured_group.keys();
            for coords in captured_group_coords {
                new_board[*coords] = EMPTY
            }
        }
        new_board
    }

    fn winner(&self) -> &dyn Player {
        todo!()
    }

    fn calculate_scores(&self) -> HashMap<&dyn Player, u8> {
        todo!()
    }

    fn eye_owner(&self, coords:Coordinates) -> Option<Stone> {
        let neighbors = Self::neighbors_indices(coords, &self.board);
        let neighbors: Vec<Stone> = neighbors.into_iter().map(|neighbor_coord| self.board[neighbor_coord]).collect();
        if  neighbors.iter().all(|neighbor| *neighbor == BLACK_STONE) {
            Some(BLACK_STONE)
        }
        else if neighbors.iter().all(|neighbor| *neighbor == WHITE_STONE) {
            Some(WHITE_STONE)
        }
        else {
            None
        }
    }

    fn group_owner(&self, coords:Coordinates) -> Option<&Rc<Box<dyn Player>>> {
        let group_dict = Self::flood_fill(coords, &self.board, false, true);
        let group = &group_dict["group"];
        let mut n_eyes: u8 = 0;
        let group_tuples = group.into_iter().map(|coord_and_stone| coord_and_stone);
        for (coord, stone) in group_tuples {
            let is_eye = self.eye_owner(*coord);
            if **stone == EMPTY && is_eye.is_some() {
                n_eyes += 1;
                if n_eyes >= 2 {
                    let stone = is_eye.unwrap();
                    let player = self.players.iter().find(|player| {*player.get_stone() == stone});
                    if player.is_none() {
                        panic!("No corresponding player found");
                    }
                    return player;
                }
            }
        }
        None
    }

    fn number_stones(&self, s: Stone) -> u8 {
        let mut res = 0;
        for row in &self.board.data {
            for stone in row {
                match stone {
                    s => res += 1
                }
            }
        }
        res
    }

    pub fn available_cases(&self, player: &Box<dyn Player>) -> Vec<Move> {
        let mut available_cases: Vec<Move> = vec![];
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                if self.eye_owner((i,j)).unwrap() == *player.get_stone() {
                    available_cases.push(Some((i,j)));
                }
            }
        }
        available_cases.push(None);
        available_cases
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // Print the board
        writeln!(f, "Board ({}x{}):", self.width(), self.height())?;
        // Print each row with border
        for (i, row) in self.board.data.iter().enumerate() {
            // Print row number
            write!(f, "{:2} ", i + 1)?;

            // Print each cell in the row
            for cell in row {
                let c = match *cell {
                    WHITE_STONE => WHITE_STONE_CHAR,
                    BLACK_STONE => BLACK_STONE_CHAR,
                    EMPTY => EMPTY_CHAR,
                    _ => panic!("Unknown value found"),
                };
                write!(f, "|{}", c)?;
            }
            writeln!(f, "|")?;
        }

        // Print players
        writeln!(f, "Players: {} vs {}", self.players[0], self.players[1])?;

        // Print other game info
        writeln!(f, "Super Ko: {}", self.super_ko)?;
        writeln!(f, "Komi: {}", self.komi)?;
        writeln!(f, "Last turn passed: {}", self.last_turned_passed)?;

        Ok(())
    }
}
