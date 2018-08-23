#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPCR {
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
#[doc = "Possible values of the field `RTC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ENR {
    #[doc = "RTC is disabled"]
    RTC_EN_0,
    #[doc = "RTC is enabled"]
    RTC_EN_1,
}
impl RTC_ENR {
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
            RTC_ENR::RTC_EN_0 => false,
            RTC_ENR::RTC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ENR {
        match value {
            false => RTC_ENR::RTC_EN_0,
            true => RTC_ENR::RTC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_EN_0`"]
    #[inline]
    pub fn is_rtc_en_0(&self) -> bool {
        *self == RTC_ENR::RTC_EN_0
    }
    #[doc = "Checks if the value of the field is `RTC_EN_1`"]
    #[inline]
    pub fn is_rtc_en_1(&self) -> bool {
        *self == RTC_ENR::RTC_EN_1
    }
}
#[doc = "Possible values of the field `HPTA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPTA_ENR {
    #[doc = "HP Time Alarm Interrupt is disabled"]
    HPTA_EN_0,
    #[doc = "HP Time Alarm Interrupt is enabled"]
    HPTA_EN_1,
}
impl HPTA_ENR {
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
            HPTA_ENR::HPTA_EN_0 => false,
            HPTA_ENR::HPTA_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPTA_ENR {
        match value {
            false => HPTA_ENR::HPTA_EN_0,
            true => HPTA_ENR::HPTA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPTA_EN_0`"]
    #[inline]
    pub fn is_hpta_en_0(&self) -> bool {
        *self == HPTA_ENR::HPTA_EN_0
    }
    #[doc = "Checks if the value of the field is `HPTA_EN_1`"]
    #[inline]
    pub fn is_hpta_en_1(&self) -> bool {
        *self == HPTA_ENR::HPTA_EN_1
    }
}
#[doc = "Possible values of the field `PI_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_ENR {
    #[doc = "HP Periodic Interrupt is disabled"]
    PI_EN_0,
    #[doc = "HP Periodic Interrupt is enabled"]
    PI_EN_1,
}
impl PI_ENR {
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
            PI_ENR::PI_EN_0 => false,
            PI_ENR::PI_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PI_ENR {
        match value {
            false => PI_ENR::PI_EN_0,
            true => PI_ENR::PI_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PI_EN_0`"]
    #[inline]
    pub fn is_pi_en_0(&self) -> bool {
        *self == PI_ENR::PI_EN_0
    }
    #[doc = "Checks if the value of the field is `PI_EN_1`"]
    #[inline]
    pub fn is_pi_en_1(&self) -> bool {
        *self == PI_ENR::PI_EN_1
    }
}
#[doc = "Possible values of the field `PI_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_FREQR {
    #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_0,
    #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_1,
    #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_2,
    #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_3,
    #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_4,
    #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_5,
    #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_6,
    #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_7,
    #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_8,
    #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_9,
    #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_10,
    #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_11,
    #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_12,
    #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_13,
    #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_14,
    #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_15,
}
impl PI_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PI_FREQR::PI_FREQ_0 => 0,
            PI_FREQR::PI_FREQ_1 => 1,
            PI_FREQR::PI_FREQ_2 => 2,
            PI_FREQR::PI_FREQ_3 => 3,
            PI_FREQR::PI_FREQ_4 => 4,
            PI_FREQR::PI_FREQ_5 => 5,
            PI_FREQR::PI_FREQ_6 => 6,
            PI_FREQR::PI_FREQ_7 => 7,
            PI_FREQR::PI_FREQ_8 => 8,
            PI_FREQR::PI_FREQ_9 => 9,
            PI_FREQR::PI_FREQ_10 => 10,
            PI_FREQR::PI_FREQ_11 => 11,
            PI_FREQR::PI_FREQ_12 => 12,
            PI_FREQR::PI_FREQ_13 => 13,
            PI_FREQR::PI_FREQ_14 => 14,
            PI_FREQR::PI_FREQ_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PI_FREQR {
        match value {
            0 => PI_FREQR::PI_FREQ_0,
            1 => PI_FREQR::PI_FREQ_1,
            2 => PI_FREQR::PI_FREQ_2,
            3 => PI_FREQR::PI_FREQ_3,
            4 => PI_FREQR::PI_FREQ_4,
            5 => PI_FREQR::PI_FREQ_5,
            6 => PI_FREQR::PI_FREQ_6,
            7 => PI_FREQR::PI_FREQ_7,
            8 => PI_FREQR::PI_FREQ_8,
            9 => PI_FREQR::PI_FREQ_9,
            10 => PI_FREQR::PI_FREQ_10,
            11 => PI_FREQR::PI_FREQ_11,
            12 => PI_FREQR::PI_FREQ_12,
            13 => PI_FREQR::PI_FREQ_13,
            14 => PI_FREQR::PI_FREQ_14,
            15 => PI_FREQR::PI_FREQ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_0`"]
    #[inline]
    pub fn is_pi_freq_0(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_0
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_1`"]
    #[inline]
    pub fn is_pi_freq_1(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_1
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_2`"]
    #[inline]
    pub fn is_pi_freq_2(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_2
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_3`"]
    #[inline]
    pub fn is_pi_freq_3(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_3
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_4`"]
    #[inline]
    pub fn is_pi_freq_4(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_4
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_5`"]
    #[inline]
    pub fn is_pi_freq_5(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_5
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_6`"]
    #[inline]
    pub fn is_pi_freq_6(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_6
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_7`"]
    #[inline]
    pub fn is_pi_freq_7(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_7
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_8`"]
    #[inline]
    pub fn is_pi_freq_8(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_8
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_9`"]
    #[inline]
    pub fn is_pi_freq_9(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_9
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_10`"]
    #[inline]
    pub fn is_pi_freq_10(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_10
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_11`"]
    #[inline]
    pub fn is_pi_freq_11(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_11
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_12`"]
    #[inline]
    pub fn is_pi_freq_12(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_12
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_13`"]
    #[inline]
    pub fn is_pi_freq_13(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_13
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_14`"]
    #[inline]
    pub fn is_pi_freq_14(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_14
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_15`"]
    #[inline]
    pub fn is_pi_freq_15(&self) -> bool {
        *self == PI_FREQR::PI_FREQ_15
    }
}
#[doc = "Possible values of the field `HPCALB_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCALB_ENR {
    #[doc = "HP Timer calibration disabled"]
    HPCALB_EN_0,
    #[doc = "HP Timer calibration enabled"]
    HPCALB_EN_1,
}
impl HPCALB_ENR {
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
            HPCALB_ENR::HPCALB_EN_0 => false,
            HPCALB_ENR::HPCALB_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPCALB_ENR {
        match value {
            false => HPCALB_ENR::HPCALB_EN_0,
            true => HPCALB_ENR::HPCALB_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPCALB_EN_0`"]
    #[inline]
    pub fn is_hpcalb_en_0(&self) -> bool {
        *self == HPCALB_ENR::HPCALB_EN_0
    }
    #[doc = "Checks if the value of the field is `HPCALB_EN_1`"]
    #[inline]
    pub fn is_hpcalb_en_1(&self) -> bool {
        *self == HPCALB_ENR::HPCALB_EN_1
    }
}
#[doc = "Possible values of the field `HPCALB_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCALB_VALR {
    #[doc = "+0 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_0,
    #[doc = "+1 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_1,
    #[doc = "+2 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_2,
    #[doc = "+15 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_15,
    #[doc = "-16 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_16,
    #[doc = "-15 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_17,
    #[doc = "-2 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_30,
    #[doc = "-1 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HPCALB_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPCALB_VALR::HPCALB_VAL_0 => 0,
            HPCALB_VALR::HPCALB_VAL_1 => 1,
            HPCALB_VALR::HPCALB_VAL_2 => 2,
            HPCALB_VALR::HPCALB_VAL_15 => 15,
            HPCALB_VALR::HPCALB_VAL_16 => 16,
            HPCALB_VALR::HPCALB_VAL_17 => 17,
            HPCALB_VALR::HPCALB_VAL_30 => 30,
            HPCALB_VALR::HPCALB_VAL_31 => 31,
            HPCALB_VALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPCALB_VALR {
        match value {
            0 => HPCALB_VALR::HPCALB_VAL_0,
            1 => HPCALB_VALR::HPCALB_VAL_1,
            2 => HPCALB_VALR::HPCALB_VAL_2,
            15 => HPCALB_VALR::HPCALB_VAL_15,
            16 => HPCALB_VALR::HPCALB_VAL_16,
            17 => HPCALB_VALR::HPCALB_VAL_17,
            30 => HPCALB_VALR::HPCALB_VAL_30,
            31 => HPCALB_VALR::HPCALB_VAL_31,
            i => HPCALB_VALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_0`"]
    #[inline]
    pub fn is_hpcalb_val_0(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_0
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_1`"]
    #[inline]
    pub fn is_hpcalb_val_1(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_1
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_2`"]
    #[inline]
    pub fn is_hpcalb_val_2(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_2
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_15`"]
    #[inline]
    pub fn is_hpcalb_val_15(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_15
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_16`"]
    #[inline]
    pub fn is_hpcalb_val_16(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_16
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_17`"]
    #[inline]
    pub fn is_hpcalb_val_17(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_17
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_30`"]
    #[inline]
    pub fn is_hpcalb_val_30(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_30
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_31`"]
    #[inline]
    pub fn is_hpcalb_val_31(&self) -> bool {
        *self == HPCALB_VALR::HPCALB_VAL_31
    }
}
#[doc = "Possible values of the field `HP_TS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_TSR {
    #[doc = "No Action"]
    HP_TS_0,
    #[doc = "Synchronize the HP Time Counter to the LP Time Counter"]
    HP_TS_1,
}
impl HP_TSR {
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
            HP_TSR::HP_TS_0 => false,
            HP_TSR::HP_TS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_TSR {
        match value {
            false => HP_TSR::HP_TS_0,
            true => HP_TSR::HP_TS_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_TS_0`"]
    #[inline]
    pub fn is_hp_ts_0(&self) -> bool {
        *self == HP_TSR::HP_TS_0
    }
    #[doc = "Checks if the value of the field is `HP_TS_1`"]
    #[inline]
    pub fn is_hp_ts_1(&self) -> bool {
        *self == HP_TSR::HP_TS_1
    }
}
#[doc = r" Value of the field"]
pub struct BTN_CONFIGR {
    bits: u8,
}
impl BTN_CONFIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BTN_MASKR {
    bits: bool,
}
impl BTN_MASKR {
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
#[doc = "Values that can be written to the field `RTC_EN`"]
pub enum RTC_ENW {
    #[doc = "RTC is disabled"]
    RTC_EN_0,
    #[doc = "RTC is enabled"]
    RTC_EN_1,
}
impl RTC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ENW::RTC_EN_0 => false,
            RTC_ENW::RTC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC is disabled"]
    #[inline]
    pub fn rtc_en_0(self) -> &'a mut W {
        self.variant(RTC_ENW::RTC_EN_0)
    }
    #[doc = "RTC is enabled"]
    #[inline]
    pub fn rtc_en_1(self) -> &'a mut W {
        self.variant(RTC_ENW::RTC_EN_1)
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
#[doc = "Values that can be written to the field `HPTA_EN`"]
pub enum HPTA_ENW {
    #[doc = "HP Time Alarm Interrupt is disabled"]
    HPTA_EN_0,
    #[doc = "HP Time Alarm Interrupt is enabled"]
    HPTA_EN_1,
}
impl HPTA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPTA_ENW::HPTA_EN_0 => false,
            HPTA_ENW::HPTA_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPTA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HPTA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPTA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HP Time Alarm Interrupt is disabled"]
    #[inline]
    pub fn hpta_en_0(self) -> &'a mut W {
        self.variant(HPTA_ENW::HPTA_EN_0)
    }
    #[doc = "HP Time Alarm Interrupt is enabled"]
    #[inline]
    pub fn hpta_en_1(self) -> &'a mut W {
        self.variant(HPTA_ENW::HPTA_EN_1)
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
#[doc = "Values that can be written to the field `PI_EN`"]
pub enum PI_ENW {
    #[doc = "HP Periodic Interrupt is disabled"]
    PI_EN_0,
    #[doc = "HP Periodic Interrupt is enabled"]
    PI_EN_1,
}
impl PI_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PI_ENW::PI_EN_0 => false,
            PI_ENW::PI_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PI_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PI_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PI_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HP Periodic Interrupt is disabled"]
    #[inline]
    pub fn pi_en_0(self) -> &'a mut W {
        self.variant(PI_ENW::PI_EN_0)
    }
    #[doc = "HP Periodic Interrupt is enabled"]
    #[inline]
    pub fn pi_en_1(self) -> &'a mut W {
        self.variant(PI_ENW::PI_EN_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PI_FREQ`"]
pub enum PI_FREQW {
    #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_0,
    #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_1,
    #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_2,
    #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_3,
    #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_4,
    #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_5,
    #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_6,
    #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_7,
    #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_8,
    #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_9,
    #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_10,
    #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_11,
    #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_12,
    #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_13,
    #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_14,
    #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_15,
}
impl PI_FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PI_FREQW::PI_FREQ_0 => 0,
            PI_FREQW::PI_FREQ_1 => 1,
            PI_FREQW::PI_FREQ_2 => 2,
            PI_FREQW::PI_FREQ_3 => 3,
            PI_FREQW::PI_FREQ_4 => 4,
            PI_FREQW::PI_FREQ_5 => 5,
            PI_FREQW::PI_FREQ_6 => 6,
            PI_FREQW::PI_FREQ_7 => 7,
            PI_FREQW::PI_FREQ_8 => 8,
            PI_FREQW::PI_FREQ_9 => 9,
            PI_FREQW::PI_FREQ_10 => 10,
            PI_FREQW::PI_FREQ_11 => 11,
            PI_FREQW::PI_FREQ_12 => 12,
            PI_FREQW::PI_FREQ_13 => 13,
            PI_FREQW::PI_FREQ_14 => 14,
            PI_FREQW::PI_FREQ_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PI_FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _PI_FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PI_FREQW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_0(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_0)
    }
    #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_1(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_1)
    }
    #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_2(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_2)
    }
    #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_3(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_3)
    }
    #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_4(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_4)
    }
    #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_5(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_5)
    }
    #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_6(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_6)
    }
    #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_7(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_7)
    }
    #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_8(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_8)
    }
    #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_9(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_9)
    }
    #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_10(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_10)
    }
    #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_11(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_11)
    }
    #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_12(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_12)
    }
    #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_13(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_13)
    }
    #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_14(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_14)
    }
    #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline]
    pub fn pi_freq_15(self) -> &'a mut W {
        self.variant(PI_FREQW::PI_FREQ_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPCALB_EN`"]
pub enum HPCALB_ENW {
    #[doc = "HP Timer calibration disabled"]
    HPCALB_EN_0,
    #[doc = "HP Timer calibration enabled"]
    HPCALB_EN_1,
}
impl HPCALB_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPCALB_ENW::HPCALB_EN_0 => false,
            HPCALB_ENW::HPCALB_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPCALB_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HPCALB_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPCALB_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HP Timer calibration disabled"]
    #[inline]
    pub fn hpcalb_en_0(self) -> &'a mut W {
        self.variant(HPCALB_ENW::HPCALB_EN_0)
    }
    #[doc = "HP Timer calibration enabled"]
    #[inline]
    pub fn hpcalb_en_1(self) -> &'a mut W {
        self.variant(HPCALB_ENW::HPCALB_EN_1)
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
#[doc = "Values that can be written to the field `HPCALB_VAL`"]
pub enum HPCALB_VALW {
    #[doc = "+0 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_0,
    #[doc = "+1 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_1,
    #[doc = "+2 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_2,
    #[doc = "+15 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_15,
    #[doc = "-16 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_16,
    #[doc = "-15 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_17,
    #[doc = "-2 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_30,
    #[doc = "-1 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_31,
}
impl HPCALB_VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPCALB_VALW::HPCALB_VAL_0 => 0,
            HPCALB_VALW::HPCALB_VAL_1 => 1,
            HPCALB_VALW::HPCALB_VAL_2 => 2,
            HPCALB_VALW::HPCALB_VAL_15 => 15,
            HPCALB_VALW::HPCALB_VAL_16 => 16,
            HPCALB_VALW::HPCALB_VAL_17 => 17,
            HPCALB_VALW::HPCALB_VAL_30 => 30,
            HPCALB_VALW::HPCALB_VAL_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPCALB_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _HPCALB_VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPCALB_VALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "+0 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_0(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_0)
    }
    #[doc = "+1 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_1(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_1)
    }
    #[doc = "+2 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_2(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_2)
    }
    #[doc = "+15 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_15(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_15)
    }
    #[doc = "-16 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_16(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_16)
    }
    #[doc = "-15 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_17(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_17)
    }
    #[doc = "-2 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_30(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_30)
    }
    #[doc = "-1 counts per each 32768 ticks of the counter"]
    #[inline]
    pub fn hpcalb_val_31(self) -> &'a mut W {
        self.variant(HPCALB_VALW::HPCALB_VAL_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HP_TS`"]
pub enum HP_TSW {
    #[doc = "No Action"]
    HP_TS_0,
    #[doc = "Synchronize the HP Time Counter to the LP Time Counter"]
    HP_TS_1,
}
impl HP_TSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_TSW::HP_TS_0 => false,
            HP_TSW::HP_TS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_TSW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_TSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_TSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn hp_ts_0(self) -> &'a mut W {
        self.variant(HP_TSW::HP_TS_0)
    }
    #[doc = "Synchronize the HP Time Counter to the LP Time Counter"]
    #[inline]
    pub fn hp_ts_1(self) -> &'a mut W {
        self.variant(HP_TSW::HP_TS_1)
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
#[doc = r" Proxy"]
pub struct _BTN_CONFIGW<'a> {
    w: &'a mut W,
}
impl<'a> _BTN_CONFIGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BTN_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _BTN_MASKW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - HP Real Time Counter Enable"]
    #[inline]
    pub fn rtc_en(&self) -> RTC_ENR {
        RTC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline]
    pub fn hpta_en(&self) -> HPTA_ENR {
        HPTA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline]
    pub fn pi_en(&self) -> PI_ENR {
        PI_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline]
    pub fn pi_freq(&self) -> PI_FREQR {
        PI_FREQR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline]
    pub fn hpcalb_en(&self) -> HPCALB_ENR {
        HPCALB_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:14 - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline]
    pub fn hpcalb_val(&self) -> HPCALB_VALR {
        HPCALB_VALR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - HP Time Synchronize"]
    #[inline]
    pub fn hp_ts(&self) -> HP_TSR {
        HP_TSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:26 - Button Configuration"]
    #[inline]
    pub fn btn_config(&self) -> BTN_CONFIGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BTN_CONFIGR { bits }
    }
    #[doc = "Bit 27 - Button interrupt mask"]
    #[inline]
    pub fn btn_mask(&self) -> BTN_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BTN_MASKR { bits }
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
    #[doc = "Bit 0 - HP Real Time Counter Enable"]
    #[inline]
    pub fn rtc_en(&mut self) -> _RTC_ENW {
        _RTC_ENW { w: self }
    }
    #[doc = "Bit 1 - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline]
    pub fn hpta_en(&mut self) -> _HPTA_ENW {
        _HPTA_ENW { w: self }
    }
    #[doc = "Bit 3 - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline]
    pub fn pi_en(&mut self) -> _PI_ENW {
        _PI_ENW { w: self }
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline]
    pub fn pi_freq(&mut self) -> _PI_FREQW {
        _PI_FREQW { w: self }
    }
    #[doc = "Bit 8 - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline]
    pub fn hpcalb_en(&mut self) -> _HPCALB_ENW {
        _HPCALB_ENW { w: self }
    }
    #[doc = "Bits 10:14 - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline]
    pub fn hpcalb_val(&mut self) -> _HPCALB_VALW {
        _HPCALB_VALW { w: self }
    }
    #[doc = "Bit 16 - HP Time Synchronize"]
    #[inline]
    pub fn hp_ts(&mut self) -> _HP_TSW {
        _HP_TSW { w: self }
    }
    #[doc = "Bits 24:26 - Button Configuration"]
    #[inline]
    pub fn btn_config(&mut self) -> _BTN_CONFIGW {
        _BTN_CONFIGW { w: self }
    }
    #[doc = "Bit 27 - Button interrupt mask"]
    #[inline]
    pub fn btn_mask(&mut self) -> _BTN_MASKW {
        _BTN_MASKW { w: self }
    }
}
