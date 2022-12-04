use crate::aoc_util::*;
use std::{collections::HashSet};

pub fn main() {
    println!("Part 1: {}", part1("./input/3.txt"));
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    lines.
    map(|l| {
        let mut half1 = HashSet::new();
        let mut half2 = HashSet::new();
        let chars: Vec<char> = l.chars().collect();
        for (i, char) in chars.iter().enumerate() {
            if i < chars.len() / 2 {
                half1.insert(char);
            }
            else {
                half2.insert(char);
            }
        }
        let mut intersection = half1.intersection(&half2);
        ascii2priority(*intersection.nth(0).unwrap())
    }).
    sum()
}

fn ascii2priority(c: &char) -> i32 {
    let cbyte = *c as i32;
    if c.is_ascii_lowercase() {
        return cbyte - 96;
    }
    else {
        return cbyte - 38;
    }
}