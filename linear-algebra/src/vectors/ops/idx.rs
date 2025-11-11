
use core::ops::Index;
use crate::Vector;

impl<T> Index<u64> for Vector<T> where T: PartialEq + Copy {
    type Output = T;

    /// Read the component at the given index, with it starting from 1 (not 0).
    ///
    /// # Examples:
    /// ```rust
    /// # use beaumont_macros::*;
    /// # use beaumont_linear_algebra::*;
    /// let v = vector![1, 2, 3];
    /// assert_eq!(v[2], 2);
    /// ```
    fn index(&self, index: u64) -> &Self::Output {
        &(self.components[(index - 1) as usize])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn idx() {
        let v = Vector { components: Box::new([1, 2, 3, 4, 5, 6]) };
        assert_eq!(v[3], 3);
    }
}