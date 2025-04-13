use rand::Rng;

use crate::brain::Brain;
use crate::game::Game;

const MUTATION_RATE : f32 = 0.05;
#[derive(Debug, Clone   )]
pub struct Agent {
    pub brain: Brain,
    pub fitness: f32,
    pub score : u32,
    pub id : u64
}

impl Agent {
    pub fn run_game(&mut self, max_steps: u32) {
        let mut game = Game::new(self.id);
        while !game.game_over && game.steps_survived < max_steps {
            let state = game.state_extraction();
            let action = self.brain.decide(&state);

            match action {
                0 => game.snake.turn_left(),
                1 => {}
                2 => game.snake.turn_right(),
                _ => panic!("Invalid Action"),
            }

            game.next_step();

        }   
        self.score = (game.snake.body.len() + 1) as u32;
        self.fitness = game.compute_fitness(game.steps_survived);
    }

    pub fn crossover(parent1 : &Agent, parent2 : &Agent) -> Agent {
        let mut rng = rand::rng();

        let mut child = Agent{brain : Brain::random(), fitness : 0.0, score : 0, id : rng.random()};
        for i in 0..child.brain.weights.len() {
            child.brain.weights[i] = if rand::random() {
                parent1.brain.weights[i]
            } else {
                parent2.brain.weights[i]
            };
        }

        child 
    }

    pub fn mutate(&mut self){
        let mut rng = rand::rng();
        for weight in &mut self.brain.weights.iter_mut() {
            if rng.random::<f32>() < MUTATION_RATE {
                *weight += rng.random_range(-0.1..0.1); // small perturbation
            }
        }
    }
}
