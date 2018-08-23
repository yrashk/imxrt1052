#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TEMPSENSE0_CLR {
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
#[doc = "Possible values of the field `POWER_DOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POWER_DOWNR {
    #[doc = "Enable power to the temperature sensor."]
    POWER_UP,
    #[doc = "Power down the temperature sensor."]
    POWER_DOWN,
}
impl POWER_DOWNR {
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
            POWER_DOWNR::POWER_UP => false,
            POWER_DOWNR::POWER_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POWER_DOWNR {
        match value {
            false => POWER_DOWNR::POWER_UP,
            true => POWER_DOWNR::POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline]
    pub fn is_power_up(&self) -> bool {
        *self == POWER_DOWNR::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline]
    pub fn is_power_down(&self) -> bool {
        *self == POWER_DOWNR::POWER_DOWN
    }
}
#[doc = "Possible values of the field `MEASURE_TEMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEASURE_TEMPR {
    #[doc = "Do not start the measurement process."]
    STOP,
    #[doc = "Start the measurement process."]
    START,
}
impl MEASURE_TEMPR {
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
            MEASURE_TEMPR::STOP => false,
            MEASURE_TEMPR::START => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEASURE_TEMPR {
        match value {
            false => MEASURE_TEMPR::STOP,
            true => MEASURE_TEMPR::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == MEASURE_TEMPR::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == MEASURE_TEMPR::START
    }
}
#[doc = "Possible values of the field `FINISHED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISHEDR {
    #[doc = "Last measurement is not ready yet."]
    INVALID,
    #[doc = "Last measurement is valid."]
    VALID,
}
impl FINISHEDR {
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
            FINISHEDR::INVALID => false,
            FINISHEDR::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FINISHEDR {
        match value {
            false => FINISHEDR::INVALID,
            true => FINISHEDR::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline]
    pub fn is_invalid(&self) -> bool {
        *self == FINISHEDR::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == FINISHEDR::VALID
    }
}
#[doc = r" Value of the field"]
pub struct TEMP_CNTR {
    bits: u16,
}
impl TEMP_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ALARM_VALUER {
    bits: u16,
}
impl ALARM_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `POWER_DOWN`"]
pub enum POWER_DOWNW {
    #[doc = "Enable power to the temperature sensor."]
    POWER_UP,
    #[doc = "Power down the temperature sensor."]
    POWER_DOWN,
}
impl POWER_DOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POWER_DOWNW::POWER_UP => false,
            POWER_DOWNW::POWER_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POWER_DOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _POWER_DOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POWER_DOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable power to the temperature sensor."]
    #[inline]
    pub fn power_up(self) -> &'a mut W {
        self.variant(POWER_DOWNW::POWER_UP)
    }
    #[doc = "Power down the temperature sensor."]
    #[inline]
    pub fn power_down(self) -> &'a mut W {
        self.variant(POWER_DOWNW::POWER_DOWN)
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
#[doc = "Values that can be written to the field `MEASURE_TEMP`"]
pub enum MEASURE_TEMPW {
    #[doc = "Do not start the measurement process."]
    STOP,
    #[doc = "Start the measurement process."]
    START,
}
impl MEASURE_TEMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEASURE_TEMPW::STOP => false,
            MEASURE_TEMPW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEASURE_TEMPW<'a> {
    w: &'a mut W,
}
impl<'a> _MEASURE_TEMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEASURE_TEMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not start the measurement process."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(MEASURE_TEMPW::STOP)
    }
    #[doc = "Start the measurement process."]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(MEASURE_TEMPW::START)
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
#[doc = r" Proxy"]
pub struct _ALARM_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _ALARM_VALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - This bit powers down the temperature sensor."]
    #[inline]
    pub fn power_down(&self) -> POWER_DOWNR {
        POWER_DOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Starts the measurement process"]
    #[inline]
    pub fn measure_temp(&self) -> MEASURE_TEMPR {
        MEASURE_TEMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Indicates that the latest temp is valid"]
    #[inline]
    pub fn finished(&self) -> FINISHEDR {
        FINISHEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:19 - This bit field contains the last measured temperature count."]
    #[inline]
    pub fn temp_cnt(&self) -> TEMP_CNTR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TEMP_CNTR { bits }
    }
    #[doc = "Bits 20:31 - This bit field contains the temperature count (raw sensor output) that will generate an alarm interrupt"]
    #[inline]
    pub fn alarm_value(&self) -> ALARM_VALUER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ALARM_VALUER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - This bit powers down the temperature sensor."]
    #[inline]
    pub fn power_down(&mut self) -> _POWER_DOWNW {
        _POWER_DOWNW { w: self }
    }
    #[doc = "Bit 1 - Starts the measurement process"]
    #[inline]
    pub fn measure_temp(&mut self) -> _MEASURE_TEMPW {
        _MEASURE_TEMPW { w: self }
    }
    #[doc = "Bits 20:31 - This bit field contains the temperature count (raw sensor output) that will generate an alarm interrupt"]
    #[inline]
    pub fn alarm_value(&mut self) -> _ALARM_VALUEW {
        _ALARM_VALUEW { w: self }
    }
}
