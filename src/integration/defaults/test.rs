use crate::defaults::IntegrationTest; //IntegrationTest;

inventory::submit!(IntegrationTest {
    name: "indev",
    test_fn: indev,
    indev: Some(true),
});

// #[cfg(not(feature = "ci"))]
pub fn indev() {
    // To generate macro result files
    let src = "src/integration/defaults/tests/001_sync.rs";
    let srcx = "src/integration/defaults/tests/001_sync.expanded.rs";
    macrotest::expand(src);

    // #[cfg(feature = "as")]
    // macrotest::expand_args(src, &["--features", "minitrace-tests/as"]);
    // #[cfg(feature = "ea")]
    // macrotest::expand_args(src, &["--features", "minitrace-tests/ea"]);
    // #[cfg(feature = "tk")]
    // macrotest::expand_args(
    //     src,
    //     &[
    //         "--features",
    //         "minitrace-tests/tk",
    //         "--manifest-path",
    //         "./Cargo.toml",
    //     ],
    // );

    build_indev(srcx);
}

fn build_indev(src: &str) {
    let t = trybuild::TestCases::new();
    t.pass(src);
}
