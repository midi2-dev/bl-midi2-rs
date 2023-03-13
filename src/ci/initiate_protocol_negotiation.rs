        
pub struct ProtocolIterator {}

macro_rules! initiate_protocol_negotiation_message {
    ($op_code:expr) => {
        use crate::{
            result::Result,
            error::Error,
            message::{
                sysex, 
                system_exclusive_8bit as sysex8,
                system_exclusive_7bit as sysex7,
            },
            ci::{
                helpers as ci_helpers,
                DeviceId, Protocol,
                initiate_protocol_negotiation::ProtocolIterator,
            },
        };


        #[derive(Clone, PartialEq, Eq, Debug)]
        pub struct InitiateProtocolNegotiationMessage<Repr: sysex::SysexMessages>(Repr);

        const STATUS: u8 = $op_code;
        impl<'a> InitiateProtocolNegotiationMessage<sysex8::Sysex8MessageGroup<'a>> {
            pub fn builder(buffer: &'a mut [u32]) -> InitiateProtocolNegotiationBuilder<sysex8::Sysex8MessageGroup<'a>> {
                InitiateProtocolNegotiationBuilder::<sysex8::Sysex8MessageGroup<'a>>::new(buffer)
            }
            pub fn group(&self) -> ux::u4 {
                self.0.group()
            }
            pub fn source(&self) -> ux::u28 {
                ci_helpers::source_from_payload(self.0.payload())
            }
            pub fn destination(&self) -> ux::u28 {
                ci_helpers::destination_from_payload(self.0.payload())
            }
            pub fn authority_level(&self) -> ux::u7 {
                ci_helpers::authority_level_from_payload(self.0.payload())
            }
            pub fn protocols(&self) -> ProtocolIterator {
                todo!()
            }
            pub fn from_data(data: &'a [u32]) -> Result<Self> {
                let messages = ci_helpers::validate_sysex8(data, STATUS)?;
                let mut payload = messages.payload();

                // authority level
                let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE) else {
                    return Err(Error::InvalidData);
                };

                todo!();

                Ok(InitiateProtocolNegotiationMessage(messages))
            }
            pub fn data(&self) -> &[u32] {
                self.0.data()
            }
        }

        impl<'a> InitiateProtocolNegotiationMessage<sysex7::Sysex7MessageGroup<'a>> {
            pub fn builder(buffer: &'a mut [u32]) -> InitiateProtocolNegotiationBuilder<sysex7::Sysex7MessageGroup<'a>> {
                InitiateProtocolNegotiationBuilder::<sysex7::Sysex7MessageGroup<'a>>::new(buffer)
            }
            pub fn group(&self) -> ux::u4 {
                self.0.group()
            }
            pub fn source(&self) -> ux::u28 {
                ci_helpers::source_from_payload(self.0.payload().map(u8::from))
            }
            pub fn destination(&self) -> ux::u28 {
                ci_helpers::destination_from_payload(self.0.payload().map(u8::from))
            }
            pub fn authority_level(&self) -> ux::u7 {
                ci_helpers::authority_level_from_payload(self.0.payload().map(u8::from))
            }
            pub fn protocols(&self) -> ProtocolIterator {
                todo!()
            }
            pub fn from_data(data: &'a [u32]) -> Result<Self> {
                let messages = ci_helpers::validate_sysex7(data, STATUS)?;
                let mut payload = messages.payload();

                // authority level
                let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE) else {
                    return Err(Error::InvalidData);
                };

                todo!();

                Ok(InitiateProtocolNegotiationMessage(messages))
            }
            
            pub fn data(&self) -> &[u32] {
                self.0.data()
            }
        }


        pub struct InitiateProtocolNegotiationBuilder<Repr: sysex::SysexMessages> {
            source: ux::u28,
            destination: ux::u28,
            authority_level: ux::u7,
            builder: Result<Repr::Builder>,
            protocols: [Option<Protocol>; 2],
        }

        impl<'a> InitiateProtocolNegotiationBuilder<sysex8::Sysex8MessageGroup<'a>> {
            pub fn group(&mut self, g: ux::u4) -> &mut Self {
                if let Ok(builder) = &mut self.builder {
                    builder.group(g);
                }
                self
            }
            pub fn stream_id(&mut self, id: u8) -> &mut Self {
                if let Ok(builder) = &mut self.builder {
                    builder.stream_id(id);
                }
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
            /// amends an additional protocol for each call.
            ///
            /// Warning! only two protocols are currently supported.
            /// the build will fail if this method is called more than twice for a single
            /// builder.
            pub fn protocol(&mut self, protocol: Protocol) -> &mut Self {
                if let Some((idx, _)) = self.protocols.iter().enumerate().find(|(_, &opt)| opt.is_none()) {
                    self.protocols[idx] = Some(protocol);
                } else {
                    self.builder = Err(Error::InvalidData);
                }
                self
            }
            fn new(buffer: &'a mut [u32]) -> Self {
                InitiateProtocolNegotiationBuilder {
                    builder: Ok(sysex8::Sysex8MessageGroupBuilder::new(buffer)),
                    destination: Default::default(),
                    source: Default::default(),
                    authority_level: Default::default(),
                    protocols: Default::default(),
                }
            }
            pub fn build(&'a mut self) -> Result<InitiateProtocolNegotiationMessage<sysex8::Sysex8MessageGroup<'a>>> {
                if let None = self.protocols[0] {
                    return Err(Error::InvalidData);
                }

                if let Err(e) = &self.builder {
                    return Err(e.clone());
                };
                
                let Ok(builder) = &mut self.builder else {
                    unreachable!();
                };

                let payload = ci_helpers::StandardDataIterator::new(
                    DeviceId::MidiPort,
                    STATUS,
                    self.source,
                    self.destination,
                );

                let payload = payload.chain(core::iter::once(u8::from(self.authority_level)));
                // number of supported protocols
                let payload = payload.chain(core::iter::once(self.protocols.iter().map(|o| -> u8 { if o.is_none() { 0 } else { 1 }}).sum()));
                let payload = payload.chain(ci_helpers::protocol_data(self.protocols[0].as_ref().unwrap()));

                if let Some(aux_protocol) = &self.protocols[1] {
                    let payload = payload.chain(ci_helpers::protocol_data(aux_protocol));
                    match builder.payload(payload).build() {
                        Ok(messages) => Ok(InitiateProtocolNegotiationMessage(messages)),
                        Err(e) => Err(e),
                    }
                } else {
                    match builder.payload(payload).build() {
                        Ok(messages) => Ok(InitiateProtocolNegotiationMessage(messages)),
                        Err(e) => Err(e),
                    }
                }
            }
        }

        impl<'a> InitiateProtocolNegotiationBuilder<sysex7::Sysex7MessageGroup<'a>> {
            pub fn group(&mut self, g: ux::u4) -> &mut Self {
                if let Ok(builder) = &mut self.builder {
                    builder.group(g);
                }
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
                if let Some((idx, _)) = self.protocols.iter().enumerate().find(|(_, &opt)| opt.is_none()) {
                    self.protocols[idx] = Some(protocol);
                } else {
                    self.builder = Err(Error::InvalidData);
                }
                self
            }
            fn new(buffer: &'a mut [u32]) -> Self {
                InitiateProtocolNegotiationBuilder {
                    builder: Ok(sysex7::Sysex7MessageGroupBuilder::new(buffer)),
                    destination: Default::default(),
                    source: Default::default(),
                    authority_level: Default::default(),
                    protocols: Default::default(),
                }
            }
            pub fn build(&'a mut self) -> Result<InitiateProtocolNegotiationMessage<sysex7::Sysex7MessageGroup<'a>>> {
                if let None = self.protocols[0] {
                    return Err(Error::InvalidData);
                }

                if let Err(e) = &self.builder {
                    return Err(e.clone());
                };
                
                let Ok(builder) = &mut self.builder else {
                    unreachable!();
                };

                let payload = ci_helpers::StandardDataIterator::new(
                    DeviceId::MidiPort,
                    STATUS,
                    self.source,
                    self.destination,
                );

                let payload = payload.chain(core::iter::once(u8::from(self.authority_level)));
                // number of supported protocols
                let payload = payload.chain(core::iter::once(self.protocols.iter().map(|o| -> u8 { if o.is_none() { 0 } else { 1 }}).sum()));
                let payload = payload.chain(ci_helpers::protocol_data(self.protocols[0].as_ref().unwrap()));

                if let Some(aux_protocol) = &self.protocols[1] {
                    let payload = payload.chain(ci_helpers::protocol_data(aux_protocol));
                    match builder.payload(payload.map(ux::u7::new)).build() {
                        Ok(messages) => Ok(InitiateProtocolNegotiationMessage(messages)),
                        Err(e) => Err(e),
                    }
                } else {
                    match builder.payload(payload.map(ux::u7::new)).build() {
                        Ok(messages) => Ok(InitiateProtocolNegotiationMessage(messages)),
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }
}

pub mod query {
    initiate_protocol_negotiation_message!(0x10);
}

//pub mod reply {
//    initiate_protocol_negotiation_message!(0x11);
//}

#[cfg(test)]
mod tests {
    use crate::{
        message::system_exclusive_7bit as sysex7,
        message::system_exclusive_8bit as sysex8,
    };
    
    use super::query::*;
    use crate::{
        ci::protocol,
        util::debug,
    };

    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(InitiateProtocolNegotiationMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                .group(ux::u4::new(0x4))
                .stream_id(0x14)
                .source(ux::u28::new(0x5FF9751))
                .destination(ux::u28::new(0x562F000))
                .authority_level(ux::u7::new(0x6C))
                .protocol(protocol::Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: protocol::Protocol::MIDI_2_VERSION,
                })
                .protocol(protocol::Protocol::Midi1 {
                    jitter_reduction_extension: true,
                    size_of_packet_extension: true,
                    version: protocol::Protocol::MIDI_1_VERSION,
                })
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x541E_147E,
                0x7F0D_1001,
                0x512E_7E2F,
                0x0060_0B2B,
                0x543D_146C,
                0x0202_0001,
                0x0000_0100,
                0x0300_0000
            ]),
        );
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(InitiateProtocolNegotiationMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 10])
                .group(ux::u4::new(0x4))
                .source(ux::u28::new(0x5FF9751))
                .destination(ux::u28::new(0x562F000))
                .authority_level(ux::u7::new(0x6C))
                .protocol(protocol::Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: protocol::Protocol::MIDI_2_VERSION,
                })
                .protocol(protocol::Protocol::Midi1 {
                    jitter_reduction_extension: true,
                    size_of_packet_extension: true,
                    version: protocol::Protocol::MIDI_1_VERSION,
                })
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x3416_7E7F,
                0x0D10_0151,
                0x3426_2E7E,
                0x2F00_600B,
                0x3426_2B6C,
                0x0202_0001,
                0x3426_0000,
                0x0100_0300,
                0x3431_0000,
                0x0000_0000
            ]),
        );
    }
}