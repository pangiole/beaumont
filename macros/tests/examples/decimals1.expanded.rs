#[macro_use]
extern crate beaumont_macros;
type Decimal = f32;
fn main() {
    let _ = "1.0".parse::<Decimal>().unwrap();
}
