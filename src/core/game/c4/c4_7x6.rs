use crate::core::game::c4::{board_trait::C4Board, cell_state::CellState};

pub type Board7By6 = [[CellState; 7]; 6];

impl C4Board for Board7By6 {
    fn new() -> Self {
        [[CellState::Vacant; 7]; 6]
    }

    fn coin(&mut self, coin: CellState, col: usize) -> Result<[usize; 2], ()> {
        for row in (0..6).rev() {
            if self[row][col] == CellState::Vacant {
                self[row][col] = coin;
                return Ok([col, row]);
            }
        }
        Err(())
    }

    fn check(&self, coin: CellState, pos: [usize; 2]) -> bool {
        let mut acc = 0u8;
        // Vertical check
        for row in self.iter() {
            if row[pos[1]] == coin {
                acc += 1;
                if acc == 4 {
                    return true;
                }
            } else {
                acc = 0;
            }
        }

        // Horizontal check
        acc = 0;
        for cell in self[pos[0]].iter() {
            if cell == &coin {
                acc += 1;
                if acc == 4 {
                    return true;
                }
            } else {
                acc = 0
            }
        }

        acc = 1;
        let coefficient = pos[0] as i8 - pos[1] as i8;
        if coefficient < 3 && coefficient > -4 {
            let mut j = pos[1];

            for i in (0..pos[0]).rev() {
                if j != 0 {
                    j -= 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            for i in (pos[0] + 1)..6 {
                if j != 6 {
                    j += 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if acc >= 4 {
                return true;
            }
        }

        acc = 1;
        let coefficient = pos[0] + pos[1];
        if coefficient > 2 && coefficient < 9 {
            let mut j = pos[1];
            for i in (0..pos[0]).rev() {
                if j != 6 {
                    j += 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            j = pos[1];
            for i in (pos[0] + 1)..6 {
                if j != 0 {
                    j -= 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if acc >= 4 {
                return true;
            }
        }

        false
    }

    fn dump(&self) -> String {
        let mut result = String::new();
        for i in self.into_iter() {
            for j in i {
                result = format!("{}{:?} ", result, *j as u8);
            }
            result.push('\n');
        }
        result.push('\n');
        result
    }
}
