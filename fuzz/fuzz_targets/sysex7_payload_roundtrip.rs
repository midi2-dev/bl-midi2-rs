#![no_main]

use libfuzzer_sys::fuzz_target;
use midi2::{prelude::*, sysex7::*};

fuzz_target!(|data: &[u8]| {
    let to_u7 = |b: u8| u7::new(b & 0x7F);
    let mut message = Sysex7::<Vec<u32>>::new();
    message.set_payload(data.iter().cloned().map(to_u7));

    // payload is unchanged
    let payload = message.payload().collect::<Vec<u7>>();
    assert_eq!(
        payload,
        data.iter().cloned().map(to_u7).collect::<Vec<u7>>()
    );

    // message is in a valid state
    let mut buffer = Vec::new();
    buffer.extend_from_slice(message.data());
    let _ = Sysex7::try_from(&buffer[..]).expect("Valid data");
});
