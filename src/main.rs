use home;
#[allow(unused)]
use std::{env, fs, fs::read_dir, io, io::Read, path::Path};
use std::{path::PathBuf, time::SystemTime};

// get linux stellaris saves path
#[cfg(target_os = "linux")]
fn get_path() -> Option<String> {
    let home = home::home_dir()?;
    let path = format!(
        "{}/.local/share/Paradox Interactive/Stellaris/save games/",
        home.display()
    );
    Some(path)
}

// get windows stellaris saves path
#[cfg(target_os = "windows")]
fn get_path() -> Option<String> {
    let home = home::home_dir()?;
    let path = format!(
        "{}\\Documents\\Paradox Interactive\\Stellaris\\save games\\",
        home.display()
    );
    Some(path)
}

// get the location to store the saves
fn get_target_path() -> String {
    env::var("TARGET_DIR").unwrap()
}

// ensure the target directory exists
fn ensure_target_dir(target_dir: &String) {
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).unwrap();
    }
}

// copy stuff
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        let src: PathBuf = working_path.components().skip(input_root).collect();

        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }
    Ok(())
}

// function to get the last modified directory
fn get_last_modified_directory(path: &String) -> String {
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

fn main() {
    dotenv::dotenv().ok();
    let directory_saves = get_path().unwrap();
    let directory_target = get_target_path();
    let target = get_target_path();
    ensure_target_dir(&target);
    let mut delimeter = "";
    if cfg!(target_os = "linux") {
        delimeter = "/";
    }
    else if cfg!(target_os = "windows") {
        delimeter = "\\";
    }

    loop {
        let last_modified_directory = get_last_modified_directory(&directory_saves);
        dbg!(&directory_saves, &directory_target, &last_modified_directory);
        let temp = last_modified_directory.split(delimeter).collect::<Vec<&str>>();

        let new_path = directory_target.clone() + delimeter.clone() + temp[temp.len() - 1].clone();
        match copy(Path::new(&last_modified_directory), Path::new(&new_path)) {
            Ok(()) => {}
            Err(_e) => println!("failed to copy."),
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
