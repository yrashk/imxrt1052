#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NORCR1 {
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
pub struct CESR {
    bits: u8,
}
impl CESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CEHR {
    bits: u8,
}
impl CEHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ASR {
    bits: u8,
}
impl ASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AHR {
    bits: u8,
}
impl AHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WELR {
    bits: u8,
}
impl WELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WEHR {
    bits: u8,
}
impl WEHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RELR {
    bits: u8,
}
impl RELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REHR {
    bits: u8,
}
impl REHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CESW<'a> {
    w: &'a mut W,
}
impl<'a> _CESW<'a> {
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
pub struct _CEHW<'a> {
    w: &'a mut W,
}
impl<'a> _CEHW<'a> {
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
pub struct _ASW<'a> {
    w: &'a mut W,
}
impl<'a> _ASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AHW<'a> {
    w: &'a mut W,
}
impl<'a> _AHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WELW<'a> {
    w: &'a mut W,
}
impl<'a> _WELW<'a> {
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
pub struct _WEHW<'a> {
    w: &'a mut W,
}
impl<'a> _WEHW<'a> {
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
#[doc = r" Proxy"]
pub struct _RELW<'a> {
    w: &'a mut W,
}
impl<'a> _RELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REHW<'a> {
    w: &'a mut W,
}
impl<'a> _REHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - CE setup time cycle"]
    #[inline]
    pub fn ces(&self) -> CESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CESR { bits }
    }
    #[doc = "Bits 4:7 - CE hold min time (CEH+1) cycle"]
    #[inline]
    pub fn ceh(&self) -> CEHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CEHR { bits }
    }
    #[doc = "Bits 8:11 - Address setup time"]
    #[inline]
    pub fn as_(&self) -> ASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ASR { bits }
    }
    #[doc = "Bits 12:15 - Address hold time"]
    #[inline]
    pub fn ah(&self) -> AHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AHR { bits }
    }
    #[doc = "Bits 16:19 - WE LOW time (WEL+1) cycle"]
    #[inline]
    pub fn wel(&self) -> WELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WELR { bits }
    }
    #[doc = "Bits 20:23 - WE HIGH time (WEH+1) cycle"]
    #[inline]
    pub fn weh(&self) -> WEHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WEHR { bits }
    }
    #[doc = "Bits 24:27 - RE LOW time (REL+1) cycle"]
    #[inline]
    pub fn rel(&self) -> RELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RELR { bits }
    }
    #[doc = "Bits 28:31 - RE HIGH time (REH+1) cycle"]
    #[inline]
    pub fn reh(&self) -> REHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REHR { bits }
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
    #[doc = "Bits 0:3 - CE setup time cycle"]
    #[inline]
    pub fn ces(&mut self) -> _CESW {
        _CESW { w: self }
    }
    #[doc = "Bits 4:7 - CE hold min time (CEH+1) cycle"]
    #[inline]
    pub fn ceh(&mut self) -> _CEHW {
        _CEHW { w: self }
    }
    #[doc = "Bits 8:11 - Address setup time"]
    #[inline]
    pub fn as_(&mut self) -> _ASW {
        _ASW { w: self }
    }
    #[doc = "Bits 12:15 - Address hold time"]
    #[inline]
    pub fn ah(&mut self) -> _AHW {
        _AHW { w: self }
    }
    #[doc = "Bits 16:19 - WE LOW time (WEL+1) cycle"]
    #[inline]
    pub fn wel(&mut self) -> _WELW {
        _WELW { w: self }
    }
    #[doc = "Bits 20:23 - WE HIGH time (WEH+1) cycle"]
    #[inline]
    pub fn weh(&mut self) -> _WEHW {
        _WEHW { w: self }
    }
    #[doc = "Bits 24:27 - RE LOW time (REL+1) cycle"]
    #[inline]
    pub fn rel(&mut self) -> _RELW {
        _RELW { w: self }
    }
    #[doc = "Bits 28:31 - RE HIGH time (REH+1) cycle"]
    #[inline]
    pub fn reh(&mut self) -> _REHW {
        _REHW { w: self }
    }
}
