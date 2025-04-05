use crate::snake::{Snake, DIRECTIONS};
use rand::{prelude::*, rngs};

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;

pub struct Game {
    fruit_position: (i32, i32),
    snake: Snake,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            fruit_position: (
                rng.random_range(0..GRID_WIDTH),
                rng.random_range(0..GRID_HEIGHT),
            ),
            snake: Snake::new(),
            game_over: false,
        }
    }

    fn randomize_fruit(&mut self) {
        let mut rng = rand::rng();

        let mut pos = (
            rng.random_range(0..GRID_WIDTH),
            rng.random_range(0..GRID_HEIGHT),
        );

        while self.snake.body.contains(&pos) || self.snake.head == pos {
            pos = (
                rng.random_range(0..GRID_WIDTH),
                rng.random_range(0..GRID_HEIGHT),
            );
        }

        self.fruit_position = pos;
    }

    pub fn next_step(&mut self) -> bool {
        let next_pos = (
            self.snake.head.0 as i32 + DIRECTIONS[self.snake.direction].0,
            self.snake.head.1 as i32 + DIRECTIONS[self.snake.direction].1,
        );

        if next_pos.0 >= GRID_WIDTH || next_pos.0 < 0 || next_pos.1 < 0 || next_pos.1 >= GRID_HEIGHT
        {
            println!("SNAKE HAS HIT A WALL");
            self.game_over = true;
            return false;
        }

        if self.snake.body.contains(&next_pos) {
            println!("SNAKE HAS HIT ITSELF");
            self.game_over = true;
            return false;
        }

        self.snake.mv(next_pos);
        if next_pos == self.fruit_position {
            self.snake.grow();
            self.randomize_fruit();
        }
        true
    }

    pub fn turn_left(&mut self) {
        self.snake.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.snake.turn_right();
    }
}
