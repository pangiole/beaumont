//! A basic implementation of decimal numbers in fixed-point arithmetic.
//!
//! <div class="warning">This module is in an early stage of development and definitively
//! <strong>not</strong> production ready.<br>Use with caution!</div>
//!
//! # Representation
//! Decimal numbers are internally represented by a signed coefficient integer and a positive
//! scaling factor, such that the actual number is given by multiplying the coefficient by
//! the power of the negative scaling factor in base 10, as follows:
//!
//! $$
//! coefficient * 10 ^{-scaling}
//! $$
//!
//! For example, the decimal number `1234.56` is internally represented as `123456` (the coefficient)
//! and `2` (the scaling factor).
//!
//! # Usage
//! To create a decimal number, you can:
//! - invoke [`Decimal::new`] and pass in both the `coefficient` integer and the `scaling` factor,
//! - or invoke the [`str::parse`] method on a string to have it parsed as [`Decimal`] type
//!
//! ```rust
//! use beaumont::num::Decimal;
//!
//! let d1 = Decimal::new(123456, 2);
//! let d2 = "1234.567".parse::<Decimal>().unwrap();
//! ```
//! Once created, you can apply mathematical operations such as negating, adding, multiplying, etc.
//!
//! ```rust
//! # use beaumont::num::Decimal;
//! # let d1 = Decimal::new(123456, 2);
//! # let d2 = Decimal::new(1234567, 3);
//! let negated = -d1;
//! assert_eq!(negated.to_string(), "-1234.56");
//! ```
//!
//! # Memory
//! To make them look alike built-in numeric types, our decimal numbers are
//! - immutable,
//! - always stack-allocated,
//! - and provided with copy semantics (instead of move semantics).
//!
//! ```rust
//! use beaumont::num::Decimal;
//!
//! let d1 = Decimal::new(123456, 2);
//! let d2 = d1; // d2 is a copy of d1 (also stack-allocated)
//! println!("d1 = {d1}"); // prints "d1 = 1234.56"
//! println!("d2 = {d2}"); // prints "d2 = 1234.56"
//! ```

use std::fmt;

/// Representation of a decimal number in fixed-point arithmetic.
#[derive(Debug, Clone, Copy)]
pub struct Decimal {
    coefficient: i32,
    scaling: u8,
    // TODO precision: u8
}

/// The maximum coefficient allowed
pub const MAX_COEFFICIENT: i32 = i32::MAX;

/// The minimum coefficient allowed
pub const MIN_COEFFICIENT: i32 = i32::MIN;

/// The maximum scaling allowed
pub const MAX_SCALING: u8 = 8;

/// The minimum scaling allowed
pub const MIN_SCALING: u8 = 0;

/// The maximum precision possible
//  Since MAX_COEFFICIENT is 2147483647 then 10 digits at maximum
pub const MAX_PRECISION: u8 = 10;



/// Error type for decimal numbers
#[derive(Debug)]
pub enum Error {
    /// Error due to bad format
    BadFormat(&'static str),
    /// Error due to the scaling factor exceeding [`MAX_SCALING`]
    ScalingOverflow,
    /// Error due to the coefficient exceeding [`MAX_COEFFICIENT`]
    CoefficientOverflow
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::BadFormat(msg) => write!(f, "{msg}"),
            Error::ScalingOverflow => write!(f, "Scaling overflow"),
            Error::CoefficientOverflow => write!(f, "Coefficient overflow")
        }
    }
}


/// Rounding mode to be used in numerical operations requiring rounding.
pub enum RoundingMode {
    /// Round towards "nearest neighbor" unless both neighbors are equidistant,
    /// in which case, it rounds up (away from zero).
    HalfUp,
    // TODO Add more rounding mode variants
}



impl Decimal {
    /// Same as the [`Decimal::try_new`] function, but it panics instead of resulting [`Error`]
    pub fn new(coefficient: i32, scaling: u8) -> Self {
        Self::try_new(coefficient, scaling)
            .unwrap_or_else(|err|
                panic!("{}", err)
            )
    }

    /// Attempt to create a new decimal number (without panicking).
    ///
    /// It returns ok by wrapping the new decimal number the new decimal number, or it returns one of
    /// the following errors:
    ///
    /// - [`Error::ScalingOverflow`]<br>
    ///   If the given scaling factor exceeds [`MAX_SCALING`]
    ///
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont::num::{Decimal, Error, MAX_SCALING};
    /// let d1 = Decimal::try_new(123456, 2);
    /// assert!(d1.is_ok());
    /// assert_eq!(d1.unwrap().to_string(), "1234.56");
    ///
    /// let d2 = Decimal::try_new(123456, MAX_SCALING + 1);
    /// assert!(d2.is_err());
    /// assert!(matches!(d2.unwrap_err(), Error::ScalingOverflow));
    /// ```
    pub fn try_new(coefficient: i32, scaling: u8) -> Result<Self, Error> {
        if scaling > MAX_SCALING {
            return Err(Error::ScalingOverflow);
        }
        Ok(Self { coefficient, scaling })
    }

    // TODO pub fn signum(&self)
    // TODO pub fn is_positive
    // TODO pub fn is_negative
}



// Functions to create values of the Decimal number type
mod from;

// Functions to display values of the Decimal number type
mod display;

// Functions to upscale (or downscale) values of the Decimal number type
mod scaling;

// Functions to perform numerical operations on values of the Decimal number type
mod ops;

