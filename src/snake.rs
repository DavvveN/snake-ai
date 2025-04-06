use std::vec;

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
pub const DIRECTIONS: [(i32, i32); 4] = [UP, RIGHT, DOWN, LEFT];

pub struct Snake {
    pub head: (i32, i32),
    pub body: Vec<(i32, i32)>,
    pub direction: usize,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            head: (10, 10),
            body: vec![(10, 9)],
            direction: 1,
        }
    }

    pub fn grow(&mut self) {
        let last_cell = self.body[self.body.len() - 1];
        self.body.push((
            last_cell.0 + DIRECTIONS[(self.direction + 2) % 4].0,
            last_cell.1 + DIRECTIONS[(self.direction + 2) % 4].1,
        ));
    }

    pub fn mv(&mut self, new_head_position: (i32, i32)) {
        // Move the body segments forward
        let mut body_c = self.body.clone();

        for i in (1..body_c.len()).rev() {
            self.body[i] = body_c[i - 1];
        }

        if !self.body.is_empty() {
            self.body[0] = self.head;
        }
        self.head = new_head_position;
    }

    pub fn turn_left(&mut self) {
        let new_direction = (self.direction + 3 as usize) % 4;
        self.direction = new_direction;
    }

    pub fn turn_right(&mut self) {
        let new_direction = (self.direction + 1 as usize) % 4;
        self.direction = new_direction;
    }
}
