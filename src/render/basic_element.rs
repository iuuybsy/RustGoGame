use ggez::{
    Context,
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh},
};

use crate::render::render_const::{BOARD_HEIGHT, LINE_WIDTH, UNIT};

fn grid_to_pixel(grid_cord: [i32; 2]) -> [f32; 2] {
    [
        (grid_cord[0] as f32 + 1.5) * UNIT,
        BOARD_HEIGHT - (grid_cord[1] as f32 + 1.5) * UNIT,
    ]
}

pub fn render_line(
    canvas: &mut Canvas,
    ctx: &mut Context,
    start_point: [i32; 2],
    end_point: [i32; 2],
) {
    let start_point = grid_to_pixel(start_point);
    let end_point = grid_to_pixel(end_point);
    let diagonal_line = Mesh::new_line(ctx, &[start_point, end_point], LINE_WIDTH, Color::BLACK)
        .expect("failed to create line mesh");

    canvas.draw(&diagonal_line, DrawParam::default());
}

pub fn render_circle(
    canvas: &mut Canvas,
    ctx: &mut Context,
    center: [i32; 2],
    radius: f32,
    color: Color,
) {
    let center = grid_to_pixel(center);
    let circle = Mesh::new_circle(ctx, DrawMode::fill(), center, radius, 0.1, color)
        .expect("failed to create circle mesh");

    canvas.draw(&circle, DrawParam::default());
}

pub fn render_square(
    canvas: &mut Canvas,
    ctx: &mut Context,
    center: [i32; 2],
    size: f32,
    color: Color,
) {
    let center = grid_to_pixel(center);
    let half_size = 0.5 * size;
    let vertices = [
        [center[0] - half_size, center[1] - half_size],
        [center[0] + half_size, center[1] - half_size],
        [center[0] + half_size, center[1] + half_size],
        [center[0] - half_size, center[1] + half_size],
    ];

    let square = Mesh::new_polygon(ctx, DrawMode::fill(), &vertices, color)
        .expect("failed to create square mesh");
    canvas.draw(&square, DrawParam::default());
}
