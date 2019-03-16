extern crate gtk;

use gtk::prelude::*;


use gtk::{Button, Window, WindowType};

fn main() {
    println!("Hello, world!");

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("riiry launcher");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Fuzzy Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::main();
}
