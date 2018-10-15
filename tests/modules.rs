extern crate minifmt;
#[macro_use]
extern crate quote;

#[macro_use]
mod support;

#[test]
fn empty_module() {
    check! {
        r#"
mod my_module;
"#,
        mod my_module;
    }

    check! {
        r#"
pub mod my_module;
"#,
        pub mod my_module;
    }

    check! {
        r#"
pub mod my_module {
}
"#,
        pub mod my_module { }
    }
}

#[test]
fn use_statements() {
    check! {
        r#"
mod my_module {
    use std::io;
}
"#,
        mod my_module {
            use std::io;
        }
}

    check! {
        r#"
mod my_module {
    #[foo]
    #[bar = "baz"]
    use std::io;
}
"#,
        mod my_module {
            #[foo]
            #[bar = "baz"]
            use std::io;
        }
    }

    check! {
        r#"
mod my_module {
    pub use std::io;
}
"#,
        mod my_module {
            pub use std::io;
        }
    }
}
