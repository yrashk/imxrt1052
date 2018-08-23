#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TUNING_CTRL {
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
pub struct TUNING_START_TAPR {
    bits: u8,
}
impl TUNING_START_TAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TUNING_COUNTERR {
    bits: u8,
}
impl TUNING_COUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TUNING_STEPR {
    bits: u8,
}
impl TUNING_STEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TUNING_WINDOWR {
    bits: u8,
}
impl TUNING_WINDOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STD_TUNING_ENR {
    bits: bool,
}
impl STD_TUNING_ENR {
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
pub struct _TUNING_START_TAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_START_TAPW<'a> {
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
pub struct _TUNING_COUNTERW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_COUNTERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TUNING_STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_STEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TUNING_WINDOWW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_WINDOWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STD_TUNING_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _STD_TUNING_ENW<'a> {
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
    #[doc = "Bits 0:7 - TUNING_START_TAP"]
    #[inline]
    pub fn tuning_start_tap(&self) -> TUNING_START_TAPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNING_START_TAPR { bits }
    }
    #[doc = "Bits 8:15 - TUNING_COUNTER"]
    #[inline]
    pub fn tuning_counter(&self) -> TUNING_COUNTERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNING_COUNTERR { bits }
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline]
    pub fn tuning_step(&self) -> TUNING_STEPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNING_STEPR { bits }
    }
    #[doc = "Bits 20:22 - TUNING_WINDOW"]
    #[inline]
    pub fn tuning_window(&self) -> TUNING_WINDOWR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNING_WINDOWR { bits }
    }
    #[doc = "Bit 24 - STD_TUNING_EN"]
    #[inline]
    pub fn std_tuning_en(&self) -> STD_TUNING_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STD_TUNING_ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2172928 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - TUNING_START_TAP"]
    #[inline]
    pub fn tuning_start_tap(&mut self) -> _TUNING_START_TAPW {
        _TUNING_START_TAPW { w: self }
    }
    #[doc = "Bits 8:15 - TUNING_COUNTER"]
    #[inline]
    pub fn tuning_counter(&mut self) -> _TUNING_COUNTERW {
        _TUNING_COUNTERW { w: self }
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline]
    pub fn tuning_step(&mut self) -> _TUNING_STEPW {
        _TUNING_STEPW { w: self }
    }
    #[doc = "Bits 20:22 - TUNING_WINDOW"]
    #[inline]
    pub fn tuning_window(&mut self) -> _TUNING_WINDOWW {
        _TUNING_WINDOWW { w: self }
    }
    #[doc = "Bit 24 - STD_TUNING_EN"]
    #[inline]
    pub fn std_tuning_en(&mut self) -> _STD_TUNING_ENW {
        _STD_TUNING_ENW { w: self }
    }
}
