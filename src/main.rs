mod board;

fn print_board(board: &Vec<Vec<board::Entry>>) {
    for (i, row) in board.iter().enumerate() {
        if i % 3 == 0 && i != 0 { 
            for _ in 0..board.len()+2 {
                print!("--");
            }
            println!("")
        }

        for (j, num) in row.iter().enumerate() {
            if j % 3 == 0 && j != 0 { print!("| ") };
            match num {
                board::Entry::Empty => print!("_ "),
                board::Entry::Num(n) => print!("{} ", n)
            }
        }
        println!("");
    }
}

fn check_rows(board: &Vec<Vec<board::Entry>>) -> bool {
    for row in board {
        let mut hm = vec!(false; row.len());
        for val in row {
            match val {
                board::Entry::Empty => (),
                board::Entry::Num(n) if hm[(n - 1) as usize] => return false,
                board::Entry::Num(n)  => hm[(n - 1) as usize] = true
            }
        }
    }
    true
}
fn check_cols(board: &Vec<Vec<board::Entry>>) -> bool {
    for i in 0..board[0].len() {
        let mut hm = vec!(false; board.len());
        for j in 0..board.len() {
            match board[j][i] {
                board::Entry::Empty => (),
                board::Entry::Num(n) if hm[(n - 1) as usize] => return false,
                board::Entry::Num(n)  => hm[(n - 1) as usize] = true
            }
        }
    }
    true
}

fn check_square(board: &Vec<Vec<board::Entry>>, square: usize) -> bool {
    let mut hm = vec!(false; board.len());
    let s_row = square / 3;
    let s_col = square % 3;

    for row in s_row*3..s_row*3+3 {
        for col in s_col*3..s_col*3+3 {
            match board[row][col] {
                board::Entry::Empty => (),
                board::Entry::Num(n) if hm[(n - 1) as usize] => return false,
                board::Entry::Num(n)  => hm[(n - 1) as usize] = true
            }
        }
    }

    true
}

fn check_squares(board: &Vec<Vec<board::Entry>>) -> bool {
    for square in 0..board.len() {
        if !check_square(&board, square) { return false }
    }
    true
}

fn check_board(board: &Vec<Vec<board::Entry>>) -> bool {
    return check_rows(&board) && check_cols(&board) && check_squares(&board);
}

fn fill_board(mut board: &mut Vec<Vec<board::Entry>>) {
    fn fill_board_rec(mut board: &mut Vec<Vec<board::Entry>>, row: usize, col: usize) -> bool {
        for n in 1..(board.len() + 1) {
            println!("{}, {}", row, col);
            board[row][col] = board::Entry::Num(n as u8);
            print_board(&board);
            println!("{}", check_board(&board));
            if check_board(&board) {
                let (row, col) = 
                    if row == board.len()-1 && col == board.len()-1 {
                        return true;
                    } else if col == board.len()-1 {
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
    let mut board: Vec<Vec<board::Entry>> = vec![vec!(board::Entry::Empty; 9); 9];

    fill_board(&mut board);

    println!("{}", check_board(&board));

    print_board(&board);
}
