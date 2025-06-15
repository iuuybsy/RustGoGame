mod logic;

fn main() {
    println!("Hello, world!");
    let mut logic = logic::go_logic::GoLogic::new();
    logic.print_board_info();
    logic.roughly_set_black_stone(0, 0);
    logic.print_board_info();
    logic.roughly_set_white_stone(1, 0);
    logic.print_board_info();

    let res1 = logic.get_local_liberty(0, 0);
    println!("local liverty at (0, 0) is {}.", res1);

    let res2 = logic.get_local_liberty(1, 0);
    println!("local liverty at (1, 0) is {}.", res2);
}
