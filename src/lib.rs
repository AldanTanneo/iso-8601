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

macro_rules! impl_fromstr_parse {
    ($ty:ty, $func:ident) => {
        impl std::str::FromStr for $ty {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                crate::parse::$func(s.as_bytes()).map(|x| x.1).or(Err(()))
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
