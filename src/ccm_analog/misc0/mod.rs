#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC0 {
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
pub struct REFTOP_PWDR {
    bits: bool,
}
impl REFTOP_PWDR {
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
#[doc = "Possible values of the field `REFTOP_SELFBIASOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFTOP_SELFBIASOFFR {
    #[doc = "Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0,
    #[doc = "Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1,
}
impl REFTOP_SELFBIASOFFR {
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
            REFTOP_SELFBIASOFFR::REFTOP_SELFBIASOFF_0 => false,
            REFTOP_SELFBIASOFFR::REFTOP_SELFBIASOFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFTOP_SELFBIASOFFR {
        match value {
            false => REFTOP_SELFBIASOFFR::REFTOP_SELFBIASOFF_0,
            true => REFTOP_SELFBIASOFFR::REFTOP_SELFBIASOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFTOP_SELFBIASOFF_0`"]
    #[inline]
    pub fn is_reftop_selfbiasoff_0(&self) -> bool {
        *self == REFTOP_SELFBIASOFFR::REFTOP_SELFBIASOFF_0
    }
    #[doc = "Checks if the value of the field is `REFTOP_SELFBIASOFF_1`"]
    #[inline]
    pub fn is_reftop_selfbiasoff_1(&self) -> bool {
        *self == REFTOP_SELFBIASOFFR::REFTOP_SELFBIASOFF_1
    }
}
#[doc = "Possible values of the field `REFTOP_VBGADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFTOP_VBGADJR {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7,
}
impl REFTOP_VBGADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFTOP_VBGADJR::REFTOP_VBGADJ_0 => 0,
            REFTOP_VBGADJR::REFTOP_VBGADJ_1 => 1,
            REFTOP_VBGADJR::REFTOP_VBGADJ_2 => 2,
            REFTOP_VBGADJR::REFTOP_VBGADJ_3 => 3,
            REFTOP_VBGADJR::REFTOP_VBGADJ_4 => 4,
            REFTOP_VBGADJR::REFTOP_VBGADJ_5 => 5,
            REFTOP_VBGADJR::REFTOP_VBGADJ_6 => 6,
            REFTOP_VBGADJR::REFTOP_VBGADJ_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFTOP_VBGADJR {
        match value {
            0 => REFTOP_VBGADJR::REFTOP_VBGADJ_0,
            1 => REFTOP_VBGADJR::REFTOP_VBGADJ_1,
            2 => REFTOP_VBGADJR::REFTOP_VBGADJ_2,
            3 => REFTOP_VBGADJR::REFTOP_VBGADJ_3,
            4 => REFTOP_VBGADJR::REFTOP_VBGADJ_4,
            5 => REFTOP_VBGADJR::REFTOP_VBGADJ_5,
            6 => REFTOP_VBGADJR::REFTOP_VBGADJ_6,
            7 => REFTOP_VBGADJR::REFTOP_VBGADJ_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_0`"]
    #[inline]
    pub fn is_reftop_vbgadj_0(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_0
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_1`"]
    #[inline]
    pub fn is_reftop_vbgadj_1(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_1
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_2`"]
    #[inline]
    pub fn is_reftop_vbgadj_2(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_2
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_3`"]
    #[inline]
    pub fn is_reftop_vbgadj_3(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_3
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_4`"]
    #[inline]
    pub fn is_reftop_vbgadj_4(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_4
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_5`"]
    #[inline]
    pub fn is_reftop_vbgadj_5(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_5
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_6`"]
    #[inline]
    pub fn is_reftop_vbgadj_6(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_6
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_7`"]
    #[inline]
    pub fn is_reftop_vbgadj_7(&self) -> bool {
        *self == REFTOP_VBGADJR::REFTOP_VBGADJ_7
    }
}
#[doc = r" Value of the field"]
pub struct REFTOP_VBGUPR {
    bits: bool,
}
impl REFTOP_VBGUPR {
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
#[doc = "Possible values of the field `STOP_MODE_CONFIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_MODE_CONFIGR {
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
    STOP_MODE_CONFIG_3,
}
impl STOP_MODE_CONFIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOP_MODE_CONFIGR::STOP_MODE_CONFIG_0 => 0,
            STOP_MODE_CONFIGR::STOP_MODE_CONFIG_1 => 1,
            STOP_MODE_CONFIGR::STOP_MODE_CONFIG_2 => 2,
            STOP_MODE_CONFIGR::STOP_MODE_CONFIG_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOP_MODE_CONFIGR {
        match value {
            0 => STOP_MODE_CONFIGR::STOP_MODE_CONFIG_0,
            1 => STOP_MODE_CONFIGR::STOP_MODE_CONFIG_1,
            2 => STOP_MODE_CONFIGR::STOP_MODE_CONFIG_2,
            3 => STOP_MODE_CONFIGR::STOP_MODE_CONFIG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_0`"]
    #[inline]
    pub fn is_stop_mode_config_0(&self) -> bool {
        *self == STOP_MODE_CONFIGR::STOP_MODE_CONFIG_0
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_1`"]
    #[inline]
    pub fn is_stop_mode_config_1(&self) -> bool {
        *self == STOP_MODE_CONFIGR::STOP_MODE_CONFIG_1
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_2`"]
    #[inline]
    pub fn is_stop_mode_config_2(&self) -> bool {
        *self == STOP_MODE_CONFIGR::STOP_MODE_CONFIG_2
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_3`"]
    #[inline]
    pub fn is_stop_mode_config_3(&self) -> bool {
        *self == STOP_MODE_CONFIGR::STOP_MODE_CONFIG_3
    }
}
#[doc = "Possible values of the field `DISCON_HIGH_SNVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCON_HIGH_SNVSR {
    #[doc = "Turn on the switch"]
    DISCON_HIGH_SNVS_0,
    #[doc = "Turn off the switch"]
    DISCON_HIGH_SNVS_1,
}
impl DISCON_HIGH_SNVSR {
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
            DISCON_HIGH_SNVSR::DISCON_HIGH_SNVS_0 => false,
            DISCON_HIGH_SNVSR::DISCON_HIGH_SNVS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISCON_HIGH_SNVSR {
        match value {
            false => DISCON_HIGH_SNVSR::DISCON_HIGH_SNVS_0,
            true => DISCON_HIGH_SNVSR::DISCON_HIGH_SNVS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCON_HIGH_SNVS_0`"]
    #[inline]
    pub fn is_discon_high_snvs_0(&self) -> bool {
        *self == DISCON_HIGH_SNVSR::DISCON_HIGH_SNVS_0
    }
    #[doc = "Checks if the value of the field is `DISCON_HIGH_SNVS_1`"]
    #[inline]
    pub fn is_discon_high_snvs_1(&self) -> bool {
        *self == DISCON_HIGH_SNVSR::DISCON_HIGH_SNVS_1
    }
}
#[doc = "Possible values of the field `OSC_I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_IR {
    #[doc = "Nominal"]
    NOMINAL,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT,
}
impl OSC_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSC_IR::NOMINAL => 0,
            OSC_IR::MINUS_12_5_PERCENT => 1,
            OSC_IR::MINUS_25_PERCENT => 2,
            OSC_IR::MINUS_37_5_PERCENT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSC_IR {
        match value {
            0 => OSC_IR::NOMINAL,
            1 => OSC_IR::MINUS_12_5_PERCENT,
            2 => OSC_IR::MINUS_25_PERCENT,
            3 => OSC_IR::MINUS_37_5_PERCENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOMINAL`"]
    #[inline]
    pub fn is_nominal(&self) -> bool {
        *self == OSC_IR::NOMINAL
    }
    #[doc = "Checks if the value of the field is `MINUS_12_5_PERCENT`"]
    #[inline]
    pub fn is_minus_12_5_percent(&self) -> bool {
        *self == OSC_IR::MINUS_12_5_PERCENT
    }
    #[doc = "Checks if the value of the field is `MINUS_25_PERCENT`"]
    #[inline]
    pub fn is_minus_25_percent(&self) -> bool {
        *self == OSC_IR::MINUS_25_PERCENT
    }
    #[doc = "Checks if the value of the field is `MINUS_37_5_PERCENT`"]
    #[inline]
    pub fn is_minus_37_5_percent(&self) -> bool {
        *self == OSC_IR::MINUS_37_5_PERCENT
    }
}
#[doc = r" Value of the field"]
pub struct OSC_XTALOKR {
    bits: bool,
}
impl OSC_XTALOKR {
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
pub struct OSC_XTALOK_ENR {
    bits: bool,
}
impl OSC_XTALOK_ENR {
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
#[doc = "Possible values of the field `CLKGATE_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGATE_CTRLR {
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE,
    #[doc = "Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE,
}
impl CLKGATE_CTRLR {
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
            CLKGATE_CTRLR::ALLOW_AUTO_GATE => false,
            CLKGATE_CTRLR::NO_AUTO_GATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKGATE_CTRLR {
        match value {
            false => CLKGATE_CTRLR::ALLOW_AUTO_GATE,
            true => CLKGATE_CTRLR::NO_AUTO_GATE,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOW_AUTO_GATE`"]
    #[inline]
    pub fn is_allow_auto_gate(&self) -> bool {
        *self == CLKGATE_CTRLR::ALLOW_AUTO_GATE
    }
    #[doc = "Checks if the value of the field is `NO_AUTO_GATE`"]
    #[inline]
    pub fn is_no_auto_gate(&self) -> bool {
        *self == CLKGATE_CTRLR::NO_AUTO_GATE
    }
}
#[doc = "Possible values of the field `CLKGATE_DELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGATE_DELAYR {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7,
}
impl CLKGATE_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKGATE_DELAYR::CLKGATE_DELAY_0 => 0,
            CLKGATE_DELAYR::CLKGATE_DELAY_1 => 1,
            CLKGATE_DELAYR::CLKGATE_DELAY_2 => 2,
            CLKGATE_DELAYR::CLKGATE_DELAY_3 => 3,
            CLKGATE_DELAYR::CLKGATE_DELAY_4 => 4,
            CLKGATE_DELAYR::CLKGATE_DELAY_5 => 5,
            CLKGATE_DELAYR::CLKGATE_DELAY_6 => 6,
            CLKGATE_DELAYR::CLKGATE_DELAY_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKGATE_DELAYR {
        match value {
            0 => CLKGATE_DELAYR::CLKGATE_DELAY_0,
            1 => CLKGATE_DELAYR::CLKGATE_DELAY_1,
            2 => CLKGATE_DELAYR::CLKGATE_DELAY_2,
            3 => CLKGATE_DELAYR::CLKGATE_DELAY_3,
            4 => CLKGATE_DELAYR::CLKGATE_DELAY_4,
            5 => CLKGATE_DELAYR::CLKGATE_DELAY_5,
            6 => CLKGATE_DELAYR::CLKGATE_DELAY_6,
            7 => CLKGATE_DELAYR::CLKGATE_DELAY_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_0`"]
    #[inline]
    pub fn is_clkgate_delay_0(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_0
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_1`"]
    #[inline]
    pub fn is_clkgate_delay_1(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_1
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_2`"]
    #[inline]
    pub fn is_clkgate_delay_2(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_2
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_3`"]
    #[inline]
    pub fn is_clkgate_delay_3(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_3
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_4`"]
    #[inline]
    pub fn is_clkgate_delay_4(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_4
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_5`"]
    #[inline]
    pub fn is_clkgate_delay_5(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_5
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_6`"]
    #[inline]
    pub fn is_clkgate_delay_6(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_6
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_7`"]
    #[inline]
    pub fn is_clkgate_delay_7(&self) -> bool {
        *self == CLKGATE_DELAYR::CLKGATE_DELAY_7
    }
}
#[doc = "Possible values of the field `RTC_XTAL_SOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_XTAL_SOURCER {
    #[doc = "Internal ring oscillator"]
    RTC_XTAL_SOURCE_0,
    #[doc = "RTC_XTAL"]
    RTC_XTAL_SOURCE_1,
}
impl RTC_XTAL_SOURCER {
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
            RTC_XTAL_SOURCER::RTC_XTAL_SOURCE_0 => false,
            RTC_XTAL_SOURCER::RTC_XTAL_SOURCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_XTAL_SOURCER {
        match value {
            false => RTC_XTAL_SOURCER::RTC_XTAL_SOURCE_0,
            true => RTC_XTAL_SOURCER::RTC_XTAL_SOURCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_XTAL_SOURCE_0`"]
    #[inline]
    pub fn is_rtc_xtal_source_0(&self) -> bool {
        *self == RTC_XTAL_SOURCER::RTC_XTAL_SOURCE_0
    }
    #[doc = "Checks if the value of the field is `RTC_XTAL_SOURCE_1`"]
    #[inline]
    pub fn is_rtc_xtal_source_1(&self) -> bool {
        *self == RTC_XTAL_SOURCER::RTC_XTAL_SOURCE_1
    }
}
#[doc = r" Value of the field"]
pub struct XTAL_24M_PWDR {
    bits: bool,
}
impl XTAL_24M_PWDR {
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
#[doc = "Possible values of the field `VID_PLL_PREDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VID_PLL_PREDIVR {
    #[doc = "Divide by 1"]
    VID_PLL_PREDIV_0,
    #[doc = "Divide by 2"]
    VID_PLL_PREDIV_1,
}
impl VID_PLL_PREDIVR {
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
            VID_PLL_PREDIVR::VID_PLL_PREDIV_0 => false,
            VID_PLL_PREDIVR::VID_PLL_PREDIV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VID_PLL_PREDIVR {
        match value {
            false => VID_PLL_PREDIVR::VID_PLL_PREDIV_0,
            true => VID_PLL_PREDIVR::VID_PLL_PREDIV_1,
        }
    }
    #[doc = "Checks if the value of the field is `VID_PLL_PREDIV_0`"]
    #[inline]
    pub fn is_vid_pll_prediv_0(&self) -> bool {
        *self == VID_PLL_PREDIVR::VID_PLL_PREDIV_0
    }
    #[doc = "Checks if the value of the field is `VID_PLL_PREDIV_1`"]
    #[inline]
    pub fn is_vid_pll_prediv_1(&self) -> bool {
        *self == VID_PLL_PREDIVR::VID_PLL_PREDIV_1
    }
}
#[doc = r" Proxy"]
pub struct _REFTOP_PWDW<'a> {
    w: &'a mut W,
}
impl<'a> _REFTOP_PWDW<'a> {
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
#[doc = "Values that can be written to the field `REFTOP_SELFBIASOFF`"]
pub enum REFTOP_SELFBIASOFFW {
    #[doc = "Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0,
    #[doc = "Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1,
}
impl REFTOP_SELFBIASOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFTOP_SELFBIASOFFW::REFTOP_SELFBIASOFF_0 => false,
            REFTOP_SELFBIASOFFW::REFTOP_SELFBIASOFF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFTOP_SELFBIASOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _REFTOP_SELFBIASOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFTOP_SELFBIASOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Uses coarse bias currents for startup"]
    #[inline]
    pub fn reftop_selfbiasoff_0(self) -> &'a mut W {
        self.variant(REFTOP_SELFBIASOFFW::REFTOP_SELFBIASOFF_0)
    }
    #[doc = "Uses bandgap-based bias currents for best performance."]
    #[inline]
    pub fn reftop_selfbiasoff_1(self) -> &'a mut W {
        self.variant(REFTOP_SELFBIASOFFW::REFTOP_SELFBIASOFF_1)
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
#[doc = "Values that can be written to the field `REFTOP_VBGADJ`"]
pub enum REFTOP_VBGADJW {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7,
}
impl REFTOP_VBGADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFTOP_VBGADJW::REFTOP_VBGADJ_0 => 0,
            REFTOP_VBGADJW::REFTOP_VBGADJ_1 => 1,
            REFTOP_VBGADJW::REFTOP_VBGADJ_2 => 2,
            REFTOP_VBGADJW::REFTOP_VBGADJ_3 => 3,
            REFTOP_VBGADJW::REFTOP_VBGADJ_4 => 4,
            REFTOP_VBGADJW::REFTOP_VBGADJ_5 => 5,
            REFTOP_VBGADJW::REFTOP_VBGADJ_6 => 6,
            REFTOP_VBGADJW::REFTOP_VBGADJ_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFTOP_VBGADJW<'a> {
    w: &'a mut W,
}
impl<'a> _REFTOP_VBGADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFTOP_VBGADJW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Nominal VBG"]
    #[inline]
    pub fn reftop_vbgadj_0(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_0)
    }
    #[doc = "VBG+0.78%"]
    #[inline]
    pub fn reftop_vbgadj_1(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_1)
    }
    #[doc = "VBG+1.56%"]
    #[inline]
    pub fn reftop_vbgadj_2(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_2)
    }
    #[doc = "VBG+2.34%"]
    #[inline]
    pub fn reftop_vbgadj_3(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_3)
    }
    #[doc = "VBG-0.78%"]
    #[inline]
    pub fn reftop_vbgadj_4(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_4)
    }
    #[doc = "VBG-1.56%"]
    #[inline]
    pub fn reftop_vbgadj_5(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_5)
    }
    #[doc = "VBG-2.34%"]
    #[inline]
    pub fn reftop_vbgadj_6(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_6)
    }
    #[doc = "VBG-3.12%"]
    #[inline]
    pub fn reftop_vbgadj_7(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJW::REFTOP_VBGADJ_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REFTOP_VBGUPW<'a> {
    w: &'a mut W,
}
impl<'a> _REFTOP_VBGUPW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOP_MODE_CONFIG`"]
pub enum STOP_MODE_CONFIGW {
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
    STOP_MODE_CONFIG_3,
}
impl STOP_MODE_CONFIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOP_MODE_CONFIGW::STOP_MODE_CONFIG_0 => 0,
            STOP_MODE_CONFIGW::STOP_MODE_CONFIG_1 => 1,
            STOP_MODE_CONFIGW::STOP_MODE_CONFIG_2 => 2,
            STOP_MODE_CONFIGW::STOP_MODE_CONFIG_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOP_MODE_CONFIGW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_MODE_CONFIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOP_MODE_CONFIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    #[inline]
    pub fn stop_mode_config_0(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIGW::STOP_MODE_CONFIG_0)
    }
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    #[inline]
    pub fn stop_mode_config_1(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIGW::STOP_MODE_CONFIG_1)
    }
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    #[inline]
    pub fn stop_mode_config_2(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIGW::STOP_MODE_CONFIG_2)
    }
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
    #[inline]
    pub fn stop_mode_config_3(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIGW::STOP_MODE_CONFIG_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISCON_HIGH_SNVS`"]
pub enum DISCON_HIGH_SNVSW {
    #[doc = "Turn on the switch"]
    DISCON_HIGH_SNVS_0,
    #[doc = "Turn off the switch"]
    DISCON_HIGH_SNVS_1,
}
impl DISCON_HIGH_SNVSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISCON_HIGH_SNVSW::DISCON_HIGH_SNVS_0 => false,
            DISCON_HIGH_SNVSW::DISCON_HIGH_SNVS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCON_HIGH_SNVSW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCON_HIGH_SNVSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCON_HIGH_SNVSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Turn on the switch"]
    #[inline]
    pub fn discon_high_snvs_0(self) -> &'a mut W {
        self.variant(DISCON_HIGH_SNVSW::DISCON_HIGH_SNVS_0)
    }
    #[doc = "Turn off the switch"]
    #[inline]
    pub fn discon_high_snvs_1(self) -> &'a mut W {
        self.variant(DISCON_HIGH_SNVSW::DISCON_HIGH_SNVS_1)
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
#[doc = "Values that can be written to the field `OSC_I`"]
pub enum OSC_IW {
    #[doc = "Nominal"]
    NOMINAL,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT,
}
impl OSC_IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSC_IW::NOMINAL => 0,
            OSC_IW::MINUS_12_5_PERCENT => 1,
            OSC_IW::MINUS_25_PERCENT => 2,
            OSC_IW::MINUS_37_5_PERCENT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC_IW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC_IW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Nominal"]
    #[inline]
    pub fn nominal(self) -> &'a mut W {
        self.variant(OSC_IW::NOMINAL)
    }
    #[doc = "Decrease current by 12.5%"]
    #[inline]
    pub fn minus_12_5_percent(self) -> &'a mut W {
        self.variant(OSC_IW::MINUS_12_5_PERCENT)
    }
    #[doc = "Decrease current by 25.0%"]
    #[inline]
    pub fn minus_25_percent(self) -> &'a mut W {
        self.variant(OSC_IW::MINUS_25_PERCENT)
    }
    #[doc = "Decrease current by 37.5%"]
    #[inline]
    pub fn minus_37_5_percent(self) -> &'a mut W {
        self.variant(OSC_IW::MINUS_37_5_PERCENT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSC_XTALOK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_XTALOK_ENW<'a> {
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
#[doc = "Values that can be written to the field `CLKGATE_CTRL`"]
pub enum CLKGATE_CTRLW {
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE,
    #[doc = "Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE,
}
impl CLKGATE_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKGATE_CTRLW::ALLOW_AUTO_GATE => false,
            CLKGATE_CTRLW::NO_AUTO_GATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATE_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATE_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKGATE_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    #[inline]
    pub fn allow_auto_gate(self) -> &'a mut W {
        self.variant(CLKGATE_CTRLW::ALLOW_AUTO_GATE)
    }
    #[doc = "Prevent the logic from ever gating off the clock."]
    #[inline]
    pub fn no_auto_gate(self) -> &'a mut W {
        self.variant(CLKGATE_CTRLW::NO_AUTO_GATE)
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
#[doc = "Values that can be written to the field `CLKGATE_DELAY`"]
pub enum CLKGATE_DELAYW {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7,
}
impl CLKGATE_DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKGATE_DELAYW::CLKGATE_DELAY_0 => 0,
            CLKGATE_DELAYW::CLKGATE_DELAY_1 => 1,
            CLKGATE_DELAYW::CLKGATE_DELAY_2 => 2,
            CLKGATE_DELAYW::CLKGATE_DELAY_3 => 3,
            CLKGATE_DELAYW::CLKGATE_DELAY_4 => 4,
            CLKGATE_DELAYW::CLKGATE_DELAY_5 => 5,
            CLKGATE_DELAYW::CLKGATE_DELAY_6 => 6,
            CLKGATE_DELAYW::CLKGATE_DELAY_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATE_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATE_DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKGATE_DELAYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.5ms"]
    #[inline]
    pub fn clkgate_delay_0(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_0)
    }
    #[doc = "1.0ms"]
    #[inline]
    pub fn clkgate_delay_1(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_1)
    }
    #[doc = "2.0ms"]
    #[inline]
    pub fn clkgate_delay_2(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_2)
    }
    #[doc = "3.0ms"]
    #[inline]
    pub fn clkgate_delay_3(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_3)
    }
    #[doc = "4.0ms"]
    #[inline]
    pub fn clkgate_delay_4(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_4)
    }
    #[doc = "5.0ms"]
    #[inline]
    pub fn clkgate_delay_5(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_5)
    }
    #[doc = "6.0ms"]
    #[inline]
    pub fn clkgate_delay_6(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_6)
    }
    #[doc = "7.0ms"]
    #[inline]
    pub fn clkgate_delay_7(self) -> &'a mut W {
        self.variant(CLKGATE_DELAYW::CLKGATE_DELAY_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_XTAL_SOURCE`"]
pub enum RTC_XTAL_SOURCEW {
    #[doc = "Internal ring oscillator"]
    RTC_XTAL_SOURCE_0,
    #[doc = "RTC_XTAL"]
    RTC_XTAL_SOURCE_1,
}
impl RTC_XTAL_SOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_XTAL_SOURCEW::RTC_XTAL_SOURCE_0 => false,
            RTC_XTAL_SOURCEW::RTC_XTAL_SOURCE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_XTAL_SOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_XTAL_SOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_XTAL_SOURCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal ring oscillator"]
    #[inline]
    pub fn rtc_xtal_source_0(self) -> &'a mut W {
        self.variant(RTC_XTAL_SOURCEW::RTC_XTAL_SOURCE_0)
    }
    #[doc = "RTC_XTAL"]
    #[inline]
    pub fn rtc_xtal_source_1(self) -> &'a mut W {
        self.variant(RTC_XTAL_SOURCEW::RTC_XTAL_SOURCE_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XTAL_24M_PWDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTAL_24M_PWDW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VID_PLL_PREDIV`"]
pub enum VID_PLL_PREDIVW {
    #[doc = "Divide by 1"]
    VID_PLL_PREDIV_0,
    #[doc = "Divide by 2"]
    VID_PLL_PREDIV_1,
}
impl VID_PLL_PREDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VID_PLL_PREDIVW::VID_PLL_PREDIV_0 => false,
            VID_PLL_PREDIVW::VID_PLL_PREDIV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VID_PLL_PREDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _VID_PLL_PREDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VID_PLL_PREDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn vid_pll_prediv_0(self) -> &'a mut W {
        self.variant(VID_PLL_PREDIVW::VID_PLL_PREDIV_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn vid_pll_prediv_1(self) -> &'a mut W {
        self.variant(VID_PLL_PREDIVW::VID_PLL_PREDIV_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Control bit to power-down the analog bandgap reference circuitry"]
    #[inline]
    pub fn reftop_pwd(&self) -> REFTOP_PWDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REFTOP_PWDR { bits }
    }
    #[doc = "Bit 3 - Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline]
    pub fn reftop_selfbiasoff(&self) -> REFTOP_SELFBIASOFFR {
        REFTOP_SELFBIASOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Not related to CCM. See Power Management Unit (PMU)"]
    #[inline]
    pub fn reftop_vbgadj(&self) -> REFTOP_VBGADJR {
        REFTOP_VBGADJR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline]
    pub fn reftop_vbgup(&self) -> REFTOP_VBGUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REFTOP_VBGUPR { bits }
    }
    #[doc = "Bits 10:11 - Configure the analog behavior in stop mode."]
    #[inline]
    pub fn stop_mode_config(&self) -> STOP_MODE_CONFIGR {
        STOP_MODE_CONFIGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline]
    pub fn discon_high_snvs(&self) -> DISCON_HIGH_SNVSR {
        DISCON_HIGH_SNVSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - This field determines the bias current in the 24MHz oscillator"]
    #[inline]
    pub fn osc_i(&self) -> OSC_IR {
        OSC_IR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline]
    pub fn osc_xtalok(&self) -> OSC_XTALOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSC_XTALOKR { bits }
    }
    #[doc = "Bit 16 - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline]
    pub fn osc_xtalok_en(&self) -> OSC_XTALOK_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSC_XTALOK_ENR { bits }
    }
    #[doc = "Bit 25 - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline]
    pub fn clkgate_ctrl(&self) -> CLKGATE_CTRLR {
        CLKGATE_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:28 - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline]
    pub fn clkgate_delay(&self) -> CLKGATE_DELAYR {
        CLKGATE_DELAYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - This field indicates which chip source is being used for the rtc clock"]
    #[inline]
    pub fn rtc_xtal_source(&self) -> RTC_XTAL_SOURCER {
        RTC_XTAL_SOURCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - This field powers down the 24M crystal oscillator if set true"]
    #[inline]
    pub fn xtal_24m_pwd(&self) -> XTAL_24M_PWDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTAL_24M_PWDR { bits }
    }
    #[doc = "Bit 31 - Predivider for the source clock of the PLL's."]
    #[inline]
    pub fn vid_pll_prediv(&self) -> VID_PLL_PREDIVR {
        VID_PLL_PREDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67108864 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Control bit to power-down the analog bandgap reference circuitry"]
    #[inline]
    pub fn reftop_pwd(&mut self) -> _REFTOP_PWDW {
        _REFTOP_PWDW { w: self }
    }
    #[doc = "Bit 3 - Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline]
    pub fn reftop_selfbiasoff(&mut self) -> _REFTOP_SELFBIASOFFW {
        _REFTOP_SELFBIASOFFW { w: self }
    }
    #[doc = "Bits 4:6 - Not related to CCM. See Power Management Unit (PMU)"]
    #[inline]
    pub fn reftop_vbgadj(&mut self) -> _REFTOP_VBGADJW {
        _REFTOP_VBGADJW { w: self }
    }
    #[doc = "Bit 7 - Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline]
    pub fn reftop_vbgup(&mut self) -> _REFTOP_VBGUPW {
        _REFTOP_VBGUPW { w: self }
    }
    #[doc = "Bits 10:11 - Configure the analog behavior in stop mode."]
    #[inline]
    pub fn stop_mode_config(&mut self) -> _STOP_MODE_CONFIGW {
        _STOP_MODE_CONFIGW { w: self }
    }
    #[doc = "Bit 12 - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline]
    pub fn discon_high_snvs(&mut self) -> _DISCON_HIGH_SNVSW {
        _DISCON_HIGH_SNVSW { w: self }
    }
    #[doc = "Bits 13:14 - This field determines the bias current in the 24MHz oscillator"]
    #[inline]
    pub fn osc_i(&mut self) -> _OSC_IW {
        _OSC_IW { w: self }
    }
    #[doc = "Bit 16 - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline]
    pub fn osc_xtalok_en(&mut self) -> _OSC_XTALOK_ENW {
        _OSC_XTALOK_ENW { w: self }
    }
    #[doc = "Bit 25 - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline]
    pub fn clkgate_ctrl(&mut self) -> _CLKGATE_CTRLW {
        _CLKGATE_CTRLW { w: self }
    }
    #[doc = "Bits 26:28 - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline]
    pub fn clkgate_delay(&mut self) -> _CLKGATE_DELAYW {
        _CLKGATE_DELAYW { w: self }
    }
    #[doc = "Bit 29 - This field indicates which chip source is being used for the rtc clock"]
    #[inline]
    pub fn rtc_xtal_source(&mut self) -> _RTC_XTAL_SOURCEW {
        _RTC_XTAL_SOURCEW { w: self }
    }
    #[doc = "Bit 30 - This field powers down the 24M crystal oscillator if set true"]
    #[inline]
    pub fn xtal_24m_pwd(&mut self) -> _XTAL_24M_PWDW {
        _XTAL_24M_PWDW { w: self }
    }
    #[doc = "Bit 31 - Predivider for the source clock of the PLL's."]
    #[inline]
    pub fn vid_pll_prediv(&mut self) -> _VID_PLL_PREDIVW {
        _VID_PLL_PREDIVW { w: self }
    }
}
