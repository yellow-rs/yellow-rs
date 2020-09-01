use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut, Range},
};

#[derive(Debug)]
pub struct RawBoard {
    pub data: Vec<u8>,
    pub dimensions: [usize; 2],
}

impl RawBoard {
    pub fn range_width(&self) -> Range<usize> {
        0..self.dimensions[0]
    }

    pub fn range_height(&self) -> Range<usize> {
        0..self.dimensions[1]
    }

    // Flattens an xy coordinate for its array
    pub fn flatten(&self, xy: &[usize; 2]) -> usize {
        xy[0] + self.dimensions[0] * xy[1]
    }
    // Checks if a coordinate is within bounds of the array dimensions
    fn within_bounds(&self, xy: &[usize; 2]) -> bool {
        xy[0] % self.dimensions[0] == xy[0] && xy[1] % self.dimensions[1] == xy[1]
    }

    pub fn new(length: usize, width: usize) -> Self {
        Self {
            data: vec![0; length * width],
            dimensions: [length, width],
        }
    }
}

impl Deref for RawBoard {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for RawBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Display for RawBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, cell) in self.iter().enumerate() {
            let _ = write!(f, "{} ", cell);
            if (i + 1) % self.dimensions[0] == 0 {
                let _ = writeln!(f);
            }
        }
        Ok(())
    }
}

#[test]
// Index accuracy
fn raw_board_ops_1() {
    for y in 1..64 {
        for x in 1..64 {
            _raw_board_ops_1(RawBoard::new(x, y));
        }
    }
}

fn _raw_board_ops_1(board: RawBoard) {
    let mut i = 0;
    for y in 0..board.dimensions[1] {
        for x in 0..board.dimensions[0] {
            assert_eq!(board.flatten(&[x, y]), i);
            i += 1;
        }
    }
}

#[test]
// Coordinate validity
fn raw_board_ops_2() {
    for y in 1..10 {
        for x in 1..10 {
            _raw_board_ops_2(RawBoard::new(x, y));
        }
    }
}

fn _raw_board_ops_2(board: RawBoard) {
    for y in 0..board.dimensions[1] {
        for x in 0..board.dimensions[0] {
            assert!(board.within_bounds(&[x, y]));
        }
    }

    for y in board.dimensions[1]..10 {
        for x in board.dimensions[0]..10 {
            assert!(!board.within_bounds(&[x, y]));
        }
    }
}
