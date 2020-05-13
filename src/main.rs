#![feature(test)]
mod board;
mod solver;
use board::Board;
use solver::fill_board;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

use std::env;

fn load(file: &str) -> Board {
    let mut file = File::open(file).expect(&format!("Could not open file: {}", file));
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    Board::from_str(&s)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <soduku file>", args[0]);
        return;
    }
    let mut b = load(&args[1]);
    println!("{}", b);

    let start = SystemTime::now();
    fill_board(&mut b);
    println!(
        "{} in {}",
        b,
        start.elapsed().expect("Time is wrong").as_millis()
    );
}
