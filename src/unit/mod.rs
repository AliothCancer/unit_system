pub mod lenght;
pub mod temperature;

use std::ops::{Add, Div, Mul, Sub};

pub trait Unit<
    RawValue: Add<Output = RawValue>
        + Mul<Output = RawValue>
        + Div<Output = RawValue>
        + Sub<Output = RawValue>,
>: Into<RawValue>
{
    fn new(value: RawValue) -> Self;
    fn get_value(self) -> RawValue;

    fn plus<T>(self, other: T) -> Self
    where
        Self: Sized,
        T: Into<RawValue>,
    {
        Self::new(self.into() + other.into())
    }

    fn minus<T>(self, other: T) -> Self
    where
        Self: Sized,
        T: Into<RawValue>,
    {
        Self::new(self.into() - other.into())
    }

    fn mul<T>(self, other: T) -> Self
    where
        Self: Sized,
        T: Into<RawValue>,
    {
        Self::new(self.into() * other.into())
    }

    fn div<T>(self, other: T) -> Self
    where
        Self: Sized,
        T: Into<RawValue>,
    {
        Self::new(self.into() / other.into())
    }
}
