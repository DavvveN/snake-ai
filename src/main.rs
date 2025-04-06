mod game;
mod game_gui;
mod snake;

use ggez::{conf, event, ContextBuilder};

use game::Game;
use game_gui::AppState;

pub fn main() {
    println!("Opening Window ... ");

    let context_builder = ContextBuilder::new("Snake-AI", "David Nilsson")
        .window_setup(conf::WindowSetup::default().title("Snake-AI"))
        .window_mode(conf::WindowMode::default().resizable(true));

    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");
    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}
