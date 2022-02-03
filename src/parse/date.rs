use super::*;
use crate::date::*;
use nom::character::is_digit;

named!(
    positive_century<u8>,
    map!(take_while_m_n!(2, 2, is_digit), buf_to_int)
);

named!(
    century<i8>,
    do_parse!(sign: opt!(sign) >> century: positive_century >> (sign.unwrap_or(1) * century as i8))
);

// TODO support expanded year
named!(
    positive_year<u16>,
    map!(take_while_m_n!(4, 4, is_digit), buf_to_int)
);

named!(
    year<i16>,
    do_parse!(sign: opt!(sign) >> year: positive_year >> (sign.unwrap_or(1) as i16 * year as i16))
);

named!(month<u8>, map!(take_while_m_n!(2, 2, is_digit), buf_to_int));

named!(day<u8>, map!(take_while_m_n!(2, 2, is_digit), buf_to_int));

named!(
    year_week<u8>,
    map!(take_while_m_n!(2, 2, is_digit), buf_to_int)
);

named!(
    year_day<u8>,
    map!(take_while_m_n!(3, 3, is_digit), buf_to_int)
);

named!(
    week_day<u8>,
    map!(take_while_m_n!(1, 1, is_digit), buf_to_int)
);

named_args!(date_ymd_format(extended: bool) <YmdDate>, do_parse!(
    year: year >>
    cond!(extended, char!('-')) >>
    month: month >>
    cond!(extended, char!('-')) >>
    day: day >>
    (YmdDate { year, month, day })
));
named!(date_ymd_basic<YmdDate>, call!(date_ymd_format, false));
named!(date_ymd_extended<YmdDate>, call!(date_ymd_format, true));

named!(pub date_ymd <YmdDate>, alt!(
    complete!(date_ymd_extended) |
    complete!(date_ymd_basic)
));

named_args!(date_wd_format(extended: bool) <WdDate>, do_parse!(
    year: year >>
    cond!(extended, char!('-')) >>
    char!('W') >>
    week: year_week >>
    cond!(extended, char!('-')) >>
    day: week_day >>
    (WdDate { year, week, day })
));
named!(date_wd_basic<WdDate>, call!(date_wd_format, false));
named!(date_wd_extended<WdDate>, call!(date_wd_format, true));

named!(pub date_wd <WdDate>, alt!(
   date_wd_extended |
   date_wd_basic
));

named_args!(date_o_format(extended: bool) <ODate>, do_parse!(
    year: year >>
    cond!(extended, char!('-')) >>
    day: year_day >>
    (ODate {
        year,
        day: day.into()
    })
));
named!(date_o_basic<ODate>, call!(date_o_format, false));
named!(date_o_extended<ODate>, call!(date_o_format, true));

named!(pub date_o <ODate>, alt!(
    date_o_extended |
    date_o_basic
));

named!(pub date <Date>, alt!(
    complete!(map!(date_wd, Date::WD)) |
    complete!(map!(date_ymd_extended, Date::YMD)) |
    complete!(map!(date_o_extended, Date::O)) |
    complete!(map!(date_ymd_basic, Date::YMD)) |
    complete!(map!(date_o_basic, Date::O))
));

named_args!(date_w_format(extended: bool) <WDate>, do_parse!(
    year: year >>
    cond!(extended, char!('-')) >>
    char!('W') >>
    week: year_week >>
    (WDate { year, week })
));
named!(date_w_basic<WDate>, call!(date_w_format, false));
named!(date_w_extended<WDate>, call!(date_w_format, true));

named!(pub date_w <WDate>, alt!(
    date_w_extended |
    date_w_basic
));

named_args!(date_ym_format(extended: bool) <YmDate>, do_parse!(
    year: year >>
    cond!(extended, char!('-')) >>
    month: month >>
    (YmDate { year, month })
));
named!(date_ym_basic<YmDate>, call!(date_ym_format, false));
named!(date_ym_extended<YmDate>, call!(date_ym_format, true));

named!(pub date_ym <YmDate>, alt!(
    date_ym_extended |
    date_ym_basic
));

named!(pub date_y <YDate>, map!(year, |year| YDate { year }));

named!(pub date_c <CDate>, map!(century, |century| CDate { century }));

named!(pub date_approx <ApproxDate>, alt!(
    complete!(map!(date, |x| x.into())) |
    complete!(map!(date_w, ApproxDate::W)) |
    complete!(map!(date_ym, ApproxDate::YM)) |
    complete!(map!(date_y, ApproxDate::Y)) |
    complete!(map!(date_c, ApproxDate::C))
));

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
        let value = ODate {
            year: 1985,
            day: 102,
        };
        assert_eq!(super::date_o(b"1985-102"), Ok((&[][..], value.clone())));
        assert_eq!(super::date_o(b"1985102"), Ok((&[][..], value)));
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
