use crate::{
    buffer::{BufferMut, Bytes},
    detail::property,
};

use super::device_id;

pub struct UniversalSystemExclusiveByteProperty;

impl<B: Bytes> property::Property<B> for UniversalSystemExclusiveByteProperty {
    type Type = ();
}

impl<'a, B: Bytes> property::ReadProperty<'a, B> for UniversalSystemExclusiveByteProperty {
    fn read(_buffer: &'a B) -> Self::Type {}
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        if buffer.buffer()[1] == 0x7E {
            Ok(())
        } else {
            Err(crate::error::InvalidData(
                "Expected Universal Sysex Byte: 0x7E",
            ))
        }
    }
}

impl<B: Bytes + BufferMut> property::WriteProperty<B> for UniversalSystemExclusiveByteProperty {
    fn write(buffer: &mut B, _v: Self::Type) {
        buffer.buffer_mut()[1] = 0x7E;
    }
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {}
}

pub struct DeviceIdProperty;

impl<B: Bytes> property::Property<B> for DeviceIdProperty {
    type Type = device_id::DeviceId;
}

impl<'a, B: Bytes> property::ReadProperty<'a, B> for DeviceIdProperty {
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        device_id::DeviceId::from_u8(buffer.buffer()[2])?;
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        device_id::DeviceId::from_u8(buffer.buffer()[2]).unwrap()
    }
}

impl<B: Bytes + BufferMut> property::WriteProperty<B> for DeviceIdProperty {
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[2] = v.to_u8();
    }
}

pub struct DeviceIdToFunctionBlockProperty;

impl<B: Bytes> property::Property<B> for DeviceIdToFunctionBlockProperty {
    type Type = ();
}

impl<'a, B: Bytes> property::ReadProperty<'a, B> for DeviceIdToFunctionBlockProperty {
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        if buffer.buffer()[2] == 0x7F {
            Ok(())
        } else {
            Err(crate::error::InvalidData(
                "Expected DeviceID: To Function Block (0x7F)",
            ))
        }
    }
    fn read(_buffer: &'a B) -> Self::Type {}
}

impl<B: Bytes + BufferMut> property::WriteProperty<B> for DeviceIdToFunctionBlockProperty {
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {}
    fn write(buffer: &mut B, _v: Self::Type) {
        buffer.buffer_mut()[2] = 0x7F;
    }
}

pub struct UniversalSysexSubId1MidiCi;

impl<B: Bytes> property::Property<B> for UniversalSysexSubId1MidiCi {
    type Type = ();
}

impl<'a, B: Bytes> property::ReadProperty<'a, B> for UniversalSysexSubId1MidiCi {
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        if buffer.buffer()[3] == 0x0D {
            Ok(())
        } else {
            Err(crate::error::InvalidData(
                "Expected Universal Sysex Sub Id #1: Midi CI (0x0D)",
            ))
        }
    }
    fn read(_buffer: &'a B) -> Self::Type {}
}

impl<B: Bytes + BufferMut> property::WriteProperty<B> for UniversalSysexSubId1MidiCi {
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {}
    fn write(buffer: &mut B, _v: Self::Type) {
        buffer.buffer_mut()[3] = 0x0D;
    }
}

pub struct UniversalSysexSubId2<const STATUS: u8>;

impl<const STATUS: u8, B: Bytes> property::Property<B> for UniversalSysexSubId2<STATUS> {
    type Type = ();
}

impl<'a, const STATUS: u8, B: Bytes> property::ReadProperty<'a, B>
    for UniversalSysexSubId2<STATUS>
{
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        if buffer.buffer()[4] == STATUS {
            Ok(())
        } else {
            Err(crate::error::InvalidData(
                "Incorrect Universal Sysex Sub Id #2 field",
            ))
        }
    }
    fn read(_buffer: &'a B) -> Self::Type {}
}

impl<const STATUS: u8, B: Bytes + BufferMut> property::WriteProperty<B>
    for UniversalSysexSubId2<STATUS>
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {}
    fn write(buffer: &mut B, _v: Self::Type) {
        buffer.buffer_mut()[4] = STATUS;
    }
}

pub struct SourceProperty;

impl<B: Bytes> property::Property<B> for SourceProperty {
    type Type = ux::u28;
}

impl<'a, B: Bytes> property::ReadProperty<'a, B> for SourceProperty {
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::Encode7Bit;
        ux::u28::from_u7s(&buffer.buffer()[6..10])
    }
}

impl<B: Bytes + BufferMut> property::WriteProperty<B> for SourceProperty {
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::detail::Encode7Bit;
        v.to_u7s(&mut buffer.buffer_mut()[6..10]);
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

pub struct DestinationProperty;

impl<B: Bytes> property::Property<B> for DestinationProperty {
    type Type = ux::u28;
}

impl<'a, B: Bytes> property::ReadProperty<'a, B> for DestinationProperty {
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::Encode7Bit;
        ux::u28::from_u7s(&buffer.buffer()[10..14])
    }
}

impl<B: Bytes + BufferMut> property::WriteProperty<B> for DestinationProperty {
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::detail::Encode7Bit;
        v.to_u7s(&mut buffer.buffer_mut()[10..14]);
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

pub struct BroadcastDestinationProperty;

impl<B: Bytes> property::Property<B> for BroadcastDestinationProperty {
    type Type = ();
}

impl<'a, B: Bytes> property::ReadProperty<'a, B> for BroadcastDestinationProperty {
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        use crate::detail::Encode7Bit;

        if ux::u28::from_u7s(&buffer.buffer()[10..14]) == ux::u28::MAX {
            Ok(())
        } else {
            Err(crate::error::InvalidData(
                "Expected broadcast destination MUID",
            ))
        }
    }
    fn read(_buffer: &'a B) -> Self::Type {}
}

impl<B: Bytes + BufferMut> property::WriteProperty<B> for BroadcastDestinationProperty {
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::detail::Encode7Bit;
        ux::u28::MAX.to_u7s(&mut buffer.buffer_mut()[10..14]);
    }
    fn default() -> Self::Type {
        Default::default()
    }
}
