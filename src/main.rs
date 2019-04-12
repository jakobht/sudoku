#![feature(test)]
mod board;
mod solver;
use board::{Board};
use solver::fill_board;
use std::time::{SystemTime};

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <size>", args[0]);
        return
    }
    let size = args[1].parse().expect("The size should be an integer");
    let mut b = Board::new(size);

    let start = SystemTime::now();
    fill_board(&mut b);
    println!("{} in {:?}", b, start.elapsed().expect("Time is wrong").as_millis());
}
