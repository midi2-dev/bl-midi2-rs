#![no_main]

use libfuzzer_sys::fuzz_target;
use midi2::{prelude::*, sysex8::*};

fuzz_target!(|data: &[u8]| {
    let mut message = Sysex8::<Vec<u32>>::new();
    message.set_payload(data.iter().cloned());

    // payload is unchanged
    let payload = message.payload().collect::<Vec<u8>>();
    assert_eq!(payload, data.to_vec());

    // message is in a valid state
    let mut buffer = Vec::new();
    buffer.extend_from_slice(message.data());
    let _ = Sysex8::try_from(&buffer[..]).expect("Valid data");
});
