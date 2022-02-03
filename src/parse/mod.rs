mod date;
mod time;
mod datetime;

pub use self::{
    date::*,
    time::*,
    datetime::*
};

use {
    std::ops::{
        AddAssign,
        MulAssign
    },
    nom
};

fn buf_to_int<T>(buf: &[u8]) -> T
where T: AddAssign + MulAssign + From<u8> {
    let mut sum = T::from(0);
    for digit in buf {
        sum *= T::from(10);
        sum += T::from(*digit - b'0');
    }
    sum
}

named!(sign <i8>, alt!(
    one_of!("-\u{2212}\u{2010}") => { |_| -1 } |
    char!('+')                   => { |_|  1 }
));

named!(frac32 <f32>, do_parse!(
    peek!(char!('.')) >>
    fraction: flat_map!(nom::number::complete::recognize_float, parse_to!(f32)) >>
    (fraction)
));

#[cfg(test)]
mod tests {
    use {
        std::num::NonZeroUsize,
        nom::{
            Err,
            error::{
                Error,
                ErrorKind::Alt
            },
            Needed::Size
        }
    };

    #[test]
    fn sign() {
        assert_eq!(super::sign(b"-"), Ok((&[][..], -1)));
        assert_eq!(super::sign(b"+"), Ok((&[][..],  1)));
        assert_eq!(super::sign(b"" ), Err(Err::Incomplete(Size(NonZeroUsize::new(1).unwrap()))));
        assert_eq!(super::sign(b" "), Err(Err::Error(Error { input: &b" "[..], code: Alt })));
    }
}
