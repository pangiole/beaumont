use super::Matrix;


/// Create a new mathematical vector from a boxed slice, with the given number of rows and columns.
impl<T> From<(Box<[T]>, u32, u32)> for Matrix<T> where T: PartialEq + Copy {
    fn from(value: (Box<[T]>, u32, u32)) -> Self {
        let (components, rows, cols) = value;
        assert_eq!((rows * cols) as usize, components.len());
        Self {
            rows,
            cols,
            components
        }
    }
}

impl<T, const LEN: usize> From<([T; LEN], u32, u32)> for Matrix<T>  where T: PartialEq + Copy {
    fn from(value: ([T; LEN], u32, u32)) -> Self {
        let (components, rows, cols) = value;
        assert_eq!((rows * cols) as usize, components.len());
        Self {
            components: components.into(),
            rows,
            cols,
        }
    }
}

// TODO impl Default for Matrix


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_array_of_integers() {
        // from a given boxed slice
        let components: Box<[i32]> = Box::new([1, 2, 3, 4, 5, 6]);
        let m = Matrix::from((components, 2, 3));
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        assert_eq!(m.components[0], 1);
        assert_eq!(m.components[1], 2);
        assert_eq!(m.components[2], 3);
        assert_eq!(m.components[3], 4);
        assert_eq!(m.components[4], 5);
        assert_eq!(m.components[5], 6);
    }

    #[test]
    fn from_array_of_floats() {
        // from a given fixed-size array
        let components: [f32; 6] = [1.1, 2.22, 3.333, 4.4444, 5.55555, 6.666666];
        let m = Matrix::from((components, 2, 3));
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        assert_eq!(m.components[0], 1.1);
        assert_eq!(m.components[1], 2.22);
        assert_eq!(m.components[2], 3.333);
        assert_eq!(m.components[3], 4.4444);
        assert_eq!(m.components[4], 5.55555);
        assert_eq!(m.components[5], 6.666666);
    }
}