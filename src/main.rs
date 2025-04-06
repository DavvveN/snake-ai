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

        for agent in &mut population {
            agent.run_game(500);
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
