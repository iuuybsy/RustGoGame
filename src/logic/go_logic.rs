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

    pub fn roughly_set_black_stone(&mut self, x: usize, y: usize) {
        self.board[x][y] = Occupy::Black;
    }

    pub fn roughly_set_white_stone(&mut self, x: usize, y: usize) {
        self.board[x][y] = Occupy::White;
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


        // for (row_idx, row) in self.board.iter().enumerate() {
        //     print!("{:2} ", row_idx + 1);   
        //     for &cell in row.iter() {
        //         match cell {
        //             Occupy::Free => print!(". "),
        //             Occupy::Black => print!("X "),
        //             Occupy::White => print!("O "),
        //         }
        //     }
        //     println!();
        // }
        print!("   ");
        for col in 0..LOGIC_WIDTH {
            let label = (b'A' + col as u8) as char;
            print!("{} ", label);
        }
        println!();
    }
}