use super::Decimal;
use std::fmt;

impl fmt::Display for Decimal {

    // Considering that, according to the Rust i32 type, the smallest value of self.coefficient
    // is -2147483648, while the largest value is 2147483647, and therefore considering that
    // those decimal numbers can be represented with:
    //
    //     1 eventual '-' character +
    //     1 eventual '0' character +
    //     1 eventual '.' character +
    //    10 digits at most         =
    //   ----
    //    13 is the length of the buffer
    //
    // Therefore, we can use a buffer of at most 13 characters to temporarily store the
    // representation of the decimal number as it's being built by this function.
    //
    // NOTE: the buffer is of type [u8], not [char], because it will be storing just
    // digits and the eventual '.' character, which are ASCII characters (no need to
    // use UTF-8 encoding which would require 4 bytes per character).
    //
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
         * The elements of the following array are the digit characters representing the ones for the first
         * 100 integers (from 0 to 99). Therefore, to answer the _"How many ones in 45?"_ question, just
         * read this array at index 45 and you'll get the character '5'. Similarly, the element at index 96
         * will be the character '6'.
         *
         *    [
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
         *   ]
         *
         * Rather than using such an array, we provide the ones_digit() function that does the same, but
         * representing the output as a single byte ASCII-encoded.
         */
        #[inline(always)]
        fn ones_digit(i: usize) -> u8 {
            // 48 is the ASCII code for the '0' character
            (48 + (i % 10)) as u8
        }

        /*
         * Similarly to the above array, the elements of the following array are the digit characters
         * representing the tens for the first 100 integers (from 0 to 99). Therefore, to answer the
         * _"How many tens in 45?"_ question, just read this array at index 45 and read the character '4'.
         *  Similarly, the element at index 96 will be the character '9'.
         *
         *
         *       [
         *           '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
         *           '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
         *           '2', '2', '2', '2', '2', '2', '2', '2', '2', '2',
         *           '3', '3', '3', '3', '3', '3', '3', '3', '3', '3',
         *           '4', '4', '4', '4', '4', '4', '4', '4', '4', '4',
         *           '5', '5', '5', '5', '5', '5', '5', '5', '5', '5',
         *           '6', '6', '6', '6', '6', '6', '6', '6', '6', '6',
         *           '7', '7', '7', '7', '7', '7', '7', '7', '7', '7',
         *           '8', '8', '8', '8', '8', '8', '8', '8', '8', '8',
         *           '9', '9', '9', '9', '9', '9', '9', '9', '9', '9',
         *       ];
         *
         * Rather than using such an array, we provide the tens_digit() function that does the same, but
         * representing the output as a single byte ASCII-encoded.
         */
        #[inline(always)]
        fn tens_digit(i: usize) -> u8 {
            // 48 is the ASCII code for the '0' character
            (48 + (i / 10)) as u8
        }


        const LEN: usize = 13;
        let mut buffer= [b'0'; LEN];

        // Helper function that shifts the content of the buffer to the left, the given number
        // of times, up to the given position (included), and fills the remaining space with the
        // given fill ASCII character.
        fn shift_left(buf: &mut [u8], times: u8, pos: usize, fill: u8) -> () {
            // TODO check if this is the most efficient way to shift the buffer left
            //      as we may not need to copy the whole buffer
            //      and we may not need to loop the given number of times
            for _ in 0..times {
                let mut i = 0;
                while i < pos {
                    buf[i] = buf[i + 1];
                    i = i + 1;
                }
                buf[pos] = fill;
            }
        }

        #[inline(always)]
        fn insert_digits(buf: &mut [u8], coefficient: i32) -> u8 {
            // The position of the next digit to be written in the buffer.
            // It starts from the rightmost position, and it gets decremented as digits are inserted.
            let mut pos = LEN;

            // Consider the absolute value of the coefficient, as the sign character will be handled separately.
            let mut c = coefficient.unsigned_abs();
            let mut q: u32;
            let mut i: usize;
            while c >= 100 {
                q = c / 100;
                i = (c - q * 100) as usize; // it's the same as (c % 100);
                // Decrement the position and insert the digits of how may ones and tens
                pos = pos - 1; buf[pos] = ones_digit(i);
                pos = pos - 1; buf[pos] = tens_digit(i);
                // It updates the coefficient being represented and loops over ...
                c = q;
            }
            // ... until the remaining coefficient is less than 100.
            i = c as usize;
            // Decrement the position and insert the digits of how may ones (and maybe tens)
            pos = pos - 1; buf[pos] = ones_digit(i);
            if i >= 10 {
                pos = pos - 1; buf[pos] = tens_digit(i);
            }

            // Finally, return the digit count
            (LEN - pos) as u8
        }

        // Depending on the given scaling, it eventually inserts the dot separator at the right position,
        // and it might shift some characters toward the lft of the buffer, and then it returns the
        // index to the first character of ultimate decimal number representation.
        #[inline(always)]
        fn apply_scaling(buf: &mut [u8], scaling: u8, digits_count: u8) -> usize {
            if scaling == 0 {
                LEN - (digits_count as usize)
            }
            else {
                let last_index = LEN - 1;
                let pos = last_index - (scaling as usize);
                if scaling < digits_count {
                    shift_left(buf,  1, pos, b'0');
                    buf[pos] = b'.';
                    last_index - (digits_count as usize)
                }
                else if scaling == digits_count {
                    buf[pos - 1] = b'0';
                    buf[pos] = b'.';
                    pos - 1
                }
                else {
                    // scaling > digits_count
                    buf[pos] = b'.';
                    pos - 1
                }
            }
        }

        #[inline(always)]
        fn apply_sign(buf: &mut[u8], coefficient: i32, i: usize) -> usize {
            if coefficient < 0 {
                buf[i-1] = b'-';
                i - 1
            }
            else {
                i
            }
        }

        // These are the 3 steps to easily build the decimal number textual representation:
        let digits_count = insert_digits(&mut buffer, self.coefficient);
        let first_index = apply_scaling(&mut buffer, self.scaling, digits_count);
        let first_index = apply_sign(&mut buffer, self.coefficient, first_index);

        // There's no concern with the following unsafe conversion because our buffer
        // certainly contains ASCII characters only (see above)
        let representation: &str = unsafe {
            std::str::from_utf8_unchecked(&buffer[first_index..LEN])
        };
        f.write_str(representation)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // ----------
    // UNSCALED scenarios
    // When the scaling is zero

    #[test]
    fn display_min_positive_number_unscaled() {
        let n = Decimal::new(-i32::MAX, 0);
        assert_eq!(n.to_string(), (-i32::MAX).to_string());
    }

    #[test]
    fn display_negative_number_unscaled() {
        let n = Decimal::new(-123456, 0);
        assert_eq!(n.to_string(), "-123456");
    }

    #[test]
    fn display_zero_unscaled() {
        let n = Decimal::new(0, 0);
        assert_eq!(n.to_string(), "0");
    }

    #[test]
    fn display_positive_number_unscaled() {
        let n = Decimal::new(123456, 0);
        assert_eq!(n.to_string(), "123456");
    }

    #[test]
    fn display_max_positive_number_unscaled() {
        let n = Decimal::new(i32::MAX, 0);
        assert_eq!(n.to_string(), i32::MAX.to_string());
    }

    // ----------
    // UNDERSCALED scenarios
    // When the scaling less than the coefficient digit count

    #[test]
    fn display_negative_underscaled() {
        let n = Decimal::new(-123456, 2);
        assert_eq!(n.to_string(), "-1234.56");
    }

    #[test]
    fn display_positive_underscaled() {
        let n = Decimal::new(123456, 2);
        assert_eq!(n.to_string(), "1234.56");
    }


    // ----------
    // FULLY SCALED scenarios
    // When the scaling is equal to the coefficient digit count

    #[test]
    fn display_negative_number_fully_scaled() {
        // the scaling is equal to the digit count of the coefficient
        let n = Decimal::new(-123456, 6);
        assert_eq!(n.to_string(), "-0.123456");
    }

    #[test]
    fn display_zero_fully_scaled() {
        // the scaling is equal to the digit count of the coefficient
        let n = Decimal::new(0, 1);
        assert_eq!(n.to_string(), "0.0");
    }

    #[test]
    fn display_positive_number_fully_scaled() {
        // the scaling is equal to the digit count of the coefficient
        let n = Decimal::new(123456, 6);
        assert_eq!(n.to_string(), "0.123456");
    }

    // ----------
    // OVER SCALING scenarios
    // When the scaling greater than coefficient digit count
    #[test]
    fn display_negative_over_scaled() {
        let n = Decimal::new(-123456, 8);
        assert_eq!(n.to_string(), "-0.00123456");
    }

    #[test]
    fn display_positive_over_scaled() {
        let n = Decimal::new(12, 8);
        assert_eq!(n.to_string(), "0.00000012");
    }
}