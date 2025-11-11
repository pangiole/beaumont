#![deny(missing_docs)]
//!
//! A collection of procedural macros for the beaumont crates.
//!
//! <div class="warning">This project is in an early stage of development, and
//! <strong>not</strong> production ready yet. Use with caution!</div>
//!

use proc_macro::TokenStream;
use syn::parse_macro_input;

// The inner implementation of the #[allow_katex] attribute macro
mod katex;

/// Allow Katex snippets in Rust documentation
///
/// # Examples:
/// ```ignore
/// use beaumont_macros::allow_katex;
///
/// #[cfg_attr(doc, beaumont_macros::allow_katex)]
/// /// This is a piece of Katex:
/// /// $$
/// /// \frac{1}{2}
/// /// $$
/// fn do_something() {}
/// ```
#[proc_macro_attribute]
pub fn allow_katex(_attr: TokenStream, item: TokenStream) -> TokenStream {
    katex::transform(syn::parse_macro_input!(item)).into()
}

// The inner implementation of the #[allow_decimals] attribute macro
mod decimals;

/// Allow custom literals of decimal numbers
///
/// Useful for creating decimal numbers the way you would normally create integer or floating
/// numbers, but appending the custom `d` suffix to the literal (instead of the `i32` or `f32`
/// usual suffixes). Also, useful for creating decimal numbers from string literals, still by appending the
/// custom `d` suffix instead of explicitly invoking a parser.
///
/// # Examples
/// ```ignore
/// use beaumont_numbers::Decimal;
/// use beaumont_macros::decimals;
///
/// #[allow_decimals]
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
/// # Unhygienic
/// This macro is unhygienic, as it depends on the presence of the `Decimal` local name, referring
/// to any type for which an implementation of the `std::str::FromStr` trait exists.
/// While you can bring any `Decimal` you like, you're recommended to bring it as follows:
///
/// 1. either by using `beaumont_number::Decimal`,
/// 2. or by using `beaumont::Decimal` (which is a re-export of the same).
///
#[proc_macro_attribute]
pub fn allow_decimals(_attr: TokenStream, item: TokenStream) -> TokenStream {
    decimals::transform(parse_macro_input!(item)).into()
}


// Inner implementation of the vector! function-like macro
mod vector;

/// Create a mathematical vector by specifying its components one by one
///
/// # Unhygienic
/// This macro is unhygienic, as it depends on the presence of the `Vector` local name.
/// While you can bring any `Vector` you wish, you're recommended to bring the beaumont
/// types as follows:
///
/// 1. either by using `beaumont_liner_algebra::Vector`,
/// 2. or by using `beaumont::Vector` (which is a re-export of the same).
///
/// Moreover, this macro can also transform decimal literals (those ending with the `d` prefix)
/// without the need to annotate your functions with the `#[allow_decimals]` attribute macro.
///
/// # Examples
/// ```ignore
/// use beaumont_macros::vector;
/// use beaumont_numbers::Decimal;
/// use beaumont_linear_algebra::Vector;
///
/// let v: Vector<Decimal> =
///   vector![ 1.05d, 2.23d, 3.789d ];
///
/// assert_eq!(v.dim(), 3);
/// ```
#[proc_macro]
pub fn vector(items: TokenStream) -> TokenStream {
    vector::expand(parse_macro_input!(items))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}


// Inner implementation of the matrix! function-like macro
mod matrix;


/// Create a mathematical matrix by specifying its components one by one
///
/// # Examples
/// ```ignore
/// use beaumont_macros::matrix;
/// use beaumont_linear_algebra::Matrix;
///
/// let m: Matrix<i32> =
///   matrix![ 1 ,  2 ,  3 ,  4 |
///          | 5 ,  6 ,  7 ,  8 |
///          | 9 , 10 , 11 , 12 ];
/// 
/// assert_eq!(m.rows, 3);
/// assert_eq!(m.cols, 4);
/// assert_eq!(m.dim(), 12);
/// ```
#[proc_macro]
pub fn matrix(items: TokenStream) -> TokenStream {
    matrix::expand(parse_macro_input!(items))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}