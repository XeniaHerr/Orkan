use std::env;
use std::collections::HashMap;
use std::vec::Vec;
use std::fs::{self, ReadDir};



struct Search {
    search_string: String
}

fn main() {


    println!("TODO:");

}

fn get_binaries() -> Vec<String> {

    let vars : HashMap<String, String>  = env::vars().collect();

    let paths = vars.get("PATH").unwrap();

    let paths = env::split_paths(paths).filter(|p| p.exists() && p.is_dir()).collect::<Vec<_>>();

    let mut binaries : Vec<String> = Vec::new();

    for p in paths.iter() {
        let dir_contents : ReadDir = fs::read_dir(p).unwrap();



        for file in dir_contents.into_iter() {
            match file {

                Ok(file) => {
                    let file_name = file.file_name().into_string().unwrap();
                    binaries.push(file_name);
                }

                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        } 
    }

    return binaries;
}
