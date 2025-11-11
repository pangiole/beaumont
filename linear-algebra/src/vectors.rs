use super::Dimension;

#[cfg_attr(doc, beaumont_macros::allow_katex)]
/// Representation of a mathematical vector
///
/// Given $n \in \mathbb{N}$, as the number of components, a vector $\vec{v}$ of dimension $n$ is defined as:
///
/// $$
/// \vec{v} \quad \equiv \quad
/// \begin{pmatrix}
/// v_{1} \\\\
/// v_{2} \\\\
/// \dots \\\\
/// v_{n}
/// \end{pmatrix}
/// $$
///
/// # Usage
/// To create a vector, you can:
/// - either use function-like macros, such as [`beaumont_macros::vector!`],
/// - or invoke the [`Vector::from`] converter function.
///
/// ```rust
/// use beaumont_macros::*;
/// use beaumont_numbers::*;
/// use beaumont_linear_algebra::*;
///
/// # #[allow_decimals]
/// # fn main() {
/// let v1 = vector![1, 2, 3];                // integers
/// let v2 = vector![1.01, 2.32, 3.675];      // floats
/// let v3 = vector![1.01d, 2.32d, 3.67500d]; // decimals
///
/// let v4 = Vector::from([1, 2, 3]);
/// let v5 = Vector::from([1.01, 2.32, 3.675]);
/// let v6 = Vector::from([1.01d, 2.32d, 3.67500d]);
/// # }
/// ```
///
/// Once created, you can apply operations such as comparison, indexing, negating, adding,
/// scaling, etc.
///
/// ```rust
/// # use beaumont_macros::*;
/// # use beaumont_numbers::*;
/// # use beaumont_linear_algebra::*;
/// # #[allow_decimals]
/// # fn main() {
/// # let v1 = vector![1, 2, 3];
/// // Comparing for equality
/// assert_eq!(v1, vector![1, 2, 3]);
///
/// // Indexing
/// assert_eq!(v1[1], 1);
///
/// // Negating
/// let negated = v1.neg();
/// assert_eq!(negated, vector![-1, -2, -3]);
///
/// // Scaling (a.k.a. "scalar multiplication")
/// let v2 = v1.scale_by(2);
/// assert_eq!(v2, vector![2, 4, 6]);
///
/// // Adding
/// let v3 = Vector::from([1, 2, 3]);
/// let v4 = v3.add(&v2);
/// assert_eq!(v4, vector![3, 6, 9]);
/// 
/// // Dot product
/// let d = v3.dot(&v4);
/// assert_eq!(d, 42);
/// # }
/// ```
///
/// # Memory
/// To make them perform well and look like built-in types, our mathematical vectors are:
///
/// - immutable (cannot be modified after creation),
/// - having components contiguously allocated (in a fixed-size array),
/// - always heap-allocated,
/// - and requiring move semantics (copy semantics could not be implemented)
///
// TODO Describe the mathematical operations
#[derive(Debug)]
pub struct Vector<T> where T: Copy + PartialEq {
    // The scalar components are held using a Rust "boxed slice". It does **not**
    // implement the Copy trait, and it's pretty much the same as the Rust Vec<T>
    // but without capacity and resizing methods.
    components: Box<[T]>,
}

impl<T> Dimension for Vector<T> where T: Copy + PartialEq {

    /// Returns the number of these vector components.
    ///
    /// It can also be thought as the dimension of the space (for example, $\Reals^3$ for
    /// tridimensional vectors) which this vector belongs to. Not to be confused with its
    /// magnitude (sometimes called its "length").
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont_linear_algebra::*;
    /// # use beaumont_macros::*;
    /// let v = vector![1, 2, 3];
    /// assert_eq!(v.dim(), 3);
    /// ```
    fn dim(&self) -> u64 {
        self.components.len()  as u64
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dim() {
        let v = Vector { components: Box::new([1, 2, 3]) };
        assert_eq!(v.dim(), 3);
    }
}


mod from;
mod ops;
mod iter;
