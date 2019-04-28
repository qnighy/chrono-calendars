use chrono::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gregorian {
    inner: NaiveDate,
}

impl Gregorian {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        NaiveDate::from_ymd(year, month, day).into()
    }

    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(|x| x.into())
    }

    pub fn from_yo(year: i32, ordinal: u32) -> Self {
        NaiveDate::from_yo(year, ordinal).into()
    }

    pub fn from_yo_opt(year: i32, ordinal: u32) -> Option<Self> {
        NaiveDate::from_yo_opt(year, ordinal).map(|x| x.into())
    }

    pub fn from_num_days_from_gregorian_ce(days: i32) -> Self {
        NaiveDate::from_num_days_from_ce(days).into()
    }

    pub fn from_num_days_from_gregorian_ce_opt(days: i32) -> Option<Self> {
        NaiveDate::from_num_days_from_ce_opt(days).map(|x| x.into())
    }

    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    pub fn month(&self) -> u32 {
        self.inner.month()
    }

    pub fn month0(&self) -> u32 {
        self.inner.month0()
    }

    pub fn day(&self) -> u32 {
        self.inner.day()
    }

    pub fn day0(&self) -> u32 {
        self.inner.day0()
    }

    pub fn ordinal(&self) -> u32 {
        self.inner.ordinal()
    }

    pub fn ordinal0(&self) -> u32 {
        self.inner.ordinal0()
    }

    pub fn weekday(&self) -> Weekday {
        self.inner.weekday()
    }

    pub fn with_year(&self, year: i32) -> Option<Self> {
        self.inner.with_year(year).map(|x| x.into())
    }

    pub fn with_month(&self, month: u32) -> Option<Self> {
        self.inner.with_month(month).map(|x| x.into())
    }

    pub fn with_month0(&self, month0: u32) -> Option<Self> {
        self.inner.with_month0(month0).map(|x| x.into())
    }

    pub fn with_day(&self, day: u32) -> Option<Self> {
        self.inner.with_day(day).map(|x| x.into())
    }

    pub fn with_day0(&self, day0: u32) -> Option<Self> {
        self.inner.with_day0(day0).map(|x| x.into())
    }

    pub fn with_ordinal(&self, ordinal: u32) -> Option<Self> {
        self.inner.with_ordinal(ordinal).map(|x| x.into())
    }

    pub fn with_ordinal0(&self, ordinal0: u32) -> Option<Self> {
        self.inner.with_ordinal0(ordinal0).map(|x| x.into())
    }
}

impl From<NaiveDate> for Gregorian {
    fn from(date: NaiveDate) -> Self {
        Self { inner: date }
    }
}

impl From<Gregorian> for NaiveDate {
    fn from(date: Gregorian) -> Self {
        date.inner
    }
}
