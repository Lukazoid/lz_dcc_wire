use core::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct DccEncodedBit {
    duration: Duration,
}

impl DccEncodedBit {
    pub(crate) fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub fn first_half_duration(&self) -> Duration {
        self.duration
    }

    pub fn second_half_duration(&self) -> Duration {
        self.duration
    }
}
