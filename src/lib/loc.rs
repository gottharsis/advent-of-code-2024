use crate::Vec2;
pub type Loc = (usize, usize);

pub fn euclidean_distance(a: Loc, b: Loc) -> f64 {
    let (a1, a2) = a;
    let (b1, b2) = b;
    let a = Vec2::new(a1 as i32, a2 as i32);
    let b = Vec2::new(b1 as i32, b2 as i32);
    (a - b).magnitude()
}

pub fn manhattan_distance(a: Loc, b: Loc) -> usize {
    let d0 = ((a.0 as i64) - (b.0 as i64)).unsigned_abs() as usize;
    let d1 = ((a.1 as i64) - (b.1 as i64)).unsigned_abs() as usize;
    d0 + d1
}
