#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct TRIG_ENABLER {
    bits: u8,
}
impl TRIG_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXT0_TRIG_ENABLER {
    bits: bool,
}
impl EXT0_TRIG_ENABLER {
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
pub struct EXT0_TRIG_PRIORITYR {
    bits: u8,
}
impl EXT0_TRIG_PRIORITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXT1_TRIG_ENABLER {
    bits: bool,
}
impl EXT1_TRIG_ENABLER {
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
pub struct EXT1_TRIG_PRIORITYR {
    bits: u8,
}
impl EXT1_TRIG_PRIORITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRE_DIVIDERR {
    bits: u8,
}
impl PRE_DIVIDERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSC_BYPASSR {
    bits: bool,
}
impl TSC_BYPASSR {
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
pub struct SOFTRSTR {
    bits: bool,
}
impl SOFTRSTR {
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
pub struct _TRIG_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIG_ENABLEW<'a> {
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
pub struct _EXT0_TRIG_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT0_TRIG_ENABLEW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXT0_TRIG_PRIORITYW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT0_TRIG_PRIORITYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXT1_TRIG_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT1_TRIG_ENABLEW<'a> {
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
pub struct _EXT1_TRIG_PRIORITYW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT1_TRIG_PRIORITYW<'a> {
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
pub struct _PRE_DIVIDERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_DIVIDERW<'a> {
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
pub struct _TSC_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSC_BYPASSW<'a> {
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
pub struct _SOFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTRSTW<'a> {
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
    #[doc = "Bits 0:7 - TRIG enable register"]
    #[inline]
    pub fn trig_enable(&self) -> TRIG_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIG_ENABLER { bits }
    }
    #[doc = "Bit 8 - TSC0 TRIG enable register. 1'b1: enable external TSC0 trigger. 1'b0: disable external TSC0 trigger."]
    #[inline]
    pub fn ext0_trig_enable(&self) -> EXT0_TRIG_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXT0_TRIG_ENABLER { bits }
    }
    #[doc = "Bits 9:11 - External TSC0 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline]
    pub fn ext0_trig_priority(&self) -> EXT0_TRIG_PRIORITYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXT0_TRIG_PRIORITYR { bits }
    }
    #[doc = "Bit 12 - TSC1 TRIG enable register. 1'b1: enable external TSC1 trigger. 1'b0: disable external TSC1 trigger."]
    #[inline]
    pub fn ext1_trig_enable(&self) -> EXT1_TRIG_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXT1_TRIG_ENABLER { bits }
    }
    #[doc = "Bits 13:15 - External TSC1 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline]
    pub fn ext1_trig_priority(&self) -> EXT1_TRIG_PRIORITYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXT1_TRIG_PRIORITYR { bits }
    }
    #[doc = "Bits 16:23 - Pre-divider for trig delay and interval ."]
    #[inline]
    pub fn pre_divider(&self) -> PRE_DIVIDERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRE_DIVIDERR { bits }
    }
    #[doc = "Bit 30 - 1'b1: TSC is bypassed; 1'b0: TSC not bypassed;"]
    #[inline]
    pub fn tsc_bypass(&self) -> TSC_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSC_BYPASSR { bits }
    }
    #[doc = "Bit 31 - Software reset, high active. When write 1 ,all logical will be reset."]
    #[inline]
    pub fn softrst(&self) -> SOFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOFTRSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - TRIG enable register"]
    #[inline]
    pub fn trig_enable(&mut self) -> _TRIG_ENABLEW {
        _TRIG_ENABLEW { w: self }
    }
    #[doc = "Bit 8 - TSC0 TRIG enable register. 1'b1: enable external TSC0 trigger. 1'b0: disable external TSC0 trigger."]
    #[inline]
    pub fn ext0_trig_enable(&mut self) -> _EXT0_TRIG_ENABLEW {
        _EXT0_TRIG_ENABLEW { w: self }
    }
    #[doc = "Bits 9:11 - External TSC0 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline]
    pub fn ext0_trig_priority(&mut self) -> _EXT0_TRIG_PRIORITYW {
        _EXT0_TRIG_PRIORITYW { w: self }
    }
    #[doc = "Bit 12 - TSC1 TRIG enable register. 1'b1: enable external TSC1 trigger. 1'b0: disable external TSC1 trigger."]
    #[inline]
    pub fn ext1_trig_enable(&mut self) -> _EXT1_TRIG_ENABLEW {
        _EXT1_TRIG_ENABLEW { w: self }
    }
    #[doc = "Bits 13:15 - External TSC1 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline]
    pub fn ext1_trig_priority(&mut self) -> _EXT1_TRIG_PRIORITYW {
        _EXT1_TRIG_PRIORITYW { w: self }
    }
    #[doc = "Bits 16:23 - Pre-divider for trig delay and interval ."]
    #[inline]
    pub fn pre_divider(&mut self) -> _PRE_DIVIDERW {
        _PRE_DIVIDERW { w: self }
    }
    #[doc = "Bit 30 - 1'b1: TSC is bypassed; 1'b0: TSC not bypassed;"]
    #[inline]
    pub fn tsc_bypass(&mut self) -> _TSC_BYPASSW {
        _TSC_BYPASSW { w: self }
    }
    #[doc = "Bit 31 - Software reset, high active. When write 1 ,all logical will be reset."]
    #[inline]
    pub fn softrst(&mut self) -> _SOFTRSTW {
        _SOFTRSTW { w: self }
    }
}
