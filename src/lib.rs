use chrono::prelude::*;
use ordslice::Ext;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Julian {
    inner: NaiveDate,
    year: i32,
    month: u8,
    day: u8,
}

impl Julian {
    pub fn year(&self) -> i32 {
        self.year
    }
    pub fn month(&self) -> u32 {
        self.month as u32
    }
    pub fn day(&self) -> u32 {
        self.day as u32
    }
}

impl From<NaiveDate> for Julian {
    fn from(date: NaiveDate) -> Self {
        // 0001-01-01 (Julian) = 0001-01-03 (proleptic Gregorian)
        // Plus an adjustment of 1 to obtain a 0-based index.
        let num_days = date.num_days_from_ce() + 1;
        let (cycle, cycle_offset) = (
            num_days.div_euclid_polyfill(365 * 4 + 1),
            num_days.rem_euclid_polyfill(365 * 4 + 1),
        );
        let (year, year_offset) = {
            let table = &[0, 365, 730, 1095];
            let idx = table.upper_bound(&cycle_offset);
            (cycle * 4 + idx as i32, cycle_offset - table[idx - 1])
        };
        let table = if year % 4 == 0 {
            &[0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335]
        } else {
            &[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334]
        };
        let month = table.upper_bound(&year_offset);
        let day = year_offset - table[month - 1] + 1;
        Self {
            inner: date,
            year,
            month: month as u8,
            day: day as u8,
        }
    }
}

impl From<Julian> for NaiveDate {
    fn from(date: Julian) -> Self {
        date.inner
    }
}

trait DivEuclidPolyfillExt {
    fn div_euclid_polyfill(self, rhs: Self) -> Self;
    fn rem_euclid_polyfill(self, rhs: Self) -> Self;
}

impl DivEuclidPolyfillExt for i32 {
    fn div_euclid_polyfill(self, rhs: Self) -> Self {
        let q = self / rhs;
        if self % rhs < 0 {
            return if rhs > 0 { q - 1 } else { q + 1 };
        }
        q
    }
    fn rem_euclid_polyfill(self, rhs: Self) -> Self {
        let r = self % rhs;
        if r < 0 {
            if rhs < 0 {
                r - rhs
            } else {
                r + rhs
            }
        } else {
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_gregorian_equivalence() {
        for o in 1..=365 {
            let date = NaiveDate::from_yo(203, o);
            let gregorian: Gregorian = date.into();
            let julian: Julian = date.into();
            assert_eq!(
                (gregorian.year(), gregorian.month(), gregorian.day()),
                (julian.year(), julian.month(), julian.day()),
            )
        }

        for o in 1..=366 {
            let date = NaiveDate::from_yo(204, o);
            let gregorian: Gregorian = date.into();
            let julian: Julian = date.into();
            assert_eq!(
                (gregorian.year(), gregorian.month(), gregorian.day()),
                (julian.year(), julian.month(), julian.day()),
            )
        }
    }

    #[test]
    fn test_10day_difference() {
        let gregorian_start = NaiveDate::from_ymd(1582, 10, 15);
        let gregorian_start: Julian = gregorian_start.into();
        assert_eq!(
            (
                gregorian_start.year(),
                gregorian_start.month(),
                gregorian_start.day()
            ),
            (1582, 10, 5)
        );
    }
}
