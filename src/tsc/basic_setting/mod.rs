#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BASIC_SETTING {
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
#[doc = "Possible values of the field `AUTO_MEASURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_MEASURER {
    #[doc = "Disable Auto Measure"]
    AUTO_MEASURE_0,
    #[doc = "Auto Measure"]
    AUTO_MEASURE_1,
}
impl AUTO_MEASURER {
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
            AUTO_MEASURER::AUTO_MEASURE_0 => false,
            AUTO_MEASURER::AUTO_MEASURE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTO_MEASURER {
        match value {
            false => AUTO_MEASURER::AUTO_MEASURE_0,
            true => AUTO_MEASURER::AUTO_MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_MEASURE_0`"]
    #[inline]
    pub fn is_auto_measure_0(&self) -> bool {
        *self == AUTO_MEASURER::AUTO_MEASURE_0
    }
    #[doc = "Checks if the value of the field is `AUTO_MEASURE_1`"]
    #[inline]
    pub fn is_auto_measure_1(&self) -> bool {
        *self == AUTO_MEASURER::AUTO_MEASURE_1
    }
}
#[doc = "Possible values of the field `_4_5_WIRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum _4_5_WIRER {
    #[doc = "4-Wire Detection Mode"]
    _4_5_WIRE_0,
    #[doc = "5-Wire Detection Mode"]
    _4_5_WIRE_1,
}
impl _4_5_WIRER {
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
            _4_5_WIRER::_4_5_WIRE_0 => false,
            _4_5_WIRER::_4_5_WIRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> _4_5_WIRER {
        match value {
            false => _4_5_WIRER::_4_5_WIRE_0,
            true => _4_5_WIRER::_4_5_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `_4_5_WIRE_0`"]
    #[inline]
    pub fn is_4_5_wire_0(&self) -> bool {
        *self == _4_5_WIRER::_4_5_WIRE_0
    }
    #[doc = "Checks if the value of the field is `_4_5_WIRE_1`"]
    #[inline]
    pub fn is_4_5_wire_1(&self) -> bool {
        *self == _4_5_WIRER::_4_5_WIRE_1
    }
}
#[doc = r" Value of the field"]
pub struct MEASURE_DELAY_TIMER {
    bits: u32,
}
impl MEASURE_DELAY_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `AUTO_MEASURE`"]
pub enum AUTO_MEASUREW {
    #[doc = "Disable Auto Measure"]
    AUTO_MEASURE_0,
    #[doc = "Auto Measure"]
    AUTO_MEASURE_1,
}
impl AUTO_MEASUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTO_MEASUREW::AUTO_MEASURE_0 => false,
            AUTO_MEASUREW::AUTO_MEASURE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_MEASUREW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_MEASUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTO_MEASUREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Auto Measure"]
    #[inline]
    pub fn auto_measure_0(self) -> &'a mut W {
        self.variant(AUTO_MEASUREW::AUTO_MEASURE_0)
    }
    #[doc = "Auto Measure"]
    #[inline]
    pub fn auto_measure_1(self) -> &'a mut W {
        self.variant(AUTO_MEASUREW::AUTO_MEASURE_1)
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
#[doc = "Values that can be written to the field `_4_5_WIRE`"]
pub enum _4_5_WIREW {
    #[doc = "4-Wire Detection Mode"]
    _4_5_WIRE_0,
    #[doc = "5-Wire Detection Mode"]
    _4_5_WIRE_1,
}
impl _4_5_WIREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            _4_5_WIREW::_4_5_WIRE_0 => false,
            _4_5_WIREW::_4_5_WIRE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct __4_5_WIREW<'a> {
    w: &'a mut W,
}
impl<'a> __4_5_WIREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _4_5_WIREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-Wire Detection Mode"]
    #[inline]
    pub fn _4_5_wire_0(self) -> &'a mut W {
        self.variant(_4_5_WIREW::_4_5_WIRE_0)
    }
    #[doc = "5-Wire Detection Mode"]
    #[inline]
    pub fn _4_5_wire_1(self) -> &'a mut W {
        self.variant(_4_5_WIREW::_4_5_WIRE_1)
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
#[doc = r" Proxy"]
pub struct _MEASURE_DELAY_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _MEASURE_DELAY_TIMEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Auto Measure"]
    #[inline]
    pub fn auto_measure(&self) -> AUTO_MEASURER {
        AUTO_MEASURER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4/5 Wire detection"]
    #[inline]
    pub fn _4_5_wire(&self) -> _4_5_WIRER {
        _4_5_WIRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:31 - Measure Delay Time"]
    #[inline]
    pub fn measure_delay_time(&self) -> MEASURE_DELAY_TIMER {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        MEASURE_DELAY_TIMER { bits }
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
    #[doc = "Bit 0 - Auto Measure"]
    #[inline]
    pub fn auto_measure(&mut self) -> _AUTO_MEASUREW {
        _AUTO_MEASUREW { w: self }
    }
    #[doc = "Bit 4 - 4/5 Wire detection"]
    #[inline]
    pub fn _4_5_wire(&mut self) -> __4_5_WIREW {
        __4_5_WIREW { w: self }
    }
    #[doc = "Bits 8:31 - Measure Delay Time"]
    #[inline]
    pub fn measure_delay_time(&mut self) -> _MEASURE_DELAY_TIMEW {
        _MEASURE_DELAY_TIMEW { w: self }
    }
}
