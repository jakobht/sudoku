mod board;
use board::{Board, Entry};
use std::time::{SystemTime, UNIX_EPOCH};

use std::env;

fn next_cord(row: &mut usize, col: &mut usize, size: usize) {
    if *col == size-1 {
        *row += 1;
        *col = 0;
    } else {
        *col += 1;
    }
}

fn prev_cord(row: &mut usize, col: &mut usize, size: usize) {
    if *col == 0 {
        *row -= 1;
        *col = size-1;
    } else {
        *col -= 1;
    }
}

fn fill_board(board: &mut Board) {
    let mut row = 0;
    let mut col = 0;

    'main: while row < board.size() && col < board.size() {
        let start = match board[row][col] {
            Entry::Empty => 1,
            Entry::Num(n) => n + 1
        };

        for n in start as usize..(board.size()+1) {
            board[row][col] = Entry::Num(n as u8);
            if board.check_board() {
                next_cord(&mut row, &mut col, board.size());
                continue 'main;
            }
        }
        board[row][col] = Entry::Empty;
        prev_cord(&mut row, &mut col, board.size());
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <size>", args[0]);
        return
    }
    let size = args[1].parse().expect("The size should be an integer");
    let mut b = board::Board::new(size);

    let start = SystemTime::now();
    fill_board(&mut b);
    println!("{} in {:?}", b, start.elapsed().expect("Time is wrong").as_millis());
}
