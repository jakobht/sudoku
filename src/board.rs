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
        self.board.iter().all(|row| {
            let mut hm = SimpleHash::new(self.size());
            row.iter().all(|v| hm.insert(v))
        })
    }

    fn check_cols(&self) -> bool {
        (0..self.size()).all(|i| {
            let mut hm = SimpleHash::new(self.size());
            (0..self.size()).all(|j| hm.insert(&self[j][i]) )
        })
    }

    fn check_square(&self, square: usize) -> bool {
        let mut hm = SimpleHash::new(self.size());
        let s_row = square / self.square_size();
        let s_col = square % self.square_size();
        let sq = self.square_size();

        (s_row*sq..s_row*sq+sq)
        .all(|row| (s_col*sq..s_col*sq+sq)
        .all(|col| hm.insert(&self[row][col])))
    }

    fn check_squares(&self) -> bool {
        (0..self.size()).all(|s| self.check_square(s))
    }

    pub fn check_board(&self) -> bool {
        self.check_rows() && self.check_cols() && self.check_squares()
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