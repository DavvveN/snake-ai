mod agent;
mod brain;
mod game;
mod game_gui;
mod snake;

use ggez::{conf, event, ContextBuilder};

use agent::Agent;
use brain::Brain;
use game::Game;
use game_gui::AppState;
use rand::Rng;

const POP_SIZE: i32 = 100;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--train".to_string()) {
        println!("Training the Snake AI...");

        let mut population: Vec<Agent> = (0..POP_SIZE)
            .map(|_| Agent {
                brain: Brain::random(),
                fitness: 0.0,
            })
            .collect();

        println!("Starting game on {} agents...", POP_SIZE);
        for agent in &mut population {
            agent.run_game(500);
        }

        println!("Evaluating Agents...");
        population.sort_by(|a,b|{
            a.fitness.partial_cmp(&b.fitness).unwrap()
        });

        // Take top 10% of agents
        let elite_population : Vec<Agent> = population[90..].to_vec();
        let mut new_population: Vec<Agent> = Vec::new();

        //The previous elite becomes part of the new generation
        for elite in &elite_population {
            new_population.push(elite.clone());
        }

        //Crossover between the previous elites
        // Fill the rest via crossover + mutation
        let mut rng = rand::rng();
        while new_population.len() < population.len() {
            let parent1 = elite_population[rng.random_range(0..elite_population.len())].clone();
            let parent2 = elite_population[rng.random_range(0..elite_population.len())].clone();
            
            let mut child = Agent::crossover(&parent1, &parent2);
            child.mutate(); // Apply mutation if you have this method
            new_population.push(child);
        }


    } else {
        println!("Opening Window ... ");

        let context_builder = ContextBuilder::new("Snake-AI", "David Nilsson")
            .window_setup(conf::WindowSetup::default().title("Snake-AI"))
            .window_mode(conf::WindowMode::default().resizable(true));

        let (mut contex, mut event_loop) =
            context_builder.build().expect("Failed to build context.");
        let state = AppState::new(&mut contex).expect("Failed to create state.");
        event::run(contex, event_loop, state) // Run window event loop
    }
}
