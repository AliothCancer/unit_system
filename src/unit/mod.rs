pub mod lenght;
pub mod temperature;

use std::ops::{Add, Div, Mul, Sub};

enum Other<RawValue, GenericUnit>
where
    RawValue: Add<Output = RawValue>
        + Sub<Output = RawValue>
        + Div<Output = RawValue>
        + Mul<Output = RawValue>,
    GenericUnit: Unit<RawValue>,
{
    Unit(GenericUnit),
    Value(RawValue),
}


pub trait Unit<
    RawValue: Add<Output = RawValue>
        + Mul<Output = RawValue>
        + Div<Output = RawValue>
        + std::ops::Sub<Output = RawValue>,
>: Into<RawValue>
{
    fn new(value: RawValue) -> Self;
    fn get_value(self) -> RawValue;

    fn plus<T: Into<RawValue>>(self, other: T) -> Self
    where
        Self: Sized,
    {
        Unit::new(self.into() + Into::<RawValue>::into(other))
    }
    fn minus(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.into() - other.into())
    }
    fn mul(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.into() * other.into())
    }
    fn div(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.into() / other.into())
    }
}
