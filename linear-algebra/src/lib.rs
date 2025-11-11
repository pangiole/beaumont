#![deny(missing_docs)]

//! A basic implementation of linear algebra common operations.
//!
//! <div class="warning">This project is in an early stage of development, and
//! <strong>not</strong> production ready yet. Use with caution!</div>
//!
//!


/// A trait for types that have a dimension (such as vectors and matrices)
pub trait Dimension {

    /// The dimension of the type.
    fn dim(&self) -> u64;

    /// Just a synonym of [`Dimension::dim`]
    fn order(&self) -> u64 {
        self.dim()
    }
}

mod vectors;
mod matrices;

// Re-export the public contents of our private modules to the crate root
pub use vectors::*;
pub use matrices::*;