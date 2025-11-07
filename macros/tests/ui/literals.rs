use beaumont_macros::*;

// We need to bring the Decimal type into scope for the beaumont_literals macro to work
type Decimal = f32;

#[beaumont_literals]
fn main() {
    let _ = 1.0d;
}