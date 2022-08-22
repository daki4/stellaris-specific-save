pub mod functions;
pub mod config;



// function that gets a file path and path delimeter as arguments and returns the file name
pub fn file_name(path: &String, delimeter: &str) -> String {
    let temp = path.split(delimeter).collect::<Vec<&str>>();
    
    temp[temp.len() - 1].to_string()
}

#[cfg(test)]
mod tests {

    use crate::functions::{get_last_modified_directory, get_last_modified_file_path, check_stellaris_save};

    #[test]
    fn check_stellaris_save_works() {
        use super::*;
        let last_modified_directory = get_last_modified_directory("/home/daki4/.local/share/Paradox Interactive/Stellaris/save games");
        let last_mod_file = get_last_modified_file_path(&last_modified_directory);
        dbg!(&last_mod_file);
        assert_eq!(true, check_stellaris_save(&file_name(&last_mod_file, "/")));
    }
}