use std::collections::BTreeSet;
use std::ops::Add;

#[derive(Debug)]
pub struct Triangle<T> {
    // Unfortunately, the trait `std::cmp::Ord` is not implemented for `{float}`
    sides: BTreeSet<T>,
}

impl<T: Add<Output = T> + Clone + Copy + Ord> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let set = sides.iter().cloned().collect::<BTreeSet<T>>();

        let vec = set.iter().collect::<Vec<_>>();

        // Check for zero
        if *vec[0] + *vec[0] == *vec[0] {
            return None;
        }

        match vec.len() {
            2 if *vec[0] + *vec[0] < *vec[1] => None,
            3 if *vec[0] + *vec[1] < *vec[2] => None,
            _ => Some(Triangle { sides: set }),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.len() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.len() == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.len() == 3
    }
}
