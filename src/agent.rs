use crate::brain::Brain;
use crate::game::Game;

#[derive(Debug, Clone   )]
pub struct Agent {
    pub brain: Brain,
    pub fitness: f32,
}

impl Agent {
    pub fn run_game(&mut self, max_steps: u32) {
        let mut game = Game::new();
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
        self.fitness = game.compute_fitness(game.steps_survived);
    }

    pub fn crossover(parent1 : &Agent, parent2 : &Agent) -> Agent {
        let mut child = Agent{brain : Brain::random(), fitness : 0.0};
        for i in 0..child.brain.weights.len() {
            child.brain.weights[i] = if rand::random() {
                parent1.brain.weights[i]
            } else {
                parent2.brain.weights[i]
            };
        }

        child 
    }

    pub fn mutate(&self){
        
    }
}
