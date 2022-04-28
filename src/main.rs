#[allow(unused)]
use log::{debug, error, info, warn};
use std::{env, fs, fs::read_dir, path::Path, path::PathBuf, time::SystemTime};

/// get linux stellaris saves path
#[cfg(target_os = "linux")]
fn get_path() -> Option<String> {
    let home = home::home_dir()?;
    let path = format!(
        "{}/.local/share/Paradox Interactive/Stellaris/save games/",
        home.display()
    );
    Some(path)
}

/// get windows stellaris saves path
#[cfg(target_os = "windows")]
fn get_path() -> Option<String> {
    let home = home::home_dir()?;
    let path = format!(
        "{}\\Documents\\Paradox Interactive\\Stellaris\\save games\\",
        home.display()
    );
    Some(path)
}

/// ensure the target directory exists
fn ensure_target_dir(target_dir: &str) {
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).unwrap();
    }
}

/// copy stuff
#[allow(clippy::vec_init_then_push)]
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
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

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
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

/// function to get the last modified directory
fn get_last_modified_directory(path: &str) -> String {
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

pub fn get_last_modified_file(path: &str) -> String {
    let mut last_modified_time = SystemTime::UNIX_EPOCH;
    let mut last_modified_name = String::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().modified().unwrap() > last_modified_time {
            last_modified_time = entry.metadata().unwrap().modified().unwrap();
            // last_modified_name = name;
            last_modified_name = Path::new(&entry.path())
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
        }
    }
    last_modified_name
}

pub struct Configuration {
    pub directory_saves: String,
    pub directory_target: String,
    pub delay_seconds: u64,
    pub years_passed: u16,
    pub default_delay_seconds: u64,
    pub delimeter: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            directory_saves: get_path().unwrap(),
            directory_target: env::var("TARGET_DIR").unwrap(),
            delay_seconds: env::var("DELAY_SECONDS").unwrap().parse().unwrap(),
            years_passed: env::var("YEARS_PASSED").unwrap().parse().unwrap(),
            default_delay_seconds: 5,
            delimeter: match env::consts::OS {
                "windows" => "\\".to_string(),
                "linux" => "/".to_string(),
                _ => panic!("unsupported OS"),
            },
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration::new()
    }
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init_timed();
    dotenv::dotenv().ok();

    let conf = Configuration::new();
    ensure_target_dir(&conf.directory_target);
    let mut last_saved_year = 2200u16;

    loop {
        let last_modified_directory = get_last_modified_directory(&conf.directory_saves);

        if conf.years_passed > 0 {
            let last_mod_file = get_last_modified_file(&last_modified_directory);
            let save_year = last_mod_file.split(".").collect::<Vec<&str>>()[0]
                .parse::<u16>()
                .unwrap();
            debug!(
                "data: last_mod_file: '{}'; save_year: {}; last_saved_year: {}",
                &last_mod_file, &save_year, &last_saved_year,
            );
            if save_year <= last_saved_year + &conf.years_passed - 1 {
                std::thread::sleep(std::time::Duration::from_secs(*&conf.default_delay_seconds));
                debug!("sleeping, no new-enough files found.");
                continue;
            }
            last_saved_year = save_year;
        }

        let temp = last_modified_directory
            .split(&conf.delimeter)
            .collect::<Vec<&str>>();

        let new_path = format!(
            "{}{}{}",
            &conf.directory_target,
            &conf.delimeter,
            temp[temp.len() - 1]
        );
        match copy(Path::new(&last_modified_directory), Path::new(&new_path)) {
            Ok(()) => info!(
                "backed up: {}",
                get_last_modified_file(&last_modified_directory)
            ),
            Err(_e) => warn!("failed to copy: {}", &last_modified_directory),
        }

        if conf.delay_seconds > 0 && conf.years_passed == 0 {
            std::thread::sleep(std::time::Duration::from_secs(conf.delay_seconds));
        }
    }
}
