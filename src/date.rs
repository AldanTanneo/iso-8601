use {crate::Valid, std::convert::From};

/// Complete date representations
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Date<Y: Year = i16> {
    YMD(YmdDate<Y>),
    WD(WdDate<Y>),
    O(ODate<Y>),
}

/// Date representations with reduced accuracy
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ApproxDate<Y: Year = i16> {
    YMD(YmdDate<Y>),
    YM(YmDate<Y>),
    Y(YDate<Y>),
    C(CDate),
    WD(WdDate<Y>),
    W(WDate<Y>),
    O(ODate<Y>),
}

/// Calendar date (4.1.2.2)
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct YmdDate<Y: Year = i16> {
    pub year: Y,
    pub month: u8,
    pub day: u8,
}

/// A specific month (4.1.2.3a)
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct YmDate<Y: Year = i16> {
    pub year: Y,
    pub month: u8,
}

/// A specific year (4.1.2.3b)
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct YDate<Y: Year = i16> {
    pub year: Y,
}

// TODO support expanded century
/// A specific century (4.1.2.3c)
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CDate {
    pub century: i8,
}

/// Week date (4.1.4.2)
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct WdDate<Y: Year = i16> {
    pub year: Y,
    pub week: u8,
    pub day: u8,
}

/// A specific week (4.1.4.3)
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct WDate<Y: Year = i16> {
    pub year: Y,
    pub week: u8,
}

/// Ordinal date (4.1.3)
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ODate<Y: Year = i16> {
    pub year: Y,
    pub day: u16,
}

pub trait Datelike<Y: Year = i16> {}

impl<Y: Year> Datelike<Y> for Date<Y> {}
impl<Y: Year> Datelike<Y> for ApproxDate<Y> {}
impl<Y: Year> Datelike<Y> for YmdDate<Y> {}
impl<Y: Year> Datelike<Y> for YmDate<Y> {}
impl<Y: Year> Datelike<Y> for YDate<Y> {}
impl<Y: Year> Datelike<Y> for CDate {}
impl<Y: Year> Datelike<Y> for WdDate<Y> {}
impl<Y: Year> Datelike<Y> for WDate<Y> {}
impl<Y: Year> Datelike<Y> for ODate<Y> {}

impl_fromstr_parse!(Date, date);
impl_fromstr_parse!(ApproxDate, date_approx);
impl_fromstr_parse!(YmdDate, date_ymd);
impl_fromstr_parse!(YmDate, date_ym);
impl_fromstr_parse!(YDate, date_y);
impl_fromstr_parse!(CDate, date_c);
impl_fromstr_parse!(WdDate, date_wd);
impl_fromstr_parse!(WDate, date_w);
impl_fromstr_parse!(ODate, date_o);

impl<Y> Valid for Date<Y>
where
    Y: Year + Clone,
{
    #[inline]
    fn is_valid(&self) -> bool {
        match self {
            Date::YMD(date) => date.is_valid(),
            Date::WD(date) => date.is_valid(),
            Date::O(date) => date.is_valid(),
        }
    }
}

impl<Y> Valid for ApproxDate<Y>
where
    Y: Year + Clone,
{
    #[inline]
    fn is_valid(&self) -> bool {
        match self {
            ApproxDate::YMD(date) => date.is_valid(),
            ApproxDate::YM(date) => date.is_valid(),
            ApproxDate::Y(date) => date.is_valid(),
            ApproxDate::C(date) => date.is_valid(),
            ApproxDate::WD(date) => date.is_valid(),
            ApproxDate::W(date) => date.is_valid(),
            ApproxDate::O(date) => date.is_valid(),
        }
    }
}

impl<Y> Valid for YmdDate<Y>
where
    Y: Year,
{
    #[inline]
    fn is_valid(&self) -> bool {
        self.day >= 1
            && self.day
                <= match self.month {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                    4 | 6 | 9 | 11 => 30,
                    2 => {
                        if self.year.is_leap() {
                            29
                        } else {
                            28
                        }
                    }
                    _ => return false,
                }
    }
}

impl<Y> Valid for YmDate<Y>
where
    Y: Year,
{
    #[inline]
    fn is_valid(&self) -> bool {
        self.month >= 1 && self.month <= 12
    }
}

impl<Y> Valid for YDate<Y>
where
    Y: Year,
{
    #[inline]
    fn is_valid(&self) -> bool {
        true
    }
}

impl Valid for CDate {
    #[inline]
    fn is_valid(&self) -> bool {
        true
    }
}

impl<Y> Valid for WdDate<Y>
where
    Y: Year + Clone,
{
    #[inline]
    fn is_valid(&self) -> bool {
        WDate::from(self.clone()).is_valid() && self.day >= 1 && self.day <= 7
    }
}

impl<Y> Valid for WDate<Y>
where
    Y: Year,
{
    #[inline]
    fn is_valid(&self) -> bool {
        self.week >= 1 && self.week <= self.year.num_weeks()
    }
}

impl<Y> Valid for ODate<Y>
where
    Y: Year,
{
    #[inline]
    fn is_valid(&self) -> bool {
        self.day >= 1 && self.day <= self.year.num_days()
    }
}

pub trait Year {
    fn is_leap(&self) -> bool;
    fn num_weeks(&self) -> u8;

    #[inline]
    fn num_days(&self) -> u16 {
        if self.is_leap() {
            366
        } else {
            365
        }
    }
}

macro_rules! impl_years {
    ($mac:ident) => {
        $mac!(i16);
        $mac!(i32);
        $mac!(i64);
        $mac!(i128);
        $mac!(isize);
        $mac!(u16);
        $mac!(u32);
        $mac!(u64);
        $mac!(u128);
        $mac!(usize);
    };
}

macro_rules! impl_year {
    ($ty:ty) => {
        impl Year for $ty {
            #[inline]
            fn is_leap(&self) -> bool {
                let factor = |x| self % x == 0;
                factor(4) && (!factor(100) || factor(400))
            }

            #[inline]
            fn num_weeks(&self) -> u8 {
                // https://en.wikipedia.org/wiki/ISO_week_date#Weeks_per_year
                let p = |x| (x + x / 4 - x / 100 + x / 400) % 7;
                if p(*self) == 4 || p(self - 1) == 3 {
                    53
                } else {
                    52
                }
            }
        }
    };
}
impl_years!(impl_year);

impl<Y> From<Date<Y>> for ApproxDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: Date<Y>) -> Self {
        match date {
            Date::YMD(date) => ApproxDate::YMD(date),
            Date::WD(date) => ApproxDate::WD(date),
            Date::O(date) => ApproxDate::O(date),
        }
    }
}

impl<Y> From<Date<Y>> for YmdDate<Y>
where
    Y: Year,
    ODate<Y>: From<WdDate<Y>>,
{
    #[inline]
    fn from(date: Date<Y>) -> Self {
        match date {
            Date::YMD(date) => date,
            Date::WD(date) => date.into(),
            Date::O(date) => date.into(),
        }
    }
}

impl<Y> From<Date<Y>> for WdDate<Y>
where
    Y: Year,
    WdDate<Y>: From<ODate<Y>>,
    ODate<Y>: From<WdDate<Y>>,
{
    #[inline]
    fn from(date: Date<Y>) -> Self {
        match date {
            Date::YMD(date) => date.into(),
            Date::WD(date) => date,
            Date::O(date) => date.into(),
        }
    }
}

impl<Y> From<Date<Y>> for ODate<Y>
where
    Y: Year,
    ODate<Y>: From<WdDate<Y>>,
{
    #[inline]
    fn from(date: Date<Y>) -> Self {
        match date {
            Date::YMD(date) => date.into(),
            Date::WD(date) => date.into(),
            Date::O(date) => date,
        }
    }
}

impl<Y> From<YmdDate<Y>> for YmDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: YmdDate<Y>) -> Self {
        Self {
            year: date.year,
            month: date.month,
        }
    }
}

impl<Y> From<YmdDate<Y>> for YDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: YmdDate<Y>) -> Self {
        Self { year: date.year }
    }
}

impl<Y> From<YmDate<Y>> for YDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: YmDate<Y>) -> Self {
        Self { year: date.year }
    }
}

impl<Y> From<WdDate<Y>> for WDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: WdDate<Y>) -> Self {
        Self {
            year: date.year,
            week: date.week,
        }
    }
}

impl<Y> From<WdDate<Y>> for YmdDate<Y>
where
    Y: Year,
    ODate<Y>: From<WdDate<Y>>,
{
    #[inline]
    fn from(date: WdDate<Y>) -> Self {
        ODate::from(date).into()
    }
}

impl<Y> From<ODate<Y>> for YmdDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: ODate<Y>) -> Self {
        let leap = date.year.is_leap();
        let (month, day) = match date.day {
            1..=31 => (1, date.day),
            32..=60 if leap => (2, date.day - 31),
            32..=59 => (2, date.day - 31),
            61..=91 if leap => (3, date.day - 60),
            60..=90 => (3, date.day - 59),
            92..=121 if leap => (4, date.day - 91),
            91..=120 => (4, date.day - 90),
            122..=152 if leap => (5, date.day - 121),
            121..=151 => (5, date.day - 120),
            153..=182 if leap => (6, date.day - 152),
            152..=181 => (6, date.day - 151),
            183..=213 if leap => (7, date.day - 182),
            182..=212 => (7, date.day - 181),
            214..=244 if leap => (8, date.day - 213),
            213..=243 => (8, date.day - 212),
            245..=274 if leap => (9, date.day - 244),
            244..=273 => (9, date.day - 243),
            275..=305 if leap => (10, date.day - 274),
            274..=304 => (10, date.day - 273),
            306..=335 if leap => (11, date.day - 305),
            305..=334 => (11, date.day - 304),
            336..=366 if leap => (12, date.day - 335),
            335..=365 => (12, date.day - 334),
            day => panic!("invalid day: {:?}", day),
        };

        Self {
            year: date.year,
            month,
            day: day as u8,
        }
    }
}

impl<Y> From<WdDate<Y>> for YmDate<Y>
where
    Y: Year,
    YmdDate<Y>: From<WdDate<Y>>,
{
    #[inline]
    fn from(date: WdDate<Y>) -> Self {
        YmdDate::from(date).into()
    }
}

impl<Y> From<ODate<Y>> for YmDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: ODate<Y>) -> Self {
        YmdDate::from(date).into()
    }
}

impl<Y> From<WdDate<Y>> for YDate<Y>
where
    Y: Year,
    YmdDate<Y>: From<WdDate<Y>>,
{
    #[inline]
    fn from(date: WdDate<Y>) -> Self {
        YmdDate::from(date).into()
    }
}

impl<Y> From<ODate<Y>> for YDate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: ODate<Y>) -> Self {
        Self { year: date.year }
    }
}

impl<Y> From<YmdDate<Y>> for WdDate<Y>
where
    Y: Year,
    ODate<Y>: From<YmdDate<Y>>,
    ODate<Y>: From<WdDate<Y>>,
    WdDate<Y>: From<ODate<Y>>,
{
    #[inline]
    fn from(date: YmdDate<Y>) -> Self {
        ODate::from(date).into()
    }
}

macro_rules! impl_wd_from_o {
    ($ty:ty) => {
        impl From<ODate<$ty>> for WdDate<$ty> {
            #[inline]
            fn from(date: ODate<$ty>) -> Self {
                // https://en.wikipedia.org/wiki/ISO_week_date#Calculating_the_week_number_of_a_given_date
                let y = date.year % 100 % 28;
                let cc = (date.year / 100) % 4;
                let mut c = ((y + (y - 1) / 4 + 5 * cc - 1) % 7) as i16;
                if c > 3 {
                    c -= 7;
                }
                let dc = date.day as i16 + c;
                Self {
                    year: date.year,
                    week: (dc as f32 / 7.).ceil() as u8,
                    day: (dc % 7) as u8,
                }
            }
        }
    };
}
impl_years!(impl_wd_from_o);

impl<Y> From<YmdDate<Y>> for ODate<Y>
where
    Y: Year,
{
    #[inline]
    fn from(date: YmdDate<Y>) -> Self {
        let leap = date.year.is_leap();
        Self {
            year: date.year,
            day: match date.month {
                1 => 0,
                2 => 31,
                3 if leap => 60,
                3 => 59,
                4 if leap => 91,
                4 => 90,
                5 if leap => 121,
                5 => 120,
                6 if leap => 152,
                6 => 151,
                7 if leap => 182,
                7 => 181,
                8 if leap => 213,
                8 => 212,
                9 if leap => 244,
                9 => 243,
                10 if leap => 274,
                10 => 273,
                11 if leap => 305,
                11 => 304,
                12 if leap => 335,
                12 => 334,
                month => panic!("invalid month: {:?}", month),
            } + date.day as u16,
        }
    }
}

macro_rules! impl_o_from_wd {
    ($ty:ty) => {
        impl From<WdDate<$ty>> for ODate<$ty> {
            #[inline]
            fn from(date: WdDate<$ty>) -> Self {
                // https://en.wikipedia.org/wiki/ISO_week_date#Calculating_a_date_given_the_year,_week_number_and_weekday

                #[inline]
                fn weekday_jan4(year: $ty) -> u8 {
                    #[inline]
                    fn weekday_jan1(year: $ty) -> u8 {
                        // https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Gauss's_algorithm
                        let y = year - 1;
                        ((1 + 5 * (y % 4) + 4 * (y % 100) + 6 * (y % 400)) % 7) as u8
                    }

                    (weekday_jan1(year) + 3) % 7
                }

                let mut day = (date.week * 7 + date.day - (weekday_jan4(date.year) + 3)) as u16;
                if day < 1 {
                    day += (date.year - 1).num_days();
                }
                if day > date.year.num_days() {
                    day -= date.year.num_days();
                }

                Self {
                    year: date.year,
                    day,
                }
            }
        }
    };
}
impl_years!(impl_o_from_wd);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ymd_from_wd() {
        assert_eq!(
            YmdDate::from(WdDate {
                year: 1985,
                week: 15,
                day: 5
            }),
            YmdDate {
                year: 1985,
                month: 4,
                day: 12
            }
        );
    }

    #[test]
    fn ymd_from_o() {
        assert_eq!(
            YmdDate::from(ODate {
                year: 1985,
                day: 102
            }),
            YmdDate {
                year: 1985,
                month: 4,
                day: 12
            }
        );
    }

    #[test]
    fn wd_from_ymd() {
        assert_eq!(
            WdDate::from(YmdDate {
                year: 1985,
                month: 4,
                day: 12
            }),
            WdDate {
                year: 1985,
                week: 15,
                day: 5
            }
        );
        assert_eq!(
            WdDate::from(YmdDate {
                year: 2023,
                month: 2,
                day: 27
            }),
            WdDate {
                year: 2023,
                week: 9,
                day: 1
            }
        );
    }

    #[test]
    fn wd_from_o() {
        assert_eq!(
            WdDate::from(ODate {
                year: 1985,
                day: 102
            }),
            WdDate {
                year: 1985,
                week: 15,
                day: 5
            }
        );
    }

    #[test]
    fn o_from_ymd() {
        assert_eq!(
            ODate::from(YmdDate {
                year: 1985,
                month: 4,
                day: 12
            }),
            ODate {
                year: 1985,
                day: 102
            }
        );
    }

    #[test]
    fn o_from_wd() {
        assert_eq!(
            ODate::from(WdDate {
                year: 1985,
                week: 15,
                day: 5
            }),
            ODate {
                year: 1985,
                day: 102
            }
        );
    }

    #[test]
    fn valid_date_ymd() {
        assert!(!YmdDate {
            year: 0,
            month: 13,
            day: 1
        }
        .is_valid());
        assert!(!YmdDate {
            year: 0,
            month: 0,
            day: 1
        }
        .is_valid());

        assert!(!YmdDate {
            year: 2018,
            month: 2,
            day: 29
        }
        .is_valid());
    }

    #[test]
    fn valid_date_wd() {
        assert!(!WdDate {
            year: 0,
            week: 0,
            day: 1
        }
        .is_valid());
        assert!(!WdDate {
            year: 2018,
            week: 53,
            day: 1
        }
        .is_valid());

        assert!(!WdDate {
            year: 0,
            week: 1,
            day: 0
        }
        .is_valid());
        assert!(!WdDate {
            year: 0,
            week: 1,
            day: 8
        }
        .is_valid());
    }

    #[test]
    fn valid_date_o() {
        assert!(!ODate {
            year: 2018,
            day: 366
        }
        .is_valid());
        assert!(ODate {
            year: 2020,
            day: 366
        }
        .is_valid());
    }
}
