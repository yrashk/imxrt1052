#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FFILT0 {
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
pub struct FILT_PERR {
    bits: u8,
}
impl FILT_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FILT_CNTR {
    bits: u8,
}
impl FILT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `GSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSTRR {
    #[doc = "Fault input glitch stretching is disabled."]
    GSTR_0,
    #[doc = "Input fault signals will be stretched to at least 2 IPBus clock cycles."]
    GSTR_1,
}
impl GSTRR {
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
            GSTRR::GSTR_0 => false,
            GSTRR::GSTR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GSTRR {
        match value {
            false => GSTRR::GSTR_0,
            true => GSTRR::GSTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `GSTR_0`"]
    #[inline]
    pub fn is_gstr_0(&self) -> bool {
        *self == GSTRR::GSTR_0
    }
    #[doc = "Checks if the value of the field is `GSTR_1`"]
    #[inline]
    pub fn is_gstr_1(&self) -> bool {
        *self == GSTRR::GSTR_1
    }
}
#[doc = r" Proxy"]
pub struct _FILT_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILT_PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILT_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FILT_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GSTR`"]
pub enum GSTRW {
    #[doc = "Fault input glitch stretching is disabled."]
    GSTR_0,
    #[doc = "Input fault signals will be stretched to at least 2 IPBus clock cycles."]
    GSTR_1,
}
impl GSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GSTRW::GSTR_0 => false,
            GSTRW::GSTR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _GSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input glitch stretching is disabled."]
    #[inline]
    pub fn gstr_0(self) -> &'a mut W {
        self.variant(GSTRW::GSTR_0)
    }
    #[doc = "Input fault signals will be stretched to at least 2 IPBus clock cycles."]
    #[inline]
    pub fn gstr_1(self) -> &'a mut W {
        self.variant(GSTRW::GSTR_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:7 - Fault Filter Period"]
    #[inline]
    pub fn filt_per(&self) -> FILT_PERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        FILT_PERR { bits }
    }
    #[doc = "Bits 8:10 - Fault Filter Count"]
    #[inline]
    pub fn filt_cnt(&self) -> FILT_CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        FILT_CNTR { bits }
    }
    #[doc = "Bit 15 - Fault Glitch Stretch Enable"]
    #[inline]
    pub fn gstr(&self) -> GSTRR {
        GSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Fault Filter Period"]
    #[inline]
    pub fn filt_per(&mut self) -> _FILT_PERW {
        _FILT_PERW { w: self }
    }
    #[doc = "Bits 8:10 - Fault Filter Count"]
    #[inline]
    pub fn filt_cnt(&mut self) -> _FILT_CNTW {
        _FILT_CNTW { w: self }
    }
    #[doc = "Bit 15 - Fault Glitch Stretch Enable"]
    #[inline]
    pub fn gstr(&mut self) -> _GSTRW {
        _GSTRW { w: self }
    }
}
