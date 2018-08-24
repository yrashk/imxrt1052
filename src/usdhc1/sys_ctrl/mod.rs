#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYS_CTRL {
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
#[doc = "Possible values of the field `DVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVSR {
    #[doc = "Divide-by-1"]
    DVS_0,
    #[doc = "Divide-by-2"]
    DVS_1,
    #[doc = "Divide-by-15"]
    DVS_14,
    #[doc = "Divide-by-16"]
    DVS_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DVSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DVSR::DVS_0 => 0,
            DVSR::DVS_1 => 1,
            DVSR::DVS_14 => 14,
            DVSR::DVS_15 => 15,
            DVSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DVSR {
        match value {
            0 => DVSR::DVS_0,
            1 => DVSR::DVS_1,
            14 => DVSR::DVS_14,
            15 => DVSR::DVS_15,
            i => DVSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DVS_0`"]
    #[inline]
    pub fn is_dvs_0(&self) -> bool {
        *self == DVSR::DVS_0
    }
    #[doc = "Checks if the value of the field is `DVS_1`"]
    #[inline]
    pub fn is_dvs_1(&self) -> bool {
        *self == DVSR::DVS_1
    }
    #[doc = "Checks if the value of the field is `DVS_14`"]
    #[inline]
    pub fn is_dvs_14(&self) -> bool {
        *self == DVSR::DVS_14
    }
    #[doc = "Checks if the value of the field is `DVS_15`"]
    #[inline]
    pub fn is_dvs_15(&self) -> bool {
        *self == DVSR::DVS_15
    }
}
#[doc = r" Value of the field"]
pub struct SDCLKFSR {
    bits: u8,
}
impl SDCLKFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DTOCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOCVR {
    #[doc = "no description available"]
    DTOCV_0,
    #[doc = "no description available"]
    DTOCV_1,
    #[doc = "no description available"]
    DTOCV_13,
    #[doc = "no description available"]
    DTOCV_14,
    #[doc = "no description available"]
    DTOCV_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTOCVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTOCVR::DTOCV_0 => 0,
            DTOCVR::DTOCV_1 => 1,
            DTOCVR::DTOCV_13 => 13,
            DTOCVR::DTOCV_14 => 14,
            DTOCVR::DTOCV_15 => 15,
            DTOCVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTOCVR {
        match value {
            0 => DTOCVR::DTOCV_0,
            1 => DTOCVR::DTOCV_1,
            13 => DTOCVR::DTOCV_13,
            14 => DTOCVR::DTOCV_14,
            15 => DTOCVR::DTOCV_15,
            i => DTOCVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DTOCV_0`"]
    #[inline]
    pub fn is_dtocv_0(&self) -> bool {
        *self == DTOCVR::DTOCV_0
    }
    #[doc = "Checks if the value of the field is `DTOCV_1`"]
    #[inline]
    pub fn is_dtocv_1(&self) -> bool {
        *self == DTOCVR::DTOCV_1
    }
    #[doc = "Checks if the value of the field is `DTOCV_13`"]
    #[inline]
    pub fn is_dtocv_13(&self) -> bool {
        *self == DTOCVR::DTOCV_13
    }
    #[doc = "Checks if the value of the field is `DTOCV_14`"]
    #[inline]
    pub fn is_dtocv_14(&self) -> bool {
        *self == DTOCVR::DTOCV_14
    }
    #[doc = "Checks if the value of the field is `DTOCV_15`"]
    #[inline]
    pub fn is_dtocv_15(&self) -> bool {
        *self == DTOCVR::DTOCV_15
    }
}
#[doc = r" Value of the field"]
pub struct IPP_RST_NR {
    bits: bool,
}
impl IPP_RST_NR {
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
#[doc = "Possible values of the field `RSTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTAR {
    #[doc = "No Reset"]
    RSTA_0,
    #[doc = "Reset"]
    RSTA_1,
}
impl RSTAR {
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
            RSTAR::RSTA_0 => false,
            RSTAR::RSTA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTAR {
        match value {
            false => RSTAR::RSTA_0,
            true => RSTAR::RSTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTA_0`"]
    #[inline]
    pub fn is_rsta_0(&self) -> bool {
        *self == RSTAR::RSTA_0
    }
    #[doc = "Checks if the value of the field is `RSTA_1`"]
    #[inline]
    pub fn is_rsta_1(&self) -> bool {
        *self == RSTAR::RSTA_1
    }
}
#[doc = "Possible values of the field `RSTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCR {
    #[doc = "No Reset"]
    RSTC_0,
    #[doc = "Reset"]
    RSTC_1,
}
impl RSTCR {
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
            RSTCR::RSTC_0 => false,
            RSTCR::RSTC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTCR {
        match value {
            false => RSTCR::RSTC_0,
            true => RSTCR::RSTC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTC_0`"]
    #[inline]
    pub fn is_rstc_0(&self) -> bool {
        *self == RSTCR::RSTC_0
    }
    #[doc = "Checks if the value of the field is `RSTC_1`"]
    #[inline]
    pub fn is_rstc_1(&self) -> bool {
        *self == RSTCR::RSTC_1
    }
}
#[doc = "Possible values of the field `RSTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTDR {
    #[doc = "No Reset"]
    RSTD_0,
    #[doc = "Reset"]
    RSTD_1,
}
impl RSTDR {
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
            RSTDR::RSTD_0 => false,
            RSTDR::RSTD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTDR {
        match value {
            false => RSTDR::RSTD_0,
            true => RSTDR::RSTD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTD_0`"]
    #[inline]
    pub fn is_rstd_0(&self) -> bool {
        *self == RSTDR::RSTD_0
    }
    #[doc = "Checks if the value of the field is `RSTD_1`"]
    #[inline]
    pub fn is_rstd_1(&self) -> bool {
        *self == RSTDR::RSTD_1
    }
}
#[doc = r" Value of the field"]
pub struct INITAR {
    bits: bool,
}
impl INITAR {
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
pub struct RSTTR {
    bits: bool,
}
impl RSTTR {
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
#[doc = "Values that can be written to the field `DVS`"]
pub enum DVSW {
    #[doc = "Divide-by-1"]
    DVS_0,
    #[doc = "Divide-by-2"]
    DVS_1,
    #[doc = "Divide-by-15"]
    DVS_14,
    #[doc = "Divide-by-16"]
    DVS_15,
}
impl DVSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DVSW::DVS_0 => 0,
            DVSW::DVS_1 => 1,
            DVSW::DVS_14 => 14,
            DVSW::DVS_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DVSW<'a> {
    w: &'a mut W,
}
impl<'a> _DVSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DVSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide-by-1"]
    #[inline]
    pub fn dvs_0(self) -> &'a mut W {
        self.variant(DVSW::DVS_0)
    }
    #[doc = "Divide-by-2"]
    #[inline]
    pub fn dvs_1(self) -> &'a mut W {
        self.variant(DVSW::DVS_1)
    }
    #[doc = "Divide-by-15"]
    #[inline]
    pub fn dvs_14(self) -> &'a mut W {
        self.variant(DVSW::DVS_14)
    }
    #[doc = "Divide-by-16"]
    #[inline]
    pub fn dvs_15(self) -> &'a mut W {
        self.variant(DVSW::DVS_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDCLKFSW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKFSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTOCV`"]
pub enum DTOCVW {
    #[doc = "no description available"]
    DTOCV_0,
    #[doc = "no description available"]
    DTOCV_1,
    #[doc = "no description available"]
    DTOCV_13,
    #[doc = "no description available"]
    DTOCV_14,
    #[doc = "no description available"]
    DTOCV_15,
}
impl DTOCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTOCVW::DTOCV_0 => 0,
            DTOCVW::DTOCV_1 => 1,
            DTOCVW::DTOCV_13 => 13,
            DTOCVW::DTOCV_14 => 14,
            DTOCVW::DTOCV_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOCVW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOCVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn dtocv_0(self) -> &'a mut W {
        self.variant(DTOCVW::DTOCV_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn dtocv_1(self) -> &'a mut W {
        self.variant(DTOCVW::DTOCV_1)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn dtocv_13(self) -> &'a mut W {
        self.variant(DTOCVW::DTOCV_13)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn dtocv_14(self) -> &'a mut W {
        self.variant(DTOCVW::DTOCV_14)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn dtocv_15(self) -> &'a mut W {
        self.variant(DTOCVW::DTOCV_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IPP_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _IPP_RST_NW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTA`"]
pub enum RSTAW {
    #[doc = "No Reset"]
    RSTA_0,
    #[doc = "Reset"]
    RSTA_1,
}
impl RSTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTAW::RSTA_0 => false,
            RSTAW::RSTA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTAW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Reset"]
    #[inline]
    pub fn rsta_0(self) -> &'a mut W {
        self.variant(RSTAW::RSTA_0)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn rsta_1(self) -> &'a mut W {
        self.variant(RSTAW::RSTA_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTC`"]
pub enum RSTCW {
    #[doc = "No Reset"]
    RSTC_0,
    #[doc = "Reset"]
    RSTC_1,
}
impl RSTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTCW::RSTC_0 => false,
            RSTCW::RSTC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Reset"]
    #[inline]
    pub fn rstc_0(self) -> &'a mut W {
        self.variant(RSTCW::RSTC_0)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn rstc_1(self) -> &'a mut W {
        self.variant(RSTCW::RSTC_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTD`"]
pub enum RSTDW {
    #[doc = "No Reset"]
    RSTD_0,
    #[doc = "Reset"]
    RSTD_1,
}
impl RSTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTDW::RSTD_0 => false,
            RSTDW::RSTD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTDW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Reset"]
    #[inline]
    pub fn rstd_0(self) -> &'a mut W {
        self.variant(RSTDW::RSTD_0)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn rstd_1(self) -> &'a mut W {
        self.variant(RSTDW::RSTD_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INITAW<'a> {
    w: &'a mut W,
}
impl<'a> _INITAW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSTTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTTW<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 4:7 - Divisor"]
    #[inline]
    pub fn dvs(&self) -> DVSR {
        DVSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclkfs(&self) -> SDCLKFSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDCLKFSR { bits }
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline]
    pub fn dtocv(&self) -> DTOCVR {
        DTOCVR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - IPP_RST_N"]
    #[inline]
    pub fn ipp_rst_n(&self) -> IPP_RST_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPP_RST_NR { bits }
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline]
    pub fn rsta(&self) -> RSTAR {
        RSTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline]
    pub fn rstc(&self) -> RSTCR {
        RSTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Software Reset For DATA Line"]
    #[inline]
    pub fn rstd(&self) -> RSTDR {
        RSTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline]
    pub fn inita(&self) -> INITAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INITAR { bits }
    }
    #[doc = "Bit 28 - Reset Tuning"]
    #[inline]
    pub fn rstt(&self) -> RSTTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSTTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2155905039 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:7 - Divisor"]
    #[inline]
    pub fn dvs(&mut self) -> _DVSW {
        _DVSW { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclkfs(&mut self) -> _SDCLKFSW {
        _SDCLKFSW { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline]
    pub fn dtocv(&mut self) -> _DTOCVW {
        _DTOCVW { w: self }
    }
    #[doc = "Bit 23 - IPP_RST_N"]
    #[inline]
    pub fn ipp_rst_n(&mut self) -> _IPP_RST_NW {
        _IPP_RST_NW { w: self }
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline]
    pub fn rsta(&mut self) -> _RSTAW {
        _RSTAW { w: self }
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline]
    pub fn rstc(&mut self) -> _RSTCW {
        _RSTCW { w: self }
    }
    #[doc = "Bit 26 - Software Reset For DATA Line"]
    #[inline]
    pub fn rstd(&mut self) -> _RSTDW {
        _RSTDW { w: self }
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline]
    pub fn inita(&mut self) -> _INITAW {
        _INITAW { w: self }
    }
    #[doc = "Bit 28 - Reset Tuning"]
    #[inline]
    pub fn rstt(&mut self) -> _RSTTW {
        _RSTTW { w: self }
    }
}
