use super::super::Vector;
use core::ops::Neg;

impl<A> Vector<A> where A: Copy + PartialEq + Neg<Output = A> {

    #[cfg_attr(doc, beaumont_macros::allow_katex)]
    /// Negate this vector
    ///
    /// Given a vector $\vec{v}$ in the n-dimensional space, its negate is defined as:
    ///
    /// $$
    /// -\vec{v} \quad \equiv \quad
    /// -1 \cdot \vec{v}  \quad \equiv \quad
    /// \begin{pmatrix}
    /// -v_{1} \\\\
    /// -v_{2} \\\\
    /// \dots \\\\
    /// -v_{n}
    /// \end{pmatrix}
    /// $$
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont_macros::*;
    /// # use beaumont_linear_algebra::*;
    /// let v = vector![1, 2, 3];
    /// let negated = v.neg();
    /// assert_eq!(negated, vector![-1, -2, -3]);
    /// ```
    ///
    pub fn neg(&self) -> Vector<A> {
        Vector {
            components: self.components.iter().map(|x| x.neg()).collect()
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use beaumont_macros::*;
    use beaumont_numbers::*;

    #[test]
    fn neg_vector_of_integers() {
        let v = Vector { components: Box::new([1, 2, 3]) };
        let negated = v.neg();
        assert_eq!(negated.components.len(), 3);
        assert_eq!(negated.components[0], -1);
        assert_eq!(negated.components[1], -2);
        assert_eq!(negated.components[2], -3);
    }

    #[test]
    fn neg_vector_of_floats() {
        let v = Vector { components: Box::new([1.1, 2.22, 3.333]) };
        let negated = v.neg();
        assert_eq!(negated.components.len(), 3);
        assert_eq!(negated.components[0], -1.1);
        assert_eq!(negated.components[1], -2.22);
        assert_eq!(negated.components[2], -3.333);
    }

    #[test]
    #[allow_decimals]
    fn neg_vector_of_decimals() {
        let v = Vector { components: Box::new([1.1d, 2.22d, 3.333d]) };
        let negated = v.neg();
        assert_eq!(negated.components.len(), 3);
        assert_eq!(negated.components[0], -1.1d);
        assert_eq!(negated.components[1], -2.22d);
        assert_eq!(negated.components[2], -3.333d);
    }
}
