use crate::{Dimension, Vector};

/// Iterator over the components of a vector.
pub struct VectorIterator<'a, T> where T: Copy + PartialEq  {
    // The referred iterable vector shall outlive this field
    iterable: &'a Vector<T>,
    // The current index (state) of the iterator
    index: u64
}

impl<'a, T> Iterator for VectorIterator<'a, T> where T: Copy + PartialEq  {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.iterable.dim() {
            let component = self.iterable.components[self.index as usize];
            self.index += 1;
            Some(component)
        }
        else {
            None
        }
    }
}

impl<T> Vector<T> where T: Copy + PartialEq {
    /// The iterator over the components of this vector.
    pub fn iter(&self) -> VectorIterator<'_, T> {
        // Using `'_` to let the compiler infer the correct, anonymous lifetime
        VectorIterator { iterable: self, index: 0}
    }
}


// TODO impl<'a, T> IntoIterator for &'a Vector<T> where T: Copy + PartialEq {
//     type Item = &'a T;
//     type IntoIter = VectorIterator<'a, &'a T>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         VectorIterator { iterable: self, index: 0}
//     }
// }



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator() {
        let v = Vector { components: Box::new([0, 1, 2]) };
        for (i, c) in v.iter().enumerate() {
            assert_eq!(i, c);
        }
    }
}