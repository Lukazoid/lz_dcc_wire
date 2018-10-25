use core::time::Duration;
use DccBitTimingConstraints;

lazy_static! {
    static ref ONE_TIMINGS: DccBitTimingConstraints = DccBitTimingConstraints::for_one_bit();
    static ref ZERO_TIMINGS: DccBitTimingConstraints = DccBitTimingConstraints::for_zero_bit();
}

pub trait DccBit {
    fn timing_constraints(&self) -> &'static DccBitTimingConstraints;

    fn from_half_duration(duration: Duration) -> Option<Self>
    where
        Self: Sized;
}

impl DccBit for bool {
    fn timing_constraints(&self) -> &'static DccBitTimingConstraints {
        if *self {
            &*ONE_TIMINGS
        } else {
            &*ZERO_TIMINGS
        }
    }

    fn from_half_duration(duration: Duration) -> Option<Self> {
        if ONE_TIMINGS.is_valid_half_duration(duration) {
            Some(true)
        } else if ZERO_TIMINGS.is_valid_half_duration(duration) {
            Some(false)
        } else {
            None
        }
    }
}
