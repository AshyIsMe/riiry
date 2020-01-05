extern crate glib;

use failure::err_msg;
use failure::Error;
use gdk::enums::key;
use gtk::prelude::*;
use gtk::{Builder, Dialog};
use gtk::{Entry, TextView};

use log::debug;
use rayon::prelude::*;
use std::path::Path;
use std::process;
use std::sync::{Arc, RwLock};
use std::thread;

use super::super::applications;
use super::super::files;
use super::super::filter;

use super::super::state::RiiryState;

pub struct App {
    pub window: Dialog,
    pub entry: Entry,
    pub textview: TextView,
}

/// A wrapped `App` which provides the capability to execute the program.
pub struct ConnectedApp(App);

impl ConnectedApp {
    /// Display the window, and execute the gtk main event loop.
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}

// AA TODO: Move this into a utils.rs.
// Obviously a buffer of file paths is not an amazing data structure.
fn exec_open(textview: &TextView) -> Result<(), Error> {
    let buffer = textview
        .get_buffer()
        .ok_or_else(|| err_msg("getting buffer"))?;
    let line = buffer
        .get_text(&buffer.get_start_iter(), &buffer.get_iter_at_line(1), false)
        .ok_or_else(|| err_msg("getting text"))?;

    debug!("Launching: {}", line);
    if line.trim().ends_with(".desktop") {
        applications::launch_application(&Path::new(&line.trim())).is_ok();
    } else {
        files::open_file_in_default_app(&Path::new(&line.trim())).is_ok();
    }

    gtk::main_quit();

    Ok(())
}

impl App {
    pub fn new() -> App {
        // Initialize GTK before proceeding.
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
            process::exit(1);
        }

        let glade_src = include_str!("riiry.glade");
        let builder = Builder::new_from_string(glade_src);
        let window: Dialog = builder
            .get_object("riiry_dialog")
            .expect("Couldn't get entry");
        let entry: Entry = builder
            .get_object("riiry_dialog_entry")
            .expect("Couldn't get entry");
        let textview: TextView = builder
            .get_object("riiry_dialog_textview")
            .expect("Couldn't get textview");

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

        App {
            window,
            entry,
            textview,
        }
    }

    pub fn connect_events(self) -> ConnectedApp {
        let riirystate: Arc<RwLock<RiiryState>> = Arc::new(RwLock::new(RiiryState::new()));

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        {
            let tx = tx.clone();
            thread::spawn(move || {
                applications::get_apps_incremental(tx);
            });
        }
        {
            let tx = tx.clone();
            thread::spawn(move || {
                files::get_home_files_incremental(tx);
            });
        }

        {
            let buffer = self
                .textview
                .get_buffer()
                .ok_or_else(|| err_msg("text view buffer missing"))
                .unwrap();

            let riirystate = riirystate.clone();

            rx.attach(None, move |mut haystack_str| {
                haystack_str[0].push_str("\n");
                buffer.insert_at_cursor(&haystack_str.join("\n"));

                riirystate.write().unwrap().extend_haystack(haystack_str);

                glib::Continue(true)
            });
        }

        {
            self.activate();
            self.key_events(riirystate);
        }

        ConnectedApp(self)
    }

    fn activate(&self) {
        let textview = self.textview.clone();
        let window = self.window.clone();
        self.entry.connect_activate(move |_| {
            if let Err(e) = exec_open(&textview) {
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

    fn key_events(&self, riirystate: Arc<RwLock<RiiryState>>) {
        let textview = self.textview.clone();

        self.entry.connect_changed(move |e| {
            let buffer = e.get_buffer();
            let query = buffer.get_text();

            riirystate.write().unwrap().set_needle(query.clone());

            let vec = riirystate.read().unwrap().get_haystack().clone();

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            riirystate
                .write()
                .unwrap()
                .search_worker
                .run(move |cancel| {
                    let results = filter::filter_lines_rff(&query, &vec, &cancel);
                    //debug!("{:?}", results);

                    if let Some(results) = results {
                        let _ = tx.send(results);
                    }
                });

            {
                let textview = textview.clone();
                rx.attach(None, move |results| {
                    //update the main list
                    // let topn: Vec<String> = results.drain(0..100).collect(); //AA TODO - drain errors out if length is shorter than range
                    let topn: Vec<String> = results;
                    let buffer = textview.get_buffer().unwrap();
                    buffer.set_text("");
                    // buffer.insert_at_cursor(&results.join("\n"));
                    buffer.insert_at_cursor(&topn.join("\n"));

                    glib::Continue(true)
                });
            }
        });
    }
}
