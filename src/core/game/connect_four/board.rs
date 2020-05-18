#[derive(Clone, Copy, Debug)]
enum CellState {
    Vacant,
    One,
    Two,
}

pub struct Board([CellState; 42], usize, usize);

enum Move {
    Valid,
    Invalid,
}

impl Board {
    pub fn new(col: usize, row: usize) -> Board {
        Board([CellState::Vacant; 42], col, row)
    }

    fn dump(&self) {
        for cells in self.0.iter() {
            println!("{:?}", cells);
        }
    }

    fn coin(&mut self, col: usize, player: CellState) -> Move {
        Move::Invalid
    }
}
