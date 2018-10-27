use core::time::Duration;
use lz_circular_bit_buffer::CircularBitBuffer;
use typenum::U1;
use {DccBitDecoder, Polarity};

#[derive(Debug, Default)]
pub struct BufferedDccBitDecoder {
    inner_decoder: DccBitDecoder,
    buffer: CircularBitBuffer<u32, U1>,
}

impl BufferedDccBitDecoder {
    pub fn on_polarity_change(
        &mut self,
        to_polarity: Polarity,
        time_since_previous_change: Duration,
    ) {
        if let Some(bit) = self
            .inner_decoder
            .on_polarity_change(to_polarity, time_since_previous_change)
        {
            self.buffer.push_bit(bit);
        }
    }

    pub fn dequeue_bit(&mut self) -> Option<bool> {
        self.buffer.dequeue_bit()
    }
}

#[cfg(test)]
mod tests {
    use super::{BufferedDccBitDecoder, Polarity};
    use core::time::Duration;

    #[test]
    fn dequeue_bit_on_default_returns_none() {
        let mut dcc_bit_decoder = BufferedDccBitDecoder::default();

        assert_matches!(dcc_bit_decoder.dequeue_bit(), None);
    }

    #[test]
    fn on_polarity_change_with_normal_one_bit() {
        let mut dcc_bit_decoder = BufferedDccBitDecoder::default();

        dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(58));
        dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(58));

        assert_matches!(dcc_bit_decoder.dequeue_bit(), Some(true));
    }

    #[test]
    fn on_polarity_change_with_normal_zero_bit() {
        let mut dcc_bit_decoder = BufferedDccBitDecoder::default();

        dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(100));
        dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(100));

        assert_matches!(dcc_bit_decoder.dequeue_bit(), Some(false));
    }

    #[test]
    fn on_polarity_change_with_stretched_zero_bit() {
        let mut dcc_bit_decoder = BufferedDccBitDecoder::default();

        dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(100));
        dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(10000));

        assert_matches!(dcc_bit_decoder.dequeue_bit(), Some(false));
    }

    #[test]
    fn on_polarity_change_with_too_long_halves() {
        let mut dcc_bit_decoder = BufferedDccBitDecoder::default();

        dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(8000));
        dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(8000));

        assert_matches!(dcc_bit_decoder.dequeue_bit(), None);
    }

    #[test]
    fn on_polarity_change_starting_mid_bit() {
        let mut dcc_bit_decoder = BufferedDccBitDecoder::default();

        // the trailing half of a zero bit
        dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(100));

        dcc_bit_decoder.on_polarity_change(Polarity::Positive, Duration::from_micros(58));
        dcc_bit_decoder.on_polarity_change(Polarity::Negative, Duration::from_micros(58));

        assert_matches!(dcc_bit_decoder.dequeue_bit(), Some(true));
    }
}
