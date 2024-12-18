use crate::Vec2;
pub type Loc = (usize, usize);

pub fn euclidean_distance(a: Loc, b: Loc) -> f64 {
    let (a1, a2) = a;
    let (b1, b2) = b;
    let a = Vec2::new(a1 as i32, a2 as i32);
    let b = Vec2::new(b1 as i32, b2 as i32);
    (a - b).magnitude()
}
