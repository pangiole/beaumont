#![deny(missing_docs)]
//!
//! A collection of procedural macros for the beaumont crates.
//!
//! <div class="warning">This crate is in an early stage of development and
//! <strong>not</strong> production ready yet.<br>Use with caution!</div>
//!

use proc_macro::TokenStream;
use syn::parse_macro_input;

// The inner implementation of the literal macro
mod literals;

/// Attribute macro enabling custom literals for beaumont decimal numbers.
///
/// Useful for creating decimal numbers the way you would normally create integer or floating
/// numbers, but appending the custom `d` suffix to the literal (instead of the `i32` or `f32`
/// usual suffixes).
///
/// Also, useful for creating decimal numbers from string literals, still by appending the
/// custom `d` suffix instead of explicitly invoking a parser.
///
/// # Examples
/// ```no_test
/// use beaumont_macros::*;
///
/// #[beaumont_literals]
/// fn do_something() {
///     // Create a decimal number from integer literals
///     let d1 = 12d;
///
///     // Create a decimal number from float literals
///     let d2 = 12.00d;
///     let d3 = -56712.3489d;
///
///     // Create a decimal number from string literals
///     let d4 = "190928.05"d;
/// }
/// ```
///
#[proc_macro_attribute]
pub fn beaumont_literals(_attr: TokenStream, item: TokenStream) -> TokenStream {
    literals::transform(parse_macro_input!(item)).into()
}
