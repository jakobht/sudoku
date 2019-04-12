#![feature(test)]
mod board;
mod solver;
use board::{Entry, Board};
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
    b[0][0] = Entry::Clue(5);
    b[0][3] = Entry::Clue(1);
    b[0][4] = Entry::Clue(8);
    b[0][7] = Entry::Clue(4);
    b[0][8] = Entry::Clue(3);

    b[1][0] = Entry::Clue(4);
    b[1][1] = Entry::Clue(1);
    b[1][2] = Entry::Clue(8);
    b[1][4] = Entry::Clue(5);
    b[1][5] = Entry::Clue(9);
    b[1][6] = Entry::Clue(7);
    
    b[2][0] = Entry::Clue(6);
    b[2][2] = Entry::Clue(9);
    b[2][4] = Entry::Clue(2);
    b[2][7] = Entry::Clue(8);

    b[3][3] = Entry::Clue(8);

    b[4][0] = Entry::Clue(9);
    b[4][4] = Entry::Clue(7);
    b[4][8] = Entry::Clue(2);

    b[5][5] = Entry::Clue(2);

    b[6][1] = Entry::Clue(6);
    b[6][4] = Entry::Clue(1);
    b[6][6] = Entry::Clue(3);
    b[6][8] = Entry::Clue(5);

    b[7][2] = Entry::Clue(5);
    b[7][3] = Entry::Clue(4);
    b[7][4] = Entry::Clue(6);
    b[7][6] = Entry::Clue(2);
    b[7][7] = Entry::Clue(1);
    b[7][8] = Entry::Clue(7);

    b[8][0] = Entry::Clue(1);
    b[8][1] = Entry::Clue(2);
    b[8][4] = Entry::Clue(9);
    b[8][5] = Entry::Clue(3);
    b[8][8] = Entry::Clue(8);

    println!("{}", b);

    let start = SystemTime::now();
    fill_board(&mut b);
    println!("{} in {}", b, start.elapsed().expect("Time is wrong").as_millis());
}
