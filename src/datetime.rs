use crate::{date::*, time::*, Valid};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DateTime<D = YmdDate, T = GlobalTime>
where
    D: Datelike,
    T: Timelike,
{
    pub date: D,
    pub time: T,
}

impl_fromstr_parse!(DateTime<Date,       GlobalTime<HmsTime>>, datetime_global_hms);
impl_fromstr_parse!(DateTime<Date,       GlobalTime<HmTime>>,  datetime_global_hm);
impl_fromstr_parse!(DateTime<Date,       GlobalTime<HTime>>,   datetime_global_h);
impl_fromstr_parse!(DateTime<Date,       LocalTime<HmsTime>>,  datetime_local_hms);
impl_fromstr_parse!(DateTime<Date,       LocalTime<HmTime>>,   datetime_local_hm);
impl_fromstr_parse!(DateTime<Date,       LocalTime<HTime>>,    datetime_local_h);
impl_fromstr_parse!(DateTime<Date,       AnyTime<HmsTime>>,    datetime_any_hms);
impl_fromstr_parse!(DateTime<Date,       AnyTime<HmTime>>,     datetime_any_hm);
impl_fromstr_parse!(DateTime<Date,       AnyTime<HTime>>,      datetime_any_h);
impl_fromstr_parse!(DateTime<Date,       ApproxGlobalTime>,    datetime_global_approx);
impl_fromstr_parse!(DateTime<Date,       ApproxLocalTime>,     datetime_local_approx);
impl_fromstr_parse!(DateTime<Date,       ApproxAnyTime>,       datetime_any_approx);
impl_fromstr_parse!(DateTime<ApproxDate, GlobalTime<HmsTime>>, datetime_approx_global_hms);
impl_fromstr_parse!(DateTime<ApproxDate, GlobalTime<HmTime>>,  datetime_approx_global_hm);
impl_fromstr_parse!(DateTime<ApproxDate, GlobalTime<HTime>>,   datetime_approx_global_h);
impl_fromstr_parse!(DateTime<ApproxDate, LocalTime<HmsTime>>,  datetime_approx_local_hms);
impl_fromstr_parse!(DateTime<ApproxDate, LocalTime<HmTime>>,   datetime_approx_local_hm);
impl_fromstr_parse!(DateTime<ApproxDate, LocalTime<HTime>>,    datetime_approx_local_h);
impl_fromstr_parse!(DateTime<ApproxDate, AnyTime<HmsTime>>,    datetime_approx_any_hms);
impl_fromstr_parse!(DateTime<ApproxDate, AnyTime<HmTime>>,     datetime_approx_any_hm);
impl_fromstr_parse!(DateTime<ApproxDate, AnyTime<HTime>>,      datetime_approx_any_h);
impl_fromstr_parse!(DateTime<ApproxDate, ApproxGlobalTime>,    datetime_approx_global_approx);
impl_fromstr_parse!(DateTime<ApproxDate, ApproxLocalTime>,     datetime_approx_local_approx);
impl_fromstr_parse!(DateTime<ApproxDate, ApproxAnyTime>,       datetime_approx_any_approx);

impl<D, T> Valid for DateTime<D, T>
where
    D: Datelike + Valid,
    T: Timelike + Valid,
{
    fn is_valid(&self) -> bool {
        self.date.is_valid() && self.time.is_valid()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum PartialDateTime<D = ApproxDate, T = ApproxAnyTime>
where
    D: Datelike,
    T: Timelike,
{
    Date(D),
    Time(T),
    DateTime(DateTime<D, T>),
}

impl_fromstr_parse!(PartialDateTime<ApproxDate, ApproxAnyTime>, partial_datetime_approx_any_approx);
