#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Vacant,
    One,
    Two,
}

impl CellState {
    fn flip(self) -> CellState {
        match self {
            CellState::One => CellState::Two,
            CellState::Two => CellState::One,
            CellState::Vacant => self,
        }
    }
}
