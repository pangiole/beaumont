use crate::num::{Decimal, Error, RoundingMode};

impl Decimal {

    /// Check if this decimal number is aligned to another decimal number.
    ///
    /// Two decimal numbers are considered aligned if they have the same scaling factor. For example,
    /// the decimal number `123.45` is considered to be aligned to `10987.65` because the scaling factors
    /// are equal (they have the same number of digits after the decimal point).
    ///
    /// Alignment of decimal numbers plays an important role in fixed-point arithmetic, as it ensures
    /// that the result of an operation is always represented with the same number of digits after the
    /// decimal point.
    ///
    /// # Examples
    /// ```
    /// # use beaumont::num::{Decimal};
    /// let d1 = Decimal::new(100, 2); // Represents 1.00
    /// let d2 = Decimal::new(500, 2); // Represents 5.00
    /// let d3 = Decimal::new(100, 3); // Represents 0.100
    ///
    /// assert_eq!(d1.is_aligned_to(d2), true);  // both d1 and d2 have scaling factor 2
    /// assert_eq!(d1.is_aligned_to(d3), false); // d3 has scaling factor 3
    /// ```
    pub fn is_aligned_to(self, that: Decimal) -> bool {
        self.scaling == that.scaling
    }


    /// Same as th [`Decimal::try_upscale_by`] method, but it panics instead of returning [`Error`]
    pub fn upscale_by(self, amount: u8) -> Self {
        self.try_upscale_by(amount)
            .unwrap_or_else(|err| {
                panic!("{}", err)
            })
    }

    /// Attempt to increase the scaling factor of this decimal number by the given amount.
    ///
    /// This method increases the scaling factor by appending trailing zeros to the coefficient via
    /// a checked multiplication to detect eventual coefficient overflows. Therefore, it returns ok
    /// by wrapping the new decimal number aligned to a greater scaling factor, or it returns one of
    /// the following errors:
    ///
    /// - [`Error::CoefficientOverflow`]<br>
    ///   If the new coefficient exceeds [`MAX_COEFFICIENT`]
    ///
    /// - [`Error::ScalingOverflow`]<br>
    ///   If the new scaling factor exceeds [`MAX_SCALING`]
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont::num::{Decimal, Error, MAX_SCALING, RoundingMode};
    /// let d = Decimal::new(123591, 3); // Represents "123.591"
    ///
    /// // Happy upscaling (add more trailing zeroes)
    /// let a1 = d.try_upscale_by(2);
    /// assert!(a1.is_ok());
    /// assert_eq!(a1.unwrap().to_string(), "123.59100");
    ///
    /// // Upscaling error (too many trailing zeroes)
    /// let a2 = d.try_upscale_by(5);
    /// assert!(a2.is_err());
    /// assert_eq!(matches!(a2.unwrap_err(), Error::CoefficientOverflow), true);
    ///
    /// // Upscaling error (scaling exceeds maximum allowed)
    /// let a3 = d.try_upscale_by(6);
    /// assert!(a3.is_err());
    /// assert_eq!(matches!(a3.unwrap_err(), Error::CoefficientOverflow), true);
    /// ```
    ///
    /// [`MAX_COEFFICIENT`]: constant.MAX_COEFFICIENT.html
    /// [`MAX_SCALING`]: constant.MAX_SCALING.html
    pub fn try_upscale_by(self, amount: u8) -> Result<Self, Error> {
        // The upscaling scenario consist of simply adding as many trailing zeros as needed.
        // Since we need to execute "checked" operations (because we want to detect overflows),
        // and since those operations return an Option<i32> envelope, we'd be better off at
        // coding with fold (and an accumulator) rather than with classic loops.
        //
        let new_scaling = self.scaling + amount;
        let iter = 1 ..= (new_scaling - self.scaling);

        // For example, if this coefficient is 567, and this scaling is 2, and we want
        // to upscale to 6, this is how the fold would work:
        //
        //          iter |   acc         |  x      |  result
        //     ----------|---------------|---------|-------------
        //     init          Some(567)
        //             1     Some(567)      567      Some(5670)
        //             2     Some(5670)     5670     Some(56700)
        //             3     Some(56700)    56700    Some(567000)
        //             4     Some(567000)   567000   Some(5670000)
        //
        let init = Some(self.coefficient);
        iter
            .fold(init, |acc, _| { acc.and_then(|x| x.checked_mul(10)) } )
            .map(|new_coefficient|
                // No need to invoke try_new() because we already ruled out the ScalingOverflow possibility
                Decimal::new(new_coefficient, new_scaling)
            )
            .ok_or(Error::CoefficientOverflow)
    }


    /// Decrease the scaling factor of this decimal number by the given amount and by applying
    /// the given rounding mode.
    ///
    /// <div class="warning">This method may incur into a loss of precision.</div>
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont::num::{Decimal, Error, MAX_SCALING, RoundingMode};
    /// let d1 = Decimal::new(123591, 3); // Represents "123.591"
    /// let rm = RoundingMode::HalfUp;
    ///
    /// let d2 = d1.downscale_by(2, &rm);
    /// assert_eq!(d2.to_string(), "123.6");
    /// ```
    ///
    pub fn downscale_by(self, amount: u8, rounding_mode: &RoundingMode) -> Self {
        // The downscaling scenario may require rounding, which may cause loss of precision!
        // For example, if the given number is `Decimal {coefficient: 12345678, scaling: 6}`, whose
        // representation is "12.345678", and we want to scale it down to 2, then the resulting
        // number becomes `Decimal {coefficient: 1235, scaling: 2}`, whose representation is "12.35".
        //
        let mut c1 = self.coefficient.unsigned_abs();
        let mut c2 = 0;
        let mut p = 0u32;
        for t in 0..amount {
            p = 10i32.pow(t as u32) as u32;
            c2 += (c1 % 10) * p;
            c1 /= 10;
        }

        // If the given number `Decimal {coefficient: 12345678, scaling: 6}`, is at this point
        // we got: `c1=12`, `c2=345678`, and `p=1000`. The next step is to round the `c2` value
        // according to one of the supported rounding modes
        //
        //            | new_coefficient | new_scaling | to_string
        //  HALF_UP   | 1235            | 2           | "12.35"
        //

        let rounded_coefficient =
            match rounding_mode {
                RoundingMode::HalfUp => Self::round_half_up(c1, c2, p) as i32
            };

        // TODO let signum = self.signum();
        let signum = self.coefficient.signum();
        let new_coefficient = signum * rounded_coefficient;

        // No need to call Decimal::new(new_coefficient, new_scaling) because we already
        // have the new_scaling and new_coefficient values checked against their ranges.
        Decimal { coefficient: new_coefficient, scaling: self.scaling - amount }
    }


    fn round_half_up(c1: u32, c2: u32, p: u32) -> u32 {
        if c2 / p >= 5 {
            c1 + 1
        }
        else {
            c1
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::num::{MIN_COEFFICIENT, MAX_SCALING};

    const RM: RoundingMode = RoundingMode::HalfUp;

    #[test]
    fn align_1234_56_upscale_by_2() {
        // upscale is no more than adding trailing zeros
        let decimal = Decimal::new(123456, 2);
        let aligned = decimal.upscale_by(2);
        assert_eq!(aligned.coefficient, 12345600);
        assert_eq!(aligned.scaling, 4);
    }

    #[test]
    #[should_panic(expected = "Coefficient overflow")]
    fn align_1234_56_upscale_by_9_panic() {
        let decimal = Decimal::new(123456, 2);
        decimal.upscale_by(MAX_SCALING + 1);
    }


    #[test]
    fn align_12_341120_downscale_by_4() {
        // Downscaling may require rounding (causing loss of precision)
        let decimal = Decimal::new(12341120, 6);
        let aligned = decimal.downscale_by(4, &RM);
        assert_eq!(aligned.coefficient, 1234);
        assert_eq!(aligned.scaling, 2);

    }

    #[test]
    fn align_12_3456789_downscale_by_5() {
        // Downscaling may require rounding (causing loss of precision)
        let decimal = Decimal::new(123456789, 7);
        let aligned = decimal.downscale_by(5, &RM);
        assert_eq!(aligned.coefficient, 1235);
        assert_eq!(aligned.scaling, 2);

    }

    #[test]
    fn align_12_6_downscale_by_1() {
        // Downscaling may require rounding (causing loss of precision)
        let decimal = Decimal::new(126, 1);
        let aligned = decimal.downscale_by(1, &RM);
        assert_eq!(aligned.coefficient, 13);
        assert_eq!(aligned.scaling, 0);
    }

    #[test]
    fn align_12_1_downscale_by_1() {
        // Downscaling may require rounding (causing loss of precision)
        let decimal = Decimal::new(121, 1);
        let aligned = decimal.downscale_by(1, &RM);
        assert_eq!(aligned.coefficient, 12);
        assert_eq!(aligned.scaling, 0);
    }

    #[test]
    fn align_min_coefficient_downscale_by_1() {
        // Downscaling "-21474836.48" by 1 should result in "-21474836.5"
        let decimal = Decimal::new(MIN_COEFFICIENT, 2);
        let aligned = decimal.downscale_by(1, &RM);
        assert_eq!(aligned.coefficient, -214748365);
        assert_eq!(aligned.scaling, 1);
    }

}