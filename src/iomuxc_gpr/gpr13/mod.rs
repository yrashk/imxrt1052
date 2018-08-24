#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR13 {
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
#[doc = "Possible values of the field `ARCACHE_USDHC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARCACHE_USDHCR {
    #[doc = "Cacheable attribute is off for read transactions."]
    ARCACHE_USDHC_0,
    #[doc = "Cacheable attribute is on for read transactions."]
    ARCACHE_USDHC_1,
}
impl ARCACHE_USDHCR {
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
            ARCACHE_USDHCR::ARCACHE_USDHC_0 => false,
            ARCACHE_USDHCR::ARCACHE_USDHC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARCACHE_USDHCR {
        match value {
            false => ARCACHE_USDHCR::ARCACHE_USDHC_0,
            true => ARCACHE_USDHCR::ARCACHE_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARCACHE_USDHC_0`"]
    #[inline]
    pub fn is_arcache_usdhc_0(&self) -> bool {
        *self == ARCACHE_USDHCR::ARCACHE_USDHC_0
    }
    #[doc = "Checks if the value of the field is `ARCACHE_USDHC_1`"]
    #[inline]
    pub fn is_arcache_usdhc_1(&self) -> bool {
        *self == ARCACHE_USDHCR::ARCACHE_USDHC_1
    }
}
#[doc = "Possible values of the field `AWCACHE_USDHC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWCACHE_USDHCR {
    #[doc = "Cacheable attribute is off for write transactions."]
    AWCACHE_USDHC_0,
    #[doc = "Cacheable attribute is on for write transactions."]
    AWCACHE_USDHC_1,
}
impl AWCACHE_USDHCR {
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
            AWCACHE_USDHCR::AWCACHE_USDHC_0 => false,
            AWCACHE_USDHCR::AWCACHE_USDHC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWCACHE_USDHCR {
        match value {
            false => AWCACHE_USDHCR::AWCACHE_USDHC_0,
            true => AWCACHE_USDHCR::AWCACHE_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWCACHE_USDHC_0`"]
    #[inline]
    pub fn is_awcache_usdhc_0(&self) -> bool {
        *self == AWCACHE_USDHCR::AWCACHE_USDHC_0
    }
    #[doc = "Checks if the value of the field is `AWCACHE_USDHC_1`"]
    #[inline]
    pub fn is_awcache_usdhc_1(&self) -> bool {
        *self == AWCACHE_USDHCR::AWCACHE_USDHC_1
    }
}
#[doc = "Possible values of the field `CACHE_ENET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_ENETR {
    #[doc = "Cacheable attribute is off for read/write transactions."]
    CACHE_ENET_0,
    #[doc = "Cacheable attribute is on for read/write transactions."]
    CACHE_ENET_1,
}
impl CACHE_ENETR {
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
            CACHE_ENETR::CACHE_ENET_0 => false,
            CACHE_ENETR::CACHE_ENET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHE_ENETR {
        match value {
            false => CACHE_ENETR::CACHE_ENET_0,
            true => CACHE_ENETR::CACHE_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHE_ENET_0`"]
    #[inline]
    pub fn is_cache_enet_0(&self) -> bool {
        *self == CACHE_ENETR::CACHE_ENET_0
    }
    #[doc = "Checks if the value of the field is `CACHE_ENET_1`"]
    #[inline]
    pub fn is_cache_enet_1(&self) -> bool {
        *self == CACHE_ENETR::CACHE_ENET_1
    }
}
#[doc = "Possible values of the field `CACHE_USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_USBR {
    #[doc = "Cacheable attribute is off for read/write transactions."]
    CACHE_USB_0,
    #[doc = "Cacheable attribute is on for read/write transactions."]
    CACHE_USB_1,
}
impl CACHE_USBR {
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
            CACHE_USBR::CACHE_USB_0 => false,
            CACHE_USBR::CACHE_USB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHE_USBR {
        match value {
            false => CACHE_USBR::CACHE_USB_0,
            true => CACHE_USBR::CACHE_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHE_USB_0`"]
    #[inline]
    pub fn is_cache_usb_0(&self) -> bool {
        *self == CACHE_USBR::CACHE_USB_0
    }
    #[doc = "Checks if the value of the field is `CACHE_USB_1`"]
    #[inline]
    pub fn is_cache_usb_1(&self) -> bool {
        *self == CACHE_USBR::CACHE_USB_1
    }
}
#[doc = "Values that can be written to the field `ARCACHE_USDHC`"]
pub enum ARCACHE_USDHCW {
    #[doc = "Cacheable attribute is off for read transactions."]
    ARCACHE_USDHC_0,
    #[doc = "Cacheable attribute is on for read transactions."]
    ARCACHE_USDHC_1,
}
impl ARCACHE_USDHCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARCACHE_USDHCW::ARCACHE_USDHC_0 => false,
            ARCACHE_USDHCW::ARCACHE_USDHC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARCACHE_USDHCW<'a> {
    w: &'a mut W,
}
impl<'a> _ARCACHE_USDHCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARCACHE_USDHCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cacheable attribute is off for read transactions."]
    #[inline]
    pub fn arcache_usdhc_0(self) -> &'a mut W {
        self.variant(ARCACHE_USDHCW::ARCACHE_USDHC_0)
    }
    #[doc = "Cacheable attribute is on for read transactions."]
    #[inline]
    pub fn arcache_usdhc_1(self) -> &'a mut W {
        self.variant(ARCACHE_USDHCW::ARCACHE_USDHC_1)
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
#[doc = "Values that can be written to the field `AWCACHE_USDHC`"]
pub enum AWCACHE_USDHCW {
    #[doc = "Cacheable attribute is off for write transactions."]
    AWCACHE_USDHC_0,
    #[doc = "Cacheable attribute is on for write transactions."]
    AWCACHE_USDHC_1,
}
impl AWCACHE_USDHCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWCACHE_USDHCW::AWCACHE_USDHC_0 => false,
            AWCACHE_USDHCW::AWCACHE_USDHC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWCACHE_USDHCW<'a> {
    w: &'a mut W,
}
impl<'a> _AWCACHE_USDHCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWCACHE_USDHCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cacheable attribute is off for write transactions."]
    #[inline]
    pub fn awcache_usdhc_0(self) -> &'a mut W {
        self.variant(AWCACHE_USDHCW::AWCACHE_USDHC_0)
    }
    #[doc = "Cacheable attribute is on for write transactions."]
    #[inline]
    pub fn awcache_usdhc_1(self) -> &'a mut W {
        self.variant(AWCACHE_USDHCW::AWCACHE_USDHC_1)
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
#[doc = "Values that can be written to the field `CACHE_ENET`"]
pub enum CACHE_ENETW {
    #[doc = "Cacheable attribute is off for read/write transactions."]
    CACHE_ENET_0,
    #[doc = "Cacheable attribute is on for read/write transactions."]
    CACHE_ENET_1,
}
impl CACHE_ENETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHE_ENETW::CACHE_ENET_0 => false,
            CACHE_ENETW::CACHE_ENET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHE_ENETW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHE_ENETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHE_ENETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cacheable attribute is off for read/write transactions."]
    #[inline]
    pub fn cache_enet_0(self) -> &'a mut W {
        self.variant(CACHE_ENETW::CACHE_ENET_0)
    }
    #[doc = "Cacheable attribute is on for read/write transactions."]
    #[inline]
    pub fn cache_enet_1(self) -> &'a mut W {
        self.variant(CACHE_ENETW::CACHE_ENET_1)
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
#[doc = "Values that can be written to the field `CACHE_USB`"]
pub enum CACHE_USBW {
    #[doc = "Cacheable attribute is off for read/write transactions."]
    CACHE_USB_0,
    #[doc = "Cacheable attribute is on for read/write transactions."]
    CACHE_USB_1,
}
impl CACHE_USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHE_USBW::CACHE_USB_0 => false,
            CACHE_USBW::CACHE_USB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHE_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHE_USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHE_USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cacheable attribute is off for read/write transactions."]
    #[inline]
    pub fn cache_usb_0(self) -> &'a mut W {
        self.variant(CACHE_USBW::CACHE_USB_0)
    }
    #[doc = "Cacheable attribute is on for read/write transactions."]
    #[inline]
    pub fn cache_usb_1(self) -> &'a mut W {
        self.variant(CACHE_USBW::CACHE_USB_1)
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
    #[doc = "Bit 0 - uSDHC block cacheable attribute value of AXI read transactions"]
    #[inline]
    pub fn arcache_usdhc(&self) -> ARCACHE_USDHCR {
        ARCACHE_USDHCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - uSDHC block cacheable attribute value of AXI write transactions"]
    #[inline]
    pub fn awcache_usdhc(&self) -> AWCACHE_USDHCR {
        AWCACHE_USDHCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - ENET block cacheable attribute value of AXI transactions"]
    #[inline]
    pub fn cache_enet(&self) -> CACHE_ENETR {
        CACHE_ENETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - USB block cacheable attribute value of AXI transactions"]
    #[inline]
    pub fn cache_usb(&self) -> CACHE_USBR {
        CACHE_USBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - uSDHC block cacheable attribute value of AXI read transactions"]
    #[inline]
    pub fn arcache_usdhc(&mut self) -> _ARCACHE_USDHCW {
        _ARCACHE_USDHCW { w: self }
    }
    #[doc = "Bit 1 - uSDHC block cacheable attribute value of AXI write transactions"]
    #[inline]
    pub fn awcache_usdhc(&mut self) -> _AWCACHE_USDHCW {
        _AWCACHE_USDHCW { w: self }
    }
    #[doc = "Bit 7 - ENET block cacheable attribute value of AXI transactions"]
    #[inline]
    pub fn cache_enet(&mut self) -> _CACHE_ENETW {
        _CACHE_ENETW { w: self }
    }
    #[doc = "Bit 13 - USB block cacheable attribute value of AXI transactions"]
    #[inline]
    pub fn cache_usb(&mut self) -> _CACHE_USBW {
        _CACHE_USBW { w: self }
    }
}
