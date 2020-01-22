use std::ops::{Add, Sub};
use specs::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Pos<T> {
    type Output = Self;

    fn add(self, other: Pos<T>) -> Self::Output {
        Pos::<T> {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for Pos<T> {
    type Output = Self;

    fn sub(self, other: Pos<T>) -> Self::Output {
        Pos::<T> {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
