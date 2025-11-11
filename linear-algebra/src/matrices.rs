use crate::Dimension;

#[cfg_attr(doc, beaumont_macros::allow_katex)]
/// Representation of a mathematical matrix.
///
/// Given $m, n \in \mathbb{N}$, as the number of rows and columns, a matrix $\textbf{A}$ of dimension
/// $m \times n$ is defined as:
///
/// $$
/// \textbf{A} \quad \equiv \quad
/// \begin{pmatrix}
/// a_{11} & a_{12} & a_{1n} \\\\
/// a_{21} & a_{22} & a_{2n} \\\\
/// \dots  & \dots  & \dots  \\\\
/// a_{m1} & a_{m2} & a_{mn} \\\\
/// \end{pmatrix}
/// $$
///
/// # Usage
/// To create a matrix, you can:
/// - either use function-like macros, such as [`beaumont_macros::matrix!`],
/// - or invoke the [`Matrix::from`] factory functions.
///
/// ```rust
/// use beaumont_macros::*;
/// use beaumont_numbers::*;
/// use beaumont_linear_algebra::*;
///
/// // Use the macro (with some special syntax)
/// let m1 = matrix![ 1 , 2 , 3 |
///                 | 4 , 5 , 6 ];
///
/// // Pass the components and the number of rows and columns
/// let m4 = Matrix::from(([1, 2, 3, 4, 5, 6], 2, 3));
/// ```
///
#[derive(Debug)]
pub struct Matrix<T> where T: Copy + PartialEq {
    // The scalar components are held using a Rust "boxed slice". It does **not**
    // implement the Copy trait, and it's pretty much the same as the Rust Vec<T>
    // but without capacity and resizing methods.
    components: Box<[T]>,

    /// The number of rows
    pub rows: u32,
    /// The number of columns
    pub cols: u32,
}

impl<T> Dimension for Matrix<T> where T: Copy + PartialEq {
    /// Returns the number of these matrix components.
    fn dim(&self) -> u64 {
        // TODO Can the following product ever overflow?
        (self.rows as u64) * (self.cols as u64)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dim() {
        let m = Matrix {
            cols: 3,
            rows: 2,
            components: Box::new([1, 2, 3, 4, 5, 6]),
        };
        assert_eq!(m.cols, 3);
        assert_eq!(m.rows, 2);
        assert_eq!(m.dim(), 6);
    }
}


mod from;
mod ops;
// TODO mod iter;
