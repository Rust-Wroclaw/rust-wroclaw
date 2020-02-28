use std::ops;

use serde::{Deserialize, Serialize};

use crate::math::Vector;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to(self, other: Self) -> Vector {
        Vector::new(
            (self.x - other.x) as f32,
            (self.y - other.y) as f32,
        )
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x as i32,
            y: self.y + rhs.y as i32,
        }
    }
}
