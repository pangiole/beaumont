//! A basic library for algorithmic trading.
//!
//! <div class="warning">This crate is in an early stage of development and definitively
//! <strong>not</strong> production ready.<br>Use with caution!</div>

// /// Macro to create vectors conveniently.
// ///
// /// Supported forms:
// /// - `v![elem1, elem2, ...]` creates a `Vector<Decimal>` populated with the given elements
// ///
// /// # Examples
// /// ```
// /// # use beaumont::v;
// /// let v = v![1, 2, 3];
// /// assert_eq!(v, [1, 2, 3]);
// /// ```
// #[macro_export]
// macro_rules! v {
//     // Element-list constructor producing Vec<Decimal>
//     ($($elem:expr),+ $(,)?) => {{
//         let mut temp_vec: ::std::vec::Vec<$crate::num::Decimal> = ::std::vec::Vec::new();
//         $( temp_vec.push($elem as $crate::num::Decimal); )+
//         temp_vec
//     }};
// }
// #[macro_export]
// macro_rules! v {
//     // Element-list constructor producing Vec<Decimal>
//     ($($elem:expr),+ $(,)?) => {{
//         let mut temp_vec: ::std::vec::Vec<i64> = ::std::vec::Vec::new();
//         $( temp_vec.push($elem as i64); )+
//         temp_vec
//     }};
// }


/// A basic implementation of decimal numbers in fixed-point arithmetic.
///
/// <div class="warning">This module is in an early stage of development and definitively
/// <strong>not</strong> production ready.<br>Use with caution!</div>
///
/// # Representation
/// Decimal numbers are internally represented by a signed coefficient integer and a positive
/// scaling factor, such that the actual number is given by multiplying the coefficient by
/// the power of the negative scaling factor in base 10, as follows:
///
/// $$
/// coefficient * 10 ^{-scaling}
/// $$
///
/// For example, the decimal number 1234.56 is internally represented as 123456 (the coefficient)
/// and 2 (the scaling factor).
///
/// # Usage
/// To create a decimal number, you can:
/// * invoke [`num::Decimal::new`] and pass in both the `coefficient` integer and the `scaling` factor,
/// * or invoke the [`str::parse`] method on a string to have it parsed as [`num::Decimal`] type
///
/// ```rust
/// use beaumont::num::Decimal;
///
/// let d1 = Decimal::new(123456, 2);
/// let d2 = "1234.56".parse::<Decimal>().unwrap();
/// ```
///
/// # Notes
/// To make them look alike built-in numeric types, our decimal numbers are
/// * immutable,
/// * always stack-allocated,
/// * and provided with copy semantics (instead of move semantics).
///
/// ```rust
/// use beaumont::num::Decimal;
///
/// let d1 = Decimal::new(123456, 2);
/// let d2 = d1; // d2 is a copy of d1 (also stack-allocated)
/// println!("d1 = {d1}"); // prints "d1 = 1234.56"
/// println!("d2 = {d2}"); // prints "d2 = 1234.56"
/// ```
pub mod num;

// /// This module provides a basic implementation of Linear Algebra operations.
// pub mod linalg;
