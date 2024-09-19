pub trait ToLength
where
    Self: PartialEq + PartialOrd + Sized,
{
    fn meters(self) -> Meters<Self> {
        Meters(self)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Meters<T: PartialEq + PartialOrd>(T);
impl Meters<f64> {
    pub fn plus(self, value: Self) -> Self {
        Meters(self.0 + value.0)
    }
}

impl ToLength for f64 {}
impl ToLength for f32 {}
impl ToLength for i64 {}
