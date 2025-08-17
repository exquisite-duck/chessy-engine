pub mod square;
pub mod piece;
pub mod board;


pub const BOARD_SIZE: usize = 64; // as 8 * 8 squares
pub const RANKS: usize = 8;
pub const FILES: usize = 8;

// here ranks and files are like
// rows(1 -> 8)
// columns (a -> h)
// so like knight moves like (e5) something like that