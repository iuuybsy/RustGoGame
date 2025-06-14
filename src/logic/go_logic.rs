const LOGIC_WIDTH: usize = 19;
const LOGIC_HEIGHT: usize = 19;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Occupy {
    Free,
    Black,
    White,
}

pub struct GoLogic {
    is_black_first: bool,
    board: [[Occupy; LOGIC_HEIGHT]; LOGIC_WIDTH],
}

impl GoLogic {
    pub fn new() ->Self {
        GoLogic{
            is_black_first: true, 
            board: [[Occupy::Free; LOGIC_HEIGHT]; LOGIC_WIDTH],
        }
    }

    pub fn set_black_stone(&mut self, x: usize, y: usize) {
        self.board[y][x] = Occupy::Black;
    }

    pub fn set_white_stone(&mut self, x: usize, y: usize) {
        self.board[y][x] = Occupy::White;
    }

    pub fn print_board_info(&self) {
        print!("   ");
        for col in 0..LOGIC_WIDTH {
            let label = (b'A' + col as u8) as char;
            print!("{} ", label);
        }
        println!();

        for (row_idx, row) in self.board.iter().enumerate() {
            print!("{:2} ", row_idx + 1);   
            for &cell in row.iter() {
                match cell {
                    Occupy::Free => print!(". "),
                    Occupy::Black => print!("X "),
                    Occupy::White => print!("O "),
                }
            }
            println!();
        }
    }
}