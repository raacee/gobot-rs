use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Error, Formatter};
use std::ptr::hash;
use std::rc::Rc;
use crate::board::{Board, BoardCoordsIterator, BoardSideLength};
use crate::players::player::Player;
use crate::signals::*;
use crate::stones::{
    Stone, BLACK_STONE, BLACK_STONE_CHAR, EMPTY, EMPTY_CHAR, WHITE_STONE, WHITE_STONE_CHAR,
};


pub type Coordinates = (usize, usize);
pub type Move = Option<Coordinates>;

pub type Group = HashMap<Coordinates, Stone>;
pub type GroupDict = HashMap<&'static str, Group>;

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
    is_over: bool,
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
            is_over: false,
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
        is_over: bool,
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

    pub fn is_over(&self) -> bool { self.is_over }

    pub fn get_current_player(&self) -> &Box<dyn Player> {
        self.players.get(0).unwrap()
    }

    fn find_player(&self, stone: Stone) -> &Rc<Box<dyn Player>> {
        self.players.iter().find(|player| {player.get_stone() == stone}).unwrap()
    }

    fn is_case_occupied(&self, (x, y): Coordinates) -> bool {
        self.get_board()[(x, y)] != EMPTY
    }

    fn check_ko(&self, board: &Board) -> bool {
        self.last_boards.contains(board)
    }

    pub fn game(&mut self) {
        loop {
            println!("{}", self);
            let current_player = self.get_current_player();
            let current_player_choice = current_player.choose_case(&self.board);
            let step_result = self.step(current_player_choice);

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
                            self.is_over = true;
                            break
                        },
                    }
                }
            }
        }
        let winner = self.winner();
        match winner {
            Some(player) => {
                println!("Winner: {}", player);
            }
            None => {
                println!("Draw");
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
                    Ok(())
                }
            }

            Some (chosen_coords) => {
                let board_result = self.verify_player_choice(chosen_coords)?;
                match self.current_player {
                    0 => self.current_player = 1,
                    1 => self.current_player = 0,
                    _ => panic!("Current player count is not in bounds"),
                }
                self.board = board_result;
                self.last_turned_passed = false;
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
        let captured_groups = self.captured_groups(player_choice);
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
            if self.last_boards.contains(&new_board){
                if self.super_ko {
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

    fn induces_suicide(&self, player_choice: Coordinates) -> bool {
        let mut test_board = self.board.clone();
        test_board[player_choice] = self.get_current_player().get_stone();
        let opposite_stone: Stone = Some(self.get_current_player().get_stone().unwrap() * -1);
        let same_stone_group_dicts = Self::flood_fill(player_choice, &test_board, true, false);
        same_stone_group_dicts
            .get("border")
            .unwrap()
            .values()
            .all(|stone| *stone == opposite_stone)
    }

    fn captured_groups(&mut self, player_choice: Coordinates) -> Vec<GroupDict> {
        let mut test_board = self.board.clone();
        test_board[player_choice] = self.get_current_player().get_stone();
        let neighbors = Self::neighbors_indices(player_choice, &test_board);
        let opposite_stone = Some(self.get_current_player().get_stone().unwrap() * -1);
        let opposite_stone_neighbors_coords: Vec<Coordinates> = neighbors
            .into_iter()
            .filter(|&coordinates| test_board[coordinates] == opposite_stone)
            .collect();

        let mut opposite_stone_neighbors_group_dicts: Vec<GroupDict> = opposite_stone_neighbors_coords
            .into_iter()
            .map(|coordinates| Self::flood_fill(coordinates, &test_board, true, false))
            .collect();

        opposite_stone_neighbors_group_dicts
            .iter_mut()
            .for_each(|group_dict| {Self::mark_capture(group_dict)});

        let opposite_stone_captured_groups = opposite_stone_neighbors_group_dicts
            .into_iter()
            .filter(|group_dict| {group_dict.contains_key("captured")})
            .collect::<Vec<GroupDict>>();

        opposite_stone_captured_groups
    }

    fn mark_capture(group_dict: &mut GroupDict) {
        if group_dict.get("border").unwrap().values().collect::<Vec<&Stone>>().contains(&&&None){
            group_dict.insert("captured", HashMap::new());
        }
        else{
            group_dict.insert("captured", HashMap::new());
        }
    }

    fn capture_groups(board:&mut Board, opposite_stone_neighbors_group_dicts: Vec<GroupDict>) {
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

    fn flood_fill(coords: Coordinates, board: &Board, add_border: bool, add_empty: bool) -> GroupDict {
        let group: HashMap<Coordinates, Stone> = HashMap::new();
        let border: HashMap<Coordinates, Stone> = HashMap::new();
        let mut group_dict: GroupDict = HashMap::new();
        group_dict.insert("group", group);
        group_dict.insert("border", border);
        let current_stone = board[coords];

        let mut queue: VecDeque<Coordinates> = VecDeque::new();
        queue.push_back(coords);
        while !queue.is_empty() {
            let coords = queue.pop_front().unwrap();
            if !group_dict["group"].contains_key(&coords)
                || !group_dict["border"].contains_key(&coords)
            {
                match board[coords] {
                    None => {
                        if add_empty {
                            group_dict.get_mut("group").unwrap().insert(coords, EMPTY);
                            let neighbors = Self::neighbors_indices(coords, &board);
                            for neighbor in neighbors {
                                queue.push_back(neighbor);
                            }
                        }
                        if add_border {
                            group_dict.get_mut("border").unwrap().insert(coords, EMPTY);
                        }
                    }
                    some_stone => {
                        if some_stone == current_stone {
                            group_dict.get_mut("group").unwrap().insert(coords, some_stone);
                            let neighbors = Self::neighbors_indices(coords, &board);
                            for neighbor in neighbors {
                                queue.push_back(neighbor);
                            }
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

    fn winner(&self) -> Option<Rc<Box<dyn Player>>> {
        let scores = self.calculate_scores();
        if scores[&self.players[0]] > scores[&self.players[1]] {
            Some(self.players[0].clone())
        }
        else if scores[&self.players[1]] < scores[&self.players[0]] {
            Some(self.players[1].clone())
        }
        else {
            None
        }
    }

    fn calculate_scores(&self) -> HashMap<Rc<Box<dyn Player>>, f32> {
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
        let mut hashmap: HashMap<Rc<Box<dyn Player>>, f32> = HashMap::from([
            (self.players[0].clone(), 0.0),
            (self.players[1].clone(), self.komi)
        ]);

        for (i,row) in self.board.data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match *cell {
                    BLACK_STONE => {
                        let black_player = self.find_player(BLACK_STONE);
                        let old_value = hashmap.get(black_player).unwrap();
                        hashmap.insert(black_player.clone(), old_value + 1.0);
                    }
                    WHITE_STONE => {
                        let white_player = self.find_player(BLACK_STONE);
                        let old_value = hashmap.get(white_player).unwrap();
                        hashmap.insert(white_player.clone(), old_value + 1.0);
                    }
                    None => {
                        match self.territory_owner((i,j)) {
                            Some((player_owner, count)) => {
                                let old_value = hashmap.get(&player_owner).unwrap();
                                hashmap.insert(player_owner, old_value + count as f32);
                            }
                            _ => continue
                        }
                    }
                    _ => {
                        panic!("Value other than -1 or 1 is found in board")
                    }
                }
            }
        }
        hashmap
    }

    fn territory_owner(&self, coords: Coordinates) -> Option<(Rc<Box<dyn Player>>, usize)> {
        let group_dict = Self::flood_fill(coords, &self.board, true, false);
        let border = group_dict.get("border").unwrap();

        if  border.values().all(|neighbor| *neighbor == BLACK_STONE) {
            Some((self.find_player(BLACK_STONE).clone(), group_dict.get("group").iter().len()))
        }
        else if border.values().all(|neighbor| *neighbor == WHITE_STONE) {
            Some((self.find_player(WHITE_STONE).clone(), group_dict.get("group").iter().len()))
        }
        else {
            None
        }

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

    fn group_owner(&self, coords:Coordinates) -> Option<Rc<Box<dyn Player>>> {
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
                if self.eye_owner((i,j)).unwrap() == player.get_stone() {
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
        println!("Board ({}x{}):", self.width(), self.height());
        // Print each row with border
        for (i, row) in self.board.data.iter().enumerate() {
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

        // Print players
        println!("Players: {} vs {}", self.players[0], self.players[1]);

        // Print other game info
        println!("Super Ko: {}", self.super_ko);
        println!("Komi: {}", self.komi);
        println!("Last turn passed: {}", self.last_turned_passed);

        Ok(())
    }
}
