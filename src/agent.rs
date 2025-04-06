use crate::brain::Brain;
use crate::game::Game;

pub struct Agent {
    pub brain: Brain,
    pub fitness: f32,
}

impl Agent {
    pub fn run_game() {
        let mut game = Game::new();
        while game.next_step() {}
    }
}
