use {DccBit, DccEncodedBit};

#[derive(Debug, Default)]
pub struct DccBitEncoder {}

impl DccBitEncoder {
    pub fn encode_bit(&self, value: bool) -> DccEncodedBit {
        DccEncodedBit::new(value.timing_constraints().half_duration())
    }
}

#[cfg(test)]
mod tests {
    use {DccBitDecoder, DccBitEncoder, Polarity};

    fn encode_decode_test(bit: bool) {
        let dcc_bit_encoder = DccBitEncoder::default();

        let dcc_encoded_bit = dcc_bit_encoder.encode_bit(bit);

        let mut dcc_bit_decoder = DccBitDecoder::default();

        dcc_bit_decoder
            .on_polarity_change(Polarity::Positive, dcc_encoded_bit.first_half_duration());
        dcc_bit_decoder
            .on_polarity_change(Polarity::Negative, dcc_encoded_bit.second_half_duration());

        assert_eq!(dcc_bit_decoder.dequeue_bit(), Some(bit));
    }

    #[test]
    fn encode_one() {
        encode_decode_test(true);
    }

    #[test]
    fn encode_zero() {
        encode_decode_test(false);
    }
}
