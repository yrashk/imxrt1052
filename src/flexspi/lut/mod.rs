#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LUT {
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
pub struct OPERAND0R {
    bits: u8,
}
impl OPERAND0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NUM_PADS0R {
    bits: u8,
}
impl NUM_PADS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OPCODE0R {
    bits: u8,
}
impl OPCODE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OPERAND1R {
    bits: u8,
}
impl OPERAND1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NUM_PADS1R {
    bits: u8,
}
impl NUM_PADS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OPCODE1R {
    bits: u8,
}
impl OPCODE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _OPERAND0W<'a> {
    w: &'a mut W,
}
impl<'a> _OPERAND0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NUM_PADS0W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM_PADS0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPCODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _OPCODE0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPERAND1W<'a> {
    w: &'a mut W,
}
impl<'a> _OPERAND1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NUM_PADS1W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM_PADS1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPCODE1W<'a> {
    w: &'a mut W,
}
impl<'a> _OPCODE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline]
    pub fn operand0(&self) -> OPERAND0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPERAND0R { bits }
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline]
    pub fn num_pads0(&self) -> NUM_PADS0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM_PADS0R { bits }
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline]
    pub fn opcode0(&self) -> OPCODE0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPCODE0R { bits }
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline]
    pub fn operand1(&self) -> OPERAND1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPERAND1R { bits }
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline]
    pub fn num_pads1(&self) -> NUM_PADS1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM_PADS1R { bits }
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline]
    pub fn opcode1(&self) -> OPCODE1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPCODE1R { bits }
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
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline]
    pub fn operand0(&mut self) -> _OPERAND0W {
        _OPERAND0W { w: self }
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline]
    pub fn num_pads0(&mut self) -> _NUM_PADS0W {
        _NUM_PADS0W { w: self }
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline]
    pub fn opcode0(&mut self) -> _OPCODE0W {
        _OPCODE0W { w: self }
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline]
    pub fn operand1(&mut self) -> _OPERAND1W {
        _OPERAND1W { w: self }
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline]
    pub fn num_pads1(&mut self) -> _NUM_PADS1W {
        _NUM_PADS1W { w: self }
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline]
    pub fn opcode1(&mut self) -> _OPCODE1W {
        _OPCODE1W { w: self }
    }
}
