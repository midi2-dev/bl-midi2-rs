pub trait FromCv2<T>: Sized {
    fn from_cv2(other: T) -> Self;
}

pub trait IntoCv1<T> {
    fn into_cv1(self) -> T;
}

impl<T, U> IntoCv1<U> for T
where
    U: FromCv2<T>,
{
    fn into_cv1(self) -> U {
        <U as FromCv2<T>>::from_cv2(self)
    }
}

pub trait TryFromCv2<T>: Sized {
    type Error;
    fn try_from_cv2(other: T) -> Result<Self, Self::Error>;
}

pub trait TryIntoCv1<T> {
    type Error;
    fn try_into_cv1(self) -> Result<T, Self::Error>;
}

impl<T, U> TryIntoCv1<U> for T
where
    U: TryFromCv2<T>,
{
    type Error = U::Error;
    fn try_into_cv1(self) -> Result<U, Self::Error> {
        <U as TryFromCv2<T>>::try_from_cv2(self)
    }
}

pub(crate) trait Center:
    Into<u32> + TryFrom<u32> + Sized + Copy + PartialEq + PartialOrd
where
    <Self as TryFrom<u32>>::Error: core::fmt::Debug,
{
    const MIN: Self;
    const MAX: Self;

    fn center_value() -> Self {
        let min: u32 = Self::MIN.into();
        let max: u32 = Self::MAX.into();

        ((max - min) / 2_u32 + 1)
            .try_into()
            .expect("Center shouldn't be larger than max.")
    }
}

trait Uxs {}
impl Uxs for ux::u7 {}

impl Center for ux::u7 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for ux::u9 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for ux::u14 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for u16 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for ux::u25 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for u32 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

pub(crate) trait MinCenterMax: Center + core::ops::Shr<u32>
where
    <Self as core::ops::Shr<u32>>::Output: Into<Self>,
    <Self as TryFrom<u32>>::Error: core::fmt::Debug,
{
    #[allow(dead_code)]
    fn mcm_upscale<
        U: Center
            + core::ops::Add<Output = U>
            + core::ops::BitAnd<Output = U>
            + core::fmt::Debug
            + core::ops::BitOr<Output = U>
            + core::ops::Shr<u32, Output = U>
            + core::ops::Shl<u32, Output = U>,
    >(
        self,
    ) -> U
    where
        Self: Into<U>,
        <U as TryFrom<u32>>::Error: core::fmt::Debug,
    {
        let min = Self::MIN;
        let center = Self::center_value();
        let max = Self::MAX;

        match self {
            s if s == min => U::MIN,
            s if s == max => U::MAX,
            s if s == center => U::center_value(),

            s if (min..center).contains(&s) => {
                let self_max: u32 = Self::MAX.into();
                let other_max: u32 = U::MAX.into();
                let shift = (other_max - self_max).count_ones();
                let other: U = self.into();
                other << shift
            }

            s if (center..max).contains(&s) => {
                let self_max: u32 = Self::MAX.into();
                let other_max: u32 = U::MAX.into();
                let self_bits = self_max.count_ones();
                let other_bits = other_max.count_ones();
                let shift = self_bits - 1;

                let initial: U = self.into();
                let initial_mask = U::MAX >> (other_bits - self_bits + 1);
                let repeating = initial & initial_mask;

                let mut mcm_upscaled = initial;
                let mut remainder = other_bits - self_bits;

                while remainder > 0 {
                    remainder = match remainder {
                        remainder if remainder > shift => {
                            mcm_upscaled = (mcm_upscaled << shift) | repeating;
                            remainder - shift
                        }
                        _ => {
                            let shift = shift - remainder;
                            mcm_upscaled = (mcm_upscaled << remainder) | (repeating >> shift);
                            0
                        }
                    };
                }
                mcm_upscaled
            }

            _ => self.into(),
        }
    }

    fn mcm_downscale<U: Center + TryFrom<Self>>(self) -> U
    where
        <U as TryFrom<u32>>::Error: core::fmt::Debug,
        <U as TryFrom<Self>>::Error: core::fmt::Debug,
    {
        let self_max: u32 = Self::MAX.into();
        let other_max: u32 = U::MAX.into();
        let shift = (self_max - other_max).count_ones();
        let mcm_downscaled = self >> shift;
        let mcm_downscaled: Self = mcm_downscaled.into();

        mcm_downscaled
            .try_into()
            .expect("Downscaling should not fail.")
    }
}

pub(crate) trait ZeroExtensionScaling: Center
where
    Self: core::ops::Add,
    <Self as TryFrom<u32>>::Error: core::fmt::Debug,
{
    fn ze_upscale<U: Center>(&self) -> U
    where
        <U as TryFrom<U>>::Error: core::fmt::Debug,
        <U as TryFrom<u32>>::Error: core::fmt::Debug,
    {
        let self_u32: u32 = (*self).into();
        let self_max: u32 = Self::MAX.into();
        let other_max: u32 = U::MAX.into();
        let shift = (other_max - self_max).count_ones();
        let ze_upscaled = self_u32 << shift;

        ze_upscaled.try_into().expect("Upscaling should not fail.")
    }

    fn ze_downscale<U: Center + TryFrom<Self>>(self) -> U
    where
        Self: core::fmt::Debug,
        Self: core::ops::Shr<u32, Output = Self>,
        Self: core::ops::Add<Self, Output = Self>,
        <U as TryFrom<u32>>::Error: core::fmt::Debug,
        <U as TryFrom<Self>>::Error: core::fmt::Debug,
    {
        let min = Self::MIN;
        let center = Self::center_value();
        let max = Self::MAX;

        match self {
            s if s == min => U::MIN,
            s if s == max => U::MAX,
            s if s == center => U::center_value(),
            _ => {
                let self_u64: u64 = self.into().into();
                let self_max: u64 = Self::MAX.into().into();
                let other_max: u64 = U::MAX.into().into();
                let shift: u32 = (self_max - other_max).count_ones().into();
                let half_scale_range: u64 = (1_u32 << (shift - 1_u32)).into();
                let shifted = (self_u64 + half_scale_range) >> shift;

                match TryInto::<u32>::try_into(shifted) {
                    Ok(ds) => match TryInto::<U>::try_into(ds) {
                        Ok(ds) => ds,
                        _ => U::MAX,
                    },
                    _ => U::MAX,
                }
            }
        }
    }
}

impl<U: Center + core::ops::Shr<u32>> MinCenterMax for U
where
    <U as TryFrom<u32>>::Error: core::fmt::Debug,
    <U as core::ops::Shr<u32>>::Output: Into<U>,
{
}

impl<U: Center + core::ops::Shr<u32> + core::ops::Add> ZeroExtensionScaling for U
where
    <U as TryFrom<u32>>::Error: core::fmt::Debug,
    <U as core::ops::Shr<u32>>::Output: Into<U>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcm_mins_upscaling() {
        let min_u7 = ux::u7::new(0);
        let min_u9 = ux::u9::new(0);
        let min_u14 = ux::u14::new(0);
        let min_u16 = 0_u16;
        let min_u32 = 0_u32;

        assert_eq!(min_u7.mcm_upscale::<ux::u7>(), min_u7);
        assert_eq!(min_u7.mcm_upscale::<ux::u9>(), min_u9);
        assert_eq!(min_u7.mcm_upscale::<ux::u14>(), min_u14);
        assert_eq!(min_u7.mcm_upscale::<u16>(), min_u16);
        assert_eq!(min_u7.mcm_upscale::<u32>(), min_u32);

        assert_eq!(min_u9.mcm_upscale::<ux::u9>(), min_u9);
        assert_eq!(min_u9.mcm_upscale::<ux::u14>(), min_u14);
        assert_eq!(min_u9.mcm_upscale::<u16>(), min_u16);
        assert_eq!(min_u9.mcm_upscale::<u32>(), min_u32);

        assert_eq!(min_u14.mcm_upscale::<ux::u14>(), min_u14);
        assert_eq!(min_u14.mcm_upscale::<u16>(), min_u16);
        assert_eq!(min_u14.mcm_upscale::<u32>(), min_u32);

        assert_eq!(min_u16.mcm_upscale::<u16>(), min_u16);
        assert_eq!(min_u16.mcm_upscale::<u32>(), min_u32);

        assert_eq!(min_u32.mcm_upscale::<u32>(), min_u32);
    }

    #[test]
    fn test_mcm_maxs_upscaling() {
        let max_u7 = ux::u7::new(127);
        let max_u9 = ux::u9::new(511);
        let max_u14 = ux::u14::new(16383);
        let max_u16 = 0xFFFF_u16;
        let max_u32 = 0xFFFFFFFF_u32;

        assert_eq!(max_u7.mcm_upscale::<ux::u7>(), max_u7);
        assert_eq!(max_u7.mcm_upscale::<ux::u9>(), max_u9);
        assert_eq!(max_u7.mcm_upscale::<ux::u14>(), max_u14);
        assert_eq!(max_u7.mcm_upscale::<u16>(), max_u16);
        assert_eq!(max_u7.mcm_upscale::<u32>(), max_u32);

        assert_eq!(max_u9.mcm_upscale::<ux::u9>(), max_u9);
        assert_eq!(max_u9.mcm_upscale::<ux::u14>(), max_u14);
        assert_eq!(max_u9.mcm_upscale::<u16>(), max_u16);
        assert_eq!(max_u9.mcm_upscale::<u32>(), max_u32);

        assert_eq!(max_u14.mcm_upscale::<ux::u14>(), max_u14);
        assert_eq!(max_u14.mcm_upscale::<u16>(), max_u16);
        assert_eq!(max_u14.mcm_upscale::<u32>(), max_u32);

        assert_eq!(max_u16.mcm_upscale::<u16>(), max_u16);
        assert_eq!(max_u16.mcm_upscale::<u32>(), max_u32);

        assert_eq!(max_u32.mcm_upscale::<u32>(), max_u32);
    }

    #[test]
    fn test_mcm_centers_upscaling() {
        let center_u7 = ux::u7::new(0x40);
        let center_u9 = ux::u9::new(256);
        let center_u14 = ux::u14::new(8192);
        let center_u16 = 0x8000_u16;
        let center_u32 = 0x80000000_u32;

        assert_eq!(center_u7.mcm_upscale::<ux::u7>(), center_u7);
        assert_eq!(center_u7.mcm_upscale::<ux::u9>(), center_u9);
        assert_eq!(center_u7.mcm_upscale::<ux::u14>(), center_u14);
        assert_eq!(center_u7.mcm_upscale::<u16>(), center_u16);
        assert_eq!(center_u7.mcm_upscale::<u32>(), center_u32);

        assert_eq!(center_u9.mcm_upscale::<ux::u9>(), center_u9);
        assert_eq!(center_u9.mcm_upscale::<ux::u14>(), center_u14);
        assert_eq!(center_u9.mcm_upscale::<u16>(), center_u16);
        assert_eq!(center_u9.mcm_upscale::<u32>(), center_u32);

        assert_eq!(center_u14.mcm_upscale::<ux::u14>(), center_u14);
        assert_eq!(center_u14.mcm_upscale::<u16>(), center_u16);
        assert_eq!(center_u14.mcm_upscale::<u32>(), center_u32);

        assert_eq!(center_u16.mcm_upscale::<u16>(), center_u16);
        assert_eq!(center_u16.mcm_upscale::<u32>(), center_u32);

        assert_eq!(center_u32.mcm_upscale::<u32>(), center_u32);
    }

    #[test]
    fn test_mcm_lower_range_upscaling() {
        let lower_u7 = ux::u7::new(0x1F);
        let lower_u9 = ux::u9::new(0x7C);
        let lower_u14 = ux::u14::new(0xF80);
        let lower_u16 = 0x3E00_u16;
        let lower_u32 = 0x3E00_0000_u32;

        assert_eq!(lower_u7.mcm_upscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u7.mcm_upscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u7.mcm_upscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u7.mcm_upscale::<u16>(), lower_u16);
        assert_eq!(lower_u7.mcm_upscale::<u32>(), lower_u32);

        assert_eq!(lower_u9.mcm_upscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u9.mcm_upscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u9.mcm_upscale::<u16>(), lower_u16);
        assert_eq!(lower_u9.mcm_upscale::<u32>(), lower_u32);

        assert_eq!(lower_u14.mcm_upscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u14.mcm_upscale::<u16>(), lower_u16);
        assert_eq!(lower_u14.mcm_upscale::<u32>(), lower_u32);

        assert_eq!(lower_u16.mcm_upscale::<u16>(), lower_u16);
        assert_eq!(lower_u16.mcm_upscale::<u32>(), lower_u32);

        assert_eq!(lower_u32.mcm_upscale::<u32>(), lower_u32);
    }

    #[test]
    fn test_mcm_upper_range_upscaling() {
        // Test cases from MIDI 2.0 Bit Scaling and Resolution v1.0.2
        let upper_u7_to_u16 = [
            (ux::u7::new(0x46).mcm_upscale::<u16>(), 0x8C30_u16),
            (ux::u7::new(0x60).mcm_upscale::<u16>(), 0xC104_u16),
            (ux::u7::new(0x78).mcm_upscale::<u16>(), 0xF1C7_u16),
        ];

        let upper_u7_to_u32 = [
            (ux::u7::new(0x46).mcm_upscale::<u32>(), 0x8C30C30C_u32),
            (ux::u7::new(0x60).mcm_upscale::<u32>(), 0xC1041041_u32),
            (ux::u7::new(0x78).mcm_upscale::<u32>(), 0xF1C71C71_u32),
        ];

        let upper_u16_to_u32 = [
            (0x9C40_u16.mcm_upscale::<u32>(), 0x9C403880_u32),
            (0xC000_u16.mcm_upscale::<u32>(), 0xC0008001_u32),
            (0xFDE8_u16.mcm_upscale::<u32>(), 0xFDE8FBD1_u32),
        ];

        for (result, expected) in upper_u7_to_u16 {
            assert_eq!(result, expected);
        }

        for (result, expected) in upper_u7_to_u32 {
            assert_eq!(result, expected);
        }

        for (result, expected) in upper_u16_to_u32 {
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_mcm_lower_range_downscaling() {
        let lower_u7 = ux::u7::new(31);
        let lower_u9 = ux::u9::new(127);
        let lower_u14 = ux::u14::new(4095);
        let lower_u16 = 0x3FFF_u16;
        let lower_u32 = 0x3FFFFFFF_u32;

        assert_eq!(lower_u32.mcm_downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u32.mcm_downscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u32.mcm_downscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u32.mcm_downscale::<u16>(), lower_u16);
        assert_eq!(lower_u32.mcm_downscale::<u32>(), lower_u32);

        assert_eq!(lower_u16.mcm_downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u16.mcm_downscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u16.mcm_downscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u16.mcm_downscale::<u16>(), lower_u16);

        assert_eq!(lower_u14.mcm_downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u14.mcm_downscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u14.mcm_downscale::<ux::u14>(), lower_u14);

        assert_eq!(lower_u9.mcm_downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u9.mcm_downscale::<ux::u9>(), lower_u9);

        assert_eq!(lower_u7.mcm_downscale::<ux::u7>(), lower_u7);
    }

    #[test]
    fn test_mcm_centers_downscaling() {
        let center_u7 = ux::u7::new(0x40);
        let center_u9 = ux::u9::new(256);
        let center_u14 = ux::u14::new(8192);
        let center_u16 = 0x8000_u16;
        let center_u32 = 0x80000000_u32;

        assert_eq!(center_u32.mcm_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u32.mcm_downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u32.mcm_downscale::<ux::u14>(), center_u14);
        assert_eq!(center_u32.mcm_downscale::<u16>(), center_u16);
        assert_eq!(center_u32.mcm_downscale::<u32>(), center_u32);

        assert_eq!(center_u16.mcm_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u16.mcm_downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u16.mcm_downscale::<ux::u14>(), center_u14);
        assert_eq!(center_u16.mcm_downscale::<u16>(), center_u16);

        assert_eq!(center_u14.mcm_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u14.mcm_downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u14.mcm_downscale::<ux::u14>(), center_u14);

        assert_eq!(center_u9.mcm_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u9.mcm_downscale::<ux::u9>(), center_u9);

        assert_eq!(center_u7.mcm_downscale::<ux::u7>(), center_u7);
    }

    #[test]
    fn test_mcm_mins_downscaling() {
        let min_u7 = ux::u7::new(0);
        let min_u9 = ux::u9::new(0);
        let min_u14 = ux::u14::new(0);
        let min_u16 = 0_u16;
        let min_u32 = 0_u32;

        assert_eq!(min_u32.mcm_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u32.mcm_downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u32.mcm_downscale::<ux::u14>(), min_u14);
        assert_eq!(min_u32.mcm_downscale::<u16>(), min_u16);
        assert_eq!(min_u32.mcm_downscale::<u32>(), min_u32);

        assert_eq!(min_u16.mcm_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u16.mcm_downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u16.mcm_downscale::<ux::u14>(), min_u14);
        assert_eq!(min_u16.mcm_downscale::<u16>(), min_u16);

        assert_eq!(min_u14.mcm_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u14.mcm_downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u14.mcm_downscale::<ux::u14>(), min_u14);

        assert_eq!(min_u9.mcm_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u9.mcm_downscale::<ux::u9>(), min_u9);

        assert_eq!(min_u7.mcm_downscale::<ux::u7>(), min_u7);
    }

    #[test]
    fn test_mcm_maxs_downscaling() {
        let max_u7 = ux::u7::new(127);
        let max_u9 = ux::u9::new(511);
        let max_u14 = ux::u14::new(16383);
        let max_u16 = 0xFFFF_u16;
        let max_u32 = 0xFFFFFFFF_u32;

        assert_eq!(max_u32.mcm_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u32.mcm_downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u32.mcm_downscale::<ux::u14>(), max_u14);
        assert_eq!(max_u32.mcm_downscale::<u16>(), max_u16);
        assert_eq!(max_u32.mcm_downscale::<u32>(), max_u32);

        assert_eq!(max_u16.mcm_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u16.mcm_downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u16.mcm_downscale::<ux::u14>(), max_u14);
        assert_eq!(max_u16.mcm_downscale::<u16>(), max_u16);

        assert_eq!(max_u14.mcm_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u14.mcm_downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u14.mcm_downscale::<ux::u14>(), max_u14);

        assert_eq!(max_u9.mcm_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u9.mcm_downscale::<ux::u9>(), max_u9);

        assert_eq!(max_u7.mcm_downscale::<ux::u7>(), max_u7);
    }

    #[test]
    fn test_mcm_example_downscaling() {
        let examples = [
            (5120_u16.mcm_downscale::<ux::u7>(), ux::u7::new(10)),
            (32768_u16.mcm_downscale::<ux::u7>(), ux::u7::new(64)),
            (44730_u16.mcm_downscale::<ux::u7>(), ux::u7::new(87)),
            (65535_u16.mcm_downscale::<ux::u7>(), ux::u7::new(127)),
        ];

        for (input, expected) in examples {
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn test_ze_centers_downscaling() {
        let center_u7 = ux::u7::new(0x40);
        let center_u9 = ux::u9::new(256);
        let center_u14 = ux::u14::new(8192);
        let center_u25 = ux::u25::new(0x0100_0000);
        let center_u16 = 0x8000_u16;
        let center_u32 = 0x80000000_u32;

        assert_eq!(center_u32.ze_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u32.ze_downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u32.ze_downscale::<ux::u14>(), center_u14);
        assert_eq!(center_u32.ze_downscale::<ux::u25>(), center_u25);
        assert_eq!(center_u32.ze_downscale::<u16>(), center_u16);
        assert_eq!(center_u32.ze_downscale::<u32>(), center_u32);

        assert_eq!(center_u25.ze_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u25.ze_downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u25.ze_downscale::<ux::u14>(), center_u14);
        assert_eq!(center_u25.ze_downscale::<u16>(), center_u16);
        assert_eq!(center_u25.ze_downscale::<ux::u25>(), center_u25);

        assert_eq!(center_u16.ze_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u16.ze_downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u16.ze_downscale::<ux::u14>(), center_u14);
        assert_eq!(center_u16.ze_downscale::<u16>(), center_u16);

        assert_eq!(center_u14.ze_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u14.ze_downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u14.ze_downscale::<ux::u14>(), center_u14);

        assert_eq!(center_u9.ze_downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u9.ze_downscale::<ux::u9>(), center_u9);

        assert_eq!(center_u7.ze_downscale::<ux::u7>(), center_u7);
    }

    #[test]
    fn test_ze_mins_downscaling() {
        let min_u7 = ux::u7::new(0);
        let min_u9 = ux::u9::new(0);
        let min_u14 = ux::u14::new(0);
        let min_u25 = ux::u25::new(0);
        let min_u16 = 0_u16;
        let min_u32 = 0_u32;

        assert_eq!(min_u32.ze_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u32.ze_downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u32.ze_downscale::<ux::u14>(), min_u14);
        assert_eq!(min_u32.ze_downscale::<ux::u25>(), min_u25);
        assert_eq!(min_u32.ze_downscale::<u16>(), min_u16);
        assert_eq!(min_u32.ze_downscale::<u32>(), min_u32);

        assert_eq!(min_u16.ze_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u16.ze_downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u16.ze_downscale::<ux::u14>(), min_u14);
        assert_eq!(min_u16.ze_downscale::<u16>(), min_u16);

        assert_eq!(min_u14.ze_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u14.ze_downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u14.ze_downscale::<ux::u14>(), min_u14);

        assert_eq!(min_u25.ze_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u25.ze_downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u25.ze_downscale::<ux::u14>(), min_u14);
        assert_eq!(min_u25.ze_downscale::<u16>(), min_u16);
        assert_eq!(min_u25.ze_downscale::<ux::u25>(), min_u25);

        assert_eq!(min_u9.ze_downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u9.ze_downscale::<ux::u9>(), min_u9);

        assert_eq!(min_u7.ze_downscale::<ux::u7>(), min_u7);
    }

    #[test]
    fn test_ze_maxs_downscaling() {
        let max_u7 = ux::u7::new(127);
        let max_u9 = ux::u9::new(511);
        let max_u14 = ux::u14::new(16383);
        let max_u25 = ux::u25::new(0x1FFFFFF);
        let max_u16 = 0xFFFF_u16;
        let max_u32 = 0xFFFFFFFF_u32;

        assert_eq!(max_u32.ze_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u32.ze_downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u32.ze_downscale::<ux::u14>(), max_u14);
        assert_eq!(max_u32.ze_downscale::<ux::u25>(), max_u25);
        assert_eq!(max_u32.ze_downscale::<u16>(), max_u16);
        assert_eq!(max_u32.ze_downscale::<u32>(), max_u32);

        assert_eq!(max_u25.ze_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u25.ze_downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u25.ze_downscale::<ux::u14>(), max_u14);
        assert_eq!(max_u25.ze_downscale::<u16>(), max_u16);
        assert_eq!(max_u25.ze_downscale::<ux::u25>(), max_u25);

        assert_eq!(max_u16.ze_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u16.ze_downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u16.ze_downscale::<ux::u14>(), max_u14);
        assert_eq!(max_u16.ze_downscale::<u16>(), max_u16);

        assert_eq!(max_u14.ze_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u14.ze_downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u14.ze_downscale::<ux::u14>(), max_u14);

        assert_eq!(max_u9.ze_downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u9.ze_downscale::<ux::u9>(), max_u9);

        assert_eq!(max_u7.ze_downscale::<ux::u7>(), max_u7);
    }

    #[test]
    fn test_ze_example_downscaling() {
        let examples = [
            (5120_u16.ze_downscale::<ux::u7>(), ux::u7::new(10)),
            (5631_u16.ze_downscale::<ux::u7>(), ux::u7::new(11)),
            (32768_u16.ze_downscale::<ux::u7>(), ux::u7::new(64)),
            (44544_u16.ze_downscale::<ux::u7>(), ux::u7::new(87)),
            (44730_u16.ze_downscale::<ux::u7>(), ux::u7::new(87)),
            (44800_u16.ze_downscale::<ux::u7>(), ux::u7::new(88)),
        ];

        for (input, expected) in examples {
            std::dbg!(input, expected);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn test_ze_example_upscaling() {
        let examples = [
            (ux::u7::new(10).ze_upscale::<u16>(), 5120_u16),
            (ux::u7::new(64).ze_upscale::<u16>(), 32768_u16),
            (ux::u7::new(87).ze_upscale::<u16>(), 44544_u16),
            (ux::u7::new(127).ze_upscale::<u16>(), 65024_u16),
        ];

        for (input, expected) in examples {
            assert_eq!(input, expected);
        }
    }
}
