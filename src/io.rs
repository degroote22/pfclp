use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_name: &str) -> String {
    let mut f = File::open(file_name).expect(&format!("file {} not found", file_name));
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(&format!(
        "something went wrong reading the file {}",
        file_name
    ));

    contents
}
