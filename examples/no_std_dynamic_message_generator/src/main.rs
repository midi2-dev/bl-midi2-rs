#![no_std]

extern crate std;

use midi2::{channel_voice1::*, prelude::*, sysex7::*};

#[derive(Clone, Copy, Debug)]
struct Buffer(*const [u8]);

impl midi2::buffer::Buffer for Buffer {
    type Unit = u8;
    fn buffer(&self) -> &[Self::Unit] {
        unsafe { &*self.0 }
    }
}

impl midi2::buffer::BufferTryResize for Buffer {
    fn try_resize(&mut self, new_len: usize) -> Result<(), midi2::error::BufferOverflow> {
        if new_len < self.0.len() {
            return Err(midi2::error::BufferOverflow);
        }
        Ok(())
    }
}

impl midi2::buffer::FromBuffer<&mut [u8]> for Buffer {
    fn from_buffer(buffer: &mut [u8]) -> Self {
        let ptr: *const u8 = buffer.as_ptr();
        let len = buffer.len();
        Buffer(core::ptr::slice_from_raw_parts(ptr, len))
    }
}

type Message = BytesMessage<Buffer>;

struct MessageIterator<'a> {
    messages: &'a [Option<Message>],
    index: usize,
}

impl Iterator for MessageIterator<'_> {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.messages.len() {
            let index = self.index;
            self.index += 1;
            Some(*self.messages[index].as_ref().unwrap())
        } else {
            None
        }
    }
}

struct Generator {
    buffer: [u8; 128],
    messages: [Option<Message>; Self::MAX_MESSAGES],
}

impl Generator {
    const MAX_MESSAGES: usize = 16;

    fn new() -> Self {
        Self {
            buffer: [0x0; 128],
            messages: [None; Self::MAX_MESSAGES],
        }
    }

    fn generate(&mut self) -> Result<MessageIterator, midi2::error::BufferOverflow> {
        let mut number_of_messages = 0;
        let buffer = &mut self.buffer[..];

        // create some channel voice messages

        let (message_buffer, buffer) = buffer.split_at_mut(2);
        let mut channel_pressure = ChannelPressure::try_new_with_buffer(message_buffer)?;
        channel_pressure.set_pressure(u7::new(0x50));
        self.messages[number_of_messages] =
            Some(ChannelPressure::<Buffer>::rebuffer_from(channel_pressure).into());
        number_of_messages += 1;

        let (message_buffer, buffer) = buffer.split_at_mut(3);
        let mut note_on = NoteOn::try_new_with_buffer(message_buffer)?;
        note_on.set_note_number(u7::new(0x38));
        note_on.set_velocity(u7::new(0x20));
        self.messages[number_of_messages] = Some(NoteOn::<Buffer>::rebuffer_from(note_on).into());
        number_of_messages += 1;

        let (message_buffer, buffer) = buffer.split_at_mut(3);
        let mut control_change = ControlChange::try_new_with_buffer(message_buffer)?;
        control_change.set_control(u7::new(0x34));
        control_change.set_control_data(u7::new(0x2A));
        self.messages[number_of_messages] =
            Some(ControlChange::<Buffer>::rebuffer_from(control_change).into());
        number_of_messages += 1;

        // and a sysex

        let (message_buffer, _) = buffer.split_at_mut(22);
        let mut sysex = Sysex7::try_new_with_buffer(message_buffer)?;
        sysex.try_set_payload((0..20).map(u7::new))?;
        self.messages[number_of_messages] = Some(Sysex7::<Buffer>::rebuffer_from(sysex).into());
        number_of_messages += 1;

        Ok(MessageIterator {
            messages: &self.messages[0..number_of_messages],
            index: 0,
        })
    }
}

fn main() {
    let mut generator = Generator::new();
    for message in generator.generate().unwrap() {
        std::println!("{message:?}");
    }
}
