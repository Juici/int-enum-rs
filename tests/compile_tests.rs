// TODO: Write new compile tests.

#[rustversion::attr(not(nightly), ignore)]
#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_tests/*.rs");
}
