pub(crate) trait DivEuclidPolyfillExt {
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

pub(crate) trait ContainsPolyfillExt<Idx> {
    fn contains_polyfill<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: PartialOrd<Idx> + ?Sized;
}

impl<Idx> ContainsPolyfillExt<Idx> for std::ops::RangeInclusive<Idx> {
    fn contains_polyfill<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: PartialOrd<Idx> + ?Sized,
    {
        self.start() <= item && item <= self.end()
    }
}
