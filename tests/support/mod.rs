#[macro_export]
macro_rules! check {
    ($expect:expr, $($actual:tt)*) => {{
        let actual = match minifmt::fmt_file(quote!($($actual)*)) {
            Ok(actual) => actual,
            Err(e) => panic!("failed; error = {:?}", e),
        };

        let expect = $expect;

        if actual != &expect[1..] {
            panic!("assertion failed. \n\n=== Expected: ===\n\n---\n{}---\n\n=== Actual: ===\n\n---\n{}---\n\n",
                   &expect[1..], actual);
        }
    }}
}
