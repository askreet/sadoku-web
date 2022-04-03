use GamePosIterKind::{Col, Row};

// Represents a position on a sudoku board, with zero-indexed row
// and column.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GamePos {
    pub col: i32,
    pub row: i32,
}

impl GamePos {
    pub fn at(row: i32, col: i32) -> Self {
        GamePos { row, col }
    }

    pub fn aligned_with(&self, other: GamePos) -> bool {
        other.col == self.col || other.row == self.row
    }

    pub fn iter_right(&self) -> impl Iterator<Item=GamePos> {
        GamePosIter { pos: *self, kind: Row, inc: true, skip: None }
    }

    pub fn iter_left(&self) -> impl Iterator<Item=GamePos> {
        GamePosIter { pos: *self, kind: Row, inc: false, skip: None }
    }

    pub fn iter_up(&self) -> impl Iterator<Item=GamePos> {
        GamePosIter { pos: *self, kind: Col, inc: false, skip: None }
    }

    pub fn iter_down(&self) -> impl Iterator<Item=GamePos> {
        GamePosIter { pos: *self, kind: Col, inc: true, skip: None }
    }

    pub fn iter_others_in_col(&self) -> impl Iterator<Item=GamePos> {
        // TODO: Starting at -1 is a hack.
        GamePosIter { pos: GamePos { col: self.col, row: -1 }, kind: Col, inc: true, skip: Some(*self) }
    }

    pub fn iter_others_in_row(&self) -> impl Iterator<Item=GamePos> {
        // TODO: Starting at -1 is a hack.
        GamePosIter { pos: GamePos { col: -1, row: self.row }, kind: Row, inc: true, skip: Some(*self) }
    }

    // pub fn iter_others_in_block(&self) -> impl Iterator<Item=GamePos> {
    //     BlockIter { block_id: self.block_id(), d_row: 0, d_col: 0, skip: Some(self.clone()) }
    // }

    pub fn conflict_candidates(&self) -> impl Iterator<Item=GamePos> {
        self.iter_others_in_col()
            .chain(self.iter_others_in_row())
    }
}


pub struct GamePosIter {
    kind: GamePosIterKind,
    pos: GamePos,
    inc: bool,
    skip: Option<GamePos>,
}

impl GamePosIter {
    fn step(&self) -> i32 {
        if self.inc {
            1
        } else {
            -1
        }
    }
}

struct BlockId(u8);

impl BlockId {
    fn of_pos(pos: &GamePos) -> BlockId {
        let inner: u8 = ((pos.col / 3) + ((pos.row / 3) * 3)) as u8;

        BlockId(inner)
    }

    fn origin(&self) -> GamePos {
        panic!("not implemented");
    }
}

// Iterates over all cells in a block, by block id.
struct BlockIter {
    // The origin of a block is it's top-left position.
    block_id: BlockId,
    next: u8,
}

impl BlockIter {
    fn containing(pos: &GamePos) -> Self {
        BlockIter {
            block_id: BlockId::of_pos(pos),
            next: 0,
        }
    }
}

impl Iterator for BlockIter {
    type Item = GamePos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 8 {
            None
        } else {
            let d_row = self.next / 3;
            let d_col = self.next % 3;

            self.next += 1;
            Some(GamePos {
                row: self.block_id.origin().row + d_row as i32,
                col: self.block_id.origin().col + d_col as i32,
            })
        }
    }
}

impl Iterator for GamePosIter {
    type Item = GamePos;

    fn next(&mut self) -> Option<Self::Item> {
        let end = if self.inc { 8 } else { 0 };

        match &self.kind {
            Row => if self.pos.col == end {
                None
            } else {
                self.pos.col += self.step();
                if self.skip == Some(self.pos) {
                    self.next()
                } else {
                    Some(self.pos)
                }
            },
            Col => if self.pos.row == end {
                None
            } else {
                self.pos.row += self.step();
                if self.skip == Some(self.pos) {
                    self.next()
                } else {
                    Some(self.pos)
                }
            },
            GamePosIterKind::Block => unimplemented!(),
        }
    }
}

enum GamePosIterKind {
    Row,
    Col,
    Block,
}

#[cfg(test)]
mod tests {
    use crate::pos::{BlockId, BlockIter, GamePos};

    #[test]
    fn test_iter_right() {
        let start = GamePos { row: 4, col: 2 };

        let actual: Vec<GamePos> = start.iter_right()
            .collect();

        assert_eq!(
            vec![
                GamePos { row: 4, col: 3 },
                GamePos { row: 4, col: 4 },
                GamePos { row: 4, col: 5 },
                GamePos { row: 4, col: 6 },
                GamePos { row: 4, col: 7 },
                GamePos { row: 4, col: 8 },
            ],
            actual
        );
    }

    #[test]
    fn test_iter_others_in_col() {
        let start = GamePos { row: 4, col: 2 };

        let actual: Vec<GamePos> = start.iter_others_in_col()
            .collect();

        assert_eq!(
            vec![
                GamePos { row: 0, col: 2 },
                GamePos { row: 1, col: 2 },
                GamePos { row: 2, col: 2 },
                GamePos { row: 3, col: 2 },
                GamePos { row: 5, col: 2 },
                GamePos { row: 6, col: 2 },
                GamePos { row: 7, col: 2 },
                GamePos { row: 8, col: 2 },
            ],
            actual
        );
    }

    #[test]
    fn test_block_id_of_pos() {
        let _start = GamePos { row: 1, col: 2 };

        let actual: Vec<u8> = pos_grid()
            .iter()
            .map(|p| BlockId::of_pos(p).0)
            .collect();

        assert_eq!(
            vec![
                0, 0, 0, 1, 1, 1, 2, 2, 2,
                0, 0, 0, 1, 1, 1, 2, 2, 2,
                0, 0, 0, 1, 1, 1, 2, 2, 2,
                3, 3, 3, 4, 4, 4, 5, 5, 5,
                3, 3, 3, 4, 4, 4, 5, 5, 5,
                3, 3, 3, 4, 4, 4, 5, 5, 5,
                6, 6, 6, 7, 7, 7, 8, 8, 8,
                6, 6, 6, 7, 7, 7, 8, 8, 8,
                6, 6, 6, 7, 7, 7, 8, 8, 8,
            ],
            actual
        );
    }

    #[test]
    fn test_block_iter() {
        let start = GamePos { row: 1, col: 2 };

        let actual: Vec<GamePos> = BlockIter::containing(&start).collect();

        assert_eq!(
            vec![
                GamePos { row: 0, col: 0 },
                GamePos { row: 0, col: 1 },
                GamePos { row: 0, col: 2 },
                GamePos { row: 1, col: 0 },
                GamePos { row: 1, col: 1 },
                GamePos { row: 1, col: 2 },
                GamePos { row: 2, col: 0 },
                GamePos { row: 2, col: 1 },
                GamePos { row: 2, col: 2 },
            ],
            actual
        )
    }

    #[test]
    fn test_conflict_candidates() {
        let start = GamePos { row: 4, col: 2 };

        let actual: Vec<GamePos> = start.conflict_candidates()
            .collect();

        assert!(actual.contains(&GamePos { row: 4, col: 8 })); // Same row.
        assert!(actual.contains(&GamePos { row: 8, col: 2 })); // Same column.
        assert!(actual.contains(&GamePos { row: 5, col: 1 })); // Same block.
    }

    fn pos_grid() -> Vec<GamePos> {
        let mut vec = Vec::new();
        for row in 0..=8 {
            for col in 0..=8 {
                vec.push(GamePos { row, col })
            }
        }
        vec
    }
}
