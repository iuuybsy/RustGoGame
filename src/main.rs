mod logic;

use std::cell::RefCell;
use ggez::{
    Context, ContextBuilder, GameResult, conf, event,
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh},
};
use logic::go_logic::GoLogic;


const UNIT: f32 = 51.0;
const BOARD_WIDTH: f32 = 21.0 * UNIT;
const BOARD_HEIGHT: f32 = 21.0 * UNIT;
const LINE_WIDTH: f32 = 3.0;
const NUM: i32 = 19;
const STAR_POSITION: [[i32; 2]; 9] = [
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
const DOT_RADIUS: f32 = 9.0;
const STONE_OUTER_RADIUS: f32 = 0.95 * 0.5 * UNIT;
const STONE_INNER_RADIUS: f32 = 0.8 * STONE_OUTER_RADIUS;

fn grid_to_pixel(grid_cord: [i32; 2]) -> [f32; 2] {
    [
        (grid_cord[0] as f32 + 1.5) * UNIT,
        BOARD_HEIGHT - (grid_cord[1] as f32 + 1.5) * UNIT,
    ]
}

fn render_line(canvas: &mut Canvas, ctx: &mut Context, start_point: [i32; 2], end_point: [i32; 2]) {
    let start_point = grid_to_pixel(start_point);
    let end_point = grid_to_pixel(end_point);
    let diagonal_line = Mesh::new_line(ctx, &[start_point, end_point], LINE_WIDTH, Color::BLACK)
        .expect("failed to create line mesh");

    canvas.draw(&diagonal_line, DrawParam::default());
}

fn render_circle(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2], radius: f32, color: Color) {
    let center = grid_to_pixel(center);
    let circle = Mesh::new_circle(ctx, DrawMode::fill(), center, radius, 0.1, color)
        .expect("failed to create circle mesh");

    canvas.draw(&circle, DrawParam::default());
}

fn render_square(
    canvas: &mut Canvas,
    ctx: &mut Context,
    center: [i32; 2],
    size: f32,
    color: Color,
) {

    let center = grid_to_pixel(center);
    let half_size = 0.5 * size;
    let vertices = [
        [center[0] - half_size, center[1] - half_size],        // 左上
        [center[0] + half_size, center[1] - half_size],    // 右上
        [center[0] + half_size, center[1] + half_size],// 右下
        [center[0] - half_size, center[1] + half_size],    // 左下
    ];

    let square = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &vertices,
        color,
    ).expect("failed to create square mesh");
    canvas.draw(&square, DrawParam::default());
}

fn render_black_dot(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, DOT_RADIUS, Color::BLACK);
}

fn render_black_square(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_square(canvas, ctx, center, DOT_RADIUS * 2.0, Color::BLACK);
}

fn render_white_square(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_square(canvas, ctx, center, DOT_RADIUS * 2.0, Color::WHITE);
}

fn render_black_stone(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, STONE_OUTER_RADIUS, Color::BLACK);
}

fn render_white_stone(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, STONE_OUTER_RADIUS, Color::BLACK);
    render_circle(canvas, ctx, center, STONE_INNER_RADIUS, Color::WHITE);
}



struct MainState {
    logic: RefCell<GoLogic>,
    mouse_x_num: i32,
    mouse_y_num: i32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState{
            logic: RefCell::new(GoLogic::new()),
            mouse_x_num: -1,
            mouse_y_num: -1,
        })
    }

    fn render_background(&mut self, canvas: &mut Canvas, ctx: &mut Context) {
        for i in 0..NUM {
            render_line(canvas, ctx, [i, 0], [i, 18]);
            render_line(canvas, ctx, [0, i], [18, i]);
        }

        for [x, y] in STAR_POSITION {
            render_black_dot(canvas, ctx, [x, y]);
        }
    }

    fn render_mouse_hint(&mut self, canvas: &mut Canvas, ctx: &mut Context) {
        let center = [self.mouse_x_num, self.mouse_y_num];
        if self.logic.borrow().is_blakc_turn() {
            render_black_square(canvas, ctx, center);
        }
        else {
            render_white_square(canvas, ctx, center);
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // 木头颜色：RGB(160, 82, 45)
        let wood_color = Color::from_rgb(160, 82, 45);

        let mut canvas = graphics::Canvas::from_frame(ctx, wood_color);

        self.render_background(&mut canvas, ctx);

        self.render_mouse_hint(&mut canvas, ctx);

        render_black_stone(&mut canvas, ctx, [0, 0]);
        render_black_stone(&mut canvas, ctx, [1, 1]);

        render_white_stone(&mut canvas, ctx, [1, 0]);
        render_white_stone(&mut canvas, ctx, [0, 1]);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        self.mouse_x_num = (x / UNIT).floor() as i32 - 1;
        self.mouse_y_num = 19 - (y / UNIT).floor() as i32;
        println!("Mouse position: ({}, {})", self.mouse_x_num, self.mouse_y_num);
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("wood_square", "author")
        .window_setup(conf::WindowSetup::default().title("Wood Square"))
        .window_mode(conf::WindowMode::default().dimensions(BOARD_WIDTH, BOARD_HEIGHT))
        .build()?;

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
