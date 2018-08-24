#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GS {
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
#[doc = "Possible values of the field `ADACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACTR {
    #[doc = "Conversion not in progress."]
    ADACT_0,
    #[doc = "Conversion in progress."]
    ADACT_1,
}
impl ADACTR {
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
            ADACTR::ADACT_0 => false,
            ADACTR::ADACT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADACTR {
        match value {
            false => ADACTR::ADACT_0,
            true => ADACTR::ADACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADACT_0`"]
    #[inline]
    pub fn is_adact_0(&self) -> bool {
        *self == ADACTR::ADACT_0
    }
    #[doc = "Checks if the value of the field is `ADACT_1`"]
    #[inline]
    pub fn is_adact_1(&self) -> bool {
        *self == ADACTR::ADACT_1
    }
}
#[doc = "Possible values of the field `CALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALFR {
    #[doc = "Calibration completed normally."]
    CALF_0,
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    CALF_1,
}
impl CALFR {
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
            CALFR::CALF_0 => false,
            CALFR::CALF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALFR {
        match value {
            false => CALFR::CALF_0,
            true => CALFR::CALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALF_0`"]
    #[inline]
    pub fn is_calf_0(&self) -> bool {
        *self == CALFR::CALF_0
    }
    #[doc = "Checks if the value of the field is `CALF_1`"]
    #[inline]
    pub fn is_calf_1(&self) -> bool {
        *self == CALFR::CALF_1
    }
}
#[doc = "Possible values of the field `AWKST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWKSTR {
    #[doc = "No asynchronous interrupt."]
    AWKST_0,
    #[doc = "Asynchronous wake up interrupt occurred in stop mode."]
    AWKST_1,
}
impl AWKSTR {
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
            AWKSTR::AWKST_0 => false,
            AWKSTR::AWKST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWKSTR {
        match value {
            false => AWKSTR::AWKST_0,
            true => AWKSTR::AWKST_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWKST_0`"]
    #[inline]
    pub fn is_awkst_0(&self) -> bool {
        *self == AWKSTR::AWKST_0
    }
    #[doc = "Checks if the value of the field is `AWKST_1`"]
    #[inline]
    pub fn is_awkst_1(&self) -> bool {
        *self == AWKSTR::AWKST_1
    }
}
#[doc = "Values that can be written to the field `CALF`"]
pub enum CALFW {
    #[doc = "Calibration completed normally."]
    CALF_0,
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    CALF_1,
}
impl CALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CALFW::CALF_0 => false,
            CALFW::CALF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALFW<'a> {
    w: &'a mut W,
}
impl<'a> _CALFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Calibration completed normally."]
    #[inline]
    pub fn calf_0(self) -> &'a mut W {
        self.variant(CALFW::CALF_0)
    }
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    #[inline]
    pub fn calf_1(self) -> &'a mut W {
        self.variant(CALFW::CALF_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AWKST`"]
pub enum AWKSTW {
    #[doc = "No asynchronous interrupt."]
    AWKST_0,
    #[doc = "Asynchronous wake up interrupt occurred in stop mode."]
    AWKST_1,
}
impl AWKSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWKSTW::AWKST_0 => false,
            AWKSTW::AWKST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWKSTW<'a> {
    w: &'a mut W,
}
impl<'a> _AWKSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWKSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No asynchronous interrupt."]
    #[inline]
    pub fn awkst_0(self) -> &'a mut W {
        self.variant(AWKSTW::AWKST_0)
    }
    #[doc = "Asynchronous wake up interrupt occurred in stop mode."]
    #[inline]
    pub fn awkst_1(self) -> &'a mut W {
        self.variant(AWKSTW::AWKST_1)
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
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Conversion Active"]
    #[inline]
    pub fn adact(&self) -> ADACTR {
        ADACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Calibration Failed Flag"]
    #[inline]
    pub fn calf(&self) -> CALFR {
        CALFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Asynchronous wakeup interrupt status"]
    #[inline]
    pub fn awkst(&self) -> AWKSTR {
        AWKSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 1 - Calibration Failed Flag"]
    #[inline]
    pub fn calf(&mut self) -> _CALFW {
        _CALFW { w: self }
    }
    #[doc = "Bit 2 - Asynchronous wakeup interrupt status"]
    #[inline]
    pub fn awkst(&mut self) -> _AWKSTW {
        _AWKSTW { w: self }
    }
}
