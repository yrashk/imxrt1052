#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXIC {
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
pub struct ICTTR {
    bits: u16,
}
impl ICTTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ICFTR {
    bits: u8,
}
impl ICFTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ICCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICCSR {
    #[doc = "Use MII/GMII TX clocks."]
    ICCS_0,
    #[doc = "Use ENET system clock."]
    ICCS_1,
}
impl ICCSR {
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
            ICCSR::ICCS_0 => false,
            ICCSR::ICCS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICCSR {
        match value {
            false => ICCSR::ICCS_0,
            true => ICCSR::ICCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ICCS_0`"]
    #[inline]
    pub fn is_iccs_0(&self) -> bool {
        *self == ICCSR::ICCS_0
    }
    #[doc = "Checks if the value of the field is `ICCS_1`"]
    #[inline]
    pub fn is_iccs_1(&self) -> bool {
        *self == ICCSR::ICCS_1
    }
}
#[doc = "Possible values of the field `ICEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICENR {
    #[doc = "Disable Interrupt coalescing."]
    ICEN_0,
    #[doc = "Enable Interrupt coalescing."]
    ICEN_1,
}
impl ICENR {
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
            ICENR::ICEN_0 => false,
            ICENR::ICEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICENR {
        match value {
            false => ICENR::ICEN_0,
            true => ICENR::ICEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ICEN_0`"]
    #[inline]
    pub fn is_icen_0(&self) -> bool {
        *self == ICENR::ICEN_0
    }
    #[doc = "Checks if the value of the field is `ICEN_1`"]
    #[inline]
    pub fn is_icen_1(&self) -> bool {
        *self == ICENR::ICEN_1
    }
}
#[doc = r" Proxy"]
pub struct _ICTTW<'a> {
    w: &'a mut W,
}
impl<'a> _ICTTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ICFTW<'a> {
    w: &'a mut W,
}
impl<'a> _ICFTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICCS`"]
pub enum ICCSW {
    #[doc = "Use MII/GMII TX clocks."]
    ICCS_0,
    #[doc = "Use ENET system clock."]
    ICCS_1,
}
impl ICCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICCSW::ICCS_0 => false,
            ICCSW::ICCS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICCSW<'a> {
    w: &'a mut W,
}
impl<'a> _ICCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use MII/GMII TX clocks."]
    #[inline]
    pub fn iccs_0(self) -> &'a mut W {
        self.variant(ICCSW::ICCS_0)
    }
    #[doc = "Use ENET system clock."]
    #[inline]
    pub fn iccs_1(self) -> &'a mut W {
        self.variant(ICCSW::ICCS_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICEN`"]
pub enum ICENW {
    #[doc = "Disable Interrupt coalescing."]
    ICEN_0,
    #[doc = "Enable Interrupt coalescing."]
    ICEN_1,
}
impl ICENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICENW::ICEN_0 => false,
            ICENW::ICEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICENW<'a> {
    w: &'a mut W,
}
impl<'a> _ICENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Interrupt coalescing."]
    #[inline]
    pub fn icen_0(self) -> &'a mut W {
        self.variant(ICENW::ICEN_0)
    }
    #[doc = "Enable Interrupt coalescing."]
    #[inline]
    pub fn icen_1(self) -> &'a mut W {
        self.variant(ICENW::ICEN_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:15 - Interrupt coalescing timer threshold"]
    #[inline]
    pub fn ictt(&self) -> ICTTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ICTTR { bits }
    }
    #[doc = "Bits 20:27 - Interrupt coalescing frame count threshold"]
    #[inline]
    pub fn icft(&self) -> ICFTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ICFTR { bits }
    }
    #[doc = "Bit 30 - Interrupt Coalescing Timer Clock Source Select"]
    #[inline]
    pub fn iccs(&self) -> ICCSR {
        ICCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable"]
    #[inline]
    pub fn icen(&self) -> ICENR {
        ICENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:15 - Interrupt coalescing timer threshold"]
    #[inline]
    pub fn ictt(&mut self) -> _ICTTW {
        _ICTTW { w: self }
    }
    #[doc = "Bits 20:27 - Interrupt coalescing frame count threshold"]
    #[inline]
    pub fn icft(&mut self) -> _ICFTW {
        _ICFTW { w: self }
    }
    #[doc = "Bit 30 - Interrupt Coalescing Timer Clock Source Select"]
    #[inline]
    pub fn iccs(&mut self) -> _ICCSW {
        _ICCSW { w: self }
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable"]
    #[inline]
    pub fn icen(&mut self) -> _ICENW {
        _ICENW { w: self }
    }
}
