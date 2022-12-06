use crate::aoc_util::*;

pub fn main() {
    println!("Part 1: {}", part1("./input/4.txt"));
    println!("Part 2: {}", part2("./input/4.txt"));
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    lines.
    filter(|line| {
        let vals = parse(line);
        let (a, b, c, d) = (vals[0], vals[1], vals[2], vals[3]);

        (a >= c && b <= d) || (a <= c && b >= d)
    }).
    collect::<Vec<String>>().
    len() as i32
}

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);

    lines.
    filter(|line| {
        let vals = parse(line);
        let (a, b, c, d) = (vals[0], vals[1], vals[2], vals[3]);

        (a <= c && b >= c) || (a <= d && b >= d) || (c <= a && d >= a) || (c <= b && d >= b) // everything but this is identical to part1
    }).
    collect::<Vec<String>>().
    len() as i32
}

fn parse(line: &str) -> Vec<i32> {
    let mut vals: Vec<i32> = Vec::new();
    for half in line.split(',') {
        for num in half.split('-') {
            vals.push(num.parse().unwrap());
        }
    }
    vals
}