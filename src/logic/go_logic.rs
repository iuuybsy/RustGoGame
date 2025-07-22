use std::collections::{VecDeque};

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

const LOGIC_WIDTH: usize = 19;
const LOGIC_HEIGHT: usize = 19;
const SEARCH_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Occupy {
    Free,
    Black,
    White,
}

pub struct GoLogic {
    current_player: Occupy,
    board: [[Occupy; LOGIC_HEIGHT]; LOGIC_WIDTH],
    visited: [[bool; LOGIC_HEIGHT]; LOGIC_WIDTH],
    board_history: VecDeque<String>,
    turn_history: VecDeque<Occupy>,
}

impl GoLogic {
    pub fn new() -> Self {
        GoLogic {
            current_player: Occupy::Black,
            board: [[Occupy::Free; LOGIC_HEIGHT]; LOGIC_WIDTH],
            visited: [[false; LOGIC_HEIGHT]; LOGIC_WIDTH],
            board_history: VecDeque::with_capacity(300),
            turn_history: VecDeque::with_capacity(300),
        }
    }

    pub fn is_blakc_turn(&self) -> bool {
        self.current_player == Occupy::Black
    }

    #[inline]
    fn in_bounds(&self, x: i8, y: i8) -> bool {
        x >= 0 && x < LOGIC_WIDTH as i8 && y >= 0 && y < LOGIC_HEIGHT as i8
    }

    fn get_board_string(&self) -> String {
        let mut s = String::with_capacity(LOGIC_WIDTH * LOGIC_HEIGHT);
        for row in self.board {
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

    fn reset_visited(&mut self) {
        self.visited = [[false; LOGIC_HEIGHT]; LOGIC_WIDTH];
    }

    fn get_group_and_liberties(&mut self, x: usize, y: usize) -> (i32, Vec<(usize, usize)>) {
        self.reset_visited();
        let mut stack = Vec::with_capacity(72);
        let mut group = Vec::new();
        let mut liberties = 0;
        let color = self.board[x][y];
        
        stack.push((x, y));
        self.visited[x][y] = true;
        
        while let Some((cx, cy)) = stack.pop() {
            group.push((cx, cy));
            
            for (dx, dy) in SEARCH_DIRECTIONS {
                let nx = cx.wrapping_add(dx as usize);
                let ny = cy.wrapping_add(dy as usize);
                
                if nx >= LOGIC_WIDTH || ny >= LOGIC_HEIGHT {
                    continue;
                }
                
                if self.visited[nx][ny] {
                    continue;
                }
                
                self.visited[nx][ny] = true;
                match self.board[nx][ny] {
                    same if same == color => stack.push((nx, ny)),
                    Occupy::Free => liberties += 1,
                    _ => {}
                }
            }
        }
        (liberties, group)
    }

    fn capture_group(&mut self, group: &[(usize, usize)]) {
        for &(x, y) in group {
            self.board[x][y] = Occupy::Free;
        }
    }

    fn is_ko_violation(&self, move_string: &str) -> bool {
        self.board_history.len() > 1 && move_string == &self.board_history[self.board_history.len() - 2]
    }

    pub fn place_stone(&mut self, x: i8, y: i8) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        
        let (x, y) = (x as usize, y as usize);
        
        if self.board[x][y] != Occupy::Free {
            return false;
        }

        self.board[x][y] = self.current_player;
        let opponent = match self.current_player {
            Occupy::Black => Occupy::White,
            Occupy::White => Occupy::Black,
            _ => unreachable!(),
        };
        
        let (own_liberties, _) = self.get_group_and_liberties(x, y);
        let mut move_valid = own_liberties > 0;
        let mut captures = Vec::new();
        
        if !move_valid {
            for (dx, dy) in SEARCH_DIRECTIONS {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                
                if nx >= LOGIC_WIDTH || ny >= LOGIC_HEIGHT {
                    continue;
                }
                
                if self.board[nx][ny] == opponent {
                    let (lib, group) = self.get_group_and_liberties(nx, ny);
                    if lib == 0 {
                        captures.push(group);
                        move_valid = true;
                    }
                }
            }
        }
        
        if !move_valid {
            self.board[x][y] = Occupy::Free;
            return false;
        }
        
        let mut cur_state = self.get_board_string();
        cur_state.set_char(x * LOGIC_WIDTH + y, match self.current_player {
            Occupy::Black => '1',
            Occupy::White => '2',
            _ => '0'
        });
        
        for group in &captures {
            for &(cx, cy) in group {
                cur_state.set_char(cx * LOGIC_WIDTH + cy, '0');
            }
            self.capture_group(group);
        }
        
        if self.is_ko_violation(&cur_state) {
            self.board[x][y] = Occupy::Free;
            for group in captures {
                self.capture_group(&group);
            }
            return false;
        }
        
        self.board_history.push_back(cur_state);
        self.turn_history.push_back(self.current_player);
        self.current_player = opponent;
        true
    }

    pub fn print_board(&self) {
        println!();
        for y in (0..LOGIC_HEIGHT).rev() {
            print!("{:2} ", y + 1);
            for x in 0..LOGIC_WIDTH {
                print!("{} ", match self.board[x][y] {
                    Occupy::Free => '.',
                    Occupy::Black => 'X',
                    Occupy::White => 'O',
                });
            }
            println!();
        }
        
        print!("   ");
        for x in 0..LOGIC_WIDTH {
            print!("{} ", (b'A' + x as u8) as char);
        }
        println!();
    }
}