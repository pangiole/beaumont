use super::Vector;


/// Create a new mathematical vector from a boxed slice
impl<T> From<Box<[T]>> for Vector<T> where T: PartialEq + Copy {
    fn from(arr: Box<[T]>) -> Self {
        Self { components: arr }
    }
}


/// Create a new mathematical vector from a stack-allocated fixed-size array
impl<T, const LEN: usize> From<[T; LEN]> for Vector<T> where T: PartialEq + Copy {
    fn from(arr: [T; LEN]) -> Self {
        Self { components: arr.into() }
    }
}

// TODO impl Default for Vector


#[cfg(test)]
mod test {
    use super::Vector;

    #[test]
    fn from_array_of_integers() {
        // from a given boxed slice
        let components: Box<[i32]> = Box::new([1, 2, 3]);
        let v = Vector::from(components);
        for (i, &el) in v.components.iter().enumerate() {
            assert_eq!(el, (i + 1) as i32);
        }
    }

    #[test]
    fn from_array_of_floats() {
        // from a given fixed-size array
        let arr: [f32; 6] = [1.01, 2.32, 3.675, 4.8976, 5.234567, 6.897654321];
        let v = Vector::from(arr);
        assert_eq!(v.components.len(), 6);
        assert_eq!(v.components[0], 1.01);
        assert_eq!(v.components[1], 2.32);
        assert_eq!(v.components[2], 3.675);
        assert_eq!(v.components[3], 4.8976);
        assert_eq!(v.components[4], 5.234567);
        assert_eq!(v.components[5], 6.897654321);
    }
}