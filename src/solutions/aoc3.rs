use crate::aoc_util::*;
use std::collections::HashSet;

pub fn main() {
    println!("Part 1: {}", part1("./input/3.txt"));
    println!("Part 2: {}", part2("./input/3.txt"));
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

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);

    lines.
    collect::<Vec<String>>().
    chunks(3).
    map(|chunk| {
        ascii2priority(&thrintersection(&chunk[0], &chunk[1], &chunk[2]))
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

fn thrintersection(s1: &str, s2: &str, s3: &str) -> char {
    let set1: HashSet<char> = s1.chars().collect();
    let set2: HashSet<char> = s2.chars().collect();
    let set3: HashSet<char> = s3.chars().collect();

    let intersection = set1.intersection(&set2);
    *intersection.
    filter(|c| set3.contains(*c)).
    nth(0).
    unwrap()
}