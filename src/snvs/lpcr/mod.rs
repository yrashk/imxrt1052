#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPCR {
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
#[doc = "Possible values of the field `SRTC_ENV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_ENVR {
    #[doc = "SRTC is disabled or invalid."]
    SRTC_ENV_0,
    #[doc = "SRTC is enabled and valid."]
    SRTC_ENV_1,
}
impl SRTC_ENVR {
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
            SRTC_ENVR::SRTC_ENV_0 => false,
            SRTC_ENVR::SRTC_ENV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRTC_ENVR {
        match value {
            false => SRTC_ENVR::SRTC_ENV_0,
            true => SRTC_ENVR::SRTC_ENV_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_ENV_0`"]
    #[inline]
    pub fn is_srtc_env_0(&self) -> bool {
        *self == SRTC_ENVR::SRTC_ENV_0
    }
    #[doc = "Checks if the value of the field is `SRTC_ENV_1`"]
    #[inline]
    pub fn is_srtc_env_1(&self) -> bool {
        *self == SRTC_ENVR::SRTC_ENV_1
    }
}
#[doc = "Possible values of the field `LPTA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTA_ENR {
    #[doc = "LP time alarm interrupt is disabled."]
    LPTA_EN_0,
    #[doc = "LP time alarm interrupt is enabled."]
    LPTA_EN_1,
}
impl LPTA_ENR {
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
            LPTA_ENR::LPTA_EN_0 => false,
            LPTA_ENR::LPTA_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPTA_ENR {
        match value {
            false => LPTA_ENR::LPTA_EN_0,
            true => LPTA_ENR::LPTA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTA_EN_0`"]
    #[inline]
    pub fn is_lpta_en_0(&self) -> bool {
        *self == LPTA_ENR::LPTA_EN_0
    }
    #[doc = "Checks if the value of the field is `LPTA_EN_1`"]
    #[inline]
    pub fn is_lpta_en_1(&self) -> bool {
        *self == LPTA_ENR::LPTA_EN_1
    }
}
#[doc = "Possible values of the field `MC_ENV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MC_ENVR {
    #[doc = "MC is disabled or invalid."]
    MC_ENV_0,
    #[doc = "MC is enabled and valid."]
    MC_ENV_1,
}
impl MC_ENVR {
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
            MC_ENVR::MC_ENV_0 => false,
            MC_ENVR::MC_ENV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MC_ENVR {
        match value {
            false => MC_ENVR::MC_ENV_0,
            true => MC_ENVR::MC_ENV_1,
        }
    }
    #[doc = "Checks if the value of the field is `MC_ENV_0`"]
    #[inline]
    pub fn is_mc_env_0(&self) -> bool {
        *self == MC_ENVR::MC_ENV_0
    }
    #[doc = "Checks if the value of the field is `MC_ENV_1`"]
    #[inline]
    pub fn is_mc_env_1(&self) -> bool {
        *self == MC_ENVR::MC_ENV_1
    }
}
#[doc = r" Value of the field"]
pub struct LPWUI_ENR {
    bits: bool,
}
impl LPWUI_ENR {
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
#[doc = "Possible values of the field `SRTC_INV_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_INV_ENR {
    #[doc = "SRTC stays valid in the case of security violation."]
    SRTC_INV_EN_0,
    #[doc = "SRTC is invalidated in the case of security violation."]
    SRTC_INV_EN_1,
}
impl SRTC_INV_ENR {
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
            SRTC_INV_ENR::SRTC_INV_EN_0 => false,
            SRTC_INV_ENR::SRTC_INV_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRTC_INV_ENR {
        match value {
            false => SRTC_INV_ENR::SRTC_INV_EN_0,
            true => SRTC_INV_ENR::SRTC_INV_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_INV_EN_0`"]
    #[inline]
    pub fn is_srtc_inv_en_0(&self) -> bool {
        *self == SRTC_INV_ENR::SRTC_INV_EN_0
    }
    #[doc = "Checks if the value of the field is `SRTC_INV_EN_1`"]
    #[inline]
    pub fn is_srtc_inv_en_1(&self) -> bool {
        *self == SRTC_INV_ENR::SRTC_INV_EN_1
    }
}
#[doc = "Possible values of the field `DP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DP_ENR {
    #[doc = "Smart PMIC enabled."]
    DP_EN_0,
    #[doc = "Dumb PMIC enabled."]
    DP_EN_1,
}
impl DP_ENR {
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
            DP_ENR::DP_EN_0 => false,
            DP_ENR::DP_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DP_ENR {
        match value {
            false => DP_ENR::DP_EN_0,
            true => DP_ENR::DP_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DP_EN_0`"]
    #[inline]
    pub fn is_dp_en_0(&self) -> bool {
        *self == DP_ENR::DP_EN_0
    }
    #[doc = "Checks if the value of the field is `DP_EN_1`"]
    #[inline]
    pub fn is_dp_en_1(&self) -> bool {
        *self == DP_ENR::DP_EN_1
    }
}
#[doc = "Possible values of the field `TOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOPR {
    #[doc = "Leave system power on."]
    TOP_0,
    #[doc = "Turn off system power."]
    TOP_1,
}
impl TOPR {
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
            TOPR::TOP_0 => false,
            TOPR::TOP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOPR {
        match value {
            false => TOPR::TOP_0,
            true => TOPR::TOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOP_0`"]
    #[inline]
    pub fn is_top_0(&self) -> bool {
        *self == TOPR::TOP_0
    }
    #[doc = "Checks if the value of the field is `TOP_1`"]
    #[inline]
    pub fn is_top_1(&self) -> bool {
        *self == TOPR::TOP_1
    }
}
#[doc = r" Value of the field"]
pub struct PWR_GLITCH_ENR {
    bits: bool,
}
impl PWR_GLITCH_ENR {
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
#[doc = "Possible values of the field `LPCALB_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCALB_ENR {
    #[doc = "SRTC Time calibration is disabled."]
    LPCALB_EN_0,
    #[doc = "SRTC Time calibration is enabled."]
    LPCALB_EN_1,
}
impl LPCALB_ENR {
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
            LPCALB_ENR::LPCALB_EN_0 => false,
            LPCALB_ENR::LPCALB_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPCALB_ENR {
        match value {
            false => LPCALB_ENR::LPCALB_EN_0,
            true => LPCALB_ENR::LPCALB_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_EN_0`"]
    #[inline]
    pub fn is_lpcalb_en_0(&self) -> bool {
        *self == LPCALB_ENR::LPCALB_EN_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_EN_1`"]
    #[inline]
    pub fn is_lpcalb_en_1(&self) -> bool {
        *self == LPCALB_ENR::LPCALB_EN_1
    }
}
#[doc = "Possible values of the field `LPCALB_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCALB_VALR {
    #[doc = "+0 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_0,
    #[doc = "+1 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_1,
    #[doc = "+2 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_2,
    #[doc = "+15 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_15,
    #[doc = "-16 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_16,
    #[doc = "-15 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_17,
    #[doc = "-2 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_30,
    #[doc = "-1 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPCALB_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPCALB_VALR::LPCALB_VAL_0 => 0,
            LPCALB_VALR::LPCALB_VAL_1 => 1,
            LPCALB_VALR::LPCALB_VAL_2 => 2,
            LPCALB_VALR::LPCALB_VAL_15 => 15,
            LPCALB_VALR::LPCALB_VAL_16 => 16,
            LPCALB_VALR::LPCALB_VAL_17 => 17,
            LPCALB_VALR::LPCALB_VAL_30 => 30,
            LPCALB_VALR::LPCALB_VAL_31 => 31,
            LPCALB_VALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPCALB_VALR {
        match value {
            0 => LPCALB_VALR::LPCALB_VAL_0,
            1 => LPCALB_VALR::LPCALB_VAL_1,
            2 => LPCALB_VALR::LPCALB_VAL_2,
            15 => LPCALB_VALR::LPCALB_VAL_15,
            16 => LPCALB_VALR::LPCALB_VAL_16,
            17 => LPCALB_VALR::LPCALB_VAL_17,
            30 => LPCALB_VALR::LPCALB_VAL_30,
            31 => LPCALB_VALR::LPCALB_VAL_31,
            i => LPCALB_VALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_0`"]
    #[inline]
    pub fn is_lpcalb_val_0(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_1`"]
    #[inline]
    pub fn is_lpcalb_val_1(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_1
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_2`"]
    #[inline]
    pub fn is_lpcalb_val_2(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_2
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_15`"]
    #[inline]
    pub fn is_lpcalb_val_15(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_15
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_16`"]
    #[inline]
    pub fn is_lpcalb_val_16(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_16
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_17`"]
    #[inline]
    pub fn is_lpcalb_val_17(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_17
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_30`"]
    #[inline]
    pub fn is_lpcalb_val_30(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_30
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_31`"]
    #[inline]
    pub fn is_lpcalb_val_31(&self) -> bool {
        *self == LPCALB_VALR::LPCALB_VAL_31
    }
}
#[doc = r" Value of the field"]
pub struct BTN_PRESS_TIMER {
    bits: u8,
}
impl BTN_PRESS_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEBOUNCER {
    bits: u8,
}
impl DEBOUNCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ON_TIMER {
    bits: u8,
}
impl ON_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PK_ENR {
    bits: bool,
}
impl PK_ENR {
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
pub struct PK_OVERRIDER {
    bits: bool,
}
impl PK_OVERRIDER {
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
pub struct GPR_Z_DISR {
    bits: bool,
}
impl GPR_Z_DISR {
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
#[doc = "Values that can be written to the field `SRTC_ENV`"]
pub enum SRTC_ENVW {
    #[doc = "SRTC is disabled or invalid."]
    SRTC_ENV_0,
    #[doc = "SRTC is enabled and valid."]
    SRTC_ENV_1,
}
impl SRTC_ENVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRTC_ENVW::SRTC_ENV_0 => false,
            SRTC_ENVW::SRTC_ENV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRTC_ENVW<'a> {
    w: &'a mut W,
}
impl<'a> _SRTC_ENVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRTC_ENVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRTC is disabled or invalid."]
    #[inline]
    pub fn srtc_env_0(self) -> &'a mut W {
        self.variant(SRTC_ENVW::SRTC_ENV_0)
    }
    #[doc = "SRTC is enabled and valid."]
    #[inline]
    pub fn srtc_env_1(self) -> &'a mut W {
        self.variant(SRTC_ENVW::SRTC_ENV_1)
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
#[doc = "Values that can be written to the field `LPTA_EN`"]
pub enum LPTA_ENW {
    #[doc = "LP time alarm interrupt is disabled."]
    LPTA_EN_0,
    #[doc = "LP time alarm interrupt is enabled."]
    LPTA_EN_1,
}
impl LPTA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPTA_ENW::LPTA_EN_0 => false,
            LPTA_ENW::LPTA_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPTA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LP time alarm interrupt is disabled."]
    #[inline]
    pub fn lpta_en_0(self) -> &'a mut W {
        self.variant(LPTA_ENW::LPTA_EN_0)
    }
    #[doc = "LP time alarm interrupt is enabled."]
    #[inline]
    pub fn lpta_en_1(self) -> &'a mut W {
        self.variant(LPTA_ENW::LPTA_EN_1)
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
#[doc = "Values that can be written to the field `MC_ENV`"]
pub enum MC_ENVW {
    #[doc = "MC is disabled or invalid."]
    MC_ENV_0,
    #[doc = "MC is enabled and valid."]
    MC_ENV_1,
}
impl MC_ENVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MC_ENVW::MC_ENV_0 => false,
            MC_ENVW::MC_ENV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MC_ENVW<'a> {
    w: &'a mut W,
}
impl<'a> _MC_ENVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MC_ENVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MC is disabled or invalid."]
    #[inline]
    pub fn mc_env_0(self) -> &'a mut W {
        self.variant(MC_ENVW::MC_ENV_0)
    }
    #[doc = "MC is enabled and valid."]
    #[inline]
    pub fn mc_env_1(self) -> &'a mut W {
        self.variant(MC_ENVW::MC_ENV_1)
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
#[doc = r" Proxy"]
pub struct _LPWUI_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPWUI_ENW<'a> {
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
#[doc = "Values that can be written to the field `SRTC_INV_EN`"]
pub enum SRTC_INV_ENW {
    #[doc = "SRTC stays valid in the case of security violation."]
    SRTC_INV_EN_0,
    #[doc = "SRTC is invalidated in the case of security violation."]
    SRTC_INV_EN_1,
}
impl SRTC_INV_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRTC_INV_ENW::SRTC_INV_EN_0 => false,
            SRTC_INV_ENW::SRTC_INV_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRTC_INV_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRTC_INV_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRTC_INV_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRTC stays valid in the case of security violation."]
    #[inline]
    pub fn srtc_inv_en_0(self) -> &'a mut W {
        self.variant(SRTC_INV_ENW::SRTC_INV_EN_0)
    }
    #[doc = "SRTC is invalidated in the case of security violation."]
    #[inline]
    pub fn srtc_inv_en_1(self) -> &'a mut W {
        self.variant(SRTC_INV_ENW::SRTC_INV_EN_1)
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
#[doc = "Values that can be written to the field `DP_EN`"]
pub enum DP_ENW {
    #[doc = "Smart PMIC enabled."]
    DP_EN_0,
    #[doc = "Dumb PMIC enabled."]
    DP_EN_1,
}
impl DP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DP_ENW::DP_EN_0 => false,
            DP_ENW::DP_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smart PMIC enabled."]
    #[inline]
    pub fn dp_en_0(self) -> &'a mut W {
        self.variant(DP_ENW::DP_EN_0)
    }
    #[doc = "Dumb PMIC enabled."]
    #[inline]
    pub fn dp_en_1(self) -> &'a mut W {
        self.variant(DP_ENW::DP_EN_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOP`"]
pub enum TOPW {
    #[doc = "Leave system power on."]
    TOP_0,
    #[doc = "Turn off system power."]
    TOP_1,
}
impl TOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOPW::TOP_0 => false,
            TOPW::TOP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Leave system power on."]
    #[inline]
    pub fn top_0(self) -> &'a mut W {
        self.variant(TOPW::TOP_0)
    }
    #[doc = "Turn off system power."]
    #[inline]
    pub fn top_1(self) -> &'a mut W {
        self.variant(TOPW::TOP_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWR_GLITCH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_GLITCH_ENW<'a> {
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
#[doc = "Values that can be written to the field `LPCALB_EN`"]
pub enum LPCALB_ENW {
    #[doc = "SRTC Time calibration is disabled."]
    LPCALB_EN_0,
    #[doc = "SRTC Time calibration is enabled."]
    LPCALB_EN_1,
}
impl LPCALB_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPCALB_ENW::LPCALB_EN_0 => false,
            LPCALB_ENW::LPCALB_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCALB_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCALB_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCALB_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRTC Time calibration is disabled."]
    #[inline]
    pub fn lpcalb_en_0(self) -> &'a mut W {
        self.variant(LPCALB_ENW::LPCALB_EN_0)
    }
    #[doc = "SRTC Time calibration is enabled."]
    #[inline]
    pub fn lpcalb_en_1(self) -> &'a mut W {
        self.variant(LPCALB_ENW::LPCALB_EN_1)
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
#[doc = "Values that can be written to the field `LPCALB_VAL`"]
pub enum LPCALB_VALW {
    #[doc = "+0 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_0,
    #[doc = "+1 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_1,
    #[doc = "+2 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_2,
    #[doc = "+15 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_15,
    #[doc = "-16 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_16,
    #[doc = "-15 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_17,
    #[doc = "-2 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_30,
    #[doc = "-1 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_31,
}
impl LPCALB_VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPCALB_VALW::LPCALB_VAL_0 => 0,
            LPCALB_VALW::LPCALB_VAL_1 => 1,
            LPCALB_VALW::LPCALB_VAL_2 => 2,
            LPCALB_VALW::LPCALB_VAL_15 => 15,
            LPCALB_VALW::LPCALB_VAL_16 => 16,
            LPCALB_VALW::LPCALB_VAL_17 => 17,
            LPCALB_VALW::LPCALB_VAL_30 => 30,
            LPCALB_VALW::LPCALB_VAL_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCALB_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCALB_VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCALB_VALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "+0 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_0(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_0)
    }
    #[doc = "+1 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_1(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_1)
    }
    #[doc = "+2 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_2(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_2)
    }
    #[doc = "+15 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_15(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_15)
    }
    #[doc = "-16 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_16(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_16)
    }
    #[doc = "-15 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_17(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_17)
    }
    #[doc = "-2 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_30(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_30)
    }
    #[doc = "-1 counts per each 32768 ticks of the counter clock"]
    #[inline]
    pub fn lpcalb_val_31(self) -> &'a mut W {
        self.variant(LPCALB_VALW::LPCALB_VAL_31)
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
#[doc = r" Proxy"]
pub struct _BTN_PRESS_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _BTN_PRESS_TIMEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBOUNCEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBOUNCEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ON_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _ON_TIMEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PK_ENW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PK_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PK_OVERRIDEW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPR_Z_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _GPR_Z_DISW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline]
    pub fn srtc_env(&self) -> SRTC_ENVR {
        SRTC_ENVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline]
    pub fn lpta_en(&self) -> LPTA_ENR {
        LPTA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline]
    pub fn mc_env(&self) -> MC_ENVR {
        MC_ENVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[inline]
    pub fn lpwui_en(&self) -> LPWUI_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPWUI_ENR { bits }
    }
    #[doc = "Bit 4 - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline]
    pub fn srtc_inv_en(&self) -> SRTC_INV_ENR {
        SRTC_INV_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Dumb PMIC Enabled When set, software can control the system power"]
    #[inline]
    pub fn dp_en(&self) -> DP_ENR {
        DP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline]
    pub fn top(&self) -> TOPR {
        TOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Power Glitch Enable By default the detection of a power glitch does not cause the pmic_en_b signal to be asserted"]
    #[inline]
    pub fn pwr_glitch_en(&self) -> PWR_GLITCH_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWR_GLITCH_ENR { bits }
    }
    #[doc = "Bit 8 - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline]
    pub fn lpcalb_en(&self) -> LPCALB_ENR {
        LPCALB_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:14 - LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline]
    pub fn lpcalb_val(&self) -> LPCALB_VALR {
        LPCALB_VALR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - This field configures the button press time out values for the PMIC Logic"]
    #[inline]
    pub fn btn_press_time(&self) -> BTN_PRESS_TIMER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BTN_PRESS_TIMER { bits }
    }
    #[doc = "Bits 18:19 - This field configures the amount of debounce time for the BTN input signal"]
    #[inline]
    pub fn debounce(&self) -> DEBOUNCER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEBOUNCER { bits }
    }
    #[doc = "Bits 20:21 - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline]
    pub fn on_time(&self) -> ON_TIMER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ON_TIMER { bits }
    }
    #[doc = "Bit 22 - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline]
    pub fn pk_en(&self) -> PK_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PK_ENR { bits }
    }
    #[doc = "Bit 23 - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline]
    pub fn pk_override(&self) -> PK_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PK_OVERRIDER { bits }
    }
    #[doc = "Bit 24 - General Purpose Registers Zeroization Disable"]
    #[inline]
    pub fn gpr_z_dis(&self) -> GPR_Z_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPR_Z_DISR { bits }
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
    #[doc = "Bit 0 - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline]
    pub fn srtc_env(&mut self) -> _SRTC_ENVW {
        _SRTC_ENVW { w: self }
    }
    #[doc = "Bit 1 - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline]
    pub fn lpta_en(&mut self) -> _LPTA_ENW {
        _LPTA_ENW { w: self }
    }
    #[doc = "Bit 2 - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline]
    pub fn mc_env(&mut self) -> _MC_ENVW {
        _MC_ENVW { w: self }
    }
    #[doc = "Bit 3 - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[inline]
    pub fn lpwui_en(&mut self) -> _LPWUI_ENW {
        _LPWUI_ENW { w: self }
    }
    #[doc = "Bit 4 - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline]
    pub fn srtc_inv_en(&mut self) -> _SRTC_INV_ENW {
        _SRTC_INV_ENW { w: self }
    }
    #[doc = "Bit 5 - Dumb PMIC Enabled When set, software can control the system power"]
    #[inline]
    pub fn dp_en(&mut self) -> _DP_ENW {
        _DP_ENW { w: self }
    }
    #[doc = "Bit 6 - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline]
    pub fn top(&mut self) -> _TOPW {
        _TOPW { w: self }
    }
    #[doc = "Bit 7 - Power Glitch Enable By default the detection of a power glitch does not cause the pmic_en_b signal to be asserted"]
    #[inline]
    pub fn pwr_glitch_en(&mut self) -> _PWR_GLITCH_ENW {
        _PWR_GLITCH_ENW { w: self }
    }
    #[doc = "Bit 8 - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline]
    pub fn lpcalb_en(&mut self) -> _LPCALB_ENW {
        _LPCALB_ENW { w: self }
    }
    #[doc = "Bits 10:14 - LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline]
    pub fn lpcalb_val(&mut self) -> _LPCALB_VALW {
        _LPCALB_VALW { w: self }
    }
    #[doc = "Bits 16:17 - This field configures the button press time out values for the PMIC Logic"]
    #[inline]
    pub fn btn_press_time(&mut self) -> _BTN_PRESS_TIMEW {
        _BTN_PRESS_TIMEW { w: self }
    }
    #[doc = "Bits 18:19 - This field configures the amount of debounce time for the BTN input signal"]
    #[inline]
    pub fn debounce(&mut self) -> _DEBOUNCEW {
        _DEBOUNCEW { w: self }
    }
    #[doc = "Bits 20:21 - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline]
    pub fn on_time(&mut self) -> _ON_TIMEW {
        _ON_TIMEW { w: self }
    }
    #[doc = "Bit 22 - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline]
    pub fn pk_en(&mut self) -> _PK_ENW {
        _PK_ENW { w: self }
    }
    #[doc = "Bit 23 - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline]
    pub fn pk_override(&mut self) -> _PK_OVERRIDEW {
        _PK_OVERRIDEW { w: self }
    }
    #[doc = "Bit 24 - General Purpose Registers Zeroization Disable"]
    #[inline]
    pub fn gpr_z_dis(&mut self) -> _GPR_Z_DISW {
        _GPR_Z_DISW { w: self }
    }
}
