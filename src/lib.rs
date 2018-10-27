#![no_std]

extern crate bit_field;
extern crate generic_array;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate matches;
#[cfg(test)]
extern crate num;
#[cfg(test)]
#[macro_use]
extern crate std;
extern crate typenum;

mod circular_bit_buffer;
pub(crate) use self::circular_bit_buffer::CircularBitBuffer;

mod dcc_bit;
pub(crate) use self::dcc_bit::DccBit;

mod dcc_bit_decoder;
pub use self::dcc_bit_decoder::DccBitDecoder;

mod dcc_bit_encoder;
pub use self::dcc_bit_encoder::DccBitEncoder;

mod dcc_bit_timing_constraints;
pub(crate) use self::dcc_bit_timing_constraints::DccBitTimingConstraints;

mod dcc_encoded_bit;
pub use self::dcc_encoded_bit::DccEncodedBit;

mod polarity;
pub use self::polarity::Polarity;

mod range_inclusive_ext;
pub(crate) use self::range_inclusive_ext::RangeInclusiveExt;
