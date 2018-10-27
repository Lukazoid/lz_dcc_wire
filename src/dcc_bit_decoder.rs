use core::time::Duration;
use {DccBit, DccBitTimingConstraints, Polarity};

#[derive(Debug, Default)]
pub struct DccBitDecoder {
    state: State,
}

impl DccBitDecoder {
    pub fn on_polarity_change(
        &mut self,
        to_polarity: Polarity,
        time_since_previous_change: Duration,
    ) -> Option<bool> {
        let half_bit_reading =
            HalfBitReading::from_polarity_change(to_polarity, time_since_previous_change);
        if let Some(half_bit_reading) = half_bit_reading {
            match self.state {
                State::Pending => {
                    self.state = State::AfterFirstChange(half_bit_reading);
                    None
                }
                State::AfterFirstChange(first_half_bit_reading) => {
                    let second_half_bit_reading = half_bit_reading;

                    if let Some(bit_reading) =
                        BitReading::from_two_halves(first_half_bit_reading, second_half_bit_reading)
                    {
                        self.state = State::Pending;
                        Some(bit_reading.bit())
                    } else {
                        // if for any reason the first and second half don't correlate then we
                        // disregard the first half and continue by treating the second half
                        // as the new first half
                        self.state = State::AfterFirstChange(second_half_bit_reading);
                        None
                    }
                }
            }
        } else {
            // if this polarity change occurred over a duration too quick or slow to be a zero or
            // one bit then we will disregard it.
            // Also clear the state as it means we aren't actually in the middle of reading a bit.
            self.state = State::Pending;
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    Pending,
    AfterFirstChange(HalfBitReading),
}

impl Default for State {
    fn default() -> Self {
        State::Pending
    }
}

#[derive(Debug, Clone, Copy)]
struct BitReading {
    first_half: HalfBitReading,
    second_half: HalfBitReading,
}

impl BitReading {
    pub fn from_two_halves(
        first_half: HalfBitReading,
        second_half: HalfBitReading,
    ) -> Option<Self> {
        if first_half.to_polarity != second_half.to_polarity
            && first_half.possible_bit == second_half.possible_bit
            && first_half.timing_constraints().are_valid_durations(
                first_half.time_since_previous_change,
                second_half.time_since_previous_change,
            ) {
            Some(BitReading {
                first_half,
                second_half,
            })
        } else {
            None
        }
    }

    pub fn bit(&self) -> bool {
        self.first_half.possible_bit
    }
}

#[derive(Debug, Clone, Copy)]
struct HalfBitReading {
    to_polarity: Polarity,
    time_since_previous_change: Duration,
    possible_bit: bool,
}

impl HalfBitReading {
    pub fn from_polarity_change(
        to_polarity: Polarity,
        time_since_previous_change: Duration,
    ) -> Option<Self> {
        bool::from_half_duration(time_since_previous_change).map(|possible_bit| HalfBitReading {
            to_polarity,
            time_since_previous_change,
            possible_bit,
        })
    }

    pub fn timing_constraints(&self) -> &'static DccBitTimingConstraints {
        self.possible_bit.timing_constraints()
    }
}

#[cfg(test)]
mod tests {
    use super::{DccBitDecoder, Polarity};
    use core::time::Duration;

    #[test]
    fn on_polarity_change_with_normal_one_bit() {
        let mut dcc_bit_decoder = DccBitDecoder::default();

        let first_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(58));
        assert_matches!(first_change, None);

        let second_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(58));
        assert_matches!(second_change, Some(true));
    }

    #[test]
    fn on_polarity_change_with_normal_zero_bit() {
        let mut dcc_bit_decoder = DccBitDecoder::default();

        let first_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(100));
        assert_matches!(first_change, None);

        let second_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(100));
        assert_matches!(second_change, Some(false));
    }

    #[test]
    fn on_polarity_change_with_stretched_zero_bit() {
        let mut dcc_bit_decoder = DccBitDecoder::default();

        let first_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(100));
        assert_matches!(first_change, None);

        let second_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(10000));
        assert_matches!(second_change, Some(false));
    }

    #[test]
    fn on_polarity_change_with_too_long_halves() {
        let mut dcc_bit_decoder = DccBitDecoder::default();

        let first_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(8000));
        assert_matches!(first_change, None);

        let second_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(8000));
        assert_matches!(second_change, None);
    }

    #[test]
    fn on_polarity_change_starting_mid_bit() {
        let mut dcc_bit_decoder = DccBitDecoder::default();

        // the trailing half of a zero bit
        let first_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(100));
        assert_matches!(first_change, None);

        let second_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(58));
        assert_matches!(second_change, None);

        let third_change =
            dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(58));
        assert_matches!(third_change, Some(true));
    }
}
