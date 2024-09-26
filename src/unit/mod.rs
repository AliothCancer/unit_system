pub mod lenght;
pub mod numbers;
pub mod temperature;


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

#[derive(Debug)]
pub struct Adim<NumberType: Numeric>(NumberType);
pub trait ToAdim where Self: Numeric{
    fn adim(self) -> Adim<Self>{
        Adim(self)
    } 
}
impl<NumberType: Numeric> ToAdim for NumberType{}

impl<NumberType> Unit<NumberType> for Adim<NumberType>
where NumberType: Numeric + From<Adim<NumberType>>{
    fn new(value: NumberType) -> Self {
        Self(value)
    }

    fn get_value(self) -> NumberType {
        self.0
    }
}