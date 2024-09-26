pub mod numbers;
pub mod lenght;
pub mod temperature;
pub mod adim;

use numbers::Numeric;

pub trait Unit<NumberType: Numeric>: Into<NumberType> {
    fn new(value: NumberType) -> Self;
    fn get_value(self) -> NumberType;

    fn plus<ConcreteUnitType>(self, other: ConcreteUnitType) -> Self
    where
        Self: Sized,
        ConcreteUnitType: Into<NumberType>,
    {
        Self::new(self.into() + other.into())
    }

    fn minus<T>(self, other: T) -> Self
    where
        Self: Sized,
        T: Into<NumberType>,
    {
        Self::new(self.into() - other.into())
    }

    fn mul<T>(self, other: T) -> Self
    where
        Self: Sized,
        T: Into<NumberType>,
    {
        Self::new(self.into() * other.into())
    }

    fn div<T>(self, other: T) -> Self
    where
        Self: Sized,
        T: Into<NumberType>,
    {
        Self::new(self.into() / other.into())
    }
}

