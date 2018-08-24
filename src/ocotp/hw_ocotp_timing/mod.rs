#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HW_OCOTP_TIMING {
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
pub struct STROBE_PROGR {
    bits: u16,
}
impl STROBE_PROGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RELAXR {
    bits: u8,
}
impl RELAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STROBE_READR {
    bits: u8,
}
impl STROBE_READR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAITR {
    bits: u8,
}
impl WAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _STROBE_PROGW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_PROGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RELAXW<'a> {
    w: &'a mut W,
}
impl<'a> _RELAXW<'a> {
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
pub struct _STROBE_READW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_READW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:11 - This count value specifies the strobe period in one time write OTP"]
    #[inline]
    pub fn strobe_prog(&self) -> STROBE_PROGR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        STROBE_PROGR { bits }
    }
    #[doc = "Bits 12:15 - This count value specifies the time to add to all default timing parameters other than the Tpgm and Trd"]
    #[inline]
    pub fn relax(&self) -> RELAXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RELAXR { bits }
    }
    #[doc = "Bits 16:21 - This count value specifies the strobe period in one time read OTP"]
    #[inline]
    pub fn strobe_read(&self) -> STROBE_READR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STROBE_READR { bits }
    }
    #[doc = "Bits 22:27 - This count value specifies time interval between auto read and write access in one time program"]
    #[inline]
    pub fn wait(&self) -> WAITR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAITR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 101554005 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - This count value specifies the strobe period in one time write OTP"]
    #[inline]
    pub fn strobe_prog(&mut self) -> _STROBE_PROGW {
        _STROBE_PROGW { w: self }
    }
    #[doc = "Bits 12:15 - This count value specifies the time to add to all default timing parameters other than the Tpgm and Trd"]
    #[inline]
    pub fn relax(&mut self) -> _RELAXW {
        _RELAXW { w: self }
    }
    #[doc = "Bits 16:21 - This count value specifies the strobe period in one time read OTP"]
    #[inline]
    pub fn strobe_read(&mut self) -> _STROBE_READW {
        _STROBE_READW { w: self }
    }
    #[doc = "Bits 22:27 - This count value specifies time interval between auto read and write access in one time program"]
    #[inline]
    pub fn wait(&mut self) -> _WAITW {
        _WAITW { w: self }
    }
}
