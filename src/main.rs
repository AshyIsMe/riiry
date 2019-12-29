#[macro_use]
extern crate lazy_static;

pub mod applications;
pub mod files;
pub mod filter;
pub mod state;
pub mod ui;

pub mod worker;

// TODO: Finish working through this tutorial:
// https://mmstick.github.io/gtkrs-tutorials/introduction.html
//
// TODO:
// - A real data structure in state.rs
// - Search the buffer async
// - better xdg support: https://crates.io/crates/xdg
// - Windows + OSX: https://crates.io/crates/directories

fn main() {
    pretty_env_logger::init();
    // simple_logger::init_with_level(log::Level::Warn).unwrap();

    // Initialize the UI's initial state
    ui::App::new()
        // Connect events to the UI
        .connect_events()
        // Display the UI and execute the program
        .then_execute();
}
