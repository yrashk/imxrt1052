#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSC1_COEF0 {
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
pub struct Y_OFFSETR {
    bits: u16,
}
impl Y_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UV_OFFSETR {
    bits: u16,
}
impl UV_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C0R {
    bits: u16,
}
impl C0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD1R {
    bits: bool,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct BYPASSR {
    bits: bool,
}
impl BYPASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct YCBCR_MODER {
    bits: bool,
}
impl YCBCR_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _Y_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _Y_OFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UV_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _UV_OFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C0W<'a> {
    w: &'a mut W,
}
impl<'a> _C0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _YCBCR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _YCBCR_MODEW<'a> {
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
    #[doc = "Bits 0:8 - Two's compliment amplitude offset implicit in the Y data"]
    #[inline]
    pub fn y_offset(&self) -> Y_OFFSETR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        Y_OFFSETR { bits }
    }
    #[doc = "Bits 9:17 - Two's compliment phase offset implicit for CbCr data"]
    #[inline]
    pub fn uv_offset(&self) -> UV_OFFSETR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        UV_OFFSETR { bits }
    }
    #[doc = "Bits 18:28 - Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[inline]
    pub fn c0(&self) -> C0R {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        C0R { bits }
    }
    #[doc = "Bit 29 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD1R { bits }
    }
    #[doc = "Bit 30 - Bypass the CSC unit in the scaling engine"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASSR { bits }
    }
    #[doc = "Bit 31 - Set to 1 when performing YCbCr conversion to RGB"]
    #[inline]
    pub fn ycbcr_mode(&self) -> YCBCR_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        YCBCR_MODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67108864 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:8 - Two's compliment amplitude offset implicit in the Y data"]
    #[inline]
    pub fn y_offset(&mut self) -> _Y_OFFSETW {
        _Y_OFFSETW { w: self }
    }
    #[doc = "Bits 9:17 - Two's compliment phase offset implicit for CbCr data"]
    #[inline]
    pub fn uv_offset(&mut self) -> _UV_OFFSETW {
        _UV_OFFSETW { w: self }
    }
    #[doc = "Bits 18:28 - Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[inline]
    pub fn c0(&mut self) -> _C0W {
        _C0W { w: self }
    }
    #[doc = "Bit 30 - Bypass the CSC unit in the scaling engine"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 31 - Set to 1 when performing YCbCr conversion to RGB"]
    #[inline]
    pub fn ycbcr_mode(&mut self) -> _YCBCR_MODEW {
        _YCBCR_MODEW { w: self }
    }
}
