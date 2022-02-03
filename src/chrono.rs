#![cfg(feature = "chrono")]
use chrono::prelude::*;

impl From<crate::DateTime<crate::Date, crate::GlobalTime>> for DateTime<FixedOffset> {
    fn from(dt: crate::DateTime<crate::Date, crate::GlobalTime>) -> Self {
        let date: crate::YmdDate = dt.date.into();

        FixedOffset::east((dt.time.timezone * 60).into())
            .ymd(date.year.into(), date.month.into(), date.day.into())
            .and_hms_nano(
                dt.time.local.naive.hour.into(),
                dt.time.local.naive.minute.into(),
                dt.time.local.naive.second.into(),
                dt.time.local.nanosecond(),
            )
    }
}

impl From<crate::DateTime<crate::Date, crate::GlobalTime>> for DateTime<Utc> {
    fn from(dt: crate::DateTime<crate::Date, crate::GlobalTime>) -> Self {
        DateTime::<FixedOffset>::from(dt).with_timezone(&Utc)
    }
}

impl From<crate::DateTime<crate::Date, crate::GlobalTime>> for DateTime<Local> {
    fn from(dt: crate::DateTime<crate::Date, crate::GlobalTime>) -> Self {
        DateTime::<FixedOffset>::from(dt).with_timezone(&Local)
    }
}

impl From<crate::DateTime<crate::Date, crate::LocalTime>> for DateTime<FixedOffset> {
    fn from(dt: crate::DateTime<crate::Date, crate::LocalTime>) -> Self {
        DateTime::<Local>::from(dt).with_timezone(&Utc.fix())
    }
}

impl From<crate::DateTime<crate::Date, crate::LocalTime>> for DateTime<Utc> {
    fn from(dt: crate::DateTime<crate::Date, crate::LocalTime>) -> Self {
        DateTime::<Local>::from(dt).with_timezone(&Utc)
    }
}

impl From<crate::DateTime<crate::Date, crate::LocalTime>> for DateTime<Local> {
    fn from(dt: crate::DateTime<crate::Date, crate::LocalTime>) -> Self {
        let date: crate::YmdDate = dt.date.into();

        Local
            .from_local_datetime(
                &NaiveDate::from_ymd(date.year.into(), date.month.into(), date.day.into())
                    .and_hms_nano(
                        dt.time.naive.hour.into(),
                        dt.time.naive.minute.into(),
                        dt.time.naive.second.into(),
                        dt.time.nanosecond(),
                    ),
            )
            .single()
            .unwrap()
    }
}

impl From<crate::DateTime<crate::Date, crate::AnyTime>> for DateTime<FixedOffset> {
    fn from(dt: crate::DateTime<crate::Date, crate::AnyTime>) -> Self {
        DateTime::<Local>::from(dt).with_timezone(&Utc.fix())
    }
}

impl From<crate::DateTime<crate::Date, crate::AnyTime>> for DateTime<Utc> {
    fn from(dt: crate::DateTime<crate::Date, crate::AnyTime>) -> Self {
        DateTime::<Local>::from(dt).with_timezone(&Utc)
    }
}

impl From<crate::DateTime<crate::Date, crate::AnyTime>> for DateTime<Local> {
    fn from(dt: crate::DateTime<crate::Date, crate::AnyTime>) -> Self {
        match dt.time {
            crate::AnyTime::Global(time) => crate::DateTime {
                date: dt.date,
                time,
            }
            .into(),
            crate::AnyTime::Local(time) => crate::DateTime {
                date: dt.date,
                time,
            }
            .into(),
        }
    }
}

#[cfg(feature = "chrono-serde")]
pub mod serde {
    use super::{DateTime, TimeZone};
    use serde::{Deserialize, Deserializer};

    #[allow(non_snake_case)]
    pub fn deserialize_DateTime<'de, D, Tz>(de: D) -> Result<DateTime<Tz>, D::Error>
    where
        D: Deserializer<'de>,
        Tz: TimeZone,
        DateTime<Tz>: From<crate::DateTime<crate::ApproxDate, crate::ApproxAnyTime>>,
    {
        Ok(
            crate::parse::datetime_approx_any_approx(String::deserialize(de)?.as_bytes())
                .map_err(serde::de::Error::custom)?
                .1
                .into(),
        )
    }
}
