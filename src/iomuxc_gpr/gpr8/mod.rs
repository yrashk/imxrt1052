#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR8 {
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
#[doc = "Possible values of the field `LPI2C1_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPI2C1_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C1_IPG_STOP_MODE_1,
}
impl LPI2C1_IPG_STOP_MODER {
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
            LPI2C1_IPG_STOP_MODER::LPI2C1_IPG_STOP_MODE_0 => false,
            LPI2C1_IPG_STOP_MODER::LPI2C1_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C1_IPG_STOP_MODER {
        match value {
            false => LPI2C1_IPG_STOP_MODER::LPI2C1_IPG_STOP_MODE_0,
            true => LPI2C1_IPG_STOP_MODER::LPI2C1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpi2c1_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C1_IPG_STOP_MODER::LPI2C1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpi2c1_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C1_IPG_STOP_MODER::LPI2C1_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPI2C1_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPI2C1_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C1_IPG_DOZE_1,
}
impl LPI2C1_IPG_DOZER {
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
            LPI2C1_IPG_DOZER::LPI2C1_IPG_DOZE_0 => false,
            LPI2C1_IPG_DOZER::LPI2C1_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C1_IPG_DOZER {
        match value {
            false => LPI2C1_IPG_DOZER::LPI2C1_IPG_DOZE_0,
            true => LPI2C1_IPG_DOZER::LPI2C1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpi2c1_ipg_doze_0(&self) -> bool {
        *self == LPI2C1_IPG_DOZER::LPI2C1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpi2c1_ipg_doze_1(&self) -> bool {
        *self == LPI2C1_IPG_DOZER::LPI2C1_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPI2C2_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPI2C2_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C2_IPG_STOP_MODE_1,
}
impl LPI2C2_IPG_STOP_MODER {
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
            LPI2C2_IPG_STOP_MODER::LPI2C2_IPG_STOP_MODE_0 => false,
            LPI2C2_IPG_STOP_MODER::LPI2C2_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C2_IPG_STOP_MODER {
        match value {
            false => LPI2C2_IPG_STOP_MODER::LPI2C2_IPG_STOP_MODE_0,
            true => LPI2C2_IPG_STOP_MODER::LPI2C2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpi2c2_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C2_IPG_STOP_MODER::LPI2C2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpi2c2_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C2_IPG_STOP_MODER::LPI2C2_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPI2C2_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPI2C2_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C2_IPG_DOZE_1,
}
impl LPI2C2_IPG_DOZER {
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
            LPI2C2_IPG_DOZER::LPI2C2_IPG_DOZE_0 => false,
            LPI2C2_IPG_DOZER::LPI2C2_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C2_IPG_DOZER {
        match value {
            false => LPI2C2_IPG_DOZER::LPI2C2_IPG_DOZE_0,
            true => LPI2C2_IPG_DOZER::LPI2C2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpi2c2_ipg_doze_0(&self) -> bool {
        *self == LPI2C2_IPG_DOZER::LPI2C2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpi2c2_ipg_doze_1(&self) -> bool {
        *self == LPI2C2_IPG_DOZER::LPI2C2_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPI2C3_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPI2C3_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C3_IPG_STOP_MODE_1,
}
impl LPI2C3_IPG_STOP_MODER {
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
            LPI2C3_IPG_STOP_MODER::LPI2C3_IPG_STOP_MODE_0 => false,
            LPI2C3_IPG_STOP_MODER::LPI2C3_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C3_IPG_STOP_MODER {
        match value {
            false => LPI2C3_IPG_STOP_MODER::LPI2C3_IPG_STOP_MODE_0,
            true => LPI2C3_IPG_STOP_MODER::LPI2C3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpi2c3_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C3_IPG_STOP_MODER::LPI2C3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpi2c3_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C3_IPG_STOP_MODER::LPI2C3_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPI2C3_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPI2C3_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C3_IPG_DOZE_1,
}
impl LPI2C3_IPG_DOZER {
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
            LPI2C3_IPG_DOZER::LPI2C3_IPG_DOZE_0 => false,
            LPI2C3_IPG_DOZER::LPI2C3_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C3_IPG_DOZER {
        match value {
            false => LPI2C3_IPG_DOZER::LPI2C3_IPG_DOZE_0,
            true => LPI2C3_IPG_DOZER::LPI2C3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpi2c3_ipg_doze_0(&self) -> bool {
        *self == LPI2C3_IPG_DOZER::LPI2C3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpi2c3_ipg_doze_1(&self) -> bool {
        *self == LPI2C3_IPG_DOZER::LPI2C3_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPI2C4_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPI2C4_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C4_IPG_STOP_MODE_1,
}
impl LPI2C4_IPG_STOP_MODER {
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
            LPI2C4_IPG_STOP_MODER::LPI2C4_IPG_STOP_MODE_0 => false,
            LPI2C4_IPG_STOP_MODER::LPI2C4_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C4_IPG_STOP_MODER {
        match value {
            false => LPI2C4_IPG_STOP_MODER::LPI2C4_IPG_STOP_MODE_0,
            true => LPI2C4_IPG_STOP_MODER::LPI2C4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpi2c4_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C4_IPG_STOP_MODER::LPI2C4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpi2c4_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C4_IPG_STOP_MODER::LPI2C4_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPI2C4_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPI2C4_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C4_IPG_DOZE_1,
}
impl LPI2C4_IPG_DOZER {
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
            LPI2C4_IPG_DOZER::LPI2C4_IPG_DOZE_0 => false,
            LPI2C4_IPG_DOZER::LPI2C4_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C4_IPG_DOZER {
        match value {
            false => LPI2C4_IPG_DOZER::LPI2C4_IPG_DOZE_0,
            true => LPI2C4_IPG_DOZER::LPI2C4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpi2c4_ipg_doze_0(&self) -> bool {
        *self == LPI2C4_IPG_DOZER::LPI2C4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpi2c4_ipg_doze_1(&self) -> bool {
        *self == LPI2C4_IPG_DOZER::LPI2C4_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPSPI1_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPSPI1_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI1_IPG_STOP_MODE_1,
}
impl LPSPI1_IPG_STOP_MODER {
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
            LPSPI1_IPG_STOP_MODER::LPSPI1_IPG_STOP_MODE_0 => false,
            LPSPI1_IPG_STOP_MODER::LPSPI1_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI1_IPG_STOP_MODER {
        match value {
            false => LPSPI1_IPG_STOP_MODER::LPSPI1_IPG_STOP_MODE_0,
            true => LPSPI1_IPG_STOP_MODER::LPSPI1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpspi1_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI1_IPG_STOP_MODER::LPSPI1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpspi1_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI1_IPG_STOP_MODER::LPSPI1_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPSPI1_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPSPI1_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI1_IPG_DOZE_1,
}
impl LPSPI1_IPG_DOZER {
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
            LPSPI1_IPG_DOZER::LPSPI1_IPG_DOZE_0 => false,
            LPSPI1_IPG_DOZER::LPSPI1_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI1_IPG_DOZER {
        match value {
            false => LPSPI1_IPG_DOZER::LPSPI1_IPG_DOZE_0,
            true => LPSPI1_IPG_DOZER::LPSPI1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpspi1_ipg_doze_0(&self) -> bool {
        *self == LPSPI1_IPG_DOZER::LPSPI1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpspi1_ipg_doze_1(&self) -> bool {
        *self == LPSPI1_IPG_DOZER::LPSPI1_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPSPI2_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPSPI2_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI2_IPG_STOP_MODE_1,
}
impl LPSPI2_IPG_STOP_MODER {
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
            LPSPI2_IPG_STOP_MODER::LPSPI2_IPG_STOP_MODE_0 => false,
            LPSPI2_IPG_STOP_MODER::LPSPI2_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI2_IPG_STOP_MODER {
        match value {
            false => LPSPI2_IPG_STOP_MODER::LPSPI2_IPG_STOP_MODE_0,
            true => LPSPI2_IPG_STOP_MODER::LPSPI2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpspi2_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI2_IPG_STOP_MODER::LPSPI2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpspi2_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI2_IPG_STOP_MODER::LPSPI2_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPSPI2_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPSPI2_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI2_IPG_DOZE_1,
}
impl LPSPI2_IPG_DOZER {
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
            LPSPI2_IPG_DOZER::LPSPI2_IPG_DOZE_0 => false,
            LPSPI2_IPG_DOZER::LPSPI2_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI2_IPG_DOZER {
        match value {
            false => LPSPI2_IPG_DOZER::LPSPI2_IPG_DOZE_0,
            true => LPSPI2_IPG_DOZER::LPSPI2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpspi2_ipg_doze_0(&self) -> bool {
        *self == LPSPI2_IPG_DOZER::LPSPI2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpspi2_ipg_doze_1(&self) -> bool {
        *self == LPSPI2_IPG_DOZER::LPSPI2_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPSPI3_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPSPI3_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI3_IPG_STOP_MODE_1,
}
impl LPSPI3_IPG_STOP_MODER {
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
            LPSPI3_IPG_STOP_MODER::LPSPI3_IPG_STOP_MODE_0 => false,
            LPSPI3_IPG_STOP_MODER::LPSPI3_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI3_IPG_STOP_MODER {
        match value {
            false => LPSPI3_IPG_STOP_MODER::LPSPI3_IPG_STOP_MODE_0,
            true => LPSPI3_IPG_STOP_MODER::LPSPI3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpspi3_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI3_IPG_STOP_MODER::LPSPI3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpspi3_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI3_IPG_STOP_MODER::LPSPI3_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPSPI3_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPSPI3_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI3_IPG_DOZE_1,
}
impl LPSPI3_IPG_DOZER {
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
            LPSPI3_IPG_DOZER::LPSPI3_IPG_DOZE_0 => false,
            LPSPI3_IPG_DOZER::LPSPI3_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI3_IPG_DOZER {
        match value {
            false => LPSPI3_IPG_DOZER::LPSPI3_IPG_DOZE_0,
            true => LPSPI3_IPG_DOZER::LPSPI3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpspi3_ipg_doze_0(&self) -> bool {
        *self == LPSPI3_IPG_DOZER::LPSPI3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpspi3_ipg_doze_1(&self) -> bool {
        *self == LPSPI3_IPG_DOZER::LPSPI3_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPSPI4_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPSPI4_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI4_IPG_STOP_MODE_1,
}
impl LPSPI4_IPG_STOP_MODER {
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
            LPSPI4_IPG_STOP_MODER::LPSPI4_IPG_STOP_MODE_0 => false,
            LPSPI4_IPG_STOP_MODER::LPSPI4_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI4_IPG_STOP_MODER {
        match value {
            false => LPSPI4_IPG_STOP_MODER::LPSPI4_IPG_STOP_MODE_0,
            true => LPSPI4_IPG_STOP_MODER::LPSPI4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpspi4_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI4_IPG_STOP_MODER::LPSPI4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpspi4_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI4_IPG_STOP_MODER::LPSPI4_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPSPI4_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPSPI4_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI4_IPG_DOZE_1,
}
impl LPSPI4_IPG_DOZER {
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
            LPSPI4_IPG_DOZER::LPSPI4_IPG_DOZE_0 => false,
            LPSPI4_IPG_DOZER::LPSPI4_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI4_IPG_DOZER {
        match value {
            false => LPSPI4_IPG_DOZER::LPSPI4_IPG_DOZE_0,
            true => LPSPI4_IPG_DOZER::LPSPI4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpspi4_ipg_doze_0(&self) -> bool {
        *self == LPSPI4_IPG_DOZER::LPSPI4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpspi4_ipg_doze_1(&self) -> bool {
        *self == LPSPI4_IPG_DOZER::LPSPI4_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART1_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART1_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART1_IPG_STOP_MODE_1,
}
impl LPUART1_IPG_STOP_MODER {
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
            LPUART1_IPG_STOP_MODER::LPUART1_IPG_STOP_MODE_0 => false,
            LPUART1_IPG_STOP_MODER::LPUART1_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART1_IPG_STOP_MODER {
        match value {
            false => LPUART1_IPG_STOP_MODER::LPUART1_IPG_STOP_MODE_0,
            true => LPUART1_IPG_STOP_MODER::LPUART1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart1_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART1_IPG_STOP_MODER::LPUART1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart1_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART1_IPG_STOP_MODER::LPUART1_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART1_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART1_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART1_IPG_DOZE_1,
}
impl LPUART1_IPG_DOZER {
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
            LPUART1_IPG_DOZER::LPUART1_IPG_DOZE_0 => false,
            LPUART1_IPG_DOZER::LPUART1_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART1_IPG_DOZER {
        match value {
            false => LPUART1_IPG_DOZER::LPUART1_IPG_DOZE_0,
            true => LPUART1_IPG_DOZER::LPUART1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart1_ipg_doze_0(&self) -> bool {
        *self == LPUART1_IPG_DOZER::LPUART1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart1_ipg_doze_1(&self) -> bool {
        *self == LPUART1_IPG_DOZER::LPUART1_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART2_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART2_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART2_IPG_STOP_MODE_1,
}
impl LPUART2_IPG_STOP_MODER {
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
            LPUART2_IPG_STOP_MODER::LPUART2_IPG_STOP_MODE_0 => false,
            LPUART2_IPG_STOP_MODER::LPUART2_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART2_IPG_STOP_MODER {
        match value {
            false => LPUART2_IPG_STOP_MODER::LPUART2_IPG_STOP_MODE_0,
            true => LPUART2_IPG_STOP_MODER::LPUART2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart2_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART2_IPG_STOP_MODER::LPUART2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart2_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART2_IPG_STOP_MODER::LPUART2_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART2_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART2_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART2_IPG_DOZE_1,
}
impl LPUART2_IPG_DOZER {
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
            LPUART2_IPG_DOZER::LPUART2_IPG_DOZE_0 => false,
            LPUART2_IPG_DOZER::LPUART2_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART2_IPG_DOZER {
        match value {
            false => LPUART2_IPG_DOZER::LPUART2_IPG_DOZE_0,
            true => LPUART2_IPG_DOZER::LPUART2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart2_ipg_doze_0(&self) -> bool {
        *self == LPUART2_IPG_DOZER::LPUART2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart2_ipg_doze_1(&self) -> bool {
        *self == LPUART2_IPG_DOZER::LPUART2_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART3_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART3_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART3_IPG_STOP_MODE_1,
}
impl LPUART3_IPG_STOP_MODER {
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
            LPUART3_IPG_STOP_MODER::LPUART3_IPG_STOP_MODE_0 => false,
            LPUART3_IPG_STOP_MODER::LPUART3_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART3_IPG_STOP_MODER {
        match value {
            false => LPUART3_IPG_STOP_MODER::LPUART3_IPG_STOP_MODE_0,
            true => LPUART3_IPG_STOP_MODER::LPUART3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart3_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART3_IPG_STOP_MODER::LPUART3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart3_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART3_IPG_STOP_MODER::LPUART3_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART3_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART3_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART3_IPG_DOZE_1,
}
impl LPUART3_IPG_DOZER {
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
            LPUART3_IPG_DOZER::LPUART3_IPG_DOZE_0 => false,
            LPUART3_IPG_DOZER::LPUART3_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART3_IPG_DOZER {
        match value {
            false => LPUART3_IPG_DOZER::LPUART3_IPG_DOZE_0,
            true => LPUART3_IPG_DOZER::LPUART3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart3_ipg_doze_0(&self) -> bool {
        *self == LPUART3_IPG_DOZER::LPUART3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart3_ipg_doze_1(&self) -> bool {
        *self == LPUART3_IPG_DOZER::LPUART3_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART4_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART4_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART4_IPG_STOP_MODE_1,
}
impl LPUART4_IPG_STOP_MODER {
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
            LPUART4_IPG_STOP_MODER::LPUART4_IPG_STOP_MODE_0 => false,
            LPUART4_IPG_STOP_MODER::LPUART4_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART4_IPG_STOP_MODER {
        match value {
            false => LPUART4_IPG_STOP_MODER::LPUART4_IPG_STOP_MODE_0,
            true => LPUART4_IPG_STOP_MODER::LPUART4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart4_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART4_IPG_STOP_MODER::LPUART4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart4_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART4_IPG_STOP_MODER::LPUART4_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART4_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART4_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART4_IPG_DOZE_1,
}
impl LPUART4_IPG_DOZER {
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
            LPUART4_IPG_DOZER::LPUART4_IPG_DOZE_0 => false,
            LPUART4_IPG_DOZER::LPUART4_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART4_IPG_DOZER {
        match value {
            false => LPUART4_IPG_DOZER::LPUART4_IPG_DOZE_0,
            true => LPUART4_IPG_DOZER::LPUART4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart4_ipg_doze_0(&self) -> bool {
        *self == LPUART4_IPG_DOZER::LPUART4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart4_ipg_doze_1(&self) -> bool {
        *self == LPUART4_IPG_DOZER::LPUART4_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART5_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART5_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART5_IPG_STOP_MODE_1,
}
impl LPUART5_IPG_STOP_MODER {
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
            LPUART5_IPG_STOP_MODER::LPUART5_IPG_STOP_MODE_0 => false,
            LPUART5_IPG_STOP_MODER::LPUART5_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART5_IPG_STOP_MODER {
        match value {
            false => LPUART5_IPG_STOP_MODER::LPUART5_IPG_STOP_MODE_0,
            true => LPUART5_IPG_STOP_MODER::LPUART5_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart5_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART5_IPG_STOP_MODER::LPUART5_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart5_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART5_IPG_STOP_MODER::LPUART5_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART5_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART5_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART5_IPG_DOZE_1,
}
impl LPUART5_IPG_DOZER {
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
            LPUART5_IPG_DOZER::LPUART5_IPG_DOZE_0 => false,
            LPUART5_IPG_DOZER::LPUART5_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART5_IPG_DOZER {
        match value {
            false => LPUART5_IPG_DOZER::LPUART5_IPG_DOZE_0,
            true => LPUART5_IPG_DOZER::LPUART5_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart5_ipg_doze_0(&self) -> bool {
        *self == LPUART5_IPG_DOZER::LPUART5_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart5_ipg_doze_1(&self) -> bool {
        *self == LPUART5_IPG_DOZER::LPUART5_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART6_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART6_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART6_IPG_STOP_MODE_1,
}
impl LPUART6_IPG_STOP_MODER {
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
            LPUART6_IPG_STOP_MODER::LPUART6_IPG_STOP_MODE_0 => false,
            LPUART6_IPG_STOP_MODER::LPUART6_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART6_IPG_STOP_MODER {
        match value {
            false => LPUART6_IPG_STOP_MODER::LPUART6_IPG_STOP_MODE_0,
            true => LPUART6_IPG_STOP_MODER::LPUART6_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart6_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART6_IPG_STOP_MODER::LPUART6_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart6_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART6_IPG_STOP_MODER::LPUART6_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART6_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART6_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART6_IPG_DOZE_1,
}
impl LPUART6_IPG_DOZER {
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
            LPUART6_IPG_DOZER::LPUART6_IPG_DOZE_0 => false,
            LPUART6_IPG_DOZER::LPUART6_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART6_IPG_DOZER {
        match value {
            false => LPUART6_IPG_DOZER::LPUART6_IPG_DOZE_0,
            true => LPUART6_IPG_DOZER::LPUART6_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart6_ipg_doze_0(&self) -> bool {
        *self == LPUART6_IPG_DOZER::LPUART6_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart6_ipg_doze_1(&self) -> bool {
        *self == LPUART6_IPG_DOZER::LPUART6_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART7_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART7_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART7_IPG_STOP_MODE_1,
}
impl LPUART7_IPG_STOP_MODER {
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
            LPUART7_IPG_STOP_MODER::LPUART7_IPG_STOP_MODE_0 => false,
            LPUART7_IPG_STOP_MODER::LPUART7_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART7_IPG_STOP_MODER {
        match value {
            false => LPUART7_IPG_STOP_MODER::LPUART7_IPG_STOP_MODE_0,
            true => LPUART7_IPG_STOP_MODER::LPUART7_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart7_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART7_IPG_STOP_MODER::LPUART7_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart7_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART7_IPG_STOP_MODER::LPUART7_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART7_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART7_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART7_IPG_DOZE_1,
}
impl LPUART7_IPG_DOZER {
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
            LPUART7_IPG_DOZER::LPUART7_IPG_DOZE_0 => false,
            LPUART7_IPG_DOZER::LPUART7_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART7_IPG_DOZER {
        match value {
            false => LPUART7_IPG_DOZER::LPUART7_IPG_DOZE_0,
            true => LPUART7_IPG_DOZER::LPUART7_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart7_ipg_doze_0(&self) -> bool {
        *self == LPUART7_IPG_DOZER::LPUART7_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart7_ipg_doze_1(&self) -> bool {
        *self == LPUART7_IPG_DOZER::LPUART7_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `LPUART8_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_IPG_STOP_MODER {
    #[doc = "the module is functional in Stop mode"]
    LPUART8_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART8_IPG_STOP_MODE_1,
}
impl LPUART8_IPG_STOP_MODER {
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
            LPUART8_IPG_STOP_MODER::LPUART8_IPG_STOP_MODE_0 => false,
            LPUART8_IPG_STOP_MODER::LPUART8_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART8_IPG_STOP_MODER {
        match value {
            false => LPUART8_IPG_STOP_MODER::LPUART8_IPG_STOP_MODE_0,
            true => LPUART8_IPG_STOP_MODER::LPUART8_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_lpuart8_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART8_IPG_STOP_MODER::LPUART8_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_lpuart8_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART8_IPG_STOP_MODER::LPUART8_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `LPUART8_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_IPG_DOZER {
    #[doc = "not in doze mode"]
    LPUART8_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART8_IPG_DOZE_1,
}
impl LPUART8_IPG_DOZER {
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
            LPUART8_IPG_DOZER::LPUART8_IPG_DOZE_0 => false,
            LPUART8_IPG_DOZER::LPUART8_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART8_IPG_DOZER {
        match value {
            false => LPUART8_IPG_DOZER::LPUART8_IPG_DOZE_0,
            true => LPUART8_IPG_DOZER::LPUART8_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_DOZE_0`"]
    #[inline]
    pub fn is_lpuart8_ipg_doze_0(&self) -> bool {
        *self == LPUART8_IPG_DOZER::LPUART8_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_DOZE_1`"]
    #[inline]
    pub fn is_lpuart8_ipg_doze_1(&self) -> bool {
        *self == LPUART8_IPG_DOZER::LPUART8_IPG_DOZE_1
    }
}
#[doc = "Values that can be written to the field `LPI2C1_IPG_STOP_MODE`"]
pub enum LPI2C1_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPI2C1_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C1_IPG_STOP_MODE_1,
}
impl LPI2C1_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C1_IPG_STOP_MODEW::LPI2C1_IPG_STOP_MODE_0 => false,
            LPI2C1_IPG_STOP_MODEW::LPI2C1_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C1_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C1_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C1_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpi2c1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_STOP_MODEW::LPI2C1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpi2c1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_STOP_MODEW::LPI2C1_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `LPI2C1_IPG_DOZE`"]
pub enum LPI2C1_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPI2C1_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C1_IPG_DOZE_1,
}
impl LPI2C1_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C1_IPG_DOZEW::LPI2C1_IPG_DOZE_0 => false,
            LPI2C1_IPG_DOZEW::LPI2C1_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C1_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C1_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C1_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpi2c1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_DOZEW::LPI2C1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpi2c1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_DOZEW::LPI2C1_IPG_DOZE_1)
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
#[doc = "Values that can be written to the field `LPI2C2_IPG_STOP_MODE`"]
pub enum LPI2C2_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPI2C2_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C2_IPG_STOP_MODE_1,
}
impl LPI2C2_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C2_IPG_STOP_MODEW::LPI2C2_IPG_STOP_MODE_0 => false,
            LPI2C2_IPG_STOP_MODEW::LPI2C2_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C2_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C2_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C2_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpi2c2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_STOP_MODEW::LPI2C2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpi2c2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_STOP_MODEW::LPI2C2_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `LPI2C2_IPG_DOZE`"]
pub enum LPI2C2_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPI2C2_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C2_IPG_DOZE_1,
}
impl LPI2C2_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C2_IPG_DOZEW::LPI2C2_IPG_DOZE_0 => false,
            LPI2C2_IPG_DOZEW::LPI2C2_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C2_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C2_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C2_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpi2c2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_DOZEW::LPI2C2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpi2c2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_DOZEW::LPI2C2_IPG_DOZE_1)
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
#[doc = "Values that can be written to the field `LPI2C3_IPG_STOP_MODE`"]
pub enum LPI2C3_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPI2C3_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C3_IPG_STOP_MODE_1,
}
impl LPI2C3_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C3_IPG_STOP_MODEW::LPI2C3_IPG_STOP_MODE_0 => false,
            LPI2C3_IPG_STOP_MODEW::LPI2C3_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C3_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C3_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C3_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpi2c3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_STOP_MODEW::LPI2C3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpi2c3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_STOP_MODEW::LPI2C3_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `LPI2C3_IPG_DOZE`"]
pub enum LPI2C3_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPI2C3_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C3_IPG_DOZE_1,
}
impl LPI2C3_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C3_IPG_DOZEW::LPI2C3_IPG_DOZE_0 => false,
            LPI2C3_IPG_DOZEW::LPI2C3_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C3_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C3_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C3_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpi2c3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_DOZEW::LPI2C3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpi2c3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_DOZEW::LPI2C3_IPG_DOZE_1)
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
#[doc = "Values that can be written to the field `LPI2C4_IPG_STOP_MODE`"]
pub enum LPI2C4_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPI2C4_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C4_IPG_STOP_MODE_1,
}
impl LPI2C4_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C4_IPG_STOP_MODEW::LPI2C4_IPG_STOP_MODE_0 => false,
            LPI2C4_IPG_STOP_MODEW::LPI2C4_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C4_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C4_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C4_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpi2c4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_STOP_MODEW::LPI2C4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpi2c4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_STOP_MODEW::LPI2C4_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `LPI2C4_IPG_DOZE`"]
pub enum LPI2C4_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPI2C4_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPI2C4_IPG_DOZE_1,
}
impl LPI2C4_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C4_IPG_DOZEW::LPI2C4_IPG_DOZE_0 => false,
            LPI2C4_IPG_DOZEW::LPI2C4_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C4_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C4_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C4_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpi2c4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_DOZEW::LPI2C4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpi2c4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_DOZEW::LPI2C4_IPG_DOZE_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSPI1_IPG_STOP_MODE`"]
pub enum LPSPI1_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPSPI1_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI1_IPG_STOP_MODE_1,
}
impl LPSPI1_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI1_IPG_STOP_MODEW::LPSPI1_IPG_STOP_MODE_0 => false,
            LPSPI1_IPG_STOP_MODEW::LPSPI1_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI1_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI1_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI1_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpspi1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_STOP_MODEW::LPSPI1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpspi1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_STOP_MODEW::LPSPI1_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `LPSPI1_IPG_DOZE`"]
pub enum LPSPI1_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPSPI1_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI1_IPG_DOZE_1,
}
impl LPSPI1_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI1_IPG_DOZEW::LPSPI1_IPG_DOZE_0 => false,
            LPSPI1_IPG_DOZEW::LPSPI1_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI1_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI1_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI1_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpspi1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_DOZEW::LPSPI1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpspi1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_DOZEW::LPSPI1_IPG_DOZE_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSPI2_IPG_STOP_MODE`"]
pub enum LPSPI2_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPSPI2_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI2_IPG_STOP_MODE_1,
}
impl LPSPI2_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI2_IPG_STOP_MODEW::LPSPI2_IPG_STOP_MODE_0 => false,
            LPSPI2_IPG_STOP_MODEW::LPSPI2_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI2_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI2_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI2_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpspi2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_STOP_MODEW::LPSPI2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpspi2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_STOP_MODEW::LPSPI2_IPG_STOP_MODE_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSPI2_IPG_DOZE`"]
pub enum LPSPI2_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPSPI2_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI2_IPG_DOZE_1,
}
impl LPSPI2_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI2_IPG_DOZEW::LPSPI2_IPG_DOZE_0 => false,
            LPSPI2_IPG_DOZEW::LPSPI2_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI2_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI2_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI2_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpspi2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_DOZEW::LPSPI2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpspi2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_DOZEW::LPSPI2_IPG_DOZE_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSPI3_IPG_STOP_MODE`"]
pub enum LPSPI3_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPSPI3_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI3_IPG_STOP_MODE_1,
}
impl LPSPI3_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI3_IPG_STOP_MODEW::LPSPI3_IPG_STOP_MODE_0 => false,
            LPSPI3_IPG_STOP_MODEW::LPSPI3_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI3_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI3_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI3_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpspi3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_STOP_MODEW::LPSPI3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpspi3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_STOP_MODEW::LPSPI3_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `LPSPI3_IPG_DOZE`"]
pub enum LPSPI3_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPSPI3_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI3_IPG_DOZE_1,
}
impl LPSPI3_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI3_IPG_DOZEW::LPSPI3_IPG_DOZE_0 => false,
            LPSPI3_IPG_DOZEW::LPSPI3_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI3_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI3_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI3_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpspi3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_DOZEW::LPSPI3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpspi3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_DOZEW::LPSPI3_IPG_DOZE_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSPI4_IPG_STOP_MODE`"]
pub enum LPSPI4_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPSPI4_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI4_IPG_STOP_MODE_1,
}
impl LPSPI4_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI4_IPG_STOP_MODEW::LPSPI4_IPG_STOP_MODE_0 => false,
            LPSPI4_IPG_STOP_MODEW::LPSPI4_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI4_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI4_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI4_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpspi4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_STOP_MODEW::LPSPI4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpspi4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_STOP_MODEW::LPSPI4_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSPI4_IPG_DOZE`"]
pub enum LPSPI4_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPSPI4_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPSPI4_IPG_DOZE_1,
}
impl LPSPI4_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI4_IPG_DOZEW::LPSPI4_IPG_DOZE_0 => false,
            LPSPI4_IPG_DOZEW::LPSPI4_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI4_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI4_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI4_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpspi4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_DOZEW::LPSPI4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpspi4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_DOZEW::LPSPI4_IPG_DOZE_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART1_IPG_STOP_MODE`"]
pub enum LPUART1_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART1_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART1_IPG_STOP_MODE_1,
}
impl LPUART1_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART1_IPG_STOP_MODEW::LPUART1_IPG_STOP_MODE_0 => false,
            LPUART1_IPG_STOP_MODEW::LPUART1_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART1_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART1_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART1_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART1_IPG_STOP_MODEW::LPUART1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART1_IPG_STOP_MODEW::LPUART1_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `LPUART1_IPG_DOZE`"]
pub enum LPUART1_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART1_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART1_IPG_DOZE_1,
}
impl LPUART1_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART1_IPG_DOZEW::LPUART1_IPG_DOZE_0 => false,
            LPUART1_IPG_DOZEW::LPUART1_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART1_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART1_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART1_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART1_IPG_DOZEW::LPUART1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART1_IPG_DOZEW::LPUART1_IPG_DOZE_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART2_IPG_STOP_MODE`"]
pub enum LPUART2_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART2_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART2_IPG_STOP_MODE_1,
}
impl LPUART2_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART2_IPG_STOP_MODEW::LPUART2_IPG_STOP_MODE_0 => false,
            LPUART2_IPG_STOP_MODEW::LPUART2_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART2_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART2_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART2_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART2_IPG_STOP_MODEW::LPUART2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART2_IPG_STOP_MODEW::LPUART2_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART2_IPG_DOZE`"]
pub enum LPUART2_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART2_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART2_IPG_DOZE_1,
}
impl LPUART2_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART2_IPG_DOZEW::LPUART2_IPG_DOZE_0 => false,
            LPUART2_IPG_DOZEW::LPUART2_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART2_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART2_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART2_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART2_IPG_DOZEW::LPUART2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART2_IPG_DOZEW::LPUART2_IPG_DOZE_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART3_IPG_STOP_MODE`"]
pub enum LPUART3_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART3_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART3_IPG_STOP_MODE_1,
}
impl LPUART3_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART3_IPG_STOP_MODEW::LPUART3_IPG_STOP_MODE_0 => false,
            LPUART3_IPG_STOP_MODEW::LPUART3_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART3_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART3_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART3_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART3_IPG_STOP_MODEW::LPUART3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART3_IPG_STOP_MODEW::LPUART3_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART3_IPG_DOZE`"]
pub enum LPUART3_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART3_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART3_IPG_DOZE_1,
}
impl LPUART3_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART3_IPG_DOZEW::LPUART3_IPG_DOZE_0 => false,
            LPUART3_IPG_DOZEW::LPUART3_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART3_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART3_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART3_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART3_IPG_DOZEW::LPUART3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART3_IPG_DOZEW::LPUART3_IPG_DOZE_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART4_IPG_STOP_MODE`"]
pub enum LPUART4_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART4_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART4_IPG_STOP_MODE_1,
}
impl LPUART4_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART4_IPG_STOP_MODEW::LPUART4_IPG_STOP_MODE_0 => false,
            LPUART4_IPG_STOP_MODEW::LPUART4_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART4_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART4_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART4_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART4_IPG_STOP_MODEW::LPUART4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART4_IPG_STOP_MODEW::LPUART4_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART4_IPG_DOZE`"]
pub enum LPUART4_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART4_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART4_IPG_DOZE_1,
}
impl LPUART4_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART4_IPG_DOZEW::LPUART4_IPG_DOZE_0 => false,
            LPUART4_IPG_DOZEW::LPUART4_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART4_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART4_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART4_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART4_IPG_DOZEW::LPUART4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART4_IPG_DOZEW::LPUART4_IPG_DOZE_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART5_IPG_STOP_MODE`"]
pub enum LPUART5_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART5_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART5_IPG_STOP_MODE_1,
}
impl LPUART5_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART5_IPG_STOP_MODEW::LPUART5_IPG_STOP_MODE_0 => false,
            LPUART5_IPG_STOP_MODEW::LPUART5_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART5_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART5_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART5_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart5_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART5_IPG_STOP_MODEW::LPUART5_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart5_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART5_IPG_STOP_MODEW::LPUART5_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART5_IPG_DOZE`"]
pub enum LPUART5_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART5_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART5_IPG_DOZE_1,
}
impl LPUART5_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART5_IPG_DOZEW::LPUART5_IPG_DOZE_0 => false,
            LPUART5_IPG_DOZEW::LPUART5_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART5_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART5_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART5_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart5_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART5_IPG_DOZEW::LPUART5_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart5_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART5_IPG_DOZEW::LPUART5_IPG_DOZE_1)
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
#[doc = "Values that can be written to the field `LPUART6_IPG_STOP_MODE`"]
pub enum LPUART6_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART6_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART6_IPG_STOP_MODE_1,
}
impl LPUART6_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART6_IPG_STOP_MODEW::LPUART6_IPG_STOP_MODE_0 => false,
            LPUART6_IPG_STOP_MODEW::LPUART6_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART6_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART6_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART6_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart6_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART6_IPG_STOP_MODEW::LPUART6_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart6_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART6_IPG_STOP_MODEW::LPUART6_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART6_IPG_DOZE`"]
pub enum LPUART6_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART6_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART6_IPG_DOZE_1,
}
impl LPUART6_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART6_IPG_DOZEW::LPUART6_IPG_DOZE_0 => false,
            LPUART6_IPG_DOZEW::LPUART6_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART6_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART6_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART6_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart6_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART6_IPG_DOZEW::LPUART6_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart6_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART6_IPG_DOZEW::LPUART6_IPG_DOZE_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART7_IPG_STOP_MODE`"]
pub enum LPUART7_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART7_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART7_IPG_STOP_MODE_1,
}
impl LPUART7_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART7_IPG_STOP_MODEW::LPUART7_IPG_STOP_MODE_0 => false,
            LPUART7_IPG_STOP_MODEW::LPUART7_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART7_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART7_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART7_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart7_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART7_IPG_STOP_MODEW::LPUART7_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart7_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART7_IPG_STOP_MODEW::LPUART7_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART7_IPG_DOZE`"]
pub enum LPUART7_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART7_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART7_IPG_DOZE_1,
}
impl LPUART7_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART7_IPG_DOZEW::LPUART7_IPG_DOZE_0 => false,
            LPUART7_IPG_DOZEW::LPUART7_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART7_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART7_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART7_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart7_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART7_IPG_DOZEW::LPUART7_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart7_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART7_IPG_DOZEW::LPUART7_IPG_DOZE_1)
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
#[doc = "Values that can be written to the field `LPUART8_IPG_STOP_MODE`"]
pub enum LPUART8_IPG_STOP_MODEW {
    #[doc = "the module is functional in Stop mode"]
    LPUART8_IPG_STOP_MODE_0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART8_IPG_STOP_MODE_1,
}
impl LPUART8_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART8_IPG_STOP_MODEW::LPUART8_IPG_STOP_MODE_0 => false,
            LPUART8_IPG_STOP_MODEW::LPUART8_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART8_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART8_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART8_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline]
    pub fn lpuart8_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART8_IPG_STOP_MODEW::LPUART8_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline]
    pub fn lpuart8_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART8_IPG_STOP_MODEW::LPUART8_IPG_STOP_MODE_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART8_IPG_DOZE`"]
pub enum LPUART8_IPG_DOZEW {
    #[doc = "not in doze mode"]
    LPUART8_IPG_DOZE_0,
    #[doc = "in doze mode"]
    LPUART8_IPG_DOZE_1,
}
impl LPUART8_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART8_IPG_DOZEW::LPUART8_IPG_DOZE_0 => false,
            LPUART8_IPG_DOZEW::LPUART8_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART8_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART8_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART8_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "not in doze mode"]
    #[inline]
    pub fn lpuart8_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART8_IPG_DOZEW::LPUART8_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline]
    pub fn lpuart8_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART8_IPG_DOZEW::LPUART8_IPG_DOZE_1)
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
    #[doc = "Bit 0 - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c1_ipg_stop_mode(&self) -> LPI2C1_IPG_STOP_MODER {
        LPI2C1_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - LPI2C1 ipg_doze mode"]
    #[inline]
    pub fn lpi2c1_ipg_doze(&self) -> LPI2C1_IPG_DOZER {
        LPI2C1_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c2_ipg_stop_mode(&self) -> LPI2C2_IPG_STOP_MODER {
        LPI2C2_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - LPI2C2 ipg_doze mode"]
    #[inline]
    pub fn lpi2c2_ipg_doze(&self) -> LPI2C2_IPG_DOZER {
        LPI2C2_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c3_ipg_stop_mode(&self) -> LPI2C3_IPG_STOP_MODER {
        LPI2C3_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - LPI2C3 ipg_doze mode"]
    #[inline]
    pub fn lpi2c3_ipg_doze(&self) -> LPI2C3_IPG_DOZER {
        LPI2C3_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c4_ipg_stop_mode(&self) -> LPI2C4_IPG_STOP_MODER {
        LPI2C4_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - LPI2C4 ipg_doze mode"]
    #[inline]
    pub fn lpi2c4_ipg_doze(&self) -> LPI2C4_IPG_DOZER {
        LPI2C4_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi1_ipg_stop_mode(&self) -> LPSPI1_IPG_STOP_MODER {
        LPSPI1_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - LPSPI1 ipg_doze mode"]
    #[inline]
    pub fn lpspi1_ipg_doze(&self) -> LPSPI1_IPG_DOZER {
        LPSPI1_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi2_ipg_stop_mode(&self) -> LPSPI2_IPG_STOP_MODER {
        LPSPI2_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - LPSPI2 ipg_doze mode"]
    #[inline]
    pub fn lpspi2_ipg_doze(&self) -> LPSPI2_IPG_DOZER {
        LPSPI2_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi3_ipg_stop_mode(&self) -> LPSPI3_IPG_STOP_MODER {
        LPSPI3_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - LPSPI3 ipg_doze mode"]
    #[inline]
    pub fn lpspi3_ipg_doze(&self) -> LPSPI3_IPG_DOZER {
        LPSPI3_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi4_ipg_stop_mode(&self) -> LPSPI4_IPG_STOP_MODER {
        LPSPI4_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - LPSPI4 ipg_doze mode"]
    #[inline]
    pub fn lpspi4_ipg_doze(&self) -> LPSPI4_IPG_DOZER {
        LPSPI4_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart1_ipg_stop_mode(&self) -> LPUART1_IPG_STOP_MODER {
        LPUART1_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - LPUART1 ipg_doze mode"]
    #[inline]
    pub fn lpuart1_ipg_doze(&self) -> LPUART1_IPG_DOZER {
        LPUART1_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart2_ipg_stop_mode(&self) -> LPUART2_IPG_STOP_MODER {
        LPUART2_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - LPUART2 ipg_doze mode"]
    #[inline]
    pub fn lpuart2_ipg_doze(&self) -> LPUART2_IPG_DOZER {
        LPUART2_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart3_ipg_stop_mode(&self) -> LPUART3_IPG_STOP_MODER {
        LPUART3_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - LPUART3 ipg_doze mode"]
    #[inline]
    pub fn lpuart3_ipg_doze(&self) -> LPUART3_IPG_DOZER {
        LPUART3_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart4_ipg_stop_mode(&self) -> LPUART4_IPG_STOP_MODER {
        LPUART4_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - LPUART4 ipg_doze mode"]
    #[inline]
    pub fn lpuart4_ipg_doze(&self) -> LPUART4_IPG_DOZER {
        LPUART4_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart5_ipg_stop_mode(&self) -> LPUART5_IPG_STOP_MODER {
        LPUART5_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - LPUART5 ipg_doze mode"]
    #[inline]
    pub fn lpuart5_ipg_doze(&self) -> LPUART5_IPG_DOZER {
        LPUART5_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart6_ipg_stop_mode(&self) -> LPUART6_IPG_STOP_MODER {
        LPUART6_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - LPUART6 ipg_doze mode"]
    #[inline]
    pub fn lpuart6_ipg_doze(&self) -> LPUART6_IPG_DOZER {
        LPUART6_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart7_ipg_stop_mode(&self) -> LPUART7_IPG_STOP_MODER {
        LPUART7_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - LPUART7 ipg_doze mode"]
    #[inline]
    pub fn lpuart7_ipg_doze(&self) -> LPUART7_IPG_DOZER {
        LPUART7_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart8_ipg_stop_mode(&self) -> LPUART8_IPG_STOP_MODER {
        LPUART8_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - LPUART8 ipg_doze mode"]
    #[inline]
    pub fn lpuart8_ipg_doze(&self) -> LPUART8_IPG_DOZER {
        LPUART8_IPG_DOZER::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c1_ipg_stop_mode(&mut self) -> _LPI2C1_IPG_STOP_MODEW {
        _LPI2C1_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 1 - LPI2C1 ipg_doze mode"]
    #[inline]
    pub fn lpi2c1_ipg_doze(&mut self) -> _LPI2C1_IPG_DOZEW {
        _LPI2C1_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 2 - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c2_ipg_stop_mode(&mut self) -> _LPI2C2_IPG_STOP_MODEW {
        _LPI2C2_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 3 - LPI2C2 ipg_doze mode"]
    #[inline]
    pub fn lpi2c2_ipg_doze(&mut self) -> _LPI2C2_IPG_DOZEW {
        _LPI2C2_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 4 - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c3_ipg_stop_mode(&mut self) -> _LPI2C3_IPG_STOP_MODEW {
        _LPI2C3_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 5 - LPI2C3 ipg_doze mode"]
    #[inline]
    pub fn lpi2c3_ipg_doze(&mut self) -> _LPI2C3_IPG_DOZEW {
        _LPI2C3_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 6 - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpi2c4_ipg_stop_mode(&mut self) -> _LPI2C4_IPG_STOP_MODEW {
        _LPI2C4_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 7 - LPI2C4 ipg_doze mode"]
    #[inline]
    pub fn lpi2c4_ipg_doze(&mut self) -> _LPI2C4_IPG_DOZEW {
        _LPI2C4_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 8 - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi1_ipg_stop_mode(&mut self) -> _LPSPI1_IPG_STOP_MODEW {
        _LPSPI1_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 9 - LPSPI1 ipg_doze mode"]
    #[inline]
    pub fn lpspi1_ipg_doze(&mut self) -> _LPSPI1_IPG_DOZEW {
        _LPSPI1_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 10 - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi2_ipg_stop_mode(&mut self) -> _LPSPI2_IPG_STOP_MODEW {
        _LPSPI2_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 11 - LPSPI2 ipg_doze mode"]
    #[inline]
    pub fn lpspi2_ipg_doze(&mut self) -> _LPSPI2_IPG_DOZEW {
        _LPSPI2_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 12 - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi3_ipg_stop_mode(&mut self) -> _LPSPI3_IPG_STOP_MODEW {
        _LPSPI3_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 13 - LPSPI3 ipg_doze mode"]
    #[inline]
    pub fn lpspi3_ipg_doze(&mut self) -> _LPSPI3_IPG_DOZEW {
        _LPSPI3_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 14 - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpspi4_ipg_stop_mode(&mut self) -> _LPSPI4_IPG_STOP_MODEW {
        _LPSPI4_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 15 - LPSPI4 ipg_doze mode"]
    #[inline]
    pub fn lpspi4_ipg_doze(&mut self) -> _LPSPI4_IPG_DOZEW {
        _LPSPI4_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 16 - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart1_ipg_stop_mode(&mut self) -> _LPUART1_IPG_STOP_MODEW {
        _LPUART1_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 17 - LPUART1 ipg_doze mode"]
    #[inline]
    pub fn lpuart1_ipg_doze(&mut self) -> _LPUART1_IPG_DOZEW {
        _LPUART1_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 18 - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart2_ipg_stop_mode(&mut self) -> _LPUART2_IPG_STOP_MODEW {
        _LPUART2_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 19 - LPUART2 ipg_doze mode"]
    #[inline]
    pub fn lpuart2_ipg_doze(&mut self) -> _LPUART2_IPG_DOZEW {
        _LPUART2_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 20 - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart3_ipg_stop_mode(&mut self) -> _LPUART3_IPG_STOP_MODEW {
        _LPUART3_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 21 - LPUART3 ipg_doze mode"]
    #[inline]
    pub fn lpuart3_ipg_doze(&mut self) -> _LPUART3_IPG_DOZEW {
        _LPUART3_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 22 - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart4_ipg_stop_mode(&mut self) -> _LPUART4_IPG_STOP_MODEW {
        _LPUART4_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 23 - LPUART4 ipg_doze mode"]
    #[inline]
    pub fn lpuart4_ipg_doze(&mut self) -> _LPUART4_IPG_DOZEW {
        _LPUART4_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 24 - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart5_ipg_stop_mode(&mut self) -> _LPUART5_IPG_STOP_MODEW {
        _LPUART5_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 25 - LPUART5 ipg_doze mode"]
    #[inline]
    pub fn lpuart5_ipg_doze(&mut self) -> _LPUART5_IPG_DOZEW {
        _LPUART5_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 26 - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart6_ipg_stop_mode(&mut self) -> _LPUART6_IPG_STOP_MODEW {
        _LPUART6_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 27 - LPUART6 ipg_doze mode"]
    #[inline]
    pub fn lpuart6_ipg_doze(&mut self) -> _LPUART6_IPG_DOZEW {
        _LPUART6_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 28 - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart7_ipg_stop_mode(&mut self) -> _LPUART7_IPG_STOP_MODEW {
        _LPUART7_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 29 - LPUART7 ipg_doze mode"]
    #[inline]
    pub fn lpuart7_ipg_doze(&mut self) -> _LPUART7_IPG_DOZEW {
        _LPUART7_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 30 - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn lpuart8_ipg_stop_mode(&mut self) -> _LPUART8_IPG_STOP_MODEW {
        _LPUART8_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 31 - LPUART8 ipg_doze mode"]
    #[inline]
    pub fn lpuart8_ipg_doze(&mut self) -> _LPUART8_IPG_DOZEW {
        _LPUART8_IPG_DOZEW { w: self }
    }
}
