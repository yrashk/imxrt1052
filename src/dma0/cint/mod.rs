#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CINT {
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
}
#[doc = r" Proxy"]
pub struct _CINTW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAIR`"]
pub enum CAIRW {
    #[doc = "no description available"]
    CAIR_0,
    #[doc = "no description available"]
    CAIR_1,
}
impl CAIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAIRW::CAIR_0 => false,
            CAIRW::CAIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn cair_0(self) -> &'a mut W {
        self.variant(CAIRW::CAIR_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn cair_1(self) -> &'a mut W {
        self.variant(CAIRW::CAIR_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NOP`"]
pub enum NOPW {
    #[doc = "Normal operation"]
    NOP_0,
    #[doc = "No operation, ignore the other bits in this register"]
    NOP_1,
}
impl NOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOPW::NOP_0 => false,
            NOPW::NOP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOPW<'a> {
    w: &'a mut W,
}
impl<'a> _NOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn nop_0(self) -> &'a mut W {
        self.variant(NOPW::NOP_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline]
    pub fn nop_1(self) -> &'a mut W {
        self.variant(NOPW::NOP_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Clear Interrupt Request"]
    #[inline]
    pub fn cint(&mut self) -> _CINTW {
        _CINTW { w: self }
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline]
    pub fn cair(&mut self) -> _CAIRW {
        _CAIRW { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline]
    pub fn nop(&mut self) -> _NOPW {
        _NOPW { w: self }
    }
}
