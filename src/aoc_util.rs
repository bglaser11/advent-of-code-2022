use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines().map(|x| x.unwrap()).into_iter()
}