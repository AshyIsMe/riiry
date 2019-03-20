extern crate gtk;

use gtk::prelude::*;


use gtk::{Button, Entry, TextView, Window, WindowType};
use std::process::{Command};
//use std::io::{self, Write};

fn get_files() -> String {
    let cmd = Command::new("fd")
        .arg("-pa")
        .arg(".")
        .output()
        .expect("Failed to run fd");

    //io::stdout().write_all(&cmd.stdout).unwrap();
    //io::stderr().write_all(&cmd.stderr).unwrap();

    let files = String::from_utf8(cmd.stdout).unwrap();
    return files;
}

fn filter_lines(query: &str, strlines: &str) -> String {
    let results = String::from("foo\nbar\nbaz");
    // AA TODO - actually filter here
    println!("We should really actually query for: {}, in {}", query, strlines);
    return results;
}

fn main() {
    println!("Hello, world!");

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let full_files_list = get_files();

    //let window = Window::new(WindowType::Toplevel);
    let window = Window::new(WindowType::Popup);
    window.set_title("riiry launcher");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("xdg-open!");

    let entry = Entry::new();

    let text_view = TextView::new();
    text_view.set_cursor_visible(false);
    text_view.set_editable(false);
    let scrolled_text_view = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);

    let buffer = text_view.get_buffer().unwrap();
    buffer.insert_at_cursor(&full_files_list);

    // Pack widgets vertically.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&entry, false, false, 0);
    vbox.pack_start(&button, false, false, 0);
    vbox.pack_start(&scrolled_text_view, true, true, 0);
    window.add(&vbox);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("TODO: Actually launch with xdg-open lolo");
    });

    //entry.connect_activate(move |e| {
    entry.connect_changed(move |e| {
        let buffer = e.get_buffer();
        let query = buffer.get_text();
        let results = filter_lines(&query, &full_files_list.clone());
        println!("{}", results);
    });

    gtk::main();
}
