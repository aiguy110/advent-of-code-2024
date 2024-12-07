use std::ops::{Mul, Add, Sub, Index};
use std::hash::Hash;
use derive_more::Display;

// Error / Result
#[derive(Display, Debug)]
pub enum Error {
    InconsitantRowLengths
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// GridVec
#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct GridVec {
    pub i: i32,
    pub j: i32
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
        GridVec { i: value[0], j: value[1] }
    }
}

impl From<[usize; 2]> for GridVec {
    fn from(value: [usize; 2]) -> Self {
        GridVec { i: value[0] as i32, j: value[1] as i32 }
    }
}

impl Mul<i32> for GridVec {
    type Output = GridVec;

    fn mul(self, rhs: i32) -> Self::Output {
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

impl Add for GridVec {
    type Output = GridVec;

    fn add(self, rhs: Self) -> Self::Output {
        GridVec {
            i: self.i + rhs.i,
            j: self.j + rhs.j
        }
    }
}

impl Sub for GridVec {
    type Output = GridVec;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -1*rhs
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
}

impl<T> Index<GridVec> for Grid<T> {
    type Output = T;

    fn index(&self, index: GridVec) -> &Self::Output {
        self.get(index).unwrap()
    }
}
