#[macro_use]
extern crate lazy_static;
extern crate regex;

use failure::Error;
use failure::ResultExt;
use failure::err_msg;
use gdk::enums::key;
use glib::{get_system_data_dirs, get_user_data_dir};
use gtk::prelude::*;
use gtk::{Entry, TextView, Window, WindowType};
use log::{debug, error, info, warn};
use regex::Regex;
use simple_logger;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use sublime_fuzzy::best_match;
use walkdir::{DirEntry, WalkDir};

// TODO: Actually work through this tutorial:
// https://mmstick.github.io/gtkrs-tutorials/introduction.html
//
// TODO:
// - A real data structure
// - better xdg support: https://crates.io/crates/xdg
// - Windows + OSX: https://crates.io/crates/directories

lazy_static! {
    static ref RE_TypeApplication: Regex = Regex::new(r"\nType=.*Application.*\n").unwrap();
    static ref RE_ExecCommand: Regex = Regex::new(r"\nExec=(.*)\n").unwrap();
}

fn get_home_files() -> Result<String, Error> {
    let mut results = String::new();
    let dir = env::var("HOME")?;
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e)) {

        if let Ok(e) = entry {
            debug!("{}", e.path().display());
            results.push_str(e.path().to_str().unwrap());
            results.push_str("\n");
        }
    }

    Ok(results)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_desktop(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".desktop"))
        .unwrap_or(false)
}

fn is_xdg_application(entry: &DirEntry) -> bool {
    let f = fs::read_to_string(entry.path());
    match f {
        Err(_) => false,
        Ok(f) => RE_TypeApplication.is_match(&f),
    }
}

fn get_apps() -> Result<String, Error> {
    let mut dirs = get_system_data_dirs();
    match get_user_data_dir() {
        Some(ud) => dirs.push(ud),
        None => info!("get_user_data_dir() empty"),
    }
    debug!("dirs: {:?}", dirs);

    let mut results = String::new();
    for dir in dirs {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| is_desktop(e) && is_xdg_application(e))
        {
            debug!("{}", entry.path().display());
            results.push_str(entry.path().to_str().unwrap());
            results.push_str("\n");
        }
    }

    Ok(results)
}

fn filter_lines(query: &str, strlines: &str) -> String {
    if query.is_empty() {
        return String::from(strlines);
    }

    let v: Vec<&str> = strlines.split('\n').collect();
    let mut results: Vec<(isize, &str)> = v
        .into_iter()
        .map(|s| match best_match(query, s) {
            Some(m) => (m.score(), s),
            None => (0, s),
        })
        .collect();
    results.sort();
    results.reverse();

    let sorted_matches: Vec<&str> = results
        .into_iter()
        .filter(|t| t.0 > 0)
        .map(|t| t.1)
        .collect();

    sorted_matches.join("\n")
}

fn main() -> Result<(), Error> {
    simple_logger::init().unwrap();

    gtk::init().with_context(|_| err_msg("failed to initialise gtk"))?;

    let full_files_list = get_home_files()?;
    let full_apps_list = get_apps()?;

    //let haystack = full_files_list;
    let haystack = full_apps_list + &full_files_list;

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

    window.connect_key_press_event(|window, gdk| {
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
            let results = filter_lines(&query, &haystack);
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

fn launch_application(path: &Path) -> Result<(), Error> {
    let f = fs::read_to_string(path).unwrap();
    let cap = RE_ExecCommand.captures(&f).unwrap();
    let line = &cap[1];

    // WARNING: Here be demons!
    // We are literally execing whatever is in the desktop file...
    debug!("Executing: {}", line);
    Command::new("sh")
        .arg("-c")
        .arg(&line)
        .spawn()
        .expect("Failed to launch process.");
    //.with_context(|_| err_msg("Failed to launch process"))?;

    Ok(())
}

fn open_file_in_default_app(path: &Path) -> Result<(), Error> {
    println!("Launching: xdg-open {:?}", path);
    Command::new("xdg-open")
        .arg(&path.as_os_str())
        .output()
        .with_context(|_| err_msg("Failed to run xdg-open"))?;

    Ok(())
}

fn exec_open(text_view: &TextView) -> Result<(), Error> {
    let buffer = text_view
        .get_buffer()
        .ok_or_else(|| err_msg("getting buffer"))?;
    let line = buffer
        .get_text(&buffer.get_start_iter(), &buffer.get_iter_at_line(1), false)
        .ok_or_else(|| err_msg("getting text"))?;

    debug!("Launching: {}", line);
    if line.trim().ends_with(".desktop") {
        launch_application(&Path::new(&line.trim())).is_ok();
    } else {
        open_file_in_default_app(&Path::new(&line.trim())).is_ok();
    }

    gtk::main_quit();

    Ok(())
}
