use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug)]
pub struct Dimensions<T> {
    pub x: T,
    pub y: T,
}

impl<T> Dimensions<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_tuple(dim: (T, T)) -> Self {
        Self::new(dim.0, dim.1)
    }
}

impl Dimensions<u32> {
    pub fn to_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl Add for Dimensions<u32> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + self.y)
    }
}

impl Sub for Dimensions<u32> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - self.y)
    }
}

impl AddAssign for Dimensions<u32> {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y)
    }
}

impl SubAssign for Dimensions<u32> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.x - other.x, self.y - other.y)
    }
}
