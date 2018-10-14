use syn;

/// A formatting error
#[derive(Debug)]
pub struct Error { _p: () }

impl From<syn::parse::Error> for Error {
    fn from(_: syn::parse::Error) -> Error {
        Error { _p: () }
    }
}
