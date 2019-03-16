extern crate gtk;

use gtk::prelude::*;


use gtk::{Button, TextView, Window, WindowType};
use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    //let window = Window::new(WindowType::Toplevel);
    let window = Window::new(WindowType::Popup);
    window.set_title("riiry launcher");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Fuzzy Click me!");

    let textview = TextView::new();
    //textview.set_cursor_visible(false);
    let scrolledtextview = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolledtextview.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolledtextview.add(&textview);

    // Pack widgets vertically.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&button, false, false, 0);
    vbox.pack_start(&scrolledtextview, true, true, 0);
    window.add(&vbox);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    let cmd = Command::new("fd")
        .arg("-pa")
        .arg(".")
        .output()
        .expect("Failed to run fd");

    //io::stdout().write_all(&cmd.stdout).unwrap();
    //io::stderr().write_all(&cmd.stderr).unwrap();

    let strlines = String::from_utf8(cmd.stdout).unwrap();
    let buffer = textview.get_buffer().unwrap();
    buffer.insert_at_cursor(&strlines);

    gtk::main();
}
