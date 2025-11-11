#[macro_use]
extern crate beaumont_macros;
pub struct Vector {
    components: Box<[f64]>,
}
impl Vector {
    pub fn from(components: Box<[f64]>) -> Self {
        Self { components }
    }
}
fn main() {
    Vector::from(
        <[_]>::into_vec(::alloc::boxed::box_new([1.0, 2.0, 3.0])).into_boxed_slice(),
    );
}
