use crate::snake::{Snake, DIRECTIONS};
use rand::prelude::*;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;

pub struct Game {
    pub fruit_position: (i32, i32),
    pub snake: Snake,
    pub game_over: bool,
    pub steps_survived: u32,
    pub steps_without_fruit: u32,
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
            steps_survived: 0,
            steps_without_fruit: 0,
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
        self.steps_survived += 1;

        let next_pos = (
            self.snake.head.0 as i32 + DIRECTIONS[self.snake.direction].0,
            self.snake.head.1 as i32 + DIRECTIONS[self.snake.direction].1,
        );

        if next_pos.0 >= GRID_WIDTH || next_pos.0 < 0 || next_pos.1 < 0 || next_pos.1 >= GRID_HEIGHT
        {
            self.game_over = true;
            return false;
        }

        if self.snake.body.contains(&next_pos) {
            self.game_over = true;
            return false;
        }

        self.snake.mv(next_pos);
        self.steps_without_fruit += 1;
        if next_pos == self.fruit_position {
            self.snake.grow();
            self.randomize_fruit();
            self.steps_without_fruit = 0;
        }
        true
    }


    // OUT contains
    // 0 - direction 0 - 3
    // 1 - fruit in front (bool) 0 or 1
    // 2 - fruit to left (bool) 0 or 1
    // 3 - fruit to right (bool) 0 or 1
    // 4 - wall distance to front 0-1
    // 5 - wall distance to left 0-1
    // 6 - wall distance to right 0-1
    // 7 - Body ahead (bool) 0 or 1
    // 8 - Body left (bool) 0 or 1
    // 9 - Body right (bool) 0 or 1
    // 10 - Distance to fruit 0-1
    // 11 - Snake length 0-1

    pub fn state_extraction(&self) -> Vec<f32> {
        let mut out: Vec<f32> = vec![];
        let dir = self.snake.direction;
        let left_dir: usize = (dir + 3) % 4;
        let right_dir: usize = (dir + 1) % 4;
        out.push(dir as f32);

        let front_square = (
            self.snake.head.0 + DIRECTIONS[dir].0,
            self.snake.head.1 + DIRECTIONS[dir].1,
        );

        let left_square = (
            self.snake.head.0 + DIRECTIONS[left_dir].0,
            self.snake.head.1 + DIRECTIONS[left_dir].1,
        );

        let right_square = (
            self.snake.head.0 + DIRECTIONS[right_dir].0,
            self.snake.head.1 + DIRECTIONS[right_dir].1,
        );

        //Direction to fruit from head
        let fruit_in_front = if self.fruit_position == front_square {
            1.0
        } else {
            0.0
        };

        let fruit_to_left = if self.fruit_position == left_square {
            1.0
        } else {
            0.0
        };

        let fruit_to_right = if self.fruit_position == right_square {
            1.0
        } else {
            0.0
        };

        out.push(fruit_in_front);
        out.push(fruit_to_left);
        out.push(fruit_to_right);

        let grid_w = 20;
        let grid_h = 20;

        let head = self.snake.head;

        let wall_front_dist = match dir {
            0 => head.1,
            1 => grid_w - head.0 - 1,
            2 => grid_h - head.1 - 1,
            3 => head.0,
            _ => 0,
        } as f32;

        let wall_left_dist = match left_dir {
            0 => head.1,
            1 => grid_w - head.0 - 1,
            2 => grid_h - head.1 - 1,
            3 => head.0,
            _ => 0,
        } as f32;

        let wall_right_dist = match right_dir {
            0 => head.1,
            1 => grid_w - head.0 - 1,
            2 => grid_h - head.1 - 1,
            3 => head.0,
            _ => 0,
        } as f32;

        out.push(wall_front_dist / grid_w as f32);
        out.push(wall_left_dist / grid_w as f32);
        out.push(wall_right_dist / grid_w as f32);

        let body_front = if self.snake.body.contains(&front_square) {
            1.0
        } else {
            0.0
        };

        let body_left = if self.snake.body.contains(&left_square) {
            1.0
        } else {
            0.0
        };

        let body_right = if self.snake.body.contains(&right_square) {
            1.0
        } else {
            0.0
        };

        out.push(body_front);
        out.push(body_left);
        out.push(body_right);

        let dx = (self.fruit_position.0 - head.0).abs();
        let dy = (self.fruit_position.1 - head.1).abs();
        let dist = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
        let max_dist = ((grid_w.pow(2) + grid_h.pow(2)) as f32).sqrt();
        out.push(dist / max_dist);

        let max_length = grid_w * grid_h;
        out.push(self.snake.body.len() as f32 / max_length as f32);

        out
    }


    pub fn compute_fitness(&self, steps_survived: u32) -> f32 {
        let fruits_eaten = (self.snake.body.len() as u32).saturating_sub(1);

        let mut fitness = (steps_survived as f32).powf(1.2) + (fruits_eaten.pow(2) * 100) as f32;

        //penalize dying early
        if self.game_over && fruits_eaten == 0 {
            fitness -= 50.0;
        }

        //penalize going in circles and such
        if self.steps_without_fruit >= 25 {
            fitness -= 10.0;
        }

        fitness.max(0.0)
    }
}
