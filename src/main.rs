mod logic;

fn main() {
    println!("Hello, world!");
    let mut logic = logic::go_logic::GoLogic::new();
    
    logic.set_stone(0, 0);
    logic.print_board_info();

    logic.set_stone(1, 0);
    logic.print_board_info();

    logic.set_stone(1, 1);
    logic.print_board_info();

    logic.set_stone(0, 1);
    logic.print_board_info();

    logic.set_stone(0, 0);
    logic.print_board_info();
}
