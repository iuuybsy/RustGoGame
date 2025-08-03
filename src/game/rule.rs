use crate::game::board::{Board, BoardPosition};
use crate::game::game_const::{BOARD_SIZE, SEARCH_DIRECTIONS};
use crate::game::stone::Occupy;
use std::collections::VecDeque;

trait SafeSetChar {
    fn set_char(&mut self, idx: usize, c: char) -> bool;
}

impl SafeSetChar for String {
    fn set_char(&mut self, idx: usize, c: char) -> bool {
        let bytes = self.as_bytes();
        let new_len = c.len_utf8();

        if !self.is_char_boundary(idx) || bytes.len().saturating_sub(idx) < new_len {
            return false;
        }

        unsafe {
            let slice = self.as_bytes_mut();
            c.encode_utf8(&mut slice[idx..idx + new_len]);
        }
        true
    }
}

#[derive(Debug)]
pub enum RuleError {
    WrongPlayerSide,
}

pub struct Rule {
    current_player: Occupy,
    visited: [[bool; BOARD_SIZE]; BOARD_SIZE],
    board_history: VecDeque<String>,
    turn_history: VecDeque<Occupy>,
}

impl Rule {
    pub fn new() -> Self {
        Rule {
            current_player: Occupy::Black,
            visited: [[false; BOARD_SIZE]; BOARD_SIZE],
            board_history: VecDeque::with_capacity(300),
            turn_history: VecDeque::with_capacity(300),
        }
    }

    pub fn is_black_turn(&self) -> bool {
        self.current_player == Occupy::Black
    }

    fn change_side(&mut self) -> Result<(), RuleError> {
        match self.current_player {
            Occupy::Black => {
                self.current_player = Occupy::White;
                Ok(())
            }
            Occupy::White => {
                self.current_player = Occupy::Black;
                Ok(())
            }
            _ => Err(RuleError::WrongPlayerSide),
        }
    }

    fn reset_visited(&mut self) {
        self.visited = [[false; BOARD_SIZE]; BOARD_SIZE];
    }

    fn get_group_and_liberties(
        &mut self,
        pos: &BoardPosition,
        board: &Board,
    ) -> (i32, Vec<(usize, usize)>) {
        self.reset_visited();
        let mut stack = Vec::with_capacity(72);
        let mut group = Vec::new();
        let mut liberties = 0;
        let color = board.get(pos);

        match color {
            Occupy::Black => {
                println!("Counting Black stone liberty.")
            }
            Occupy::White => {
                println!("Counting White stone liberty.")
            }
            _ => {}
        }

        stack.push((pos.x, pos.y));
        self.visited[pos.x][pos.y] = true;

        while let Some((cx, cy)) = stack.pop() {
            group.push((cx, cy));

            for (dx, dy) in SEARCH_DIRECTIONS {
                let nx = cx.wrapping_add(dx as usize);
                let ny = cy.wrapping_add(dy as usize);

                let next_pos = BoardPosition::new(nx, ny);
                if next_pos.is_none() {
                    continue;
                }

                let next_pos = next_pos.unwrap();

                if self.visited[next_pos.x][next_pos.y] {
                    continue;
                }

                self.visited[next_pos.x][next_pos.y] = true;
                match board.get(&next_pos) {
                    same if same == color => stack.push((nx, ny)),
                    Occupy::Free => liberties += 1,
                    _ => {}
                }
            }
        }
        (liberties, group)
    }

    fn capture_group(&mut self, group: &[(usize, usize)], board: &mut Board) {
        for &(x, y) in group {
            board.remove_stone(BoardPosition { x: x, y: y });
        }
    }

    fn is_ko_violation(&self, move_string: &str) -> bool {
        self.board_history.len() > 1
            && move_string == &self.board_history[self.board_history.len() - 2]
    }

    pub fn set_stone(&mut self, pos: &BoardPosition, board: &mut Board) {
        println!("------------------------------------");

        if board.get(pos) != Occupy::Free {
            // return false;
            println!(
                "There is already a stone in position ({}, {})",
                pos.x, pos.y
            );
            return;
        }

        board.set_stone(*pos, self.current_player);
        let opponent = match self.current_player {
            Occupy::Black => Occupy::White,
            Occupy::White => Occupy::Black,
            _ => unreachable!(),
        };

        match self.current_player {
            Occupy::Black => println!("Now is black turn."),
            Occupy::White => println!("Now is white turn."),
            _ => println!("Neither black turn nor white turn, sonmething unexcepted happened."),
        }

        let (own_liberties, _) = self.get_group_and_liberties(pos, board);
        println!(
            "Liberty at position ({}, {}) is {}",
            pos.x, pos.y, own_liberties
        );
        let mut move_valid = own_liberties > 0;
        match move_valid {
            true => println!(
                "Move of setting stone to position ({}, {}) is valid.",
                pos.x, pos.y
            ),
            false => println!(
                "Move of setting stone to position ({}, {}) is NOT valid.",
                pos.x, pos.y
            ),
        }
        let mut captures = Vec::new();

        for (dx, dy) in SEARCH_DIRECTIONS {
            let nx = pos.x.wrapping_add(dx as usize);
            let ny = pos.y.wrapping_add(dy as usize);

            if nx >= BOARD_SIZE || ny >= BOARD_SIZE {
                continue;
            }

            let next_pos = BoardPosition { x: nx, y: ny };

            if board.get(&next_pos) == opponent {
                let (lib, group) = self.get_group_and_liberties(&next_pos, board);
                println!(
                    "Liberty of hostile stone at positon ({}, {}) is {}.",
                    nx, ny, lib
                );
                if lib == 0 {
                    captures.push(group);
                    move_valid = true;
                    println!("Stone chain contains position ({}, {}) is removed.", nx, ny);
                }
            }
        }

        if !move_valid {
            match board.remove_stone(*pos) {
                Ok(()) => {
                    let cur_status = board.get(pos);
                    match cur_status {
                        Occupy::Free => {
                            println!("Postion is ({}, {}) is CLEARED.", pos.x, pos.y)
                        }
                        _ => {
                            println!("Postion is ({}, {}) is STILL OCCUPUED.", pos.x, pos.y)
                        }
                    }
                    println!(
                        "Definately not a valid move because position at ({}, {}).",
                        pos.x, pos.y
                    );
                    // return false;
                    return;
                }
                Err(e) => {
                    println!("Failed to remove the stone at ({}, {})", pos.x, pos.y);
                    return;
                }
            }
        }

        let mut cur_state = board.get_board_string();
        cur_state.set_char(
            pos.x * BOARD_SIZE + pos.y,
            match self.current_player {
                Occupy::Black => '1',
                Occupy::White => '2',
                _ => '0',
            },
        );

        for group in &captures {
            for &(cx, cy) in group {
                cur_state.set_char(cx * BOARD_SIZE + cy, '0');
            }
            self.capture_group(group, board);
        }

        if self.is_ko_violation(&cur_state) {
            println!("Ko deteced.");
            board.remove_stone(*pos);
            for group in captures {
                // self.capture_group(&group);
                for ind in group {
                    let (x_ind, y_ind) = ind;
                    board.set_stone(BoardPosition { x: x_ind, y: y_ind }, opponent);
                }
            }
            // return false;
            return;
        }

        self.board_history.push_back(cur_state);
        self.turn_history.push_back(self.current_player);
        self.change_side();
    }
}
