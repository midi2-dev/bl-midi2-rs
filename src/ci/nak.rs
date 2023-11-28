use crate::{
    ci::{helpers, Ci, CiStandardData, DeviceId},
    message::sysex_bytes::{Sysex7BytesBorrowed, Sysex7BytesBorrowedBuilder},
    util::{Encode7Bit, Truncate},
    *,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NakBorrowed<'a>(Sysex7BytesBorrowed<'a>);

pub struct NakBorrowedBuilder<'a> {
    sysex_builder: Sysex7BytesBorrowedBuilder<'a>,
    status: Status,
    original_transaction: Option<OriginalTransaction>,
    standard_data: CiStandardData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Status {
    Nak,
    CiMessageNotSuppported,
    CiVersionNotSupported,
    ChannelGroupFunctionBlockNotSupported,
    ProfileNotSupported,
    TerminateInquiry,
    PropertyExchangeChunksOutOfSequence,
    ErrorOccurredPleaseRetry,
    MessageMalformed,
    TimeoutOccurred,
    BusyTryAgain(u7),
}

impl Status {
    fn from_code_and_data(code: u7, data: u7) -> Result<Status> {
        use Status::*;
        match u8::from(code) {
            // fail no retry
            0x0 => Ok(Nak),
            0x1 => Ok(CiMessageNotSuppported),
            0x2 => Ok(CiVersionNotSupported),
            0x3 => Ok(ChannelGroupFunctionBlockNotSupported),
            0x4 => Ok(ProfileNotSupported),
            // notifications
            0x20 => Ok(TerminateInquiry),
            0x21 => Ok(PropertyExchangeChunksOutOfSequence),
            // fail recommend retry
            0x40 => Ok(ErrorOccurredPleaseRetry),
            0x41 => Ok(MessageMalformed),
            0x42 => Ok(TimeoutOccurred),
            0x43 => Ok(BusyTryAgain(data)),
            _ => Err(Error::InvalidData),
        }
    }
    fn data(&self) -> u7 {
        use Status::*;
        match self {
            Nak => u7::new(0x0),
            CiMessageNotSuppported => u7::new(0x0),
            CiVersionNotSupported => u7::new(0x0),
            ChannelGroupFunctionBlockNotSupported => u7::new(0x0),
            ProfileNotSupported => u7::new(0x0),
            TerminateInquiry => u7::new(0x0),
            PropertyExchangeChunksOutOfSequence => u7::new(0x0),
            ErrorOccurredPleaseRetry => u7::new(0x0),
            MessageMalformed => u7::new(0x0),
            TimeoutOccurred => u7::new(0x0),
            BusyTryAgain(d) => *d,
        }
    }
    fn code(&self) -> u7 {
        use Status::*;
        match self {
            Nak => u7::new(0x0),
            CiMessageNotSuppported => u7::new(0x1),
            CiVersionNotSupported => u7::new(0x2),
            ChannelGroupFunctionBlockNotSupported => u7::new(0x3),
            ProfileNotSupported => u7::new(0x4),
            TerminateInquiry => u7::new(0x20),
            PropertyExchangeChunksOutOfSequence => u7::new(0x21),
            ErrorOccurredPleaseRetry => u7::new(0x40),
            MessageMalformed => u7::new(0x41),
            TimeoutOccurred => u7::new(0x42),
            BusyTryAgain(_) => u7::new(0x43),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OriginalTransaction {
    ProfileConfiguration {
        id: u7,
        profile_id: [u7; 5],
    },
    PropertyExchange {
        id: u7,
        stream_id: u7,
        chunk_number: u14,
    },
    ProcessInquiry {
        id: u7,
    },
    Management {
        id: u7,
    },
    ProtocolNegotiation {
        id: u7,
    },
}

fn profile_config_from_sub_id_and_data(id: u7, data: &[u8]) -> OriginalTransaction {
    let mut profile_id = [u7::default(); 5];
    for (i, d) in data.iter().enumerate() {
        profile_id[i] = d.truncate();
    }
    OriginalTransaction::ProfileConfiguration { id, profile_id }
}

impl OriginalTransaction {
    fn from_sub_id_and_data(id: u7, data: &[u8]) -> Result<OriginalTransaction> {
        use OriginalTransaction::*;
        match u8::from(id) {
            0x10..=0x1E => Ok(ProtocolNegotiation { id }),
            0x20..=0x2E => Ok(profile_config_from_sub_id_and_data(id, data)),
            0x30..=0x3E => Ok(PropertyExchange {
                id,
                stream_id: data[0].truncate(),
                chunk_number: u14::from_u7s(&data[1..3]),
            }),
            0x40..=0x4E => Ok(ProcessInquiry { id }),
            0x70..=0x7E => Ok(Management { id }),
            _ => Err(Error::InvalidData),
        }
    }
}

pub trait Nak: BytesData {
    fn original_transaction(&self) -> OriginalTransaction {
        let data = self.bytes_data();
        OriginalTransaction::from_sub_id_and_data(data[14].truncate(), &data[17..22]).unwrap()
    }
    fn status(&self) -> Status {
        let data = self.bytes_data();
        Status::from_code_and_data(data[15].truncate(), data[16].truncate()).unwrap()
    }
    fn builder(buffer: &mut [u8]) -> NakBorrowedBuilder {
        NakBorrowedBuilder::new(buffer)
    }
    // todo: message
    // may require the alloc feature
}

impl<'a> BytesData for NakBorrowed<'a> {
    fn bytes_data(&self) -> &[u8] {
        self.0.bytes_data()
    }
}

impl<'a> Nak for NakBorrowed<'a> {}

impl<'a> Ci for NakBorrowed<'a> {}

impl<'a> FromBytesData<'a> for NakBorrowed<'a> {
    type Target = Self;
    fn validate_bytes_data(buffer: &'a [u8]) -> Result<()> {
        Sysex7BytesBorrowed::validate_bytes_data(buffer)?;
        helpers::validate_ci_standard_bytes(buffer)?;
        if buffer.len() < 25 {
            return Err(Error::InvalidData);
        }
        Status::from_code_and_data(buffer[15].truncate(), buffer[16].truncate())?;
        Ok(())
    }
    fn from_bytes_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        Self(Sysex7BytesBorrowed::from_bytes_data_unchecked(buffer))
    }
}

impl<'a> NakBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            sysex_builder: Sysex7BytesBorrowedBuilder::new(buffer),
            status: Status::Nak,
            original_transaction: None,
            standard_data: Default::default(),
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn original_transaction(mut self, original_transaction: OriginalTransaction) -> Self {
        self.original_transaction = Some(original_transaction);
        self
    }

    pub fn source(mut self, v: u28) -> Self {
        self.standard_data.source = Some(v);
        self
    }

    pub fn destination(mut self, v: u28) -> Self {
        self.standard_data.destination = Some(v);
        self
    }

    pub fn device_id(mut self, v: DeviceId) -> Self {
        self.standard_data.device_id = v;
        self
    }

    pub fn build(mut self) -> Result<NakBorrowed<'a>> {
        let Some(original_transaction) = self.original_transaction else {
            return Err(Error::InvalidData);
        };
        self.standard_data.sysex_sub_id2 = Some(u7::new(0x7F));
        self.sysex_builder = self.sysex_builder.payload(self.standard_data.payload()?);
        let mut nak_data = [u7::default(); 10];
        // original transaction data
        {
            use OriginalTransaction::*;
            match original_transaction {
                ProfileConfiguration { id, profile_id } => {
                    nak_data[0] = id;
                    nak_data[3..8].copy_from_slice(&profile_id);
                }
                PropertyExchange {
                    id,
                    stream_id,
                    chunk_number,
                } => {
                    nak_data[0] = id;
                    nak_data[3] = stream_id;
                    chunk_number.to_u7s(&mut nak_data[4..6]);
                }
                ProcessInquiry { id } => {
                    nak_data[0] = id;
                }
                Management { id } => {
                    nak_data[0] = id;
                }
                ProtocolNegotiation { id } => {
                    nak_data[0] = id;
                }
            };
        }
        // status data
        {
            nak_data[1] = self.status.code();
            nak_data[2] = self.status.data();
        }
        self.sysex_builder = self.sysex_builder.payload(nak_data.iter().cloned());
        Ok(NakBorrowed(self.sysex_builder.build()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ci::VERSION,
        util::{debug, RandomBuffer},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            debug::ByteData(
                NakBorrowed::builder(&mut Bytes::random_buffer::<25>())
                    .source(u28::new(0x7E1311))
                    .destination(u28::new(0x398F97B))
                    .device_id(DeviceId::Group)
                    .status(Status::BusyTryAgain(u7::new(0x35)))
                    .original_transaction(OriginalTransaction::ProfileConfiguration {
                        id: u7::new(0x2C),
                        profile_id: [
                            u7::new(0x5D),
                            u7::new(0x75),
                            u7::new(0x29),
                            u7::new(0x6D),
                            u7::new(0x3A),
                        ],
                    })
                    .build()
                    .unwrap()
                    .bytes_data()
            ),
            debug::ByteData(&[
                0xF0,
                0x7E,
                0x7E,
                0x0D,
                0x7F,
                VERSION.into(),
                0x11,
                0x26,
                0x78,
                0x03,
                0x7B,
                0x72,
                0x63,
                0x1C,
                0x2C,
                0x43,
                0x35,
                0x5D,
                0x75,
                0x29,
                0x6D,
                0x3A,
                0x0,
                0x0,
                0xF7,
            ]),
        );
    }

    #[test]
    fn source() {
        assert_eq!(
            NakBorrowed::from_bytes_data(&[
                0xF0,
                0x7E,
                0x7E,
                0x0D,
                0x7F,
                VERSION.into(),
                0x11,
                0x26,
                0x78,
                0x03,
                0x7B,
                0x72,
                0x63,
                0x1C,
                0x2C,
                0x43,
                0x35,
                0x5D,
                0x75,
                0x29,
                0x6D,
                0x3A,
                0x0,
                0x0,
                0xF7,
            ])
            .unwrap()
            .source(),
            u28::new(0x7E1311),
        );
    }

    #[test]
    fn destination() {
        assert_eq!(
            NakBorrowed::from_bytes_data(&[
                0xF0,
                0x7E,
                0x7E,
                0x0D,
                0x7F,
                VERSION.into(),
                0x11,
                0x26,
                0x78,
                0x03,
                0x7B,
                0x72,
                0x63,
                0x1C,
                0x2C,
                0x43,
                0x35,
                0x5D,
                0x75,
                0x29,
                0x6D,
                0x3A,
                0x0,
                0x0,
                0xF7,
            ])
            .unwrap()
            .destination(),
            u28::new(0x398F97B),
        );
    }

    #[test]
    fn device_id() {
        assert_eq!(
            NakBorrowed::from_bytes_data(&[
                0xF0,
                0x7E,
                0x7E,
                0x0D,
                0x7F,
                VERSION.into(),
                0x11,
                0x26,
                0x78,
                0x03,
                0x7B,
                0x72,
                0x63,
                0x1C,
                0x2C,
                0x43,
                0x35,
                0x5D,
                0x75,
                0x29,
                0x6D,
                0x3A,
                0x0,
                0x0,
                0xF7,
            ])
            .unwrap()
            .device_id(),
            DeviceId::Group,
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            NakBorrowed::from_bytes_data(&[
                0xF0,
                0x7E,
                0x7E,
                0x0D,
                0x7F,
                VERSION.into(),
                0x11,
                0x26,
                0x78,
                0x03,
                0x7B,
                0x72,
                0x63,
                0x1C,
                0x2C,
                0x43,
                0x35,
                0x5D,
                0x75,
                0x29,
                0x6D,
                0x3A,
                0x0,
                0x0,
                0xF7,
            ])
            .unwrap()
            .status(),
            Status::BusyTryAgain(u7::new(0x35)),
        );
    }

    #[test]
    fn original_transaction() {
        assert_eq!(
            NakBorrowed::from_bytes_data(&[
                0xF0,
                0x7E,
                0x7E,
                0x0D,
                0x7F,
                VERSION.into(),
                0x11,
                0x26,
                0x78,
                0x03,
                0x7B,
                0x72,
                0x63,
                0x1C,
                0x2C,
                0x43,
                0x35,
                0x5D,
                0x75,
                0x29,
                0x6D,
                0x3A,
                0x0,
                0x0,
                0xF7,
            ])
            .unwrap()
            .original_transaction(),
            OriginalTransaction::ProfileConfiguration {
                id: u7::new(0x2C),
                profile_id: [
                    u7::new(0x5D),
                    u7::new(0x75),
                    u7::new(0x29),
                    u7::new(0x6D),
                    u7::new(0x3A),
                ],
            },
        );
    }
}
