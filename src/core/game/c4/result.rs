#[derive(Debug)]
pub enum GameResult {
    // Player A beats player B
    Win,
    // Player A and B tie
    Loose,
    // Player A looses to player B
    Tie,
}

impl GameResult {
    /// Get respective probabilities for player A and player B
    pub fn get_rep(&self) -> (f32, f32) {
        match self {
            GameResult::Win => (1.0, 0.0),
            GameResult::Loose => (0.0, 1.0),
            GameResult::Tie => (0.5, 0.5),
        }
    }
}

