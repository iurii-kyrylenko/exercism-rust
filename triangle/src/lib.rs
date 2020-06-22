use std::ops::Add;

pub struct Triangle<T> {
    sides: Vec<T>,
}

impl<T: Add<Output = T> + Clone + Copy + PartialOrd> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        // Create sorted set manually (cannot use BTreeSet because
        // the trait Ord is not implemented for floats)
        let mut vec: Vec<T> = sides.to_vec();
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vec.dedup();

        // Check for zero side
        if vec[0] + vec[0] == vec[0] {
            return None;
        }

        // Exclude invalid triangles
        match vec.len() {
            2 if vec[0] + vec[0] < vec[1] => None,
            3 if vec[0] + vec[1] < vec[2] => None,
            _ => Some(Triangle { sides: vec }),
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
