use crate::{Field, GenRandom};
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};


/// Represents an element iusen the field GF(2**8) without the use of lookup tables.

#[derive(Eq, Debug, Clone, Copy)]
pub struct GF256(pub u8);

impl GF256 {
    //reference https://github.com/zkcrypto/ff/blob/661558e0c8a5e02e08dac6530d39b2e38919aa04/src/lib.rs#L55
    pub fn pow(self, elem: u8) -> Self {
        let mut res = GF256::one();
        for i in 0..8 {
            res.square();
            let mut tmp = res;

            tmp.mul_assign(self);

            res.conditional_assign(&tmp, (((elem >> i) & 0x01) as u8).into());
        }

        res.conditional_assign(&GF256::one(),(elem.ct_eq(&0)).into());
      
            res
    }
}



impl GenRandom for GF256 {
    fn gen_random() -> Self {
        GF256(rand::thread_rng().gen())
    }
}



impl Field for GF256 {
    /// Returns the zero element of the field (additive identity)
    fn zero() -> Self {
        GF256(0)
    }

    /// Returns the zero element of the field (multiplicative identity)
    fn one() -> Self {
        GF256(1)
    }

    /// Returns true if this element is the additive identity
    fn is_zero(&self) -> bool {
        self.0.ct_eq(&0).into()
    }

    /// Squares the element
    fn square(&mut self) {
        self.mul_assign(*self);
    }

    /// Returns multiplicative inverse (self^254)
    fn inverse(&self) -> Option<Self> {
        let mut res = *self;

        for _ in 0..6 {
            res.square();
            res.mul_assign(*self);
        }

        res.square();
        res.conditional_assign(&GF256::zero(), self.0.ct_eq(&0x00));

        Some(res)
    }
}

impl ConditionallySelectable for GF256 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        GF256(u8::conditional_select(&a.0, &b.0, choice))
    }
}

impl ConstantTimeEq for GF256 {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}






impl Display for GF256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }

}

impl PartialEq for GF256 {
    fn eq(&self, other: &GF256) -> bool {
        self.0.ct_eq(&other.0).into()
    }
}

impl Add for GF256 {
    type Output = GF256;

    fn add(self, other: GF256) -> GF256 {
        let mut result = self;
        result.add_assign(other);
        result
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign for GF256 {
    fn add_assign(&mut self, other: GF256) {
        *self = GF256(self.0 ^ other.0);
    }
}

impl Sub for GF256 {
    type Output = GF256;

    fn sub(self, other: GF256) -> GF256 {
        let mut result = self;
        result.sub_assign(other);
        result
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl SubAssign for GF256 {
    fn sub_assign(&mut self, other: GF256) {
        *self = GF256(self.0 ^ other.0);
    }
}

impl Mul for GF256 {
    type Output = GF256;

    fn mul(self, rhs: GF256) -> GF256 {
        let mut result = self;
        result.mul_assign(rhs);
        result
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl MulAssign for GF256 {
    fn mul_assign(&mut self, rhs: GF256) {
        let a = self.0;
        let mut b = rhs.0;
        // let mut t: u8;
        self.0 = 0x00;
        for i in 0..8 {

            let lsb_of_a_not_0 = !(((a >> i) & 0x01).ct_eq(&0x00));
            self.conditional_assign(&GF256(self.0 ^ b), lsb_of_a_not_0);

            let choice = (b & 0x80).ct_eq(&0x00);
            b <<= 1;
            let tmp = b ^ 0x1b;
            b.conditional_assign(&tmp, !choice);
        }
    }
}

impl Div for GF256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = self;
        result.div_assign(rhs);
        result
    }
}

impl DivAssign for GF256 {
    fn div_assign(&mut self, rhs: GF256) {
        *self *= rhs.inverse().unwrap();
    }
}

impl Neg for GF256 {
    type Output = GF256;

    fn neg(self) -> GF256 {
        self
    }
}



