use std::fmt;

#[derive(Debug)]
pub struct f16 {
    pub sign: u16,
    pub exp: u16,
    pub mantissa: u16,
}

impl fmt::Binary for f16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016b}", (self.sign << 15) | (self.exp << 10) | self.mantissa)
    }
}
