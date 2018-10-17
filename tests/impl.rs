#![recursion_limit="1024"]

extern crate minifmt;
#[macro_use]
extern crate quote;

#[macro_use]
mod support;

#[test]
fn empty_impl_block() {
    check! {
        r#"
impl MyStruct {
}
"#,
        impl MyStruct {
        }
    }
}

#[test]
fn attributes() {
    check! {
        r#"
#[foo]
impl MyStruct {
    #![bar]
}
"#,
        #[foo]
        impl MyStruct {
            #![bar]
        }
    }
}

#[test]
fn init_tuple_struct() {
    check! {
        r#"
impl Foo {
    fn new() -> Self {
        Foo(1, "two", foo(), 2 + 3)
    }
}
"#,
        impl Foo {
            fn new() -> Self {
                Foo(1, "two", foo(), 2 + 3)
            }
        }
    }
}

#[test]
fn basic_functions() {
    check! {
        r#"
impl MyStruct {
    fn new() -> FormatFile {
        FormatFile {
            out: "".to_string(),
            indent: 0,
        }
    }

    fn visit_attributes(&mut self, i: &[syn::Attribute]) {
        for attr in i {
            self.visit_attribute(attr);
        }
    }

    fn visit_inner_attributes(&mut self, i: &[syn::Attribute]) {
        for attr in i {
            if is_inner_attr(attr) {
                self.visit_attribute(attr);
            }
        }
    }

    fn visit_outer_attributes(&mut self, i: &[syn::Attribute]) {
        for attr in i {
            if !is_inner_attr(attr) {
                self.visit_attribute(attr);
            }
        }
    }

    fn visit_doc_attribute(&mut self, i: &syn::Attribute) {
        use syn::Meta::*;
        use syn::Lit::*;

        let meta = i.parse_meta().unwrap();
        match meta {
            NameValue(name_value) => {
                match name_value.lit {
                    Str(s) => {
                        self.visit_doc_comment(is_inner_attr(i), &s.value());
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}
"#,
        impl MyStruct {
            fn new() -> FormatFile {
                FormatFile {
                    out: "".to_string(),
                    indent: 0,
                }
            }

            fn visit_attributes(&mut self, i: &[syn::Attribute]) {
                for attr in i {
                    self.visit_attribute(attr);
                }
            }

            fn visit_inner_attributes(&mut self, i: &[syn::Attribute]) {
                for attr in i {
                    if is_inner_attr(attr) {
                        self.visit_attribute(attr);
                    }
                }
            }

            fn visit_outer_attributes(&mut self, i: &[syn::Attribute]) {
                for attr in i {
                    if !is_inner_attr(attr) {
                        self.visit_attribute(attr);
                    }
                }
            }

            fn visit_doc_attribute(&mut self, i: &syn::Attribute) {
                use syn::Meta::*;
                use syn::Lit::*;

                let meta = i.parse_meta().unwrap();

                match meta {
                    NameValue(name_value) => {
                        match name_value.lit {
                            Str(s) => {
                                self.visit_doc_comment(is_inner_attr(i), &s.value());
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }
}
