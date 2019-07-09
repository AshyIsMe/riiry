use failure::Error;
use failure::ResultExt;
use failure::err_msg;
//use log::{debug};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

//pub fn get_home_files() -> Result<String, Error> {
    //let mut results = String::new();
    //let dir = env::var("HOME")?;
    //for entry in WalkDir::new(dir)
        //.into_iter()
        //.filter_entry(|e| !is_hidden(e))
    //{
        //if let Ok(e) = entry {
            //debug!("{}", e.path().display());
            //results.push_str(e.path().to_str().unwrap());
            //results.push_str("\n");
        //}
    //}

    //Ok(results)
//}

pub fn get_home_files() -> Option<Vec<PathBuf>> {
    match env::var("HOME") {
        Ok(dir) => {
            Some(WalkDir::new(dir)
                .into_iter()
		.filter_entry(|e| is_not_hidden(e))
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
		    Some(entry.path().to_path_buf())
                })
                .collect())
        },
        _ => None
    }
}

pub fn open_file_in_default_app(path: &Path) -> Result<(), Error> {
    println!("Launching: xdg-open {:?}", path);
    Command::new("xdg-open")
        .arg(&path.as_os_str())
        .output()
        .with_context(|_| err_msg("Failed to run xdg-open"))?;

    Ok(())
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
         .file_name()
         .to_str()
         .map(|s| entry.depth() == 0 || !s.starts_with("."))
         .unwrap_or(false)
}
