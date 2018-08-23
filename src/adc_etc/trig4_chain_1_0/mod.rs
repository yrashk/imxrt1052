#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRIG4_CHAIN_1_0 {
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
pub struct CSEL0R {
    bits: u8,
}
impl CSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HWTS0R {
    bits: u8,
}
impl HWTS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B2B0R {
    bits: bool,
}
impl B2B0R {
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
pub struct IE0R {
    bits: u8,
}
impl IE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CSEL1R {
    bits: u8,
}
impl CSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HWTS1R {
    bits: u8,
}
impl HWTS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B2B1R {
    bits: bool,
}
impl B2B1R {
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
pub struct IE1R {
    bits: u8,
}
impl IE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CSEL0W<'a> {
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
pub struct _HWTS0W<'a> {
    w: &'a mut W,
}
impl<'a> _HWTS0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B2B0W<'a> {
    w: &'a mut W,
}
impl<'a> _B2B0W<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _IE0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CSEL1W<'a> {
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
pub struct _HWTS1W<'a> {
    w: &'a mut W,
}
impl<'a> _HWTS1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B2B1W<'a> {
    w: &'a mut W,
}
impl<'a> _B2B1W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _IE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:3 - CHAIN0 CSEL ADC channel selection"]
    #[inline]
    pub fn csel0(&self) -> CSEL0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CSEL0R { bits }
    }
    #[doc = "Bits 4:11 - CHAIN0 HWTS ADC hardware trigger selection"]
    #[inline]
    pub fn hwts0(&self) -> HWTS0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HWTS0R { bits }
    }
    #[doc = "Bit 12 - CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline]
    pub fn b2b0(&self) -> B2B0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        B2B0R { bits }
    }
    #[doc = "Bits 13:14 - CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline]
    pub fn ie0(&self) -> IE0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IE0R { bits }
    }
    #[doc = "Bits 16:19 - CHAIN1 CSEL ADC channel selection"]
    #[inline]
    pub fn csel1(&self) -> CSEL1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CSEL1R { bits }
    }
    #[doc = "Bits 20:27 - CHAIN1 HWTS ADC hardware trigger selection"]
    #[inline]
    pub fn hwts1(&self) -> HWTS1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HWTS1R { bits }
    }
    #[doc = "Bit 28 - CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline]
    pub fn b2b1(&self) -> B2B1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        B2B1R { bits }
    }
    #[doc = "Bits 29:30 - CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline]
    pub fn ie1(&self) -> IE1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IE1R { bits }
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
    #[doc = "Bits 0:3 - CHAIN0 CSEL ADC channel selection"]
    #[inline]
    pub fn csel0(&mut self) -> _CSEL0W {
        _CSEL0W { w: self }
    }
    #[doc = "Bits 4:11 - CHAIN0 HWTS ADC hardware trigger selection"]
    #[inline]
    pub fn hwts0(&mut self) -> _HWTS0W {
        _HWTS0W { w: self }
    }
    #[doc = "Bit 12 - CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline]
    pub fn b2b0(&mut self) -> _B2B0W {
        _B2B0W { w: self }
    }
    #[doc = "Bits 13:14 - CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline]
    pub fn ie0(&mut self) -> _IE0W {
        _IE0W { w: self }
    }
    #[doc = "Bits 16:19 - CHAIN1 CSEL ADC channel selection"]
    #[inline]
    pub fn csel1(&mut self) -> _CSEL1W {
        _CSEL1W { w: self }
    }
    #[doc = "Bits 20:27 - CHAIN1 HWTS ADC hardware trigger selection"]
    #[inline]
    pub fn hwts1(&mut self) -> _HWTS1W {
        _HWTS1W { w: self }
    }
    #[doc = "Bit 28 - CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger"]
    #[inline]
    pub fn b2b1(&mut self) -> _B2B1W {
        _B2B1W { w: self }
    }
    #[doc = "Bits 29:30 - CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2"]
    #[inline]
    pub fn ie1(&mut self) -> _IE1W {
        _IE1W { w: self }
    }
}
