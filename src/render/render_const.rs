pub const BOARD_SIZE: i32 = 19;
pub const UNIT: f32 = 51.0;
pub const BOARD_WIDTH: f32 = 21.0 * UNIT;
pub const BOARD_HEIGHT: f32 = 21.0 * UNIT;
pub const LINE_WIDTH: f32 = 3.0;
pub const NUM: i32 = 19;
pub const STAR_POSITION: [[i32; 2]; 9] = [
    [3, 3],
    [3, 9],
    [3, 15],
    [9, 3],
    [9, 9],
    [9, 15],
    [15, 3],
    [15, 9],
    [15, 15],
];
pub const DOT_RADIUS: f32 = 9.0;
pub const STONE_OUTER_RADIUS: f32 = 0.95 * 0.5 * UNIT;
pub const STONE_INNER_RADIUS: f32 = 0.8 * STONE_OUTER_RADIUS;
