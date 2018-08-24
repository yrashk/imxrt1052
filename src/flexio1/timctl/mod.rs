#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCTL {
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
#[doc = "Possible values of the field `TIMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMODR {
    #[doc = "Timer Disabled."]
    TIMOD_0,
    #[doc = "Dual 8-bit counters baud mode."]
    TIMOD_1,
    #[doc = "Dual 8-bit counters PWM high mode."]
    TIMOD_2,
    #[doc = "Single 16-bit counter mode."]
    TIMOD_3,
}
impl TIMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMODR::TIMOD_0 => 0,
            TIMODR::TIMOD_1 => 1,
            TIMODR::TIMOD_2 => 2,
            TIMODR::TIMOD_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMODR {
        match value {
            0 => TIMODR::TIMOD_0,
            1 => TIMODR::TIMOD_1,
            2 => TIMODR::TIMOD_2,
            3 => TIMODR::TIMOD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMOD_0`"]
    #[inline]
    pub fn is_timod_0(&self) -> bool {
        *self == TIMODR::TIMOD_0
    }
    #[doc = "Checks if the value of the field is `TIMOD_1`"]
    #[inline]
    pub fn is_timod_1(&self) -> bool {
        *self == TIMODR::TIMOD_1
    }
    #[doc = "Checks if the value of the field is `TIMOD_2`"]
    #[inline]
    pub fn is_timod_2(&self) -> bool {
        *self == TIMODR::TIMOD_2
    }
    #[doc = "Checks if the value of the field is `TIMOD_3`"]
    #[inline]
    pub fn is_timod_3(&self) -> bool {
        *self == TIMODR::TIMOD_3
    }
}
#[doc = "Possible values of the field `PINPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPOLR {
    #[doc = "Pin is active high"]
    PINPOL_0,
    #[doc = "Pin is active low"]
    PINPOL_1,
}
impl PINPOLR {
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
            PINPOLR::PINPOL_0 => false,
            PINPOLR::PINPOL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINPOLR {
        match value {
            false => PINPOLR::PINPOL_0,
            true => PINPOLR::PINPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PINPOL_0`"]
    #[inline]
    pub fn is_pinpol_0(&self) -> bool {
        *self == PINPOLR::PINPOL_0
    }
    #[doc = "Checks if the value of the field is `PINPOL_1`"]
    #[inline]
    pub fn is_pinpol_1(&self) -> bool {
        *self == PINPOLR::PINPOL_1
    }
}
#[doc = r" Value of the field"]
pub struct PINSELR {
    bits: u8,
}
impl PINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PINCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCFGR {
    #[doc = "Timer pin output disabled"]
    PINCFG_0,
    #[doc = "Timer pin open drain or bidirectional output enable"]
    PINCFG_1,
    #[doc = "Timer pin bidirectional output data"]
    PINCFG_2,
    #[doc = "Timer pin output"]
    PINCFG_3,
}
impl PINCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINCFGR::PINCFG_0 => 0,
            PINCFGR::PINCFG_1 => 1,
            PINCFGR::PINCFG_2 => 2,
            PINCFGR::PINCFG_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINCFGR {
        match value {
            0 => PINCFGR::PINCFG_0,
            1 => PINCFGR::PINCFG_1,
            2 => PINCFGR::PINCFG_2,
            3 => PINCFGR::PINCFG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PINCFG_0`"]
    #[inline]
    pub fn is_pincfg_0(&self) -> bool {
        *self == PINCFGR::PINCFG_0
    }
    #[doc = "Checks if the value of the field is `PINCFG_1`"]
    #[inline]
    pub fn is_pincfg_1(&self) -> bool {
        *self == PINCFGR::PINCFG_1
    }
    #[doc = "Checks if the value of the field is `PINCFG_2`"]
    #[inline]
    pub fn is_pincfg_2(&self) -> bool {
        *self == PINCFGR::PINCFG_2
    }
    #[doc = "Checks if the value of the field is `PINCFG_3`"]
    #[inline]
    pub fn is_pincfg_3(&self) -> bool {
        *self == PINCFGR::PINCFG_3
    }
}
#[doc = "Possible values of the field `TRGSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSRCR {
    #[doc = "External trigger selected"]
    TRGSRC_0,
    #[doc = "Internal trigger selected"]
    TRGSRC_1,
}
impl TRGSRCR {
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
            TRGSRCR::TRGSRC_0 => false,
            TRGSRCR::TRGSRC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGSRCR {
        match value {
            false => TRGSRCR::TRGSRC_0,
            true => TRGSRCR::TRGSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRGSRC_0`"]
    #[inline]
    pub fn is_trgsrc_0(&self) -> bool {
        *self == TRGSRCR::TRGSRC_0
    }
    #[doc = "Checks if the value of the field is `TRGSRC_1`"]
    #[inline]
    pub fn is_trgsrc_1(&self) -> bool {
        *self == TRGSRCR::TRGSRC_1
    }
}
#[doc = "Possible values of the field `TRGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPOLR {
    #[doc = "Trigger active high"]
    TRGPOL_0,
    #[doc = "Trigger active low"]
    TRGPOL_1,
}
impl TRGPOLR {
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
            TRGPOLR::TRGPOL_0 => false,
            TRGPOLR::TRGPOL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGPOLR {
        match value {
            false => TRGPOLR::TRGPOL_0,
            true => TRGPOLR::TRGPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRGPOL_0`"]
    #[inline]
    pub fn is_trgpol_0(&self) -> bool {
        *self == TRGPOLR::TRGPOL_0
    }
    #[doc = "Checks if the value of the field is `TRGPOL_1`"]
    #[inline]
    pub fn is_trgpol_1(&self) -> bool {
        *self == TRGPOLR::TRGPOL_1
    }
}
#[doc = r" Value of the field"]
pub struct TRGSELR {
    bits: u8,
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TIMOD`"]
pub enum TIMODW {
    #[doc = "Timer Disabled."]
    TIMOD_0,
    #[doc = "Dual 8-bit counters baud mode."]
    TIMOD_1,
    #[doc = "Dual 8-bit counters PWM high mode."]
    TIMOD_2,
    #[doc = "Single 16-bit counter mode."]
    TIMOD_3,
}
impl TIMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMODW::TIMOD_0 => 0,
            TIMODW::TIMOD_1 => 1,
            TIMODW::TIMOD_2 => 2,
            TIMODW::TIMOD_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMODW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Disabled."]
    #[inline]
    pub fn timod_0(self) -> &'a mut W {
        self.variant(TIMODW::TIMOD_0)
    }
    #[doc = "Dual 8-bit counters baud mode."]
    #[inline]
    pub fn timod_1(self) -> &'a mut W {
        self.variant(TIMODW::TIMOD_1)
    }
    #[doc = "Dual 8-bit counters PWM high mode."]
    #[inline]
    pub fn timod_2(self) -> &'a mut W {
        self.variant(TIMODW::TIMOD_2)
    }
    #[doc = "Single 16-bit counter mode."]
    #[inline]
    pub fn timod_3(self) -> &'a mut W {
        self.variant(TIMODW::TIMOD_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINPOL`"]
pub enum PINPOLW {
    #[doc = "Pin is active high"]
    PINPOL_0,
    #[doc = "Pin is active low"]
    PINPOL_1,
}
impl PINPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINPOLW::PINPOL_0 => false,
            PINPOLW::PINPOL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PINPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is active high"]
    #[inline]
    pub fn pinpol_0(self) -> &'a mut W {
        self.variant(PINPOLW::PINPOL_0)
    }
    #[doc = "Pin is active low"]
    #[inline]
    pub fn pinpol_1(self) -> &'a mut W {
        self.variant(PINPOLW::PINPOL_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PINSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINCFG`"]
pub enum PINCFGW {
    #[doc = "Timer pin output disabled"]
    PINCFG_0,
    #[doc = "Timer pin open drain or bidirectional output enable"]
    PINCFG_1,
    #[doc = "Timer pin bidirectional output data"]
    PINCFG_2,
    #[doc = "Timer pin output"]
    PINCFG_3,
}
impl PINCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINCFGW::PINCFG_0 => 0,
            PINCFGW::PINCFG_1 => 1,
            PINCFGW::PINCFG_2 => 2,
            PINCFGW::PINCFG_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer pin output disabled"]
    #[inline]
    pub fn pincfg_0(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_0)
    }
    #[doc = "Timer pin open drain or bidirectional output enable"]
    #[inline]
    pub fn pincfg_1(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_1)
    }
    #[doc = "Timer pin bidirectional output data"]
    #[inline]
    pub fn pincfg_2(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_2)
    }
    #[doc = "Timer pin output"]
    #[inline]
    pub fn pincfg_3(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGSRC`"]
pub enum TRGSRCW {
    #[doc = "External trigger selected"]
    TRGSRC_0,
    #[doc = "Internal trigger selected"]
    TRGSRC_1,
}
impl TRGSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGSRCW::TRGSRC_0 => false,
            TRGSRCW::TRGSRC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger selected"]
    #[inline]
    pub fn trgsrc_0(self) -> &'a mut W {
        self.variant(TRGSRCW::TRGSRC_0)
    }
    #[doc = "Internal trigger selected"]
    #[inline]
    pub fn trgsrc_1(self) -> &'a mut W {
        self.variant(TRGSRCW::TRGSRC_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGPOL`"]
pub enum TRGPOLW {
    #[doc = "Trigger active high"]
    TRGPOL_0,
    #[doc = "Trigger active low"]
    TRGPOL_1,
}
impl TRGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGPOLW::TRGPOL_0 => false,
            TRGPOLW::TRGPOL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger active high"]
    #[inline]
    pub fn trgpol_0(self) -> &'a mut W {
        self.variant(TRGPOLW::TRGPOL_0)
    }
    #[doc = "Trigger active low"]
    #[inline]
    pub fn trgpol_1(self) -> &'a mut W {
        self.variant(TRGPOLW::TRGPOL_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline]
    pub fn timod(&self) -> TIMODR {
        TIMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline]
    pub fn pinpol(&self) -> PINPOLR {
        PINPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Timer Pin Select"]
    #[inline]
    pub fn pinsel(&self) -> PINSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PINSELR { bits }
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline]
    pub fn pincfg(&self) -> PINCFGR {
        PINCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&self) -> TRGSRCR {
        TRGSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline]
    pub fn trgpol(&self) -> TRGPOLR {
        TRGPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - Trigger Select"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGSELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline]
    pub fn timod(&mut self) -> _TIMODW {
        _TIMODW { w: self }
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline]
    pub fn pinpol(&mut self) -> _PINPOLW {
        _PINPOLW { w: self }
    }
    #[doc = "Bits 8:12 - Timer Pin Select"]
    #[inline]
    pub fn pinsel(&mut self) -> _PINSELW {
        _PINSELW { w: self }
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline]
    pub fn pincfg(&mut self) -> _PINCFGW {
        _PINCFGW { w: self }
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&mut self) -> _TRGSRCW {
        _TRGSRCW { w: self }
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline]
    pub fn trgpol(&mut self) -> _TRGPOLW {
        _TRGPOLW { w: self }
    }
    #[doc = "Bits 24:29 - Trigger Select"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
}
