use super::super::Vector;
use std::ops::Mul;

impl<T> Vector<T> where T: Copy + PartialEq + Mul<Output = T> {

    #[cfg_attr(doc, beaumont_macros::allow_katex)]
    /// Multiply this vector by a scalar.
    ///
    /// Given a vector $\vec{v}$ in the n-dimensional space and a scalar $a$,
    /// the scalar multiplication is defined as:
    ///
    /// $$
    /// a \cdot \vec{v} \quad \equiv \quad
    /// a \cdot
    /// \begin{pmatrix}
    /// v_{1} \\\\
    /// v_{2} \\\\
    /// \dots \\\\
    /// v_{n}
    /// \end{pmatrix}
    /// \quad \equiv \quad
    /// \begin{pmatrix}
    /// a v_{1} \\\\
    /// a v_{2} \\\\
    /// \dots \\\\
    /// a v_{n}
    /// \end{pmatrix}
    /// $$
    ///
    /// # Examples:
    /// ```rust
    /// # use beaumont_macros::* ;
    /// # use beaumont_linear_algebra::*;
    /// let v = vector![1, 2, 3];
    /// let scaled = v.scale_by(2);
    /// assert_eq!(scaled, vector![2, 4, 6]);
    /// ```
    ///
    pub fn scale_by(&self, scalar: T) -> Self {
        Vector {
            components: self.components.iter().map(|x| x.mul(scalar)).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scale_vectors_of_integers() {
        let v = Vector { components: Box::new([1, 2, 3]) };
        let scaled = v.scale_by(2);
        assert_eq!(scaled.components.len(), 3);
        assert_eq!(scaled.components[0], 2);
        assert_eq!(scaled.components[1], 4);
        assert_eq!(scaled.components[2], 6);
    }

    #[test]
    fn scale_vectors_of_floats() {
        let v = Vector { components: Box::new([1.1, 2.22, 3.333]) };
        let scaled = v.scale_by(2.0);
        assert_eq!(scaled.components.len(), 3);
        assert_eq!(scaled.components[0], 2.2);
        assert_eq!(scaled.components[1], 4.44);
        assert_eq!(scaled.components[2], 6.666);
    }

    // #[test]
    // #[allow_decimals]
    // TODO fn scale_vectors_of_decimals() {
    //     use beaumont_macros::vector;
    //     let v = Vector { components: Box::new([1.1d, 2.22d, 3.333d]) };
    //     let scaled = v.scale_by(2.0d);
    //     assert_eq!(scaled.components.len(), 3);
    //     assert_eq!(scaled.components[0], 2.22d);
    //     assert_eq!(scaled.components[1], 4.444d);
    //     assert_eq!(scaled.components[2], 6.6666d);
    // }
}