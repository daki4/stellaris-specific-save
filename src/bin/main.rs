#[allow(unused)]
use log::{debug, error, info, warn};
use regex::Regex;
use std::env;
use std::path::Path;
use stellaris_specific_save::config::Configuration;
use stellaris_specific_save::functions::*;
use stellaris_specific_save::*;

fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init_timed();

    let conf = Configuration::new();
    let mut last_saved_year = 2200u16;
    let year_regex = Regex::new(r"\d{4}").unwrap();
    dbg!(&conf);
    loop {
        let last_modified_directory = get_last_modified_directory(&conf.directory_saves);

        let temp = last_modified_directory
            .split(&conf.delimeter)
            .collect::<Vec<&str>>();

        let new_path = format!(
            "{}{}{}",
            &conf.directory_target,
            &conf.delimeter,
            temp[temp.len() - 1]
        );

        let filepath = get_last_modified_file_path(&last_modified_directory);

        if conf.years_passed > 0 && conf.delay_seconds == 0 {
            let last_mod_file = file_name(&filepath, &conf.delimeter);
            let save_year = year_regex
                .find(&last_mod_file)
                .unwrap()
                .as_str()
                .parse::<u16>()
                .unwrap();
            if save_year < last_saved_year + conf.years_passed {
                info!("sleeping, no new-enough files found. Last backed up save was at year : {}, and backup will commence in {} years.", last_saved_year, save_year - last_saved_year);
            } else {
                match copy(Path::new(&filepath), Path::new(&new_path)) {
                    Ok(()) => {
                        let a = get_last_modified_file_path(&last_modified_directory);
                        let b = file_name(&a, &conf.delimeter);
                        info!("backed up: {}", b);
                        last_saved_year = save_year;
                    }
                    Err(e) => warn!("failed to copy: {} reason: {}", &filepath, e),
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(100));
        } else if conf.delay_seconds > 0 && conf.years_passed == 0 {
            match copy(Path::new(&last_modified_directory), Path::new(&new_path)) {
                Ok(()) => {
                    let a = get_last_modified_file_path(&last_modified_directory);
                    let b = file_name(&a, &conf.delimeter);
                    info!("backed up: {}", b);
                }
                Err(e) => warn!("failed to copy: {} reason: {}", &filepath, e),
            }
        }
    }
}
