mod board;
use board::{Board, Entry};

fn fill_board(mut board: &mut Board) {
    fn fill_board_rec(mut board: &mut Board, row: usize, col: usize) -> bool {
        for n in 1..(board.size() + 1) {
            println!("{}, {}", row, col);
            board[row][col] = board::Entry::Num(n as u8);
            println!("{}", board);
            println!("{}", board.check_board());
            if board.check_board() {
                let (row, col) = 
                    if row == board.size()-1 && col == board.size()-1 {
                        return true;
                    } else if col == board.size()-1 {
                        (row+1, 0)
                    } else {
                        (row, col+1)
                    };
                println!{"Will call with {}, {}", row, col}
                if fill_board_rec(&mut board, row, col) { return true }
            }
        }
        board[row][col] = board::Entry::Empty;
        return false
    }

    fill_board_rec(&mut board, 0, 0);
}

fn main() {
    let mut b = board::Board::new(9);

    fill_board(&mut b);

    println!("{}", b)
}
