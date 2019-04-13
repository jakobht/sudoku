use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub enum Entry {
    Empty,
    Num(u8),
    Clue(u8)
}

#[derive(Clone)]
pub struct Board {
    pub board: Vec<Vec<Entry>>,
    size: usize,
    square_size: usize
}

impl Board {
    pub fn size(&self) -> usize { self.size }
    pub fn square_size(&self) -> usize { self.square_size } 

    pub fn new(size: usize) -> Board {
        Board {
            board: vec![vec!(Entry::Empty; size); size],
            size: size,
            square_size: (size as f64).sqrt() as usize
        }
    }

    pub fn from_str(s: &str) -> Board {
        let mut board = Board::new(9);
        for (i, line) in s.lines().enumerate() {
            for (j, n) in line.split_whitespace().enumerate() {
                board[i][j] = match n.parse() {
                    Ok(n) => Entry::Clue(n),
                    Err(_) => Entry::Empty
                }
            }
        }
    board
}

}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            if i % self.square_size() == 0 && i != 0 {
                for _ in 0..self.size() + self.square_size()-1 {
                    write!(f, "--")?;
                }
                writeln!(f, "")?;
            }

            for (j, num) in row.iter().enumerate() {
                if j % self.square_size() == 0 && j != 0 {
                    write!(f, "| ")?
                };
                match num {
                    Entry::Empty => write!(f, " _  ")?,
                    Entry::Num(n) => write!(f, " {}  ", n)?,
                    Entry::Clue(n) => write!(f, "-{}- ", n)?,
                }
            }
            writeln!(f, "")?;
        }
        Result::Ok(())
    }
}

impl Index<usize> for Board {
    type Output = Vec<Entry>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Entry> {
        &mut self.board[index]
    }
}