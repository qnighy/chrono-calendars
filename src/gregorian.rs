use chrono::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gregorian {
    inner: NaiveDate,
}

impl Gregorian {
    pub fn year(&self) -> i32 {
        self.inner.year()
    }
    pub fn month(&self) -> u32 {
        self.inner.month()
    }
    pub fn day(&self) -> u32 {
        self.inner.day()
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
