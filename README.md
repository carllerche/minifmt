# minifmt

Rust library for performing minimal formatting. Takes [`TokenStream`] arguments
and outputs a formatted string.

** This is not a replacement for rustfmt **

In fact, this crate will most likely be deprecated once rustfmt works on stable.
You also probably shouldn't use this to format handwritten code.

[`TokenStream`]: https://docs.rs/proc-macro2/0.4/proc_macro2/struct.TokenStream.html

## Usage

To use `minfmt`, first add this to your `Cargo.toml`:

```toml
[dependencies]
minifmt = { git = "https://github.com/carllerche/minifmt" } # Soon on crates.io
```

Next, add this to your crate:

```rust
extern crate minifmt;

fn main() {
    // Generate a TokenStream somehow.
    let token_stream = ...;

    println!("{}", minifmt::fmt(token_stream));
}

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bytes` by you, shall be licensed as MIT, without any additional
terms or conditions.
