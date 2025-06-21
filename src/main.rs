mod logic;

fn main() {
    println!("Hello, world!");
    let mut logic = logic::go_logic::GoLogic::new();
    
    logic.set_stone(8, 8);
    logic.set_stone(8, 9);
    logic.set_stone(7, 9);
    logic.set_stone(7, 10);
    logic.set_stone(9, 9);
    logic.set_stone(9, 10);
    logic.set_stone(8, 10);
    logic.set_stone(8, 11);
    logic.set_stone(6, 8);
    logic.print_board_info();

    logic.set_stone(8, 9);
    logic.print_board_info();

    logic.set_stone(8, 10);
    logic.print_board_info();


    logic.set_stone(0, 0);
    logic.print_board_info();
}
