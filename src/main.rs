extern crate gtk;

use gtk::prelude::*;

use gtk::{Entry, TextView, Window, WindowType};
use std::process::Command;

use sublime_fuzzy::best_match;

// TODO: Actually work through this tutorial:
// https://mmstick.github.io/gtkrs-tutorials/introduction.html

fn get_files() -> String {
    let cmd = Command::new("fd")
        .arg("-pa")
        .arg(".")
        .output()
        .expect("Failed to run fd");

    let files = String::from_utf8(cmd.stdout).unwrap();
    return files;
}

fn filter_lines(query: &str, strlines: &str) -> String {
    if query.len() == 0 {
        return String::from(strlines);
    }

    let v: Vec<&str> = strlines.split("\n").collect();
    let mut results: Vec<(isize, &str)> = v
        .into_iter()
        .map(|s| match best_match(query, s) {
            Some(m) => (m.score(), s),
            None => (0, s),
        })
        .collect();
    results.sort();

    let sortedmatches: Vec<&str> = results
        .into_iter()
        .filter(|t| t.0 > 0)
        .map(|t| t.1)
        .collect();

    return sortedmatches.join("\n");
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
    vbox.pack_start(&scrolled_text_view, true, true, 0);
    window.add(&vbox);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    {
        let text_view = text_view.clone();
        entry.connect_activate(move |_| {
            let buffer = text_view.get_buffer().unwrap();
            let line = buffer
                .get_text(&buffer.get_start_iter(), &buffer.get_iter_at_line(1), false)
                .unwrap();
            println!("Launching: xdg-open {}", line);
            Command::new("xdg-open")
                .arg(&line)
                .output()
                .expect("Failed to run xdg-open");

            gtk::main_quit();
        });
    }

    entry.connect_changed(move |e| {
        let buffer = e.get_buffer();
        let query = buffer.get_text();
        let results = filter_lines(&query, &full_files_list.clone());
        println!("{}", results);

        //update the main list
        let buffer = text_view.get_buffer().unwrap();
        buffer.set_text("");
        buffer.insert_at_cursor(&results);
    });

    gtk::main();
}
