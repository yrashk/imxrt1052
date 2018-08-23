#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NANDCR2 {
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
pub struct TWHRR {
    bits: u8,
}
impl TWHRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRHWR {
    bits: u8,
}
impl TRHWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TADLR {
    bits: u8,
}
impl TADLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRRR {
    bits: u8,
}
impl TRRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TWBR {
    bits: u8,
}
impl TWBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TWHRW<'a> {
    w: &'a mut W,
}
impl<'a> _TWHRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRHWW<'a> {
    w: &'a mut W,
}
impl<'a> _TRHWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TADLW<'a> {
    w: &'a mut W,
}
impl<'a> _TADLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRRW<'a> {
    w: &'a mut W,
}
impl<'a> _TRRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TWBW<'a> {
    w: &'a mut W,
}
impl<'a> _TWBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:5 - WE# HIGH to RE# LOW wait time"]
    #[inline]
    pub fn twhr(&self) -> TWHRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TWHRR { bits }
    }
    #[doc = "Bits 6:11 - RE# HIGH to WE# LOW wait time"]
    #[inline]
    pub fn trhw(&self) -> TRHWR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRHWR { bits }
    }
    #[doc = "Bits 12:17 - ALE to WRITE Data start wait time"]
    #[inline]
    pub fn tadl(&self) -> TADLR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TADLR { bits }
    }
    #[doc = "Bits 18:23 - Ready to RE# LOW min wait time"]
    #[inline]
    pub fn trr(&self) -> TRRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRRR { bits }
    }
    #[doc = "Bits 24:29 - WE# HIGH to busy wait time"]
    #[inline]
    pub fn twb(&self) -> TWBR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TWBR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 66576 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - WE# HIGH to RE# LOW wait time"]
    #[inline]
    pub fn twhr(&mut self) -> _TWHRW {
        _TWHRW { w: self }
    }
    #[doc = "Bits 6:11 - RE# HIGH to WE# LOW wait time"]
    #[inline]
    pub fn trhw(&mut self) -> _TRHWW {
        _TRHWW { w: self }
    }
    #[doc = "Bits 12:17 - ALE to WRITE Data start wait time"]
    #[inline]
    pub fn tadl(&mut self) -> _TADLW {
        _TADLW { w: self }
    }
    #[doc = "Bits 18:23 - Ready to RE# LOW min wait time"]
    #[inline]
    pub fn trr(&mut self) -> _TRRW {
        _TRRW { w: self }
    }
    #[doc = "Bits 24:29 - WE# HIGH to busy wait time"]
    #[inline]
    pub fn twb(&mut self) -> _TWBW {
        _TWBW { w: self }
    }
}
