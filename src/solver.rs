extern crate test;

use crate::board::{Board, Entry};

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

pub fn fill_board(board: &mut Board) {
    let mut row = 0;
    let mut col = 0;
    let mut forward = true;

    'main: while row < board.size() && col < board.size() {
        // if col == 0 {
        //    println!("{}", board);
        //    println!("");
        //}
        let start = match board[row][col] {
            Entry::Empty => 1,
            Entry::Num(n) => n + 1,
            Entry::Clue(_n) => { 
                if forward {
                    next_cord(&mut row, &mut col, board.size()); 
                } else {
                    prev_cord(&mut row, &mut col, board.size());
                }
                continue 'main 
            } 
        };

        for n in start as usize..(board.size()+1) {
            board[row][col] = Entry::Num(n as u8);
            if board.check_board(row, col) {
                forward = true;
                next_cord(&mut row, &mut col, board.size());
                continue 'main;
            }
        }
        board[row][col] = Entry::Empty;
        forward = false;
        prev_cord(&mut row, &mut col, board.size());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fill_9(b: &mut Bencher) {
        b.iter(|| {
            let mut board = Board::new(9);
            fill_board(&mut board);
        });
    }

    #[bench]
    fn bench_fill_16(b: &mut Bencher) {
        b.iter(|| {
            let mut board = Board::new(16);
            fill_board(&mut board);
        });
    }
}