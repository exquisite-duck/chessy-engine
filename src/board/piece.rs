#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_piece_equality() {
        assert_eq!(Piece::Pawn, Piece::Pawn);
        assert_ne!(Piece::Knight, Piece::Bishop);
    }
}