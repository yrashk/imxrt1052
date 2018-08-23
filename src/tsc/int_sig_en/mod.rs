#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_SIG_EN {
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
pub struct MEASURE_SIG_ENR {
    bits: bool,
}
impl MEASURE_SIG_ENR {
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
#[doc = "Possible values of the field `DETECT_SIG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_SIG_ENR {
    #[doc = "Disable detect signal"]
    DETECT_SIG_EN_0,
    #[doc = "Enable detect signal"]
    DETECT_SIG_EN_1,
}
impl DETECT_SIG_ENR {
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
            DETECT_SIG_ENR::DETECT_SIG_EN_0 => false,
            DETECT_SIG_ENR::DETECT_SIG_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DETECT_SIG_ENR {
        match value {
            false => DETECT_SIG_ENR::DETECT_SIG_EN_0,
            true => DETECT_SIG_ENR::DETECT_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_SIG_EN_0`"]
    #[inline]
    pub fn is_detect_sig_en_0(&self) -> bool {
        *self == DETECT_SIG_ENR::DETECT_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `DETECT_SIG_EN_1`"]
    #[inline]
    pub fn is_detect_sig_en_1(&self) -> bool {
        *self == DETECT_SIG_ENR::DETECT_SIG_EN_1
    }
}
#[doc = "Possible values of the field `VALID_SIG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_SIG_ENR {
    #[doc = "Disable valid signal"]
    VALID_SIG_EN_0,
    #[doc = "Enable valid signal"]
    VALID_SIG_EN_1,
}
impl VALID_SIG_ENR {
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
            VALID_SIG_ENR::VALID_SIG_EN_0 => false,
            VALID_SIG_ENR::VALID_SIG_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALID_SIG_ENR {
        match value {
            false => VALID_SIG_ENR::VALID_SIG_EN_0,
            true => VALID_SIG_ENR::VALID_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_SIG_EN_0`"]
    #[inline]
    pub fn is_valid_sig_en_0(&self) -> bool {
        *self == VALID_SIG_ENR::VALID_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `VALID_SIG_EN_1`"]
    #[inline]
    pub fn is_valid_sig_en_1(&self) -> bool {
        *self == VALID_SIG_ENR::VALID_SIG_EN_1
    }
}
#[doc = "Possible values of the field `IDLE_SW_SIG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_SW_SIG_ENR {
    #[doc = "Disable idle software signal"]
    IDLE_SW_SIG_EN_0,
    #[doc = "Enable idle software signal"]
    IDLE_SW_SIG_EN_1,
}
impl IDLE_SW_SIG_ENR {
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
            IDLE_SW_SIG_ENR::IDLE_SW_SIG_EN_0 => false,
            IDLE_SW_SIG_ENR::IDLE_SW_SIG_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLE_SW_SIG_ENR {
        match value {
            false => IDLE_SW_SIG_ENR::IDLE_SW_SIG_EN_0,
            true => IDLE_SW_SIG_ENR::IDLE_SW_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_SIG_EN_0`"]
    #[inline]
    pub fn is_idle_sw_sig_en_0(&self) -> bool {
        *self == IDLE_SW_SIG_ENR::IDLE_SW_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_SIG_EN_1`"]
    #[inline]
    pub fn is_idle_sw_sig_en_1(&self) -> bool {
        *self == IDLE_SW_SIG_ENR::IDLE_SW_SIG_EN_1
    }
}
#[doc = r" Proxy"]
pub struct _MEASURE_SIG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MEASURE_SIG_ENW<'a> {
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
#[doc = "Values that can be written to the field `DETECT_SIG_EN`"]
pub enum DETECT_SIG_ENW {
    #[doc = "Disable detect signal"]
    DETECT_SIG_EN_0,
    #[doc = "Enable detect signal"]
    DETECT_SIG_EN_1,
}
impl DETECT_SIG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DETECT_SIG_ENW::DETECT_SIG_EN_0 => false,
            DETECT_SIG_ENW::DETECT_SIG_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DETECT_SIG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DETECT_SIG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DETECT_SIG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable detect signal"]
    #[inline]
    pub fn detect_sig_en_0(self) -> &'a mut W {
        self.variant(DETECT_SIG_ENW::DETECT_SIG_EN_0)
    }
    #[doc = "Enable detect signal"]
    #[inline]
    pub fn detect_sig_en_1(self) -> &'a mut W {
        self.variant(DETECT_SIG_ENW::DETECT_SIG_EN_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VALID_SIG_EN`"]
pub enum VALID_SIG_ENW {
    #[doc = "Disable valid signal"]
    VALID_SIG_EN_0,
    #[doc = "Enable valid signal"]
    VALID_SIG_EN_1,
}
impl VALID_SIG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VALID_SIG_ENW::VALID_SIG_EN_0 => false,
            VALID_SIG_ENW::VALID_SIG_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALID_SIG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VALID_SIG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALID_SIG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable valid signal"]
    #[inline]
    pub fn valid_sig_en_0(self) -> &'a mut W {
        self.variant(VALID_SIG_ENW::VALID_SIG_EN_0)
    }
    #[doc = "Enable valid signal"]
    #[inline]
    pub fn valid_sig_en_1(self) -> &'a mut W {
        self.variant(VALID_SIG_ENW::VALID_SIG_EN_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLE_SW_SIG_EN`"]
pub enum IDLE_SW_SIG_ENW {
    #[doc = "Disable idle software signal"]
    IDLE_SW_SIG_EN_0,
    #[doc = "Enable idle software signal"]
    IDLE_SW_SIG_EN_1,
}
impl IDLE_SW_SIG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLE_SW_SIG_ENW::IDLE_SW_SIG_EN_0 => false,
            IDLE_SW_SIG_ENW::IDLE_SW_SIG_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLE_SW_SIG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLE_SW_SIG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLE_SW_SIG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable idle software signal"]
    #[inline]
    pub fn idle_sw_sig_en_0(self) -> &'a mut W {
        self.variant(IDLE_SW_SIG_ENW::IDLE_SW_SIG_EN_0)
    }
    #[doc = "Enable idle software signal"]
    #[inline]
    pub fn idle_sw_sig_en_1(self) -> &'a mut W {
        self.variant(IDLE_SW_SIG_ENW::IDLE_SW_SIG_EN_1)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Measure Signal Enable"]
    #[inline]
    pub fn measure_sig_en(&self) -> MEASURE_SIG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEASURE_SIG_ENR { bits }
    }
    #[doc = "Bit 4 - Detect Signal Enable"]
    #[inline]
    pub fn detect_sig_en(&self) -> DETECT_SIG_ENR {
        DETECT_SIG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Valid Signal Enable"]
    #[inline]
    pub fn valid_sig_en(&self) -> VALID_SIG_ENR {
        VALID_SIG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Idle Software Signal Enable"]
    #[inline]
    pub fn idle_sw_sig_en(&self) -> IDLE_SW_SIG_ENR {
        IDLE_SW_SIG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Measure Signal Enable"]
    #[inline]
    pub fn measure_sig_en(&mut self) -> _MEASURE_SIG_ENW {
        _MEASURE_SIG_ENW { w: self }
    }
    #[doc = "Bit 4 - Detect Signal Enable"]
    #[inline]
    pub fn detect_sig_en(&mut self) -> _DETECT_SIG_ENW {
        _DETECT_SIG_ENW { w: self }
    }
    #[doc = "Bit 8 - Valid Signal Enable"]
    #[inline]
    pub fn valid_sig_en(&mut self) -> _VALID_SIG_ENW {
        _VALID_SIG_ENW { w: self }
    }
    #[doc = "Bit 12 - Idle Software Signal Enable"]
    #[inline]
    pub fn idle_sw_sig_en(&mut self) -> _IDLE_SW_SIG_ENW {
        _IDLE_SW_SIG_ENW { w: self }
    }
}
