use super::*;
use crate::time::*;
use nom::{
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::char,
    character::is_digit,
    combinator::{complete, cond, map, opt},
    sequence::{pair, preceded, separated_pair, tuple},
};

#[inline]
fn hour(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(2, 2, is_digit), buf_to_int)(i)
}

#[inline]
fn minute(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(2, 2, is_digit), buf_to_int)(i)
}

#[inline]
fn second(i: &[u8]) -> ParseResult<u8> {
    map(take_while_m_n(2, 2, is_digit), buf_to_int)(i)
}

#[inline]
fn time_hms_format(i: &[u8], extended: bool) -> ParseResult<HmsTime> {
    map(
        tuple((
            hour,
            cond(extended, char(':')),
            minute,
            cond(extended, char(':')),
            second,
        )),
        |(hour, _, minute, _, second)| HmsTime {
            hour,
            minute,
            second,
        },
    )(i)
}

#[inline]
fn time_hms_basic(i: &[u8]) -> ParseResult<HmsTime> {
    time_hms_format(i, false)
}

#[inline]
fn time_hms_extended(i: &[u8]) -> ParseResult<HmsTime> {
    time_hms_format(i, true)
}

#[inline]
pub fn time_hms(i: &[u8]) -> ParseResult<HmsTime> {
    alt((time_hms_extended, time_hms_basic))(i)
}

#[inline]
fn time_hm_format(i: &[u8], extended: bool) -> ParseResult<HmTime> {
    map(
        separated_pair(hour, cond(extended, char(':')), minute),
        |(hour, minute)| HmTime { hour, minute },
    )(i)
}

#[inline]
fn time_hm_basic(i: &[u8]) -> ParseResult<HmTime> {
    time_hm_format(i, false)
}

#[inline]
fn time_hm_extended(i: &[u8]) -> ParseResult<HmTime> {
    time_hm_format(i, true)
}

#[inline]
pub fn time_hm(i: &[u8]) -> ParseResult<HmTime> {
    alt((time_hm_extended, time_hm_basic))(i)
}

#[inline]
pub fn time_h(i: &[u8]) -> ParseResult<HTime> {
    map(hour, |hour| HTime { hour })(i)
}

#[inline]
fn time_naive_approx(i: &[u8]) -> ParseResult<ApproxNaiveTime> {
    alt((
        complete(map(time_hms, ApproxNaiveTime::HMS)),
        complete(map(time_hm, ApproxNaiveTime::HM)),
        complete(map(time_h, ApproxNaiveTime::H)),
    ))(i)
}

#[inline]
pub fn time_local_approx(i: &[u8]) -> ParseResult<ApproxLocalTime> {
    map(
        pair(time_naive_approx, opt(complete(frac32))),
        |(naive, fraction)| match naive {
            ApproxNaiveTime::HMS(naive) => ApproxLocalTime::HMS(LocalTime {
                naive,
                fraction: fraction.unwrap_or(0.),
            }),
            ApproxNaiveTime::HM(naive) => ApproxLocalTime::HM(LocalTime {
                naive,
                fraction: fraction.unwrap_or(0.),
            }),
            ApproxNaiveTime::H(naive) => ApproxLocalTime::H(LocalTime {
                naive,
                fraction: fraction.unwrap_or(0.),
            }),
        },
    )(i)
}

#[inline]
pub fn time_global_approx(i: &[u8]) -> ParseResult<ApproxGlobalTime> {
    map(
        pair(time_local_approx, timezone),
        |(local, timezone)| match local {
            ApproxLocalTime::HMS(local) => ApproxGlobalTime::HMS(GlobalTime { local, timezone }),
            ApproxLocalTime::HM(local) => ApproxGlobalTime::HM(GlobalTime { local, timezone }),
            ApproxLocalTime::H(local) => ApproxGlobalTime::H(GlobalTime { local, timezone }),
        },
    )(i)
}

#[inline]
pub fn time_any_approx(i: &[u8]) -> ParseResult<ApproxAnyTime> {
    alt((
        map(time_any_hms, ApproxAnyTime::HMS),
        map(time_any_hm, ApproxAnyTime::HM),
        map(time_any_h, ApproxAnyTime::H),
    ))(i)
}

macro_rules! time_local_accuracy {
    (pub $name:ident, $naive:ty, $naive_submac:ident) => {
        #[inline]
        pub fn $name(i: &[u8]) -> ParseResult<LocalTime<$naive>> {
            map(
                tuple((opt(char('T')), $naive_submac, opt(complete(frac32)))),
                |(_, naive, fraction)| LocalTime {
                    naive,
                    fraction: fraction.unwrap_or(0.),
                },
            )(i)
        }
    };
}

time_local_accuracy!(pub time_local_hms, HmsTime, time_hms);
time_local_accuracy!(pub time_local_hm,  HmTime,  time_hm);
time_local_accuracy!(pub time_local_h,   HTime,   time_h);

macro_rules! time_global_accuracy {
    (pub $name:ident, $naive:ty, $local_submac:ident) => {
        #[inline]
        pub fn $name(i: &[u8]) -> ParseResult<GlobalTime<$naive>> {
            map(
                pair($local_submac, complete(timezone)),
                |(local, timezone)| GlobalTime { local, timezone },
            )(i)
        }
    };
}
time_global_accuracy!(pub time_global_hms, HmsTime, time_local_hms);
time_global_accuracy!(pub time_global_hm,  HmTime,  time_local_hm);
time_global_accuracy!(pub time_global_h,   HTime,   time_local_h);

macro_rules! time_any_accuracy {
    (pub $name:ident, $naive:ty, $local_submac:ident, $global_submac:ident) => {
        #[inline]
        pub fn $name(i: &[u8]) -> ParseResult<AnyTime<$naive>> {
            alt((
                complete(map($global_submac, AnyTime::Global)),
                complete(map($local_submac, AnyTime::Local)),
            ))(i)
        }
    };
}
time_any_accuracy!(pub time_any_hms, HmsTime, time_local_hms, time_global_hms);
time_any_accuracy!(pub time_any_hm,  HmTime,  time_local_hm,  time_global_hm);
time_any_accuracy!(pub time_any_h,   HTime,   time_local_h,   time_global_h);

#[inline]
fn timezone_utc(i: &[u8]) -> ParseResult<i16> {
    map(char('Z'), |_| 0)(i)
}

#[inline]
fn timezone_fixed(i: &[u8]) -> ParseResult<i16> {
    map(
        tuple((sign, hour, opt(complete(preceded(opt(char(':')), minute))))),
        |(sign, hour, minute)| sign as i16 * (hour as i16 * 60 + minute.unwrap_or(0) as i16),
    )(i)
}

#[inline]
fn timezone(i: &[u8]) -> ParseResult<i16> {
    alt((timezone_utc, timezone_fixed))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{Error, ErrorKind::Char},
        Err,
    };

    #[test]
    fn hour() {
        assert_eq!(super::hour(b"02"), Ok((&[][..], 2)));
        assert_eq!(super::hour(b"24"), Ok((&[][..], 24)));
    }

    #[test]
    fn minute() {
        assert_eq!(super::minute(b"02"), Ok((&[][..], 2)));
        assert_eq!(super::minute(b"59"), Ok((&[][..], 59)));
    }

    #[test]
    fn second() {
        assert_eq!(super::second(b"02 "), Ok((&b" "[..], 2)));
        assert_eq!(super::second(b"02"), Ok((&[][..], 2)));
        assert_eq!(super::second(b"60 "), Ok((&b" "[..], 60)));
        assert_eq!(super::second(b"60"), Ok((&[][..], 60)));
    }

    #[test]
    fn timezone_fixed() {
        assert_eq!(
            super::timezone_fixed(b"+23:59 "),
            Ok((&b" "[..], 23 * 60 + 59))
        );
        assert_eq!(
            super::timezone_fixed(b"+23:59"),
            Ok((&[][..], 23 * 60 + 59))
        );
        assert_eq!(
            super::timezone_fixed(b"+2359 "),
            Ok((&b" "[..], 23 * 60 + 59))
        );
        assert_eq!(super::timezone_fixed(b"+2359"), Ok((&[][..], 23 * 60 + 59)));
        assert_eq!(super::timezone_fixed(b"-23 "), Ok((&b" "[..], -23 * 60)));
        assert_eq!(super::timezone_fixed(b"-23"), Ok((&[][..], -23 * 60)));
    }

    #[test]
    fn timezone_utc() {
        assert_eq!(super::timezone_utc(b"Z "), Ok((&b" "[..], 0)));
        assert_eq!(super::timezone_utc(b"Z"), Ok((&[][..], 0)));
        assert_eq!(
            super::timezone_utc(b"z"),
            Err(Err::Error(Error {
                input: &b"z"[..],
                code: Char
            }))
        );
    }

    #[test]
    fn timezone() {
        assert_eq!(super::timezone(b"-22:11 "), Ok((&b" "[..], -22 * 60 - 11)));
        assert_eq!(super::timezone(b"-22:11"), Ok((&[][..], -22 * 60 - 11)));
        assert_eq!(super::timezone(b"-2211 "), Ok((&b" "[..], -22 * 60 - 11)));
        assert_eq!(super::timezone(b"-2211"), Ok((&[][..], -22 * 60 - 11)));
        assert_eq!(super::timezone(b"Z "), Ok((&b" "[..], 0)));
        assert_eq!(super::timezone(b"Z"), Ok((&[][..], 0)));
    }

    #[test]
    fn time_hms() {
        let value = HmsTime {
            hour: 11,
            minute: 22,
            second: 33,
        };
        assert_eq!(
            super::time_hms(b"11:22:33 "),
            Ok((&b" "[..], value.clone()))
        );
        assert_eq!(super::time_hms(b"11:22:33"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_hms(b"112233 "), Ok((&b" "[..], value.clone())));
        assert_eq!(super::time_hms(b"112233"), Ok((&[][..], value)));
    }

    #[test]
    fn time_hm() {
        let value = HmTime {
            hour: 11,
            minute: 22,
        };
        assert_eq!(super::time_hm(b"11:22 "), Ok((&b" "[..], value.clone())));
        assert_eq!(super::time_hm(b"11:22"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_hm(b"1122 "), Ok((&b" "[..], value.clone())));
        assert_eq!(super::time_hm(b"1122"), Ok((&[][..], value)));
    }

    #[test]
    fn time_h() {
        let value = HTime { hour: 11 };
        assert_eq!(super::time_h(b"11 "), Ok((&b" "[..], value.clone())));
        assert_eq!(super::time_h(b"11"), Ok((&[][..], value)));
    }

    #[test]
    fn time_local_hms() {
        let value = LocalTime {
            naive: HmsTime {
                hour: 16,
                minute: 43,
                second: 52,
            },
            fraction: 0.1,
        };
        assert_eq!(
            super::time_local_hms(b"T16:43:52.1 "),
            Ok((&b" "[..], value.clone()))
        );
        assert_eq!(
            super::time_local_hms(b"T16:43:52.1"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_local_hms(b"16:43:52.1"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_local_hms(b"T164352.1"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_local_hms(b"164352.1"),
            Ok((&[][..], value.clone()))
        );

        let value = LocalTime {
            fraction: 0.,
            ..value
        };
        assert_eq!(
            super::time_local_hms(b"T16:43:52"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_local_hms(b"16:43:52"), Ok((&[][..], value)));
    }

    #[test]
    fn time_local_hm() {
        let value = LocalTime {
            naive: HmTime {
                hour: 16,
                minute: 43,
            },
            fraction: 0.1,
        };
        assert_eq!(
            super::time_local_hm(b"T16:43.1"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_local_hm(b"16:43.1"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_local_hm(b"T1643.1"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_local_hm(b"1643.1"),
            Ok((&[][..], value.clone()))
        );

        let value = LocalTime {
            fraction: 0.,
            ..value
        };
        assert_eq!(
            super::time_local_hm(b"T16:43"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_local_hm(b"16:43"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_local_hm(b"T1643"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_local_hm(b"1643"), Ok((&[][..], value)));
    }

    #[test]
    fn time_local_h() {
        let value = LocalTime {
            naive: HTime { hour: 16 },
            fraction: 0.1,
        };
        assert_eq!(super::time_local_h(b"T16.1"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_local_h(b"16.1"), Ok((&[][..], value.clone())));

        let value = LocalTime {
            fraction: 0.,
            ..value
        };
        assert_eq!(super::time_local_h(b"T16"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_local_h(b"16"), Ok((&[][..], value)));
    }

    #[test]
    fn time_global_hms() {
        let value = GlobalTime {
            local: LocalTime {
                naive: HmsTime {
                    hour: 16,
                    minute: 43,
                    second: 52,
                },
                fraction: 0.,
            },
            timezone: 0,
        };
        assert_eq!(
            super::time_global_hms(b"T16:43:52Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hms(b"16:43:52Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hms(b"T164352Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hms(b"164352Z"),
            Ok((&[][..], value.clone()))
        );

        {
            let value = GlobalTime {
                timezone: 2,
                ..value.clone()
            };
            assert_eq!(
                super::time_global_hms(b"T16:43:52+0002"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::time_global_hms(b"16:43:52+0002"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::time_global_hms(b"T164352+0002"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::time_global_hms(b"164352+0002"),
                Ok((&[][..], value.clone()))
            );

            let value = GlobalTime {
                local: LocalTime {
                    fraction: 0.1,
                    ..value.local
                },
                ..value
            };
            assert_eq!(
                super::time_global_hms(b"T16:43:52.1+0002"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::time_global_hms(b"16:43:52.1+0002"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::time_global_hms(b"T164352.1+0002"),
                Ok((&[][..], value.clone()))
            );
            assert_eq!(
                super::time_global_hms(b"164352.1+0002"),
                Ok((&[][..], value))
            );
        }

        let value = GlobalTime {
            local: LocalTime {
                fraction: 0.1,
                ..value.local
            },
            ..value
        };
        assert_eq!(
            super::time_global_hms(b"T16:43:52.1Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hms(b"16:43:52.1Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hms(b"T164352.1Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_global_hms(b"164352.1Z"), Ok((&[][..], value)));
    }

    #[test]
    fn time_global_hm() {
        let value = GlobalTime {
            local: LocalTime {
                naive: HmTime {
                    hour: 16,
                    minute: 43,
                },
                fraction: 0.,
            },
            timezone: 0,
        };
        assert_eq!(
            super::time_global_hm(b"T16:43Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hm(b"16:43Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hm(b"T1643Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hm(b"1643Z"),
            Ok((&[][..], value.clone()))
        );

        let value = GlobalTime {
            local: LocalTime {
                fraction: 0.1,
                ..value.local
            },
            ..value
        };
        assert_eq!(
            super::time_global_hm(b"T16:43.1Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hm(b"16:43.1Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_global_hm(b"T1643.1Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_global_hm(b"1643.1Z"), Ok((&[][..], value)));
    }

    #[test]
    fn time_global_h() {
        let value = GlobalTime {
            local: LocalTime {
                naive: HTime { hour: 16 },
                fraction: 0.,
            },
            timezone: 0,
        };
        assert_eq!(super::time_global_h(b"T16Z"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_global_h(b"16Z"), Ok((&[][..], value.clone())));

        let value = GlobalTime {
            local: LocalTime {
                fraction: 0.1,
                ..value.local
            },
            ..value
        };
        assert_eq!(
            super::time_global_h(b"T16.1Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_global_h(b"16.1Z"), Ok((&[][..], value)));
    }

    #[test]
    fn time_any_hms() {
        let value = AnyTime::Local(LocalTime {
            naive: HmsTime {
                hour: 16,
                minute: 43,
                second: 52,
            },
            fraction: 0.,
        });
        assert_eq!(
            super::time_any_hms(b"T16:43:52"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hms(b"16:43:52"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hms(b"T164352"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_any_hms(b"164352"), Ok((&[][..], value)));

        let value = AnyTime::Global(GlobalTime {
            local: LocalTime {
                naive: HmsTime {
                    hour: 2,
                    minute: 3,
                    second: 52,
                },
                fraction: 0.,
            },
            timezone: 0,
        });
        assert_eq!(
            super::time_any_hms(b"T02:03:52Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hms(b"02:03:52Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hms(b"T020352Z"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_any_hms(b"020352Z"), Ok((&[][..], value)));

        let value = AnyTime::Global(GlobalTime {
            local: LocalTime {
                naive: HmsTime {
                    hour: 2,
                    minute: 3,
                    second: 52,
                },
                fraction: 0.,
            },
            timezone: -1 * 60,
        });
        assert_eq!(
            super::time_any_hms(b"T02:03:52-01"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hms(b"02:03:52-01"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hms(b"T020352-01"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_any_hms(b"020352-01"), Ok((&[][..], value)));
    }

    #[test]
    fn time_any_hm() {
        let value = AnyTime::Local(LocalTime {
            naive: HmTime {
                hour: 16,
                minute: 43,
            },
            fraction: 0.,
        });
        assert_eq!(super::time_any_hm(b"T16:43"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_hm(b"16:43"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_hm(b"T1643"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_hm(b"1643"), Ok((&[][..], value)));

        let value = AnyTime::Global(GlobalTime {
            local: LocalTime {
                naive: HmTime { hour: 2, minute: 3 },
                fraction: 0.,
            },
            timezone: 0,
        });
        assert_eq!(super::time_any_hm(b"T02:03Z"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_hm(b"02:03Z"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_hm(b"T0203Z"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_hm(b"0203Z"), Ok((&[][..], value)));

        let value = AnyTime::Global(GlobalTime {
            local: LocalTime {
                naive: HmTime { hour: 2, minute: 3 },
                fraction: 0.,
            },
            timezone: -1 * 60,
        });
        assert_eq!(
            super::time_any_hm(b"T02:03-01"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hm(b"02:03-01"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(
            super::time_any_hm(b"T0203-01"),
            Ok((&[][..], value.clone()))
        );
        assert_eq!(super::time_any_hm(b"0203-01"), Ok((&[][..], value)));
    }

    #[test]
    fn time_any_h() {
        let value = AnyTime::Local(LocalTime {
            naive: HTime { hour: 16 },
            fraction: 0.,
        });
        assert_eq!(super::time_any_h(b"T16"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_h(b"16"), Ok((&[][..], value)));

        let value = AnyTime::Global(GlobalTime {
            local: LocalTime {
                naive: HTime { hour: 2 },
                fraction: 0.,
            },
            timezone: 0,
        });
        assert_eq!(super::time_any_h(b"T02Z"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_h(b"02Z"), Ok((&[][..], value)));

        let value = AnyTime::Global(GlobalTime {
            local: LocalTime {
                naive: HTime { hour: 2 },
                fraction: 0.,
            },
            timezone: -1 * 60,
        });
        assert_eq!(super::time_any_h(b"T02-01"), Ok((&[][..], value.clone())));
        assert_eq!(super::time_any_h(b"02-01"), Ok((&[][..], value)));
    }

    #[test]
    fn time_local_approx() {
        assert_eq!(
            super::time_local_approx(b"16:22:48"),
            Ok((
                &[][..],
                ApproxLocalTime::HMS(LocalTime {
                    naive: HmsTime {
                        hour: 16,
                        minute: 22,
                        second: 48
                    },
                    fraction: 0.
                })
            ))
        );

        assert_eq!(
            super::time_local_approx(b"16:22"),
            Ok((
                &[][..],
                ApproxLocalTime::HM(LocalTime {
                    naive: HmTime {
                        hour: 16,
                        minute: 22
                    },
                    fraction: 0.
                })
            ))
        );

        assert_eq!(
            super::time_local_approx(b"16"),
            Ok((
                &[][..],
                ApproxLocalTime::H(LocalTime {
                    naive: HTime { hour: 16 },
                    fraction: 0.
                })
            ))
        );
    }

    #[test]
    fn time_global_approx() {
        assert_eq!(
            super::time_global_approx(b"16:22:48Z"),
            Ok((
                &[][..],
                ApproxGlobalTime::HMS(GlobalTime {
                    local: LocalTime {
                        naive: HmsTime {
                            hour: 16,
                            minute: 22,
                            second: 48
                        },
                        fraction: 0.
                    },
                    timezone: 0
                })
            ))
        );

        assert_eq!(
            super::time_global_approx(b"16:22Z"),
            Ok((
                &[][..],
                ApproxGlobalTime::HM(GlobalTime {
                    local: LocalTime {
                        naive: HmTime {
                            hour: 16,
                            minute: 22
                        },
                        fraction: 0.
                    },
                    timezone: 0
                })
            ))
        );

        assert_eq!(
            super::time_global_approx(b"16Z"),
            Ok((
                &[][..],
                ApproxGlobalTime::H(GlobalTime {
                    local: LocalTime {
                        naive: HTime { hour: 16 },
                        fraction: 0.
                    },
                    timezone: 0
                })
            ))
        );
    }

    #[test]
    fn time_any_approx() {
        assert_eq!(
            super::time_any_approx(b"16:22:48"),
            Ok((
                &[][..],
                ApproxAnyTime::HMS(AnyTime::Local(LocalTime {
                    naive: HmsTime {
                        hour: 16,
                        minute: 22,
                        second: 48
                    },
                    fraction: 0.
                }))
            ))
        );
        assert_eq!(
            super::time_any_approx(b"16:22"),
            Ok((
                &[][..],
                ApproxAnyTime::HM(AnyTime::Local(LocalTime {
                    naive: HmTime {
                        hour: 16,
                        minute: 22
                    },
                    fraction: 0.
                }))
            ))
        );
        assert_eq!(
            super::time_any_approx(b"16"),
            Ok((
                &[][..],
                ApproxAnyTime::H(AnyTime::Local(LocalTime {
                    naive: HTime { hour: 16 },
                    fraction: 0.
                }))
            ))
        );

        assert_eq!(
            super::time_any_approx(b"16:22:48Z"),
            Ok((
                &[][..],
                ApproxAnyTime::HMS(AnyTime::Global(GlobalTime {
                    local: LocalTime {
                        naive: HmsTime {
                            hour: 16,
                            minute: 22,
                            second: 48
                        },
                        fraction: 0.
                    },
                    timezone: 0
                }))
            ))
        );
        assert_eq!(
            super::time_any_approx(b"16:22Z"),
            Ok((
                &[][..],
                ApproxAnyTime::HM(AnyTime::Global(GlobalTime {
                    local: LocalTime {
                        naive: HmTime {
                            hour: 16,
                            minute: 22
                        },
                        fraction: 0.
                    },
                    timezone: 0
                }))
            ))
        );
        assert_eq!(
            super::time_any_approx(b"16Z"),
            Ok((
                &[][..],
                ApproxAnyTime::H(AnyTime::Global(GlobalTime {
                    local: LocalTime {
                        naive: HTime { hour: 16 },
                        fraction: 0.
                    },
                    timezone: 0
                }))
            ))
        );
    }
}
