use super::{Decimal, DecimalError};
use std::str::FromStr;


impl From<i32> for Decimal {
    fn from(int: i32) -> Self {
        Self::new(int, 0)
    }
}

impl From<f32> for Decimal {
    fn from(_float: f32) -> Self {
        // TODO Implement the ability to create a decimal number from a float
        Self::new(0, 0)
    }
}

impl From<&str> for Decimal {
    fn from(value: &str) -> Self {
        value.parse::<Decimal>().unwrap()
    }
}



impl FromStr for Decimal {
    type Err = DecimalError;

    // TODO Implement the ability to create a decimal number from scientific notation (such as 1.234e5)

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len == 0 {
            return Err(DecimalError::BadFormat("Empty string"));
        }

        let chars = s.as_bytes();
        let mut accumulated_coefficient: i32 = 0;
        let mut accumulated_scaling: u8 = 0;
        let mut minus_encountered = false;
        let mut dot_encountered = false;
        let mut maybe_error: Option<DecimalError> = None;

        for (i, c) in chars.iter().enumerate().take(len) {

            // 43 is the ASCII code for the character '+'
            // and it is allowed only once (at the beginning of the given string)
            if *c == 43 && i > 0 {
                maybe_error = Some(DecimalError::BadFormat("Misplaced + (plus)")); break;
            }

            // 45 is the ASCII code for the character '-'
            else if *c == 45 {
                if i > 0 {
                    maybe_error = Some(DecimalError::BadFormat("Misplaced - (minus)")); break;
                }
                minus_encountered = true;
            }

            // 46 is the ASCII code for the character '.'
            else if *c == 46 {
                if dot_encountered {
                    // double dot encountered!
                    maybe_error = Some(DecimalError::BadFormat("Double . (dot)"));
                    break;
                }
                dot_encountered = true;
            }

            // 48 is the ASCII code for the character '0'
            else if (48..=59).contains(c) {
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

                if let Some(c) = checked_coefficient {
                    accumulated_coefficient = c;
                    if dot_encountered {
                        accumulated_scaling += 1;
                    }
                }
                else {
                    maybe_error = Some(DecimalError::CoefficientOverflow);
                    break;
                }
            }

            else {
                maybe_error = Some(DecimalError::BadFormat("Invalid character"));
                break;
            }
        }

        // Finally, check if we encountered an error and return the result accordingly
        if let Some(error) = maybe_error {
            Err(error)
        }
        else {
            Self::try_new(accumulated_coefficient, accumulated_scaling)
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::{MIN_COEFFICIENT, MAX_SCALING};

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
    fn from_i32() {
        let decimal = Decimal::from(123456);
        assert_eq!(decimal.coefficient, 123456);
        assert_eq!(decimal.scaling, 0);
    }

    #[test]
    #[ignore]
    fn from_f32() {
        let decimal = Decimal::from(123456.789);
        assert_eq!(decimal.coefficient, 123456789);
        assert_eq!(decimal.scaling, 6);
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
        assert!(matches!(res.err().unwrap(), DecimalError::BadFormat("Empty string")));
    }

    #[test]
    fn from_str_invalid_char_err() {
        let res = Decimal::from_str("1.?34");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), DecimalError::BadFormat("Invalid character")));
    }

    #[test]
    fn from_str_misplaced_plus_sign_err() {
        let res = Decimal::from_str("1.+34");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), DecimalError::BadFormat("Misplaced + (plus)")));
    }

    #[test]
    fn from_str_misplaced_minus_sign_err() {
        let res = Decimal::from_str("1.34-");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), DecimalError::BadFormat("Misplaced - (minus)")));
    }

    #[test]
    fn from_str_double_dot_err() {
        let res = Decimal::from_str("1.234.56");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), DecimalError::BadFormat("Double . (dot)")));
    }

    #[test]
    fn from_str_exceeds_max_scale_err() {
        let res = Decimal::from_str("1.234567890");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), DecimalError::ScalingOverflow));
    }

    #[test]
    fn from_str_coefficient_overflow_err() {
        // Consider that the maximum coefficient is 2147483647
        // and that we want to represent 2147483648
        let res = Decimal::from_str("2147483648");
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), DecimalError::CoefficientOverflow));
    }
}