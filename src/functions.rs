use std::{
    fs::{self, read_dir},
    io,
    path::{Path, PathBuf},
    time::SystemTime,
};

use regex;
use log::debug;

// #[allow(clippy::vec_init_then_push)]
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> io::Result<()> {
    let mut stack = vec![];
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        debug!("process: {:?}", &working_path);

        let src: PathBuf = working_path.components().skip(input_root).collect();

        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            debug!("mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        if fs::read_dir(&working_path).is_err() {
            let filename = working_path.file_name().unwrap();
            let dest_path = &dest.join(filename);
            fs::copy(&from, dest_path)?;
            // fs::copy(&from, &dest)?;
            debug!("copy: {:?} -> {:?}", &from.as_ref(), &dest_path);
            break;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        if !check_stellaris_save(&filename.to_string_lossy().to_string()) {
                            continue;
                        }
                        let dest_path = dest.join(filename);
                        debug!("copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        debug!("failed: {:?}", path);
                    }
                }
            }
        }
    }
    Ok(())
}

/// check if the file extension of the file is ".sav"
pub fn check_stellaris_save(filename: &String) -> bool {
    let save_regex = regex::Regex::new(r"\.sav").unwrap();
    if save_regex.is_match(filename) {
        return true;
    }
    false
}

/// function to get the last modified directory
pub fn get_last_modified_directory(path: &str) -> String {
    let mut last_modified_time = SystemTime::UNIX_EPOCH;
    let mut last_modified_path = String::new();
    for entry in read_dir(&path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.metadata().unwrap().modified().unwrap() > last_modified_time {
            last_modified_time = entry.metadata().unwrap().modified().unwrap();
            last_modified_path = path.to_str().unwrap().to_string();
        }
    }
    last_modified_path
}

/// function to get the last modified file's path
pub fn get_last_modified_file_path(path: &str) -> String {
    let mut last_modified_time = SystemTime::UNIX_EPOCH;
    let mut last_modified_name = String::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if check_stellaris_save(&entry.file_name().to_string_lossy().to_string()) && entry.metadata().unwrap().modified().unwrap() > last_modified_time {
            last_modified_time = entry.metadata().unwrap().modified().unwrap();
            last_modified_name = entry.path().to_string_lossy().to_string();
        }
    }
    last_modified_name
}
