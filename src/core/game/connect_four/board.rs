#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Vacant,
    Player,
}

pub trait C4Board {
    fn new() -> Self;
    fn dump(&self);
    fn coin(&mut self, col: usize, player: CellState);
    fn is_over(&self) -> bool;
    fn check(&self, row: usize, col: usize, player: CellState) -> usize;
}

pub struct Board([CellState; 42], bool);

impl C4Board for Board {
    fn new() -> Board {
        Board([CellState::Vacant; 42], false)
    }
    fn dump(&self) {
        let mut counter = 0u8;
        for cell in self.0.iter() {
            print!("{:?} ", cell);
            counter += 1;
            if counter > 6 {
                println!("");
                counter = 0;
            }
        }
        println!("");
    }
    fn coin(&mut self, col: usize, player: CellState) {
        for i in (0..6).rev() {
            if self.0[find_pos(i, col)] == CellState::Vacant {
                self.0[find_pos(i, col)] = player;
                let _ = self.check(i, col, player);
                break;
            }
        }
    }
    fn is_over(&self) -> bool {
        self.1
    }
    fn check(&self, row: usize, col: usize, player: CellState) -> usize {
        let mut n_s = 1usize;
        //let mut w_e = 1usize;
        //let mut nw_se = 1usize;
        //let mut ne_sw = 1usize;

        let mut iterate = row;
        {
            while iterate != 5 {
                iterate += 1;
                if self.0[find_pos(col, iterate)] == player {
                    n_s += 1;
                    continue;
                }
                break;
            }

            iterate = row;
            while iterate != 0 {
                iterate -= 1;
                if self.0[find_pos(col, iterate)] == player {
                    n_s += 1;
                    continue;
                }
                break;
            }
            n_s
        }
    }
}

fn find_pos(col: usize, row: usize) -> usize {
    col + (row * 7) - 1
}

// 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0

// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0

// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0
