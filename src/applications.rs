use failure::Error;
use glib::Sender;
use glib::{get_system_data_dirs, get_user_data_dir};
use log::{debug, info};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

lazy_static! {
    static ref RE_TypeApplication: Regex = Regex::new(r"\nType=.*Application.*\n").unwrap();
    static ref RE_ExecCommand: Regex = Regex::new(r"\nExec=(.*)\n").unwrap();
}

pub fn get_apps() -> Option<Vec<PathBuf>> {
    let mut dirs = get_system_data_dirs();
    match get_user_data_dir() {
        Some(ud) => dirs.push(ud),
        None => info!("get_user_data_dir() empty"),
    }
    debug!("dirs: {:?}", dirs);

    let mut apps: Vec<PathBuf> = Vec::new();
    for dir in dirs {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| is_desktop(e) && is_xdg_application(e))
        {
            debug!("{}", entry.path().display());
            apps.push(entry.path().to_path_buf());
        }
    }

    Some(apps)
}

pub fn get_apps_incremental(tx: Sender<Vec<String>>) {
    let mut dirs = get_system_data_dirs();
    match get_user_data_dir() {
        Some(ud) => dirs.push(ud),
        None => info!("get_user_data_dir() empty"),
    }
    debug!("dirs: {:?}", dirs);

    for dir in dirs {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| is_desktop(e) && is_xdg_application(e))
        {
            debug!("{}", entry.path().display());
            // apps.push(entry.path().to_path_buf());
            tx.send(vec![String::from(entry.path().to_str().unwrap())]);
        }
    }
}
pub fn launch_application(path: &Path) -> Result<(), Error> {
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
