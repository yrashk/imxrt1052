#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMCR1 {
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
pub struct PRE2ACTR {
    bits: u8,
}
impl PRE2ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ACT2RWR {
    bits: u8,
}
impl ACT2RWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFRCR {
    bits: u8,
}
impl RFRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WRCR {
    bits: u8,
}
impl WRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CKEOFFR {
    bits: u8,
}
impl CKEOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ACT2PRER {
    bits: u8,
}
impl ACT2PRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRE2ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE2ACTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACT2RWW<'a> {
    w: &'a mut W,
}
impl<'a> _ACT2RWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RFRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRCW<'a> {
    w: &'a mut W,
}
impl<'a> _WRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CKEOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CKEOFFW<'a> {
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
pub struct _ACT2PREW<'a> {
    w: &'a mut W,
}
impl<'a> _ACT2PREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:3 - PRECHARGE to ACT/Refresh wait time"]
    #[inline]
    pub fn pre2act(&self) -> PRE2ACTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRE2ACTR { bits }
    }
    #[doc = "Bits 4:7 - ACT to Read/Write wait time"]
    #[inline]
    pub fn act2rw(&self) -> ACT2RWR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACT2RWR { bits }
    }
    #[doc = "Bits 8:12 - Refresh recovery time"]
    #[inline]
    pub fn rfrc(&self) -> RFRCR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFRCR { bits }
    }
    #[doc = "Bits 13:15 - Write recovery time"]
    #[inline]
    pub fn wrc(&self) -> WRCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WRCR { bits }
    }
    #[doc = "Bits 16:19 - CKE OFF minimum time"]
    #[inline]
    pub fn ckeoff(&self) -> CKEOFFR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CKEOFFR { bits }
    }
    #[doc = "Bits 20:23 - ACT to Precharge minimum time"]
    #[inline]
    pub fn act2pre(&self) -> ACT2PRER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACT2PRER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 10045748 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - PRECHARGE to ACT/Refresh wait time"]
    #[inline]
    pub fn pre2act(&mut self) -> _PRE2ACTW {
        _PRE2ACTW { w: self }
    }
    #[doc = "Bits 4:7 - ACT to Read/Write wait time"]
    #[inline]
    pub fn act2rw(&mut self) -> _ACT2RWW {
        _ACT2RWW { w: self }
    }
    #[doc = "Bits 8:12 - Refresh recovery time"]
    #[inline]
    pub fn rfrc(&mut self) -> _RFRCW {
        _RFRCW { w: self }
    }
    #[doc = "Bits 13:15 - Write recovery time"]
    #[inline]
    pub fn wrc(&mut self) -> _WRCW {
        _WRCW { w: self }
    }
    #[doc = "Bits 16:19 - CKE OFF minimum time"]
    #[inline]
    pub fn ckeoff(&mut self) -> _CKEOFFW {
        _CKEOFFW { w: self }
    }
    #[doc = "Bits 20:23 - ACT to Precharge minimum time"]
    #[inline]
    pub fn act2pre(&mut self) -> _ACT2PREW {
        _ACT2PREW { w: self }
    }
}
