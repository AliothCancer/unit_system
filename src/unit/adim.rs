use super::{numbers::Numeric, Unit};

#[derive(Debug)]
pub struct Adim<NumberType: Numeric>(NumberType);

pub trait ToAdim
where
    Self: Numeric,
{
    fn adim(self) -> Adim<Self> {
        Adim(self)
    }
}
impl<NumberType: Numeric> ToAdim for NumberType {}

impl<NumberType> Unit<NumberType> for Adim<NumberType>
where
    NumberType: Numeric + From<Adim<NumberType>>
{
    fn new(value: NumberType) -> Self {
        Self(value)
    }

    fn get_value(self) -> NumberType {
        self.0
    }
}

impl From<Adim<f64>> for f64{
    fn from(value: Adim<f64>) -> Self {
        value.0
    }
}

impl From<Adim<i32>> for i32{
    fn from(value: Adim<i32>) -> Self {
        value.0
    }
}