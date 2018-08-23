#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLOW_CONTROL {
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
pub struct SW_RSTR {
    bits: bool,
}
impl SW_RSTR {
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
#[doc = "Possible values of the field `START_MEASURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_MEASURER {
    #[doc = "Do not start measure for now"]
    START_MEASURE_0,
    #[doc = "Start measure the X/Y coordinate value"]
    START_MEASURE_1,
}
impl START_MEASURER {
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
            START_MEASURER::START_MEASURE_0 => false,
            START_MEASURER::START_MEASURE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> START_MEASURER {
        match value {
            false => START_MEASURER::START_MEASURE_0,
            true => START_MEASURER::START_MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `START_MEASURE_0`"]
    #[inline]
    pub fn is_start_measure_0(&self) -> bool {
        *self == START_MEASURER::START_MEASURE_0
    }
    #[doc = "Checks if the value of the field is `START_MEASURE_1`"]
    #[inline]
    pub fn is_start_measure_1(&self) -> bool {
        *self == START_MEASURER::START_MEASURE_1
    }
}
#[doc = "Possible values of the field `DROP_MEASURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DROP_MEASURER {
    #[doc = "Do not drop measure for now"]
    DROP_MEASURE_0,
    #[doc = "Drop the measure and controller return to idle status"]
    DROP_MEASURE_1,
}
impl DROP_MEASURER {
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
            DROP_MEASURER::DROP_MEASURE_0 => false,
            DROP_MEASURER::DROP_MEASURE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DROP_MEASURER {
        match value {
            false => DROP_MEASURER::DROP_MEASURE_0,
            true => DROP_MEASURER::DROP_MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DROP_MEASURE_0`"]
    #[inline]
    pub fn is_drop_measure_0(&self) -> bool {
        *self == DROP_MEASURER::DROP_MEASURE_0
    }
    #[doc = "Checks if the value of the field is `DROP_MEASURE_1`"]
    #[inline]
    pub fn is_drop_measure_1(&self) -> bool {
        *self == DROP_MEASURER::DROP_MEASURE_1
    }
}
#[doc = "Possible values of the field `START_SENSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_SENSER {
    #[doc = "Stay at idle status"]
    START_SENSE_0,
    #[doc = "Start sense detection and (if auto_measure set to 1) measure after detect a touch"]
    START_SENSE_1,
}
impl START_SENSER {
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
            START_SENSER::START_SENSE_0 => false,
            START_SENSER::START_SENSE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> START_SENSER {
        match value {
            false => START_SENSER::START_SENSE_0,
            true => START_SENSER::START_SENSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `START_SENSE_0`"]
    #[inline]
    pub fn is_start_sense_0(&self) -> bool {
        *self == START_SENSER::START_SENSE_0
    }
    #[doc = "Checks if the value of the field is `START_SENSE_1`"]
    #[inline]
    pub fn is_start_sense_1(&self) -> bool {
        *self == START_SENSER::START_SENSE_1
    }
}
#[doc = "Possible values of the field `DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLER {
    #[doc = "Leave HW state machine control"]
    DISABLE_0,
    #[doc = "SW set to idle status"]
    DISABLE_1,
}
impl DISABLER {
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
            DISABLER::DISABLE_0 => false,
            DISABLER::DISABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLER {
        match value {
            false => DISABLER::DISABLE_0,
            true => DISABLER::DISABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_0`"]
    #[inline]
    pub fn is_disable_0(&self) -> bool {
        *self == DISABLER::DISABLE_0
    }
    #[doc = "Checks if the value of the field is `DISABLE_1`"]
    #[inline]
    pub fn is_disable_1(&self) -> bool {
        *self == DISABLER::DISABLE_1
    }
}
#[doc = r" Proxy"]
pub struct _SW_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_RSTW<'a> {
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
#[doc = "Values that can be written to the field `START_MEASURE`"]
pub enum START_MEASUREW {
    #[doc = "Do not start measure for now"]
    START_MEASURE_0,
    #[doc = "Start measure the X/Y coordinate value"]
    START_MEASURE_1,
}
impl START_MEASUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            START_MEASUREW::START_MEASURE_0 => false,
            START_MEASUREW::START_MEASURE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _START_MEASUREW<'a> {
    w: &'a mut W,
}
impl<'a> _START_MEASUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: START_MEASUREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not start measure for now"]
    #[inline]
    pub fn start_measure_0(self) -> &'a mut W {
        self.variant(START_MEASUREW::START_MEASURE_0)
    }
    #[doc = "Start measure the X/Y coordinate value"]
    #[inline]
    pub fn start_measure_1(self) -> &'a mut W {
        self.variant(START_MEASUREW::START_MEASURE_1)
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
#[doc = "Values that can be written to the field `DROP_MEASURE`"]
pub enum DROP_MEASUREW {
    #[doc = "Do not drop measure for now"]
    DROP_MEASURE_0,
    #[doc = "Drop the measure and controller return to idle status"]
    DROP_MEASURE_1,
}
impl DROP_MEASUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DROP_MEASUREW::DROP_MEASURE_0 => false,
            DROP_MEASUREW::DROP_MEASURE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DROP_MEASUREW<'a> {
    w: &'a mut W,
}
impl<'a> _DROP_MEASUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DROP_MEASUREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not drop measure for now"]
    #[inline]
    pub fn drop_measure_0(self) -> &'a mut W {
        self.variant(DROP_MEASUREW::DROP_MEASURE_0)
    }
    #[doc = "Drop the measure and controller return to idle status"]
    #[inline]
    pub fn drop_measure_1(self) -> &'a mut W {
        self.variant(DROP_MEASUREW::DROP_MEASURE_1)
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
#[doc = "Values that can be written to the field `START_SENSE`"]
pub enum START_SENSEW {
    #[doc = "Stay at idle status"]
    START_SENSE_0,
    #[doc = "Start sense detection and (if auto_measure set to 1) measure after detect a touch"]
    START_SENSE_1,
}
impl START_SENSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            START_SENSEW::START_SENSE_0 => false,
            START_SENSEW::START_SENSE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _START_SENSEW<'a> {
    w: &'a mut W,
}
impl<'a> _START_SENSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: START_SENSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stay at idle status"]
    #[inline]
    pub fn start_sense_0(self) -> &'a mut W {
        self.variant(START_SENSEW::START_SENSE_0)
    }
    #[doc = "Start sense detection and (if auto_measure set to 1) measure after detect a touch"]
    #[inline]
    pub fn start_sense_1(self) -> &'a mut W {
        self.variant(START_SENSEW::START_SENSE_1)
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
#[doc = "Values that can be written to the field `DISABLE`"]
pub enum DISABLEW {
    #[doc = "Leave HW state machine control"]
    DISABLE_0,
    #[doc = "SW set to idle status"]
    DISABLE_1,
}
impl DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISABLEW::DISABLE_0 => false,
            DISABLEW::DISABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Leave HW state machine control"]
    #[inline]
    pub fn disable_0(self) -> &'a mut W {
        self.variant(DISABLEW::DISABLE_0)
    }
    #[doc = "SW set to idle status"]
    #[inline]
    pub fn disable_1(self) -> &'a mut W {
        self.variant(DISABLEW::DISABLE_1)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Soft Reset"]
    #[inline]
    pub fn sw_rst(&self) -> SW_RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_RSTR { bits }
    }
    #[doc = "Bit 4 - Start Measure"]
    #[inline]
    pub fn start_measure(&self) -> START_MEASURER {
        START_MEASURER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Drop Measure"]
    #[inline]
    pub fn drop_measure(&self) -> DROP_MEASURER {
        DROP_MEASURER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Start Sense"]
    #[inline]
    pub fn start_sense(&self) -> START_SENSER {
        START_SENSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - This bit is for SW disable registers"]
    #[inline]
    pub fn disable(&self) -> DISABLER {
        DISABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Soft Reset"]
    #[inline]
    pub fn sw_rst(&mut self) -> _SW_RSTW {
        _SW_RSTW { w: self }
    }
    #[doc = "Bit 4 - Start Measure"]
    #[inline]
    pub fn start_measure(&mut self) -> _START_MEASUREW {
        _START_MEASUREW { w: self }
    }
    #[doc = "Bit 8 - Drop Measure"]
    #[inline]
    pub fn drop_measure(&mut self) -> _DROP_MEASUREW {
        _DROP_MEASUREW { w: self }
    }
    #[doc = "Bit 12 - Start Sense"]
    #[inline]
    pub fn start_sense(&mut self) -> _START_SENSEW {
        _START_SENSEW { w: self }
    }
    #[doc = "Bit 16 - This bit is for SW disable registers"]
    #[inline]
    pub fn disable(&mut self) -> _DISABLEW {
        _DISABLEW { w: self }
    }
}
