use std::{env, path::Path, fs};

use log::warn;

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
        let a = Configuration {
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
        };
        ensure_target_dir(&a.directory_target.as_str());
        
        if a.delay_seconds > 0 && a.years_passed == 0 {

            warn!("both a year and seconds delay were specified, using seconds delay.");
        }

        a
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration::new()
    }
}

/// get stellaris saves path
#[cfg(target_os = "linux")]
pub fn get_path() -> Option<String> {
    let home = home::home_dir()?;
    let _platform_path = match env::consts::OS {
        "linux" => "/.local/share/Paradox Interactive/Stellaris/save games",
        "windows" => "\\Documents\\Paradox Interactive\\Stellaris\\save games",
        _ => panic!("unsupported OS"),
    };
    let path = format!(
        "{}{}",
        home.display(),
        _platform_path
    );
    Some(path)
}

pub fn ensure_target_dir(target_dir: &str) {
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).unwrap();
    }
}
