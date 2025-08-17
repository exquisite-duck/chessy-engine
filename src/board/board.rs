use crate::board::piece::Piece;
use super::BOARD_SIZE;

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: [Option<Piece>; BOARD_SIZE],
}

impl Board {
    pub fn empty() -> Self {
        Board {
            squares: [None; BOARD_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_board() {
        let b = Board::empty();
        assert!(b.squares.iter().all(|sq| sq.is_none()));
    }
}