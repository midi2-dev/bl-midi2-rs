#![no_main]

use libfuzzer_sys::fuzz_target;
use midi2::Sysex;
use rand::{Rng, SeedableRng};

struct FixedSizeBuffer<U: midi2::buffer::Unit>(Vec<U>);

impl<U: midi2::buffer::Unit> midi2::buffer::Buffer for FixedSizeBuffer<U> {
    type Unit = U;
    fn buffer(&self) -> &[Self::Unit] {
        &self.0
    }
}

impl<U: midi2::buffer::Unit> midi2::buffer::BufferMut for FixedSizeBuffer<U> {
    fn buffer_mut(&mut self) -> &mut [Self::Unit] {
        &mut self.0
    }
}

impl<U: midi2::buffer::Unit> midi2::buffer::BufferTryResize for FixedSizeBuffer<U> {
    fn try_resize(&mut self, new_size: usize) -> Result<(), midi2::error::BufferOverflow> {
        if new_size > self.0.len() {
            return Err(midi2::error::BufferOverflow);
        }
        Ok(())
    }
}

impl<U: midi2::buffer::Unit> FixedSizeBuffer<U> {
    fn new(size: usize) -> Self {
        Self(std::iter::repeat_n(U::zero(), size).collect())
    }
}

#[derive(arbitrary::Arbitrary, Debug)]
struct InputData {
    seed: u64,
    initial_data: Vec<u8>,
    data_to_insert: Vec<u8>,
}

const MAX_BUFFER_SIZE: usize = 1024;

trait IntoByte<B> {
    fn byte(&self) -> B;
}

impl IntoByte<midi2::ux::u7> for u8 {
    fn byte(&self) -> midi2::ux::u7 {
        midi2::num::u7::new(self & 0x7F)
    }
}

impl IntoByte<u8> for u8 {
    fn byte(&self) -> u8 {
        *self
    }
}

fn test_case<B, M>(data: &InputData, mut message: M, index: usize)
where
    B: midi2::buffer::Buffer + midi2::buffer::BufferTryResize + midi2::buffer::BufferMut,
    M: midi2::Sysex<B>,
    <M as Sysex<B>>::Byte: Eq + core::fmt::Debug,
    u8: IntoByte<<M as Sysex<B>>::Byte>,
{
    let Ok(()) = message.try_set_payload(data.initial_data.iter().map(u8::byte)) else {
        return;
    };

    {
        let initial = message.payload().collect::<Vec<_>>();
        assert_eq!(
            initial,
            data.initial_data.iter().map(u8::byte).collect::<Vec<_>>()
        );
    }

    let Ok(()) = message.try_insert_payload(data.data_to_insert.iter().map(u8::byte), index) else {
        return;
    };

    let actual = message.payload().collect::<Vec<_>>();
    let expected = {
        let mut ret = data.initial_data.clone();
        ret.splice(index..index, data.data_to_insert.clone());
        ret.iter().map(u8::byte).collect::<Vec<_>>()
    };
    assert_eq!(actual, expected);
}

fuzz_target!(|data: InputData| {
    let mut rng = rand::rngs::StdRng::seed_from_u64(data.seed);
    let fized_size_buffer_size = rng.random_range(4..MAX_BUFFER_SIZE);
    let index = if data.initial_data.is_empty() {
        0
    } else {
        rng.random_range(0..data.initial_data.len())
    };
    test_case(
        &data,
        midi2::sysex8::Sysex8::<FixedSizeBuffer<u32>>::try_new_with_buffer(
            FixedSizeBuffer::<u32>::new(fized_size_buffer_size),
        )
        .unwrap(),
        index,
    );
    test_case(
        &data,
        midi2::sysex7::Sysex7::<FixedSizeBuffer<u32>>::try_new_with_buffer(
            FixedSizeBuffer::<u32>::new(fized_size_buffer_size),
        )
        .unwrap(),
        index,
    );
    test_case(
        &data,
        midi2::sysex7::Sysex7::<FixedSizeBuffer<u8>>::try_new_with_buffer(
            FixedSizeBuffer::<u8>::new(fized_size_buffer_size),
        )
        .unwrap(),
        index,
    );
});
