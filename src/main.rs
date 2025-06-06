mod agent;
mod brain;
mod game;
mod game_gui;
mod snake;
mod training_gui;

use ggez::{conf, event, ContextBuilder};

use agent::Agent;
use brain::Brain;
use game_gui::AppState;
use rand::Rng;
use training_gui::TrainingState;

const POP_SIZE: i32 = 100;
const ITERATIONS: usize = 100;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--train".to_string()) {
        println!("Training the Snake AI...");
        let mut rng = rand::rng();

        let mut population: Vec<Agent> = (0..POP_SIZE)
            .map(|_| Agent {
                brain: Brain::random(),
                fitness: 0.0,
                score: 0,
                id: rng.random(),
            })
            .collect();

        for i in 0..ITERATIONS {
            println!("Starting training cycle {} ...", i);
            population = train_population(&mut population);
        }

        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        println!(
            "\n Best Snake: \n fitness: {} \n Score: {}",
            population[0].fitness, population[0].score
        );

        let best_agent = population[0].clone();

        println!("Opening Window ... ");

        let context_builder = ContextBuilder::new("Snake-AI", "David Nilsson")
            .window_setup(conf::WindowSetup::default().title("Snake-AI"))
            .window_mode(conf::WindowMode::default().resizable(true));

        let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");
        let state = TrainingState::new(&mut contex, best_agent).expect("Failed to create state.");
        event::run(contex, event_loop, state) // Run window event loop
    } else {
        println!("Opening Window ... ");

        let context_builder = ContextBuilder::new("Snake-AI", "David Nilsson")
            .window_setup(conf::WindowSetup::default().title("Snake-AI"))
            .window_mode(conf::WindowMode::default().resizable(true));

        let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");
        let state = AppState::new(&mut contex).expect("Failed to create state.");
        event::run(contex, event_loop, state) // Run window event loop
    }
}

pub fn train_population(population: &mut Vec<Agent>) -> Vec<Agent> {
    println!("Starting game on {} agents...", population.len());

    // Generate multiple random seeds for testing
    let mut rng = rand::rng();
    let seeds: Vec<u64> = (0..5).map(|_| rng.random()).collect(); // Test on 5 different maps

    for agent in population.iter_mut() {
        agent.run_game(10000, &seeds);
    }

    println!("Evaluating Agents...");

    // Sort by fitness in descending order
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    // Take top 10% as elites
    let elite_count = (population.len() as f32 * 0.1).ceil() as usize;
    let elite_population: Vec<Agent> = population[..elite_count].to_vec();

    let mut new_population: Vec<Agent> = Vec::new();

    // Keep the elite
    for elite in &elite_population {
        new_population.push(elite.clone());
    }

    // Reproduce until population is full
    let mut rng = rand::rng();
    while new_population.len() < population.len() {
        let i = rng.random_range(0..elite_population.len());
        let mut j = rng.random_range(0..elite_population.len());
        while j == i {
            j = rng.random_range(0..elite_population.len());
        }

        let parent1 = &elite_population[i];
        let parent2 = &elite_population[j];

        let mut child = Agent::crossover(parent1, parent2);
        child.mutate(); // Apply mutation
        new_population.push(child);
    }

    new_population
}

mod tests;
