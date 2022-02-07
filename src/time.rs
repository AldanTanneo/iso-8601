use crate::Valid;

/// Local time (4.2.2.2)
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HmsTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// A specific hour and minute (4.2.2.3a)
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HmTime {
    pub hour: u8,
    pub minute: u8,
}

/// A specific hour (4.2.2.3b)
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HTime {
    pub hour: u8,
}

impl From<HTime> for HmTime {
    #[inline]
    fn from(HTime { hour }: HTime) -> Self {
        Self { hour, minute: 0 }
    }
}

impl From<HmTime> for HmsTime {
    #[inline]
    fn from(HmTime { hour, minute }: HmTime) -> Self {
        Self {
            hour,
            minute,
            second: 0,
        }
    }
}

/// Local time with decimal fraction (4.2.2.4)
#[derive(PartialEq, Clone, Debug)]
pub struct LocalTime<N = HmsTime>
where
    N: NaiveTime,
{
    pub naive: N,
    pub fraction: f32,
}

impl<N: NaiveTime + Copy> Copy for LocalTime<N> {}

/// Local time with timezone (4.2.4)
#[derive(PartialEq, Clone, Debug)]
pub struct GlobalTime<N = HmsTime>
where
    N: NaiveTime,
{
    pub local: LocalTime<N>,
    /// Difference from UTC in minutes (4.2.5.2)
    pub timezone: i16,
}

impl<N: NaiveTime + Copy> Copy for GlobalTime<N> {}

#[derive(PartialEq, Clone, Debug)]
pub enum AnyTime<N = HmsTime>
where
    N: NaiveTime,
{
    Global(GlobalTime<N>),
    Local(LocalTime<N>),
}

impl<N: NaiveTime + Copy> Copy for AnyTime<N> {}

pub trait NaiveTime {}

impl NaiveTime for HmsTime {}
impl NaiveTime for HmTime {}
impl NaiveTime for HTime {}

impl LocalTime<HmsTime> {
    #[inline]
    pub fn nanosecond(&self) -> u32 {
        (self.fraction * 1_000_000_000.) as u32
    }
}

impl LocalTime<HmTime> {
    #[inline]
    pub fn second(&self) -> u8 {
        (self.fraction * 60.) as u8
    }

    #[inline]
    pub fn nanosecond(&self) -> u32 {
        (self.fraction * 60_000_000_000.) as u32 % 1_000_000_000
    }
}

impl LocalTime<HTime> {
    #[inline]
    pub fn minute(&self) -> u8 {
        (self.fraction * 60.) as u8
    }

    #[inline]
    pub fn second(&self) -> u8 {
        (self.fraction * 3_600.) as u8 % 60
    }

    #[inline]
    pub fn nanosecond(&self) -> u32 {
        (self.fraction * 3_600_000_000_000.) as u32 % 1_000_000_000
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ApproxNaiveTime {
    HMS(HmsTime),
    HM(HmTime),
    H(HTime),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ApproxLocalTime {
    HMS(LocalTime<HmsTime>),
    HM(LocalTime<HmTime>),
    H(LocalTime<HTime>),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ApproxGlobalTime {
    HMS(GlobalTime<HmsTime>),
    HM(GlobalTime<HmTime>),
    H(GlobalTime<HTime>),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ApproxAnyTime {
    HMS(AnyTime<HmsTime>),
    HM(AnyTime<HmTime>),
    H(AnyTime<HTime>),
}

pub trait Timelike {}

impl<N: NaiveTime> Timelike for N {}
impl<N: NaiveTime> Timelike for LocalTime<N> {}
impl<N: NaiveTime> Timelike for GlobalTime<N> {}
impl<N: NaiveTime> Timelike for AnyTime<N> {}
impl Timelike for ApproxLocalTime {}
impl Timelike for ApproxGlobalTime {}
impl Timelike for ApproxAnyTime {}

impl_fromstr_parse!(GlobalTime<HmsTime>, time_global_hms);
impl_fromstr_parse!(GlobalTime<HmTime>, time_global_hm);
impl_fromstr_parse!(GlobalTime<HTime>, time_global_h);
impl_fromstr_parse!(LocalTime<HmsTime>, time_local_hms);
impl_fromstr_parse!(LocalTime<HmTime>, time_local_hm);
impl_fromstr_parse!(LocalTime<HTime>, time_local_h);
impl_fromstr_parse!(AnyTime<HmsTime>, time_any_hms);
impl_fromstr_parse!(AnyTime<HmTime>, time_any_hm);
impl_fromstr_parse!(AnyTime<HTime>, time_any_h);
impl_fromstr_parse!(ApproxGlobalTime, time_global_approx);
impl_fromstr_parse!(ApproxLocalTime, time_local_approx);
impl_fromstr_parse!(ApproxAnyTime, time_any_approx);

impl Valid for HmsTime {
    /// Accepts leap seconds on any day
    /// since they are not predictable.
    #[inline]
    fn is_valid(&self) -> bool {
        HmTime::from(*self).is_valid() && self.second <= 60
    }
}

impl Valid for HmTime {
    #[inline]
    fn is_valid(&self) -> bool {
        HTime::from(*self).is_valid() && self.minute <= 59
    }
}

impl Valid for HTime {
    #[inline]
    fn is_valid(&self) -> bool {
        self.hour <= 24
    }
}

impl<N> Valid for LocalTime<N>
where
    N: NaiveTime + Valid,
{
    #[inline]
    fn is_valid(&self) -> bool {
        self.naive.is_valid() && self.fraction >= 0. && self.fraction < 1.
    }
}

impl<N> Valid for GlobalTime<N>
where
    N: NaiveTime + Valid,
{
    #[inline]
    fn is_valid(&self) -> bool {
        self.local.is_valid() && self.timezone > -24 * 60 && self.timezone < 24 * 60
    }
}

impl<N> Valid for AnyTime<N>
where
    N: NaiveTime + Valid,
{
    #[inline]
    fn is_valid(&self) -> bool {
        match self {
            Self::Global(time) => time.is_valid(),
            Self::Local(time) => time.is_valid(),
        }
    }
}

impl Valid for ApproxLocalTime {
    #[inline]
    fn is_valid(&self) -> bool {
        match self {
            Self::HMS(time) => time.is_valid(),
            Self::HM(time) => time.is_valid(),
            Self::H(time) => time.is_valid(),
        }
    }
}

impl Valid for ApproxGlobalTime {
    #[inline]
    fn is_valid(&self) -> bool {
        match self {
            Self::HMS(time) => time.is_valid(),
            Self::HM(time) => time.is_valid(),
            Self::H(time) => time.is_valid(),
        }
    }
}

impl Valid for ApproxAnyTime {
    #[inline]
    fn is_valid(&self) -> bool {
        match self {
            Self::HMS(time) => time.is_valid(),
            Self::HM(time) => time.is_valid(),
            Self::H(time) => time.is_valid(),
        }
    }
}

impl From<HmsTime> for HmTime {
    #[inline]
    fn from(t: HmsTime) -> Self {
        Self {
            hour: t.hour,
            minute: t.minute,
        }
    }
}

impl From<HmsTime> for HTime {
    #[inline]
    fn from(t: HmsTime) -> Self {
        Self { hour: t.hour }
    }
}

impl From<HmTime> for HTime {
    #[inline]
    fn from(t: HmTime) -> Self {
        Self { hour: t.hour }
    }
}

impl From<HTime> for HmsTime {
    #[inline]
    fn from(t: HTime) -> Self {
        Self {
            hour: t.hour,
            minute: 0,
            second: 0,
        }
    }
}

impl From<LocalTime<HmsTime>> for LocalTime<HmTime> {
    #[inline]
    fn from(t: LocalTime<HmsTime>) -> Self {
        Self {
            naive: HmTime {
                hour: t.naive.hour,
                minute: t.naive.minute,
            },
            fraction: (t.naive.second as f32 + t.fraction) / 60.,
        }
    }
}

impl From<LocalTime<HmsTime>> for LocalTime<HTime> {
    #[inline]
    fn from(t: LocalTime<HmsTime>) -> Self {
        Self {
            naive: HTime { hour: t.naive.hour },
            fraction: t.naive.minute as f32 / 60. + (t.naive.second as f32 + t.fraction) / 3_600.,
        }
    }
}

impl From<LocalTime<HmTime>> for LocalTime<HTime> {
    #[inline]
    fn from(t: LocalTime<HmTime>) -> Self {
        Self {
            naive: HTime { hour: t.naive.hour },
            fraction: (t.naive.minute as f32 + t.fraction) / 60.,
        }
    }
}

impl From<LocalTime<HmTime>> for LocalTime<HmsTime> {
    #[inline]
    fn from(t: LocalTime<HmTime>) -> Self {
        Self {
            naive: HmsTime {
                hour: t.naive.hour,
                minute: t.naive.minute,
                second: t.second(),
            },
            fraction: (t.fraction * 60.) % 1.,
        }
    }
}

impl From<LocalTime<HTime>> for LocalTime<HmsTime> {
    #[inline]
    fn from(t: LocalTime<HTime>) -> Self {
        Self {
            naive: HmsTime {
                hour: t.naive.hour,
                minute: t.minute(),
                second: t.second(),
            },
            fraction: (t.fraction * 3600.) % 1.,
        }
    }
}

impl From<GlobalTime<HmsTime>> for GlobalTime<HmTime> {
    #[inline]
    fn from(t: GlobalTime<HmsTime>) -> Self {
        Self {
            local: t.local.into(),
            timezone: t.timezone,
        }
    }
}

impl From<GlobalTime<HmsTime>> for GlobalTime<HTime> {
    #[inline]
    fn from(t: GlobalTime<HmsTime>) -> Self {
        Self {
            local: t.local.into(),
            timezone: t.timezone,
        }
    }
}

impl From<GlobalTime<HmTime>> for GlobalTime<HTime> {
    #[inline]
    fn from(t: GlobalTime<HmTime>) -> Self {
        Self {
            local: t.local.into(),
            timezone: t.timezone,
        }
    }
}

impl From<GlobalTime<HmTime>> for GlobalTime<HmsTime> {
    #[inline]
    fn from(t: GlobalTime<HmTime>) -> Self {
        Self {
            local: t.local.into(),
            timezone: t.timezone,
        }
    }
}

impl From<GlobalTime<HTime>> for GlobalTime<HmsTime> {
    #[inline]
    fn from(t: GlobalTime<HTime>) -> Self {
        Self {
            local: t.local.into(),
            timezone: t.timezone,
        }
    }
}

impl From<AnyTime<HmsTime>> for AnyTime<HmTime> {
    #[inline]
    fn from(t: AnyTime<HmsTime>) -> Self {
        match t {
            AnyTime::Global(t) => AnyTime::Global(t.into()),
            AnyTime::Local(t) => AnyTime::Local(t.into()),
        }
    }
}

impl From<AnyTime<HmsTime>> for AnyTime<HTime> {
    #[inline]
    fn from(t: AnyTime<HmsTime>) -> Self {
        match t {
            AnyTime::Global(t) => AnyTime::Global(t.into()),
            AnyTime::Local(t) => AnyTime::Local(t.into()),
        }
    }
}

impl From<AnyTime<HmTime>> for AnyTime<HTime> {
    #[inline]
    fn from(t: AnyTime<HmTime>) -> Self {
        match t {
            AnyTime::Global(t) => AnyTime::Global(t.into()),
            AnyTime::Local(t) => AnyTime::Local(t.into()),
        }
    }
}

impl From<ApproxNaiveTime> for HmsTime {
    #[inline]
    fn from(t: ApproxNaiveTime) -> Self {
        match t {
            ApproxNaiveTime::HMS(t) => t,
            ApproxNaiveTime::HM(t) => t.into(),
            ApproxNaiveTime::H(t) => t.into(),
        }
    }
}

impl From<ApproxLocalTime> for LocalTime<HmsTime> {
    #[inline]
    fn from(t: ApproxLocalTime) -> Self {
        match t {
            ApproxLocalTime::HMS(t) => t,
            ApproxLocalTime::HM(t) => t.into(),
            ApproxLocalTime::H(t) => t.into(),
        }
    }
}

impl From<ApproxGlobalTime> for GlobalTime<HmsTime> {
    #[inline]
    fn from(t: ApproxGlobalTime) -> Self {
        match t {
            ApproxGlobalTime::HMS(t) => t,
            ApproxGlobalTime::HM(t) => t.into(),
            ApproxGlobalTime::H(t) => t.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_time_hms() {
        assert!(HmsTime {
            hour: 0,
            minute: 1,
            second: 60
        }
        .is_valid());

        assert!(!HmsTime {
            hour: 0,
            minute: 1,
            second: 61
        }
        .is_valid());
    }

    #[test]
    fn valid_time_hm() {
        assert!(HmTime {
            hour: 0,
            minute: 59
        }
        .is_valid());

        assert!(!HmTime {
            hour: 0,
            minute: 60
        }
        .is_valid());
    }

    #[test]
    fn valid_time_h() {
        assert!(HTime { hour: 24 }.is_valid());

        assert!(!HTime { hour: 25 }.is_valid());
    }

    #[test]
    fn valid_time_local() {
        assert!(LocalTime {
            naive: HTime { hour: 0 },
            fraction: 0.999
        }
        .is_valid());

        assert!(!LocalTime {
            naive: HTime { hour: 0 },
            fraction: 1.
        }
        .is_valid());
    }

    #[test]
    fn valid_time_global() {
        assert!(GlobalTime {
            local: LocalTime {
                naive: HTime { hour: 0 },
                fraction: 0.
            },
            timezone: 24 * 60 - 1
        }
        .is_valid());

        assert!(!GlobalTime {
            local: LocalTime {
                naive: HTime { hour: 0 },
                fraction: 0.
            },
            timezone: 24 * 60
        }
        .is_valid());
        assert!(!GlobalTime {
            local: LocalTime {
                naive: HTime { hour: 0 },
                fraction: 0.
            },
            timezone: -24 * 60
        }
        .is_valid());

        assert!(!GlobalTime {
            local: LocalTime {
                naive: HTime { hour: 25 },
                fraction: 0.
            },
            timezone: 0
        }
        .is_valid());
    }

    #[test]
    fn valid_time_any() {
        let local = LocalTime {
            naive: HTime { hour: 25 },
            fraction: 0.,
        };
        assert!(!AnyTime::Local(local.clone()).is_valid());
        assert!(!AnyTime::Global(GlobalTime { local, timezone: 0 }).is_valid());
    }
}
