use std::collections::BTreeSet;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign };
use std::cmp::{PartialOrd, Ord};
use std::hash::Hash;
use derive_more::Display;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BRIGHT_BG: &str = "\x1b[1;47m\x1b[1;30m";

// Error / Result
#[derive(Display, Debug)]
pub enum Error {
    InconsitantRowLengths
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// GridVec
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct GridVec {
    pub i: i64,
    pub j: i64
}

impl GridVec {
    /// Returns a copy of `self` which as been rotated 90 degrees clockwise
    pub fn rot_90(&self) -> GridVec {
        GridVec {
            i: -self.j,
            j: self.i
        }
    }

    /// Returns a copy of `self` which as been rotated 90 degrees anti-clockwise
    pub fn rot_90_anti(&self) -> GridVec {
        GridVec {
            i: self.j,
            j: -self.i
        }
    }
}

impl From<[i32; 2]> for GridVec {
    fn from(value: [i32; 2]) -> Self {
        GridVec { i: value[0] as i64, j: value[1] as i64 }
    }
}

impl From<[i64; 2]> for GridVec {
    fn from(value: [i64; 2]) -> Self {
        GridVec { i: value[0], j: value[1] }
    }
}

impl From<[usize; 2]> for GridVec {
    fn from(value: [usize; 2]) -> Self {
        GridVec { i: value[0] as i64, j: value[1] as i64 }
    }
}

impl Mul<i32> for GridVec {
    type Output = GridVec;

    fn mul(self, rhs: i32) -> Self::Output {
        GridVec { 
            i: self.i * rhs as i64, 
            j: self.j * rhs as i64
        }
    }
}

impl Mul<i64> for GridVec {
    type Output = GridVec;

    fn mul(self, rhs: i64) -> Self::Output {
        GridVec { 
            i: self.i * rhs, 
            j: self.j * rhs
        }
    }
}

impl Mul<GridVec> for i32 {
    type Output = GridVec;

    fn mul(self, rhs: GridVec) -> Self::Output {
        rhs*self
    }
}

impl Mul<GridVec> for i64 {
    type Output = GridVec;

    fn mul(self, rhs: GridVec) -> Self::Output {
        rhs*self
    }
}

impl MulAssign<i32> for GridVec {
    fn mul_assign(&mut self, rhs: i32) {
        let _ = std::mem::replace(self, *self * rhs);
    }
}

impl MulAssign<i64> for GridVec {
    fn mul_assign(&mut self, rhs: i64) {
        let _ = std::mem::replace(self, *self * rhs);
    }
}


impl Add for GridVec {
    type Output = GridVec;

    fn add(self, rhs: Self) -> Self::Output {
        GridVec {
            i: self.i + rhs.i,
            j: self.j + rhs.j
        }
    }
}

impl AddAssign for GridVec {
    fn add_assign(&mut self, rhs: Self) {
        let _ = std::mem::replace(self, *self + rhs);
    }
}

impl Sub for GridVec {
    type Output = GridVec;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -1*rhs
    }
}

impl SubAssign for GridVec {
    fn sub_assign(&mut self, rhs: Self) {
        let _ = std::mem::replace(self, *self - rhs);
    }
}

// Grid
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
    pub row_count: usize,
    pub col_count: usize
}

impl<T> Grid<T> {
    pub fn from_iter(row_it: impl Iterator<Item = impl Iterator<Item = T>>) -> Result<Self> {
        let mut grid = Grid {
            rows: Vec::new(),
            row_count: 0,
            col_count: 0
        };
        let mut first_row = true;
        for item_it in row_it {
            let mut row = vec![];
            for item in item_it {
                row.push(item); 
            }

            if first_row {
                grid.col_count = row.len();
                first_row = false;
            } else if !first_row && row.len() != grid.col_count {
                return Err(Error::InconsitantRowLengths);
            }

            grid.rows.push(row)
        }
        grid.row_count = grid.rows.len();

        Ok(grid)
    }

    pub fn get(&self, loc: GridVec) -> Option<&T> {
        if loc.i < 0
            || loc.j < 0 
            || loc.i as usize >= self.row_count 
            || loc.j as usize >= self.col_count 
        {
            return None
        }

        Some(&self.rows[loc.i as usize][loc.j as usize])
    }

    pub fn get_mut(&mut self, loc: GridVec) -> Option<&mut T> {
        if loc.i < 0
            || loc.j < 0 
            || loc.i as usize >= self.row_count 
            || loc.j as usize >= self.col_count 
        {
            return None
        }

        Some(&mut self.rows[loc.i as usize][loc.j as usize])
    }

}

impl Grid<char> {
    pub fn render(&self) {
        let highlights: BTreeSet<GridVec> = BTreeSet::new();
        for i in 0..self.row_count {
            for j in 0..self.col_count {
                let loc = GridVec::from([i, j]);
                let hl = highlights.contains(&loc);
                if hl {
                    print!("{}", ANSI_BRIGHT_BG);
                }
                print!("{}", self[loc]);
                if hl {
                    print!("{}", ANSI_RESET);
                }
            }
            println!();
        }
    }

    pub fn render_with_highlights(&self, highlights: &BTreeSet<GridVec>) {
        for i in 0..self.row_count {
            for j in 0..self.col_count {
                let loc = GridVec::from([i, j]);
                let hl = highlights.contains(&loc);
                if hl {
                    print!("{}", ANSI_BRIGHT_BG);
                }
                print!("{}", self[loc]);
                if hl {
                    print!("{}", ANSI_RESET);
                }
            }
            println!();
        }
    }
}

impl<T> Index<GridVec> for Grid<T> {
    type Output = T;

    fn index(&self, index: GridVec) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<GridVec> for Grid<T> {
    fn index_mut(&mut self, index: GridVec) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
