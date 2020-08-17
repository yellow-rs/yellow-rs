pub trait C4Board {
    fn new() -> Self;
    fn coin(&mut self, coin: CellState, col: usize) -> Result<[usize; 2], ()>;
    fn check(&self, coin: CellState, pos: [usize; 2]) -> bool;
    fn dump(&self) -> String;
}
