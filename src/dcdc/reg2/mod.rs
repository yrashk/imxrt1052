#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG2 {
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
pub struct LOOPCTRL_DC_CR {
    bits: u8,
}
impl LOOPCTRL_DC_CR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOOPCTRL_DC_RR {
    bits: u8,
}
impl LOOPCTRL_DC_RR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOOPCTRL_DC_FFR {
    bits: u8,
}
impl LOOPCTRL_DC_FFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOOPCTRL_EN_RCSCALER {
    bits: u8,
}
impl LOOPCTRL_EN_RCSCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOOPCTRL_RCSCALE_THRSHR {
    bits: bool,
}
impl LOOPCTRL_RCSCALE_THRSHR {
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
pub struct LOOPCTRL_HYST_SIGNR {
    bits: bool,
}
impl LOOPCTRL_HYST_SIGNR {
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
pub struct DISABLE_PULSE_SKIPR {
    bits: bool,
}
impl DISABLE_PULSE_SKIPR {
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
pub struct DCM_SET_CTRLR {
    bits: bool,
}
impl DCM_SET_CTRLR {
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
pub struct _LOOPCTRL_DC_CW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_DC_CW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOOPCTRL_DC_RW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_DC_RW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOOPCTRL_DC_FFW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_DC_FFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOOPCTRL_EN_RCSCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_EN_RCSCALEW<'a> {
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
pub struct _LOOPCTRL_RCSCALE_THRSHW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_RCSCALE_THRSHW<'a> {
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
pub struct _LOOPCTRL_HYST_SIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPCTRL_HYST_SIGNW<'a> {
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
pub struct _DISABLE_PULSE_SKIPW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_PULSE_SKIPW<'a> {
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
pub struct _DCM_SET_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _DCM_SET_CTRLW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Ratio of integral control parameter to proportional control parameter in the switching DC-DC converter, and can be used to optimize efficiency and loop response"]
    #[inline]
    pub fn loopctrl_dc_c(&self) -> LOOPCTRL_DC_CR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOOPCTRL_DC_CR { bits }
    }
    #[doc = "Bits 2:5 - Magnitude of proportional control parameter in the switching DC-DC converter control loop."]
    #[inline]
    pub fn loopctrl_dc_r(&self) -> LOOPCTRL_DC_RR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOOPCTRL_DC_RR { bits }
    }
    #[doc = "Bits 6:8 - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline]
    pub fn loopctrl_dc_ff(&self) -> LOOPCTRL_DC_FFR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOOPCTRL_DC_FFR { bits }
    }
    #[doc = "Bits 9:11 - Enable analog circuit of DC-DC converter to respond faster under transient load conditions."]
    #[inline]
    pub fn loopctrl_en_rcscale(&self) -> LOOPCTRL_EN_RCSCALER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOOPCTRL_EN_RCSCALER { bits }
    }
    #[doc = "Bit 12 - Increase the threshold detection for RC scale circuit."]
    #[inline]
    pub fn loopctrl_rcscale_thrsh(&self) -> LOOPCTRL_RCSCALE_THRSHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPCTRL_RCSCALE_THRSHR { bits }
    }
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline]
    pub fn loopctrl_hyst_sign(&self) -> LOOPCTRL_HYST_SIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPCTRL_HYST_SIGNR { bits }
    }
    #[doc = "Bit 27 - Set to \"0\" : stop charging if the duty cycle is lower than what set by dcdc_neglimit_in"]
    #[inline]
    pub fn disable_pulse_skip(&self) -> DISABLE_PULSE_SKIPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_PULSE_SKIPR { bits }
    }
    #[doc = "Bit 28 - Set high to improve the transition from heavy load to light load"]
    #[inline]
    pub fn dcm_set_ctrl(&self) -> DCM_SET_CTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCM_SET_CTRLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16393 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Ratio of integral control parameter to proportional control parameter in the switching DC-DC converter, and can be used to optimize efficiency and loop response"]
    #[inline]
    pub fn loopctrl_dc_c(&mut self) -> _LOOPCTRL_DC_CW {
        _LOOPCTRL_DC_CW { w: self }
    }
    #[doc = "Bits 2:5 - Magnitude of proportional control parameter in the switching DC-DC converter control loop."]
    #[inline]
    pub fn loopctrl_dc_r(&mut self) -> _LOOPCTRL_DC_RW {
        _LOOPCTRL_DC_RW { w: self }
    }
    #[doc = "Bits 6:8 - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline]
    pub fn loopctrl_dc_ff(&mut self) -> _LOOPCTRL_DC_FFW {
        _LOOPCTRL_DC_FFW { w: self }
    }
    #[doc = "Bits 9:11 - Enable analog circuit of DC-DC converter to respond faster under transient load conditions."]
    #[inline]
    pub fn loopctrl_en_rcscale(&mut self) -> _LOOPCTRL_EN_RCSCALEW {
        _LOOPCTRL_EN_RCSCALEW { w: self }
    }
    #[doc = "Bit 12 - Increase the threshold detection for RC scale circuit."]
    #[inline]
    pub fn loopctrl_rcscale_thrsh(&mut self) -> _LOOPCTRL_RCSCALE_THRSHW {
        _LOOPCTRL_RCSCALE_THRSHW { w: self }
    }
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline]
    pub fn loopctrl_hyst_sign(&mut self) -> _LOOPCTRL_HYST_SIGNW {
        _LOOPCTRL_HYST_SIGNW { w: self }
    }
    #[doc = "Bit 27 - Set to \"0\" : stop charging if the duty cycle is lower than what set by dcdc_neglimit_in"]
    #[inline]
    pub fn disable_pulse_skip(&mut self) -> _DISABLE_PULSE_SKIPW {
        _DISABLE_PULSE_SKIPW { w: self }
    }
    #[doc = "Bit 28 - Set high to improve the transition from heavy load to light load"]
    #[inline]
    pub fn dcm_set_ctrl(&mut self) -> _DCM_SET_CTRLW {
        _DCM_SET_CTRLW { w: self }
    }
}
