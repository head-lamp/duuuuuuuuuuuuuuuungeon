use std::ops::{Add, Sub};

#[derive(clone)]
pub struct Pos<T: Add + Sub + Clone> {
    pub x: T,
    pub y: T,
}

/*
//todo fixme implement this shit
impl<T: Add + Clone> Add for Pos<T> { /* ... */ }

impl<T: Sub + Clone> Sub for Pos<T> { 
}
*/
