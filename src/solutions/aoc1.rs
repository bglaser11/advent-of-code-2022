use crate::aoc_util::*;
use std::collections::BinaryHeap;

pub fn main() {
    println!("Part 1: {}", part1("./input/1.txt"));
    println!("Part 2: {}", part2("./input/1.txt"));
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);
    lines.
    collect::<Vec<String>>().
    split(|str| str.trim().is_empty()).
    map(|arr| arr.iter().map(|str| str.parse::<i32>().unwrap())).
    map(|arr| arr.sum()).
    max().
    unwrap()
}

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);
    lines.
    collect::<Vec<String>>().
    split(|str| str.trim().is_empty()).
    map(|arr| arr.iter().map(|str| str.parse::<i32>().unwrap())).
    map(|arr| arr.sum()).
    collect::<BinaryHeap<i32>>().
    iter().
    take(3).
    sum()
}