#[macro_use]
extern crate beaumont_macros;
struct Matrix {
    components: Box<[i32]>,
    rows: u32,
    cols: u32,
}
impl Matrix {
    fn from(tuple: (Box<[i32]>, u32, u32)) -> Self {
        let (components, rows, cols) = tuple;
        Self { components, rows, cols }
    }
}
fn main() {
    matrix![ 1  , 2  , 3  |
           | 4  , 5  , 6  |
           | 7  , 8  , 9  |
           | 10 , 11 , 12 ];
}