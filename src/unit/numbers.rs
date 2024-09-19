use std::ops::{Add, Div, Mul, Sub};

pub trait Numeric:
    PartialOrd
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    where Self: Sized
{
}


impl Numeric for f64 {}
impl Numeric for f32 {}
impl Numeric for i32 {}
impl Numeric for u32 {}
impl Numeric for i64 {}
impl Numeric for u64 {}
