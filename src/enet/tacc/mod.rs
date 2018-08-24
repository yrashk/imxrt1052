#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TACC {
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
#[doc = "Possible values of the field `SHIFT16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIFT16R {
    #[doc = "Disabled."]
    SHIFT16_0,
    #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    SHIFT16_1,
}
impl SHIFT16R {
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
            SHIFT16R::SHIFT16_0 => false,
            SHIFT16R::SHIFT16_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHIFT16R {
        match value {
            false => SHIFT16R::SHIFT16_0,
            true => SHIFT16R::SHIFT16_1,
        }
    }
    #[doc = "Checks if the value of the field is `SHIFT16_0`"]
    #[inline]
    pub fn is_shift16_0(&self) -> bool {
        *self == SHIFT16R::SHIFT16_0
    }
    #[doc = "Checks if the value of the field is `SHIFT16_1`"]
    #[inline]
    pub fn is_shift16_1(&self) -> bool {
        *self == SHIFT16R::SHIFT16_1
    }
}
#[doc = "Possible values of the field `IPCHK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCHKR {
    #[doc = "Checksum is not inserted."]
    IPCHK_0,
    #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    IPCHK_1,
}
impl IPCHKR {
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
            IPCHKR::IPCHK_0 => false,
            IPCHKR::IPCHK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPCHKR {
        match value {
            false => IPCHKR::IPCHK_0,
            true => IPCHKR::IPCHK_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPCHK_0`"]
    #[inline]
    pub fn is_ipchk_0(&self) -> bool {
        *self == IPCHKR::IPCHK_0
    }
    #[doc = "Checks if the value of the field is `IPCHK_1`"]
    #[inline]
    pub fn is_ipchk_1(&self) -> bool {
        *self == IPCHKR::IPCHK_1
    }
}
#[doc = "Possible values of the field `PROCHK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROCHKR {
    #[doc = "Checksum not inserted."]
    PROCHK_0,
    #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    PROCHK_1,
}
impl PROCHKR {
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
            PROCHKR::PROCHK_0 => false,
            PROCHKR::PROCHK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROCHKR {
        match value {
            false => PROCHKR::PROCHK_0,
            true => PROCHKR::PROCHK_1,
        }
    }
    #[doc = "Checks if the value of the field is `PROCHK_0`"]
    #[inline]
    pub fn is_prochk_0(&self) -> bool {
        *self == PROCHKR::PROCHK_0
    }
    #[doc = "Checks if the value of the field is `PROCHK_1`"]
    #[inline]
    pub fn is_prochk_1(&self) -> bool {
        *self == PROCHKR::PROCHK_1
    }
}
#[doc = "Values that can be written to the field `SHIFT16`"]
pub enum SHIFT16W {
    #[doc = "Disabled."]
    SHIFT16_0,
    #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    SHIFT16_1,
}
impl SHIFT16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHIFT16W::SHIFT16_0 => false,
            SHIFT16W::SHIFT16_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHIFT16W<'a> {
    w: &'a mut W,
}
impl<'a> _SHIFT16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHIFT16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn shift16_0(self) -> &'a mut W {
        self.variant(SHIFT16W::SHIFT16_0)
    }
    #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    #[inline]
    pub fn shift16_1(self) -> &'a mut W {
        self.variant(SHIFT16W::SHIFT16_1)
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
#[doc = "Values that can be written to the field `IPCHK`"]
pub enum IPCHKW {
    #[doc = "Checksum is not inserted."]
    IPCHK_0,
    #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    IPCHK_1,
}
impl IPCHKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPCHKW::IPCHK_0 => false,
            IPCHKW::IPCHK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPCHKW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCHKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPCHKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Checksum is not inserted."]
    #[inline]
    pub fn ipchk_0(self) -> &'a mut W {
        self.variant(IPCHKW::IPCHK_0)
    }
    #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    #[inline]
    pub fn ipchk_1(self) -> &'a mut W {
        self.variant(IPCHKW::IPCHK_1)
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
#[doc = "Values that can be written to the field `PROCHK`"]
pub enum PROCHKW {
    #[doc = "Checksum not inserted."]
    PROCHK_0,
    #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    PROCHK_1,
}
impl PROCHKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROCHKW::PROCHK_0 => false,
            PROCHKW::PROCHK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROCHKW<'a> {
    w: &'a mut W,
}
impl<'a> _PROCHKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROCHKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Checksum not inserted."]
    #[inline]
    pub fn prochk_0(self) -> &'a mut W {
        self.variant(PROCHKW::PROCHK_0)
    }
    #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    #[inline]
    pub fn prochk_1(self) -> &'a mut W {
        self.variant(PROCHKW::PROCHK_1)
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
    #[doc = "Bit 0 - TX FIFO Shift-16"]
    #[inline]
    pub fn shift16(&self) -> SHIFT16R {
        SHIFT16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables insertion of IP header checksum."]
    #[inline]
    pub fn ipchk(&self) -> IPCHKR {
        IPCHKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables insertion of protocol checksum."]
    #[inline]
    pub fn prochk(&self) -> PROCHKR {
        PROCHKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - TX FIFO Shift-16"]
    #[inline]
    pub fn shift16(&mut self) -> _SHIFT16W {
        _SHIFT16W { w: self }
    }
    #[doc = "Bit 3 - Enables insertion of IP header checksum."]
    #[inline]
    pub fn ipchk(&mut self) -> _IPCHKW {
        _IPCHKW { w: self }
    }
    #[doc = "Bit 4 - Enables insertion of protocol checksum."]
    #[inline]
    pub fn prochk(&mut self) -> _PROCHKW {
        _PROCHKW { w: self }
    }
}
