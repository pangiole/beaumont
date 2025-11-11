use std::ops::Index;
use crate::Matrix;

impl<T> Index<(u32, u32)> for Matrix<T> where T: Copy + PartialEq {
    type Output = T;

    /// Read the component at the given `(row, col)` index, with them starting from 1 (not 0).
    ///
    /// # Examples
    /// ```rust
    /// # use beaumont_linear_algebra::*;
    /// # use beaumont_macros::*;
    /// let m = matrix![ 1, 2, 3 |
    ///                | 4, 5, 6 |
    ///                | 7, 8, 9 ];
    ///
    /// assert_eq!(m[(1, 2)], 2);
    /// assert_eq!(m[(3, 3)], 9);
    /// ```
    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let (row, col) = index;
        let idx: usize = ((row - 1) * self.cols + (col - 1)) as usize;
        &(self.components[idx])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn idx() {
        let m = Matrix { components: Box::new([1, 2, 3, 4, 5, 6]), rows: 2, cols: 3 };
        assert_eq!(m[(1, 3)], 3);
        assert_eq!(m[(2, 1)], 4);
    }
}