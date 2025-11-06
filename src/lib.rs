#![deny(missing_docs)]

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


pub mod num;

// /// This module provides a basic implementation of Linear Algebra operations.
// pub mod linalg;
