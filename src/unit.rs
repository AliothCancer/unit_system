use std::ops::{Add, Div, Mul};

pub trait Unit<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + std::ops::Sub<Output = T>> {
    fn new(value: T) -> Self;
    fn get_value(self) -> T;

    fn plus(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.get_value() + other.get_value())
    }
    fn minus(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.get_value() - other.get_value())
    }
    fn mul(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.get_value() * other.get_value())
    }
    fn div(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.get_value() / other.get_value())
    }
}
