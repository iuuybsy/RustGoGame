use crate::game::board::{Board, BoardPosition};
use crate::game::rule::Rule;
use crate::game::stone::Occupy;
use crate::render::basic_element::{render_circle, render_square};
use crate::render::render_const::{BOARD_SIZE, DOT_RADIUS};
use ggez::{
    Context,
    graphics::{Canvas, Color},
};

fn render_white_square(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_square(canvas, ctx, center, DOT_RADIUS * 2.0, Color::WHITE);
}

fn render_black_square(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_square(canvas, ctx, center, DOT_RADIUS * 2.0, Color::BLACK);
}

pub fn render_mouse_hint(
    canvas: &mut Canvas,
    ctx: &mut Context,
    mouse_x_num: i32,
    mouse_y_num: i32,
    board: &Board,
    rule: &Rule,
) {
    let center = [mouse_x_num, mouse_y_num];
    if 0 <= mouse_x_num && mouse_x_num < BOARD_SIZE && 0 <= mouse_y_num && mouse_y_num < BOARD_SIZE
    {
        if board.get(&BoardPosition {
            x: mouse_x_num as usize,
            y: mouse_y_num as usize,
        }) == Occupy::Free
        {
            if rule.is_black_turn() {
                render_black_square(canvas, ctx, center);
            } else {
                render_white_square(canvas, ctx, center);
            }
        }
    }
}

fn render_red_dot(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, DOT_RADIUS, Color::RED);
}

pub fn render_last_move_hint(
    canvas: &mut Canvas,
    ctx: &mut Context,
    last_x_num: i32,
    last_y_num: i32,
) {
    if last_x_num >= 0 && last_y_num >= 0 {
        let center = [last_x_num, last_y_num];
        render_red_dot(canvas, ctx, center);
    }
}
