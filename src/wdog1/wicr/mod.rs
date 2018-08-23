#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::WICR {
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
#[doc = "Possible values of the field `WICT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WICTR {
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 0 seconds."]
    WICT_0,
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 0.5 seconds."]
    WICT_1,
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 2 seconds (Default)."]
    WICT_4,
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 127.5 seconds."]
    WICT_255,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WICTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WICTR::WICT_0 => 0,
            WICTR::WICT_1 => 1,
            WICTR::WICT_4 => 4,
            WICTR::WICT_255 => 255,
            WICTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WICTR {
        match value {
            0 => WICTR::WICT_0,
            1 => WICTR::WICT_1,
            4 => WICTR::WICT_4,
            255 => WICTR::WICT_255,
            i => WICTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WICT_0`"]
    #[inline]
    pub fn is_wict_0(&self) -> bool {
        *self == WICTR::WICT_0
    }
    #[doc = "Checks if the value of the field is `WICT_1`"]
    #[inline]
    pub fn is_wict_1(&self) -> bool {
        *self == WICTR::WICT_1
    }
    #[doc = "Checks if the value of the field is `WICT_4`"]
    #[inline]
    pub fn is_wict_4(&self) -> bool {
        *self == WICTR::WICT_4
    }
    #[doc = "Checks if the value of the field is `WICT_255`"]
    #[inline]
    pub fn is_wict_255(&self) -> bool {
        *self == WICTR::WICT_255
    }
}
#[doc = "Possible values of the field `WTIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTISR {
    #[doc = "No interrupt has occurred (Default)."]
    WTIS_0,
    #[doc = "Interrupt has occurred"]
    WTIS_1,
}
impl WTISR {
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
            WTISR::WTIS_0 => false,
            WTISR::WTIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTISR {
        match value {
            false => WTISR::WTIS_0,
            true => WTISR::WTIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `WTIS_0`"]
    #[inline]
    pub fn is_wtis_0(&self) -> bool {
        *self == WTISR::WTIS_0
    }
    #[doc = "Checks if the value of the field is `WTIS_1`"]
    #[inline]
    pub fn is_wtis_1(&self) -> bool {
        *self == WTISR::WTIS_1
    }
}
#[doc = "Possible values of the field `WIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIER {
    #[doc = "Disable Interrupt (Default)."]
    WIE_0,
    #[doc = "Enable Interrupt."]
    WIE_1,
}
impl WIER {
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
            WIER::WIE_0 => false,
            WIER::WIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIER {
        match value {
            false => WIER::WIE_0,
            true => WIER::WIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIE_0`"]
    #[inline]
    pub fn is_wie_0(&self) -> bool {
        *self == WIER::WIE_0
    }
    #[doc = "Checks if the value of the field is `WIE_1`"]
    #[inline]
    pub fn is_wie_1(&self) -> bool {
        *self == WIER::WIE_1
    }
}
#[doc = "Values that can be written to the field `WICT`"]
pub enum WICTW {
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 0 seconds."]
    WICT_0,
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 0.5 seconds."]
    WICT_1,
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 2 seconds (Default)."]
    WICT_4,
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 127.5 seconds."]
    WICT_255,
}
impl WICTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WICTW::WICT_0 => 0,
            WICTW::WICT_1 => 1,
            WICTW::WICT_4 => 4,
            WICTW::WICT_255 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WICTW<'a> {
    w: &'a mut W,
}
impl<'a> _WICTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WICTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 0 seconds."]
    #[inline]
    pub fn wict_0(self) -> &'a mut W {
        self.variant(WICTW::WICT_0)
    }
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 0.5 seconds."]
    #[inline]
    pub fn wict_1(self) -> &'a mut W {
        self.variant(WICTW::WICT_1)
    }
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 2 seconds (Default)."]
    #[inline]
    pub fn wict_4(self) -> &'a mut W {
        self.variant(WICTW::WICT_4)
    }
    #[doc = "WICT[7:0] = Time duration between interrupt and time-out is 127.5 seconds."]
    #[inline]
    pub fn wict_255(self) -> &'a mut W {
        self.variant(WICTW::WICT_255)
    }
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
#[doc = "Values that can be written to the field `WTIS`"]
pub enum WTISW {
    #[doc = "No interrupt has occurred (Default)."]
    WTIS_0,
    #[doc = "Interrupt has occurred"]
    WTIS_1,
}
impl WTISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTISW::WTIS_0 => false,
            WTISW::WTIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTISW<'a> {
    w: &'a mut W,
}
impl<'a> _WTISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt has occurred (Default)."]
    #[inline]
    pub fn wtis_0(self) -> &'a mut W {
        self.variant(WTISW::WTIS_0)
    }
    #[doc = "Interrupt has occurred"]
    #[inline]
    pub fn wtis_1(self) -> &'a mut W {
        self.variant(WTISW::WTIS_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WIE`"]
pub enum WIEW {
    #[doc = "Disable Interrupt (Default)."]
    WIE_0,
    #[doc = "Enable Interrupt."]
    WIE_1,
}
impl WIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WIEW::WIE_0 => false,
            WIEW::WIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Interrupt (Default)."]
    #[inline]
    pub fn wie_0(self) -> &'a mut W {
        self.variant(WIEW::WIE_0)
    }
    #[doc = "Enable Interrupt."]
    #[inline]
    pub fn wie_1(self) -> &'a mut W {
        self.variant(WIEW::WIE_1)
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
    #[doc = "Bits 0:7 - WICT"]
    #[inline]
    pub fn wict(&self) -> WICTR {
        WICTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 14 - WTIS"]
    #[inline]
    pub fn wtis(&self) -> WTISR {
        WTISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - WIE"]
    #[inline]
    pub fn wie(&self) -> WIER {
        WIER::_from({
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
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - WICT"]
    #[inline]
    pub fn wict(&mut self) -> _WICTW {
        _WICTW { w: self }
    }
    #[doc = "Bit 14 - WTIS"]
    #[inline]
    pub fn wtis(&mut self) -> _WTISW {
        _WTISW { w: self }
    }
    #[doc = "Bit 15 - WIE"]
    #[inline]
    pub fn wie(&mut self) -> _WIEW {
        _WIEW { w: self }
    }
}
