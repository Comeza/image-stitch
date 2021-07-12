use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug)]
pub struct Dimension<T> {
    pub x: T,
    pub y: T,
}

impl<T> Dimension<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Dimension<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl Dimension<u32> {
    pub fn to_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl Add for Dimension<u32> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + self.y)
    }
}

impl Sub for Dimension<u32> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - self.y)
    }
}

impl AddAssign for Dimension<u32> {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y)
    }
}

impl SubAssign for Dimension<u32> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.x - other.x, self.y - other.y)
    }
}
