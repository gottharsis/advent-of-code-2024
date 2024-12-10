
pub struct Grid<T>(pub Vec<Vec<T>>);
pub type Loc = (usize, usize);
use std::ops::{Index, IndexMut};

impl<T> Grid<T> {
    pub fn n_rows(&self) -> usize {
        self.0.len()
    }
    pub fn n_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn in_bounds(&self, loc: &Loc) -> bool {
        let (r, c) = loc;
        (*r < self.n_rows()) && (*c < self.n_cols())
    }

    pub fn at(&self, loc: &Loc) -> Option<&T> {
        if !self.in_bounds(loc) {
            return None;
        }
        let (r, c) = *loc;
        Some(&self.0[r][c])
    }

    pub fn at_mut(&mut self, loc: &Loc) -> Option<&mut T> {
        if !self.in_bounds(loc) {
            None
        } else {
            let (r, c) = *loc;
            Some(&mut self.0[r][c])
        }
    }

    pub fn iter_rows(&self) -> std::slice::Iter<'_, Vec<T>> {
        self.0.iter()
    }

    pub fn iter_rows_mut(&mut self) -> std::slice::IterMut<'_, Vec<T>> {
        self.0.iter_mut()
    }

    // attempt to step one step from the current position 
    pub fn step(&self, loc: &Loc, direction: Dir) -> Option<Loc> {
        let (r, c) = *loc;
        let (dr, dc) = direction.delta();
        
        // out of bounds negative
        if (r == 0 && dr < 0) || (c == 0 && dc < 0) {
            return None;
        } 

        let nr = ((r as i32) + dr) as usize;
        let nc = ((c as i32) + dc) as usize;
        let new_pos = (nr, nc);
        if !self.in_bounds(&new_pos) {
            return None;
        }
        Some(new_pos)
    }

    pub fn iter_with_loc(&self) -> GridIterWithLoc<'_, T> {
        GridIterWithLoc::new(self)
    }
}

pub struct GridIterWithLoc<'a, T> {
    grid: &'a Grid<T>,
    r: usize,
    c: usize,
}

impl <'a, T> GridIterWithLoc<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        GridIterWithLoc{grid, r: 0, c: 0}
    }
}

impl <'a, T> Iterator for GridIterWithLoc<'a, T> {
    type Item = (Loc, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.r == self.grid.n_rows() {
            return None;
        }
        let idx = (self.r, self.c);
        let item = &self.grid[&idx];
        self.c += 1;
        if self.c == self.grid.n_cols() {
            self.c = 0;
            self.r += 1;
        }
        Some((idx, item))
    }
}

impl<T> Index <&Loc> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Loc) -> &Self::Output {
        let (r, c) = *index;
        &self.0[r][c]
    }
}

impl <T>IndexMut<&Loc> for Grid <T>{
    fn index_mut(&mut self, index: &Loc) -> &mut Self::Output {
        let (r, c) = *index; 
        &mut self.0[r][c]
    }
}

impl <T> Grid<T> 
    where T: Copy {
    pub fn new(n_rows: usize, n_cols: usize, value: T) -> Self {
        Grid((0..n_rows).map(|_| (0..n_cols).map(|_| value).collect()).collect())
    }
}

impl Grid<char> {
    pub fn from_string(s: &str) -> Self {
        Grid(s.trim().lines().map(|line| line.chars().collect()).collect())
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    pub fn delta(&self) -> (i32, i32) {
        use Dir::*;
        match self {
            N => (-1, 0),
            NE => (-1, 1),
            E => (0, 1),
            SE => (1, 1),
            S => (1, 0),
            SW => (1, -1),
            W => (0, -1),
            NW => (-1, -1),
        }
    }

    pub fn rotate_clockwise(dir: &Dir) -> Dir {
        use Dir::*;
        match dir {
            N => E,
            NE => SE,
            E => S,
            SE => SW,
            S => W,
            SW => NW,
            W => N,
            NW => NE
        }
    }
}
