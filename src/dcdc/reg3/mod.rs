#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG3 {
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
pub struct TRGR {
    bits: u8,
}
impl TRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TARGET_LPR {
    bits: u8,
}
impl TARGET_LPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINPWR_DC_HALFCLKR {
    bits: bool,
}
impl MINPWR_DC_HALFCLKR {
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
pub struct MISC_DELAY_TIMINGR {
    bits: bool,
}
impl MISC_DELAY_TIMINGR {
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
pub struct MISC_DISABLEFET_LOGICR {
    bits: bool,
}
impl MISC_DISABLEFET_LOGICR {
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
pub struct DISABLE_STEPR {
    bits: bool,
}
impl DISABLE_STEPR {
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
pub struct _TRGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TARGET_LPW<'a> {
    w: &'a mut W,
}
impl<'a> _TARGET_LPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MINPWR_DC_HALFCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _MINPWR_DC_HALFCLKW<'a> {
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
#[doc = r" Proxy"]
pub struct _MISC_DELAY_TIMINGW<'a> {
    w: &'a mut W,
}
impl<'a> _MISC_DELAY_TIMINGW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MISC_DISABLEFET_LOGICW<'a> {
    w: &'a mut W,
}
impl<'a> _MISC_DISABLEFET_LOGICW<'a> {
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
pub struct _DISABLE_STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_STEPW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Target value of VDD_SOC, 25 mV each step 0x0: 0.8V 0xE: 1.15V 0x1F:1.575V"]
    #[inline]
    pub fn trg(&self) -> TRGR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGR { bits }
    }
    #[doc = "Bits 8:10 - Target value of standby (low power) mode 0x0: 0"]
    #[inline]
    pub fn target_lp(&self) -> TARGET_LPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TARGET_LPR { bits }
    }
    #[doc = "Bit 24 - Set DCDC clock to half freqeuncy for continuous mode"]
    #[inline]
    pub fn minpwr_dc_halfclk(&self) -> MINPWR_DC_HALFCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MINPWR_DC_HALFCLKR { bits }
    }
    #[doc = "Bit 27 - Ajust delay to reduce ground noise"]
    #[inline]
    pub fn misc_delay_timing(&self) -> MISC_DELAY_TIMINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MISC_DELAY_TIMINGR { bits }
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline]
    pub fn misc_disablefet_logic(&self) -> MISC_DISABLEFET_LOGICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MISC_DISABLEFET_LOGICR { bits }
    }
    #[doc = "Bit 30 - Disable stepping for the output VDD_SOC of DCDC"]
    #[inline]
    pub fn disable_step(&self) -> DISABLE_STEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_STEPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 270 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Target value of VDD_SOC, 25 mV each step 0x0: 0.8V 0xE: 1.15V 0x1F:1.575V"]
    #[inline]
    pub fn trg(&mut self) -> _TRGW {
        _TRGW { w: self }
    }
    #[doc = "Bits 8:10 - Target value of standby (low power) mode 0x0: 0"]
    #[inline]
    pub fn target_lp(&mut self) -> _TARGET_LPW {
        _TARGET_LPW { w: self }
    }
    #[doc = "Bit 24 - Set DCDC clock to half freqeuncy for continuous mode"]
    #[inline]
    pub fn minpwr_dc_halfclk(&mut self) -> _MINPWR_DC_HALFCLKW {
        _MINPWR_DC_HALFCLKW { w: self }
    }
    #[doc = "Bit 27 - Ajust delay to reduce ground noise"]
    #[inline]
    pub fn misc_delay_timing(&mut self) -> _MISC_DELAY_TIMINGW {
        _MISC_DELAY_TIMINGW { w: self }
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline]
    pub fn misc_disablefet_logic(&mut self) -> _MISC_DISABLEFET_LOGICW {
        _MISC_DISABLEFET_LOGICW { w: self }
    }
    #[doc = "Bit 30 - Disable stepping for the output VDD_SOC of DCDC"]
    #[inline]
    pub fn disable_step(&mut self) -> _DISABLE_STEPW {
        _DISABLE_STEPW { w: self }
    }
}
