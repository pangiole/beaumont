#![deny(missing_docs)]

//! A basic implementation of decimal numbers in fixed-point arithmetic.
//!
//! <div class="warning">This crate is in an early stage of development and
//! <strong>not</strong> production ready yet.<br>Use with caution!</div>
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
//! For example, the decimal number `1234.56d` is internally represented as $123456 * 10^{-2}$.
//!
//! # Usage
//! To create a decimal number, you can:
//! - use custom literals, such as `1234.56d` or `"-234.00"d`,
//! - invoke the [`Decimal::from`] converter functions
//! - invoke the [`Decimal::new`] factory function
//!
//! ```rust
//! use beaumont_numbers::*;
//! use beaumont_macros::*;
//!
//! // Annotate your function with the beaumont macros
//! #[beaumont_literals]
//! fn do_something() {
//!     // Create decimal numbers from custom literals
//!     let d1 = 12d;                         // d suffix
//!     let d2 = -56712.3489d;
//!     let d3 = "19.092801"d;
//!
//!     // Or invoke converter functions
//!     let d4 = Decimal::from(12);           // integer
//!     let d5 = Decimal::from(-56712.3489);  // float
//!     let d6 = Decimal::from("19.092801");  // string
//!
//!     // Or invoke factory functions
//!     let d7 = Decimal::new(12, 0);         // coefficient, scaling
//!     let d8 = Decimal::new(567123489, 4);
//!     let d9 = Decimal::new(19092801, 6);
//! }
//! ```
//!
//! Once created, you can apply mathematical operations such as negating, adding, multiplying, etc.
//!
//! ```rust
//! # use beaumont_numbers::Decimal;
//! # let d2 = Decimal::new(-567123489, 4);
//! let negated = -d2;
//! assert_eq!(negated.to_string(), "56712.3489");
//! ```
//!
//! # Memory
//! To make them look alike built-in numeric types, our decimal numbers are
//! - immutable,
//! - always stack-allocated,
//! - and provided with copy semantics (instead of move semantics).
//!
//! ```rust
//! use beaumont_numbers::Decimal;
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
    /// # use beaumont_numbers::{Decimal, Error, MAX_SCALING};
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

// Re-export the beaumont macros
pub use beaumont_macros::*;