use crate::render::basic_element::{render_circle, render_line};
use crate::render::render_const::{BOARD_SIZE, DOT_RADIUS, STAR_POSITION};
use ggez::{
    Context,
    graphics::{Canvas, Color},
};

fn render_black_dot(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, DOT_RADIUS, Color::BLACK);
}

pub fn render_background(canvas: &mut Canvas, ctx: &mut Context) {
    for i in 0..BOARD_SIZE {
        render_line(canvas, ctx, [i, 0], [i, 18]);
        render_line(canvas, ctx, [0, i], [18, i]);
    }

    for [x, y] in STAR_POSITION {
        render_black_dot(canvas, ctx, [x, y]);
    }
}
