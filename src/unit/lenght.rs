// lenght.rs
use super::Unit;

pub trait ToLength
where
    Self: PartialEq + PartialOrd + Sized,
{
    fn meters(self) -> Meters<Self> {
        Meters(self)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Meters<T: PartialEq + PartialOrd>(pub T);
impl Unit<f64> for Meters<f64>{
    fn new(value: f64) -> Self {
        Meters(value)
    }
    fn get_value(self) -> f64 {
        self.0
    }
}
impl Into<f64> for Meters<f64>{
    fn into(self) -> f64 {
        self.0
    }
}

impl ToLength for f64{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus() {
        let m1 = Meters(5.0);
        let m2 = Meters(5.0);
        let result = m1.plus(m2);
        assert_eq!(Into::<f64>::into(result), 10.0_f64);
    }

    #[test]
    fn test_plus_variant() {
        let m1 = Meters(5.0);
        let result = m1.plus(5.0);
        assert_eq!(Into::<f64>::into(result), 10.0_f64);
    }


    #[test]
    fn test_minus() {
        let m1 = Meters(5.0);
        let m2 = Meters(5.0);
        let result = m1.minus(m2);
        assert_eq!(Into::<f64>::into(result), 0.0);
    }

    #[test]
    fn test_mul() {
        let m1 = Meters(5.0);
        let m2 = Meters(5.0);
        let result = m1.mul(m2);
        assert_eq!(Into::<f64>::into(result), 25.0);
    }

    #[test]
    fn test_div() {
        let m1 = Meters(5.0);
        let m2 = Meters(5.0);
        let result = m1.div(m2);
        assert_eq!(Into::<f64>::into(result), 1.0);
    }
}
