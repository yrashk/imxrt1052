#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IPCR1 {
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
pub struct IDATSZR {
    bits: u16,
}
impl IDATSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISEQIDR {
    bits: u8,
}
impl ISEQIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISEQNUMR {
    bits: u8,
}
impl ISEQNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IPAREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPARENR {
    #[doc = "Flash will be accessed in Individual mode."]
    IPAREN_0,
    #[doc = "Flash will be accessed in Parallel mode."]
    IPAREN_1,
}
impl IPARENR {
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
            IPARENR::IPAREN_0 => false,
            IPARENR::IPAREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPARENR {
        match value {
            false => IPARENR::IPAREN_0,
            true => IPARENR::IPAREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPAREN_0`"]
    #[inline]
    pub fn is_iparen_0(&self) -> bool {
        *self == IPARENR::IPAREN_0
    }
    #[doc = "Checks if the value of the field is `IPAREN_1`"]
    #[inline]
    pub fn is_iparen_1(&self) -> bool {
        *self == IPARENR::IPAREN_1
    }
}
#[doc = r" Proxy"]
pub struct _IDATSZW<'a> {
    w: &'a mut W,
}
impl<'a> _IDATSZW<'a> {
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
pub struct _ISEQIDW<'a> {
    w: &'a mut W,
}
impl<'a> _ISEQIDW<'a> {
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
pub struct _ISEQNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _ISEQNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IPAREN`"]
pub enum IPARENW {
    #[doc = "Flash will be accessed in Individual mode."]
    IPAREN_0,
    #[doc = "Flash will be accessed in Parallel mode."]
    IPAREN_1,
}
impl IPARENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPARENW::IPAREN_0 => false,
            IPARENW::IPAREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPARENW<'a> {
    w: &'a mut W,
}
impl<'a> _IPARENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPARENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline]
    pub fn iparen_0(self) -> &'a mut W {
        self.variant(IPARENW::IPAREN_0)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline]
    pub fn iparen_1(self) -> &'a mut W {
        self.variant(IPARENW::IPAREN_1)
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
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline]
    pub fn idatsz(&self) -> IDATSZR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IDATSZR { bits }
    }
    #[doc = "Bits 16:19 - Sequence Index in LUT for IP command."]
    #[inline]
    pub fn iseqid(&self) -> ISEQIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ISEQIDR { bits }
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline]
    pub fn iseqnum(&self) -> ISEQNUMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ISEQNUMR { bits }
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline]
    pub fn iparen(&self) -> IPARENR {
        IPARENR::_from({
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
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline]
    pub fn idatsz(&mut self) -> _IDATSZW {
        _IDATSZW { w: self }
    }
    #[doc = "Bits 16:19 - Sequence Index in LUT for IP command."]
    #[inline]
    pub fn iseqid(&mut self) -> _ISEQIDW {
        _ISEQIDW { w: self }
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline]
    pub fn iseqnum(&mut self) -> _ISEQNUMW {
        _ISEQNUMW { w: self }
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline]
    pub fn iparen(&mut self) -> _IPARENW {
        _IPARENW { w: self }
    }
}
