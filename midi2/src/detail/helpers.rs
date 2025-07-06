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

pub fn try_splice_sysex_data<
    B: crate::buffer::Buffer + crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    S: SysexInternal<B>,
    D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
    R: core::ops::RangeBounds<usize>,
>(
    sysex: &mut S,
    data: D,
    range: R,
) -> core::result::Result<(), crate::error::BufferOverflow> {
    match detail::try_splice_sysex_data(sysex, data, |s, sz| s.try_resize(sz), range) {
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

pub fn splice_sysex_data<
    B: crate::buffer::Buffer + crate::buffer::BufferMut + crate::buffer::BufferResize,
    S: SysexInternal<B>,
    D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
    R: core::ops::RangeBounds<usize>,
>(
    sysex: &mut S,
    data: D,
    range: R,
) {
    detail::try_splice_sysex_data(
        sysex,
        data,
        |s, sz| {
            s.resize(sz);
            Ok(())
        },
        range,
    )
    .expect("Resizable buffers should not fail here")
}

mod detail {
    use crate::error::BufferOverflow;

    use super::*;

    pub fn try_splice_sysex_data<
        B: crate::buffer::Buffer + crate::buffer::BufferMut,
        S: crate::traits::SysexInternal<B>,
        D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
        R: Fn(&mut S, usize) -> core::result::Result<(), SysexTryResizeError>,
        Rg: core::ops::RangeBounds<usize>,
    >(
        sysex: &mut S,
        data: D,
        resize: R,
        range: Rg,
    ) -> core::result::Result<(), crate::error::BufferOverflow> {
        // reformat first to ensure data is optimally filling the
        // underlying buffer
        sysex.compact();

        // get an initial estimate for the size of the data
        let initial_size = sysex.payload_size();

        let splice_begin = match range.start_bound() {
            core::ops::Bound::Included(&v) => v,
            core::ops::Bound::Excluded(&v) => v + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let splice_end = match range.end_bound() {
            core::ops::Bound::Included(&v) => v + 1,
            core::ops::Bound::Excluded(&v) => v,
            core::ops::Bound::Unbounded => initial_size,
        };
        let splice_size = splice_end - splice_begin;

        let mut running_data_size_estimate = match data.size_hint() {
            (_, Some(upper)) => upper,
            // not the optimal case - could lead to additional copying
            (lower, None) => lower,
        };
        let mut written = 0;
        let mut additional_size_for_overflow = 1;
        let mut data = data.peekable();

        if splice_end < splice_end + running_data_size_estimate - splice_size {
            // we need to grow
            // initial buffer resize
            if let Err(SysexTryResizeError(sz)) = resize(
                sysex,
                running_data_size_estimate + initial_size - splice_size,
            ) {
                // failed. we'll work with what we've got
                running_data_size_estimate = sz.saturating_sub(initial_size - splice_size);
            };
        }

        let mut tail = splice_end + running_data_size_estimate - splice_size;
        sysex.move_payload_tail(splice_end, tail);

        'main: loop {
            while written < running_data_size_estimate {
                match data.next() {
                    Some(v) => {
                        sysex.write_datum(v, splice_begin + written);
                        written += 1;
                    }
                    None => {
                        break 'main;
                    }
                }
            }
            debug_assert_eq!(written, running_data_size_estimate);

            if data.peek().is_none() {
                // done
                break;
            }

            // we underestimated.
            running_data_size_estimate += additional_size_for_overflow;

            {
                let mut to = splice_begin + running_data_size_estimate;
                if tail < to {
                    // we need to grow
                    // resize to make more space
                    // and move tail

                    if let Err(SysexTryResizeError(sz)) = resize(
                        sysex,
                        running_data_size_estimate + initial_size - splice_size,
                    ) {
                        // failed. we'll work with what we've got
                        running_data_size_estimate = sz.saturating_sub(initial_size - splice_size);
                        to = splice_begin + running_data_size_estimate - splice_size;
                    };
                }

                sysex.move_payload_tail(tail, to);
                tail = splice_begin + running_data_size_estimate;
            }

            additional_size_for_overflow *= 2;

            if written >= running_data_size_estimate {
                return Err(BufferOverflow);
            }
        }

        // we ensure the buffer is the correct size and move the tail
        // to the final position
        sysex.move_payload_tail(tail, splice_begin + written);
        resize(sysex, written + initial_size - splice_size)
            .map_err(|_| crate::error::BufferOverflow)?;

        Ok(())
    }
}
