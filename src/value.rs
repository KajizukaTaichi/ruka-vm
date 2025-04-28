use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    pub sign: bool,
    pub num: usize,
    pub den: usize,
}

impl Value {
    pub fn new(number: f64) -> Self {
        Value {
            sign: number.is_sign_positive(),
            num: number.abs() as usize,
            den: 1,
        }
    }

    pub fn as_f64(&self) -> f64 {
        if self.sign {
            self.num as f64 / self.den as f64
        } else {
            -(self.num as f64 / self.den as f64)
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! denominator {
            () => {
                if self.den == 1 {
                    String::new()
                } else {
                    format!("/{}", self.den)
                }
            };
        }
        if self.sign {
            write!(f, "{}{}", self.num, denominator!())
        } else {
            write!(f, "-{}{}", self.num, denominator!())
        }
    }
}
