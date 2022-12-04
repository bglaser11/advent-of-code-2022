use crate::aoc_util::*;

pub fn main() {
    println!("Part 1: {}", part1("./input/2.txt"));
    println!("Part 2: {}", part2("./input/2.txt"));
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);
    lines.
    map(|l| {
        let mut line = l.chars();
        let theirs: char = line.nth(0).unwrap(); // nth consumes line[0]...
        let mine: char = line.nth(1).unwrap(); // so this is accessing line[2]
        let mut sum = 0;
        match mine {
            'X' => {
                sum += 1;
                match theirs {
                    'A' => sum += 3,
                    'C' => sum += 6,
                    _ => (),
                }
            }
            'Y' => {
                sum += 2;
                match theirs {
                    'A' => sum += 6,
                    'B' => sum += 3,
                    _ => (),
                }
            }
            'Z' => {
                sum += 3;
                match theirs {
                    'B' => sum += 6,
                    'C' => sum += 3,
                    _ => (),
                }
            }
            _ => println!("Bad input")
        }
        sum
    }).
    sum()   
}

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);
    lines.
    map(|l| {
        let mut line = l.chars();
        let theirs: char = line.nth(0).unwrap(); // nth consumes line[0]...
        let mine: char = line.nth(1).unwrap(); // so this is accessing line[2]
        let mut sum = 0;
        match mine {
            'X' => {
                match theirs {
                    'A' => sum += 3,
                    'B' => sum += 1,
                    'C' => sum += 2,
                    _ => sum += 0,
                }
            }
            'Y' => {
                sum += 3;
                match theirs {
                    'A' => sum += 1,
                    'B' => sum += 2,
                    'C' => sum += 3,
                    _ => sum += 0,
                }
            }
            'Z' => {
                sum += 6;
                match theirs {
                    'A' => sum += 2,
                    'B' => sum += 3,
                    'C' => sum += 1,
                    _ => sum += 0,
                }
            }
            _ => println!("Bad input")
        }
        sum
    }).
    sum()   
}