#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCTRL {
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
#[doc = "Possible values of the field `TEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TENR {
    #[doc = "Timer n is disabled."]
    TEN_0,
    #[doc = "Timer n is enabled."]
    TEN_1,
}
impl TENR {
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
            TENR::TEN_0 => false,
            TENR::TEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TENR {
        match value {
            false => TENR::TEN_0,
            true => TENR::TEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEN_0`"]
    #[inline]
    pub fn is_ten_0(&self) -> bool {
        *self == TENR::TEN_0
    }
    #[doc = "Checks if the value of the field is `TEN_1`"]
    #[inline]
    pub fn is_ten_1(&self) -> bool {
        *self == TENR::TEN_1
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Interrupt requests from Timer n are disabled."]
    TIE_0,
    #[doc = "Interrupt will be requested whenever TIF is set."]
    TIE_1,
}
impl TIER {
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
            TIER::TIE_0 => false,
            TIER::TIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::TIE_0,
            true => TIER::TIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE_0`"]
    #[inline]
    pub fn is_tie_0(&self) -> bool {
        *self == TIER::TIE_0
    }
    #[doc = "Checks if the value of the field is `TIE_1`"]
    #[inline]
    pub fn is_tie_1(&self) -> bool {
        *self == TIER::TIE_1
    }
}
#[doc = "Possible values of the field `CHN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHNR {
    #[doc = "Timer is not chained."]
    CHN_0,
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    CHN_1,
}
impl CHNR {
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
            CHNR::CHN_0 => false,
            CHNR::CHN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHNR {
        match value {
            false => CHNR::CHN_0,
            true => CHNR::CHN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CHN_0`"]
    #[inline]
    pub fn is_chn_0(&self) -> bool {
        *self == CHNR::CHN_0
    }
    #[doc = "Checks if the value of the field is `CHN_1`"]
    #[inline]
    pub fn is_chn_1(&self) -> bool {
        *self == CHNR::CHN_1
    }
}
#[doc = "Values that can be written to the field `TEN`"]
pub enum TENW {
    #[doc = "Timer n is disabled."]
    TEN_0,
    #[doc = "Timer n is enabled."]
    TEN_1,
}
impl TENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TENW::TEN_0 => false,
            TENW::TEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TENW<'a> {
    w: &'a mut W,
}
impl<'a> _TENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer n is disabled."]
    #[inline]
    pub fn ten_0(self) -> &'a mut W {
        self.variant(TENW::TEN_0)
    }
    #[doc = "Timer n is enabled."]
    #[inline]
    pub fn ten_1(self) -> &'a mut W {
        self.variant(TENW::TEN_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Interrupt requests from Timer n are disabled."]
    TIE_0,
    #[doc = "Interrupt will be requested whenever TIF is set."]
    TIE_1,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::TIE_0 => false,
            TIEW::TIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline]
    pub fn tie_0(self) -> &'a mut W {
        self.variant(TIEW::TIE_0)
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline]
    pub fn tie_1(self) -> &'a mut W {
        self.variant(TIEW::TIE_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHN`"]
pub enum CHNW {
    #[doc = "Timer is not chained."]
    CHN_0,
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    CHN_1,
}
impl CHNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHNW::CHN_0 => false,
            CHNW::CHN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHNW<'a> {
    w: &'a mut W,
}
impl<'a> _CHNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer is not chained."]
    #[inline]
    pub fn chn_0(self) -> &'a mut W {
        self.variant(CHNW::CHN_0)
    }
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    #[inline]
    pub fn chn_1(self) -> &'a mut W {
        self.variant(CHNW::CHN_1)
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
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Timer Enable"]
    #[inline]
    pub fn ten(&self) -> TENR {
        TENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline]
    pub fn chn(&self) -> CHNR {
        CHNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Timer Enable"]
    #[inline]
    pub fn ten(&mut self) -> _TENW {
        _TENW { w: self }
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline]
    pub fn chn(&mut self) -> _CHNW {
        _CHNW { w: self }
    }
}
