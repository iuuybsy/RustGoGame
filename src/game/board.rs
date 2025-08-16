use crate::game::game_const::BOARD_SIZE;
use crate::game::stone::Occupy;

#[derive(Debug)]
pub enum BoardError {
    PositionOccupied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

impl BoardPosition {
    pub fn new(x_: usize, y_: usize) -> Option<Self> {
        (x_ < BOARD_SIZE && y_ < BOARD_SIZE).then_some(Self { x: x_, y: y_ })
    }
}

pub struct Board {
    grid: [[Occupy; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Occupy::Free; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn set_stone(&mut self, pos: BoardPosition, stone_type: Occupy) -> Result<(), BoardError> {
        if self.grid[pos.x][pos.y] != Occupy::Free {
            return Err(BoardError::PositionOccupied);
        }
        self.grid[pos.x][pos.y] = stone_type;
        Ok(())
    }

    pub fn remove_stone(&mut self, pos: BoardPosition) {
        self.grid[pos.x][pos.y] = Occupy::Free;
    }

    pub fn get(&self, pos: &BoardPosition) -> Occupy {
        self.grid[pos.x][pos.y]
    }

    pub fn get_board_string(&self) -> String {
        let mut s = String::with_capacity(BOARD_SIZE * BOARD_SIZE);
        for row in self.grid {
            for cell in row {
                s.push(match cell {
                    Occupy::Free => '0',
                    Occupy::Black => '1',
                    Occupy::White => '2',
                });
            }
        }
        s
    }
}
