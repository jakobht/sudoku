use std::ops::{Index, IndexMut};

struct SimpleHash {
    vec: Vec<bool>
}

impl SimpleHash {
    fn new(size: usize) -> SimpleHash {
        SimpleHash { vec: vec!(false; size) }
    }

    fn insert(&mut self, e: &Entry) -> bool {
        match e {
            Entry::Empty => true,
            Entry::Num(n) if self.vec[(n - 1) as usize] => false,
            Entry::Num(n)  => { self.vec[(n - 1) as usize] = true; true }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Entry {
    Empty,
    Num(u8),
}

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

    fn check_rows(&self) -> bool {
        for row in &self.board {
            let mut hm = SimpleHash::new(self.size());
            for val in row {
                if !hm.insert(val) { return false }
            }
        }
        true
    }

    fn check_cols(&self) -> bool {
        for i in 0..self.size() {
            let mut hm = SimpleHash::new(self.size());
            for j in 0..self.size() {
                if !hm.insert(&self[j][i]) { return false }
            }
        }
        true
    }

    fn check_square(&self, square: usize) -> bool {
        let mut hm = SimpleHash::new(self.size());
        let s_row = square / 3;
        let s_col = square % 3;

        for row in s_row*3..s_row*3+3 {
            for col in s_col*3..s_col*3+3 {
                if !hm.insert(&self[row][col]) { return false }
            }
        }

        true
    }

    fn check_squares(&self) -> bool {
        for square in 0..self.size() {
            if !self.check_square(square) { return false }
        }
        true
    }

    pub fn check_board(&self) -> bool {
        return self.check_rows() && self.check_cols() && self.check_squares();
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
                    Entry::Empty => write!(f, "_ ")?,
                    Entry::Num(n) => write!(f, "{} ", n)?,
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