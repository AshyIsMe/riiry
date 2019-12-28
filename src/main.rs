#[macro_use]

extern crate lazy_static;
extern crate regex;

pub mod ui;
pub mod applications;
pub mod files;
pub mod state;
pub mod filter;

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
    // Initialize the UI's initial state
    ui::App::new()
        // Connect events to the UI
        .connect_events()
        // Display the UI and execute the program
        .then_execute();
}
