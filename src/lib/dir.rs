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

    pub fn rotate_clockwise(&self) -> Self {
        use Dir::*;
        match self {
            N => E,
            NE => SE,
            E => S,
            SE => SW,
            S => W,
            SW => NW,
            W => N,
            NW => NE,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        use Dir::*;
        match self {
            E => N,
            SE => NE,
            S => E,
            SW => SE,
            W => S,
            NW => SW,
            N => W,
            NE => NW,
        }
    }

    pub fn opposite(&self) -> Self {
        use Dir::*;
        match self {
            N => S,
            NE => SW,
            E => W,
            SE => NW,
            S => N,
            SW => NE,
            W => E,
            NW => SE,
        }
    }
}
