use crate::aoc_util::*;
use itertools::Itertools;

pub fn main() {
    println!("Part 1: {}", part1("./input/5.txt"));
}

fn part1(path: &str) -> String {
    let mut lines = read_lines(path);
    let collected: String = lines.join("\n");
    let (boxes, instructions) = collected.split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    for line in boxes.lines().rev().skip(1) {
        for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
            let letter = chunk[1];
            if letter.is_ascii_alphabetic() {
                stacks[i].push(letter as char);
            }
        }
    }

    for instruction in instructions.lines() {
        let vals = parse(instruction);
        let (amount, from, to) = (vals[0], vals[1], vals[2]);

        for _ in 0..amount {
            let letter = stacks[from-1].pop().unwrap();
            stacks[to-1].push(letter);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>()
}

fn parse(line: &str) -> Vec<usize> {
    line.split(" ").
    filter_map(|s| s.parse::<usize>().ok()).
    collect()
}

