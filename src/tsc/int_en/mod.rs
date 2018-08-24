#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_EN {
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
#[doc = "Possible values of the field `MEASURE_INT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEASURE_INT_ENR {
    #[doc = "Disable measure"]
    MEASURE_INT_EN_0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MEASURE_INT_ENR {
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
            MEASURE_INT_ENR::MEASURE_INT_EN_0 => false,
            MEASURE_INT_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEASURE_INT_ENR {
        match value {
            false => MEASURE_INT_ENR::MEASURE_INT_EN_0,
            i => MEASURE_INT_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MEASURE_INT_EN_0`"]
    #[inline]
    pub fn is_measure_int_en_0(&self) -> bool {
        *self == MEASURE_INT_ENR::MEASURE_INT_EN_0
    }
}
#[doc = "Possible values of the field `DETECT_INT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_INT_ENR {
    #[doc = "Disable detect interrupt"]
    DETECT_INT_EN_0,
    #[doc = "Enable detect interrupt"]
    DETECT_INT_EN_1,
}
impl DETECT_INT_ENR {
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
            DETECT_INT_ENR::DETECT_INT_EN_0 => false,
            DETECT_INT_ENR::DETECT_INT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DETECT_INT_ENR {
        match value {
            false => DETECT_INT_ENR::DETECT_INT_EN_0,
            true => DETECT_INT_ENR::DETECT_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_INT_EN_0`"]
    #[inline]
    pub fn is_detect_int_en_0(&self) -> bool {
        *self == DETECT_INT_ENR::DETECT_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `DETECT_INT_EN_1`"]
    #[inline]
    pub fn is_detect_int_en_1(&self) -> bool {
        *self == DETECT_INT_ENR::DETECT_INT_EN_1
    }
}
#[doc = "Possible values of the field `IDLE_SW_INT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_SW_INT_ENR {
    #[doc = "Disable idle software interrupt"]
    IDLE_SW_INT_EN_0,
    #[doc = "Enable idle software interrupt"]
    IDLE_SW_INT_EN_1,
}
impl IDLE_SW_INT_ENR {
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
            IDLE_SW_INT_ENR::IDLE_SW_INT_EN_0 => false,
            IDLE_SW_INT_ENR::IDLE_SW_INT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLE_SW_INT_ENR {
        match value {
            false => IDLE_SW_INT_ENR::IDLE_SW_INT_EN_0,
            true => IDLE_SW_INT_ENR::IDLE_SW_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_INT_EN_0`"]
    #[inline]
    pub fn is_idle_sw_int_en_0(&self) -> bool {
        *self == IDLE_SW_INT_ENR::IDLE_SW_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_INT_EN_1`"]
    #[inline]
    pub fn is_idle_sw_int_en_1(&self) -> bool {
        *self == IDLE_SW_INT_ENR::IDLE_SW_INT_EN_1
    }
}
#[doc = "Values that can be written to the field `MEASURE_INT_EN`"]
pub enum MEASURE_INT_ENW {
    #[doc = "Disable measure"]
    MEASURE_INT_EN_0,
}
impl MEASURE_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEASURE_INT_ENW::MEASURE_INT_EN_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEASURE_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MEASURE_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEASURE_INT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable measure"]
    #[inline]
    pub fn measure_int_en_0(self) -> &'a mut W {
        self.variant(MEASURE_INT_ENW::MEASURE_INT_EN_0)
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
#[doc = "Values that can be written to the field `DETECT_INT_EN`"]
pub enum DETECT_INT_ENW {
    #[doc = "Disable detect interrupt"]
    DETECT_INT_EN_0,
    #[doc = "Enable detect interrupt"]
    DETECT_INT_EN_1,
}
impl DETECT_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DETECT_INT_ENW::DETECT_INT_EN_0 => false,
            DETECT_INT_ENW::DETECT_INT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DETECT_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DETECT_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DETECT_INT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable detect interrupt"]
    #[inline]
    pub fn detect_int_en_0(self) -> &'a mut W {
        self.variant(DETECT_INT_ENW::DETECT_INT_EN_0)
    }
    #[doc = "Enable detect interrupt"]
    #[inline]
    pub fn detect_int_en_1(self) -> &'a mut W {
        self.variant(DETECT_INT_ENW::DETECT_INT_EN_1)
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
#[doc = "Values that can be written to the field `IDLE_SW_INT_EN`"]
pub enum IDLE_SW_INT_ENW {
    #[doc = "Disable idle software interrupt"]
    IDLE_SW_INT_EN_0,
    #[doc = "Enable idle software interrupt"]
    IDLE_SW_INT_EN_1,
}
impl IDLE_SW_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLE_SW_INT_ENW::IDLE_SW_INT_EN_0 => false,
            IDLE_SW_INT_ENW::IDLE_SW_INT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLE_SW_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLE_SW_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLE_SW_INT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable idle software interrupt"]
    #[inline]
    pub fn idle_sw_int_en_0(self) -> &'a mut W {
        self.variant(IDLE_SW_INT_ENW::IDLE_SW_INT_EN_0)
    }
    #[doc = "Enable idle software interrupt"]
    #[inline]
    pub fn idle_sw_int_en_1(self) -> &'a mut W {
        self.variant(IDLE_SW_INT_ENW::IDLE_SW_INT_EN_1)
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
    #[doc = "Bit 0 - Measure Interrupt Enable"]
    #[inline]
    pub fn measure_int_en(&self) -> MEASURE_INT_ENR {
        MEASURE_INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Detect Interrupt Enable"]
    #[inline]
    pub fn detect_int_en(&self) -> DETECT_INT_ENR {
        DETECT_INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Idle Software Interrupt Enable"]
    #[inline]
    pub fn idle_sw_int_en(&self) -> IDLE_SW_INT_ENR {
        IDLE_SW_INT_ENR::_from({
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
    #[doc = "Bit 0 - Measure Interrupt Enable"]
    #[inline]
    pub fn measure_int_en(&mut self) -> _MEASURE_INT_ENW {
        _MEASURE_INT_ENW { w: self }
    }
    #[doc = "Bit 4 - Detect Interrupt Enable"]
    #[inline]
    pub fn detect_int_en(&mut self) -> _DETECT_INT_ENW {
        _DETECT_INT_ENW { w: self }
    }
    #[doc = "Bit 12 - Idle Software Interrupt Enable"]
    #[inline]
    pub fn idle_sw_int_en(&mut self) -> _IDLE_SW_INT_ENW {
        _IDLE_SW_INT_ENW { w: self }
    }
}
