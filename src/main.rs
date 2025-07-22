use ggez::{
    Context, ContextBuilder, GameResult, conf, event,
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh},
};

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
        (grid_cord[1] as f32 + 1.5) * UNIT,
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

fn render_square() {

}

fn render_black_dot(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, DOT_RADIUS, Color::BLACK);
}

fn render_black_stone(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, STONE_OUTER_RADIUS, Color::BLACK);
}

fn render_white_stone(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, STONE_OUTER_RADIUS, Color::BLACK);
    render_circle(canvas, ctx, center, STONE_INNER_RADIUS, Color::WHITE);
}



struct MainState;

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState)
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

        for i in 0..NUM {
            render_line(&mut canvas, ctx, [i, 0], [i, 18]);
            render_line(&mut canvas, ctx, [0, i], [18, i]);
        }

        for [x, y] in STAR_POSITION {
            render_black_dot(&mut canvas, ctx, [x, y]);
        }

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
        let x_num: i32 = (x / UNIT).floor() as i32 - 1;
        let y_num: i32 = 19 - (y / UNIT).floor() as i32;
        println!("Mouse position: ({}, {})", x_num, y_num);
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
