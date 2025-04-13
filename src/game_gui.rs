use std::collections::VecDeque;

use ggez::{
    event,
    graphics::{self, Color},
    input::keyboard::KeyCode,
    Context, GameResult,
};

use crate::game::Game;

const TOP_LEFT_CORNER: (i32, i32) = (50, 50);
const SQUARE_SIZE: i32 = 25;
pub struct AppState {
    game: Game,
    tick: u32,
    inputs: VecDeque<usize>,
}

impl AppState {
    pub fn new(_ctx: &mut Context) -> GameResult<AppState> {
        let state = AppState {
            game: Game::new(),
            tick: 0,
            inputs: VecDeque::new(),
        };

        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.tick += 1;

        if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            self.inputs.push_back(0);
        }

        if ctx.keyboard.is_key_just_pressed(KeyCode::Right) {
            self.inputs.push_back(1);
        }

        if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            self.inputs.push_back(2);
        }

        if ctx.keyboard.is_key_just_pressed(KeyCode::Left) {
            self.inputs.push_back(3);
        }

        if self.tick % 5 == 0 {
            if !self.inputs.is_empty() {
                let current = self.game.snake.direction;
                let opposite = (current + 2) % 4;
                let dir = self.inputs.pop_front().unwrap();

                // Only change direction if not trying to go backwards
                if dir != opposite {
                    self.game.snake.direction = dir;
                }
            }

            self.game.next_step();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for r in 0..20 {
            for c in 0..20 {
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        TOP_LEFT_CORNER.0 + c as i32 * SQUARE_SIZE + 1,
                        TOP_LEFT_CORNER.1 + r as i32 * SQUARE_SIZE + 1,
                        SQUARE_SIZE - 2,
                        SQUARE_SIZE - 2,
                    ),
                    Color::RED,
                )
                .expect("COULDNT CREATE RECTANGLE FROM BLOCK");

                canvas.draw(&rectangle, graphics::DrawParam::default());
            }
        }
        let head = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                TOP_LEFT_CORNER.0 + self.game.snake.head.0 as i32 * SQUARE_SIZE + 1,
                TOP_LEFT_CORNER.1 + self.game.snake.head.1 as i32 * SQUARE_SIZE + 1,
                SQUARE_SIZE - 2,
                SQUARE_SIZE - 2,
            ),
            Color::GREEN,
        )
        .expect("COULDNT CREATE RECTANGLE FROM BLOCK");

        canvas.draw(&head, graphics::DrawParam::default());

        for part in &self.game.snake.body {
            let body_part = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    TOP_LEFT_CORNER.0 + part.0 as i32 * SQUARE_SIZE + 1,
                    TOP_LEFT_CORNER.1 + part.1 as i32 * SQUARE_SIZE + 1,
                    SQUARE_SIZE - 2,
                    SQUARE_SIZE - 2,
                ),
                Color::BLUE,
            )
            .expect("COULDNT CREATE RECTANGLE FROM BLOCK");

            canvas.draw(&body_part, graphics::DrawParam::default());
        }
        let fruit = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                TOP_LEFT_CORNER.0 + self.game.fruit_position.0 as i32 * SQUARE_SIZE + 1,
                TOP_LEFT_CORNER.1 + self.game.fruit_position.1 as i32 * SQUARE_SIZE + 1,
                SQUARE_SIZE - 2,
                SQUARE_SIZE - 2,
            ),
            Color::MAGENTA,
        )
        .expect("COULDNT CREATE RECTANGLE FROM BLOCK");

        canvas.draw(&fruit, graphics::DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }
}
