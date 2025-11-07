use std::ops::Neg;
use crate::{Decimal, RoundingMode};


impl Decimal {
    /// Negate this decimal number while checking for eventual overflows.
    ///
    /// This method relies on the underlying implementation of [`i32::checked_neg`]
    /// to detect overflow conditions for the `coefficient` field.
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont_numbers::{Decimal, MIN_COEFFICIENT};
    /// let d1 = Decimal::new(123, 2);
    /// let n1 = d1.checked_neg();
    /// assert!(n1.is_some());
    /// assert_eq!(n1.unwrap().to_string(), "-1.23");
    ///
    /// // Overflow!
    /// let d2 = Decimal::new(MIN_COEFFICIENT, 2);
    /// let n2 = d2.checked_neg();
    /// assert!(n2.is_none());
    /// ```
    pub fn checked_neg(self) -> Option<Self> {
        self.coefficient
            .checked_neg()
            .map(|new_coefficient|
                Decimal {
                    coefficient: new_coefficient,
                    scaling: self.scaling
                }
            )

    }

    /// Negate this decimal number while downscaling it if an overflow occurs during the operation.
    ///
    /// If an overflow occurs, this method automatically invokes [`Self::downscale_by`] to decrease
    /// the scaling factor by `1`. Although making for a loss of precision, the negation is
    /// retried on the downscaled number so to ensure it completes without causing an overflow.
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont_numbers::{Decimal, RoundingMode};
    /// let d = "-21474836.48".parse::<Decimal>().unwrap();
    /// assert!(d.checked_neg().is_none()); // Overflowed
    ///
    /// let rm = RoundingMode::HalfUp;
    /// let n = d.rounding_neg(&rm);
    /// assert_eq!(n.to_string(), "21474836.5"); // Rounded
    /// ```
    pub fn rounding_neg(self, rounding_mode: &RoundingMode) -> Self {
        self
            .checked_neg()
            .unwrap_or_else(|| {
                // OVERFLOW!
                // We're handling overflows by automatically downscaling this decimal number by 1,
                // although we know that incurs into a loss of precision.
                self
                    .downscale_by(1, rounding_mode)
                    .neg()
            })
    }
}


impl Neg for Decimal {
    type Output = Self;

    /// Negate this decimal number, and it panics in case of coefficient overflow.
    fn neg(self) -> Self::Output {
        self
            .checked_neg()
            .unwrap_or_else(|| panic!("Coefficient overflow while negating {}", self))
    }
}



#[cfg(test)]
mod test {
    use crate::MIN_COEFFICIENT;
    use super::*;

    #[test]
    fn negate_from_positive() {
        let d1 = Decimal::new(123456, 2);
        let d2 = -d1;
        assert_eq!(d2.coefficient, -123456);
        assert_eq!(d2.scaling, 2);
    }

    // NOTE: negating from a positive decimal number would never overflow

    #[test]
    fn negate_from_negative() {
        let d1 = Decimal::new(-123456, 2);
        let d2 = -d1;
        assert_eq!(d2.coefficient, 123456);
        assert_eq!(d2.scaling, 2);
    }


    #[test]
    #[should_panic(expected = "Coefficient overflow while negating -21474836.48")]
    fn negate_from_negative_overflow() {
        let d1 = Decimal::new(i32::MIN, 2);
        let _ = -d1;
    }

    #[test]
    fn rounding_negate_min_coefficient() {
        // -21474836.48  ->  21474836.48
        // -21474836.5   ->  21474836.5
        let d1 = Decimal::new(MIN_COEFFICIENT, 2);
        let rm = RoundingMode::HalfUp;
        let negated = d1.rounding_neg(&rm);
        assert_eq!(negated.coefficient, 214748365);
        assert_eq!(negated.scaling, 1);
    }
}