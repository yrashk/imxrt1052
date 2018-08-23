#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBMODE {
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
#[doc = "Possible values of the field `CM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMR {
    #[doc = "Idle [Default for combination host/device]"]
    CM_0,
    #[doc = "Device Controller [Default for device only controller]"]
    CM_2,
    #[doc = "Host Controller [Default for host only controller]"]
    CM_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMR::CM_0 => 0,
            CMR::CM_2 => 2,
            CMR::CM_3 => 3,
            CMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMR {
        match value {
            0 => CMR::CM_0,
            2 => CMR::CM_2,
            3 => CMR::CM_3,
            i => CMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline]
    pub fn is_cm_0(&self) -> bool {
        *self == CMR::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline]
    pub fn is_cm_2(&self) -> bool {
        *self == CMR::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline]
    pub fn is_cm_3(&self) -> bool {
        *self == CMR::CM_3
    }
}
#[doc = "Possible values of the field `ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESR {
    #[doc = "Little Endian [Default]"]
    ES_0,
    #[doc = "Big Endian"]
    ES_1,
}
impl ESR {
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
            ESR::ES_0 => false,
            ESR::ES_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESR {
        match value {
            false => ESR::ES_0,
            true => ESR::ES_1,
        }
    }
    #[doc = "Checks if the value of the field is `ES_0`"]
    #[inline]
    pub fn is_es_0(&self) -> bool {
        *self == ESR::ES_0
    }
    #[doc = "Checks if the value of the field is `ES_1`"]
    #[inline]
    pub fn is_es_1(&self) -> bool {
        *self == ESR::ES_1
    }
}
#[doc = "Possible values of the field `SLOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOMR {
    #[doc = "Setup Lockouts On (default);"]
    SLOM_0,
    #[doc = "Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    SLOM_1,
}
impl SLOMR {
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
            SLOMR::SLOM_0 => false,
            SLOMR::SLOM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOMR {
        match value {
            false => SLOMR::SLOM_0,
            true => SLOMR::SLOM_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLOM_0`"]
    #[inline]
    pub fn is_slom_0(&self) -> bool {
        *self == SLOMR::SLOM_0
    }
    #[doc = "Checks if the value of the field is `SLOM_1`"]
    #[inline]
    pub fn is_slom_1(&self) -> bool {
        *self == SLOMR::SLOM_1
    }
}
#[doc = r" Value of the field"]
pub struct SDISR {
    bits: bool,
}
impl SDISR {
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
#[doc = "Values that can be written to the field `CM`"]
pub enum CMW {
    #[doc = "Idle [Default for combination host/device]"]
    CM_0,
    #[doc = "Device Controller [Default for device only controller]"]
    CM_2,
    #[doc = "Host Controller [Default for host only controller]"]
    CM_3,
}
impl CMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMW::CM_0 => 0,
            CMW::CM_2 => 2,
            CMW::CM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle [Default for combination host/device]"]
    #[inline]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CMW::CM_0)
    }
    #[doc = "Device Controller [Default for device only controller]"]
    #[inline]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CMW::CM_2)
    }
    #[doc = "Host Controller [Default for host only controller]"]
    #[inline]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CMW::CM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ES`"]
pub enum ESW {
    #[doc = "Little Endian [Default]"]
    ES_0,
    #[doc = "Big Endian"]
    ES_1,
}
impl ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESW::ES_0 => false,
            ESW::ES_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESW<'a> {
    w: &'a mut W,
}
impl<'a> _ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Little Endian [Default]"]
    #[inline]
    pub fn es_0(self) -> &'a mut W {
        self.variant(ESW::ES_0)
    }
    #[doc = "Big Endian"]
    #[inline]
    pub fn es_1(self) -> &'a mut W {
        self.variant(ESW::ES_1)
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
#[doc = "Values that can be written to the field `SLOM`"]
pub enum SLOMW {
    #[doc = "Setup Lockouts On (default);"]
    SLOM_0,
    #[doc = "Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    SLOM_1,
}
impl SLOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOMW::SLOM_0 => false,
            SLOMW::SLOM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Setup Lockouts On (default);"]
    #[inline]
    pub fn slom_0(self) -> &'a mut W {
        self.variant(SLOMW::SLOM_0)
    }
    #[doc = "Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    #[inline]
    pub fn slom_1(self) -> &'a mut W {
        self.variant(SLOMW::SLOM_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SDISW<'a> {
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Controller Mode - R/WO"]
    #[inline]
    pub fn cm(&self) -> CMR {
        CMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Endian Select - Read/Write"]
    #[inline]
    pub fn es(&self) -> ESR {
        ESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Setup Lockout Mode"]
    #[inline]
    pub fn slom(&self) -> SLOMR {
        SLOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stream Disable Mode"]
    #[inline]
    pub fn sdis(&self) -> SDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDISR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 20480 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Controller Mode - R/WO"]
    #[inline]
    pub fn cm(&mut self) -> _CMW {
        _CMW { w: self }
    }
    #[doc = "Bit 2 - Endian Select - Read/Write"]
    #[inline]
    pub fn es(&mut self) -> _ESW {
        _ESW { w: self }
    }
    #[doc = "Bit 3 - Setup Lockout Mode"]
    #[inline]
    pub fn slom(&mut self) -> _SLOMW {
        _SLOMW { w: self }
    }
    #[doc = "Bit 4 - Stream Disable Mode"]
    #[inline]
    pub fn sdis(&mut self) -> _SDISW {
        _SDISW { w: self }
    }
}
