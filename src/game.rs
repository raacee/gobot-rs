use crate::board::{Board, SIDE};
use crate::players::player::Player;
use crate::signals::*;
use crate::stones::{
    Stone, BLACK_STONE, BLACK_STONE_CHAR, EMPTY, EMPTY_CHAR, WHITE_STONE, WHITE_STONE_CHAR,
};
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;
use crate::players::human::Human;

const KO_LENGTH: usize = 2;

pub type Coordinates = (usize, usize);
pub type Move = Option<Coordinates>;

pub type Group = HashMap<Coordinates, Stone>;
pub type GroupDict = HashMap<&'static str, Group>;

#[derive(Clone, PartialEq)]
pub struct Game {
    board_size: usize,
    board: Board,
    last_boards: [Board; KO_LENGTH],
    players: [Rc<Box<dyn Player>>; 2],
    current_player: usize,
    display: bool,
    last_turned_passed: bool,
    komi: f32,
    is_over: bool,
}

impl Game {
    pub fn new(
        players: [Rc<Box<dyn Player>>; 2],
        display: bool,
        komi: f32,
    ) -> Self {
        let board = Board::new();
        let last_boards = [board.clone(), board.clone()];
        Game {
            board_size:SIDE,
            board,
            last_boards,
            players,
            current_player: 0,
            display,
            last_turned_passed: false,
            komi,
            is_over: false,
        }
    }

    pub fn from(
        board: Board,
        last_boards: [Board; KO_LENGTH],
        players: [Rc<Box<dyn Player>>; 2],
        display: bool,
        current_player: usize,
        last_turned_passed: bool,
        komi: f32,
        is_over: bool,
    ) -> Self {
        Game {
            board_size: SIDE,
            board,
            last_boards,
            players,
            current_player,
            display,
            last_turned_passed,
            komi,
            is_over,
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

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn get_current_player(&self) -> Rc<Box<dyn Player>> {
        Rc::clone(&self.players[self.current_player])
    }

    fn find_player(&self, stone: Stone) -> Rc<Box<dyn Player>> {
        Rc::clone(self.players
            .iter()
            .find(|player| player.get_stone() == stone)
            .unwrap()
        )
    }

    fn is_case_occupied(&self, (x, y): Coordinates) -> bool {
        self.get_board()[(x, y)] != EMPTY
    }

    fn check_ko(&self, board: &Board) -> bool {
        self.last_boards[self.current_player] == *board
    }

    pub fn game(&mut self) {
        if self.display{
            println!("{}", self);
        }
        loop {
            let current_player = self.get_current_player();
            let current_player_choice = current_player.choose_case(&self);
            let step_result = self.step(current_player_choice);

            match step_result {
                Ok(()) => continue,
                Err(e) => match e {
                    Signals::InducesSuicide
                    | Signals::OccupiedCase
                    | Signals::BreakingKo
                    | Signals::OutsideBounds => {
                        if self.display {
                            println!("{}", e)
                        }
                    }

                    Signals::GameOver | Signals::DoublePass => {
                        if self.display{
                            println!("{}", e);
                        }
                        self.is_over = true;
                        break;
                    }
                },
            }
        }
        let winner = self.winner();
        if self.display {
            match winner {
                Some(player) => {
                    println!("Winner: {}", player);
                }
                None => {
                    println!("Draw");
                }
            }
        }
    }

    pub fn step(&mut self, player_choice: Move) -> Result<(), Signals> {
        match player_choice {
            None => {
                if self.last_turned_passed {
                    self.is_over = true;
                    Err(Signals::DoublePass)
                } else {
                    self.last_turned_passed = true;
                    match self.current_player {
                        0 => self.current_player = 1,
                        1 => self.current_player = 0,
                        _ => panic!("Current player count is outside of bounds"),
                    }
                    if self.display{
                        println!("{}", self);
                    }
                    Ok(())
                }
            }

            Some(chosen_coords) => {
                let board_result = self.verify_player_choice(chosen_coords)?;
                self.last_boards[self.current_player] = board_result.clone();

                self.board = board_result;
                self.last_turned_passed = false;
                match self.current_player {
                    0 => self.current_player = 1,
                    1 => self.current_player = 0,
                    _ => panic!("Current player count is not in bounds"),
                }
                if self.display{
                    println!("{}", self);
                }
                Ok(())
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
        let captured_groups = self.groups_to_capture(player_choice);
        let induces_capture = captured_groups.len() > 0;

        if induces_suicide {
            if !induces_capture {
                return Err(Signals::InducesSuicide);
            }
        }

        let mut new_board = self.board.clone();
        new_board[player_choice] = self.get_current_player().get_stone();

        if induces_capture {
            Self::capture_groups(&mut new_board, captured_groups);
            if self.check_ko(&new_board) {
                Err(Signals::BreakingKo)
            } else {
                Ok(new_board)
            }
        } else {
            Ok(new_board)
        }
    }

    fn induces_suicide(&self, player_choice: Coordinates) -> bool {
        let mut test_board = self.board.clone();
        test_board[player_choice] = self.get_current_player().get_stone();
        let opposite_stone: Stone = Some(!self.get_current_player().get_stone().unwrap());
        let same_stone_group_dicts = Self::flood_fill(player_choice, &test_board, true, false);
        same_stone_group_dicts
            .get("border")
            .unwrap()
            .values()
            .all(|stone| *stone == opposite_stone)
    }

    fn groups_to_capture(&mut self, player_choice: Coordinates) -> Vec<GroupDict> {
        let mut test_board = self.board.clone();
        let player_stone = self.get_current_player().get_stone();
        test_board[player_choice] = player_stone;
        let neighbors = Self::neighbors_indices(player_choice, &test_board);
        let opposite_stone = Some(!self.get_current_player().get_stone().unwrap());
        let opposite_stone_neighbors_coords: Vec<Coordinates> = neighbors
            .into_iter()
            .filter(|&coordinates| test_board[coordinates] == opposite_stone)
            .collect();

        let mut opposite_stone_neighbors_group_dicts: Vec<GroupDict> =
            opposite_stone_neighbors_coords
                .into_iter()
                .map(|coordinates| Self::flood_fill(coordinates, &test_board, true, false))
                .collect();

        opposite_stone_neighbors_group_dicts
            .iter_mut()
            .for_each(|group_dict| self.mark_capture(group_dict));

        let opposite_stone_captured_groups = opposite_stone_neighbors_group_dicts
            .into_iter()
            .filter(|group_dict| group_dict.contains_key("captured"))
            .collect::<Vec<GroupDict>>();

        opposite_stone_captured_groups
    }

    fn mark_capture(&self, group_dict: &mut GroupDict) {
        let player_stone = self.get_current_player().get_stone();
        if group_dict
            .get("border")
            .unwrap()
            .values()
            .collect::<Vec<&Stone>>()
            .iter()
            .all(|stone_value| **stone_value == player_stone)
        {
            group_dict.insert("captured", HashMap::new());
        }
    }

    fn capture_groups(board: &mut Board, opposite_stone_neighbors_group_dicts: Vec<GroupDict>) {
        for group_dict in opposite_stone_neighbors_group_dicts {
            let captured_group = group_dict.get("group").unwrap();
            let captured_group_coords = captured_group.keys();
            for coords in captured_group_coords {
                board[*coords] = EMPTY
            }
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

    fn flood_fill(
        coords: Coordinates,
        board: &Board,
        add_border: bool,
        add_empty: bool,
    ) -> GroupDict {
        let group: HashMap<Coordinates, Stone> = HashMap::new();
        let border: HashMap<Coordinates, Stone> = HashMap::new();
        let mut group_dict: GroupDict = HashMap::new();
        group_dict.insert("group", group);
        group_dict.insert("border", border);
        let stone_of_origin = board[coords];

        let mut queue: VecDeque<Coordinates> = VecDeque::new();
        queue.push_back(coords);
        while !queue.is_empty() {
            let coords = queue.pop_front().unwrap();
            if group_dict["group"].contains_key(&coords)
                || group_dict["border"].contains_key(&coords)
            {
                continue;
            } else {
                match board[coords] {
                    EMPTY => {
                        // if flood fill is called on an empty case, it should behave like a normal call and
                        // add every adjacent cases if they are empty as well
                        if stone_of_origin == EMPTY {
                            group_dict
                                .get_mut("group")
                                .unwrap()
                                .insert(coords, stone_of_origin);
                            if add_border {
                                group_dict.get_mut("border").unwrap().insert(coords, EMPTY);
                            }
                        }
                        else if add_empty {
                            group_dict.get_mut("group").unwrap().insert(coords, EMPTY);
                            let neighbors = Self::neighbors_indices(coords, &board);
                            for neighbor in neighbors {
                                queue.push_back(neighbor);
                            }
                        } else if add_border {
                            group_dict.get_mut("border").unwrap().insert(coords, EMPTY);
                        }
                    }
                    some_stone => {
                        if some_stone == stone_of_origin {
                            group_dict
                                .get_mut("group")
                                .unwrap()
                                .insert(coords, some_stone);
                            let neighbors = Self::neighbors_indices(coords, &board);
                            for neighbor in neighbors {
                                queue.push_back(neighbor);
                            }
                        } else {
                            if add_border {
                                group_dict
                                    .get_mut("border")
                                    .unwrap()
                                    .insert(coords, some_stone);
                            }
                        }
                    }
                }
            }
        }
        group_dict
    }

    pub fn winner(&self) -> Option<Rc<Box<dyn Player>>> {
        let scores = self.calculate_scores();
        println!("Black score : {}", scores[&BLACK_STONE]);
        println!("White score : {}", scores[&WHITE_STONE]);
        if scores[&BLACK_STONE] > scores[&WHITE_STONE] {
            Some(self.find_player(BLACK_STONE))
        } else if scores[&WHITE_STONE] > scores[&BLACK_STONE] {
            Some(self.find_player(WHITE_STONE))
        } else {
            None
        }
    }

    fn calculate_scores(&self) -> HashMap<Stone, f32> {
        /*
        Area Scoring

        In area scoring, your score is:

        - The number of empty points which only your stones surround

        - The number of your stones on the board

        Area scoring is used in certain rulesets, notably Chinese Rules, AGA Rules and Ing Rules.

        To determine the score with area scoring, Chinese counting is generally used. An alternative method is Ing counting.

        Example: Assume each player has had 100 turns with no passes, this means they have played 100 stones each. At the end of the game there are 70 white stones surrounding 45 territory points, and 60 black stones surrounding 35 territory points.

        White's score is 70 + 45 = 115; black's score is 60 + 35 = 95; the margin of victory is 20 points to white.
        */
        let mut scores: HashMap<Stone, f32> = HashMap::from([
            (BLACK_STONE, self.number_stones(BLACK_STONE) as f32),
            (WHITE_STONE, self.komi + self.number_stones(WHITE_STONE) as f32),
        ]);

        for (i, row) in self.board.data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.is_none() {
                    match self.territory_owner((i,j)) {
                        Some((player, count)) => {
                            let player_stone = player.get_stone();
                            let current_score = scores[&player_stone];
                            scores.insert(player_stone, current_score + count as f32);
                        }
                        None => {
                            continue
                        }
                    }
                }
            }
        }
        scores
    }

    pub fn scores_difference(&self, scores: HashMap<Stone, f32>) -> f32 {
        scores[&BLACK_STONE] - scores[&WHITE_STONE]
    }

    pub fn calculate_scores_difference(&self) -> f32 {
        let scores = self.calculate_scores();
        scores[&BLACK_STONE] - scores[&WHITE_STONE]
    }

    fn territory_owner(&self, coords: Coordinates) -> Option<(Rc<Box<dyn Player>>, usize)> {
        if self.board[coords] != EMPTY {
            panic!("Cannot call territory_owner on an empty case")
        }

        let group_dict = Self::flood_fill(coords, &self.board, true, false);
        let border = group_dict.get("border").unwrap();
        if border.values().all(|neighbor| *neighbor == BLACK_STONE) {
            Some((
                Rc::clone(&self.find_player(BLACK_STONE)),
                group_dict.get("group").iter().len(),
            ))
        } else if border.values().all(|neighbor| *neighbor == WHITE_STONE) {
            Some((
                Rc::clone(&self.find_player(WHITE_STONE)),
                group_dict.get("group").iter().len(),
            ))
        } else {
            None
        }
    }

    fn eye_owner(&self, coords: Coordinates) -> Option<Stone> {
        if self.board[coords].is_some(){
            panic!("Did not call eye_owner on an empty case")
        }
        let neighbors = Self::neighbors_indices(coords, &self.board);
        let neighbors: Vec<Stone> = neighbors
            .into_iter()
            .map(|neighbor_coord| self.board[neighbor_coord])
            .collect();
        let mut neighbors_iter = neighbors.iter();
        if neighbors_iter.all(|neighbor| *neighbor == BLACK_STONE) {
            Some(BLACK_STONE)
        } else if neighbors_iter.all(|neighbor| *neighbor == WHITE_STONE) {
            Some(WHITE_STONE)
        } else {
            None
        }
    }

    fn is_eye(&self, coords: Coordinates) -> bool {
        self.eye_owner(coords).is_some()
    }

    fn group_owner(&self, coords: Coordinates) -> Option<Rc<Box<dyn Player>>> {
        let group_dict = Self::flood_fill(coords, &self.board, false, true);
        let group = &group_dict["group"];
        let mut n_eyes: u8 = 0;
        let group_tuples = group.into_iter().map(|coord_and_stone| coord_and_stone);
        for (coord, stone) in group_tuples {
            let is_eye = self.eye_owner(*coord);
            if *stone == EMPTY && is_eye.is_some() {
                n_eyes += 1;
                if n_eyes >= 2 {
                    let stone = is_eye.unwrap();
                    let player = self.find_player(stone);
                    return Some(player.clone());
                }
            }
        }
        None
    }

    fn is_alive(&self, coords: Coordinates) -> bool {
        self.group_owner(coords).is_some()
    }

    fn number_stones(&self, _s: Stone) -> f32 {
        let mut res = 0.0;
        for row in &self.board.data {
            for stone in row {
                match stone {
                    _s => res += 1.0,
                }
            }
        }
        res
    }

    pub fn available_cases(&self, player: Rc<Box<dyn Player>>) -> Vec<Move> {
        let mut available_cases: Vec<Move> = vec![];
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                if self.board[(i,j)].is_none() {
                    let induces_suicide = self.induces_suicide((i,j));
                    if !induces_suicide {
                        available_cases.push(Some((i,j)))
                    }
                }
            }
        }
        available_cases.push(None);
        available_cases
    }
}

impl Display for Game {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        println!("It's {} player turn\n", self.get_current_player());
        // Print each row with border
        for row in self.board.data.iter() {
            // Print each cell in the row
            for cell in row {
                let c = match *cell {
                    WHITE_STONE => WHITE_STONE_CHAR,
                    BLACK_STONE => BLACK_STONE_CHAR,
                    EMPTY => EMPTY_CHAR,
                    _ => panic!("Unknown value found"),
                };
                print!("|{}", c);
            }
            println!("|");
        }
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board_size:SIDE,
            board:Board::default(),
            last_boards: [Board::default(), Board::default()],
            players: [Rc::new(Box::new(Human::new(BLACK_STONE))), Rc::new(Box::new(Human::new(WHITE_STONE)))],
            current_player: 0,
            display: true,
            last_turned_passed: false,
            komi:7.5,
            is_over: false,
        }
    }
}
