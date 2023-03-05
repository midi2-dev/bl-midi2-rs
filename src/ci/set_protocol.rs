use crate::{
    error::Error,
    result::Result,
    message::{
        sysex,
        system_exclusive_8bit as sysex8,
        system_exclusive_7bit as sysex7,
    },
    ci::{
        helpers as ci_helpers,
        DeviceId,
        Protocol,
    },
    util::{Encode7Bit, Truncate},
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SetProtocolMessage<Repr: sysex::SysexMessages>(Repr);

const STATUS: u8 = 0x12;

impl<'a> SetProtocolMessage<sysex8::Sysex8MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> SetProtocolBuilder<sysex8::Sysex8MessageGroup<'a>> {
        SetProtocolBuilder::<sysex8::Sysex8MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        self.0.group()
    }
    pub fn source(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        ux::u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn destination(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(8);
        ux::u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn authority_level(&self) -> ux::u7 {
        let mut payload = self.0.payload();
        payload.nth(ci_helpers::STANDARD_DATA_SIZE).unwrap().truncate()
    }
    pub fn protocol(&self) -> Protocol {
        let mut payload = self.0.payload();
        payload.nth(ci_helpers::STANDARD_DATA_SIZE);
        ci_helpers::read_protocol(payload).unwrap()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex8(data, STATUS)?;
        let mut payload = messages.payload();

        // authority level
        let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE) else {
            return Err(Error::InvalidData);
        };

        ci_helpers::validate_protocol_data(payload.clone())?;
        payload.nth(4);

        let None = payload.next() else {
            return Err(Error::InvalidData);
        };

        Ok(SetProtocolMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}

impl<'a> SetProtocolMessage<sysex7::Sysex7MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> SetProtocolBuilder<sysex7::Sysex7MessageGroup<'a>> {
        SetProtocolBuilder::<sysex7::Sysex7MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        self.0.group()
    }
    pub fn source(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        ux::u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn destination(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(8);
        ux::u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn authority_level(&self) -> ux::u7 {
        let mut payload = self.0.payload();
        payload.nth(ci_helpers::STANDARD_DATA_SIZE).unwrap()
    }
    pub fn protocol(&self) -> Protocol {
        let mut payload = self.0.payload();
        payload.nth(ci_helpers::STANDARD_DATA_SIZE);
        ci_helpers::read_protocol(payload.map(u8::from)).unwrap()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex7(data, STATUS)?;
        let mut payload = messages.payload();

        // authority level
        let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE) else {
            return Err(Error::InvalidData);
        };

        ci_helpers::validate_protocol_data(payload.clone().map(u8::from))?;
        payload.nth(4);

        let None = payload.next() else {
            return Err(Error::InvalidData);
        };

        Ok(SetProtocolMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}


pub struct SetProtocolBuilder<Repr: sysex::SysexMessages> {
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7,
    builder: Repr::Builder,
    protocol: Protocol,
}

impl<'a> SetProtocolBuilder<sysex8::Sysex8MessageGroup<'a>> {
    pub fn group(&mut self, g: ux::u4) -> &mut Self {
        self.builder.group(g);
        self
    }
    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        self.builder.stream_id(id);
        self
    }
    pub fn source(&mut self, source: ux::u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn destination(&mut self, dest: ux::u28) -> &mut Self {
        self.destination = dest;
        self
    }
    pub fn authority_level(&mut self, auth: ux::u7) -> &mut Self{
        self.authority_level = auth;
        self
    }
    pub fn protocol(&mut self, protocol: Protocol) -> &mut Self {
        self.protocol = protocol;
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        SetProtocolBuilder {
            builder: sysex8::Sysex8MessageGroupBuilder::new(buffer),
            destination: Default::default(),
            source: Default::default(),
            authority_level: Default::default(),
            protocol: Protocol::Midi1 {
                size_of_packet_extension: false,
                jitter_reduction_extension: false,
                version: Protocol::MIDI_1_VERSION,
            },
        }
    }
    pub fn build(&'a mut self) -> Result<SetProtocolMessage<sysex8::Sysex8MessageGroup<'a>>> {
        match self.builder.payload(
            ci_helpers::StandardDataIterator::new(
                DeviceId::MidiPort,
                STATUS,
                self.source,
                self.destination,
            )
            .chain([self.authority_level.into()].iter().copied())
            .chain(ci_helpers::protocol_data(&self.protocol))
        ).build() {
            Ok(messages) => Ok(SetProtocolMessage(messages)),
            Err(e) => Err(e)
        }            
    }
}

impl<'a> SetProtocolBuilder<sysex7::Sysex7MessageGroup<'a>> {
    pub fn group(&mut self, g: ux::u4) -> &mut Self {
        self.builder.group(g);
        self
    }
    pub fn source(&mut self, source: ux::u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn destination(&mut self, dest: ux::u28) -> &mut Self {
        self.destination = dest;
        self
    }
    pub fn authority_level(&mut self, auth: ux::u7) -> &mut Self{
        self.authority_level = auth;
        self
    }
    pub fn protocol(&mut self, protocol: Protocol) -> &mut Self {
        self.protocol = protocol;
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        SetProtocolBuilder {
            builder: sysex7::Sysex7MessageGroupBuilder::new(buffer),
            destination: Default::default(),
            source: Default::default(),
            authority_level: Default::default(),
            protocol: Protocol::Midi1 {
                size_of_packet_extension: false,
                jitter_reduction_extension: false,
                version: Protocol::MIDI_1_VERSION,
            },
        }
    }
    pub fn build(&'a mut self) -> Result<SetProtocolMessage<sysex7::Sysex7MessageGroup<'a>>> {
        match self.builder.payload(
            ci_helpers::StandardDataIterator::new(
                DeviceId::MidiPort,
                STATUS,
                self.source,
                self.destination,
            )
            .map(ux::u7::new)
            .chain([self.authority_level].iter().copied())
            .chain(ci_helpers::protocol_data(&self.protocol).map(ux::u7::new))
        ).build() {
            Ok(messages) => Ok(SetProtocolMessage(messages)),
            Err(e) => Err(e)
        }            
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        util::debug,
        ci::protocol,
    };
    
    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(SetProtocolMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                .group(ux::u4::new(0x5))
                .stream_id(0xA1)
                .source(ux::u28::new(126104616))
                .destination(ux::u28::new(124848818))
                .authority_level(ux::u7::new(0x3C))
                .protocol(protocol::Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: protocol::Protocol::MIDI_2_VERSION,
                })
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x551E_A17E,
                0x7F0D_1201,
                0x2868_103C,
                0x3215_443B,
                0x5537_A13C,
                0x0200_0100,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(SetProtocolMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 8])
                .group(ux::u4::new(0x5))
                .source(ux::u28::new(126104616))
                .destination(ux::u28::new(124848818))
                .authority_level(ux::u7::new(0x3C))
                .protocol(protocol::Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: protocol::Protocol::MIDI_2_VERSION,
                })
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x3516_7E7F,
                0x0D12_0128,
                0x3526_6810,
                0x3C32_1544,
                0x3526_3B3C,
                0x0200_0100,
                0x3531_0000,
                0x0000_0000,
            ]),
        );
    }
    
    #[test]
    fn sysex8_from_data() {
        assert!(SetProtocolMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
            0x551E_A17E,
            0x7F0D_1201,
            0x2868_103C,
            0x3215_443B,
            0x5537_A13C,
            0x0200_0100,
            0x0000_0000,
            0x0000_0000,
        ]).is_ok());
    }

    #[test]
    fn sysex7_from_data() {
        assert!(SetProtocolMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
            0x3516_7E7F,
            0x0D12_0128,
            0x3526_6810,
            0x3C32_1544,
            0x3526_3B3C,
            0x0200_0100,
            0x3531_0000,
            0x0000_0000,
        ]).is_ok());
    }

    #[test]
    fn sysex8_group() {
        assert_eq!(
            SetProtocolMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                .group(ux::u4::new(0x6))
                .build()
                .unwrap()
                .group(),
            ux::u4::new(0x6),
        );
    }

    #[test]
    fn sysex7_group() {
        assert_eq!(
            SetProtocolMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 8])
                .group(ux::u4::new(0x6))
                .build()
                .unwrap()
                .group(),
            ux::u4::new(0x6),
        );
    }

    #[test]
    fn sysex8_source() {
        assert_eq!(
            SetProtocolMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                .source(ux::u28::new(250850768))
                .build()
                .unwrap()
                .source(),
            ux::u28::new(250850768),
        );
    }

    #[test]
    fn sysex7_source() {
        assert_eq!(
            SetProtocolMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 8])
                .source(ux::u28::new(250850768))
                .build()
                .unwrap()
                .source(),
            ux::u28::new(250850768),
        );
    }

    #[test]
    fn sysex8_destination() {
        assert_eq!(
            SetProtocolMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                .destination(ux::u28::new(250850768))
                .build()
                .unwrap()
                .destination(),
            ux::u28::new(250850768),
        );
    }

    #[test]
    fn sysex7_destination() {
        assert_eq!(
            SetProtocolMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 8])
                .destination(ux::u28::new(250850768))
                .build()
                .unwrap()
                .destination(),
            ux::u28::new(250850768),
        );
    }

    #[test]
    fn sysex8_protocol() {
        assert_eq!(
            SetProtocolMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                .protocol(protocol::Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: Protocol::MIDI_2_VERSION,
                })
                .build()
                .unwrap()
                .protocol(),
            protocol::Protocol::Midi2 {
                jitter_reduction_extension: true,
                version: Protocol::MIDI_2_VERSION,
            },
        );
    }

    #[test]
    fn sysex7_protocol() {
        assert_eq!(
            SetProtocolMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 8])
                .protocol(protocol::Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: Protocol::MIDI_2_VERSION,
                })
                .build()
                .unwrap()
                .protocol(),
            protocol::Protocol::Midi2 {
                jitter_reduction_extension: true,
                version: Protocol::MIDI_2_VERSION,
            },
        );
    }
}