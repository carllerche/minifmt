extern crate minifmt;
#[macro_use]
extern crate quote;

#[macro_use]
mod support;

#[test]
fn empty_struct() {
    check! {
        r#"
struct MyStruct;
"#,
        struct MyStruct;
    }

    check! {
        r#"
pub struct MyStruct;
"#,
        pub struct MyStruct;
    }

    check! {
        r#"
pub struct MyStruct {
}
"#,
        pub struct MyStruct { }
    }
}

#[test]
fn fields() {
    check! {
        r#"
struct MyStruct {
    foo: Bar,
    baz: usize,
    wut: (A, u32),
    arr: [u8; 64],
}
"#,
        struct MyStruct {
            foo: Bar,
            baz: usize,
            wut: (A, u32),
            arr: [u8; 64],
        }
    }
}

#[test]
fn generics() {
    check! {
        r#"
struct MyStruct<T, U: One, V: Two<U>> {
    _p: (T, U, V),
}
"#,
        struct MyStruct<T, U: One, V: Two<U>> {
            _p: (T, U, V),
        }
    }
}

#[test]
fn derive() {
    check! {
        r#"
#[derive(Foo, Bar, Baz)]
struct MyStruct {
}
"#,
        #[derive(Foo, Bar, Baz)]
        struct MyStruct {
        }
    }
}
