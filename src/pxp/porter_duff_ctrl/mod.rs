#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTER_DUFF_CTRL {
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
pub struct POTER_DUFF_ENABLER {
    bits: bool,
}
impl POTER_DUFF_ENABLER {
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
pub struct S0_S1_FACTOR_MODER {
    bits: u8,
}
impl S0_S1_FACTOR_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S0_GLOBAL_ALPHA_MODER {
    bits: u8,
}
impl S0_GLOBAL_ALPHA_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S0_ALPHA_MODER {
    bits: bool,
}
impl S0_ALPHA_MODER {
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
pub struct S0_COLOR_MODER {
    bits: bool,
}
impl S0_COLOR_MODER {
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
pub struct S1_S0_FACTOR_MODER {
    bits: u8,
}
impl S1_S0_FACTOR_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S1_GLOBAL_ALPHA_MODER {
    bits: u8,
}
impl S1_GLOBAL_ALPHA_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S1_ALPHA_MODER {
    bits: bool,
}
impl S1_ALPHA_MODER {
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
pub struct S1_COLOR_MODER {
    bits: bool,
}
impl S1_COLOR_MODER {
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
pub struct S0_GLOBAL_ALPHAR {
    bits: u8,
}
impl S0_GLOBAL_ALPHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S1_GLOBAL_ALPHAR {
    bits: u8,
}
impl S1_GLOBAL_ALPHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _POTER_DUFF_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _POTER_DUFF_ENABLEW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0_S1_FACTOR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S0_S1_FACTOR_MODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0_GLOBAL_ALPHA_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S0_GLOBAL_ALPHA_MODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0_ALPHA_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S0_ALPHA_MODEW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0_COLOR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S0_COLOR_MODEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S1_S0_FACTOR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S1_S0_FACTOR_MODEW<'a> {
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
pub struct _S1_GLOBAL_ALPHA_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S1_GLOBAL_ALPHA_MODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S1_ALPHA_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S1_ALPHA_MODEW<'a> {
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
pub struct _S1_COLOR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S1_COLOR_MODEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0_GLOBAL_ALPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _S0_GLOBAL_ALPHAW<'a> {
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
pub struct _S1_GLOBAL_ALPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _S1_GLOBAL_ALPHAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - poter_duff enable"]
    #[inline]
    pub fn poter_duff_enable(&self) -> POTER_DUFF_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POTER_DUFF_ENABLER { bits }
    }
    #[doc = "Bits 1:2 - s0 to s1 factor mode"]
    #[inline]
    pub fn s0_s1_factor_mode(&self) -> S0_S1_FACTOR_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S0_S1_FACTOR_MODER { bits }
    }
    #[doc = "Bits 3:4 - s0 global alpha mode"]
    #[inline]
    pub fn s0_global_alpha_mode(&self) -> S0_GLOBAL_ALPHA_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S0_GLOBAL_ALPHA_MODER { bits }
    }
    #[doc = "Bit 5 - s0 alpha mode"]
    #[inline]
    pub fn s0_alpha_mode(&self) -> S0_ALPHA_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S0_ALPHA_MODER { bits }
    }
    #[doc = "Bit 6 - s0 color mode"]
    #[inline]
    pub fn s0_color_mode(&self) -> S0_COLOR_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S0_COLOR_MODER { bits }
    }
    #[doc = "Bits 8:9 - s1 to s0 factor mode"]
    #[inline]
    pub fn s1_s0_factor_mode(&self) -> S1_S0_FACTOR_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S1_S0_FACTOR_MODER { bits }
    }
    #[doc = "Bits 10:11 - s1 global alpha mode"]
    #[inline]
    pub fn s1_global_alpha_mode(&self) -> S1_GLOBAL_ALPHA_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S1_GLOBAL_ALPHA_MODER { bits }
    }
    #[doc = "Bit 12 - s1 alpha mode"]
    #[inline]
    pub fn s1_alpha_mode(&self) -> S1_ALPHA_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S1_ALPHA_MODER { bits }
    }
    #[doc = "Bit 13 - s1 color mode"]
    #[inline]
    pub fn s1_color_mode(&self) -> S1_COLOR_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S1_COLOR_MODER { bits }
    }
    #[doc = "Bits 16:23 - s0 global alpha"]
    #[inline]
    pub fn s0_global_alpha(&self) -> S0_GLOBAL_ALPHAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S0_GLOBAL_ALPHAR { bits }
    }
    #[doc = "Bits 24:31 - s1 global alpha"]
    #[inline]
    pub fn s1_global_alpha(&self) -> S1_GLOBAL_ALPHAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S1_GLOBAL_ALPHAR { bits }
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
    #[doc = "Bit 0 - poter_duff enable"]
    #[inline]
    pub fn poter_duff_enable(&mut self) -> _POTER_DUFF_ENABLEW {
        _POTER_DUFF_ENABLEW { w: self }
    }
    #[doc = "Bits 1:2 - s0 to s1 factor mode"]
    #[inline]
    pub fn s0_s1_factor_mode(&mut self) -> _S0_S1_FACTOR_MODEW {
        _S0_S1_FACTOR_MODEW { w: self }
    }
    #[doc = "Bits 3:4 - s0 global alpha mode"]
    #[inline]
    pub fn s0_global_alpha_mode(&mut self) -> _S0_GLOBAL_ALPHA_MODEW {
        _S0_GLOBAL_ALPHA_MODEW { w: self }
    }
    #[doc = "Bit 5 - s0 alpha mode"]
    #[inline]
    pub fn s0_alpha_mode(&mut self) -> _S0_ALPHA_MODEW {
        _S0_ALPHA_MODEW { w: self }
    }
    #[doc = "Bit 6 - s0 color mode"]
    #[inline]
    pub fn s0_color_mode(&mut self) -> _S0_COLOR_MODEW {
        _S0_COLOR_MODEW { w: self }
    }
    #[doc = "Bits 8:9 - s1 to s0 factor mode"]
    #[inline]
    pub fn s1_s0_factor_mode(&mut self) -> _S1_S0_FACTOR_MODEW {
        _S1_S0_FACTOR_MODEW { w: self }
    }
    #[doc = "Bits 10:11 - s1 global alpha mode"]
    #[inline]
    pub fn s1_global_alpha_mode(&mut self) -> _S1_GLOBAL_ALPHA_MODEW {
        _S1_GLOBAL_ALPHA_MODEW { w: self }
    }
    #[doc = "Bit 12 - s1 alpha mode"]
    #[inline]
    pub fn s1_alpha_mode(&mut self) -> _S1_ALPHA_MODEW {
        _S1_ALPHA_MODEW { w: self }
    }
    #[doc = "Bit 13 - s1 color mode"]
    #[inline]
    pub fn s1_color_mode(&mut self) -> _S1_COLOR_MODEW {
        _S1_COLOR_MODEW { w: self }
    }
    #[doc = "Bits 16:23 - s0 global alpha"]
    #[inline]
    pub fn s0_global_alpha(&mut self) -> _S0_GLOBAL_ALPHAW {
        _S0_GLOBAL_ALPHAW { w: self }
    }
    #[doc = "Bits 24:31 - s1 global alpha"]
    #[inline]
    pub fn s1_global_alpha(&mut self) -> _S1_GLOBAL_ALPHAW {
        _S1_GLOBAL_ALPHAW { w: self }
    }
}
