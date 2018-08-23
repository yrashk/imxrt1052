#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM0CTRL {
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
#[doc = "Possible values of the field `DBLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLENR {
    #[doc = "Double switching disabled."]
    DBLEN_0,
    #[doc = "Double switching enabled."]
    DBLEN_1,
}
impl DBLENR {
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
            DBLENR::DBLEN_0 => false,
            DBLENR::DBLEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBLENR {
        match value {
            false => DBLENR::DBLEN_0,
            true => DBLENR::DBLEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBLEN_0`"]
    #[inline]
    pub fn is_dblen_0(&self) -> bool {
        *self == DBLENR::DBLEN_0
    }
    #[doc = "Checks if the value of the field is `DBLEN_1`"]
    #[inline]
    pub fn is_dblen_1(&self) -> bool {
        *self == DBLENR::DBLEN_1
    }
}
#[doc = "Possible values of the field `DBLX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLXR {
    #[doc = "PWMX double pulse disabled."]
    DBLX_0,
    #[doc = "PWMX double pulse enabled."]
    DBLX_1,
}
impl DBLXR {
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
            DBLXR::DBLX_0 => false,
            DBLXR::DBLX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBLXR {
        match value {
            false => DBLXR::DBLX_0,
            true => DBLXR::DBLX_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBLX_0`"]
    #[inline]
    pub fn is_dblx_0(&self) -> bool {
        *self == DBLXR::DBLX_0
    }
    #[doc = "Checks if the value of the field is `DBLX_1`"]
    #[inline]
    pub fn is_dblx_1(&self) -> bool {
        *self == DBLXR::DBLX_1
    }
}
#[doc = "Possible values of the field `LDMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMODR {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL[LDOK] is set."]
    LDMOD_0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL[LDOK] being set. In this case it is not necessary to set CTRL[FULL] or CTRL[HALF]."]
    LDMOD_1,
}
impl LDMODR {
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
            LDMODR::LDMOD_0 => false,
            LDMODR::LDMOD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDMODR {
        match value {
            false => LDMODR::LDMOD_0,
            true => LDMODR::LDMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `LDMOD_0`"]
    #[inline]
    pub fn is_ldmod_0(&self) -> bool {
        *self == LDMODR::LDMOD_0
    }
    #[doc = "Checks if the value of the field is `LDMOD_1`"]
    #[inline]
    pub fn is_ldmod_1(&self) -> bool {
        *self == LDMODR::LDMOD_1
    }
}
#[doc = "Possible values of the field `SPLIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLITR {
    #[doc = "DBLPWM is not split. PWMA and PWMB each have double pulses."]
    SPLIT_0,
    #[doc = "DBLPWM is split to PWMA and PWMB."]
    SPLIT_1,
}
impl SPLITR {
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
            SPLITR::SPLIT_0 => false,
            SPLITR::SPLIT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPLITR {
        match value {
            false => SPLITR::SPLIT_0,
            true => SPLITR::SPLIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPLIT_0`"]
    #[inline]
    pub fn is_split_0(&self) -> bool {
        *self == SPLITR::SPLIT_0
    }
    #[doc = "Checks if the value of the field is `SPLIT_1`"]
    #[inline]
    pub fn is_split_1(&self) -> bool {
        *self == SPLITR::SPLIT_1
    }
}
#[doc = "Possible values of the field `PRSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSCR {
    #[doc = "no description available"]
    PRSC_0,
    #[doc = "no description available"]
    PRSC_1,
    #[doc = "no description available"]
    PRSC_2,
    #[doc = "no description available"]
    PRSC_3,
    #[doc = "no description available"]
    PRSC_4,
    #[doc = "no description available"]
    PRSC_5,
    #[doc = "no description available"]
    PRSC_6,
    #[doc = "no description available"]
    PRSC_7,
}
impl PRSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSCR::PRSC_0 => 0,
            PRSCR::PRSC_1 => 1,
            PRSCR::PRSC_2 => 2,
            PRSCR::PRSC_3 => 3,
            PRSCR::PRSC_4 => 4,
            PRSCR::PRSC_5 => 5,
            PRSCR::PRSC_6 => 6,
            PRSCR::PRSC_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSCR {
        match value {
            0 => PRSCR::PRSC_0,
            1 => PRSCR::PRSC_1,
            2 => PRSCR::PRSC_2,
            3 => PRSCR::PRSC_3,
            4 => PRSCR::PRSC_4,
            5 => PRSCR::PRSC_5,
            6 => PRSCR::PRSC_6,
            7 => PRSCR::PRSC_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSC_0`"]
    #[inline]
    pub fn is_prsc_0(&self) -> bool {
        *self == PRSCR::PRSC_0
    }
    #[doc = "Checks if the value of the field is `PRSC_1`"]
    #[inline]
    pub fn is_prsc_1(&self) -> bool {
        *self == PRSCR::PRSC_1
    }
    #[doc = "Checks if the value of the field is `PRSC_2`"]
    #[inline]
    pub fn is_prsc_2(&self) -> bool {
        *self == PRSCR::PRSC_2
    }
    #[doc = "Checks if the value of the field is `PRSC_3`"]
    #[inline]
    pub fn is_prsc_3(&self) -> bool {
        *self == PRSCR::PRSC_3
    }
    #[doc = "Checks if the value of the field is `PRSC_4`"]
    #[inline]
    pub fn is_prsc_4(&self) -> bool {
        *self == PRSCR::PRSC_4
    }
    #[doc = "Checks if the value of the field is `PRSC_5`"]
    #[inline]
    pub fn is_prsc_5(&self) -> bool {
        *self == PRSCR::PRSC_5
    }
    #[doc = "Checks if the value of the field is `PRSC_6`"]
    #[inline]
    pub fn is_prsc_6(&self) -> bool {
        *self == PRSCR::PRSC_6
    }
    #[doc = "Checks if the value of the field is `PRSC_7`"]
    #[inline]
    pub fn is_prsc_7(&self) -> bool {
        *self == PRSCR::PRSC_7
    }
}
#[doc = "Possible values of the field `COMPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPMODER {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
    COMPMODE_0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    COMPMODE_1,
}
impl COMPMODER {
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
            COMPMODER::COMPMODE_0 => false,
            COMPMODER::COMPMODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPMODER {
        match value {
            false => COMPMODER::COMPMODE_0,
            true => COMPMODER::COMPMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `COMPMODE_0`"]
    #[inline]
    pub fn is_compmode_0(&self) -> bool {
        *self == COMPMODER::COMPMODE_0
    }
    #[doc = "Checks if the value of the field is `COMPMODE_1`"]
    #[inline]
    pub fn is_compmode_1(&self) -> bool {
        *self == COMPMODER::COMPMODE_1
    }
}
#[doc = r" Value of the field"]
pub struct DTR {
    bits: u8,
}
impl DTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULLR {
    #[doc = "Full-cycle reloads disabled."]
    FULL_0,
    #[doc = "Full-cycle reloads enabled."]
    FULL_1,
}
impl FULLR {
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
            FULLR::FULL_0 => false,
            FULLR::FULL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FULLR {
        match value {
            false => FULLR::FULL_0,
            true => FULLR::FULL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_0`"]
    #[inline]
    pub fn is_full_0(&self) -> bool {
        *self == FULLR::FULL_0
    }
    #[doc = "Checks if the value of the field is `FULL_1`"]
    #[inline]
    pub fn is_full_1(&self) -> bool {
        *self == FULLR::FULL_1
    }
}
#[doc = "Possible values of the field `HALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALFR {
    #[doc = "Half-cycle reloads disabled."]
    HALF_0,
    #[doc = "Half-cycle reloads enabled."]
    HALF_1,
}
impl HALFR {
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
            HALFR::HALF_0 => false,
            HALFR::HALF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HALFR {
        match value {
            false => HALFR::HALF_0,
            true => HALFR::HALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_0`"]
    #[inline]
    pub fn is_half_0(&self) -> bool {
        *self == HALFR::HALF_0
    }
    #[doc = "Checks if the value of the field is `HALF_1`"]
    #[inline]
    pub fn is_half_1(&self) -> bool {
        *self == HALFR::HALF_1
    }
}
#[doc = "Possible values of the field `LDFQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDFQR {
    #[doc = "Every PWM opportunity"]
    LDFQ_0,
    #[doc = "Every 2 PWM opportunities"]
    LDFQ_1,
    #[doc = "Every 3 PWM opportunities"]
    LDFQ_2,
    #[doc = "Every 4 PWM opportunities"]
    LDFQ_3,
    #[doc = "Every 5 PWM opportunities"]
    LDFQ_4,
    #[doc = "Every 6 PWM opportunities"]
    LDFQ_5,
    #[doc = "Every 7 PWM opportunities"]
    LDFQ_6,
    #[doc = "Every 8 PWM opportunities"]
    LDFQ_7,
    #[doc = "Every 9 PWM opportunities"]
    LDFQ_8,
    #[doc = "Every 10 PWM opportunities"]
    LDFQ_9,
    #[doc = "Every 11 PWM opportunities"]
    LDFQ_10,
    #[doc = "Every 12 PWM opportunities"]
    LDFQ_11,
    #[doc = "Every 13 PWM opportunities"]
    LDFQ_12,
    #[doc = "Every 14 PWM opportunities"]
    LDFQ_13,
    #[doc = "Every 15 PWM opportunities"]
    LDFQ_14,
    #[doc = "Every 16 PWM opportunities"]
    LDFQ_15,
}
impl LDFQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDFQR::LDFQ_0 => 0,
            LDFQR::LDFQ_1 => 1,
            LDFQR::LDFQ_2 => 2,
            LDFQR::LDFQ_3 => 3,
            LDFQR::LDFQ_4 => 4,
            LDFQR::LDFQ_5 => 5,
            LDFQR::LDFQ_6 => 6,
            LDFQR::LDFQ_7 => 7,
            LDFQR::LDFQ_8 => 8,
            LDFQR::LDFQ_9 => 9,
            LDFQR::LDFQ_10 => 10,
            LDFQR::LDFQ_11 => 11,
            LDFQR::LDFQ_12 => 12,
            LDFQR::LDFQ_13 => 13,
            LDFQR::LDFQ_14 => 14,
            LDFQR::LDFQ_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDFQR {
        match value {
            0 => LDFQR::LDFQ_0,
            1 => LDFQR::LDFQ_1,
            2 => LDFQR::LDFQ_2,
            3 => LDFQR::LDFQ_3,
            4 => LDFQR::LDFQ_4,
            5 => LDFQR::LDFQ_5,
            6 => LDFQR::LDFQ_6,
            7 => LDFQR::LDFQ_7,
            8 => LDFQR::LDFQ_8,
            9 => LDFQR::LDFQ_9,
            10 => LDFQR::LDFQ_10,
            11 => LDFQR::LDFQ_11,
            12 => LDFQR::LDFQ_12,
            13 => LDFQR::LDFQ_13,
            14 => LDFQR::LDFQ_14,
            15 => LDFQR::LDFQ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LDFQ_0`"]
    #[inline]
    pub fn is_ldfq_0(&self) -> bool {
        *self == LDFQR::LDFQ_0
    }
    #[doc = "Checks if the value of the field is `LDFQ_1`"]
    #[inline]
    pub fn is_ldfq_1(&self) -> bool {
        *self == LDFQR::LDFQ_1
    }
    #[doc = "Checks if the value of the field is `LDFQ_2`"]
    #[inline]
    pub fn is_ldfq_2(&self) -> bool {
        *self == LDFQR::LDFQ_2
    }
    #[doc = "Checks if the value of the field is `LDFQ_3`"]
    #[inline]
    pub fn is_ldfq_3(&self) -> bool {
        *self == LDFQR::LDFQ_3
    }
    #[doc = "Checks if the value of the field is `LDFQ_4`"]
    #[inline]
    pub fn is_ldfq_4(&self) -> bool {
        *self == LDFQR::LDFQ_4
    }
    #[doc = "Checks if the value of the field is `LDFQ_5`"]
    #[inline]
    pub fn is_ldfq_5(&self) -> bool {
        *self == LDFQR::LDFQ_5
    }
    #[doc = "Checks if the value of the field is `LDFQ_6`"]
    #[inline]
    pub fn is_ldfq_6(&self) -> bool {
        *self == LDFQR::LDFQ_6
    }
    #[doc = "Checks if the value of the field is `LDFQ_7`"]
    #[inline]
    pub fn is_ldfq_7(&self) -> bool {
        *self == LDFQR::LDFQ_7
    }
    #[doc = "Checks if the value of the field is `LDFQ_8`"]
    #[inline]
    pub fn is_ldfq_8(&self) -> bool {
        *self == LDFQR::LDFQ_8
    }
    #[doc = "Checks if the value of the field is `LDFQ_9`"]
    #[inline]
    pub fn is_ldfq_9(&self) -> bool {
        *self == LDFQR::LDFQ_9
    }
    #[doc = "Checks if the value of the field is `LDFQ_10`"]
    #[inline]
    pub fn is_ldfq_10(&self) -> bool {
        *self == LDFQR::LDFQ_10
    }
    #[doc = "Checks if the value of the field is `LDFQ_11`"]
    #[inline]
    pub fn is_ldfq_11(&self) -> bool {
        *self == LDFQR::LDFQ_11
    }
    #[doc = "Checks if the value of the field is `LDFQ_12`"]
    #[inline]
    pub fn is_ldfq_12(&self) -> bool {
        *self == LDFQR::LDFQ_12
    }
    #[doc = "Checks if the value of the field is `LDFQ_13`"]
    #[inline]
    pub fn is_ldfq_13(&self) -> bool {
        *self == LDFQR::LDFQ_13
    }
    #[doc = "Checks if the value of the field is `LDFQ_14`"]
    #[inline]
    pub fn is_ldfq_14(&self) -> bool {
        *self == LDFQR::LDFQ_14
    }
    #[doc = "Checks if the value of the field is `LDFQ_15`"]
    #[inline]
    pub fn is_ldfq_15(&self) -> bool {
        *self == LDFQR::LDFQ_15
    }
}
#[doc = "Values that can be written to the field `DBLEN`"]
pub enum DBLENW {
    #[doc = "Double switching disabled."]
    DBLEN_0,
    #[doc = "Double switching enabled."]
    DBLEN_1,
}
impl DBLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBLENW::DBLEN_0 => false,
            DBLENW::DBLEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBLENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Double switching disabled."]
    #[inline]
    pub fn dblen_0(self) -> &'a mut W {
        self.variant(DBLENW::DBLEN_0)
    }
    #[doc = "Double switching enabled."]
    #[inline]
    pub fn dblen_1(self) -> &'a mut W {
        self.variant(DBLENW::DBLEN_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBLX`"]
pub enum DBLXW {
    #[doc = "PWMX double pulse disabled."]
    DBLX_0,
    #[doc = "PWMX double pulse enabled."]
    DBLX_1,
}
impl DBLXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBLXW::DBLX_0 => false,
            DBLXW::DBLX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBLXW<'a> {
    w: &'a mut W,
}
impl<'a> _DBLXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBLXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWMX double pulse disabled."]
    #[inline]
    pub fn dblx_0(self) -> &'a mut W {
        self.variant(DBLXW::DBLX_0)
    }
    #[doc = "PWMX double pulse enabled."]
    #[inline]
    pub fn dblx_1(self) -> &'a mut W {
        self.variant(DBLXW::DBLX_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LDMOD`"]
pub enum LDMODW {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL[LDOK] is set."]
    LDMOD_0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL[LDOK] being set. In this case it is not necessary to set CTRL[FULL] or CTRL[HALF]."]
    LDMOD_1,
}
impl LDMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDMODW::LDMOD_0 => false,
            LDMODW::LDMOD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDMODW<'a> {
    w: &'a mut W,
}
impl<'a> _LDMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL[LDOK] is set."]
    #[inline]
    pub fn ldmod_0(self) -> &'a mut W {
        self.variant(LDMODW::LDMOD_0)
    }
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL[LDOK] being set. In this case it is not necessary to set CTRL[FULL] or CTRL[HALF]."]
    #[inline]
    pub fn ldmod_1(self) -> &'a mut W {
        self.variant(LDMODW::LDMOD_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPLIT`"]
pub enum SPLITW {
    #[doc = "DBLPWM is not split. PWMA and PWMB each have double pulses."]
    SPLIT_0,
    #[doc = "DBLPWM is split to PWMA and PWMB."]
    SPLIT_1,
}
impl SPLITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPLITW::SPLIT_0 => false,
            SPLITW::SPLIT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPLITW<'a> {
    w: &'a mut W,
}
impl<'a> _SPLITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPLITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DBLPWM is not split. PWMA and PWMB each have double pulses."]
    #[inline]
    pub fn split_0(self) -> &'a mut W {
        self.variant(SPLITW::SPLIT_0)
    }
    #[doc = "DBLPWM is split to PWMA and PWMB."]
    #[inline]
    pub fn split_1(self) -> &'a mut W {
        self.variant(SPLITW::SPLIT_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSC`"]
pub enum PRSCW {
    #[doc = "no description available"]
    PRSC_0,
    #[doc = "no description available"]
    PRSC_1,
    #[doc = "no description available"]
    PRSC_2,
    #[doc = "no description available"]
    PRSC_3,
    #[doc = "no description available"]
    PRSC_4,
    #[doc = "no description available"]
    PRSC_5,
    #[doc = "no description available"]
    PRSC_6,
    #[doc = "no description available"]
    PRSC_7,
}
impl PRSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSCW::PRSC_0 => 0,
            PRSCW::PRSC_1 => 1,
            PRSCW::PRSC_2 => 2,
            PRSCW::PRSC_3 => 3,
            PRSCW::PRSC_4 => 4,
            PRSCW::PRSC_5 => 5,
            PRSCW::PRSC_6 => 6,
            PRSCW::PRSC_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_0(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_1(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_1)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_2(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_2)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_3(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_3)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_4(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_4)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_5(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_5)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_6(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_6)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn prsc_7(self) -> &'a mut W {
        self.variant(PRSCW::PRSC_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPMODE`"]
pub enum COMPMODEW {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
    COMPMODE_0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    COMPMODE_1,
}
impl COMPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPMODEW::COMPMODE_0 => false,
            COMPMODEW::COMPMODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
    #[inline]
    pub fn compmode_0(self) -> &'a mut W {
        self.variant(COMPMODEW::COMPMODE_0)
    }
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    #[inline]
    pub fn compmode_1(self) -> &'a mut W {
        self.variant(COMPMODEW::COMPMODE_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FULL`"]
pub enum FULLW {
    #[doc = "Full-cycle reloads disabled."]
    FULL_0,
    #[doc = "Full-cycle reloads enabled."]
    FULL_1,
}
impl FULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FULLW::FULL_0 => false,
            FULLW::FULL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full-cycle reloads disabled."]
    #[inline]
    pub fn full_0(self) -> &'a mut W {
        self.variant(FULLW::FULL_0)
    }
    #[doc = "Full-cycle reloads enabled."]
    #[inline]
    pub fn full_1(self) -> &'a mut W {
        self.variant(FULLW::FULL_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HALF`"]
pub enum HALFW {
    #[doc = "Half-cycle reloads disabled."]
    HALF_0,
    #[doc = "Half-cycle reloads enabled."]
    HALF_1,
}
impl HALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HALFW::HALF_0 => false,
            HALFW::HALF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALFW<'a> {
    w: &'a mut W,
}
impl<'a> _HALFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half-cycle reloads disabled."]
    #[inline]
    pub fn half_0(self) -> &'a mut W {
        self.variant(HALFW::HALF_0)
    }
    #[doc = "Half-cycle reloads enabled."]
    #[inline]
    pub fn half_1(self) -> &'a mut W {
        self.variant(HALFW::HALF_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LDFQ`"]
pub enum LDFQW {
    #[doc = "Every PWM opportunity"]
    LDFQ_0,
    #[doc = "Every 2 PWM opportunities"]
    LDFQ_1,
    #[doc = "Every 3 PWM opportunities"]
    LDFQ_2,
    #[doc = "Every 4 PWM opportunities"]
    LDFQ_3,
    #[doc = "Every 5 PWM opportunities"]
    LDFQ_4,
    #[doc = "Every 6 PWM opportunities"]
    LDFQ_5,
    #[doc = "Every 7 PWM opportunities"]
    LDFQ_6,
    #[doc = "Every 8 PWM opportunities"]
    LDFQ_7,
    #[doc = "Every 9 PWM opportunities"]
    LDFQ_8,
    #[doc = "Every 10 PWM opportunities"]
    LDFQ_9,
    #[doc = "Every 11 PWM opportunities"]
    LDFQ_10,
    #[doc = "Every 12 PWM opportunities"]
    LDFQ_11,
    #[doc = "Every 13 PWM opportunities"]
    LDFQ_12,
    #[doc = "Every 14 PWM opportunities"]
    LDFQ_13,
    #[doc = "Every 15 PWM opportunities"]
    LDFQ_14,
    #[doc = "Every 16 PWM opportunities"]
    LDFQ_15,
}
impl LDFQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDFQW::LDFQ_0 => 0,
            LDFQW::LDFQ_1 => 1,
            LDFQW::LDFQ_2 => 2,
            LDFQW::LDFQ_3 => 3,
            LDFQW::LDFQ_4 => 4,
            LDFQW::LDFQ_5 => 5,
            LDFQW::LDFQ_6 => 6,
            LDFQW::LDFQ_7 => 7,
            LDFQW::LDFQ_8 => 8,
            LDFQW::LDFQ_9 => 9,
            LDFQW::LDFQ_10 => 10,
            LDFQW::LDFQ_11 => 11,
            LDFQW::LDFQ_12 => 12,
            LDFQW::LDFQ_13 => 13,
            LDFQW::LDFQ_14 => 14,
            LDFQW::LDFQ_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDFQW<'a> {
    w: &'a mut W,
}
impl<'a> _LDFQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDFQW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Every PWM opportunity"]
    #[inline]
    pub fn ldfq_0(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_0)
    }
    #[doc = "Every 2 PWM opportunities"]
    #[inline]
    pub fn ldfq_1(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_1)
    }
    #[doc = "Every 3 PWM opportunities"]
    #[inline]
    pub fn ldfq_2(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_2)
    }
    #[doc = "Every 4 PWM opportunities"]
    #[inline]
    pub fn ldfq_3(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_3)
    }
    #[doc = "Every 5 PWM opportunities"]
    #[inline]
    pub fn ldfq_4(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_4)
    }
    #[doc = "Every 6 PWM opportunities"]
    #[inline]
    pub fn ldfq_5(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_5)
    }
    #[doc = "Every 7 PWM opportunities"]
    #[inline]
    pub fn ldfq_6(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_6)
    }
    #[doc = "Every 8 PWM opportunities"]
    #[inline]
    pub fn ldfq_7(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_7)
    }
    #[doc = "Every 9 PWM opportunities"]
    #[inline]
    pub fn ldfq_8(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_8)
    }
    #[doc = "Every 10 PWM opportunities"]
    #[inline]
    pub fn ldfq_9(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_9)
    }
    #[doc = "Every 11 PWM opportunities"]
    #[inline]
    pub fn ldfq_10(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_10)
    }
    #[doc = "Every 12 PWM opportunities"]
    #[inline]
    pub fn ldfq_11(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_11)
    }
    #[doc = "Every 13 PWM opportunities"]
    #[inline]
    pub fn ldfq_12(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_12)
    }
    #[doc = "Every 14 PWM opportunities"]
    #[inline]
    pub fn ldfq_13(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_13)
    }
    #[doc = "Every 15 PWM opportunities"]
    #[inline]
    pub fn ldfq_14(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_14)
    }
    #[doc = "Every 16 PWM opportunities"]
    #[inline]
    pub fn ldfq_15(self) -> &'a mut W {
        self.variant(LDFQW::LDFQ_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Double Switching Enable"]
    #[inline]
    pub fn dblen(&self) -> DBLENR {
        DBLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - PWMX Double Switching Enable"]
    #[inline]
    pub fn dblx(&self) -> DBLXR {
        DBLXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Load Mode Select"]
    #[inline]
    pub fn ldmod(&self) -> LDMODR {
        LDMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Split the DBLPWM signal to PWMA and PWMB"]
    #[inline]
    pub fn split(&self) -> SPLITR {
        SPLITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:6 - Prescaler"]
    #[inline]
    pub fn prsc(&self) -> PRSCR {
        PRSCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 7 - Compare Mode"]
    #[inline]
    pub fn compmode(&self) -> COMPMODER {
        COMPMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:9 - Deadtime"]
    #[inline]
    pub fn dt(&self) -> DTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        DTR { bits }
    }
    #[doc = "Bit 10 - Full Cycle Reload"]
    #[inline]
    pub fn full(&self) -> FULLR {
        FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Half Cycle Reload"]
    #[inline]
    pub fn half(&self) -> HALFR {
        HALFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 12:15 - Load Frequency"]
    #[inline]
    pub fn ldfq(&self) -> LDFQR {
        LDFQR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1024 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Double Switching Enable"]
    #[inline]
    pub fn dblen(&mut self) -> _DBLENW {
        _DBLENW { w: self }
    }
    #[doc = "Bit 1 - PWMX Double Switching Enable"]
    #[inline]
    pub fn dblx(&mut self) -> _DBLXW {
        _DBLXW { w: self }
    }
    #[doc = "Bit 2 - Load Mode Select"]
    #[inline]
    pub fn ldmod(&mut self) -> _LDMODW {
        _LDMODW { w: self }
    }
    #[doc = "Bit 3 - Split the DBLPWM signal to PWMA and PWMB"]
    #[inline]
    pub fn split(&mut self) -> _SPLITW {
        _SPLITW { w: self }
    }
    #[doc = "Bits 4:6 - Prescaler"]
    #[inline]
    pub fn prsc(&mut self) -> _PRSCW {
        _PRSCW { w: self }
    }
    #[doc = "Bit 7 - Compare Mode"]
    #[inline]
    pub fn compmode(&mut self) -> _COMPMODEW {
        _COMPMODEW { w: self }
    }
    #[doc = "Bit 10 - Full Cycle Reload"]
    #[inline]
    pub fn full(&mut self) -> _FULLW {
        _FULLW { w: self }
    }
    #[doc = "Bit 11 - Half Cycle Reload"]
    #[inline]
    pub fn half(&mut self) -> _HALFW {
        _HALFW { w: self }
    }
    #[doc = "Bits 12:15 - Load Frequency"]
    #[inline]
    pub fn ldfq(&mut self) -> _LDFQW {
        _LDFQW { w: self }
    }
}
