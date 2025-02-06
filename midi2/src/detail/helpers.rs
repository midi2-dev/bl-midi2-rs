use crate::traits::{SysexInternal, SysexTryResizeError};

#[cfg(any(feature = "sysex7", feature = "sysex8", feature = "flex-data"))]
pub fn group_from_packet(p: &[u32]) -> crate::ux::u4 {
    use crate::detail::BitOps;
    p[0].nibble(1)
}

pub const ERR_INCONSISTENT_GROUPS: &str = "Inconsistent groups across packets";

#[cfg(any(feature = "sysex7", feature = "sysex8", feature = "flex-data"))]
pub fn sysex_group_consistent_groups(
    buffer: &[u32],
    stride: usize,
    ump_type: crate::ux::u4,
) -> Result<(), crate::error::InvalidData> {
    use crate::detail::BitOps;
    use group_from_packet as gfp;
    if buffer
        .chunks_exact(stride)
        .take_while(|chunk| chunk[0].nibble(0) == ump_type)
        .all(|chunk| gfp(chunk) == gfp(buffer))
    {
        Ok(())
    } else {
        Err(crate::error::InvalidData(ERR_INCONSISTENT_GROUPS))
    }
}

pub const ERR_SYSEX_EXPECTED_COMPLETE: &str = "Expected Complete packet";
pub const ERR_SYSEX_EXPECTED_BEGIN: &str = "Expected Begin packet";
pub const ERR_SYSEX_EXPECTED_CONTINUE: &str = "Expected Continue packet";
pub const ERR_SYSEX_EXPECTED_END: &str = "Expected End packet";
pub const ERR_EMPTY_MESSAGE: &str = "The message buffer is empty";

// assumes that buffer contains valid messages
#[cfg(any(
    feature = "sysex7",
    feature = "sysex8",
    feature = "flex-data",
    feature = "ump-stream"
))]
pub fn validate_sysex_group_statuses<
    IsComplete: Fn(&[u32]) -> bool,
    IsBegin: Fn(&[u32]) -> bool,
    IsContinue: Fn(&[u32]) -> bool,
    IsEnd: Fn(&[u32]) -> bool,
>(
    buffer: &[u32],
    is_complete: IsComplete,
    is_begin: IsBegin,
    is_continue: IsContinue,
    is_end: IsEnd,
    stride: usize,
    ump_type: crate::ux::u4,
) -> Result<(), crate::error::InvalidData> {
    use crate::{detail::BitOps, error::InvalidData};

    let mut iter = buffer
        .chunks(stride)
        .take_while(|chunk| chunk[0].nibble(0) == ump_type)
        .peekable();

    let Some(first_packet) = iter.next() else {
        return Err(InvalidData(ERR_EMPTY_MESSAGE));
    };

    if iter.peek().is_none() {
        if is_complete(first_packet) {
            return Ok(());
        } else {
            return Err(InvalidData(ERR_SYSEX_EXPECTED_COMPLETE));
        }
    }

    if !is_begin(first_packet) {
        return Err(InvalidData(ERR_SYSEX_EXPECTED_BEGIN));
    }

    while let Some(chunk) = iter.next() {
        if iter.peek().is_some() && !is_continue(chunk) {
            return Err(InvalidData(ERR_SYSEX_EXPECTED_CONTINUE));
        }
        if iter.peek().is_none() && !is_end(chunk) {
            return Err(InvalidData(ERR_SYSEX_EXPECTED_END));
        }
    }

    Ok(())
}

pub fn try_set_sysex_data<
    B: crate::buffer::Buffer + crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    S: SysexInternal<B>,
    D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
>(
    sysex: &mut S,
    data: D,
) -> core::result::Result<(), crate::error::BufferOverflow> {
    match detail::try_set_sysex_data(sysex, data, |s, sz| s.try_resize(sz)) {
        Err(e) => {
            // if the write failed we reset the message
            // back to zero data
            sysex
                .try_resize(0)
                .map_err(|_| crate::error::BufferOverflow)?;
            Err(e)
        }
        Ok(()) => Ok(()),
    }
}

pub fn set_sysex_data<
    B: crate::buffer::Buffer + crate::buffer::BufferMut + crate::buffer::BufferResize,
    S: SysexInternal<B>,
    D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
>(
    sysex: &mut S,
    data: D,
) {
    detail::try_set_sysex_data(sysex, data, |s, sz| {
        s.resize(sz);
        Ok(())
    })
    .expect("Resizable buffers should not fail here")
}

mod detail {
    use super::*;

    pub fn try_set_sysex_data<
        B: crate::buffer::Buffer + crate::buffer::BufferMut,
        S: crate::traits::SysexInternal<B>,
        D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
        R: Fn(&mut S, usize) -> core::result::Result<(), SysexTryResizeError>,
    >(
        sysex: &mut S,
        data: D,
        resize: R,
    ) -> core::result::Result<(), crate::error::BufferOverflow> {
        // get an initial estimate for the size of the data
        let mut running_data_size_estimate = match data.size_hint() {
            (_, Some(upper)) => upper,
            // not the optimal case - could lead to additional copying
            (lower, None) => lower,
        };
        let mut written = 0;
        let mut additional_size_for_overflow = 1;
        let mut data = data.peekable();

        // initial buffer resize
        if let Err(SysexTryResizeError(sz)) = resize(sysex, running_data_size_estimate) {
            // failed. we'll work with what we've got
            running_data_size_estimate = sz;
        };

        'main: loop {
            while written < running_data_size_estimate {
                match data.next() {
                    Some(v) => {
                        sysex.write_datum(v, written);
                        written += 1;
                    }
                    None => {
                        break 'main;
                    }
                }
            }
            assert_eq!(written, running_data_size_estimate);

            // we underestimated.
            // resize to make more space
            running_data_size_estimate += additional_size_for_overflow;
            additional_size_for_overflow *= 2;

            if data.peek().is_some() {
                if let Err(SysexTryResizeError(sz)) = resize(sysex, running_data_size_estimate) {
                    // failed. we'll work with what we've got
                    running_data_size_estimate = sz;
                };
            }

            if written >= running_data_size_estimate {
                return Err(Default::default());
            }
        }

        if written < running_data_size_estimate {
            // we shrink the buffer back down to the correct size
            resize(sysex, written).map_err(|_| crate::error::BufferOverflow)?;
        }

        Ok(())
    }
}
