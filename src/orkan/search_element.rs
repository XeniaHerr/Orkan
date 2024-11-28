
use std::env;
use std::collections::HashMap;
use std::vec::Vec;
use std::fs;

pub struct SearchElement{
       pub search_string: String,
        pub matches: Vec<u8>,
        pub is_selected: bool
}


 pub fn get_binaries() -> Vec<SearchElement> {

    let vars : HashMap<String, String>  = env::vars().collect();
    let path_var = vars.get("PATH").unwrap();
    let paths = env::split_paths(path_var).filter(|p| p.exists() && p.is_dir()).collect::<Vec<_>>();
    let mut binaries : Vec<SearchElement> = Vec::new();

    for p in paths.iter() {
        if let Ok(dir_contents) = fs::read_dir(p) {
            for file in dir_contents.into_iter() {
                match file {

                    Ok(file) => {
                        let file_name = file.file_name().into_string().unwrap();
                        binaries.push( SearchElement{ search_string: file_name, matches: Vec::new(), is_selected: false});
                    }

                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            } 
        } else {
            println!("Error reading directory {p:?}");
        }
    }

    return binaries;
}
