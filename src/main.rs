mod game;
mod render;

use crate::game::board::{Board, BoardPosition};
use crate::game::game_const::BOARD_SIZE;
use crate::game::rule::Rule;

use crate::render::grid::render_background;
use crate::render::hint::{render_last_move_hint, render_mouse_hint};
use crate::render::render_const::UNIT;
use crate::render::stones::render_stone;

use crate::render::render_const::{BOARD_HEIGHT, BOARD_WIDTH};
use ggez::{
    Context, ContextBuilder, GameResult, conf, event,
    graphics::{self, Color},
};

struct MainState {
    board: Board,
    logic: Rule,
    mouse_x_num: i32,
    mouse_y_num: i32,
    last_x_num: i32,
    last_y_num: i32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState {
            board: Board::new(),
            logic: Rule::new(),
            mouse_x_num: -1,
            mouse_y_num: -1,
            last_x_num: -1,
            last_y_num: -1,
        })
    }

    fn is_mouse_in_board_area(&self) -> bool {
        let limit = BOARD_SIZE as i32;
        self.mouse_x_num >= 0
            && self.mouse_x_num < limit
            && self.mouse_y_num >= 0
            && self.mouse_y_num < limit
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let wood_color = Color::from_rgb(160, 82, 45);

        let mut canvas = graphics::Canvas::from_frame(ctx, wood_color);

        render_background(&mut canvas, ctx);
        render_stone(&mut canvas, ctx, &self.board);

        render_mouse_hint(
            &mut canvas,
            ctx,
            self.mouse_x_num,
            self.mouse_y_num,
            &self.board,
            &self.logic,
        );

        render_last_move_hint(&mut canvas, ctx, self.last_x_num, self.last_y_num);

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

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        match _button {
            event::MouseButton::Left => {
                if self.is_mouse_in_board_area() {
                    if self.logic.set_stone(
                        &BoardPosition {
                            x: self.mouse_x_num as usize,
                            y: self.mouse_y_num as usize,
                        },
                        &mut self.board,
                    ) {
                        self.last_x_num = self.mouse_x_num;
                        self.last_y_num = self.mouse_y_num;
                    }
                }
            }
            event::MouseButton::Right => {}
            _ => {}
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("wood_square", "author")
        .window_setup(conf::WindowSetup::default())
        .window_mode(
            conf::WindowMode::default()
                .dimensions(BOARD_WIDTH, BOARD_HEIGHT)
                .resizable(false)
                .fullscreen_type(conf::FullscreenType::Windowed),
        )
        .build()?;

    let state = MainState::new()?;
    event::run(ctx, event_loop, state);
}
