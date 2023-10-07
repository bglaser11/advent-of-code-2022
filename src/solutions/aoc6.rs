use crate::aoc_util::*;
use itertools::Itertools;

pub fn main() {
    println!("Part 1: {}", part1("./input/6.txt"));
    println!("Part 2: {}", part2("./input/6.txt"));
}

fn part1(path: &str) -> String {
    let mut lines = read_lines(path);
    let signal: Vec<char> = lines.next().unwrap().chars().collect();

    for (i, window) in signal.windows(4).enumerate() {
        if window.iter().unique().collect::<Vec<&char>>().len() == 4 {
            return (i+4).to_string();
        }
    }

    "".to_string()
}

fn part2(path: &str) -> String {
    let mut lines = read_lines(path);
    let signal: Vec<char> = lines.next().unwrap().chars().collect();

    for (i, window) in signal.windows(14).enumerate() {
        if window.iter().unique().collect::<Vec<&char>>().len() == 14 {
            return (i+14).to_string();
        }
    }

    "".to_string()
}