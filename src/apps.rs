use failure::Error;
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

//pub fn get_apps() -> Result<String, Error> {
    //let mut dirs = get_system_data_dirs();
    //match get_user_data_dir() {
        //Some(ud) => dirs.push(ud),
        //None => info!("get_user_data_dir() empty"),
    //}
    //debug!("dirs: {:?}", dirs);

    //let mut results = String::new();
    //for dir in dirs {
        //for entry in WalkDir::new(dir)
            //.into_iter()
            //.filter_map(|e| e.ok())
            //.filter(|e| is_desktop(e) && is_xdg_application(e))
        //{
            //debug!("{}", entry.path().display());
            //results.push_str(entry.path().to_str().unwrap());
            //results.push_str("\n");
        //}
    //}

    //Ok(results)
//}

//pub fn get_apps() -> Option<Vec<PathBuf>> {
pub fn get_apps_strings() -> Option<Vec<String>> {
    let mut dirs = get_system_data_dirs();
    match get_user_data_dir() {
        Some(ud) => dirs.push(ud),
        None => info!("get_user_data_dir() empty"),
    }
    debug!("dirs: {:?}", dirs);

    //let apps: Vec<Option<PathBuf>> = dirs.into_iter()
    let apps: Vec<Option<String>> = dirs.into_iter()
        .map(|dir| WalkDir::new(dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                if is_desktop(&entry) && is_xdg_application(&entry) {
                    //Some(entry.path().to_path_buf())
                    //Some(entry.path().to_path_buf())
                    let s = String::from(entry.path().to_str().unwrap());
                    Some(s)
                } else {
                    None
                }
            })
            .collect())
        .collect();

    Some(apps.into_iter().flatten().collect())
}

pub fn get_apps() -> Option<Vec<PathBuf>> {
    let mut dirs = get_system_data_dirs();
    match get_user_data_dir() {
        Some(ud) => dirs.push(ud),
        None => info!("get_user_data_dir() empty"),
    }
    debug!("dirs: {:?}", dirs);

    let apps: Vec<Option<PathBuf>> = dirs.into_iter()
        .map(|dir| WalkDir::new(dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                if is_desktop(&entry) && is_xdg_application(&entry) {
                    //Some(entry.path().to_path_buf())
                    //Some(entry.path().to_path_buf())
                    let s = String::from(entry.path().to_str().unwrap());
                    Some(s)
                } else {
                    None
                }
            })
            .collect())
        .collect();

    Some(apps.into_iter().flatten().collect())
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
