pub type Stone = Option<i8>;

const BLACK: i8 = -1;
const WHITE: i8 = 1;

pub const BLACK_STONE: Stone = Some(-1);
pub const WHITE_STONE: Stone = Some(1);
pub const EMPTY: Stone = None;

// pub const BLACK_STR: char = 'b';
// pub const WHITE_STR: char = 'w';
pub const WHITE_STONE_CHAR: &str = "●";
pub const BLACK_STONE_CHAR: &str = "○";
pub const EMPTY_CHAR: &str = " ";
pub const BLACK_NAME: &str = "Black";
pub const WHITE_NAME: &str = "White";
const PASS_NAME: &str = "PASS";

pub fn get_stone_name_from_stone(stone: Stone) -> &'static str {
    match stone {
        Some(stone_color) => {
            if stone_color == BLACK {
                BLACK_NAME
            } else if stone_color == WHITE {
                WHITE_NAME
            } else { panic!("Unknown stone") }
        }
        None => PASS_NAME,
    }
}