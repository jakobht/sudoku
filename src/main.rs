mod board;
use board::{Board};

use std::env;

fn fill_board(mut board: &mut Board) {
    fn fill_board_rec(mut board: &mut Board, row: usize, col: usize) -> bool {
        for n in 1..(board.size() + 1) {
            board[row][col] = board::Entry::Num(n as u8);
            if board.check_board() {
                let (row, col) = 
                    if row == board.size()-1 && col == board.size()-1 {
                        return true;
                    } else if col == board.size()-1 {
                        (row+1, 0)
                    } else {
                        (row, col+1)
                    };
                if fill_board_rec(&mut board, row, col) { return true }
            }
        }
        board[row][col] = board::Entry::Empty;
        return false
    }

    fill_board_rec(&mut board, 0, 0);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <size>", args[0]);
        return
    }
    let size = args[1].parse().expect("The size should be an integer");
    let mut b = board::Board::new(size);

    for _ in 0..10000 {
        fill_board(&mut b);
    }

    println!("{}", b)
}
