use super::*;
use crate::time::*;
use nom::character::is_digit;

named!(hour<u8>, map!(take_while_m_n!(2, 2, is_digit), buf_to_int));

named!(
    minute<u8>,
    map!(take_while_m_n!(2, 2, is_digit), buf_to_int)
);

named!(
    second<u8>,
    map!(take_while_m_n!(2, 2, is_digit), buf_to_int)
);

named_args!(time_hms_format(extended: bool) <HmsTime>, do_parse!(
    hour: hour >>
    cond!(extended, char!(':')) >>
    minute: minute >>
    cond!(extended, char!(':')) >>
    second: second >>
    (HmsTime { hour, minute, second })
));
named!(time_hms_basic<HmsTime>, call!(time_hms_format, false));
named!(time_hms_extended<HmsTime>, call!(time_hms_format, true));

named!(pub time_hms <HmsTime>, alt!(
    time_hms_extended |
    time_hms_basic
));

named_args!(time_hm_format(extended: bool) <HmTime>, do_parse!(
    hour: hour >>
    cond!(extended, char!(':')) >>
    minute: minute >>
    (HmTime { hour, minute })
));
named!(time_hm_basic<HmTime>, call!(time_hm_format, false));
named!(time_hm_extended<HmTime>, call!(time_hm_format, true));

named!(pub time_hm <HmTime>, alt!(
    time_hm_extended |
    time_hm_basic
));

named!(pub time_h <HTime>, map!(hour, |hour| HTime { hour }));

named!(
    time_naive_approx<ApproxNaiveTime>,
    alt!(
        complete!(map!(time_hms, ApproxNaiveTime::HMS))
            | complete!(map!(time_hm, ApproxNaiveTime::HM))
            | complete!(map!(time_h, ApproxNaiveTime::H))
    )
);

named!(pub time_local_approx <ApproxLocalTime>, do_parse!(
    naive: time_naive_approx >>
    fraction: opt!(complete!(frac32)) >>
    (match naive {
        ApproxNaiveTime::HMS(naive) => ApproxLocalTime::HMS(LocalTime {
            naive,
            fraction: fraction.unwrap_or(0.)
        }),
        ApproxNaiveTime::HM(naive) => ApproxLocalTime::HM(LocalTime {
            naive,
            fraction: fraction.unwrap_or(0.)
        }),
        ApproxNaiveTime::H(naive) => ApproxLocalTime::H(LocalTime {
            naive,
            fraction: fraction.unwrap_or(0.)
        })
    })
));

named!(pub time_global_approx <ApproxGlobalTime>, do_parse!(
    local: time_local_approx >>
    timezone: timezone >>
    (match local {
        ApproxLocalTime::HMS(local) => ApproxGlobalTime::HMS(GlobalTime { local, timezone }),
        ApproxLocalTime::HM (local) => ApproxGlobalTime::HM (GlobalTime { local, timezone }),
        ApproxLocalTime::H  (local) => ApproxGlobalTime::H  (GlobalTime { local, timezone })
    })
));

named!(pub time_any_approx <ApproxAnyTime>, alt!(
    map!(time_any_hms, ApproxAnyTime::HMS) |
    map!(time_any_hm,  ApproxAnyTime::HM) |
    map!(time_any_h,   ApproxAnyTime::H)
));

macro_rules! time_local_accuracy {
    (pub $name:ident, $naive:ty, $naive_submac:ident) => {
        named!(pub $name <LocalTime<$naive>>, do_parse!(
            opt!(char!('T')) >>
            naive: $naive_submac >>
            fraction: opt!(complete!(frac32)) >>
            (LocalTime {
                naive,
                fraction: fraction.unwrap_or(0.)
            })
        ));
    }
}
time_local_accuracy!(pub time_local_hms, HmsTime, time_hms);
time_local_accuracy!(pub time_local_hm,  HmTime,  time_hm);
time_local_accuracy!(pub time_local_h,   HTime,   time_h);

macro_rules! time_global_accuracy {
    (pub $name:ident, $naive:ty, $local_submac:ident) => {
        named!(pub $name <GlobalTime<$naive>>, do_parse!(
            local: $local_submac >>
            timezone: complete!(timezone) >>
            (GlobalTime { local, timezone })
        ));
    }
}
time_global_accuracy!(pub time_global_hms, HmsTime, time_local_hms);
time_global_accuracy!(pub time_global_hm,  HmTime,  time_local_hm);
time_global_accuracy!(pub time_global_h,   HTime,   time_local_h);

macro_rules! time_any_accuracy {
    (pub $name:ident, $naive:ty, $local_submac:ident, $global_submac:ident) => {
        named!(pub $name <AnyTime<$naive>>, alt!(
            complete!(map!($global_submac, AnyTime::Global)) |
            complete!(map!($local_submac, AnyTime::Local))
        ));
    }
}
time_any_accuracy!(pub time_any_hms, HmsTime, time_local_hms, time_global_hms);
time_any_accuracy!(pub time_any_hm,  HmTime,  time_local_hm,  time_global_hm);
time_any_accuracy!(pub time_any_h,   HTime,   time_local_h,   time_global_h);

named!(timezone_utc<i16>, map!(char!('Z'), |_| 0));

named!(
    timezone_fixed<i16>,
    do_parse!(
        sign: sign
            >> hour: hour
            >> minute:
                opt!(complete!(do_parse!(
                    opt!(char!(':')) >> minute: minute >> (minute)
                )))
            >> (sign as i16 * (hour as i16 * 60 + minute.unwrap_or(0) as i16))
    )
);

named!(timezone<i16>, alt!(timezone_utc | timezone_fixed));

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
