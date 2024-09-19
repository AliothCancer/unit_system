pub trait ToTemperature
where
    Self: PartialEq + PartialOrd + Sized,
{
    fn celsius(self) -> Celsius<Self> {
        Celsius(self)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Celsius<T: PartialEq + PartialOrd>(T);
impl Celsius<f64> {
    pub fn plus(self, value: Self) -> Self {
        Celsius(self.0 + value.0)
    }
}

impl ToTemperature for f64{}