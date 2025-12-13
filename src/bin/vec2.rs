use std::ops::{Add, Sub};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, b: Vec2) -> <Self as Add<Vec2>>::Output {
        Self::Output {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, b: Vec2) -> <Self as Add<Vec2>>::Output {
        Self::Output {
            x: self.x - b.x,
            y: self.y - b.y,
        }
    }
}
