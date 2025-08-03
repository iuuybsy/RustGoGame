use crate::game::board::{Board, BoardPosition};
use crate::game::stone::Occupy;
use crate::render::basic_element::render_circle;
use crate::render::render_const::{BOARD_SIZE, STONE_INNER_RADIUS, STONE_OUTER_RADIUS};
use ggez::{
    Context, ContextBuilder, GameResult, conf, event,
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh},
};

fn render_black_stone(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, STONE_OUTER_RADIUS, Color::BLACK);
}

fn render_white_stone(canvas: &mut Canvas, ctx: &mut Context, center: [i32; 2]) {
    render_circle(canvas, ctx, center, STONE_OUTER_RADIUS, Color::BLACK);
    render_circle(canvas, ctx, center, STONE_INNER_RADIUS, Color::WHITE);
}

pub fn render_stone(canvas: &mut Canvas, ctx: &mut Context, board: &Board) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let pos = BoardPosition {
                x: i as usize,
                y: j as usize,
            };
            let status = board.get(&pos);
            if status == Occupy::Black {
                render_black_stone(canvas, ctx, [i, j]);
            } else if status == Occupy::White {
                render_white_stone(canvas, ctx, [i, j]);
            }
        }
    }
}
