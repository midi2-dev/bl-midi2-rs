macro_rules! test_protocol_message {
    ($op_code:expr) => {
        use crate::{
            ci::{
                ci_message_impl,
                helpers as ci_helpers,
                test_protocol::{validate_test_data, test_data},
                DeviceId
            },
            error::Error,
            util::{builder, getter, sysex_message, Truncate},
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            source: ux::u28,
            destination: ux::u28,
            authority_level: ux::u7,
        }

        builder::builder!(
            group: ux::u4,
            source: ux::u28,
            destination: ux::u28,
            authority_level: ux::u7
        );

        impl Message {
            const STATUS: u8 = $op_code;
            const DATA_SIZE: usize = 62;
            getter::getter!(group, ux::u4);
            getter::getter!(source, ux::u28);
            getter::getter!(destination, ux::u28);
            getter::getter!(authority_level, ux::u7);
            builder::builder_method!();
        }

        fn to_sysex<'a, M: sysex_message::SysexMessage>(message: &Message, messages: &'a mut [M]) -> &'a mut [M] {
            let mut test_data_buffer = [ux::u7::default(); 48];
            ci_helpers::write_ci_data(
                message.group,
                DeviceId::MidiPort,
                Message::STATUS,
                message.source,
                message.destination,
                &[&[message.authority_level], test_data(&mut test_data_buffer)].concat(),
                messages,
            )
        }

        fn from_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Message {
            let standard_data = ci_helpers::read_standard_data(messages);
            let messages = sysex_message::SysexMessages::new(messages);
            Message {
                group: messages.group(),
                source: standard_data.source,
                destination: standard_data.destination,
                authority_level: messages.datum(13).truncate(),
            }
        }

        fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
            ci_helpers::validate_sysex(messages, Message::STATUS)?;
            ci_helpers::validate_buffer_size(messages, Message::DATA_SIZE)?;
            let messages = sysex_message::SysexMessages::new(messages);
            validate_test_data(&[
                messages.datum(14),
                messages.datum(15),
                messages.datum(16),
                messages.datum(17),
                messages.datum(18),
                messages.datum(19),
                messages.datum(20),
                messages.datum(21),
                messages.datum(22),
                messages.datum(23),
                messages.datum(24),
                messages.datum(25),
                messages.datum(26),
                messages.datum(27),
                messages.datum(28),
                messages.datum(29),
                messages.datum(30),
                messages.datum(31),
                messages.datum(32),
                messages.datum(33),
                messages.datum(34),
                messages.datum(35),
                messages.datum(36),
                messages.datum(37),
                messages.datum(38),
                messages.datum(39),
                messages.datum(40),
                messages.datum(41),
                messages.datum(42),
                messages.datum(43),
                messages.datum(44),
                messages.datum(45),
                messages.datum(46),
                messages.datum(47),
                messages.datum(48),
                messages.datum(49),
                messages.datum(50),
                messages.datum(51),
                messages.datum(52),
                messages.datum(53),
                messages.datum(54),
                messages.datum(55),
                messages.datum(56),
                messages.datum(57),
                messages.datum(58),
                messages.datum(59),
                messages.datum(60),
                messages.datum(61),
            ])
        }

        fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(
            _message: &Message,
            messages: &[M],
        ) -> Result<(), Error> {
            ci_helpers::validate_buffer_size(messages, Message::DATA_SIZE)
        }
        
        ci_message_impl!();
    }
}

use crate::error::Error;

fn test_data(buff: &mut [ux::u7]) -> &[ux::u7] {
    for i in 0u8..48u8 {
        buff[i as usize] = ux::u7::new(i);
    }
    buff
}

fn validate_test_data(buff: &[u8; 48]) -> Result<(), Error> {
    match buff.iter().zip(0u8..48u8).all(|(&l, r)| l == r) {
        true => Ok(()),
        false => Err(Error::InvalidData),
    }
}

pub mod query {
    test_protocol_message!(0x13);
}

pub mod reply {
    test_protocol_message!(0x14);
}

#[cfg(test)]
mod tests {
    use crate::{
        ci::{CiMessage, VERSION, test_protocol::query::Message},
        message::system_exclusive_8bit as sysex8,
        message::system_exclusive_7bit as sysex7,
    };
    
    const TEST_STREAM_ID: u8 = 0x88;
    const TEST_GROUP: ux::u4 = ux::u4::new(0xE);
    const TEST_SOURCE: ux::u28 = ux::u28::new(31193279);
    const TEST_DESTINATION: ux::u28 = ux::u28::new(196547546);
    const TEST_AUTHORITY_LEVEL: ux::u7 = ux::u7::new(0x19);
    
    #[test]
    #[rustfmt::skip]
    fn try_to_sysex8() {
        assert_eq!(
            Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
                .try_to_sysex8(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ], TEST_STREAM_ID)
                .unwrap(),
            &[
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: whole midi port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x13, // universal sysex sub-id 2: test protocol query
                        VERSION,
                        0b00111111, 0b01110001, 0b01101111, 0b00001110, // source
                        0b01011010, 0b00100111, 0b01011100, // destination
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0b01011101, // destination
                        0x19, // authority level
                        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
                        0x08, 0x09, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 
                        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                        0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 
                        0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::End)
                    .data(&[
                        0x2E, 0x2F, // test data
                    ])
                    .build(),
            ],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_to_sysex7() {
        assert_eq!(
            Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
                .try_to_sysex7(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ])
                .unwrap(),
            &[
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID: whole midi port
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x13), // universal sysex sub-id 2: test protocol query
                        ux::u7::new(VERSION),
                        ux::u7::new(0b00111111), // source
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01110001), ux::u7::new(0b01101111), ux::u7::new(0b00001110), // source
                        ux::u7::new(0b01011010), ux::u7::new(0b00100111), ux::u7::new(0b01011100), // destination
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01011101), // destination
                        ux::u7::new(0x19), // authority level
                        ux::u7::new(0x00), ux::u7::new(0x01),
                        ux::u7::new(0x02), ux::u7::new(0x03), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x04), ux::u7::new(0x05),
                        ux::u7::new(0x06), ux::u7::new(0x07), 
                        ux::u7::new(0x08), ux::u7::new(0x09), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x0A), ux::u7::new(0x0B),
                        ux::u7::new(0x0C), ux::u7::new(0x0D),
                        ux::u7::new(0x0E), ux::u7::new(0x0F), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x10), ux::u7::new(0x11),
                        ux::u7::new(0x12), ux::u7::new(0x13),
                        ux::u7::new(0x14), ux::u7::new(0x15), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x16), ux::u7::new(0x17),
                        ux::u7::new(0x18), ux::u7::new(0x19),
                        ux::u7::new(0x1A), ux::u7::new(0x1B), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x1C), ux::u7::new(0x1D),
                        ux::u7::new(0x1E), ux::u7::new(0x1F),
                        ux::u7::new(0x20), ux::u7::new(0x21), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x22), ux::u7::new(0x23),
                        ux::u7::new(0x24), ux::u7::new(0x25),
                        ux::u7::new(0x26), ux::u7::new(0x27), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x28), ux::u7::new(0x29),
                        ux::u7::new(0x2A), ux::u7::new(0x2B),
                        ux::u7::new(0x2C), ux::u7::new(0x2D), // test data
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(TEST_GROUP)
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0x2E), ux::u7::new(0x2F), // test data
                    ])
                    .build(),
            ],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_sysex8() {
        assert_eq!(
            Message::try_from_sysex8(&[
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: whole midi port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x13, // universal sysex sub-id 2: test protocol query
                        VERSION,
                        0b00111111, 0b01110001, 0b01101111, 0b00001110, // source
                        0b01011010, 0b00100111, 0b01011100, // destination
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0b01011101, // destination
                        0x19, // authority level
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x0, 0x1, 0x2, 0x3, 0x4, 0x5,      
                        0x6, 0x7, 0x8, 0x9, 0xA, 0xB,      
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0xC, 0xD, 0xE, 0xF, 0x10, 0x11,      
                        0x12, 0x13, 0x14, 0x15, 0x16, 0x17,      
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,      
                        0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23,      
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::End)
                    .data(&[
                        0x24, 0x25, 0x26, 0x27, 0x28, 0x29,      
                        0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,      
                    ])
                    .build(),
            ]),
            Ok(Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
            ),
        )
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_sysex7() {
        assert_eq!(
            Message::try_from_sysex7(&[
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID: whole midi port
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x13), // universal sysex sub-id 2: test protocol query
                        ux::u7::new(VERSION),
                        ux::u7::new(0b00111111),// source
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01110001), ux::u7::new(0b01101111),
                        ux::u7::new(0b00001110), ux::u7::new(0b01011010),
                        ux::u7::new(0b00100111), ux::u7::new(0b01011100), // destination
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01011101), // destination
                        ux::u7::new(0x19), // authority level
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x0), ux::u7::new(0x1), 
                        ux::u7::new(0x2), ux::u7::new(0x3), 
                        ux::u7::new(0x4), ux::u7::new(0x5), 
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x6), ux::u7::new(0x7), 
                        ux::u7::new(0x8), ux::u7::new(0x9), 
                        ux::u7::new(0xA), ux::u7::new(0xB), 
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0xC), ux::u7::new(0xD), 
                        ux::u7::new(0xE), ux::u7::new(0xF), 
                        ux::u7::new(0x10), ux::u7::new(0x11), 
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x12), ux::u7::new(0x13), 
                        ux::u7::new(0x14), ux::u7::new(0x15), 
                        ux::u7::new(0x16), ux::u7::new(0x17), 
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x18), ux::u7::new(0x19), 
                        ux::u7::new(0x1A), ux::u7::new(0x1B), 
                        ux::u7::new(0x1C), ux::u7::new(0x1D), 
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x1E), ux::u7::new(0x1F), 
                        ux::u7::new(0x20), ux::u7::new(0x21), 
                        ux::u7::new(0x22), ux::u7::new(0x23), 
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x24), ux::u7::new(0x25), 
                        ux::u7::new(0x26), ux::u7::new(0x27), 
                        ux::u7::new(0x28), ux::u7::new(0x29), 
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0x2A), ux::u7::new(0x2B), 
                        ux::u7::new(0x2C), ux::u7::new(0x2D), 
                        ux::u7::new(0x2E), ux::u7::new(0x2F), 
                    ])
                    .build(),
            ]),
            Ok(Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
            ),
        )
    }
}