use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

pub fn lines(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path);

    let reader = BufReader::new(f.unwrap());

    reader.lines()
}
