use directories::UserDirs;
use log::warn;
use serde::Deserialize;
use std::{env, fs, path::Path};

#[derive(Debug)]
pub struct Configuration {
    pub directory_saves: String,
    pub directory_target: String,
    pub delay_seconds: u64,
    pub years_passed: u16,
    pub default_delay_seconds: u64,
    pub delimeter: String,
}

#[derive(Deserialize)]
struct ReadSettings {
    pub target_dir: String,
    pub delay_seconds: u64,
    pub years_passed: u16,
}

impl ReadSettings {
    fn new() -> ReadSettings {
        // Read file "settings.json" from current directory and parse it into a struct of type ReadSettings
        let path = Path::new("settings.json");
        let file = fs::read_to_string(path).unwrap();

        dbg!(&file);

        let settings: ReadSettings = serde_json::from_str(&file).unwrap();
        settings
    }
}

impl Configuration {
    pub fn new() -> Configuration {
        let delimeter = match env::consts::OS {
            "windows" => "\\".to_string(),
            "linux" => "/".to_string(),
            _ => panic!("unsupported OS"),
        };

        let saves_path = if let Some(user_dirs) = UserDirs::new() {
            format!(
                "{}{}{}{}{}{}{}",
                user_dirs.document_dir().unwrap().to_string_lossy().to_string(),
                &delimeter,
                "Paradox Interactive",
                &delimeter,
                "Stellaris",
                &delimeter,
                "save games"
            )
        } else {
            warn!("Could not find user directories");
            "".to_string()
        };

        let stx = ReadSettings::new();
        let a = Configuration {
            directory_saves: saves_path,
            directory_target: stx.target_dir,
            delay_seconds: stx.delay_seconds,
            years_passed: stx.years_passed,
            default_delay_seconds: 5,
            delimeter: delimeter,
        };
        ensure_target_dir(a.directory_target.as_str());

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

pub fn ensure_target_dir(target_dir: &str) {
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).unwrap();
    }
}
