use super::super::Vector;

impl<T> PartialEq for Vector<T> where T: Copy + PartialEq {

    #[cfg_attr(doc, beaumont_macros::allow_katex)]
    /// Tests for `self` and `other` values to be equal, and is used by `==`.
    ///
    /// Note that vectors, such as $\vec{v}$ and $\vec{w}$ in the n-dimensional space,
    /// are considered to be equal if and only if all of their correspondent scalar
    /// components are equal:
    ///
    /// $$
    /// \begin{aligned}
    /// &\vec{v} = \vec{w} \iff v_i = w_i \quad \forall i = 1, 2, \dots, n
    /// \end{aligned}
    /// $$
    fn eq(&self, other: &Self) -> bool {
        self.components.iter()
            .zip(&other.components)
            .all(|(x, y)| x.eq(y) )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq_vectors_of_integers() {
        let v = Vector { components: Box::new([1, 2, 3]) };
        assert_eq!(v, v);
    }
    
    #[test]
    fn eq_vectors_of_floats() {
        let v = Vector { components: Box::new([1.0, 2.0, 3.0]) };
        assert_eq!(v, v);
    }
}