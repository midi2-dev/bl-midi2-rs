#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protocol {
    Midi1 {
        size_of_packet_extension: bool,
        jitter_reduction_extension: bool,
        version: ux::u7,
    },
    Midi2 {
        jitter_reduction_extension: bool,
        version: ux::u7,
    },
}

impl Protocol {
    pub const MIDI_1_VERSION: ux::u7 = ux::u7::new(0x0);
    pub const MIDI_2_VERSION: ux::u7 = ux::u7::new(0x0);
}
