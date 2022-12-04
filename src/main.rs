use std::env;
mod aoc1;
mod aoc2;
mod aoc3;
mod aoc_util;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: 'cargo run -- <1..25>'");
        return;
    }

    let arg: i32 = args[1].parse().unwrap();

    if !(1..50).contains(&arg) {
        println!("Usage: 'cargo run -- <1..25>'");
        return;
    }

    match arg {
        1 => aoc1::main(),
        2 => aoc2::main(),
        3 => aoc3::main(),
        _ => println!("{} isn't implemented!", arg),
    }
}