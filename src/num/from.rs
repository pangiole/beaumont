use super::{Decimal, Error, MAX_SCALING};
use std::str::FromStr;

impl Decimal {
    /// Same as the [`Decimal::try_new`] function, but it panics instead of resulting [`Error`]
    pub fn new(coefficient: i32, scaling: u8) -> Self {
        Self::
            try_new(coefficient, scaling)
            .unwrap_or_else(|err| panic!("{}", err))
    }

    /// Attempt to create a new decimal number (without panicking).
    ///
    /// # Parameters
    /// * `coefficient` - A signed coefficient integer
    /// * `scaling` - A positive scaling factor
    ///
    /// # Returns
    /// An [`Ok`] result wrapping a new decimal number or one of the following:
    /// * [`Error::ScalingOverflow`]<br>
    ///   If the given scaling factor exceeds [`MAX_SCALING`]
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont::num::{Decimal, Error, MAX_SCALING};
    /// let d1 = Decimal::try_new(123456, 2);
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
}


impl FromStr for Decimal {
    type Err = Error;

    // TODO Implement the ability to create a decimal number from scientific notation (such as 1.234e5)

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len == 0 {
            return Err(Error::BadFormat("Empty string"));
        }

        let chars = s.as_bytes();
        let mut accumulated_coefficient: i32 = 0;
        let mut accumulated_scaling: u8 = 0;
        let mut minus_encountered = false;
        let mut dot_encountered = false;
        let mut maybe_error: Option<Error> = None;

        for i in 0..len {
            let c = chars[i];

            // 43 is the ASCII code for the character '+'
            // and it is allowed only once (at the beginning of the given string)
            if c == 43 && i > 0 {
                maybe_error = Some(Error::BadFormat("Misplaced + (plus)")); break;
            }

            // 45 is the ASCII code for the character '-'
            else if c == 45 {
                if i > 0 {
                    maybe_error = Some(Error::BadFormat("Misplaced - (minus)")); break;
                }
                minus_encountered = true;
            }

            // 46 is the ASCII code for the character '.'
            else if c == 46 {
                if dot_encountered {
                    // double dot encountered!
                    maybe_error = Some(Error::BadFormat("Double . (dot)"));
                    break;
                }
                dot_encountered = true;
            }

            // 48 is the ASCII code for the character '0'
            else if c >= 48 && c <= 59 {
                //
                // Update the coefficient by multiplying it by 10 and then adding (or subtracting)
                // the current character digit
                //
                //      coefficient = coefficient * 10 + (c - 48)
                //
                // If the result overflows, break the loop and set an error
                //
                let checked_coefficient =
                    accumulated_coefficient
                        .checked_mul(10)
                        .and_then(|x|
                            if !minus_encountered { x.checked_add((c - 48) as i32) }
                            else                  { x.checked_sub((c - 48) as i32) }
                        );

                if checked_coefficient.is_some() {
                    accumulated_coefficient = checked_coefficient.unwrap();
                    if dot_encountered {
                        accumulated_scaling = accumulated_scaling + 1;
                    }
                }
                else {
                    maybe_error = Some(Error::CoefficientOverflow);
                    break;
                }
            }

            else {
                maybe_error = Some(Error::BadFormat("Invalid character"));
                break;
            }
        }

        // Finally, check if we encountered an error and return the result accordingly
        maybe_error
            .map_or_else(
                || Self::try_new(accumulated_coefficient, accumulated_scaling),
                |error| Err(error)
            )
    }
}



#[cfg(test)]
mod test {
    use crate::num::MIN_COEFFICIENT;
    use super::*;

    #[test]
    fn new() {
        let decimal = Decimal::new(123456, 2);
        assert_eq!(decimal.coefficient, 123456);
        assert_eq!(decimal.scaling, 2);
    }

    #[test]
    #[should_panic]
    fn new_panic() {
        Decimal::new(123456, MAX_SCALING + 1);
    }

    #[test]
    fn from_str_min_coefficient() {
        let res = Decimal::from_str("-21474836.48");
        assert!(res.is_ok());
        let decimal = res.unwrap();
        assert_eq!(decimal.coefficient, MIN_COEFFICIENT);
        assert_eq!(decimal.scaling, 2);
    }

    #[test]
    fn from_str_minus_123456() {
        let res = Decimal::from_str("-123456");
        assert!(res.is_ok());
        let decimal = res.unwrap();
        assert_eq!(decimal.coefficient, -123456);
        assert_eq!(decimal.scaling, 0);
    }

    #[test]
    fn from_str_123456() {
        let res = Decimal::from_str("123456");
        assert!(res.is_ok());
        let decimal = res.unwrap();
        assert_eq!(decimal.coefficient, 123456);
        assert_eq!(decimal.scaling, 0);
    }

    #[test]
    fn from_str_1234_56() {
        let res = Decimal::from_str("1234.56");
        assert!(res.is_ok());
        let decimal = res.unwrap();
        assert_eq!(decimal.coefficient, 123456);
        assert_eq!(decimal.scaling, 2);
    }

    #[test]
    fn from_str_dot_123456() {
        let res = Decimal::from_str(".123456");
        assert!(res.is_ok());
        let decimal = res.unwrap();
        assert_eq!(decimal.coefficient, 123456);
        assert_eq!(decimal.scaling, 6);
    }

    #[test]
    fn from_str_0_123456() {
        let res = Decimal::from_str("0.123456");
        assert!(res.is_ok());
        let decimal = res.unwrap();
        assert_eq!(decimal.coefficient, 123456);
        assert_eq!(decimal.scaling, 6);
    }

    #[test]
    fn from_str_0_000123456() {
        let res = Decimal::from_str("0.00045678");
        assert!(res.is_ok());
        let decimal = res.unwrap();
        assert_eq!(decimal.coefficient, 45678);
        assert_eq!(decimal.scaling, 8);
    }


    // error scenarios

    #[test]
    fn from_str_empty_err() {
        let res = Decimal::from_str("");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::BadFormat("Empty string")));
    }

    #[test]
    fn from_str_invalid_char_err() {
        let res = Decimal::from_str("1.?34");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::BadFormat("Invalid character")));
    }

    #[test]
    fn from_str_misplaced_plus_sign_err() {
        let res = Decimal::from_str("1.+34");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::BadFormat("Misplaced + (plus)")));
    }

    #[test]
    fn from_str_misplaced_minus_sign_err() {
        let res = Decimal::from_str("1.34-");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::BadFormat("Misplaced - (minus)")));
    }

    #[test]
    fn from_str_double_dot_err() {
        let res = Decimal::from_str("1.234.56");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::BadFormat("Double . (dot)")));
    }

    #[test]
    fn from_str_exceeds_max_scale_err() {
        let res = Decimal::from_str("1.234567890");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::ScalingOverflow));
    }

    #[test]
    fn from_str_coefficient_overflow_err() {
        // Consider that the maximum coefficient is 2147483647
        // and that we want to represent 2147483648
        let res = Decimal::from_str("2147483648");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::CoefficientOverflow));
    }
}