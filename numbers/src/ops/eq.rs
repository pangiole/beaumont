use crate::Decimal;

impl PartialEq for Decimal {

    /// Tests for `self` and `other` values to be equal, and is used by `==`.
    fn eq(&self, other: &Self) -> bool {
        // TODO Avoid having to convert to_string. We may be able to leverage coefficient and precision
        self.to_string().trim_end_matches("0") ==  other.to_string().trim_end_matches("0")
    }
}

impl Eq for Decimal {}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn eq() {
        let d1 = Decimal::new(123456, 2);  // "1234.56""
        let d2 = Decimal::new(1234560, 3); // "1234.560"
        assert_eq!(d1, d2);
    }

    #[test]
    fn neq() {
        let d1 = Decimal::new(1234560, 2); // "12345.60""
        let d2 = Decimal::new(1234560, 3); //  "1234.560"
        assert_ne!(d1, d2);
    }
}