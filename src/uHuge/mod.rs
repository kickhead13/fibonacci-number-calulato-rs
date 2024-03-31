use std::fmt;
use std::ops::Add;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct uHuge<const N: usize> {
    pub digits: [u8; N],
    pub len: usize
}

impl<const N: usize> From<u128> for uHuge<N> {

    fn from(integer: u128) -> Self {
        let mut sav = integer;
        let mut returnee: Self = Self::new();
        let mut iter: usize = ( returnee.len - 1 ) as usize;
        while iter > 0 {
            returnee.digits[iter] = ( sav % 10 ) as u8;
            sav /= 10;
            iter -= 1;
        }
        return returnee;
    }

}

impl<const N: usize>  fmt::Display for uHuge<N> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first_non_zero: bool = false;
        let mut result_string = format!("");
        for digit in self.digits {
            if digit != 0 || first_non_zero {
                result_string = format!("{}{}", result_string, digit);
                if ! first_non_zero {
                    first_non_zero = true;
                }
            }
        }
        write!(f, "{}", result_string)
    }

}

impl<const N: usize>  Add for uHuge<N> {
    type Output = Self;

    fn add(self, other: Self ) -> Self {
        let mut carry: u8 = 0;
        let mut iter_self: usize = ( self.len - 1 ) as usize;
        let mut iter_other: usize = ( other.len - 1 ) as usize;
        let mut returnee: Self = Self::new();
        let mut iter_returnee: usize = ( returnee.len - 1 ) as usize;
        while iter_self > 0 && iter_other > 0 {
            returnee.digits[iter_returnee] = 
                (self.digits[iter_self] 
                + other.digits[iter_other] 
                + carry) 
                % 10;
            carry = 
                (self.digits[iter_self] 
                + other.digits[iter_other] 
                + carry) 
                /10;
            iter_self -= 1;
            iter_other -= 1;
            iter_returnee -= 1;
        }
        return returnee;
    }
}

impl<const N: usize> uHuge<N> {

    pub fn new() -> Self {
        Self {
            digits: [0; N],
            len: N
        }
    }

}