
use std::env;
use std::collections::HashMap;
use std::vec::Vec;
use std::fs;

#[derive(Debug)]
pub struct SearchElement{
       pub search_string: String,
        pub matches: Vec<u8>,
        pub is_selected: bool
}


impl Clone for SearchElement {
    fn clone(&self) -> Self {
        return SearchElement { search_string: self.search_string.clone(), matches: self.matches.clone(),
        is_selected: self.is_selected.clone() };
    }
}


pub struct Searcher {

    content : Vec<SearchElement>,

}

impl Searcher {

 pub fn binary_searcher() -> Self {

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

    return Searcher { content :binaries };
}

pub fn simple_search(&self, target: &str) -> Vec<SearchElement> {

    let matches = self.content.iter().filter(|x| x.search_string.contains(target)).cloned().collect::<Vec<_>>();

    return matches;

}



}
