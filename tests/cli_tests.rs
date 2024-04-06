use trycmd::TestCases;

#[test]
fn cli_tests() {
    let cases = TestCases::new();
    cases.default_bin_name("nrr");

    cases
        .case("tests/default/*.toml")
        .case("tests/run/*.toml")
        .case("tests/exec/*.toml")
        .case("tests/list/*.toml");

    #[cfg(unix)]
    cases.skip("tests/**/*.windows.toml");
    #[cfg(windows)]
    cases.skip("tests/**/*.unix.toml");
}
