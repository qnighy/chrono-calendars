use std::fmt;

use chrono::prelude::*;
use ordslice::Ext;

use crate::utils::{ContainsPolyfillExt, DivEuclidPolyfillExt};

// TODO: fix this to correctly represent MIN_DATE
static MIN_YMD: (i32, u32, u32) = (-262139, 5, 22);
// TODO: fix this to correctly represent MAX_DATE
static MAX_YMD: (i32, u32, u32) = (262138, 8, 15);

// TODO: fix this to correctly represent MIN_DATE
static MIN_YO: (i32, u32) = (-262139, 142);
// TODO: fix this to correctly represent MAX_DATE
static MAX_YO: (i32, u32) = (262138, 227);

// TODO: fix this to correctly represent MIN_DATE
static MIN_DAYS: i32 = -95746493;
// TODO: fix this to correctly represent MAX_DATE
static MAX_DAYS: i32 = 95745766;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Julian {
    ymd: PackedYMD,
}

impl Julian {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        Self::from_ymd_opt(year, month, day).expect("invalid or out-of-range date")
    }

    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(MIN_YMD..=MAX_YMD).contains_polyfill(&(year, month, day)) {
            return None;
        }
        if month == 0 || month > 12 || day == 0 || day > days_of_month(year, month) {
            return None;
        }
        Some(Julian {
            ymd: PackedYMD::new(year, month, day),
        })
    }

    pub fn from_yo(year: i32, ordinal: u32) -> Self {
        Self::from_yo_opt(year, ordinal).expect("invalid or out-of-range date")
    }

    pub fn from_yo_opt(year: i32, ordinal: u32) -> Option<Self> {
        if !(MIN_YO..=MAX_YO).contains_polyfill(&(year, ordinal)) {
            return None;
        }
        if ordinal == 0 || ordinal > days_of_year(year) {
            return None;
        }
        let (month, day) = ordinal_to_md(year, ordinal);
        Some(Julian {
            ymd: PackedYMD::new(year, month, day),
        })
    }

    pub fn from_num_days_from_julian_ce(days: i32) -> Self {
        Self::from_num_days_from_julian_ce_opt(days).expect("invalid or out-of-range date")
    }

    pub fn from_num_days_from_julian_ce_opt(days: i32) -> Option<Self> {
        if !(MIN_DAYS..=MAX_DAYS).contains_polyfill(&days) {
            return None;
        }
        let (year, ordinal) = ce_days_to_yo(days);
        let (month, day) = ordinal_to_md(year, ordinal);
        Some(Julian {
            ymd: PackedYMD::new(year, month, day),
        })
    }

    pub fn year(&self) -> i32 {
        self.ymd.year()
    }

    pub fn month(&self) -> u32 {
        self.ymd.month()
    }

    pub fn month0(&self) -> u32 {
        self.ymd.month() - 1
    }

    pub fn day(&self) -> u32 {
        self.ymd.day()
    }

    pub fn day0(&self) -> u32 {
        self.ymd.day() - 1
    }

    pub fn ordinal(&self) -> u32 {
        md_to_ordinal(self.year(), self.month(), self.day())
    }

    pub fn ordinal0(&self) -> u32 {
        md_to_ordinal(self.year(), self.month(), self.day()) - 1
    }

    pub fn weekday(&self) -> Weekday {
        use Weekday::*;
        static TABLE: [Weekday; 7] = [Fri, Sat, Sun, Mon, Tue, Wed, Thu];
        TABLE[self.num_days_from_julian_ce().rem_euclid_polyfill(7) as usize]
    }

    pub fn with_year(&self, year: i32) -> Option<Self> {
        Self::from_ymd_opt(year, self.month(), self.day())
    }

    pub fn with_month(&self, month: u32) -> Option<Self> {
        Self::from_ymd_opt(self.year(), month, self.day())
    }

    pub fn with_month0(&self, month0: u32) -> Option<Self> {
        Self::from_ymd_opt(self.year(), month0.wrapping_add(1), self.day())
    }

    pub fn with_day(&self, day: u32) -> Option<Self> {
        Self::from_ymd_opt(self.year(), self.month(), day)
    }

    pub fn with_day0(&self, day0: u32) -> Option<Self> {
        Self::from_ymd_opt(self.year(), self.month(), day0.wrapping_add(1))
    }

    pub fn with_ordinal(&self, ordinal: u32) -> Option<Self> {
        Self::from_yo_opt(self.year(), ordinal)
    }

    pub fn with_ordinal0(&self, ordinal0: u32) -> Option<Self> {
        Self::from_yo_opt(self.year(), ordinal0.wrapping_add(1))
    }

    pub fn year_ce(&self) -> (bool, u32) {
        let year = self.year();
        if year > 0 {
            (true, year as u32)
        } else {
            (false, (1 - year) as u32)
        }
    }

    pub fn num_days_from_julian_ce(&self) -> i32 {
        let ordinal = md_to_ordinal(self.year(), self.month(), self.day());
        yo_to_ce_days(self.year(), ordinal)
    }
}

impl From<NaiveDate> for Julian {
    fn from(date: NaiveDate) -> Self {
        Self::from_num_days_from_julian_ce(date.num_days_from_ce() + 2)
    }
}

impl From<Julian> for NaiveDate {
    fn from(date: Julian) -> Self {
        Self::from_num_days_from_ce(date.num_days_from_julian_ce() - 2)
    }
}

static YEAR_ACCUM_TABLE: [u32; 4] = [0, 366, 731, 1096];

fn ce_days_to_yo(days: i32) -> (i32, u32) {
    let days = days + 366 - 1;
    let cycle = days.div_euclid_polyfill(365 * 4 + 1);
    let cycle_offset = days.rem_euclid_polyfill(365 * 4 + 1) as u32;
    let year_in_cycle = YEAR_ACCUM_TABLE[1..].upper_bound(&cycle_offset);
    let year = cycle * 4 + year_in_cycle as i32;
    let ordinal = cycle_offset - YEAR_ACCUM_TABLE[year_in_cycle] + 1;
    (year, ordinal as u32)
}

fn yo_to_ce_days(year: i32, ordinal: u32) -> i32 {
    let cycle = year >> 2;
    let cycle_offset = YEAR_ACCUM_TABLE[(year & 3) as usize] as i32 + (ordinal as i32 - 1);
    cycle * (365 * 4 + 1) + cycle_offset - 366 + 1
}

fn ordinal_to_md(year: i32, ordinal: u32) -> (u32, u32) {
    let table = month_accum_table(year);
    let month0 = table[1..].upper_bound(&(ordinal - 1));
    let day = ordinal - table[month0];
    (month0 as u32 + 1, day)
}

fn md_to_ordinal(year: i32, month: u32, day: u32) -> u32 {
    let table = month_accum_table(year);
    table[month as usize - 1] + day
}

fn days_of_year(year: i32) -> u32 {
    365 + (year % 4 == 0) as u32
}

fn days_of_month(year: i32, month: u32) -> u32 {
    (if year % 4 == 0 {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    })[month as usize - 1]
}

fn month_accum_table(year: i32) -> &'static [u32; 12] {
    if year % 4 == 0 {
        &[0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335]
    } else {
        &[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PackedYMD(i32);

impl PackedYMD {
    fn new(year: i32, month: u32, day: u32) -> Self {
        PackedYMD((year << 9) | (month << 5) as i32 | day as i32)
    }

    fn year(&self) -> i32 {
        self.0 >> 9
    }

    fn month(&self) -> u32 {
        ((self.0 >> 5) & 15) as u32
    }

    fn day(&self) -> u32 {
        (self.0 & 31) as u32
    }
}

impl fmt::Debug for PackedYMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PackedYMD({:?}, {:?}, {:?})",
            self.year(),
            self.month(),
            self.day()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::Gregorian;

    static CONVERSION_TABLE: [(i32, u32, u32, i32, u32, u32); 69] = [
        (-500, 3, 5, -500, 2, 28),
        (-500, 3, 6, -500, 3, 1),
        (-300, 3, 3, -300, 2, 27),
        (-300, 3, 4, -300, 2, 28),
        (-300, 3, 5, -300, 3, 1),
        (-200, 3, 2, -200, 2, 27),
        (-200, 3, 3, -200, 2, 28),
        (-200, 3, 4, -200, 3, 1),
        (-100, 3, 1, -100, 2, 27),
        (-100, 3, 2, -100, 2, 28),
        (-100, 3, 3, -100, 3, 1),
        (100, 2, 29, 100, 2, 27),
        (100, 3, 1, 100, 2, 28),
        (100, 3, 2, 100, 3, 1),
        (200, 2, 28, 200, 2, 27),
        (200, 2, 29, 200, 2, 28),
        (200, 3, 1, 200, 3, 1),
        (300, 2, 28, 300, 2, 28),
        (300, 2, 29, 300, 3, 1),
        (300, 3, 1, 300, 3, 2),
        (500, 2, 28, 500, 3, 1),
        (500, 2, 29, 500, 3, 2),
        (500, 3, 1, 500, 3, 3),
        (600, 2, 28, 600, 3, 2),
        (600, 2, 29, 600, 3, 3),
        (600, 3, 1, 600, 3, 4),
        (700, 2, 28, 700, 3, 3),
        (700, 2, 29, 700, 3, 4),
        (700, 3, 1, 700, 3, 5),
        (900, 2, 28, 900, 3, 4),
        (900, 2, 29, 900, 3, 5),
        (900, 3, 1, 900, 3, 6),
        (1000, 2, 28, 1000, 3, 5),
        (1000, 2, 29, 1000, 3, 6),
        (1000, 3, 1, 1000, 3, 7),
        (1100, 2, 28, 1100, 3, 6),
        (1100, 2, 29, 1100, 3, 7),
        (1100, 3, 1, 1100, 3, 8),
        (1300, 2, 28, 1300, 3, 7),
        (1300, 2, 29, 1300, 3, 8),
        (1300, 3, 1, 1300, 3, 9),
        (1400, 2, 28, 1400, 3, 8),
        (1400, 2, 29, 1400, 3, 9),
        (1400, 3, 1, 1400, 3, 10),
        (1500, 2, 28, 1500, 3, 9),
        (1500, 2, 29, 1500, 3, 10),
        (1500, 3, 1, 1500, 3, 11),
        (1582, 10, 4, 1582, 10, 14),
        (1582, 10, 5, 1582, 10, 15),
        (1582, 10, 6, 1582, 10, 16),
        (1700, 2, 18, 1700, 2, 28),
        (1700, 2, 19, 1700, 3, 1),
        (1700, 2, 28, 1700, 3, 10),
        (1700, 2, 29, 1700, 3, 11),
        (1700, 3, 1, 1700, 3, 12),
        (1800, 2, 17, 1800, 2, 28),
        (1800, 2, 18, 1800, 3, 1),
        (1800, 2, 28, 1800, 3, 11),
        (1800, 2, 29, 1800, 3, 12),
        (1800, 3, 1, 1800, 3, 13),
        (1900, 2, 16, 1900, 2, 28),
        (1900, 2, 17, 1900, 3, 1),
        (1900, 2, 28, 1900, 3, 12),
        (1900, 2, 29, 1900, 3, 13),
        (1900, 3, 1, 1900, 3, 14),
        (2100, 2, 15, 2100, 2, 28),
        (2100, 2, 16, 2100, 3, 1),
        (2100, 2, 28, 2100, 3, 13),
        (2100, 2, 29, 2100, 3, 14),
    ];

    #[test]
    fn test_julian_gregorian_conversion1() {
        for &(jy, jm, jd, gy, gm, gd) in &CONVERSION_TABLE as &[_] {
            let julian = Julian::from_ymd(jy, jm, jd);
            let date: NaiveDate = julian.into();
            assert_eq!((date.year(), date.month(), date.day()), (gy, gm, gd));
        }
    }

    #[test]
    fn test_gregorian_julian_conversion1() {
        for &(jy, jm, jd, gy, gm, gd) in &CONVERSION_TABLE as &[_] {
            let julian: Julian = NaiveDate::from_ymd(gy, gm, gd).into();
            assert_eq!((julian.year(), julian.month(), julian.day()), (jy, jm, jd));
        }
    }

    #[test]
    fn test_weekday() {
        for &(jy, jm, jd, gy, gm, gd) in &CONVERSION_TABLE as &[_] {
            let julian = Julian::from_ymd(jy, jm, jd);
            let gregorian = Gregorian::from_ymd(gy, gm, gd);
            assert_eq!(julian.weekday(), gregorian.weekday());
        }
    }

    #[test]
    fn test_julian_epoch() {
        assert_eq!(Julian::from_ymd(1, 1, 1).num_days_from_julian_ce(), 1);
        assert_eq!(Julian::from_ymd(0, 12, 31).num_days_from_julian_ce(), 0);
        assert_eq!(
            Julian::from_num_days_from_julian_ce(1),
            Julian::from_ymd(1, 1, 1)
        );
        assert_eq!(
            Julian::from_num_days_from_julian_ce(0),
            Julian::from_ymd(0, 12, 31)
        );
    }

    #[test]
    fn test_julian_ymd_ce_inverse() {
        for &(jy, jm, jd, _, _, _) in &CONVERSION_TABLE as &[_] {
            let julian = Julian::from_ymd(jy, jm, jd);
            let julian2 = Julian::from_num_days_from_julian_ce(julian.num_days_from_julian_ce());
            assert_eq!(julian, julian2);
        }
    }

    #[test]
    fn test_julian_ce_ymd_inverse() {
        for &days in [
            0, 1, 2, 3, 5, 10, 30, 50, 100, 300, 1000, 3000, 5000, 8000, 10000, 20000, 30000, 40000,
        ]
        .iter()
        {
            let days2 = Julian::from_num_days_from_julian_ce(days).num_days_from_julian_ce();
            assert_eq!(days, days2);
        }
    }

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

    #[test]
    fn test_min_max_dates() {
        let date: Julian = chrono::naive::MIN_DATE.into();
        assert_eq!((date.year(), date.month(), date.day()), MIN_YMD);
        assert_eq!((date.year(), date.ordinal()), MIN_YO);
        assert_eq!(date.num_days_from_julian_ce(), MIN_DAYS);
        let date: Julian = chrono::naive::MAX_DATE.into();
        assert_eq!((date.year(), date.month(), date.day()), MAX_YMD);
        assert_eq!((date.year(), date.ordinal()), MAX_YO);
        assert_eq!(date.num_days_from_julian_ce(), MAX_DAYS);
    }
}
