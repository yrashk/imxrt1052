#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_STATUS {
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
#[doc = "Possible values of the field `MEASURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEASURER {
    #[doc = "Does not exist a measure signal"]
    MEASURE_0,
    #[doc = "Exist a measure signal"]
    MEASURE_1,
}
impl MEASURER {
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
            MEASURER::MEASURE_0 => false,
            MEASURER::MEASURE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEASURER {
        match value {
            false => MEASURER::MEASURE_0,
            true => MEASURER::MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEASURE_0`"]
    #[inline]
    pub fn is_measure_0(&self) -> bool {
        *self == MEASURER::MEASURE_0
    }
    #[doc = "Checks if the value of the field is `MEASURE_1`"]
    #[inline]
    pub fn is_measure_1(&self) -> bool {
        *self == MEASURER::MEASURE_1
    }
}
#[doc = "Possible values of the field `DETECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECTR {
    #[doc = "Does not exist a detect signal"]
    DETECT_0,
    #[doc = "Exist detect signal"]
    DETECT_1,
}
impl DETECTR {
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
            DETECTR::DETECT_0 => false,
            DETECTR::DETECT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DETECTR {
        match value {
            false => DETECTR::DETECT_0,
            true => DETECTR::DETECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_0`"]
    #[inline]
    pub fn is_detect_0(&self) -> bool {
        *self == DETECTR::DETECT_0
    }
    #[doc = "Checks if the value of the field is `DETECT_1`"]
    #[inline]
    pub fn is_detect_1(&self) -> bool {
        *self == DETECTR::DETECT_1
    }
}
#[doc = "Possible values of the field `VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDR {
    #[doc = "There is no touch detected after measurement, indicates that the measured value is not valid"]
    VALID_0,
    #[doc = "There is touch detection after measurement, indicates that the measure is valid"]
    VALID_1,
}
impl VALIDR {
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
            VALIDR::VALID_0 => false,
            VALIDR::VALID_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALIDR {
        match value {
            false => VALIDR::VALID_0,
            true => VALIDR::VALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_0`"]
    #[inline]
    pub fn is_valid_0(&self) -> bool {
        *self == VALIDR::VALID_0
    }
    #[doc = "Checks if the value of the field is `VALID_1`"]
    #[inline]
    pub fn is_valid_1(&self) -> bool {
        *self == VALIDR::VALID_1
    }
}
#[doc = "Possible values of the field `IDLE_SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_SWR {
    #[doc = "Haven't return to idle status"]
    IDLE_SW_0,
    #[doc = "Already return to idle status"]
    IDLE_SW_1,
}
impl IDLE_SWR {
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
            IDLE_SWR::IDLE_SW_0 => false,
            IDLE_SWR::IDLE_SW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLE_SWR {
        match value {
            false => IDLE_SWR::IDLE_SW_0,
            true => IDLE_SWR::IDLE_SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_0`"]
    #[inline]
    pub fn is_idle_sw_0(&self) -> bool {
        *self == IDLE_SWR::IDLE_SW_0
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_1`"]
    #[inline]
    pub fn is_idle_sw_1(&self) -> bool {
        *self == IDLE_SWR::IDLE_SW_1
    }
}
#[doc = "Values that can be written to the field `MEASURE`"]
pub enum MEASUREW {
    #[doc = "Does not exist a measure signal"]
    MEASURE_0,
    #[doc = "Exist a measure signal"]
    MEASURE_1,
}
impl MEASUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEASUREW::MEASURE_0 => false,
            MEASUREW::MEASURE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEASUREW<'a> {
    w: &'a mut W,
}
impl<'a> _MEASUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEASUREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Does not exist a measure signal"]
    #[inline]
    pub fn measure_0(self) -> &'a mut W {
        self.variant(MEASUREW::MEASURE_0)
    }
    #[doc = "Exist a measure signal"]
    #[inline]
    pub fn measure_1(self) -> &'a mut W {
        self.variant(MEASUREW::MEASURE_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DETECT`"]
pub enum DETECTW {
    #[doc = "Does not exist a detect signal"]
    DETECT_0,
    #[doc = "Exist detect signal"]
    DETECT_1,
}
impl DETECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DETECTW::DETECT_0 => false,
            DETECTW::DETECT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _DETECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DETECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Does not exist a detect signal"]
    #[inline]
    pub fn detect_0(self) -> &'a mut W {
        self.variant(DETECTW::DETECT_0)
    }
    #[doc = "Exist detect signal"]
    #[inline]
    pub fn detect_1(self) -> &'a mut W {
        self.variant(DETECTW::DETECT_1)
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
#[doc = "Values that can be written to the field `VALID`"]
pub enum VALIDW {
    #[doc = "There is no touch detected after measurement, indicates that the measured value is not valid"]
    VALID_0,
    #[doc = "There is touch detection after measurement, indicates that the measure is valid"]
    VALID_1,
}
impl VALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VALIDW::VALID_0 => false,
            VALIDW::VALID_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _VALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "There is no touch detected after measurement, indicates that the measured value is not valid"]
    #[inline]
    pub fn valid_0(self) -> &'a mut W {
        self.variant(VALIDW::VALID_0)
    }
    #[doc = "There is touch detection after measurement, indicates that the measure is valid"]
    #[inline]
    pub fn valid_1(self) -> &'a mut W {
        self.variant(VALIDW::VALID_1)
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
#[doc = "Values that can be written to the field `IDLE_SW`"]
pub enum IDLE_SWW {
    #[doc = "Haven't return to idle status"]
    IDLE_SW_0,
    #[doc = "Already return to idle status"]
    IDLE_SW_1,
}
impl IDLE_SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLE_SWW::IDLE_SW_0 => false,
            IDLE_SWW::IDLE_SW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLE_SWW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLE_SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLE_SWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Haven't return to idle status"]
    #[inline]
    pub fn idle_sw_0(self) -> &'a mut W {
        self.variant(IDLE_SWW::IDLE_SW_0)
    }
    #[doc = "Already return to idle status"]
    #[inline]
    pub fn idle_sw_1(self) -> &'a mut W {
        self.variant(IDLE_SWW::IDLE_SW_1)
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
    #[doc = "Bit 0 - Measure Signal"]
    #[inline]
    pub fn measure(&self) -> MEASURER {
        MEASURER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Detect Signal"]
    #[inline]
    pub fn detect(&self) -> DETECTR {
        DETECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Valid Signal"]
    #[inline]
    pub fn valid(&self) -> VALIDR {
        VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Idle Software"]
    #[inline]
    pub fn idle_sw(&self) -> IDLE_SWR {
        IDLE_SWR::_from({
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
    #[doc = "Bit 0 - Measure Signal"]
    #[inline]
    pub fn measure(&mut self) -> _MEASUREW {
        _MEASUREW { w: self }
    }
    #[doc = "Bit 4 - Detect Signal"]
    #[inline]
    pub fn detect(&mut self) -> _DETECTW {
        _DETECTW { w: self }
    }
    #[doc = "Bit 8 - Valid Signal"]
    #[inline]
    pub fn valid(&mut self) -> _VALIDW {
        _VALIDW { w: self }
    }
    #[doc = "Bit 12 - Idle Software"]
    #[inline]
    pub fn idle_sw(&mut self) -> _IDLE_SWW {
        _IDLE_SWW { w: self }
    }
}
