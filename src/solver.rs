extern crate test;

use crate::board::{Board, Entry};

#[derive(Debug)]
struct FilledCache {
    rows: Vec<bool>,
    cols: Vec<bool>,
    squares: Vec<bool>,
}

#[inline(always)]
fn get<'a>(v: &'a mut Vec<bool>, i: usize, j: u8, length: usize) -> &'a mut bool {
    // unsafe { v.get_unchecked_mut(i*length+j as usize) }
    &mut v[i * length + j as usize]
}

impl FilledCache {
    fn new(board: &Board) -> FilledCache {
        let mut f = FilledCache {
            rows: vec![false; board.size() * board.size()],
            cols: vec![false; board.size() * board.size()],
            squares: vec![false; board.size() * board.size()],
        };

        for (i, row) in board.board.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                f.insert(i, j, v, board);
            }
        }
        f
    }

    #[inline(always)]
    fn add_num(&mut self, row: usize, col: usize, num: u8, board: &Board) -> bool {
        let sq_number = row / board.square_size() * board.square_size() + col / board.square_size();
        let num = num - 1;
        if *get(&mut self.rows, row, num, board.size())
            || *get(&mut self.cols, col, num, board.size())
            || *get(&mut self.squares, sq_number, num, board.size())
        {
            false
        } else {
            *get(&mut self.rows, row, num, board.size()) = true;
            *get(&mut self.cols, col, num, board.size()) = true;
            *get(&mut self.squares, sq_number, num, board.size()) = true;
            true
        }
    }
    #[inline(always)]
    fn insert(&mut self, row: usize, col: usize, val: &Entry, board: &Board) -> bool {
        let r = match val {
            Entry::Empty => true,
            Entry::Num(n) => self.add_num(row, col, *n, board),
            Entry::Clue(n) => self.add_num(row, col, *n, board),
        };
        r
    }

    #[inline(always)]
    fn remove_num(&mut self, row: usize, col: usize, num: u8, board: &Board) {
        let num = num - 1;
        let sq_number = row / board.square_size() * board.square_size() + col / board.square_size();
        *get(&mut self.rows, row, num, board.size()) = false;
        *get(&mut self.cols, col, num, board.size()) = false;
        *get(&mut self.squares, sq_number, num, board.size()) = false;
    }

    #[inline(always)]
    fn remove(&mut self, row: usize, col: usize, val: &Entry, board: &Board) {
        match val {
            Entry::Empty => (),
            Entry::Num(n) => self.remove_num(row, col, *n, board),
            Entry::Clue(n) => self.remove_num(row, col, *n, board),
        }
    }
}

fn generate_order(board: &Board) -> Vec<(usize, usize)> {
    let mut res = Vec::with_capacity(board.size() * board.size());
    for row in 0..board.size() {
        for col in 0..board.size() {
            match board[row][col] {
                Entry::Clue(_) => (),
                _ => res.push((row, col)),
            }
        }
    }
    res
}

pub fn fill_board(board: &mut Board) {
    let order = generate_order(board);

    let mut current_cords = 0;
    let mut forward = true;
    let mut fc = FilledCache::new(&board);

    'main: while current_cords < order.len() {
        // println!("{}", board);
        let (row, col) = unsafe { *order.get_unchecked(current_cords) };
        let start = match board[row][col] {
            Entry::Empty => 1,
            Entry::Num(n) => n + 1,
            Entry::Clue(_n) => {
                if forward {
                    current_cords += 1;
                } else {
                    current_cords -= 1;
                }
                continue 'main;
            }
        };

        fc.remove(row, col, &board[row][col], board);
        for n in start as usize..(board.size() + 1) {
            if fc.insert(row, col, &Entry::Num(n as u8), board) {
                board[row][col] = Entry::Num(n as u8);
                forward = true;
                current_cords += 1;
                continue 'main;
            }
        }
        board[row][col] = Entry::Empty;
        forward = false;
        current_cords -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fill_hard(b: &mut Bencher) {
        let s = "- - - - - - - - -
        - - - - - 3 - 8 5
        - - 1 - 2 - - - -
        - - - 5 - 7 - - -
        - - 4 - - - 1 - -
        - 9 - - - - - - -
        5 - - - - - - 7 3
        - - 2 - 1 - - - -
        - - - - 4 - - - 9";
        let board_empty = Board::from_str(s);
        b.iter(|| {
            fill_board(&mut board_empty.clone());
        });
    }
}
