#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WTMK_LVL {
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
pub struct RD_WMLR {
    bits: u8,
}
impl RD_WMLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RD_BRST_LENR {
    bits: u8,
}
impl RD_BRST_LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WR_WMLR {
    bits: u8,
}
impl WR_WMLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WR_BRST_LENR {
    bits: u8,
}
impl WR_BRST_LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RD_WMLW<'a> {
    w: &'a mut W,
}
impl<'a> _RD_WMLW<'a> {
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
pub struct _RD_BRST_LENW<'a> {
    w: &'a mut W,
}
impl<'a> _RD_BRST_LENW<'a> {
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
pub struct _WR_WMLW<'a> {
    w: &'a mut W,
}
impl<'a> _WR_WMLW<'a> {
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
pub struct _WR_BRST_LENW<'a> {
    w: &'a mut W,
}
impl<'a> _WR_BRST_LENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline]
    pub fn rd_wml(&self) -> RD_WMLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RD_WMLR { bits }
    }
    #[doc = "Bits 8:12 - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline]
    pub fn rd_brst_len(&self) -> RD_BRST_LENR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RD_BRST_LENR { bits }
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline]
    pub fn wr_wml(&self) -> WR_WMLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WR_WMLR { bits }
    }
    #[doc = "Bits 24:28 - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline]
    pub fn wr_brst_len(&self) -> WR_BRST_LENR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WR_BRST_LENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 135268368 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline]
    pub fn rd_wml(&mut self) -> _RD_WMLW {
        _RD_WMLW { w: self }
    }
    #[doc = "Bits 8:12 - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline]
    pub fn rd_brst_len(&mut self) -> _RD_BRST_LENW {
        _RD_BRST_LENW { w: self }
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline]
    pub fn wr_wml(&mut self) -> _WR_WMLW {
        _WR_WMLW { w: self }
    }
    #[doc = "Bits 24:28 - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline]
    pub fn wr_brst_len(&mut self) -> _WR_BRST_LENW {
        _WR_BRST_LENW { w: self }
    }
}
