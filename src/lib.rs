
pub struct Grid<T>(pub Vec<Vec<T>>);
pub type Loc = (usize, usize);

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
}

impl Grid<char> {
    pub fn from_string(s: &str) -> Grid<char> {
        Grid(s.lines().map(|line| line.chars().collect()).collect())
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
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