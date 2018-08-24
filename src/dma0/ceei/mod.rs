#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CEEI {
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
pub struct _CEEIW<'a> {
    w: &'a mut W,
}
impl<'a> _CEEIW<'a> {
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
#[doc = "Values that can be written to the field `CAEE`"]
pub enum CAEEW {
    #[doc = "no description available"]
    CAEE_0,
    #[doc = "no description available"]
    CAEE_1,
}
impl CAEEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAEEW::CAEE_0 => false,
            CAEEW::CAEE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAEEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAEEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAEEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn caee_0(self) -> &'a mut W {
        self.variant(CAEEW::CAEE_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn caee_1(self) -> &'a mut W {
        self.variant(CAEEW::CAEE_1)
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
    #[doc = "Bits 0:4 - Clear Enable Error Interrupt"]
    #[inline]
    pub fn ceei(&mut self) -> _CEEIW {
        _CEEIW { w: self }
    }
    #[doc = "Bit 6 - Clear All Enable Error Interrupts"]
    #[inline]
    pub fn caee(&mut self) -> _CAEEW {
        _CAEEW { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline]
    pub fn nop(&mut self) -> _NOPW {
        _NOPW { w: self }
    }
}
