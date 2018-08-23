#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HOST_CTRL_CAP {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SDR50_SUPPORTR {
    bits: bool,
}
impl SDR50_SUPPORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SDR104_SUPPORTR {
    bits: bool,
}
impl SDR104_SUPPORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DDR50_SUPPORTR {
    bits: bool,
}
impl DDR50_SUPPORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TIME_COUNT_RETUNINGR {
    bits: u8,
}
impl TIME_COUNT_RETUNINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `USE_TUNING_SDR50`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE_TUNING_SDR50R {
    #[doc = "SDR does not require tuning"]
    USE_TUNING_SDR50_0,
    #[doc = "SDR50 requires tuning"]
    USE_TUNING_SDR50_1,
}
impl USE_TUNING_SDR50R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USE_TUNING_SDR50R::USE_TUNING_SDR50_0 => false,
            USE_TUNING_SDR50R::USE_TUNING_SDR50_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USE_TUNING_SDR50R {
        match value {
            false => USE_TUNING_SDR50R::USE_TUNING_SDR50_0,
            true => USE_TUNING_SDR50R::USE_TUNING_SDR50_1,
        }
    }
    #[doc = "Checks if the value of the field is `USE_TUNING_SDR50_0`"]
    #[inline]
    pub fn is_use_tuning_sdr50_0(&self) -> bool {
        *self == USE_TUNING_SDR50R::USE_TUNING_SDR50_0
    }
    #[doc = "Checks if the value of the field is `USE_TUNING_SDR50_1`"]
    #[inline]
    pub fn is_use_tuning_sdr50_1(&self) -> bool {
        *self == USE_TUNING_SDR50R::USE_TUNING_SDR50_1
    }
}
#[doc = "Possible values of the field `RETUNING_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETUNING_MODER {
    #[doc = "Mode 1"]
    RETUNING_MODE_0,
    #[doc = "Mode 2"]
    RETUNING_MODE_1,
    #[doc = "Mode 3"]
    RETUNING_MODE_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RETUNING_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RETUNING_MODER::RETUNING_MODE_0 => 0,
            RETUNING_MODER::RETUNING_MODE_1 => 1,
            RETUNING_MODER::RETUNING_MODE_2 => 2,
            RETUNING_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RETUNING_MODER {
        match value {
            0 => RETUNING_MODER::RETUNING_MODE_0,
            1 => RETUNING_MODER::RETUNING_MODE_1,
            2 => RETUNING_MODER::RETUNING_MODE_2,
            i => RETUNING_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RETUNING_MODE_0`"]
    #[inline]
    pub fn is_retuning_mode_0(&self) -> bool {
        *self == RETUNING_MODER::RETUNING_MODE_0
    }
    #[doc = "Checks if the value of the field is `RETUNING_MODE_1`"]
    #[inline]
    pub fn is_retuning_mode_1(&self) -> bool {
        *self == RETUNING_MODER::RETUNING_MODE_1
    }
    #[doc = "Checks if the value of the field is `RETUNING_MODE_2`"]
    #[inline]
    pub fn is_retuning_mode_2(&self) -> bool {
        *self == RETUNING_MODER::RETUNING_MODE_2
    }
}
#[doc = "Possible values of the field `MBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBLR {
    #[doc = "512 bytes"]
    MBL_0,
    #[doc = "1024 bytes"]
    MBL_1,
    #[doc = "2048 bytes"]
    MBL_2,
    #[doc = "4096 bytes"]
    MBL_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MBLR::MBL_0 => 0,
            MBLR::MBL_1 => 1,
            MBLR::MBL_2 => 2,
            MBLR::MBL_3 => 3,
            MBLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MBLR {
        match value {
            0 => MBLR::MBL_0,
            1 => MBLR::MBL_1,
            2 => MBLR::MBL_2,
            3 => MBLR::MBL_3,
            i => MBLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MBL_0`"]
    #[inline]
    pub fn is_mbl_0(&self) -> bool {
        *self == MBLR::MBL_0
    }
    #[doc = "Checks if the value of the field is `MBL_1`"]
    #[inline]
    pub fn is_mbl_1(&self) -> bool {
        *self == MBLR::MBL_1
    }
    #[doc = "Checks if the value of the field is `MBL_2`"]
    #[inline]
    pub fn is_mbl_2(&self) -> bool {
        *self == MBLR::MBL_2
    }
    #[doc = "Checks if the value of the field is `MBL_3`"]
    #[inline]
    pub fn is_mbl_3(&self) -> bool {
        *self == MBLR::MBL_3
    }
}
#[doc = "Possible values of the field `ADMAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMASR {
    #[doc = "Advanced DMA Not supported"]
    ADMAS_0,
    #[doc = "Advanced DMA Supported"]
    ADMAS_1,
}
impl ADMASR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADMASR::ADMAS_0 => false,
            ADMASR::ADMAS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMASR {
        match value {
            false => ADMASR::ADMAS_0,
            true => ADMASR::ADMAS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMAS_0`"]
    #[inline]
    pub fn is_admas_0(&self) -> bool {
        *self == ADMASR::ADMAS_0
    }
    #[doc = "Checks if the value of the field is `ADMAS_1`"]
    #[inline]
    pub fn is_admas_1(&self) -> bool {
        *self == ADMASR::ADMAS_1
    }
}
#[doc = "Possible values of the field `HSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSSR {
    #[doc = "High Speed Not Supported"]
    HSS_0,
    #[doc = "High Speed Supported"]
    HSS_1,
}
impl HSSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HSSR::HSS_0 => false,
            HSSR::HSS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSSR {
        match value {
            false => HSSR::HSS_0,
            true => HSSR::HSS_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSS_0`"]
    #[inline]
    pub fn is_hss_0(&self) -> bool {
        *self == HSSR::HSS_0
    }
    #[doc = "Checks if the value of the field is `HSS_1`"]
    #[inline]
    pub fn is_hss_1(&self) -> bool {
        *self == HSSR::HSS_1
    }
}
#[doc = "Possible values of the field `DMAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASR {
    #[doc = "DMA not supported"]
    DMAS_0,
    #[doc = "DMA Supported"]
    DMAS_1,
}
impl DMASR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMASR::DMAS_0 => false,
            DMASR::DMAS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMASR {
        match value {
            false => DMASR::DMAS_0,
            true => DMASR::DMAS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAS_0`"]
    #[inline]
    pub fn is_dmas_0(&self) -> bool {
        *self == DMASR::DMAS_0
    }
    #[doc = "Checks if the value of the field is `DMAS_1`"]
    #[inline]
    pub fn is_dmas_1(&self) -> bool {
        *self == DMASR::DMAS_1
    }
}
#[doc = "Possible values of the field `SRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSR {
    #[doc = "Not supported"]
    SRS_0,
    #[doc = "Supported"]
    SRS_1,
}
impl SRSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SRSR::SRS_0 => false,
            SRSR::SRS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSR {
        match value {
            false => SRSR::SRS_0,
            true => SRSR::SRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRS_0`"]
    #[inline]
    pub fn is_srs_0(&self) -> bool {
        *self == SRSR::SRS_0
    }
    #[doc = "Checks if the value of the field is `SRS_1`"]
    #[inline]
    pub fn is_srs_1(&self) -> bool {
        *self == SRSR::SRS_1
    }
}
#[doc = "Possible values of the field `VS33`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS33R {
    #[doc = "3.3V not supported"]
    VS33_0,
    #[doc = "3.3V supported"]
    VS33_1,
}
impl VS33R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VS33R::VS33_0 => false,
            VS33R::VS33_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VS33R {
        match value {
            false => VS33R::VS33_0,
            true => VS33R::VS33_1,
        }
    }
    #[doc = "Checks if the value of the field is `VS33_0`"]
    #[inline]
    pub fn is_vs33_0(&self) -> bool {
        *self == VS33R::VS33_0
    }
    #[doc = "Checks if the value of the field is `VS33_1`"]
    #[inline]
    pub fn is_vs33_1(&self) -> bool {
        *self == VS33R::VS33_1
    }
}
#[doc = "Possible values of the field `VS30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS30R {
    #[doc = "3.0V not supported"]
    VS30_0,
    #[doc = "3.0V supported"]
    VS30_1,
}
impl VS30R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VS30R::VS30_0 => false,
            VS30R::VS30_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VS30R {
        match value {
            false => VS30R::VS30_0,
            true => VS30R::VS30_1,
        }
    }
    #[doc = "Checks if the value of the field is `VS30_0`"]
    #[inline]
    pub fn is_vs30_0(&self) -> bool {
        *self == VS30R::VS30_0
    }
    #[doc = "Checks if the value of the field is `VS30_1`"]
    #[inline]
    pub fn is_vs30_1(&self) -> bool {
        *self == VS30R::VS30_1
    }
}
#[doc = "Possible values of the field `VS18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS18R {
    #[doc = "1.8V not supported"]
    VS18_0,
    #[doc = "1.8V supported"]
    VS18_1,
}
impl VS18R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VS18R::VS18_0 => false,
            VS18R::VS18_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VS18R {
        match value {
            false => VS18R::VS18_0,
            true => VS18R::VS18_1,
        }
    }
    #[doc = "Checks if the value of the field is `VS18_0`"]
    #[inline]
    pub fn is_vs18_0(&self) -> bool {
        *self == VS18R::VS18_0
    }
    #[doc = "Checks if the value of the field is `VS18_1`"]
    #[inline]
    pub fn is_vs18_1(&self) -> bool {
        *self == VS18R::VS18_1
    }
}
#[doc = r" Proxy"]
pub struct _TIME_COUNT_RETUNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _TIME_COUNT_RETUNINGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USE_TUNING_SDR50`"]
pub enum USE_TUNING_SDR50W {
    #[doc = "SDR does not require tuning"]
    USE_TUNING_SDR50_0,
    #[doc = "SDR50 requires tuning"]
    USE_TUNING_SDR50_1,
}
impl USE_TUNING_SDR50W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USE_TUNING_SDR50W::USE_TUNING_SDR50_0 => false,
            USE_TUNING_SDR50W::USE_TUNING_SDR50_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USE_TUNING_SDR50W<'a> {
    w: &'a mut W,
}
impl<'a> _USE_TUNING_SDR50W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USE_TUNING_SDR50W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SDR does not require tuning"]
    #[inline]
    pub fn use_tuning_sdr50_0(self) -> &'a mut W {
        self.variant(USE_TUNING_SDR50W::USE_TUNING_SDR50_0)
    }
    #[doc = "SDR50 requires tuning"]
    #[inline]
    pub fn use_tuning_sdr50_1(self) -> &'a mut W {
        self.variant(USE_TUNING_SDR50W::USE_TUNING_SDR50_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SDR50 support"]
    #[inline]
    pub fn sdr50_support(&self) -> SDR50_SUPPORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR50_SUPPORTR { bits }
    }
    #[doc = "Bit 1 - SDR104 support"]
    #[inline]
    pub fn sdr104_support(&self) -> SDR104_SUPPORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR104_SUPPORTR { bits }
    }
    #[doc = "Bit 2 - DDR50 support"]
    #[inline]
    pub fn ddr50_support(&self) -> DDR50_SUPPORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR50_SUPPORTR { bits }
    }
    #[doc = "Bits 8:11 - Time Counter for Retuning"]
    #[inline]
    pub fn time_count_retuning(&self) -> TIME_COUNT_RETUNINGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIME_COUNT_RETUNINGR { bits }
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline]
    pub fn use_tuning_sdr50(&self) -> USE_TUNING_SDR50R {
        USE_TUNING_SDR50R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Retuning Mode"]
    #[inline]
    pub fn retuning_mode(&self) -> RETUNING_MODER {
        RETUNING_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Max Block Length"]
    #[inline]
    pub fn mbl(&self) -> MBLR {
        MBLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - ADMA Support"]
    #[inline]
    pub fn admas(&self) -> ADMASR {
        ADMASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline]
    pub fn hss(&self) -> HSSR {
        HSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - DMA Support"]
    #[inline]
    pub fn dmas(&self) -> DMASR {
        DMASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline]
    pub fn srs(&self) -> SRSR {
        SRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline]
    pub fn vs33(&self) -> VS33R {
        VS33R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Voltage Support 3.0 V"]
    #[inline]
    pub fn vs30(&self) -> VS30R {
        VS30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Voltage Support 1.8 V"]
    #[inline]
    pub fn vs18(&self) -> VS18R {
        VS18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 133411847 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:11 - Time Counter for Retuning"]
    #[inline]
    pub fn time_count_retuning(&mut self) -> _TIME_COUNT_RETUNINGW {
        _TIME_COUNT_RETUNINGW { w: self }
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline]
    pub fn use_tuning_sdr50(&mut self) -> _USE_TUNING_SDR50W {
        _USE_TUNING_SDR50W { w: self }
    }
}
