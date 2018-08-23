#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG1 {
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
pub struct REG_FBK_SELR {
    bits: u8,
}
impl REG_FBK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REG_RLOAD_SWR {
    bits: bool,
}
impl REG_RLOAD_SWR {
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
pub struct LP_CMP_ISRC_SELR {
    bits: u8,
}
impl LP_CMP_ISRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOOPCTRL_HST_THRESHR {
    bits: bool,
}
impl LOOPCTRL_HST_THRESHR {
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
pub struct LOOPCTRL_EN_HYSTR {
    bits: bool,
}
impl LOOPCTRL_EN_HYSTR {
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
pub struct VBG_TRIMR {
    bits: u8,
}
impl VBG_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _REG_FBK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _REG_FBK_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REG_RLOAD_SWW<'a> {
    w: &'a mut W,
}
impl<'a> _REG_RLOAD_SWW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LP_CMP_ISRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LP_CMP_ISRC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOOPCTRL_HST_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_HST_THRESHW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOOPCTRL_EN_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_EN_HYSTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBG_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _VBG_TRIMW<'a> {
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
    #[doc = "Bits 7:8 - select the feedback point of the internal regulator"]
    #[inline]
    pub fn reg_fbk_sel(&self) -> REG_FBK_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REG_FBK_SELR { bits }
    }
    #[doc = "Bit 9 - control the load resistor of the internal regulator of DCDC, the load resistor is connected as default \"1\", and need set to \"0\" to disconnect the load resistor"]
    #[inline]
    pub fn reg_rload_sw(&self) -> REG_RLOAD_SWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REG_RLOAD_SWR { bits }
    }
    #[doc = "Bits 12:13 - set the current bias of low power comparator 0x0: 50 nA 0x1: 100 nA 0x2: 200 nA 0x3: 400 nA"]
    #[inline]
    pub fn lp_cmp_isrc_sel(&self) -> LP_CMP_ISRC_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LP_CMP_ISRC_SELR { bits }
    }
    #[doc = "Bit 21 - increase the threshold detection for common mode analog comparator"]
    #[inline]
    pub fn loopctrl_hst_thresh(&self) -> LOOPCTRL_HST_THRESHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPCTRL_HST_THRESHR { bits }
    }
    #[doc = "Bit 23 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline]
    pub fn loopctrl_en_hyst(&self) -> LOOPCTRL_EN_HYSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPCTRL_EN_HYSTR { bits }
    }
    #[doc = "Bits 24:28 - trim bandgap voltage"]
    #[inline]
    pub fn vbg_trim(&self) -> VBG_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VBG_TRIMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 287023772 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 7:8 - select the feedback point of the internal regulator"]
    #[inline]
    pub fn reg_fbk_sel(&mut self) -> _REG_FBK_SELW {
        _REG_FBK_SELW { w: self }
    }
    #[doc = "Bit 9 - control the load resistor of the internal regulator of DCDC, the load resistor is connected as default \"1\", and need set to \"0\" to disconnect the load resistor"]
    #[inline]
    pub fn reg_rload_sw(&mut self) -> _REG_RLOAD_SWW {
        _REG_RLOAD_SWW { w: self }
    }
    #[doc = "Bits 12:13 - set the current bias of low power comparator 0x0: 50 nA 0x1: 100 nA 0x2: 200 nA 0x3: 400 nA"]
    #[inline]
    pub fn lp_cmp_isrc_sel(&mut self) -> _LP_CMP_ISRC_SELW {
        _LP_CMP_ISRC_SELW { w: self }
    }
    #[doc = "Bit 21 - increase the threshold detection for common mode analog comparator"]
    #[inline]
    pub fn loopctrl_hst_thresh(&mut self) -> _LOOPCTRL_HST_THRESHW {
        _LOOPCTRL_HST_THRESHW { w: self }
    }
    #[doc = "Bit 23 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline]
    pub fn loopctrl_en_hyst(&mut self) -> _LOOPCTRL_EN_HYSTW {
        _LOOPCTRL_EN_HYSTW { w: self }
    }
    #[doc = "Bits 24:28 - trim bandgap voltage"]
    #[inline]
    pub fn vbg_trim(&mut self) -> _VBG_TRIMW {
        _VBG_TRIMW { w: self }
    }
}
