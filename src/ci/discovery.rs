use crate::{
    util::{BitOps, getter, Truncate},
    ci::{DeviceId, helpers, CiMessage},
    message::system_exclusive_8bit::Message as Sysex8Message,
    error::Error,
};

pub struct Message {
    source: ux::u28,
    device_manufacturer: ux::u21,
    device_family: ux::u14,
    device_model_number: ux::u14,
    software_version: [ux::u7; 4],
    protocol_negotiation_supported: bool,
    profile_configuration_supported: bool,
    property_exchange_supported: bool,
    max_sysex_message_size: ux::u28,
}

impl Message {
    getter::getter!(source, ux::u28);
    getter::getter!(device_manufacturer, ux::u21);
    getter::getter!(device_family, ux::u14);
    getter::getter!(device_model_number, ux::u14);
    getter::getter!(software_version, [ux::u7; 4]);
    getter::getter!(protocol_negotiation_supported, bool);
    getter::getter!(profile_configuration_supported, bool);
    getter::getter!(property_exchange_supported, bool);
    getter::getter!(max_sysex_message_size, ux::u28);
}

impl CiMessage for Message {
    fn to_sysex_8<'a>(&self, messages: &'a mut [Sysex8Message], stream_id: u8) -> &'a [Sysex8Message] {
        let ret = helpers::write_ci_data(
            DeviceId::MidiPort, 
            0x70, 
            self.source, 
            ux::u28::new(0xFFF_FFFF),
            &[
                self.device_manufacturer.truncate(),
                (self.device_manufacturer >> 7).truncate(),
                (self.device_manufacturer >> 14).truncate(),
                self.device_family.truncate(),
                (self.device_family >> 7).truncate(),
                self.device_model_number.truncate(),
                (self.device_model_number >> 7).truncate(),
                self.software_version[0],
                self.software_version[1],
                self.software_version[2],
                self.software_version[3],
                {
                    let flags = *0x0_u8
                        .set_bit(6, self.protocol_negotiation_supported)
                        .set_bit(5, self.profile_configuration_supported)
                        .set_bit(4, self.property_exchange_supported);
                    ux::u7::new(flags)
                },
                self.max_sysex_message_size.truncate(),
                (self.max_sysex_message_size >> 7).truncate(),
                (self.max_sysex_message_size >> 14).truncate(),
                (self.max_sysex_message_size >> 21).truncate(),
            ],
            messages,
        );
        helpers::write_stream_id(ret, stream_id);
        ret
    }

    fn from_sysex_8(_messages: &[Sysex8Message]) -> Self {
        todo!()
    }
    fn validate_sysex_8(_message: &[Sysex8Message]) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn to_sysex_8() {
        assert_eq!(
            Message {
                source: ux::u28::new(0x123_1000_2000),
                device_manufacturer: ux::u21,
                device_family: ux::u14,
                device_model_number: ux::u14,
                software_version: [ux::u7; 4],
                protocol_negotiation_supported: bool,
                profile_configuration_supported: bool,
                property_exchange_supported: bool,
                max_sysex_message_size: ux::u28,
            }.to_sysex_8(&mut [Default::default(); 1], 0x31),
            &[],
        );
    }
}