#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG_MODE {
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
pub struct ADC_CONV_VALUER {
    bits: u16,
}
impl ADC_CONV_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_COCOR {
    bits: bool,
}
impl ADC_COCOR {
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
pub struct EXT_HWTSR {
    bits: u8,
}
impl EXT_HWTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TRIGGER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERR {
    #[doc = "No hardware trigger signal"]
    TRIGGER_0,
    #[doc = "Hardware trigger signal, the signal must last at least 1 ips clock period"]
    TRIGGER_1,
}
impl TRIGGERR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TRIGGERR::TRIGGER_0 => false,
            TRIGGERR::TRIGGER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERR {
        match value {
            false => TRIGGERR::TRIGGER_0,
            true => TRIGGERR::TRIGGER_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGER_0`"]
    #[inline]
    pub fn is_trigger_0(&self) -> bool {
        *self == TRIGGERR::TRIGGER_0
    }
    #[doc = "Checks if the value of the field is `TRIGGER_1`"]
    #[inline]
    pub fn is_trigger_1(&self) -> bool {
        *self == TRIGGERR::TRIGGER_1
    }
}
#[doc = "Possible values of the field `ADC_COCO_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_COCO_CLEARR {
    #[doc = "No ADC COCO clear"]
    ADC_COCO_CLEAR_0,
    #[doc = "Set ADC COCO clear"]
    ADC_COCO_CLEAR_1,
}
impl ADC_COCO_CLEARR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADC_COCO_CLEARR::ADC_COCO_CLEAR_0 => false,
            ADC_COCO_CLEARR::ADC_COCO_CLEAR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_COCO_CLEARR {
        match value {
            false => ADC_COCO_CLEARR::ADC_COCO_CLEAR_0,
            true => ADC_COCO_CLEARR::ADC_COCO_CLEAR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_0`"]
    #[inline]
    pub fn is_adc_coco_clear_0(&self) -> bool {
        *self == ADC_COCO_CLEARR::ADC_COCO_CLEAR_0
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_1`"]
    #[inline]
    pub fn is_adc_coco_clear_1(&self) -> bool {
        *self == ADC_COCO_CLEARR::ADC_COCO_CLEAR_1
    }
}
#[doc = "Possible values of the field `ADC_COCO_CLEAR_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_COCO_CLEAR_DISABLER {
    #[doc = "Allow TSC hardware generates ADC COCO clear"]
    ADC_COCO_CLEAR_DISABLE_0,
    #[doc = "Prevent TSC from generate ADC COCO clear signal"]
    ADC_COCO_CLEAR_DISABLE_1,
}
impl ADC_COCO_CLEAR_DISABLER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADC_COCO_CLEAR_DISABLER::ADC_COCO_CLEAR_DISABLE_0 => false,
            ADC_COCO_CLEAR_DISABLER::ADC_COCO_CLEAR_DISABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_COCO_CLEAR_DISABLER {
        match value {
            false => ADC_COCO_CLEAR_DISABLER::ADC_COCO_CLEAR_DISABLE_0,
            true => ADC_COCO_CLEAR_DISABLER::ADC_COCO_CLEAR_DISABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_DISABLE_0`"]
    #[inline]
    pub fn is_adc_coco_clear_disable_0(&self) -> bool {
        *self == ADC_COCO_CLEAR_DISABLER::ADC_COCO_CLEAR_DISABLE_0
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_DISABLE_1`"]
    #[inline]
    pub fn is_adc_coco_clear_disable_1(&self) -> bool {
        *self == ADC_COCO_CLEAR_DISABLER::ADC_COCO_CLEAR_DISABLE_1
    }
}
#[doc = "Possible values of the field `DEBUG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_ENR {
    #[doc = "Enable debug mode"]
    DEBUG_EN_0,
    #[doc = "Disable debug mode"]
    DEBUG_EN_1,
}
impl DEBUG_ENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DEBUG_ENR::DEBUG_EN_0 => false,
            DEBUG_ENR::DEBUG_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBUG_ENR {
        match value {
            false => DEBUG_ENR::DEBUG_EN_0,
            true => DEBUG_ENR::DEBUG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBUG_EN_0`"]
    #[inline]
    pub fn is_debug_en_0(&self) -> bool {
        *self == DEBUG_ENR::DEBUG_EN_0
    }
    #[doc = "Checks if the value of the field is `DEBUG_EN_1`"]
    #[inline]
    pub fn is_debug_en_1(&self) -> bool {
        *self == DEBUG_ENR::DEBUG_EN_1
    }
}
#[doc = r" Proxy"]
pub struct _EXT_HWTSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT_HWTSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGER`"]
pub enum TRIGGERW {
    #[doc = "No hardware trigger signal"]
    TRIGGER_0,
    #[doc = "Hardware trigger signal, the signal must last at least 1 ips clock period"]
    TRIGGER_1,
}
impl TRIGGERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERW::TRIGGER_0 => false,
            TRIGGERW::TRIGGER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No hardware trigger signal"]
    #[inline]
    pub fn trigger_0(self) -> &'a mut W {
        self.variant(TRIGGERW::TRIGGER_0)
    }
    #[doc = "Hardware trigger signal, the signal must last at least 1 ips clock period"]
    #[inline]
    pub fn trigger_1(self) -> &'a mut W {
        self.variant(TRIGGERW::TRIGGER_1)
    }
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
#[doc = "Values that can be written to the field `ADC_COCO_CLEAR`"]
pub enum ADC_COCO_CLEARW {
    #[doc = "No ADC COCO clear"]
    ADC_COCO_CLEAR_0,
    #[doc = "Set ADC COCO clear"]
    ADC_COCO_CLEAR_1,
}
impl ADC_COCO_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_COCO_CLEARW::ADC_COCO_CLEAR_0 => false,
            ADC_COCO_CLEARW::ADC_COCO_CLEAR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_COCO_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_COCO_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_COCO_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ADC COCO clear"]
    #[inline]
    pub fn adc_coco_clear_0(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEARW::ADC_COCO_CLEAR_0)
    }
    #[doc = "Set ADC COCO clear"]
    #[inline]
    pub fn adc_coco_clear_1(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEARW::ADC_COCO_CLEAR_1)
    }
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_COCO_CLEAR_DISABLE`"]
pub enum ADC_COCO_CLEAR_DISABLEW {
    #[doc = "Allow TSC hardware generates ADC COCO clear"]
    ADC_COCO_CLEAR_DISABLE_0,
    #[doc = "Prevent TSC from generate ADC COCO clear signal"]
    ADC_COCO_CLEAR_DISABLE_1,
}
impl ADC_COCO_CLEAR_DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_COCO_CLEAR_DISABLEW::ADC_COCO_CLEAR_DISABLE_0 => false,
            ADC_COCO_CLEAR_DISABLEW::ADC_COCO_CLEAR_DISABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_COCO_CLEAR_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_COCO_CLEAR_DISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_COCO_CLEAR_DISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow TSC hardware generates ADC COCO clear"]
    #[inline]
    pub fn adc_coco_clear_disable_0(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEAR_DISABLEW::ADC_COCO_CLEAR_DISABLE_0)
    }
    #[doc = "Prevent TSC from generate ADC COCO clear signal"]
    #[inline]
    pub fn adc_coco_clear_disable_1(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEAR_DISABLEW::ADC_COCO_CLEAR_DISABLE_1)
    }
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEBUG_EN`"]
pub enum DEBUG_ENW {
    #[doc = "Enable debug mode"]
    DEBUG_EN_0,
    #[doc = "Disable debug mode"]
    DEBUG_EN_1,
}
impl DEBUG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBUG_ENW::DEBUG_EN_0 => false,
            DEBUG_ENW::DEBUG_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBUG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBUG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable debug mode"]
    #[inline]
    pub fn debug_en_0(self) -> &'a mut W {
        self.variant(DEBUG_ENW::DEBUG_EN_0)
    }
    #[doc = "Disable debug mode"]
    #[inline]
    pub fn debug_en_1(self) -> &'a mut W {
        self.variant(DEBUG_ENW::DEBUG_EN_1)
    }
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
    #[doc = "Bits 0:11 - ADC Conversion Value"]
    #[inline]
    pub fn adc_conv_value(&self) -> ADC_CONV_VALUER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ADC_CONV_VALUER { bits }
    }
    #[doc = "Bit 12 - ADC COCO Signal"]
    #[inline]
    pub fn adc_coco(&self) -> ADC_COCOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_COCOR { bits }
    }
    #[doc = "Bits 16:20 - Hardware Trigger Select Signal"]
    #[inline]
    pub fn ext_hwts(&self) -> EXT_HWTSR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXT_HWTSR { bits }
    }
    #[doc = "Bit 24 - Trigger"]
    #[inline]
    pub fn trigger(&self) -> TRIGGERR {
        TRIGGERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - ADC Coco Clear"]
    #[inline]
    pub fn adc_coco_clear(&self) -> ADC_COCO_CLEARR {
        ADC_COCO_CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - ADC COCO Clear Disable"]
    #[inline]
    pub fn adc_coco_clear_disable(&self) -> ADC_COCO_CLEAR_DISABLER {
        ADC_COCO_CLEAR_DISABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Debug Enable"]
    #[inline]
    pub fn debug_en(&self) -> DEBUG_ENR {
        DEBUG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 16:20 - Hardware Trigger Select Signal"]
    #[inline]
    pub fn ext_hwts(&mut self) -> _EXT_HWTSW {
        _EXT_HWTSW { w: self }
    }
    #[doc = "Bit 24 - Trigger"]
    #[inline]
    pub fn trigger(&mut self) -> _TRIGGERW {
        _TRIGGERW { w: self }
    }
    #[doc = "Bit 25 - ADC Coco Clear"]
    #[inline]
    pub fn adc_coco_clear(&mut self) -> _ADC_COCO_CLEARW {
        _ADC_COCO_CLEARW { w: self }
    }
    #[doc = "Bit 26 - ADC COCO Clear Disable"]
    #[inline]
    pub fn adc_coco_clear_disable(&mut self) -> _ADC_COCO_CLEAR_DISABLEW {
        _ADC_COCO_CLEAR_DISABLEW { w: self }
    }
    #[doc = "Bit 28 - Debug Enable"]
    #[inline]
    pub fn debug_en(&mut self) -> _DEBUG_ENW {
        _DEBUG_ENW { w: self }
    }
}
