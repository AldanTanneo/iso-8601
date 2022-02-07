use super::*;
use crate::date::*;
use nom::character::is_digit;

use nom::{
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::char,
    combinator::{complete, cond, map, opt},
    sequence::{pair, separated_pair, tuple},
};

#[inline]
fn positive_century(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(2, 2, is_digit), buf_to_int)(i)
}

#[inline]
fn century(i: &[u8]) -> ParseResult<i8> {
    map(pair(opt(sign), positive_century), |(sign, century)| {
        sign.unwrap_or(1) * century as i8
    })(i)
}

#[inline]
// TODO support expanded year
fn positive_year(i: &[u8]) -> ParseResult<u16> {
    map(take_while_m_n(4, 4, is_digit), buf_to_int)(i)
}

#[inline]
fn year(i: &[u8]) -> ParseResult<i16> {
    map(pair(opt(sign), positive_year), |(sign, year)| {
        sign.unwrap_or(1) as i16 * year as i16
    })(i)
}

#[inline]
fn month(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(2, 2, is_digit), buf_to_int)(i)
}

#[inline]
fn day(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(2, 2, is_digit), buf_to_int)(i)
}

#[inline]
fn year_week(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(2, 2, is_digit), buf_to_int)(i)
}

#[inline]
fn year_day(i: &[u8]) -> ParseResult<u16> {
    map(take_while_m_n(3, 3, is_digit), buf_to_int)(i)
}

#[inline]
fn week_day(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(1, 1, is_digit), buf_to_int)(i)
}

#[inline]
fn date_ymd_format(i: &[u8], extended: bool) -> ParseResult<YmdDate> {
    map(
        tuple((
            year,
            cond(extended, char('-')),
            month,
            cond(extended, char('-')),
            day,
        )),
        |(year, _, month, _, day)| YmdDate { year, month, day },
    )(i)
}

#[inline]
fn date_ymd_basic(i: &[u8]) -> ParseResult<YmdDate> {
    date_ymd_format(i, false)
}

#[inline]
fn date_ymd_extended(i: &[u8]) -> ParseResult<YmdDate> {
    date_ymd_format(i, true)
}

#[inline]
pub fn date_ymd(i: &[u8]) -> ParseResult<YmdDate> {
    alt((date_ymd_extended, date_ymd_basic))(i)
}

#[inline]
fn date_wd_format(i: &[u8], extended: bool) -> ParseResult<WdDate> {
    map(
        tuple((
            year,
            cond(extended, char('-')),
            char('W'),
            year_week,
            cond(extended, char('-')),
            week_day,
        )),
        |(year, _, _, week, _, day)| WdDate { year, week, day },
    )(i)
}

#[inline]
fn date_wd_basic(i: &[u8]) -> ParseResult<WdDate> {
    date_wd_format(i, false)
}

#[inline]
fn date_wd_extended(i: &[u8]) -> ParseResult<WdDate> {
    date_wd_format(i, true)
}

#[inline]
pub fn date_wd(i: &[u8]) -> ParseResult<WdDate> {
    alt((date_wd_extended, date_wd_basic))(i)
}

#[inline]
fn date_o_format(i: &[u8], extended: bool) -> ParseResult<ODate> {
    map(
        separated_pair(year, cond(extended, char('-')), year_day),
        |(year, day)| ODate { year, day },
    )(i)
}

#[inline]
fn date_o_basic(i: &[u8]) -> ParseResult<ODate> {
    date_o_format(i, false)
}

#[inline]
fn date_o_extended(i: &[u8]) -> ParseResult<ODate> {
    date_o_format(i, true)
}

#[inline]
pub fn date_o(i: &[u8]) -> ParseResult<ODate> {
    alt((date_o_extended, date_o_basic))(i)
}

#[inline]
pub fn date(i: &[u8]) -> ParseResult<Date> {
    alt((
        complete(map(date_wd, Date::WD)),
        complete(map(date_ymd, Date::YMD)),
        complete(map(date_o, Date::O)),
    ))(i)
}

#[inline]
fn date_w_format(i: &[u8], extended: bool) -> ParseResult<WDate> {
    map(
        tuple((year, cond(extended, char('-')), char('W'), year_week)),
        |(year, _, _, week)| WDate { year, week },
    )(i)
}

#[inline]
fn date_w_basic(i: &[u8]) -> ParseResult<WDate> {
    date_w_format(i, false)
}

#[inline]
fn date_w_extended(i: &[u8]) -> ParseResult<WDate> {
    date_w_format(i, true)
}

#[inline]
pub fn date_w(i: &[u8]) -> ParseResult<WDate> {
    alt((date_w_extended, date_w_basic))(i)
}

#[inline]
fn date_ym_format(i: &[u8], extended: bool) -> ParseResult<YmDate> {
    map(
        separated_pair(year, cond(extended, char('-')), month),
        |(year, month)| YmDate { year, month },
    )(i)
}

#[inline]
fn date_ym_basic(i: &[u8]) -> ParseResult<YmDate> {
    date_ym_format(i, false)
}

#[inline]
fn date_ym_extended(i: &[u8]) -> ParseResult<YmDate> {
    date_ym_format(i, true)
}

#[inline]
pub fn date_ym(i: &[u8]) -> ParseResult<YmDate> {
    alt((date_ym_extended, date_ym_basic))(i)
}

#[inline]
pub fn date_y(i: &[u8]) -> ParseResult<YDate> {
    map(year, |year| YDate { year })(i)
}

#[inline]
pub fn date_c(i: &[u8]) -> ParseResult<CDate> {
    map(century, |century| CDate { century })(i)
}

#[inline]
pub fn date_approx(i: &[u8]) -> ParseResult<ApproxDate> {
    alt((
        map(date, |x| x.into()),
        map(date_w, ApproxDate::W),
        map(date_ym, ApproxDate::YM),
        map(date_y, ApproxDate::Y),
        map(date_c, ApproxDate::C),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_year() {
        assert_eq!(super::positive_year(b"2018"), Ok((&[][..], 2018)));
    }

    #[test]
    fn year() {
        assert_eq!(super::year(b"2018"), Ok((&[][..], 2018)));
        assert_eq!(super::year(b"+2018"), Ok((&[][..], 2018)));
        assert_eq!(super::year(b"-2018"), Ok((&[][..], -2018)));
    }

    #[test]
    fn month() {
        assert_eq!(super::month(b"06"), Ok((&[][..], 6)));
        assert_eq!(super::month(b"12"), Ok((&[][..], 12)));
    }

    #[test]
    fn year_week() {
        assert_eq!(super::year_week(b"01"), Ok((&[][..], 1)));
    }

    #[test]
    fn year_day() {
        assert_eq!(super::year_day(b"001"), Ok((&[][..], 1)));
        assert_eq!(super::year_day(b"011"), Ok((&[][..], 11)));
        assert_eq!(super::year_day(b"111"), Ok((&[][..], 111)));
        assert_eq!(super::year_day(b"1111"), Ok((&b"1"[..], 111)));
    }

    #[test]
    fn day() {
        assert_eq!(super::day(b"18"), Ok((&[][..], 18)));
    }

    #[test]
    fn week_day() {
        assert_eq!(super::week_day(b"1"), Ok((&[][..], 1)));
        assert_eq!(super::week_day(b"2"), Ok((&[][..], 2)));
        assert_eq!(super::week_day(b"3"), Ok((&[][..], 3)));
        assert_eq!(super::week_day(b"4"), Ok((&[][..], 4)));
        assert_eq!(super::week_day(b"5"), Ok((&[][..], 5)));
        assert_eq!(super::week_day(b"6"), Ok((&[][..], 6)));
        assert_eq!(super::week_day(b"7"), Ok((&[][..], 7)));
    }

    #[test]
    fn date_ymd() {
        {
            let value = YmdDate {
                year: 2015,
                month: 7,
                day: 16,
            };
            assert_eq!(super::date_ymd(b"2015-07-16"), Ok((&[][..], value.clone())));
            assert_eq!(super::date_ymd(b"20150716"), Ok((&[][..], value)));
        }
        {
            let value = YmdDate {
                year: -333,
                month: 6,
                day: 11,
            };
            assert_eq!(
                super::date_ymd(b"-0333-06-11"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(super::date_ymd(b"-03330611"), Ok((&[][..], value)));
        }
        assert_eq!(
            super::date_ymd(b"2016-02-29"),
            Ok((
                &[][..],
                YmdDate {
                    year: 2016,
                    month: 2,
                    day: 29
                }
            ))
        );
    }

    #[test]
    fn date_ym() {
        assert_eq!(
            super::date_ym(b"2016-02"),
            Ok((
                &[][..],
                YmDate {
                    year: 2016,
                    month: 2
                }
            ))
        );
        assert_eq!(
            super::date_ym(b"201602"),
            Ok((
                &[][..],
                YmDate {
                    year: 2016,
                    month: 2
                }
            ))
        );
    }

    #[test]
    fn date_y() {
        assert_eq!(super::date_y(b"2016"), Ok((&[][..], YDate { year: 2016 })));
    }

    #[test]
    fn date_c() {
        assert_eq!(super::date_c(b"20"), Ok((&[][..], CDate { century: 20 })));
    }

    #[test]
    fn date_wd() {
        assert_eq!(
            super::date_wd(b"2018-W01-1"),
            Ok((
                &[][..],
                WdDate {
                    year: 2018,
                    week: 1,
                    day: 1
                }
            ))
        );
        assert_eq!(
            super::date_wd(b"2018-W52-7"),
            Ok((
                &[][..],
                WdDate {
                    year: 2018,
                    week: 52,
                    day: 7
                }
            ))
        );
        assert_eq!(
            super::date_wd(b"2018W223"),
            Ok((
                &[][..],
                WdDate {
                    year: 2018,
                    week: 22,
                    day: 3
                }
            ))
        );
    }

    #[test]
    fn date_w() {
        let value = WDate {
            year: 2020,
            week: 53,
        };
        assert_eq!(super::date_w(b"2020-W53 "), Ok((&b" "[..], value.clone())));
        assert_eq!(super::date_w(b"2020-W53"), Ok((&[][..], value.clone())));
        assert_eq!(super::date_w(b"2020W53 "), Ok((&b" "[..], value.clone())));
        assert_eq!(super::date_w(b"2020W53"), Ok((&[][..], value)));
    }

    #[test]
    fn date_o() {
        use crate::Valid;

        let value = ODate {
            year: 1985,
            day: 102,
        };
        assert_eq!(super::date_o(b"1985-102"), Ok((&[][..], value.clone())));
        assert_eq!(super::date_o(b"1985102"), Ok((&[][..], value)));
        assert!(!super::date_o(b"2022-367").unwrap().1.is_valid());
    }

    #[test]
    fn date() {
        {
            let value = Date::YMD(YmdDate {
                year: 2018,
                month: 2,
                day: 12,
            });
            assert_eq!(super::date(b"2018-02-12"), Ok((&[][..], value.clone())));
            assert_eq!(super::date(b"2018-02-12 "), Ok((&b" "[..], value)));
        }

        {
            let value = Date::WD(WdDate {
                year: 2018,
                week: 2,
                day: 2,
            });
            assert_eq!(super::date(b"2018-W02-2"), Ok((&[][..], value.clone())));
            assert_eq!(super::date(b"2018-W02-2 "), Ok((&b" "[..], value)));
        }

        {
            let value = Date::O(ODate {
                year: 2018,
                day: 102,
            });
            assert_eq!(super::date(b"2018-102"), Ok((&[][..], value.clone())));
            assert_eq!(super::date(b"2018-102 "), Ok((&b" "[..], value)));
        }
    }

    #[test]
    fn date_approx() {
        {
            let value = ApproxDate::YMD(YmdDate {
                year: 2000,
                month: 5,
                day: 5,
            });
            assert_eq!(
                super::date_approx(b"2000-05-05 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"20000505 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"2000-05-05"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(super::date_approx(b"20000505"), Ok((&[][..], value)));
        }
        {
            let value = ApproxDate::YM(YmDate {
                year: 2000,
                month: 5,
            });
            assert_eq!(
                super::date_approx(b"2000-05 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(super::date_approx(b"2000-05"), Ok((&[][..], value)));
        }
        {
            let value = ApproxDate::Y(YDate { year: 2000 });
            assert_eq!(super::date_approx(b"2000 "), Ok((&b" "[..], value.clone())));
            assert_eq!(super::date_approx(b"2000"), Ok((&[][..], value)));
        }
        {
            let value = ApproxDate::C(CDate { century: 20 });
            assert_eq!(super::date_approx(b"20 "), Ok((&b" "[..], value.clone())));
            assert_eq!(super::date_approx(b"20"), Ok((&[][..], value)));
        }

        {
            let value = ApproxDate::WD(WdDate {
                year: 2000,
                week: 5,
                day: 5,
            });
            assert_eq!(
                super::date_approx(b"2000-W05-5 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"2000-W05-5"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"2000W055 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(super::date_approx(b"2000W055"), Ok((&[][..], value)));
        }
        {
            let value = ApproxDate::W(WDate {
                year: 2000,
                week: 5,
            });
            assert_eq!(
                super::date_approx(b"2000-W05 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"2000-W05"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"2000W05 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(super::date_approx(b"2000W05"), Ok((&[][..], value)));
        }

        {
            let value = ApproxDate::O(ODate { year: 2000, day: 5 });
            assert_eq!(
                super::date_approx(b"2000-005 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"2000-005"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::date_approx(b"2000005 "),
                Ok((&b" "[..], value.clone()))
            );
            assert_eq!(super::date_approx(b"2000005"), Ok((&[][..], value)));
        }
    }
}
