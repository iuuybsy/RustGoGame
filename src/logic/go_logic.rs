use std::{collections::VecDeque};

trait SafeSetChar {
    fn set_char(&mut self, idx: usize, c: char) -> bool;
}

impl SafeSetChar for String {
    fn set_char(&mut self, idx: usize, c: char) -> bool {
        let new_len = c.len_utf8();
        let bytes = self.as_bytes();

        if !self.is_char_boundary(idx) || bytes.len() - idx < new_len {
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
    related_stones: VecDeque<(usize, usize)>,
    board_history: Box<VecDeque<String>>,
    turn_history: Box<VecDeque<bool>>,
}

impl GoLogic {
    pub fn new() ->Self {
        GoLogic{
            is_black_turn: true, 
            board: [[Occupy::Free; LOGIC_HEIGHT]; LOGIC_WIDTH],
            related_stones: VecDeque::new(),
            board_history: Box::new(VecDeque::new()),
            turn_history: Box::new(VecDeque::new()),
        }
    }

    #[inline]
    fn is_cord_valid(&self, x: usize, y: usize) -> bool {
        x < self.board.len() && y < self.board[0].len()
    }

    #[inline]
    fn is_occupied_by_stone(&self, x: usize, y: usize) -> bool {
        self.board[x][y] != Occupy::Free
    }

    fn get_cur_board_string(&self) -> String {
        let mut cur_string = "0".repeat(LOGIC_WIDTH * LOGIC_HEIGHT);
        for i in 0..LOGIC_WIDTH {
            for j in 0..LOGIC_HEIGHT {
                let mut stone = '0';
                if self.is_occupied_by_stone(i, j) {
                    stone = if self.board[i][j] == Occupy::Black {'1'} else {'2'};
                }
                cur_string.set_char(i * LOGIC_WIDTH + j, stone);
            }
        }
        return cur_string;
    }

    pub fn get_local_liberty(&mut self, x:usize, y: usize, need_record: bool) -> i32 {
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
                if need_record {
                    self.related_stones.push_back((x_cri, y_cri));
                }

                for i in 0..SEARCH_DIRECTION.len() {
                    let x_next = x_cri + SEARCH_DIRECTION[i][0];
                    let y_next = y_cri + SEARCH_DIRECTION[i][1];
                    if !self.is_cord_valid(x_next, y_next) {
                        continue;
                    }
                    else if !unvisited[x_next][y_next] {
                        continue;
                    }

                    if self.board[x_next][y_next] == friend {
                        stack.push_back((x_next, y_next));
                    }
                    else if self.board[x_next][y_next] == Occupy::Free {
                        local_liberty += 1;
                    }
                    unvisited[x_next][y_next] = false;
                }
                for i in 0..SEARCH_DIRECTION.len() {
                    if x_cri >= SEARCH_DIRECTION[i][0] && y_cri >= SEARCH_DIRECTION[i][1] {
                        let x_next = x_cri - SEARCH_DIRECTION[i][0];
                        let y_next = y_cri - SEARCH_DIRECTION[i][1];
                        if !self.is_cord_valid(x_next, y_next) {
                            continue;
                        }
                        else if !unvisited[x_next][y_next] {
                            continue;
                        }

                        if self.board[x_next][y_next] == friend {
                            stack.push_back((x_next, y_next));
                        }
                        else if self.board[x_next][y_next] == Occupy::Free {
                            local_liberty += 1;
                        }
                        unvisited[x_next][y_next] = false;
                    }
                }
            }
        }
        return local_liberty;
    }

    pub fn set_stone(&mut self, x: usize, y: usize) {
        if !self.is_cord_valid(x, y) {
            println!("Cord ({}, {}) is not valid.", x, y);
            return;
        }
        else if self.is_occupied_by_stone(x, y) {
            println!("Cord ({}, {}) is not occupied by stone.", x, y);
            return;
        }

        let friend = if self.is_black_turn {Occupy::Black} else {Occupy::White};
        let hostile = if self.is_black_turn {Occupy::White} else {Occupy::Black};
        self.board[x][y] = friend;

        let local_liberty = self.get_local_liberty(x, y, false);
        if local_liberty == 0 {
            let mut will_kill_enemy = false;
            for i in 0..SEARCH_DIRECTION.len() {
                let x_next = x + SEARCH_DIRECTION[i][0];
                let y_next = y + SEARCH_DIRECTION[i][1];
                if !self.is_cord_valid(x_next, y_next) {
                    continue;
                }
                else if self.board[x_next][y_next] != hostile {
                    continue;
                }
                let hostile_liberty = self.get_local_liberty(x_next, y_next, false);
                if hostile_liberty == 0 {
                    will_kill_enemy = true;
                }
            }

            for i in 0..SEARCH_DIRECTION.len() {
                let x_next = x - SEARCH_DIRECTION[i][0];
                let y_next = y - SEARCH_DIRECTION[i][1];
                if !self.is_cord_valid(x_next, y_next) {
                    continue;
                }
                else if self.board[x_next][y_next] != hostile {
                    continue;
                }
                let hostile_liberty = self.get_local_liberty(x_next, y_next, false);
                if hostile_liberty == 0 {
                    will_kill_enemy = true;
                }
            }
            if !will_kill_enemy {
                self.board[x][y] = Occupy::Free;
                return;
            }
        }

        for i in 0..SEARCH_DIRECTION.len() {
            let x_next = x + SEARCH_DIRECTION[i][0];
            let y_next = y + SEARCH_DIRECTION[i][1];

            if !self.is_cord_valid(x_next, y_next) {
                continue;
            }
            else if self.board[x_next][y_next] != hostile {
                continue;
            }
            let hostile_liberty = self.get_local_liberty(x_next, y_next, true);

            if hostile_liberty == 0 {
                let board_history_len = self.board_history.len();
                if board_history_len > 1 {
                    let mut cur_board_info = self.get_cur_board_string();
                    let stone = if self.is_black_turn {'1'} else {'2'};
                    cur_board_info.set_char(x * LOGIC_WIDTH + y, stone);
                    for j in 0..self.related_stones.len() {
                        let (x_cur, y_cur) = self.related_stones[j];
                        cur_board_info.set_char(x_cur * LOGIC_WIDTH + y_cur, '0');
                    }
                    if cur_board_info == self.board_history[board_history_len - 2] {
                        self.board[x][y] = Occupy::Free;
                        return;
                    }
                }

                while !self.related_stones.is_empty() {
                    if let Some((x_cur, y_cur)) = self.related_stones.pop_back() {
                        self.board[x_cur][y_cur] = Occupy::Free;
                    }
                }
            }
            else {
                self.related_stones.clear();
            }
        }

        for i in 0..SEARCH_DIRECTION.len() {
            if x >= SEARCH_DIRECTION[i][0] && y >= SEARCH_DIRECTION[i][1] {
                let x_next = x - SEARCH_DIRECTION[i][0];
                let y_next = y - SEARCH_DIRECTION[i][1];

                if !self.is_cord_valid(x_next, y_next) {
                    continue;
                }
                else if self.board[x_next][y_next] != hostile {
                    continue;
                }
                let hostile_liberty = self.get_local_liberty(x_next, y_next, true);
                if hostile_liberty == 0 {

                    let board_history_len = self.board_history.len();
                    if board_history_len > 1 {
                        let mut cur_board_info = self.get_cur_board_string();
                        let stone = if self.is_black_turn {'1'} else {'2'};
                        cur_board_info.set_char(x * LOGIC_WIDTH + y, stone);
                        for j in 0..self.related_stones.len() {
                            let (x_cur, y_cur) = self.related_stones[j];
                            cur_board_info.set_char(x_cur * LOGIC_WIDTH + y_cur, '0');
                        }
                        if cur_board_info == self.board_history[board_history_len - 2] {
                            self.board[x][y] = Occupy::Free;
                            return;
                        }
                    }

                    while !self.related_stones.is_empty() {
                        if let Some((x_cur, y_cur)) = self.related_stones.pop_back() {
                            self.board[x_cur][y_cur] = Occupy::Free;
                        }
                    }
                }
                else {
                    self.related_stones.clear();
                }
            }
        }

        self.is_black_turn = !self.is_black_turn;

        let cur_board_info = self.get_cur_board_string();

        self.board_history.push_back(cur_board_info);
        self.turn_history.push_back(self.is_black_turn);
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