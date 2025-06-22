mod logic;

fn main() {
    println!("Hello, world!");
    let mut logic = logic::go_logic::GoLogic::new();
    
    logic.place_stone(8, 8);  // Black
    logic.place_stone(8, 9);  // White
    logic.place_stone(7, 9);  // Black
    logic.place_stone(7, 10);  // White
    logic.place_stone(9, 9);  // Black
    logic.place_stone(9, 10);  // White
    logic.place_stone(8, 10);  // Black
    logic.place_stone(8, 11);  // White
    logic.place_stone(6, 8);  // Black
    logic.print_board();

    logic.place_stone(8, 9);  // White
    logic.print_board();

    logic.place_stone(8, 10);  // Black
    logic.print_board();


    logic.place_stone(0, 0);  // White
    logic.print_board();
}
