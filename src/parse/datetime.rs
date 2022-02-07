#![allow(unused_imports)]

use super::*;
use crate::{date::*, datetime::*, time::*};

use nom::{
    character::complete::char,
    combinator::{complete, cond, map, map_res, not, opt, peek},
    error::Error,
    sequence::tuple,
    IResult,
};

use nom_regex::bytes::re_match;

macro_rules! datetime {
    (pub $name:ident, $date:ty, $date_parser:ident, $time:ty, $time_parser:ident) => {
        #[inline]
        pub fn $name(i: &[u8]) -> ParseResult<DateTime<$date, $time>> {
            map(
                tuple(($date_parser, char('T'), peek(not(char('T'))), $time_parser)),
                |(date, _, _, time)| DateTime { date, time },
            )(i)
        }
    };
}

datetime!(pub datetime_global_hms,           Date,       date,        GlobalTime<HmsTime>, time_global_hms);
datetime!(pub datetime_global_hm,            Date,       date,        GlobalTime<HmTime>,  time_global_hm);
datetime!(pub datetime_global_h,             Date,       date,        GlobalTime<HTime>,   time_global_h);
datetime!(pub datetime_local_hms,            Date,       date,        LocalTime<HmsTime>,  time_local_hms);
datetime!(pub datetime_local_hm,             Date,       date,        LocalTime<HmTime>,   time_local_hm);
datetime!(pub datetime_local_h,              Date,       date,        LocalTime<HTime>,    time_local_h);
datetime!(pub datetime_any_hms,              Date,       date,        AnyTime<HmsTime>,    time_any_hms);
datetime!(pub datetime_any_hm,               Date,       date,        AnyTime<HmTime>,     time_any_hm);
datetime!(pub datetime_any_h,                Date,       date,        AnyTime<HTime>,      time_any_h);
datetime!(pub datetime_global_approx,        Date,       date,        ApproxGlobalTime,    time_global_approx);
datetime!(pub datetime_local_approx,         Date,       date,        ApproxLocalTime,     time_local_approx);
datetime!(pub datetime_any_approx,           Date,       date,        ApproxAnyTime,       time_any_approx);
datetime!(pub datetime_approx_global_hms,    ApproxDate, date_approx, GlobalTime<HmsTime>, time_global_hms);
datetime!(pub datetime_approx_global_hm,     ApproxDate, date_approx, GlobalTime<HmTime>,  time_global_hm);
datetime!(pub datetime_approx_global_h,      ApproxDate, date_approx, GlobalTime<HTime>,   time_global_h);
datetime!(pub datetime_approx_local_hms,     ApproxDate, date_approx, LocalTime<HmsTime>,  time_local_hms);
datetime!(pub datetime_approx_local_hm,      ApproxDate, date_approx, LocalTime<HmTime>,   time_local_hm);
datetime!(pub datetime_approx_local_h,       ApproxDate, date_approx, LocalTime<HTime>,    time_local_h);
datetime!(pub datetime_approx_any_hms,       ApproxDate, date_approx, AnyTime<HmsTime>,    time_any_hms);
datetime!(pub datetime_approx_any_hm,        ApproxDate, date_approx, AnyTime<HmTime>,     time_any_hm);
datetime!(pub datetime_approx_any_h,         ApproxDate, date_approx, AnyTime<HTime>,      time_any_h);
datetime!(pub datetime_approx_global_approx, ApproxDate, date_approx, ApproxGlobalTime,    time_global_approx);
datetime!(pub datetime_approx_local_approx,  ApproxDate, date_approx, ApproxLocalTime,     time_local_approx);
datetime!(pub datetime_approx_any_approx,    ApproxDate, date_approx, ApproxAnyTime,       time_any_approx);

pub fn partial_datetime_approx_any_approx(
    i: &[u8],
) -> ParseResult<PartialDateTime<ApproxDate, ApproxAnyTime>> {
    use nom_regex::lib::regex::bytes::Regex;
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new("^(.+T.*|[^T:]*)$").expect("Could not compile regex");
    }
    map_res(
        tuple((
            opt(preceded(peek(re_match(RE.clone())), date_approx)),
            opt(complete(char('T'))),
            opt(complete(peek(not(char('T'))))),
            opt(time_any_approx),
        )),
        |(d, _, _, time)| match (d, time) {
            (None, None) => Err(nom::Err::<&[u8]>::Incomplete(nom::Needed::Unknown)),
            (Some(date), None) => Ok(PartialDateTime::Date(date)),
            (None, Some(time)) => Ok(PartialDateTime::Time(time)),
            (Some(date), Some(time)) => Ok(PartialDateTime::DateTime(DateTime { date, time })),
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn tt() {
        datetime_approx_any_approx(b"2018-08-02TT22:01:39Z").unwrap();
    }

    #[test]
    fn partial_datetime_approx_any_approx_date_y() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"2018"),
            Ok((
                &[][..],
                PartialDateTime::Date(ApproxDate::Y(YDate { year: 2018 }))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_date_ym_basic() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"201808"),
            Ok((
                &[][..],
                PartialDateTime::Date(ApproxDate::YM(YmDate {
                    year: 2018,
                    month: 8,
                }))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_date_ym_extended() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"2018-08"),
            Ok((
                &[][..],
                PartialDateTime::Date(ApproxDate::YM(YmDate {
                    year: 2018,
                    month: 8,
                }))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_date_ymd_basic() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"20180802"),
            Ok((
                &[][..],
                PartialDateTime::Date(ApproxDate::YMD(YmdDate {
                    year: 2018,
                    month: 8,
                    day: 2,
                }))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_date_ymd_extended() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"2018-08-02"),
            Ok((
                &[][..],
                PartialDateTime::Date(ApproxDate::YMD(YmdDate {
                    year: 2018,
                    month: 8,
                    day: 2,
                }))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_time_h() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"T12"),
            Ok((
                &[][..],
                PartialDateTime::Time(ApproxAnyTime::H(AnyTime::Local(LocalTime {
                    naive: HTime { hour: 12 },
                    fraction: 0.,
                })))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_time_hm_basic() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"T1230"),
            Ok((
                &[][..],
                PartialDateTime::Time(ApproxAnyTime::HM(AnyTime::Local(LocalTime {
                    naive: HmTime {
                        hour: 12,
                        minute: 30,
                    },
                    fraction: 0.,
                })))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_time_hm_extended() {
        let result = PartialDateTime::Time(ApproxAnyTime::HM(AnyTime::Local(LocalTime {
            naive: HmTime {
                hour: 12,
                minute: 30,
            },
            fraction: 0.,
        })));

        assert_eq!(
            partial_datetime_approx_any_approx(b"T12:30"),
            Ok((&[][..], result.clone()))
        );
        assert_eq!(
            partial_datetime_approx_any_approx(b"12:30"),
            Ok((&[][..], result))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_time_hms_basic() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"T123015"),
            Ok((
                &[][..],
                PartialDateTime::Time(ApproxAnyTime::HMS(AnyTime::Local(LocalTime {
                    naive: HmsTime {
                        hour: 12,
                        minute: 30,
                        second: 15,
                    },
                    fraction: 0.,
                })))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_time_hms_extended() {
        let result = PartialDateTime::Time(ApproxAnyTime::HMS(AnyTime::Local(LocalTime {
            naive: HmsTime {
                hour: 12,
                minute: 30,
                second: 15,
            },
            fraction: 0.,
        })));

        assert_eq!(
            partial_datetime_approx_any_approx(b"T12:30:15"),
            Ok((&[][..], result.clone()))
        );
        assert_eq!(
            partial_datetime_approx_any_approx(b"12:30:15"),
            Ok((&[][..], result))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_time_hmsf_basic() {
        assert_eq!(
            partial_datetime_approx_any_approx(b"T123015.2"),
            Ok((
                &[][..],
                PartialDateTime::Time(ApproxAnyTime::HMS(AnyTime::Local(LocalTime {
                    naive: HmsTime {
                        hour: 12,
                        minute: 30,
                        second: 15,
                    },
                    fraction: 0.2,
                })))
            ))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_time_hmsf_extended() {
        let result = PartialDateTime::Time(ApproxAnyTime::HMS(AnyTime::Local(LocalTime {
            naive: HmsTime {
                hour: 12,
                minute: 30,
                second: 15,
            },
            fraction: 0.2,
        })));

        assert_eq!(
            partial_datetime_approx_any_approx(b"T12:30:15.2"),
            Ok((&[][..], result.clone()))
        );
        assert_eq!(
            partial_datetime_approx_any_approx(b"12:30:15.2"),
            Ok((&[][..], result))
        );
    }

    #[test]
    fn partial_datetime_approx_any_approx_datetime_extended() {
        let result = PartialDateTime::DateTime(DateTime {
            date: ApproxDate::YMD(YmdDate {
                year: 2018,
                month: 8,
                day: 2,
            }),
            time: ApproxAnyTime::HMS(AnyTime::Local(LocalTime {
                naive: HmsTime {
                    hour: 12,
                    minute: 30,
                    second: 15,
                },
                fraction: 0.2,
            })),
        });

        assert_eq!(
            partial_datetime_approx_any_approx(b"2018-08-02T12:30:15.2"),
            Ok((&[][..], result.clone()))
        );
        assert_eq!(
            partial_datetime_approx_any_approx(b"20180802T123015.2"),
            Ok((&[][..], result))
        );
    }
}
