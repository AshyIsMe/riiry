#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate regex;
extern crate simple_logger;

pub mod applications;
pub mod files;
pub mod filter;
pub mod state;
pub mod ui;

// TODO: Finish working through this tutorial:
// https://mmstick.github.io/gtkrs-tutorials/introduction.html
//
// TODO:
// - A real data structure in state.rs
// - Load applications and files lists async
// - Search the buffer async
// - better xdg support: https://crates.io/crates/xdg
// - Windows + OSX: https://crates.io/crates/directories

fn main() {
    simple_logger::init().unwrap();

    // Initialize the UI's initial state
    ui::App::new()
        // Connect events to the UI
        .connect_events()
        // Display the UI and execute the program
        .then_execute();
}
