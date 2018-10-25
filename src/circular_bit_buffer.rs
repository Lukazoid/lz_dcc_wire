use bit_field::{BitArray, BitField};
use generic_array::{ArrayLength, GenericArray};

#[derive(Debug, Clone)]
pub struct CircularBitBuffer<T, U: ArrayLength<T>> {
    buffer: GenericArray<T, U>,
    index: usize,
    length: usize,
}

impl<T: Default, U: ArrayLength<T>> Default for CircularBitBuffer<T, U> {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            index: 0,
            length: 0,
        }
    }
}

impl<T: BitField, U: ArrayLength<T>> CircularBitBuffer<T, U> {
    pub fn capacity(&self) -> usize {
        self.buffer.bit_length()
    }

    fn increment_index(&mut self) {
        self.index = self.wrap_index(self.index + 1);
    }

    fn wrap_index(&self, index: usize) -> usize {
        index % self.capacity()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn dequeue_bit(&mut self) -> Option<bool> {
        if self.len() == 0 {
            return None;
        }

        let bit = self.buffer.get_bit(self.index);

        self.increment_index();
        self.length -= 1;

        Some(bit)
    }

    pub fn push_bit(&mut self, value: bool) {
        if self.len() == self.capacity() {
            if let Some(dropped_bit) = self.dequeue_bit() {
                warn!("dropping bit {}", if dropped_bit { "1" } else { "0" });
            }
        }

        let index = self.wrap_index(self.index + self.len());
        self.buffer.set_bit(index, value);
        self.length += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::CircularBitBuffer;
    use typenum::{U1, U2};

    #[test]
    fn push_bit_beyond_capacity_overwrites() {
        let mut buffer = CircularBitBuffer::<u32, U1>::default();

        buffer.push_bit(false);

        for _ in 0..buffer.capacity() {
            buffer.push_bit(true);
        }

        assert_matches!(buffer.dequeue_bit(), Some(true));
    }

    #[test]
    fn len_returns_bit_length() {
        let mut buffer = CircularBitBuffer::<u32, U2>::default();
        for n in 0..buffer.capacity() {
            assert_eq!(buffer.len(), n);
            buffer.push_bit(true);
            assert_eq!(buffer.len(), n + 1);
        }
    }

    #[test]
    fn dequeue_bit_with_default_returns_none() {
        let mut buffer = CircularBitBuffer::<u32, U1>::default();

        assert_matches!(buffer.dequeue_bit(), None);
    }

    #[test]
    fn len_with_default_returns_zero() {
        let buffer = CircularBitBuffer::<u32, U1>::default();

        assert_eq!(buffer.len(), 0);
    }
}
