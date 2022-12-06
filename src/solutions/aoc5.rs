use crate::aoc_util::*;
use std::collections::VecDeque;



fn part1(path: &str) -> String {
    let lines = read_lines(path);
    let stacks: Vec<*mut VecDeque<char>> = vec![];
    for _ in 0..9 {
        let mut d: VecDeque<char> = VecDeque::new();
        stacks.push(d);
    }

    let mut sections = lines.collect::<Vec<String>>().split(|line| line.trim().is_empty());
    // Starting setup section, I'm sorry
    for (i, chunk) in sections.next().unwrap().chunks(4).map(|chunk| chunk.join("").as_bytes()).enumerate() {
        if chunk[1].is_ascii_alphabetic() {
            stacks[i].push_back(chunk[1] as char);
        }
    }

    // Now for the actual instructions
    for instr in sections.next().unwrap() {
        let vals = parse(instr);
        move_n_crates(vals[0], stacks[vals[1] as usize], stacks[vals[2] as usize]);
    }

    // Now return the tops of every stack, packed into one string
    let mut output = String::new();
    for stack in stacks {
        output.push(stack.pop_front().unwrap());
    }
    output
}

fn parse(line: &str) -> Vec<i32> {
    line.chars().
    filter(|c| c.is_numeric()).
    map(|dig| dig.to_digit(10).
    unwrap() as i32).
    collect()
}

fn move_n_crates(n: i32, src: *mut VecDeque<char>, dest: *mut VecDeque<char>) {
    for _ in 0..n {
        let item = src.pop_front().unwrap();
        dest.push_front(item);
    }
}



