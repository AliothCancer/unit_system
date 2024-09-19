use super::Unit;

pub trait ToTemperature
where
    Self: PartialEq + PartialOrd + Sized,
{
    fn celsius(self) -> Celsius<Self> {
        Celsius(self)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Celsius<T: PartialEq + PartialOrd>(pub T);

impl ToTemperature for f64 {}

impl Unit<f64> for Celsius<f64> {
    fn new(value: f64) -> Self {
        Celsius(value)
    }

    fn get_value(self) -> f64 {
        self.0
    }
}

impl Into<f64> for Celsius<f64> {
    fn into(self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus() {
        let c1 = Celsius(5.0);
        let c2 = Celsius(3.0);
        let result = c1.plus(c2);
        assert_eq!(Into::<f64>::into(result), 8.0);

        let c1 = Celsius(5.0);
        let result = c1.plus(3.0);
        assert_eq!(Into::<f64>::into(result), 8.0);
    }

    #[test]
    fn test_minus() {
        let c1 = Celsius(5.0);
        let c2 = Celsius(3.0);
        let result = c1.minus(c2);
        assert_eq!(Into::<f64>::into(result), 2.0);

        let c1 = Celsius(5.0);
        let result = c1.minus(3.0);
        assert_eq!(Into::<f64>::into(result), 2.0);
    }

    #[test]
    fn test_mul() {
        let c1 = Celsius(5.0);
        let c2 = Celsius(3.0);
        let result = c1.mul(c2);
        assert_eq!(Into::<f64>::into(result), 15.0);

        let c1 = Celsius(5.0);
        let result = c1.mul(3.0);
        assert_eq!(Into::<f64>::into(result), 15.0);
    }

    #[test]
    fn test_div() {
        let c1 = Celsius(6.0);
        let c2 = Celsius(3.0);
        let result = c1.div(c2);
        assert_eq!(Into::<f64>::into(result), 2.0);

        let c1 = Celsius(6.0);
        let result = c1.div(3.0);
        assert_eq!(Into::<f64>::into(result), 2.0);
    }
}
