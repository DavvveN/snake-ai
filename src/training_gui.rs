use ggez::{
    event,
    graphics::{self, Color, Text},
    Context, GameResult,
};

use crate::{agent::Agent, game::Game};

const TOP_LEFT_CORNER: (i32, i32) = (50, 50);
const SQUARE_SIZE: i32 = 25;
pub struct TrainingState {
    game: Game,
    tick: u32,
    agent : Agent,
}

impl TrainingState {
    pub fn new(_ctx: &mut Context, agent : Agent) -> GameResult<TrainingState> {
        let state = TrainingState {
            game: Game::new(agent.id),
            tick: 0,
            agent,
        };

        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for TrainingState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.tick += 1;

        if self.tick % 5 == 0 {
            let state = self.game.state_extraction();
            let action = self.agent.brain.decide(&state);

            match action {
                0 => self.game.snake.turn_left(),
                1 => {}
                2 => self.game.snake.turn_right(),
                _ => panic!("Invalid Action"),
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

        let mut text = Text::new(format!("Score: {}",self.game.snake.body.len() - 1).to_string());
        text.set_scale(40.0);
        canvas.draw(
            &text,
            graphics::DrawParam::from([600.0, 100.0]).color(Color::from_rgb(20,135,66)),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
