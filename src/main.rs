mod logic;

fn main() {
    println!("Hello, world!");
    let mut logic = logic::go_logic::GoLogic::new();
    logic.print_board_info();
    logic.set_black_stone(0, 0);
    logic.print_board_info();
    logic.set_white_stone(1, 0);
    logic.print_board_info();

}
