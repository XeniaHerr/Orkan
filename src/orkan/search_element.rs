use std::env;
use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use nix::unistd::execve;

use std::ffi::CString;

#[derive(Debug)]
pub struct SearchElement{
       pub search_string: String,
       pub ful_path: String,
        pub matches: Vec<u8>,
        pub is_selected: bool
}


impl Clone for SearchElement {
    fn clone(&self) -> Self {
        return SearchElement { search_string: self.search_string.clone(), matches: self.matches.clone(),
        is_selected: self.is_selected.clone(), ful_path: self.ful_path.clone() };
    }
}


pub struct Searcher {

    content : Vec<SearchElement>,

}

impl Searcher {

    pub fn content(&self) -> &Vec<SearchElement> {
        return &self.content;
    }

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
                        binaries.push( SearchElement{ search_string: file_name, matches: Vec::new(), is_selected: false, ful_path : file.path().to_str().unwrap().to_string()});
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

pub fn file_searcher( name : &String) -> Self {

    Result::expect(fs::exists(name), "File does not exist");

    let content = fs::read_to_string(name).unwrap();
    let mut binaries : Vec<SearchElement> = Vec::new();

    for line in content.lines() {
        binaries.push(SearchElement { search_string: line.to_string(), matches: Vec::new(), is_selected: false, ful_path: line.to_string() });
    }
    return Searcher { content : binaries };

    

}

pub fn simple_search(&self, target: &str) -> Vec<SearchElement> {

    let mut matches = self.content.iter().filter(|x| x.search_string.contains(target)).cloned().collect::<Vec<_>>();

    matches.sort_by(|a, b| a.search_string.len().cmp(&b.search_string.len()));
    return matches

}



}


pub fn executer( full_path : &String) {

                //let command = CString::new(full_path.clone()).unwrap();

                let args = vec![CString::new(full_path.clone()).unwrap()];

                let env = env::vars().map(|(k,v)| { CString::new(format!("{}={}", k,v)).unwrap()}).collect::<Vec<CString>>();

                let command = args[0].clone();
                execve(&command, &args, &env).expect("Failed to execute");
                panic!("Execve failed");
}


pub fn printer (content : &String) {
    println!("{}", content);
}
