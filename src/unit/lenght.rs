// lenght.rs
use super::{numbers::Numeric, Unit};

pub trait ToLength
where
    Self: Numeric,
{
    fn meters(self) -> Meters<Self> {
        Meters(self)
    }
}
impl<T: Numeric> ToLength for T{}

#[derive(Debug, Clone, Copy)]
pub struct Meters<T: Numeric>(T);

impl Unit<f64> for Meters<f64> {
    fn new(value: f64) -> Self {
        Meters(value)
    }
    fn get_value(self) -> f64 {
        self.0
    }
}

impl From<Meters<f64>> for f64 {
    fn from(val: Meters<f64>) -> Self {
        val.0
    }
}


// TESTING

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus() {
        let m1 = Meters(5.0);
        let m2 = Meters(3.0);
        let result = m1.plus(m2);
        assert_eq!(Into::<f64>::into(result), 8.0);

        let m1 = Meters(5.0);
        let result = m1.plus(3.0);
        assert_eq!(Into::<f64>::into(result), 8.0);
    }

    #[test]
    fn test_minus() {
        let m1 = Meters(5.0);
        let m2 = Meters(3.0);
        let result = m1.minus(m2);
        assert_eq!(Into::<f64>::into(result), 2.0);

        let m1 = Meters(5.0);
        let result = m1.minus(3.0);
        assert_eq!(Into::<f64>::into(result), 2.0);
    }

    #[test]
    fn test_mul() {
        let m1 = Meters(5.0);
        let m2 = Meters(3.0);
        let result = m1.mul(m2);
        assert_eq!(Into::<f64>::into(result), 15.0);

        let m1 = Meters(5.0);
        let result = m1.mul(3.0);
        assert_eq!(Into::<f64>::into(result), 15.0);
    }

    #[test]
    fn test_div() {
        let m1 = Meters(6.0);
        let m2 = Meters(3.0);
        let result = m1.div(m2);
        assert_eq!(Into::<f64>::into(result), 2.0);

        let m1 = Meters(6.0);
        let result = m1.div(3.0);
        assert_eq!(Into::<f64>::into(result), 2.0);
    }
}
