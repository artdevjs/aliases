use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use super::super::super::Config; // TODO fix this please!

pub struct Init {
    target_path: PathBuf,
    config: Config,
}

impl Init {

    pub fn new(target_path: PathBuf, config: Config) -> Init {
        Init { target_path: target_path, config: config}
    }

    pub fn execute(&mut self) {
        if Path::new(&self.target_path.join(".aliases")).exists() {
            println!("Directory already initialized.");
        } else {
            let mut new_file = File::create(self.target_path.join(".aliases")).unwrap();
            let template_string = self.template_string();
            let array = template_string.as_bytes();
            let _ = new_file.write_all(array);
            self.add_to_global_config();
        }
    }

    // ------------ private ---------- //

    fn template_string(&self) -> String {
        String::from("# alias_name:\n  # command: some command here")
    }

    fn add_to_global_config(&mut self) {
        self.config.add_alias_directory(&self.target_path);
    }
}
