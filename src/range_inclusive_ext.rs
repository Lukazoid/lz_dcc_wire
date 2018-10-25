use core::ops::RangeInclusive;

pub trait RangeInclusiveExt<T> {
    fn range_contains(&self, value: &T) -> bool
    where
        T: PartialOrd;
}

impl<T> RangeInclusiveExt<T> for RangeInclusive<T> {
    fn range_contains(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        value >= self.start() && value <= self.end()
    }
}
