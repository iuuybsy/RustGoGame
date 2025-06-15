use std::collections::VecDeque;

const LOGIC_WIDTH: usize = 19;
const LOGIC_HEIGHT: usize = 19;
const SEARCH_DIRECTION: [[usize; 2]; 2] = [
    [0, 1],
    [1, 0],
];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Occupy {
    Free,
    Black,
    White,
}

pub struct GoLogic {
    is_black_turn: bool,
    board: [[Occupy; LOGIC_HEIGHT]; LOGIC_WIDTH],
}

impl GoLogic {
    pub fn new() ->Self {
        GoLogic{
            is_black_turn: true, 
            board: [[Occupy::Free; LOGIC_HEIGHT]; LOGIC_WIDTH],
        }
    }

    pub fn roughly_set_black_stone(&mut self, x: usize, y: usize) {
        self.board[x][y] = Occupy::Black;
    }

    pub fn roughly_set_white_stone(&mut self, x: usize, y: usize) {
        self.board[x][y] = Occupy::White;
    }

    #[inline]
    fn is_cord_valid(&self, x: usize, y: usize) -> bool {
        x < self.board.len() && y < self.board[0].len()
    }

    #[inline]
    fn is_occupied_by_stone(&self, x: usize, y: usize) -> bool {
        self.board[x][y] != Occupy::Free
    }

    pub fn get_local_liberty(&self, x:usize, y: usize) -> i32 {
        let mut local_liberty: i32 = 0;

        if !self.is_cord_valid(x, y) {
            println!("Cord ({}, {}) is not valid.", x, y);
            return local_liberty;
        }
        else if !self.is_occupied_by_stone(x, y) {
            println!("Cord ({}, {}) is not occupied by stone.", x, y);
            return local_liberty;
        }

        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        let mut unvisited: [[bool; LOGIC_HEIGHT]; LOGIC_WIDTH] = [[true; LOGIC_HEIGHT]; LOGIC_WIDTH];

        let friend = if self.board[x][y] == Occupy::Black {Occupy::Black} else {Occupy::White};

        stack.push_back((x, y));
        unvisited[x][y] = false;

        while !stack.is_empty() {
            
            if let Some((x_cri, y_cri)) = stack.pop_back() {
                for i in 0..SEARCH_DIRECTION.len() {
                    let x_next_add = x_cri + SEARCH_DIRECTION[i][0];
                    let y_next_add = y_cri + SEARCH_DIRECTION[i][1];
                    if !self.is_cord_valid(x_next_add, y_next_add) {
                        continue;
                    }
                    else if !unvisited[x_next_add][y_next_add] {
                        continue;
                    }

                    if self.board[x_next_add][y_next_add] == friend {
                        stack.push_back((x_next_add, y_next_add));
                    }
                    else if self.board[x_next_add][y_next_add] == Occupy::Free {
                        local_liberty += 1;
                    }
                    unvisited[x_next_add][y_next_add] = false;
                    
                    if x_cri >= SEARCH_DIRECTION[i][0] && y_cri >= SEARCH_DIRECTION[i][1] {
                        let x_next_sub = x_cri - SEARCH_DIRECTION[i][0];
                        let y_next_sub = y_cri - SEARCH_DIRECTION[i][1];
                        if self.board[x_next_sub][y_next_sub] == friend {
                            stack.push_back((x_next_sub, y_next_sub));
                        }
                        else if self.board[x_next_sub][y_next_sub] == Occupy::Free {
                            local_liberty += 1;
                        }
                        unvisited[x_next_sub][y_next_sub] = false;
                    }

                }
            }

        }
        return local_liberty;
    }

    pub fn print_board_info(&self) {
        let width = self.board.len();
        let height = self.board[0].len();


        println!();
        for j in 0..height {
            print!("{:2} ", height - j - 1);
            for i in 0..width {
                let cell = self.board[i][height - j - 1];
                match cell {
                    Occupy::Free => print!(". "),
                    Occupy::Black => print!("X "),
                    Occupy::White => print!("O "),
                }
            }
            println!();
        }
        print!("   ");
        for col in 0..LOGIC_WIDTH {
            let label = (b'A' + col as u8) as char;
            print!("{} ", label);
        }
        println!();
    }
}