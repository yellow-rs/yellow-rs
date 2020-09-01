use super::raw_board::RawBoard;
use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct C4Board {
    raw: RawBoard,
    connections: u8,
}

impl C4Board {
    pub fn new([length, width]: [usize; 2], connections: u8) -> Self {
        C4Board {
            raw: RawBoard::new(length, width),
            connections,
        }
    }

    /// Validates a position and returns it
    pub fn coin(&mut self, coin: u8, col: usize) -> Option<[usize; 2]> {
        for row in 0..self.dimensions[1] {
            let index = self.flatten(&[col, row]);
            if self[index] == 0 {
                self[index] = coin;
                println!("{}\n", self.raw);
                return Some([col, row]);
            }
        }
        None
    }

    // Checks a position if it constitutes a full connection
    fn check(&self, [x, y]: [usize; 2], coin: u8) -> bool {
        let mut acc = 0u8;
        // Row check
        for x in self.range_width() {
            if self[self.flatten(&[x, y])] == coin {
                acc += 1;
                if acc == self.connections {
                    return true;
                }
            } else {
                acc = 0;
            }
        }

        acc = 0;
        // Column check
        for y in self.range_height() {
            if self[self.flatten(&[x, y])] == coin {
                acc += 1;
                if acc == self.connections {
                    return true;
                }
            } else {
                acc = 0;
            }
        }

        false
    }
}

impl Deref for C4Board {
    type Target = RawBoard;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl DerefMut for C4Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl Display for C4Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn c4_test() {
    let mut b = C4Board {
        raw: RawBoard::new(7, 6),
        connections: 4,
    };
    b.coin(1, 3);
    println!("{}", b.check([3, 0], 1));
    b.coin(1, 2);
    println!("{}", b.check([2, 0], 1));
    b.coin(1, 1);
    println!("{}", b.check([1, 0], 1));
    b.coin(1, 0);
    println!("{}", b.check([0, 0], 1));
    println!("{}", *b);
}
