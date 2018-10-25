use core::ops::RangeInclusive;
use core::time::Duration;
use RangeInclusiveExt;

#[derive(Debug, Clone)]
pub struct DccBitTimingConstraints {
    half_bounds: RangeInclusive<Duration>,
    half_nominal: Duration,
    bounds: Option<RangeInclusive<Duration>>,
}

impl DccBitTimingConstraints {
    pub fn for_zero_bit() -> Self {
        Self {
            half_bounds: Duration::from_micros(90)..=Duration::from_micros(10000),
            half_nominal: Duration::from_micros(100),
            bounds: Some(Duration::from_micros(90)..=Duration::from_micros(12000)),
        }
    }

    pub fn for_one_bit() -> Self {
        Self {
            half_bounds: Duration::from_micros(52)..=Duration::from_micros(64),
            half_nominal: Duration::from_micros(58),
            bounds: None,
        }
    }

    pub fn half_duration(&self) -> Duration {
        self.half_nominal
    }

    pub fn is_valid_half_duration(&self, half_duration: Duration) -> bool {
        self.half_bounds.range_contains(&half_duration)
    }

    pub fn are_valid_durations(
        &self,
        first_half_duration: Duration,
        second_half_duration: Duration,
    ) -> bool {
        if !self.is_valid_half_duration(first_half_duration)
            || !self.is_valid_half_duration(second_half_duration)
        {
            return false;
        }

        if let Some(ref bounds) = self.bounds {
            let total_duration = first_half_duration + second_half_duration;

            if !bounds.range_contains(&total_duration) {
                return false;
            }
        }

        true
    }
}
