use super::super::Vector;
use std::ops::{Add, Mul};
use std::iter::Sum;

impl<T> Vector<T> where T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sum<T> {

    #[cfg_attr(doc, beaumont_macros::allow_katex)]
    /// Compute the dot product of this and the other vector.
    ///
    /// Given two vectors $\vec{v}$ and $\vec{w}$ in the n-dimensional space, the dot product
    /// is defined as the sum of the products of their correspondent scalar components, or
    /// alternatively by using their magnitudes and the cosine of the angle between them:
    ///
    /// $$
    /// \vec{v} \cdot \vec{w} \quad \equiv \quad
    /// \sum_{i=1}^n v_{i} w_{i} \quad \equiv \quad
    /// v_{1} w_{1} + v_{2} w_{2} + \dots + v_{n} w_{n} \quad \equiv \quad
    /// \\|\vec{v}\\| \\|\vec{w}\\| \cos \theta
    /// $$
    ///
    /// Its result is a scalar number that represents how much two vectors are similar to each other,
    /// with a value close to 1 meaning they are very similar.
    ///
    pub fn dot(&self, other: &Vector<T>) -> T {
        self.components
            .iter()
            .zip(&other.components)
            .map(|(v, w)| v.mul(*w))
            .sum()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product_arrays_of_integers() {
        let u = Vector { components: Box::new([1, 2, 3]) };
        let w = Vector { components: Box::new([4 ,5, 6]) };
        let r = u.dot(&w);
        assert_eq!(r, 32);
    }

    #[test]
    fn dot_product_vectors_of_floats() {
        let u = Vector { components: Box::new([1.0, 2.0, 3.0]) };
        let w = Vector { components: Box::new([4.0, 5.0, 6.0]) };
        let r = u.dot(&w);
        assert_eq!(r, 32.0);
    }

    // #[test]
    // #[allow_decimals]
    // TODO fn dot_product_vectors_of_decimals() {
    //     let u = Vector { components: Box::new([1.0d, 2.0d, 3.0d]) };
    //     let w = Vector { components: Box::new([4.0d, 5.0d, 6.0d]) };
    //     let r = u.dot_product(&w);
    //     assert_eq!(r, 32.0d);
    // }
}