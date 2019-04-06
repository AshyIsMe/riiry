#[macro_use]

extern crate lazy_static;
extern crate regex;

use failure::Error;
use failure::ResultExt;
use failure::err_msg;
use gdk::enums::key;
use gtk::prelude::*;
use gtk::{Entry, TextView, Window, WindowType};
use log::{debug};
use simple_logger;
use std::path::Path;

mod apps;
mod files;
mod filter;

// TODO: Actually work through this tutorial:
// https://mmstick.github.io/gtkrs-tutorials/introduction.html
//
// TODO:
// - A real data structure
// - better xdg support: https://crates.io/crates/xdg
// - Windows + OSX: https://crates.io/crates/directories


fn exec_open(text_view: &TextView) -> Result<(), Error> {
    let buffer = text_view
        .get_buffer()
        .ok_or_else(|| err_msg("getting buffer"))?;
    let line = buffer
        .get_text(&buffer.get_start_iter(), &buffer.get_iter_at_line(1), false)
        .ok_or_else(|| err_msg("getting text"))?;

    debug!("Launching: {}", line);
    if line.trim().ends_with(".desktop") {
        apps::launch_application(&Path::new(&line.trim())).is_ok();
    } else {
        files::open_file_in_default_app(&Path::new(&line.trim())).is_ok();
    }

    gtk::main_quit();

    Ok(())
}

fn main() -> Result<(), Error> {
    simple_logger::init().unwrap();

    gtk::init().with_context(|_| err_msg("failed to initialise gtk"))?;

    let full_files_list = files::get_home_files()?;
    let full_apps_list = apps::get_apps()?;

    // TODO: Add file launching back in.
    let haystack = full_apps_list + &full_files_list;
    //let haystack = full_apps_list;

    // Popup is not what we want (broken af on i3wm).  Toplevel is a "normal" window, also not what
    // we want.  Maybe needs to be Dialog?
    //let window = Window::new(WindowType::Popup);
    let window = Window::new(WindowType::Toplevel);
    window.set_title("riiry launcher");
    window.set_default_size(350, 70);

    let entry = Entry::new();

    let text_view = TextView::new();
    text_view.set_cursor_visible(false);
    text_view.set_editable(false);
    let scrolled_text_view = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);

    let buffer = text_view
        .get_buffer()
        .ok_or_else(|| err_msg("text view buffer missing"))?;
    buffer.insert_at_cursor(&haystack);

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

    window.connect_key_press_event(|_window, gdk| {
        match gdk.get_keyval() {
            key::Escape => gtk::main_quit(),
            _ => (),
        }
        Inhibit(false)
    });

    {
        let text_view = text_view.clone();
        entry.connect_activate(move |_| {
            if let Err(e) = exec_open(&text_view) {
                gtk::MessageDialog::new(
                    Some(&window),
                    gtk::DialogFlags::empty(),
                    gtk::MessageType::Error,
                    gtk::ButtonsType::Close,
                    &format!("oh no! {:?}", e),
                )
                .run();
            }
        });
    }

    {
        let text_view = text_view.clone();
        entry.connect_changed(move |e| {
            let buffer = e.get_buffer();
            let query = buffer.get_text();
            let results = filter::filter_lines(&query, &haystack);
            debug!("{}", results);

            //update the main list
            let buffer = text_view.get_buffer().unwrap();
            buffer.set_text("");
            buffer.insert_at_cursor(&results);
        });
    }

    gtk::main();

    Ok(())
}
