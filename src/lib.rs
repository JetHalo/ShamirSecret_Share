
//! ShamirSecret_Share is a Rust libary for secret sharing
//!


#[cfg(test)]
extern crate quickcheck;
extern crate rand;
extern crate subtle;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// gf256 is a module for field elements over the field GF(2^8) with irreducible polynomial x^8+x^4+x^3+x+1
///
/// 
////// # Examples
///
/// All elements are their own additive inverse
/// GF256::zero() is the additive identity
///
/// ```
/// use shamir_share::ff_gf256::GF256;
/// use shamir_share::Field;
///
/// let x = GF256(80);
/// let x_plus_x = x + x;
///
/// assert_eq!(x_plus_x, GF256::zero());
/// ```
///
/// All elements except zero have inverses
///
/// ```
/// use shamir_share::ff_gf256::GF256;
/// use shamir_share::Field;
///
/// let x = GF256(80);
/// let x_mul_x = x.inverse().unwrap() * x;
///
/// assert_eq!(x_mul_x, GF256::one());
/// ```

mod lagrange_interpolation;
mod polynomial;
pub mod ff_gf256;
pub mod shamir_secret;

/// Shamir is a module for shamir secret sharing.
///
/// # Example
///
/// The following code splits a secret (a slice of bytes) into n shares
/// of which k are required to recover the secret.
///
/// ```
/// extern crate rand;
///
/// use rand::{thread_rng,Rng};
/// use shamir_share::shamir_secret::Shamir;
///
/// let k = 3;
/// let n = 5;
/// let size = 32;
///
/// // Generate a random secret
/// let mut random_secret = Vec::with_capacity(size);
/// (0..size).for_each(|_| random_secret.push(thread_rng().gen::<u8>()));
///
/// // Split this random secret into n shares, of which k are needed to recover the secret
/// let shares = Shamir::separate(&random_secret, k, n).unwrap();
///
/// // Combine the shares to recover the secret
/// let combined = Shamir::recover(&shares);
/// assert_eq!(combined, random_secret);
/// ```
///






pub trait GenRandom {
    fn gen_random() -> Self;
}



/// This trait represents an element of a field.
pub trait Field:
Sized
+ Eq
+ Copy
+ Clone
+ Send
+ Sync
+ std::fmt::Debug
+ std::fmt::Display
+ 'static
+ Add<Output = Self>
+ AddAssign
+ Div<Output = Self>
+ DivAssign
+ Mul<Output = Self>
+ MulAssign
+ Neg<Output = Self>
+ Sub<Output = Self>
+ SubAssign
+ GenRandom
{
    /// Returns the zero element of the field, the additive identity.
    fn zero() -> Self;

    /// Returns the one element of the field, the multiplicative identity.
    fn one() -> Self;

    /// Returns true if this element is zero.
    fn is_zero(&self) -> bool;

    /// Computes the multiplicative inverse of this element, if nonzero.
    fn inverse(&self) -> Option<Self>;

    /// Squares this element.
    fn square(&mut self);
}
