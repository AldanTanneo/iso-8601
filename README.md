## Fork from https://hg.sr.ht/~dermetfan/iso-8601

# Fully Compliant ISO 8601 Parser

The go-to library for all your date and time parsing needs.

Any unimplemented notation the standard supports is considered a bug.

Chrono support is included.

## Roadmap

Version 1.0.0 will be reached when the entire standard is implemented.

- [x] calendar dates
- [x] week dates
- [x] ordinal dates
- [x] approximate dates
- [x] local time
- [x] global time / time zone offsets
- [x] time decimal fractions
- [x] approximate time
- [x] datetime
- [ ] durations
- [ ] intervals
- [ ] recurring intervals

Chrono support is very limited at the moment.
Contributions would very welcome, should be a low-hanging fruit.

## Examples

Basics:

```rust
use std::str::FromStr;
use iso_8601::*;

assert_eq!(
    Date::from_str("2018-08-02").unwrap(),
    Date::YMD(YmdDate {
        year: 2018,
        month: 8,
        day: 2,
    })
);

assert_eq!(
    LocalTime::from_str("13:42").unwrap(),
    LocalTime {
        naive: HmTime {
            hour: 13,
            minute: 42,
        },
        fraction: 0.,
    }
);

// Parse any date, time, or datetime expression without knowing what it is:

assert_eq!(
    PartialDateTime::from_str("12:30").unwrap(),
    PartialDateTime::Time(ApproxAnyTime::HM(AnyTime::Local(LocalTime {
        naive: HmTime {
            hour: 12,
            minute: 30,
        },
        fraction: 0.,
    })))
);

assert_eq!(
    PartialDateTime::from_str("2018-08").unwrap(),
    PartialDateTime::Date(ApproxDate::YM(YmDate {
        year: 2018,
        month: 8,
    }))
);

assert_eq!(
    PartialDateTime::from_str("20180802T1230").unwrap(),
    PartialDateTime::DateTime(DateTime {
        date: ApproxDate::YMD(YmdDate {
            year: 2018,
            month: 8,
            day: 2,
        }),
        time: ApproxAnyTime::HM(AnyTime::Local(LocalTime {
            naive: HmTime {
                hour: 12,
                minute: 30,
            },
            fraction: 0.,
        }))
    })
);
```

Chrono support:

```rust
use std::str::FromStr;
use iso_8601::{DateTime, Date, GlobalTime};

fn main() {
    #[cfg(feature = "chrono")]
    assert_eq!(
        chrono::DateTime::<chrono::Utc>::from(
            DateTime::<Date, GlobalTime>::from_str("2018-08-02T13:42:02Z").unwrap()
        ),
        chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd(2018, 8, 2),
                chrono::NaiveTime::from_hms(13, 42, 2)
            ),
            chrono::Utc
        )
    );
}
```
