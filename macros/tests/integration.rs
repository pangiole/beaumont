
#[test]
#[ignore]
fn good_decimals() {
    macrotest::expand("tests/examples/decimals1.rs");
    trybuild::TestCases::new().pass("tests/examples/decimals1.rs");
}

#[test]
#[ignore]
fn good_vectors() {
    macrotest::expand("tests/examples/vector1.rs");
    trybuild::TestCases::new().pass("tests/examples/vector1.rs");

}

#[test]
#[ignore]
fn bad_vectors() {
    trybuild::TestCases::new().compile_fail("tests/examples/vector2.rs");
    trybuild::TestCases::new().compile_fail("tests/examples/vector3.rs");
}



#[test]
// #[ignore]
fn good_matrices() {
    macrotest::expand("tests/examples/matrix1.rs");
    trybuild::TestCases::new().pass("tests/examples/matrix1.rs");
}

#[test]
#[ignore]
fn bad_matrices() {
    trybuild::TestCases::new().compile_fail("tests/examples/matrix2.rs");
    trybuild::TestCases::new().compile_fail("tests/examples/matrix3.rs");
    trybuild::TestCases::new().compile_fail("tests/examples/matrix4.rs");
    trybuild::TestCases::new().compile_fail("tests/examples/matrix5.rs");
    trybuild::TestCases::new().compile_fail("tests/examples/matrix6.rs");
}