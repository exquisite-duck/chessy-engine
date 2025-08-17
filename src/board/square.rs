#[allow(unused_imports)]
use super::{FILES, RANKS};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square(pub usize);

impl Square {
    // we have the 0..63 squares so to get the rows we use the rank and for col we use the file
    pub fn rank(&self) -> usize {
        self.0 / FILES
    }
    pub fn file(&self) -> usize {
        self.0 % FILES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        let sq = Square(0);
        assert_eq!(sq.0, 0);
    }

    #[test]
    fn test_square_rank_file() {
        let sq = Square(0);
        assert_eq!(sq.rank(), 0);
        assert_eq!(sq.file(), 0);

        let sq = Square(7);
        assert_eq!(sq.rank(), 0);
        assert_eq!(sq.file(), 7);
    }
}