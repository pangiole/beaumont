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


// impl Decimal {
//     // TODO pub fn signum(&self)
//     // TODO pub fn is_positive
//     // TODO pub fn is_negative
// }


/// Error type for decimal numbers
#[derive(Debug)]
pub enum Error {
    /// Error due to bad format
    BadFormat(&'static str),
    /// Error due to scaling overflowing [`MAX_SCALING`]
    ScalingOverflow,
    /// Error due to the coefficient overflowing [`MAX_COEFFICIENT`]
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

// Functions to create values of the Decimal number type
mod from;

// Functions to display values of the Decimal number type
mod display;

// // Functions to upscale (or downscale) values of the Decimal number type
// mod scaling;
//
// // Functions to perform arithmetic operations on values of the Decimal number type
// mod ops;

