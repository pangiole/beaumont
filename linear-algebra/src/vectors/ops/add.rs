use std::ops::Add;
use crate::Dimension;
use super::super::Vector;

impl<A> Vector<A> where A: Copy + PartialEq + Add<Output = A> {

    #[cfg_attr(doc, beaumont_macros::allow_katex)]
    /// Add this vector to the other vector.
    ///
    /// Given two vectors $\vec{v}$ and $\vec{w}$ in the n-dimensional space, their addition
    /// is defined by the sum of the correspondent components.
    ///
    /// $$
    /// \vec{v} + \vec{w}
    /// \quad \equiv \quad
    /// \begin{pmatrix}
    /// v_{1} \\\\
    /// v_{2} \\\\
    /// \dots \\\\
    /// v_{n}
    /// \end{pmatrix}
    /// +
    /// \begin{pmatrix}
    /// w_{1} \\\\
    /// w_{2} \\\\
    /// \dots \\\\
    /// w_{n}
    /// \end{pmatrix}
    /// \quad \equiv \quad
    /// \begin{pmatrix}
    /// v_{1} + w_{1} \\\\
    /// v_{2} + w_{2} \\\\
    /// \dots \\\\
    /// v_{n} + w_{n}
    /// \end{pmatrix}
    /// $$
    ///
    pub fn add(&self, rhs: &Self) -> Self {
        if self.dim() != rhs.dim() {
            panic!("must have the same dimension");
        }

        let components: Vec<A> =
            self.components.iter()
                .zip(&rhs.components)
                .map(|(x, y)| x.add(*y) )
                .collect();

        Vector {
            components: components.into_boxed_slice()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "must have the same dimension")]
    fn add_vectors_of_different_dimensions() {
        let v1 = Vector { components: Box::new([1, 2, 3]) };
        let v2 = Vector { components: Box::new([4]) };
        let _ = v1.add(&v2);
        // panic!
    }

    #[test]
    fn add_vectors_of_integers() {
        let v1 = Vector { components: Box::new([1, 2, 3]) };
        let v2 = Vector { components: Box::new([4, 5, 6]) };
        let v3 = v1.add(&v2);
        assert_eq!(v3.components.len(), 3);
        assert_eq!(v3.components[0], 5);
        assert_eq!(v3.components[1], 7);
        assert_eq!(v3.components[2], 9);
    }

    #[test]
    fn add_vectors_of_float() {
        let v1 = Vector { components: Box::new([1.1, 2.22, 3.333]) };
        let v2 = Vector { components: Box::new([4.4, 5.55, 6.666]) };
        let v3 = v1.add(&v2);
        assert_eq!(v3.components.len(), 3);
        assert_eq!(v3.components[0], 5.5);
        assert_eq!(v3.components[1], 7.77);
        assert_eq!(v3.components[2], 9.999);
    }


    // #[test]
    // #[allow_decimals]
    //TODO fn add_vectors_of_decimals() {
    //     use beaumont_macros::vector;
    //     let v1 = Vector { components: [1.1d, 2.22d, 3.333d] };
    //     let v2 = Vector { components: [4.4d, 5.55d, 6.666d] };
    //     let v3 = v1.add(&v2);
    //     assert_eq!(v3.components.len(), 3);
    //     assert_eq!(v3.components[0], 5.5d);
    //     assert_eq!(v3.components[1], 7.77d);
    //     assert_eq!(v3.components[2], 9.999d);
    // }
}