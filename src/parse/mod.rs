mod date;
mod datetime;
mod time;

pub use self::{date::*, datetime::*, time::*};

use nom::{
    self,
    branch::alt,
    character::streaming::char,
    character::streaming::one_of,
    combinator::{map, map_parser, peek},
    number::complete::{float, recognize_float},
    sequence::preceded,
};
use std::ops::{AddAssign, MulAssign};

pub(crate) type ParseResult<'a, T> = nom::IResult<&'a [u8], T, nom::error::Error<&'a [u8]>>;

fn buf_to_int<T>(buf: &[u8]) -> T
where
    T: AddAssign + MulAssign + From<u8>,
{
    let mut sum = T::from(0);
    for digit in buf {
        sum *= T::from(10);
        sum += T::from(*digit - b'0');
    }
    sum
}

fn sign(i: &[u8]) -> ParseResult<i8> {
    alt((
        map(one_of("-\u{2212}\u{2010}"), |_| -1),
        map(char('+'), |_| 1),
    ))(i)
}
/*
named!(
    frac32<f32>,
    do_parse!(
        peek!(char!('.'))
            >> fraction: flat_map!(nom::number::complete::recognize_float, parse_to!(f32))
            >> (fraction)
    )
); */

fn frac32(i: &[u8]) -> ParseResult<f32> {
    preceded(peek(char('.')), map_parser(recognize_float, float))(i)
}

#[cfg(test)]
mod tests {
    use {
        nom::{
            error::{Error, ErrorKind::Char},
            Err,
            Needed::Size,
        },
        std::num::NonZeroUsize,
    };

    #[test]
    fn sign() {
        assert_eq!(super::sign(b"-"), Ok((&[][..], -1)));
        assert_eq!(super::sign(b"+"), Ok((&[][..], 1)));
        assert_eq!(
            super::sign(b""),
            Err(Err::Incomplete(Size(NonZeroUsize::new(1).unwrap())))
        );
        assert_eq!(
            super::sign(b" "),
            Err(Err::Error(Error {
                input: &b" "[..],
                code: Char
            }))
        );
    }
}
