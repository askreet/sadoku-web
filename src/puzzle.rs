use crate::pos::GamePos;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    Clue(u8),
    // TODO: This could be optimized probably.
    Guess(u8),

    Pencilmarks([bool; 9]),
}

impl CellState {
    fn conflicts_with(self, other: CellState) -> bool {
        let my_value = match self {
            CellState::Clue(v) => v,
            CellState::Guess(v) => v,
            CellState::Pencilmarks(_) => return false,
        };

        let their_value = match other {
            CellState::Clue(v) => v,
            CellState::Guess(v) => v,
            CellState::Pencilmarks(_) => return false,
        };

        return my_value == their_value;
    }
}


// Describes the attributes of a cell that are essential to drawing it on the
// screen.
pub struct Cell {
    pub pos: GamePos,

    pub state: CellState,
    pub error: bool,
}

impl Cell {
    pub fn pos_at_index(idx: i32) -> GamePos {
        let row = idx / 9;
        let col = idx % 9;

        GamePos::at(row, col)
    }

    pub fn from_index(idx: i32, state: CellState, error: bool) -> Cell {
        Cell {
            pos: Self::pos_at_index(idx),
            state,
            error,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Puzzle {
    cells: [CellState; 9 * 9]
}

impl Puzzle {
    pub fn from(input: [u8; 9 * 9]) -> Puzzle {
        let mut cells = [CellState::Pencilmarks([false; 9]); 9 * 9];

        for (idx, value) in input.iter().enumerate() {
            if *value > 0 {
                cells[idx] = CellState::Clue(*value);
            }
        }

        Puzzle { cells }
    }

    pub fn is_error(&self, pos: GamePos) -> bool {
        let my_cell = self.state_at(&pos);

        pos.conflict_candidates()
            .any(|other| self.state_at(&other).conflicts_with(my_cell))
    }

    pub fn iter_cells(&self) -> CellIter {
        CellIter { puzzle: &self, index: 0 }
    }

    pub fn clear(&mut self, pos: &GamePos) {
        if let CellState::Clue(_) = self.state_at(pos) {
            return;
        }

        self.set_state_at(pos, CellState::Pencilmarks([false; 9]));
    }

    pub fn set_guess(&mut self, pos: &GamePos, candidate: u8) {
        if let CellState::Clue(_) = self.state_at(pos) {
            return;
        }

        // TODO: Preserve pencilmarks under guess for undo?
        self.set_state_at(pos, CellState::Guess(candidate));
    }

    pub fn toggle_candidate(&mut self, pos: &GamePos, candidate: usize) {
        if let CellState::Guess(_) = self.state_at(pos) {
            self.set_state_at(pos, CellState::Pencilmarks([false; 9]));
        }

        match self.state_at_mut(pos) {
            CellState::Clue(_) => {}
            CellState::Pencilmarks(candidates) => {
                let current_value = *candidates.get(candidate - 1).unwrap();

                candidates[candidate - 1] = !current_value;
            }
            CellState::Guess(_) => panic!("Should not be a Guess after attemping to set pencilmarks.")
        }
    }

    pub(crate) fn state_at(&self, pos: &GamePos) -> CellState {
        self.cells[Self::idx_of(pos)]
    }

    fn set_state_at(&mut self, pos: &GamePos, state: CellState) {
        self.cells[Self::idx_of(pos)] = state;
    }

    fn state_at_mut(&mut self, pos: &GamePos) -> &mut CellState {
        self.cells.get_mut(Self::idx_of(pos)).unwrap()
    }

    fn idx_of(pos: &GamePos) -> usize {
        ((pos.row * 9) + pos.col) as usize
    }
}

#[cfg(test)]
fn a_test_puzzle() -> Puzzle {
    Puzzle::from([
        // NYTimes Medium Jan 2, 2021
        0, 3, 0, 0, 1, 0, 0, 5, 4,
        0, 0, 0, 7, 8, 0, 0, 0, 3,
        7, 0, 2, 0, 0, 0, 0, 6, 0,
        4, 1, 0, 0, 5, 0, 0, 8, 0,
        0, 0, 3, 0, 0, 2, 9, 0, 0,
        0, 0, 0, 0, 0, 3, 0, 4, 6,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 5, 0, 4, 0, 0, 0, 0, 0,
        9, 0, 0, 0, 0, 0, 0, 3, 0,
    ])
}

#[test]
fn test_is_error_duplicate_guess_row() {
    let mut puzzle = a_test_puzzle();

    puzzle.set_guess(&GamePos::at(0, 0), 5);

    assert_eq!(true, puzzle.is_error(GamePos::at(0, 0)));
    assert_eq!(true, puzzle.is_error(GamePos::at(0, 7)));
}

pub struct CellIter<'a> {
    puzzle: &'a Puzzle,
    index: usize,
}

impl Iterator for CellIter<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let this_idx = self.index;

        self.index += 1;

        if this_idx <= 80 {
            let cell_state = self.puzzle.cells[this_idx];
            let cell = Cell::from_index(this_idx as i32, cell_state, self.puzzle.is_error(Cell::pos_at_index(this_idx as i32)));

            Some(cell)
        } else {
            None
        }
    }
}
