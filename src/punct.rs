use syn::token;

pub enum Space {
    NoSpace,
    SpaceBoth,
    SpaceRight,
    NewLine,
}

pub trait Punctuation {
    fn as_str(&self) -> &str;
}

macro_rules! impl_punct {
    ($t:ident, $s:expr) => {
        impl Punctuation for token::$t {
            fn as_str(&self) -> &str {
                $s
            }
        }
    }
}

impl_punct!(Add, "+");
impl_punct!(Comma, ",");
impl_punct!(Colon2, "::");
impl_punct!(Or, "|");
