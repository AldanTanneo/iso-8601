// https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}

extern crate nom;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    InvalidFormat,
    InvalidDate,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            InvalidFormat => write!(f, "Invalid ISO-8601 format"),
            InvalidDate => write!(f, "Invalid date or time"),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! impl_fromstr_parse {
    ($ty:ty, $func:ident) => {
        impl std::str::FromStr for $ty {
            type Err = crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use crate::Valid;

                let res = crate::parse::$func(s.as_bytes())
                    .map(|x| x.1)
                    .or(Err(Self::Err::InvalidFormat))?;

                res.is_valid().then(|| res).ok_or(Self::Err::InvalidDate)
            }
        }
    };
}

pub mod chrono;
mod date;
mod datetime;
mod parse;
mod time;

pub use {date::*, datetime::*, time::*};

pub trait Valid {
    fn is_valid(&self) -> bool;
}
