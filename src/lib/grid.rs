use crate::Dir;
use crate::Loc;

use std::ops::{Index, IndexMut};

pub struct Grid<T>(pub Vec<Vec<T>>);

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

impl<T> Grid<T>
where
    T: Eq,
{
    pub fn find_item(&self, needle: &T) -> Option<Loc> {
        self.iter_with_loc()
            .filter_map(|(loc, val)| if val == needle { Some(loc) } else { None })
            .next()
    }
}

pub struct GridIterWithLoc<'a, T> {
    grid: &'a Grid<T>,
    r: usize,
    c: usize,
}

impl<'a, T> GridIterWithLoc<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        GridIterWithLoc { grid, r: 0, c: 0 }
    }
}

impl<'a, T> Iterator for GridIterWithLoc<'a, T> {
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

impl<T> Index<&Loc> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Loc) -> &Self::Output {
        let (r, c) = *index;
        &self.0[r][c]
    }
}

impl<T> IndexMut<&Loc> for Grid<T> {
    fn index_mut(&mut self, index: &Loc) -> &mut Self::Output {
        let (r, c) = *index;
        &mut self.0[r][c]
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(n_rows: usize, n_cols: usize, value: T) -> Self {
        Grid(
            (0..n_rows)
                .map(|_| (0..n_cols).map(|_| value).collect())
                .collect(),
        )
    }

    pub fn set_all(&mut self, val: T) {
        for i in 0..self.n_rows() {
            for j in 0..self.n_cols() {
                self.0[i][j] = val
            }
        }
    }
}

impl Grid<char> {
    pub fn from_string(s: &str) -> Self {
        Grid(
            s.trim()
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
        )
    }
}
