#[macro_export]
macro_rules! check {
    ($expect:expr, $($actual:tt)*) => {{
        let actual = match minifmt::fmt_file(quote!($($actual)*)) {
            Ok(actual) => actual,
            Err(e) => panic!("failed; error = {:?}", e),
        };

        let expect = $expect;

        assert_eq!(actual, &expect[1..]);
    }}
}
