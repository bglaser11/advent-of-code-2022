use std::env;
mod solutions;
mod aoc_util;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: 'cargo run -- 1..25'");
        return;
    }

    let arg: i32 = args[1].parse().unwrap();

    if !(1..50).contains(&arg) {
        println!("Usage: 'cargo run -- 1..25'");
        return;
    }

    match arg {
        1 => solutions::aoc1::main(),
        2 => solutions::aoc2::main(),
        3 => solutions::aoc3::main(),
        4 => solutions::aoc4::main(),
        5 => solutions::aoc5::main(),
        _ => println!("{} isn't implemented!", arg),
    }
}