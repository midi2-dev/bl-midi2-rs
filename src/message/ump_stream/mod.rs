use crate::{numeric_types::u10, util::Truncate, Data, Error, FromData, Result};
#[cfg(feature = "std")]
use crate::{IntoOwned, Level2Message};
mod ump_stream_group;

pub mod device_identity;
pub mod end_of_clip;
pub mod endpoint_discovery;
pub mod endpoint_info;
pub mod endpoint_name;
pub mod function_block_discovery;
pub mod function_block_info;
pub mod function_block_name;
pub mod product_instance_id;
pub mod start_of_clip;
pub mod stream_configuration_notification;
pub mod stream_configuration_request;

use device_identity::DeviceIdentityBorrowed;
#[cfg(feature = "std")]
use device_identity::DeviceIdentityBuilder;
use device_identity::DeviceIdentityMessage;
#[cfg(feature = "std")]
use device_identity::DeviceIdentityOwned;
use end_of_clip::EndOfClipBorrowed;
#[cfg(feature = "std")]
use end_of_clip::EndOfClipBuilder;
use end_of_clip::EndOfClipMessage;
#[cfg(feature = "std")]
use end_of_clip::EndOfClipOwned;
use endpoint_discovery::EndpointDiscoveryBorrowed;
#[cfg(feature = "std")]
use endpoint_discovery::EndpointDiscoveryBuilder;
use endpoint_discovery::EndpointDiscoveryMessage;
#[cfg(feature = "std")]
use endpoint_discovery::EndpointDiscoveryOwned;
use endpoint_info::EndpointInfoBorrowed;
#[cfg(feature = "std")]
use endpoint_info::EndpointInfoBuilder;
use endpoint_info::EndpointInfoMessage;
#[cfg(feature = "std")]
use endpoint_info::EndpointInfoOwned;
use endpoint_name::EndpointNameBorrowed;
#[cfg(feature = "std")]
use endpoint_name::EndpointNameBuilder;
use endpoint_name::EndpointNameMessage;
#[cfg(feature = "std")]
use endpoint_name::EndpointNameOwned;
use function_block_discovery::FunctionBlockDiscoveryBorrowed;
#[cfg(feature = "std")]
use function_block_discovery::FunctionBlockDiscoveryBuilder;
use function_block_discovery::FunctionBlockDiscoveryMessage;
#[cfg(feature = "std")]
use function_block_discovery::FunctionBlockDiscoveryOwned;
use function_block_info::FunctionBlockInfoBorrowed;
#[cfg(feature = "std")]
use function_block_info::FunctionBlockInfoBuilder;
use function_block_info::FunctionBlockInfoMessage;
#[cfg(feature = "std")]
use function_block_info::FunctionBlockInfoOwned;
use function_block_name::FunctionBlockNameBorrowed;
#[cfg(feature = "std")]
use function_block_name::FunctionBlockNameBuilder;
use function_block_name::FunctionBlockNameMessage;
#[cfg(feature = "std")]
use function_block_name::FunctionBlockNameOwned;
use product_instance_id::ProductInstanceIdBorrowed;
#[cfg(feature = "std")]
use product_instance_id::ProductInstanceIdBuilder;
use product_instance_id::ProductInstanceIdMessage;
#[cfg(feature = "std")]
use product_instance_id::ProductInstanceIdOwned;
use start_of_clip::StartOfClipBorrowed;
#[cfg(feature = "std")]
use start_of_clip::StartOfClipBuilder;
use start_of_clip::StartOfClipMessage;
#[cfg(feature = "std")]
use start_of_clip::StartOfClipOwned;
use stream_configuration_notification::StreamConfigurationNotificationBorrowed;
#[cfg(feature = "std")]
use stream_configuration_notification::StreamConfigurationNotificationBuilder;
use stream_configuration_notification::StreamConfigurationNotificationMessage;
#[cfg(feature = "std")]
use stream_configuration_notification::StreamConfigurationNotificationOwned;
use stream_configuration_request::StreamConfigurationRequestBorrowed;
#[cfg(feature = "std")]
use stream_configuration_request::StreamConfigurationRequestBuilder;
use stream_configuration_request::StreamConfigurationRequestMessage;
#[cfg(feature = "std")]
use stream_configuration_request::StreamConfigurationRequestOwned;

#[derive(midi2_proc::UmpDebug, derive_more::From, midi2_proc::Data, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UmpStreamMessage<'a> {
    DeviceIdentity(DeviceIdentityMessage<'a>),
    EndOfClip(EndOfClipMessage<'a>),
    EndpointDiscovery(EndpointDiscoveryMessage<'a>),
    EndpointInfo(EndpointInfoMessage<'a>),
    EndpointName(EndpointNameMessage<'a>),
    FunctionBlockDiscovery(FunctionBlockDiscoveryMessage<'a>),
    FunctionBlockInfo(FunctionBlockInfoMessage<'a>),
    FunctionBlockName(FunctionBlockNameMessage<'a>),
    ProductInstanceId(ProductInstanceIdMessage<'a>),
    StartOfClip(StartOfClipMessage<'a>),
    StreamConfigurationNotification(StreamConfigurationNotificationMessage<'a>),
    StreamConfigurationRequest(StreamConfigurationRequestMessage<'a>),
}

#[derive(midi2_proc::UmpDebug, derive_more::From, midi2_proc::Data, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UmpStreamBorrowed<'a> {
    DeviceIdentity(DeviceIdentityBorrowed<'a>),
    EndOfClip(EndOfClipBorrowed<'a>),
    EndpointDiscovery(EndpointDiscoveryBorrowed<'a>),
    EndpointInfo(EndpointInfoBorrowed<'a>),
    EndpointName(EndpointNameBorrowed<'a>),
    FunctionBlockDiscovery(FunctionBlockDiscoveryBorrowed<'a>),
    FunctionBlockInfo(FunctionBlockInfoBorrowed<'a>),
    FunctionBlockName(FunctionBlockNameBorrowed<'a>),
    ProductInstanceId(ProductInstanceIdBorrowed<'a>),
    StartOfClip(StartOfClipBorrowed<'a>),
    StreamConfigurationNotification(StreamConfigurationNotificationBorrowed<'a>),
    StreamConfigurationRequest(StreamConfigurationRequestBorrowed<'a>),
}

#[derive(midi2_proc::UmpDebug, derive_more::From, midi2_proc::Data, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg(feature = "std")]
pub enum UmpStreamOwned {
    DeviceIdentity(DeviceIdentityOwned),
    EndOfClip(EndOfClipOwned),
    EndpointDiscovery(EndpointDiscoveryOwned),
    EndpointInfo(EndpointInfoOwned),
    EndpointName(EndpointNameOwned),
    FunctionBlockDiscovery(FunctionBlockDiscoveryOwned),
    FunctionBlockInfo(FunctionBlockInfoOwned),
    FunctionBlockName(FunctionBlockNameOwned),
    ProductInstanceId(ProductInstanceIdOwned),
    StartOfClip(StartOfClipOwned),
    StreamConfigurationNotification(StreamConfigurationNotificationOwned),
    StreamConfigurationRequest(StreamConfigurationRequestOwned),
}

#[derive(Default)]
#[cfg(feature = "std")]
pub struct UmpStreamBuilder<M>(core::marker::PhantomData<M>)
where
    M: core::convert::From<DeviceIdentityOwned>
        + core::convert::From<EndOfClipOwned>
        + core::convert::From<EndpointDiscoveryOwned>
        + core::convert::From<EndpointInfoOwned>
        + core::convert::From<EndpointNameOwned>
        + core::convert::From<FunctionBlockDiscoveryOwned>
        + core::convert::From<FunctionBlockInfoOwned>
        + core::convert::From<FunctionBlockNameOwned>
        + core::convert::From<ProductInstanceIdOwned>
        + core::convert::From<StartOfClipOwned>
        + core::convert::From<StreamConfigurationNotificationOwned>
        + core::convert::From<StreamConfigurationRequestOwned>;

#[cfg(feature = "std")]
impl<M> UmpStreamBuilder<M>
where
    M: core::convert::From<DeviceIdentityOwned>
        + core::convert::From<EndOfClipOwned>
        + core::convert::From<EndpointDiscoveryOwned>
        + core::convert::From<EndpointInfoOwned>
        + core::convert::From<EndpointNameOwned>
        + core::convert::From<FunctionBlockDiscoveryOwned>
        + core::convert::From<FunctionBlockInfoOwned>
        + core::convert::From<FunctionBlockNameOwned>
        + core::convert::From<ProductInstanceIdOwned>
        + core::convert::From<StartOfClipOwned>
        + core::convert::From<StreamConfigurationNotificationOwned>
        + core::convert::From<StreamConfigurationRequestOwned>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn device_identity(self) -> DeviceIdentityBuilder<M> {
        DeviceIdentityBuilder::new()
    }
    pub fn end_of_clip(self) -> EndOfClipBuilder<M> {
        EndOfClipBuilder::new()
    }
    pub fn endpoint_discovery(self) -> EndpointDiscoveryBuilder<M> {
        EndpointDiscoveryBuilder::new()
    }
    pub fn endpoint_info(self) -> EndpointInfoBuilder<M> {
        EndpointInfoBuilder::new()
    }
    pub fn endpoint_name(self) -> EndpointNameBuilder<M> {
        EndpointNameBuilder::new()
    }
    pub fn function_block_discovery(self) -> FunctionBlockDiscoveryBuilder<M> {
        FunctionBlockDiscoveryBuilder::new()
    }
    pub fn function_block_info(self) -> FunctionBlockInfoBuilder<M> {
        FunctionBlockInfoBuilder::new()
    }
    pub fn function_block_name(self) -> FunctionBlockNameBuilder<M> {
        FunctionBlockNameBuilder::new()
    }
    pub fn product_instance_id(self) -> ProductInstanceIdBuilder<M> {
        ProductInstanceIdBuilder::new()
    }
    pub fn start_of_clip(self) -> StartOfClipBuilder<M> {
        StartOfClipBuilder::new()
    }
    pub fn stream_configuration_notification(self) -> StreamConfigurationNotificationBuilder<M> {
        StreamConfigurationNotificationBuilder::new()
    }
    pub fn stream_configuration_request_o(self) -> StreamConfigurationRequestBuilder<M> {
        StreamConfigurationRequestBuilder::new()
    }
}

#[cfg(feature = "std")]
impl<'a> UmpStreamMessage<'a> {
    pub fn builder() -> UmpStreamBuilder<Self> {
        UmpStreamBuilder::new()
    }
}

#[cfg(feature = "std")]
impl UmpStreamOwned {
    pub fn builder() -> UmpStreamBuilder<Self> {
        UmpStreamBuilder::new()
    }
}

impl<'a> core::convert::From<UmpStreamBorrowed<'a>> for UmpStreamMessage<'a> {
    fn from(value: UmpStreamBorrowed<'a>) -> Self {
        use UmpStreamBorrowed as B;
        use UmpStreamMessage as M;
        match value {
            B::DeviceIdentity(m) => M::DeviceIdentity(m.into()),
            B::EndOfClip(m) => M::EndOfClip(m.into()),
            B::EndpointDiscovery(m) => M::EndpointDiscovery(m.into()),
            B::EndpointInfo(m) => M::EndpointInfo(m.into()),
            B::EndpointName(m) => M::EndpointName(m.into()),
            B::FunctionBlockDiscovery(m) => M::FunctionBlockDiscovery(m.into()),
            B::FunctionBlockInfo(m) => M::FunctionBlockInfo(m.into()),
            B::FunctionBlockName(m) => M::FunctionBlockName(m.into()),
            B::ProductInstanceId(m) => M::ProductInstanceId(m.into()),
            B::StartOfClip(m) => M::StartOfClip(m.into()),
            B::StreamConfigurationNotification(m) => M::StreamConfigurationNotification(m.into()),
            B::StreamConfigurationRequest(m) => M::StreamConfigurationRequest(m.into()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> core::convert::From<UmpStreamOwned> for UmpStreamMessage<'a> {
    fn from(value: UmpStreamOwned) -> Self {
        use UmpStreamMessage as M;
        use UmpStreamOwned as O;
        match value {
            O::DeviceIdentity(m) => M::DeviceIdentity(m.into()),
            O::EndOfClip(m) => M::EndOfClip(m.into()),
            O::EndpointDiscovery(m) => M::EndpointDiscovery(m.into()),
            O::EndpointInfo(m) => M::EndpointInfo(m.into()),
            O::EndpointName(m) => M::EndpointName(m.into()),
            O::FunctionBlockDiscovery(m) => M::FunctionBlockDiscovery(m.into()),
            O::FunctionBlockInfo(m) => M::FunctionBlockInfo(m.into()),
            O::FunctionBlockName(m) => M::FunctionBlockName(m.into()),
            O::ProductInstanceId(m) => M::ProductInstanceId(m.into()),
            O::StartOfClip(m) => M::StartOfClip(m.into()),
            O::StreamConfigurationNotification(m) => M::StreamConfigurationNotification(m.into()),
            O::StreamConfigurationRequest(m) => M::StreamConfigurationRequest(m.into()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a, M> core::convert::From<M> for UmpStreamMessage<'a>
where
    M: Level2Message,
    UmpStreamOwned: core::convert::From<M>,
{
    fn from(value: M) -> Self {
        <UmpStreamOwned as core::convert::From<M>>::from(value).into()
    }
}

const TYPE_CODE: u32 = 0xF;

const DEVICE_IDENTITY: u32 = 0x2;
const END_OF_CLIP: u32 = 0x21;
const ENDPOINT_DISCOVERY: u32 = 0x0;
const ENDPOINT_INFO: u32 = 0x1;
const ENDPOINT_NAME: u32 = 0x3;
const FUNCTION_BLOCK_DISCOVERY: u32 = 0x10;
const FUNCTION_BLOCK_INFO: u32 = 0x11;
const FUNCTION_BLOCK_NAME: u32 = 0x12;
const PRODUCT_INSTANCE_ID: u32 = 0x4;
const START_OF_CLIP: u32 = 0x20;
const STREAM_CONFIGURATION_NOTIFICATION: u32 = 0x06;
const STREAM_CONFIGURATION_REQUEST: u32 = 0x05;

impl<'a> FromData<'a> for UmpStreamBorrowed<'a> {
    type Target = Self;
    fn validate_data(data: &[u32]) -> Result<()> {
        let code = (data[0] & 0x03FF_0000) >> 16;
        match code {
            DEVICE_IDENTITY => DeviceIdentityBorrowed::validate_data(data),
            END_OF_CLIP => EndOfClipBorrowed::validate_data(data),
            ENDPOINT_DISCOVERY => EndpointDiscoveryBorrowed::validate_data(data),
            ENDPOINT_INFO => EndpointInfoBorrowed::validate_data(data),
            ENDPOINT_NAME => EndpointNameBorrowed::validate_data(data),
            FUNCTION_BLOCK_DISCOVERY => FunctionBlockDiscoveryBorrowed::validate_data(data),
            FUNCTION_BLOCK_INFO => FunctionBlockInfoBorrowed::validate_data(data),
            FUNCTION_BLOCK_NAME => FunctionBlockNameBorrowed::validate_data(data),
            PRODUCT_INSTANCE_ID => ProductInstanceIdBorrowed::validate_data(data),
            START_OF_CLIP => StartOfClipBorrowed::validate_data(data),
            STREAM_CONFIGURATION_NOTIFICATION => {
                StreamConfigurationNotificationBorrowed::validate_data(data)
            }
            STREAM_CONFIGURATION_REQUEST => StreamConfigurationRequestBorrowed::validate_data(data),
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        use UmpStreamBorrowed::*;
        let code = (data[0] & 0x03FF_0000) >> 16;
        match code {
            DEVICE_IDENTITY => DeviceIdentity(DeviceIdentityBorrowed::from_data_unchecked(data)),
            END_OF_CLIP => EndOfClip(EndOfClipBorrowed::from_data_unchecked(data)),
            ENDPOINT_DISCOVERY => {
                EndpointDiscovery(EndpointDiscoveryBorrowed::from_data_unchecked(data))
            }
            ENDPOINT_INFO => EndpointInfo(EndpointInfoBorrowed::from_data_unchecked(data)),
            ENDPOINT_NAME => EndpointName(EndpointNameBorrowed::from_data_unchecked(data)),
            FUNCTION_BLOCK_DISCOVERY => {
                FunctionBlockDiscovery(FunctionBlockDiscoveryBorrowed::from_data_unchecked(data))
            }
            FUNCTION_BLOCK_INFO => {
                FunctionBlockInfo(FunctionBlockInfoBorrowed::from_data_unchecked(data))
            }
            FUNCTION_BLOCK_NAME => {
                FunctionBlockName(FunctionBlockNameBorrowed::from_data_unchecked(data))
            }
            PRODUCT_INSTANCE_ID => {
                ProductInstanceId(ProductInstanceIdBorrowed::from_data_unchecked(data))
            }
            START_OF_CLIP => StartOfClip(StartOfClipBorrowed::from_data_unchecked(data)),
            STREAM_CONFIGURATION_NOTIFICATION => StreamConfigurationNotification(
                StreamConfigurationNotificationBorrowed::from_data_unchecked(data),
            ),
            STREAM_CONFIGURATION_REQUEST => StreamConfigurationRequest(
                StreamConfigurationRequestBorrowed::from_data_unchecked(data),
            ),
            _ => panic!(),
        }
    }
}

impl<'a> FromData<'a> for UmpStreamMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        UmpStreamBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        UmpStreamBorrowed::from_data_unchecked(buffer).into()
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for UmpStreamBorrowed<'a> {
    type Owned = UmpStreamOwned;
    fn into_owned(self) -> Self::Owned {
        use UmpStreamBorrowed as B;
        use UmpStreamOwned as O;
        match self {
            B::DeviceIdentity(m) => O::DeviceIdentity(m.into_owned()),
            B::EndOfClip(m) => O::EndOfClip(m.into_owned()),
            B::EndpointDiscovery(m) => O::EndpointDiscovery(m.into_owned()),
            B::EndpointInfo(m) => O::EndpointInfo(m.into_owned()),
            B::EndpointName(m) => O::EndpointName(m.into_owned()),
            B::FunctionBlockDiscovery(m) => O::FunctionBlockDiscovery(m.into_owned()),
            B::FunctionBlockInfo(m) => O::FunctionBlockInfo(m.into_owned()),
            B::FunctionBlockName(m) => O::FunctionBlockName(m.into_owned()),
            B::ProductInstanceId(m) => O::ProductInstanceId(m.into_owned()),
            B::StartOfClip(m) => O::StartOfClip(m.into_owned()),
            B::StreamConfigurationNotification(m) => {
                O::StreamConfigurationNotification(m.into_owned())
            }
            B::StreamConfigurationRequest(m) => O::StreamConfigurationRequest(m.into_owned()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for UmpStreamMessage<'a> {
    type Owned = UmpStreamOwned;
    fn into_owned(self) -> UmpStreamOwned {
        use UmpStreamMessage as M;
        use UmpStreamOwned as O;
        match self {
            M::DeviceIdentity(m) => O::DeviceIdentity(m.into_owned()),
            M::EndOfClip(m) => O::EndOfClip(m.into_owned()),
            M::EndpointDiscovery(m) => O::EndpointDiscovery(m.into_owned()),
            M::EndpointInfo(m) => O::EndpointInfo(m.into_owned()),
            M::EndpointName(m) => O::EndpointName(m.into_owned()),
            M::FunctionBlockDiscovery(m) => O::FunctionBlockDiscovery(m.into_owned()),
            M::FunctionBlockInfo(m) => O::FunctionBlockInfo(m.into_owned()),
            M::FunctionBlockName(m) => O::FunctionBlockName(m.into_owned()),
            M::ProductInstanceId(m) => O::ProductInstanceId(m.into_owned()),
            M::StartOfClip(m) => O::StartOfClip(m.into_owned()),
            M::StreamConfigurationNotification(m) => {
                O::StreamConfigurationNotification(m.into_owned())
            }
            M::StreamConfigurationRequest(m) => O::StreamConfigurationRequest(m.into_owned()),
        }
    }
}

fn status_from_buffer(buffer: &[u32]) -> u10 {
    (buffer[0] >> 16).truncate()
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use crate::test_support::debug;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            debug::Data(
                UmpStreamMessage::builder()
                    .endpoint_name()
                    .name("RhythmRevelation: Beats Beyond Boundaries🌍🥁🚀")
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF403_5268,
                0x7974_686D,
                0x5265_7665,
                0x6C61_7469,
                0xF803_6F6E,
                0x3A20_4265,
                0x6174_7320,
                0x4265_796F,
                0xF803_6E64,
                0x2042_6F75,
                0x6E64_6172,
                0x6965_73F0,
                0xFC03_9F8C,
                0x8DF0_9FA5,
                0x81F0_9F9A,
                0x8000_0000,
            ]),
        );
    }

    #[test]
    fn into_owned() {
        let _owned = {
            let buffer = [
                0xF403_5268,
                0x7974_686D,
                0x5265_7665,
                0x6C61_7469,
                0xF803_6F6E,
                0x3A20_4265,
                0x6174_7320,
                0x4265_796F,
                0xF803_6E64,
                0x2042_6F75,
                0x6E64_6172,
                0x6965_73F0,
                0xFC03_9F8C,
                0x8DF0_9FA5,
                0x81F0_9F9A,
                0x8000_0000,
            ];
            let message = UmpStreamMessage::from_data(&buffer).unwrap();
            message.into_owned()
        };
    }
}
