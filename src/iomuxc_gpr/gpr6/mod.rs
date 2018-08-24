#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR6 {
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
#[doc = "Possible values of the field `QTIMER1_TRM0_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM0_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM0_INPUT_SEL_1,
}
impl QTIMER1_TRM0_INPUT_SELR {
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
            QTIMER1_TRM0_INPUT_SELR::QTIMER1_TRM0_INPUT_SEL_0 => false,
            QTIMER1_TRM0_INPUT_SELR::QTIMER1_TRM0_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER1_TRM0_INPUT_SELR {
        match value {
            false => QTIMER1_TRM0_INPUT_SELR::QTIMER1_TRM0_INPUT_SEL_0,
            true => QTIMER1_TRM0_INPUT_SELR::QTIMER1_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM0_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer1_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM0_INPUT_SELR::QTIMER1_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM0_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer1_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM0_INPUT_SELR::QTIMER1_TRM0_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER1_TRM1_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM1_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM1_INPUT_SEL_1,
}
impl QTIMER1_TRM1_INPUT_SELR {
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
            QTIMER1_TRM1_INPUT_SELR::QTIMER1_TRM1_INPUT_SEL_0 => false,
            QTIMER1_TRM1_INPUT_SELR::QTIMER1_TRM1_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER1_TRM1_INPUT_SELR {
        match value {
            false => QTIMER1_TRM1_INPUT_SELR::QTIMER1_TRM1_INPUT_SEL_0,
            true => QTIMER1_TRM1_INPUT_SELR::QTIMER1_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM1_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer1_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM1_INPUT_SELR::QTIMER1_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM1_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer1_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM1_INPUT_SELR::QTIMER1_TRM1_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER1_TRM2_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM2_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM2_INPUT_SEL_1,
}
impl QTIMER1_TRM2_INPUT_SELR {
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
            QTIMER1_TRM2_INPUT_SELR::QTIMER1_TRM2_INPUT_SEL_0 => false,
            QTIMER1_TRM2_INPUT_SELR::QTIMER1_TRM2_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER1_TRM2_INPUT_SELR {
        match value {
            false => QTIMER1_TRM2_INPUT_SELR::QTIMER1_TRM2_INPUT_SEL_0,
            true => QTIMER1_TRM2_INPUT_SELR::QTIMER1_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM2_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer1_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM2_INPUT_SELR::QTIMER1_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM2_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer1_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM2_INPUT_SELR::QTIMER1_TRM2_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER1_TRM3_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM3_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM3_INPUT_SEL_1,
}
impl QTIMER1_TRM3_INPUT_SELR {
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
            QTIMER1_TRM3_INPUT_SELR::QTIMER1_TRM3_INPUT_SEL_0 => false,
            QTIMER1_TRM3_INPUT_SELR::QTIMER1_TRM3_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER1_TRM3_INPUT_SELR {
        match value {
            false => QTIMER1_TRM3_INPUT_SELR::QTIMER1_TRM3_INPUT_SEL_0,
            true => QTIMER1_TRM3_INPUT_SELR::QTIMER1_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM3_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer1_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM3_INPUT_SELR::QTIMER1_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM3_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer1_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM3_INPUT_SELR::QTIMER1_TRM3_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER2_TRM0_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM0_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM0_INPUT_SEL_1,
}
impl QTIMER2_TRM0_INPUT_SELR {
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
            QTIMER2_TRM0_INPUT_SELR::QTIMER2_TRM0_INPUT_SEL_0 => false,
            QTIMER2_TRM0_INPUT_SELR::QTIMER2_TRM0_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER2_TRM0_INPUT_SELR {
        match value {
            false => QTIMER2_TRM0_INPUT_SELR::QTIMER2_TRM0_INPUT_SEL_0,
            true => QTIMER2_TRM0_INPUT_SELR::QTIMER2_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM0_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer2_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM0_INPUT_SELR::QTIMER2_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM0_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer2_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM0_INPUT_SELR::QTIMER2_TRM0_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER2_TRM1_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM1_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM1_INPUT_SEL_1,
}
impl QTIMER2_TRM1_INPUT_SELR {
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
            QTIMER2_TRM1_INPUT_SELR::QTIMER2_TRM1_INPUT_SEL_0 => false,
            QTIMER2_TRM1_INPUT_SELR::QTIMER2_TRM1_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER2_TRM1_INPUT_SELR {
        match value {
            false => QTIMER2_TRM1_INPUT_SELR::QTIMER2_TRM1_INPUT_SEL_0,
            true => QTIMER2_TRM1_INPUT_SELR::QTIMER2_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM1_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer2_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM1_INPUT_SELR::QTIMER2_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM1_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer2_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM1_INPUT_SELR::QTIMER2_TRM1_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER2_TRM2_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM2_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM2_INPUT_SEL_1,
}
impl QTIMER2_TRM2_INPUT_SELR {
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
            QTIMER2_TRM2_INPUT_SELR::QTIMER2_TRM2_INPUT_SEL_0 => false,
            QTIMER2_TRM2_INPUT_SELR::QTIMER2_TRM2_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER2_TRM2_INPUT_SELR {
        match value {
            false => QTIMER2_TRM2_INPUT_SELR::QTIMER2_TRM2_INPUT_SEL_0,
            true => QTIMER2_TRM2_INPUT_SELR::QTIMER2_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM2_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer2_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM2_INPUT_SELR::QTIMER2_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM2_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer2_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM2_INPUT_SELR::QTIMER2_TRM2_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER2_TRM3_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM3_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM3_INPUT_SEL_1,
}
impl QTIMER2_TRM3_INPUT_SELR {
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
            QTIMER2_TRM3_INPUT_SELR::QTIMER2_TRM3_INPUT_SEL_0 => false,
            QTIMER2_TRM3_INPUT_SELR::QTIMER2_TRM3_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER2_TRM3_INPUT_SELR {
        match value {
            false => QTIMER2_TRM3_INPUT_SELR::QTIMER2_TRM3_INPUT_SEL_0,
            true => QTIMER2_TRM3_INPUT_SELR::QTIMER2_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM3_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer2_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM3_INPUT_SELR::QTIMER2_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM3_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer2_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM3_INPUT_SELR::QTIMER2_TRM3_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER3_TRM0_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM0_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM0_INPUT_SEL_1,
}
impl QTIMER3_TRM0_INPUT_SELR {
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
            QTIMER3_TRM0_INPUT_SELR::QTIMER3_TRM0_INPUT_SEL_0 => false,
            QTIMER3_TRM0_INPUT_SELR::QTIMER3_TRM0_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER3_TRM0_INPUT_SELR {
        match value {
            false => QTIMER3_TRM0_INPUT_SELR::QTIMER3_TRM0_INPUT_SEL_0,
            true => QTIMER3_TRM0_INPUT_SELR::QTIMER3_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM0_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer3_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM0_INPUT_SELR::QTIMER3_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM0_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer3_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM0_INPUT_SELR::QTIMER3_TRM0_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER3_TRM1_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM1_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM1_INPUT_SEL_1,
}
impl QTIMER3_TRM1_INPUT_SELR {
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
            QTIMER3_TRM1_INPUT_SELR::QTIMER3_TRM1_INPUT_SEL_0 => false,
            QTIMER3_TRM1_INPUT_SELR::QTIMER3_TRM1_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER3_TRM1_INPUT_SELR {
        match value {
            false => QTIMER3_TRM1_INPUT_SELR::QTIMER3_TRM1_INPUT_SEL_0,
            true => QTIMER3_TRM1_INPUT_SELR::QTIMER3_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM1_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer3_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM1_INPUT_SELR::QTIMER3_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM1_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer3_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM1_INPUT_SELR::QTIMER3_TRM1_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER3_TRM2_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM2_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM2_INPUT_SEL_1,
}
impl QTIMER3_TRM2_INPUT_SELR {
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
            QTIMER3_TRM2_INPUT_SELR::QTIMER3_TRM2_INPUT_SEL_0 => false,
            QTIMER3_TRM2_INPUT_SELR::QTIMER3_TRM2_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER3_TRM2_INPUT_SELR {
        match value {
            false => QTIMER3_TRM2_INPUT_SELR::QTIMER3_TRM2_INPUT_SEL_0,
            true => QTIMER3_TRM2_INPUT_SELR::QTIMER3_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM2_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer3_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM2_INPUT_SELR::QTIMER3_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM2_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer3_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM2_INPUT_SELR::QTIMER3_TRM2_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER3_TRM3_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM3_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM3_INPUT_SEL_1,
}
impl QTIMER3_TRM3_INPUT_SELR {
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
            QTIMER3_TRM3_INPUT_SELR::QTIMER3_TRM3_INPUT_SEL_0 => false,
            QTIMER3_TRM3_INPUT_SELR::QTIMER3_TRM3_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER3_TRM3_INPUT_SELR {
        match value {
            false => QTIMER3_TRM3_INPUT_SELR::QTIMER3_TRM3_INPUT_SEL_0,
            true => QTIMER3_TRM3_INPUT_SELR::QTIMER3_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM3_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer3_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM3_INPUT_SELR::QTIMER3_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM3_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer3_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM3_INPUT_SELR::QTIMER3_TRM3_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER4_TRM0_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM0_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM0_INPUT_SEL_1,
}
impl QTIMER4_TRM0_INPUT_SELR {
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
            QTIMER4_TRM0_INPUT_SELR::QTIMER4_TRM0_INPUT_SEL_0 => false,
            QTIMER4_TRM0_INPUT_SELR::QTIMER4_TRM0_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER4_TRM0_INPUT_SELR {
        match value {
            false => QTIMER4_TRM0_INPUT_SELR::QTIMER4_TRM0_INPUT_SEL_0,
            true => QTIMER4_TRM0_INPUT_SELR::QTIMER4_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM0_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer4_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM0_INPUT_SELR::QTIMER4_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM0_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer4_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM0_INPUT_SELR::QTIMER4_TRM0_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER4_TRM1_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM1_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM1_INPUT_SEL_1,
}
impl QTIMER4_TRM1_INPUT_SELR {
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
            QTIMER4_TRM1_INPUT_SELR::QTIMER4_TRM1_INPUT_SEL_0 => false,
            QTIMER4_TRM1_INPUT_SELR::QTIMER4_TRM1_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER4_TRM1_INPUT_SELR {
        match value {
            false => QTIMER4_TRM1_INPUT_SELR::QTIMER4_TRM1_INPUT_SEL_0,
            true => QTIMER4_TRM1_INPUT_SELR::QTIMER4_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM1_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer4_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM1_INPUT_SELR::QTIMER4_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM1_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer4_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM1_INPUT_SELR::QTIMER4_TRM1_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER4_TRM2_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM2_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM2_INPUT_SEL_1,
}
impl QTIMER4_TRM2_INPUT_SELR {
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
            QTIMER4_TRM2_INPUT_SELR::QTIMER4_TRM2_INPUT_SEL_0 => false,
            QTIMER4_TRM2_INPUT_SELR::QTIMER4_TRM2_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER4_TRM2_INPUT_SELR {
        match value {
            false => QTIMER4_TRM2_INPUT_SELR::QTIMER4_TRM2_INPUT_SEL_0,
            true => QTIMER4_TRM2_INPUT_SELR::QTIMER4_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM2_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer4_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM2_INPUT_SELR::QTIMER4_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM2_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer4_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM2_INPUT_SELR::QTIMER4_TRM2_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `QTIMER4_TRM3_INPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM3_INPUT_SELR {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM3_INPUT_SEL_1,
}
impl QTIMER4_TRM3_INPUT_SELR {
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
            QTIMER4_TRM3_INPUT_SELR::QTIMER4_TRM3_INPUT_SEL_0 => false,
            QTIMER4_TRM3_INPUT_SELR::QTIMER4_TRM3_INPUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER4_TRM3_INPUT_SELR {
        match value {
            false => QTIMER4_TRM3_INPUT_SELR::QTIMER4_TRM3_INPUT_SEL_0,
            true => QTIMER4_TRM3_INPUT_SELR::QTIMER4_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM3_INPUT_SEL_0`"]
    #[inline]
    pub fn is_qtimer4_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM3_INPUT_SELR::QTIMER4_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM3_INPUT_SEL_1`"]
    #[inline]
    pub fn is_qtimer4_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM3_INPUT_SELR::QTIMER4_TRM3_INPUT_SEL_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_4R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_4_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_4_1,
}
impl IOMUXC_XBAR_DIR_SEL_4R {
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
            IOMUXC_XBAR_DIR_SEL_4R::IOMUXC_XBAR_DIR_SEL_4_0 => false,
            IOMUXC_XBAR_DIR_SEL_4R::IOMUXC_XBAR_DIR_SEL_4_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_4R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_4R::IOMUXC_XBAR_DIR_SEL_4_0,
            true => IOMUXC_XBAR_DIR_SEL_4R::IOMUXC_XBAR_DIR_SEL_4_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_4_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_4_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_4R::IOMUXC_XBAR_DIR_SEL_4_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_4_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_4_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_4R::IOMUXC_XBAR_DIR_SEL_4_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_5R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_5_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_5_1,
}
impl IOMUXC_XBAR_DIR_SEL_5R {
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
            IOMUXC_XBAR_DIR_SEL_5R::IOMUXC_XBAR_DIR_SEL_5_0 => false,
            IOMUXC_XBAR_DIR_SEL_5R::IOMUXC_XBAR_DIR_SEL_5_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_5R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_5R::IOMUXC_XBAR_DIR_SEL_5_0,
            true => IOMUXC_XBAR_DIR_SEL_5R::IOMUXC_XBAR_DIR_SEL_5_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_5_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_5_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_5R::IOMUXC_XBAR_DIR_SEL_5_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_5_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_5_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_5R::IOMUXC_XBAR_DIR_SEL_5_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_6R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_6_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_6_1,
}
impl IOMUXC_XBAR_DIR_SEL_6R {
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
            IOMUXC_XBAR_DIR_SEL_6R::IOMUXC_XBAR_DIR_SEL_6_0 => false,
            IOMUXC_XBAR_DIR_SEL_6R::IOMUXC_XBAR_DIR_SEL_6_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_6R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_6R::IOMUXC_XBAR_DIR_SEL_6_0,
            true => IOMUXC_XBAR_DIR_SEL_6R::IOMUXC_XBAR_DIR_SEL_6_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_6_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_6_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_6R::IOMUXC_XBAR_DIR_SEL_6_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_6_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_6_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_6R::IOMUXC_XBAR_DIR_SEL_6_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_7R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_7_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_7_1,
}
impl IOMUXC_XBAR_DIR_SEL_7R {
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
            IOMUXC_XBAR_DIR_SEL_7R::IOMUXC_XBAR_DIR_SEL_7_0 => false,
            IOMUXC_XBAR_DIR_SEL_7R::IOMUXC_XBAR_DIR_SEL_7_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_7R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_7R::IOMUXC_XBAR_DIR_SEL_7_0,
            true => IOMUXC_XBAR_DIR_SEL_7R::IOMUXC_XBAR_DIR_SEL_7_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_7_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_7_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_7R::IOMUXC_XBAR_DIR_SEL_7_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_7_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_7_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_7R::IOMUXC_XBAR_DIR_SEL_7_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_8R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_8_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_8_1,
}
impl IOMUXC_XBAR_DIR_SEL_8R {
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
            IOMUXC_XBAR_DIR_SEL_8R::IOMUXC_XBAR_DIR_SEL_8_0 => false,
            IOMUXC_XBAR_DIR_SEL_8R::IOMUXC_XBAR_DIR_SEL_8_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_8R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_8R::IOMUXC_XBAR_DIR_SEL_8_0,
            true => IOMUXC_XBAR_DIR_SEL_8R::IOMUXC_XBAR_DIR_SEL_8_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_8_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_8_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_8R::IOMUXC_XBAR_DIR_SEL_8_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_8_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_8_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_8R::IOMUXC_XBAR_DIR_SEL_8_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_9R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_9_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_9_1,
}
impl IOMUXC_XBAR_DIR_SEL_9R {
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
            IOMUXC_XBAR_DIR_SEL_9R::IOMUXC_XBAR_DIR_SEL_9_0 => false,
            IOMUXC_XBAR_DIR_SEL_9R::IOMUXC_XBAR_DIR_SEL_9_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_9R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_9R::IOMUXC_XBAR_DIR_SEL_9_0,
            true => IOMUXC_XBAR_DIR_SEL_9R::IOMUXC_XBAR_DIR_SEL_9_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_9_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_9_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_9R::IOMUXC_XBAR_DIR_SEL_9_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_9_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_9_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_9R::IOMUXC_XBAR_DIR_SEL_9_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_10R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_10_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_10_1,
}
impl IOMUXC_XBAR_DIR_SEL_10R {
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
            IOMUXC_XBAR_DIR_SEL_10R::IOMUXC_XBAR_DIR_SEL_10_0 => false,
            IOMUXC_XBAR_DIR_SEL_10R::IOMUXC_XBAR_DIR_SEL_10_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_10R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_10R::IOMUXC_XBAR_DIR_SEL_10_0,
            true => IOMUXC_XBAR_DIR_SEL_10R::IOMUXC_XBAR_DIR_SEL_10_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_10_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_10_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_10R::IOMUXC_XBAR_DIR_SEL_10_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_10_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_10_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_10R::IOMUXC_XBAR_DIR_SEL_10_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_11R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_11_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_11_1,
}
impl IOMUXC_XBAR_DIR_SEL_11R {
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
            IOMUXC_XBAR_DIR_SEL_11R::IOMUXC_XBAR_DIR_SEL_11_0 => false,
            IOMUXC_XBAR_DIR_SEL_11R::IOMUXC_XBAR_DIR_SEL_11_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_11R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_11R::IOMUXC_XBAR_DIR_SEL_11_0,
            true => IOMUXC_XBAR_DIR_SEL_11R::IOMUXC_XBAR_DIR_SEL_11_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_11_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_11_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_11R::IOMUXC_XBAR_DIR_SEL_11_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_11_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_11_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_11R::IOMUXC_XBAR_DIR_SEL_11_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_12R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_12_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_12_1,
}
impl IOMUXC_XBAR_DIR_SEL_12R {
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
            IOMUXC_XBAR_DIR_SEL_12R::IOMUXC_XBAR_DIR_SEL_12_0 => false,
            IOMUXC_XBAR_DIR_SEL_12R::IOMUXC_XBAR_DIR_SEL_12_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_12R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_12R::IOMUXC_XBAR_DIR_SEL_12_0,
            true => IOMUXC_XBAR_DIR_SEL_12R::IOMUXC_XBAR_DIR_SEL_12_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_12_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_12_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_12R::IOMUXC_XBAR_DIR_SEL_12_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_12_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_12_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_12R::IOMUXC_XBAR_DIR_SEL_12_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_13R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_13_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_13_1,
}
impl IOMUXC_XBAR_DIR_SEL_13R {
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
            IOMUXC_XBAR_DIR_SEL_13R::IOMUXC_XBAR_DIR_SEL_13_0 => false,
            IOMUXC_XBAR_DIR_SEL_13R::IOMUXC_XBAR_DIR_SEL_13_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_13R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_13R::IOMUXC_XBAR_DIR_SEL_13_0,
            true => IOMUXC_XBAR_DIR_SEL_13R::IOMUXC_XBAR_DIR_SEL_13_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_13_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_13_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_13R::IOMUXC_XBAR_DIR_SEL_13_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_13_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_13_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_13R::IOMUXC_XBAR_DIR_SEL_13_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_14R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_14_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_14_1,
}
impl IOMUXC_XBAR_DIR_SEL_14R {
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
            IOMUXC_XBAR_DIR_SEL_14R::IOMUXC_XBAR_DIR_SEL_14_0 => false,
            IOMUXC_XBAR_DIR_SEL_14R::IOMUXC_XBAR_DIR_SEL_14_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_14R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_14R::IOMUXC_XBAR_DIR_SEL_14_0,
            true => IOMUXC_XBAR_DIR_SEL_14R::IOMUXC_XBAR_DIR_SEL_14_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_14_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_14_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_14R::IOMUXC_XBAR_DIR_SEL_14_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_14_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_14_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_14R::IOMUXC_XBAR_DIR_SEL_14_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_15R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_15_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_15_1,
}
impl IOMUXC_XBAR_DIR_SEL_15R {
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
            IOMUXC_XBAR_DIR_SEL_15R::IOMUXC_XBAR_DIR_SEL_15_0 => false,
            IOMUXC_XBAR_DIR_SEL_15R::IOMUXC_XBAR_DIR_SEL_15_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_15R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_15R::IOMUXC_XBAR_DIR_SEL_15_0,
            true => IOMUXC_XBAR_DIR_SEL_15R::IOMUXC_XBAR_DIR_SEL_15_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_15_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_15_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_15R::IOMUXC_XBAR_DIR_SEL_15_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_15_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_15_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_15R::IOMUXC_XBAR_DIR_SEL_15_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_16R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_16_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_16_1,
}
impl IOMUXC_XBAR_DIR_SEL_16R {
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
            IOMUXC_XBAR_DIR_SEL_16R::IOMUXC_XBAR_DIR_SEL_16_0 => false,
            IOMUXC_XBAR_DIR_SEL_16R::IOMUXC_XBAR_DIR_SEL_16_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_16R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_16R::IOMUXC_XBAR_DIR_SEL_16_0,
            true => IOMUXC_XBAR_DIR_SEL_16R::IOMUXC_XBAR_DIR_SEL_16_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_16_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_16_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_16R::IOMUXC_XBAR_DIR_SEL_16_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_16_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_16_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_16R::IOMUXC_XBAR_DIR_SEL_16_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_17R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_17_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_17_1,
}
impl IOMUXC_XBAR_DIR_SEL_17R {
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
            IOMUXC_XBAR_DIR_SEL_17R::IOMUXC_XBAR_DIR_SEL_17_0 => false,
            IOMUXC_XBAR_DIR_SEL_17R::IOMUXC_XBAR_DIR_SEL_17_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_17R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_17R::IOMUXC_XBAR_DIR_SEL_17_0,
            true => IOMUXC_XBAR_DIR_SEL_17R::IOMUXC_XBAR_DIR_SEL_17_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_17_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_17_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_17R::IOMUXC_XBAR_DIR_SEL_17_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_17_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_17_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_17R::IOMUXC_XBAR_DIR_SEL_17_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_18R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_18_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_18_1,
}
impl IOMUXC_XBAR_DIR_SEL_18R {
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
            IOMUXC_XBAR_DIR_SEL_18R::IOMUXC_XBAR_DIR_SEL_18_0 => false,
            IOMUXC_XBAR_DIR_SEL_18R::IOMUXC_XBAR_DIR_SEL_18_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_18R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_18R::IOMUXC_XBAR_DIR_SEL_18_0,
            true => IOMUXC_XBAR_DIR_SEL_18R::IOMUXC_XBAR_DIR_SEL_18_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_18_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_18_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_18R::IOMUXC_XBAR_DIR_SEL_18_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_18_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_18_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_18R::IOMUXC_XBAR_DIR_SEL_18_1
    }
}
#[doc = "Possible values of the field `IOMUXC_XBAR_DIR_SEL_19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_19R {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_19_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_19_1,
}
impl IOMUXC_XBAR_DIR_SEL_19R {
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
            IOMUXC_XBAR_DIR_SEL_19R::IOMUXC_XBAR_DIR_SEL_19_0 => false,
            IOMUXC_XBAR_DIR_SEL_19R::IOMUXC_XBAR_DIR_SEL_19_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOMUXC_XBAR_DIR_SEL_19R {
        match value {
            false => IOMUXC_XBAR_DIR_SEL_19R::IOMUXC_XBAR_DIR_SEL_19_0,
            true => IOMUXC_XBAR_DIR_SEL_19R::IOMUXC_XBAR_DIR_SEL_19_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_19_0`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_19_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_19R::IOMUXC_XBAR_DIR_SEL_19_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_19_1`"]
    #[inline]
    pub fn is_iomuxc_xbar_dir_sel_19_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_19R::IOMUXC_XBAR_DIR_SEL_19_1
    }
}
#[doc = "Values that can be written to the field `QTIMER1_TRM0_INPUT_SEL`"]
pub enum QTIMER1_TRM0_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM0_INPUT_SEL_1,
}
impl QTIMER1_TRM0_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER1_TRM0_INPUT_SELW::QTIMER1_TRM0_INPUT_SEL_0 => false,
            QTIMER1_TRM0_INPUT_SELW::QTIMER1_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER1_TRM0_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER1_TRM0_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER1_TRM0_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer1_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM0_INPUT_SELW::QTIMER1_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer1_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM0_INPUT_SELW::QTIMER1_TRM0_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER1_TRM1_INPUT_SEL`"]
pub enum QTIMER1_TRM1_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM1_INPUT_SEL_1,
}
impl QTIMER1_TRM1_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER1_TRM1_INPUT_SELW::QTIMER1_TRM1_INPUT_SEL_0 => false,
            QTIMER1_TRM1_INPUT_SELW::QTIMER1_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER1_TRM1_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER1_TRM1_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER1_TRM1_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer1_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM1_INPUT_SELW::QTIMER1_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer1_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM1_INPUT_SELW::QTIMER1_TRM1_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER1_TRM2_INPUT_SEL`"]
pub enum QTIMER1_TRM2_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM2_INPUT_SEL_1,
}
impl QTIMER1_TRM2_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER1_TRM2_INPUT_SELW::QTIMER1_TRM2_INPUT_SEL_0 => false,
            QTIMER1_TRM2_INPUT_SELW::QTIMER1_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER1_TRM2_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER1_TRM2_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER1_TRM2_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer1_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM2_INPUT_SELW::QTIMER1_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer1_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM2_INPUT_SELW::QTIMER1_TRM2_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER1_TRM3_INPUT_SEL`"]
pub enum QTIMER1_TRM3_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER1_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER1_TRM3_INPUT_SEL_1,
}
impl QTIMER1_TRM3_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER1_TRM3_INPUT_SELW::QTIMER1_TRM3_INPUT_SEL_0 => false,
            QTIMER1_TRM3_INPUT_SELW::QTIMER1_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER1_TRM3_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER1_TRM3_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER1_TRM3_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer1_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM3_INPUT_SELW::QTIMER1_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer1_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM3_INPUT_SELW::QTIMER1_TRM3_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER2_TRM0_INPUT_SEL`"]
pub enum QTIMER2_TRM0_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM0_INPUT_SEL_1,
}
impl QTIMER2_TRM0_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER2_TRM0_INPUT_SELW::QTIMER2_TRM0_INPUT_SEL_0 => false,
            QTIMER2_TRM0_INPUT_SELW::QTIMER2_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER2_TRM0_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER2_TRM0_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER2_TRM0_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer2_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM0_INPUT_SELW::QTIMER2_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer2_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM0_INPUT_SELW::QTIMER2_TRM0_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER2_TRM1_INPUT_SEL`"]
pub enum QTIMER2_TRM1_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM1_INPUT_SEL_1,
}
impl QTIMER2_TRM1_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER2_TRM1_INPUT_SELW::QTIMER2_TRM1_INPUT_SEL_0 => false,
            QTIMER2_TRM1_INPUT_SELW::QTIMER2_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER2_TRM1_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER2_TRM1_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER2_TRM1_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer2_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM1_INPUT_SELW::QTIMER2_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer2_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM1_INPUT_SELW::QTIMER2_TRM1_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER2_TRM2_INPUT_SEL`"]
pub enum QTIMER2_TRM2_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM2_INPUT_SEL_1,
}
impl QTIMER2_TRM2_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER2_TRM2_INPUT_SELW::QTIMER2_TRM2_INPUT_SEL_0 => false,
            QTIMER2_TRM2_INPUT_SELW::QTIMER2_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER2_TRM2_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER2_TRM2_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER2_TRM2_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer2_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM2_INPUT_SELW::QTIMER2_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer2_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM2_INPUT_SELW::QTIMER2_TRM2_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER2_TRM3_INPUT_SEL`"]
pub enum QTIMER2_TRM3_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER2_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER2_TRM3_INPUT_SEL_1,
}
impl QTIMER2_TRM3_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER2_TRM3_INPUT_SELW::QTIMER2_TRM3_INPUT_SEL_0 => false,
            QTIMER2_TRM3_INPUT_SELW::QTIMER2_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER2_TRM3_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER2_TRM3_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER2_TRM3_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer2_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM3_INPUT_SELW::QTIMER2_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer2_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM3_INPUT_SELW::QTIMER2_TRM3_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER3_TRM0_INPUT_SEL`"]
pub enum QTIMER3_TRM0_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM0_INPUT_SEL_1,
}
impl QTIMER3_TRM0_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER3_TRM0_INPUT_SELW::QTIMER3_TRM0_INPUT_SEL_0 => false,
            QTIMER3_TRM0_INPUT_SELW::QTIMER3_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER3_TRM0_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER3_TRM0_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER3_TRM0_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer3_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM0_INPUT_SELW::QTIMER3_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer3_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM0_INPUT_SELW::QTIMER3_TRM0_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER3_TRM1_INPUT_SEL`"]
pub enum QTIMER3_TRM1_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM1_INPUT_SEL_1,
}
impl QTIMER3_TRM1_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER3_TRM1_INPUT_SELW::QTIMER3_TRM1_INPUT_SEL_0 => false,
            QTIMER3_TRM1_INPUT_SELW::QTIMER3_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER3_TRM1_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER3_TRM1_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER3_TRM1_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer3_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM1_INPUT_SELW::QTIMER3_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer3_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM1_INPUT_SELW::QTIMER3_TRM1_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER3_TRM2_INPUT_SEL`"]
pub enum QTIMER3_TRM2_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM2_INPUT_SEL_1,
}
impl QTIMER3_TRM2_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER3_TRM2_INPUT_SELW::QTIMER3_TRM2_INPUT_SEL_0 => false,
            QTIMER3_TRM2_INPUT_SELW::QTIMER3_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER3_TRM2_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER3_TRM2_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER3_TRM2_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer3_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM2_INPUT_SELW::QTIMER3_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer3_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM2_INPUT_SELW::QTIMER3_TRM2_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER3_TRM3_INPUT_SEL`"]
pub enum QTIMER3_TRM3_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER3_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER3_TRM3_INPUT_SEL_1,
}
impl QTIMER3_TRM3_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER3_TRM3_INPUT_SELW::QTIMER3_TRM3_INPUT_SEL_0 => false,
            QTIMER3_TRM3_INPUT_SELW::QTIMER3_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER3_TRM3_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER3_TRM3_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER3_TRM3_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer3_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM3_INPUT_SELW::QTIMER3_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer3_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM3_INPUT_SELW::QTIMER3_TRM3_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER4_TRM0_INPUT_SEL`"]
pub enum QTIMER4_TRM0_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM0_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM0_INPUT_SEL_1,
}
impl QTIMER4_TRM0_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER4_TRM0_INPUT_SELW::QTIMER4_TRM0_INPUT_SEL_0 => false,
            QTIMER4_TRM0_INPUT_SELW::QTIMER4_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER4_TRM0_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER4_TRM0_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER4_TRM0_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer4_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM0_INPUT_SELW::QTIMER4_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer4_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM0_INPUT_SELW::QTIMER4_TRM0_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER4_TRM1_INPUT_SEL`"]
pub enum QTIMER4_TRM1_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM1_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM1_INPUT_SEL_1,
}
impl QTIMER4_TRM1_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER4_TRM1_INPUT_SELW::QTIMER4_TRM1_INPUT_SEL_0 => false,
            QTIMER4_TRM1_INPUT_SELW::QTIMER4_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER4_TRM1_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER4_TRM1_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER4_TRM1_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer4_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM1_INPUT_SELW::QTIMER4_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer4_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM1_INPUT_SELW::QTIMER4_TRM1_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER4_TRM2_INPUT_SEL`"]
pub enum QTIMER4_TRM2_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM2_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM2_INPUT_SEL_1,
}
impl QTIMER4_TRM2_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER4_TRM2_INPUT_SELW::QTIMER4_TRM2_INPUT_SEL_0 => false,
            QTIMER4_TRM2_INPUT_SELW::QTIMER4_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER4_TRM2_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER4_TRM2_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER4_TRM2_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer4_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM2_INPUT_SELW::QTIMER4_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer4_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM2_INPUT_SELW::QTIMER4_TRM2_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `QTIMER4_TRM3_INPUT_SEL`"]
pub enum QTIMER4_TRM3_INPUT_SELW {
    #[doc = "input from IOMUX"]
    QTIMER4_TRM3_INPUT_SEL_0,
    #[doc = "input from XBAR"]
    QTIMER4_TRM3_INPUT_SEL_1,
}
impl QTIMER4_TRM3_INPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER4_TRM3_INPUT_SELW::QTIMER4_TRM3_INPUT_SEL_0 => false,
            QTIMER4_TRM3_INPUT_SELW::QTIMER4_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER4_TRM3_INPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER4_TRM3_INPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER4_TRM3_INPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline]
    pub fn qtimer4_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM3_INPUT_SELW::QTIMER4_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline]
    pub fn qtimer4_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM3_INPUT_SELW::QTIMER4_TRM3_INPUT_SEL_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_4`"]
pub enum IOMUXC_XBAR_DIR_SEL_4W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_4_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_4_1,
}
impl IOMUXC_XBAR_DIR_SEL_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_4W::IOMUXC_XBAR_DIR_SEL_4_0 => false,
            IOMUXC_XBAR_DIR_SEL_4W::IOMUXC_XBAR_DIR_SEL_4_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_4W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_4_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_4W::IOMUXC_XBAR_DIR_SEL_4_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_4_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_4W::IOMUXC_XBAR_DIR_SEL_4_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_5`"]
pub enum IOMUXC_XBAR_DIR_SEL_5W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_5_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_5_1,
}
impl IOMUXC_XBAR_DIR_SEL_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_5W::IOMUXC_XBAR_DIR_SEL_5_0 => false,
            IOMUXC_XBAR_DIR_SEL_5W::IOMUXC_XBAR_DIR_SEL_5_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_5W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_5_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_5W::IOMUXC_XBAR_DIR_SEL_5_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_5_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_5W::IOMUXC_XBAR_DIR_SEL_5_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_6`"]
pub enum IOMUXC_XBAR_DIR_SEL_6W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_6_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_6_1,
}
impl IOMUXC_XBAR_DIR_SEL_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_6W::IOMUXC_XBAR_DIR_SEL_6_0 => false,
            IOMUXC_XBAR_DIR_SEL_6W::IOMUXC_XBAR_DIR_SEL_6_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_6W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_6_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_6W::IOMUXC_XBAR_DIR_SEL_6_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_6_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_6W::IOMUXC_XBAR_DIR_SEL_6_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_7`"]
pub enum IOMUXC_XBAR_DIR_SEL_7W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_7_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_7_1,
}
impl IOMUXC_XBAR_DIR_SEL_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_7W::IOMUXC_XBAR_DIR_SEL_7_0 => false,
            IOMUXC_XBAR_DIR_SEL_7W::IOMUXC_XBAR_DIR_SEL_7_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_7W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_7_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_7W::IOMUXC_XBAR_DIR_SEL_7_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_7_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_7W::IOMUXC_XBAR_DIR_SEL_7_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_8`"]
pub enum IOMUXC_XBAR_DIR_SEL_8W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_8_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_8_1,
}
impl IOMUXC_XBAR_DIR_SEL_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_8W::IOMUXC_XBAR_DIR_SEL_8_0 => false,
            IOMUXC_XBAR_DIR_SEL_8W::IOMUXC_XBAR_DIR_SEL_8_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_8W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_8_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_8W::IOMUXC_XBAR_DIR_SEL_8_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_8_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_8W::IOMUXC_XBAR_DIR_SEL_8_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_9`"]
pub enum IOMUXC_XBAR_DIR_SEL_9W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_9_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_9_1,
}
impl IOMUXC_XBAR_DIR_SEL_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_9W::IOMUXC_XBAR_DIR_SEL_9_0 => false,
            IOMUXC_XBAR_DIR_SEL_9W::IOMUXC_XBAR_DIR_SEL_9_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_9W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_9_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_9W::IOMUXC_XBAR_DIR_SEL_9_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_9_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_9W::IOMUXC_XBAR_DIR_SEL_9_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_10`"]
pub enum IOMUXC_XBAR_DIR_SEL_10W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_10_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_10_1,
}
impl IOMUXC_XBAR_DIR_SEL_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_10W::IOMUXC_XBAR_DIR_SEL_10_0 => false,
            IOMUXC_XBAR_DIR_SEL_10W::IOMUXC_XBAR_DIR_SEL_10_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_10W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_10_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_10W::IOMUXC_XBAR_DIR_SEL_10_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_10_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_10W::IOMUXC_XBAR_DIR_SEL_10_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_11`"]
pub enum IOMUXC_XBAR_DIR_SEL_11W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_11_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_11_1,
}
impl IOMUXC_XBAR_DIR_SEL_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_11W::IOMUXC_XBAR_DIR_SEL_11_0 => false,
            IOMUXC_XBAR_DIR_SEL_11W::IOMUXC_XBAR_DIR_SEL_11_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_11W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_11_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_11W::IOMUXC_XBAR_DIR_SEL_11_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_11_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_11W::IOMUXC_XBAR_DIR_SEL_11_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_12`"]
pub enum IOMUXC_XBAR_DIR_SEL_12W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_12_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_12_1,
}
impl IOMUXC_XBAR_DIR_SEL_12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_12W::IOMUXC_XBAR_DIR_SEL_12_0 => false,
            IOMUXC_XBAR_DIR_SEL_12W::IOMUXC_XBAR_DIR_SEL_12_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_12W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_12_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_12W::IOMUXC_XBAR_DIR_SEL_12_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_12_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_12W::IOMUXC_XBAR_DIR_SEL_12_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_13`"]
pub enum IOMUXC_XBAR_DIR_SEL_13W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_13_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_13_1,
}
impl IOMUXC_XBAR_DIR_SEL_13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_13W::IOMUXC_XBAR_DIR_SEL_13_0 => false,
            IOMUXC_XBAR_DIR_SEL_13W::IOMUXC_XBAR_DIR_SEL_13_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_13W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_13_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_13W::IOMUXC_XBAR_DIR_SEL_13_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_13_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_13W::IOMUXC_XBAR_DIR_SEL_13_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_14`"]
pub enum IOMUXC_XBAR_DIR_SEL_14W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_14_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_14_1,
}
impl IOMUXC_XBAR_DIR_SEL_14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_14W::IOMUXC_XBAR_DIR_SEL_14_0 => false,
            IOMUXC_XBAR_DIR_SEL_14W::IOMUXC_XBAR_DIR_SEL_14_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_14W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_14_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_14W::IOMUXC_XBAR_DIR_SEL_14_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_14_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_14W::IOMUXC_XBAR_DIR_SEL_14_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_15`"]
pub enum IOMUXC_XBAR_DIR_SEL_15W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_15_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_15_1,
}
impl IOMUXC_XBAR_DIR_SEL_15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_15W::IOMUXC_XBAR_DIR_SEL_15_0 => false,
            IOMUXC_XBAR_DIR_SEL_15W::IOMUXC_XBAR_DIR_SEL_15_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_15W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_15_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_15W::IOMUXC_XBAR_DIR_SEL_15_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_15_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_15W::IOMUXC_XBAR_DIR_SEL_15_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_16`"]
pub enum IOMUXC_XBAR_DIR_SEL_16W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_16_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_16_1,
}
impl IOMUXC_XBAR_DIR_SEL_16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_16W::IOMUXC_XBAR_DIR_SEL_16_0 => false,
            IOMUXC_XBAR_DIR_SEL_16W::IOMUXC_XBAR_DIR_SEL_16_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_16W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_16_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_16W::IOMUXC_XBAR_DIR_SEL_16_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_16_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_16W::IOMUXC_XBAR_DIR_SEL_16_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_17`"]
pub enum IOMUXC_XBAR_DIR_SEL_17W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_17_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_17_1,
}
impl IOMUXC_XBAR_DIR_SEL_17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_17W::IOMUXC_XBAR_DIR_SEL_17_0 => false,
            IOMUXC_XBAR_DIR_SEL_17W::IOMUXC_XBAR_DIR_SEL_17_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_17W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_17_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_17W::IOMUXC_XBAR_DIR_SEL_17_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_17_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_17W::IOMUXC_XBAR_DIR_SEL_17_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_18`"]
pub enum IOMUXC_XBAR_DIR_SEL_18W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_18_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_18_1,
}
impl IOMUXC_XBAR_DIR_SEL_18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_18W::IOMUXC_XBAR_DIR_SEL_18_0 => false,
            IOMUXC_XBAR_DIR_SEL_18W::IOMUXC_XBAR_DIR_SEL_18_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_18W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_18_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_18W::IOMUXC_XBAR_DIR_SEL_18_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_18_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_18W::IOMUXC_XBAR_DIR_SEL_18_1)
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
#[doc = "Values that can be written to the field `IOMUXC_XBAR_DIR_SEL_19`"]
pub enum IOMUXC_XBAR_DIR_SEL_19W {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_19_0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_19_1,
}
impl IOMUXC_XBAR_DIR_SEL_19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOMUXC_XBAR_DIR_SEL_19W::IOMUXC_XBAR_DIR_SEL_19_0 => false,
            IOMUXC_XBAR_DIR_SEL_19W::IOMUXC_XBAR_DIR_SEL_19_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMUXC_XBAR_DIR_SEL_19W<'a> {
    w: &'a mut W,
}
impl<'a> _IOMUXC_XBAR_DIR_SEL_19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_19_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_19W::IOMUXC_XBAR_DIR_SEL_19_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_19_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_19W::IOMUXC_XBAR_DIR_SEL_19_1)
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
    #[doc = "Bit 0 - QTIMER1 TMR0 input select"]
    #[inline]
    pub fn qtimer1_trm0_input_sel(&self) -> QTIMER1_TRM0_INPUT_SELR {
        QTIMER1_TRM0_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - QTIMER1 TMR1 input select"]
    #[inline]
    pub fn qtimer1_trm1_input_sel(&self) -> QTIMER1_TRM1_INPUT_SELR {
        QTIMER1_TRM1_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - QTIMER1 TMR2 input select"]
    #[inline]
    pub fn qtimer1_trm2_input_sel(&self) -> QTIMER1_TRM2_INPUT_SELR {
        QTIMER1_TRM2_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - QTIMER1 TMR3 input select"]
    #[inline]
    pub fn qtimer1_trm3_input_sel(&self) -> QTIMER1_TRM3_INPUT_SELR {
        QTIMER1_TRM3_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - QTIMER2 TMR0 input select"]
    #[inline]
    pub fn qtimer2_trm0_input_sel(&self) -> QTIMER2_TRM0_INPUT_SELR {
        QTIMER2_TRM0_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - QTIMER2 TMR1 input select"]
    #[inline]
    pub fn qtimer2_trm1_input_sel(&self) -> QTIMER2_TRM1_INPUT_SELR {
        QTIMER2_TRM1_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - QTIMER2 TMR2 input select"]
    #[inline]
    pub fn qtimer2_trm2_input_sel(&self) -> QTIMER2_TRM2_INPUT_SELR {
        QTIMER2_TRM2_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - QTIMER2 TMR3 input select"]
    #[inline]
    pub fn qtimer2_trm3_input_sel(&self) -> QTIMER2_TRM3_INPUT_SELR {
        QTIMER2_TRM3_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - QTIMER3 TMR0 input select"]
    #[inline]
    pub fn qtimer3_trm0_input_sel(&self) -> QTIMER3_TRM0_INPUT_SELR {
        QTIMER3_TRM0_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - QTIMER3 TMR1 input select"]
    #[inline]
    pub fn qtimer3_trm1_input_sel(&self) -> QTIMER3_TRM1_INPUT_SELR {
        QTIMER3_TRM1_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - QTIMER3 TMR2 input select"]
    #[inline]
    pub fn qtimer3_trm2_input_sel(&self) -> QTIMER3_TRM2_INPUT_SELR {
        QTIMER3_TRM2_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - QTIMER3 TMR3 input select"]
    #[inline]
    pub fn qtimer3_trm3_input_sel(&self) -> QTIMER3_TRM3_INPUT_SELR {
        QTIMER3_TRM3_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - QTIMER4 TMR0 input select"]
    #[inline]
    pub fn qtimer4_trm0_input_sel(&self) -> QTIMER4_TRM0_INPUT_SELR {
        QTIMER4_TRM0_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - QTIMER4 TMR1 input select"]
    #[inline]
    pub fn qtimer4_trm1_input_sel(&self) -> QTIMER4_TRM1_INPUT_SELR {
        QTIMER4_TRM1_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - QTIMER4 TMR2 input select"]
    #[inline]
    pub fn qtimer4_trm2_input_sel(&self) -> QTIMER4_TRM2_INPUT_SELR {
        QTIMER4_TRM2_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - QTIMER4 TMR3 input select"]
    #[inline]
    pub fn qtimer4_trm3_input_sel(&self) -> QTIMER4_TRM3_INPUT_SELR {
        QTIMER4_TRM3_INPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - IOMUXC XBAR_INOUT4 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_4(&self) -> IOMUXC_XBAR_DIR_SEL_4R {
        IOMUXC_XBAR_DIR_SEL_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - IOMUXC XBAR_INOUT5 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_5(&self) -> IOMUXC_XBAR_DIR_SEL_5R {
        IOMUXC_XBAR_DIR_SEL_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - IOMUXC XBAR_INOUT6 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_6(&self) -> IOMUXC_XBAR_DIR_SEL_6R {
        IOMUXC_XBAR_DIR_SEL_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - IOMUXC XBAR_INOUT7 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_7(&self) -> IOMUXC_XBAR_DIR_SEL_7R {
        IOMUXC_XBAR_DIR_SEL_7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - IOMUXC XBAR_INOUT8 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_8(&self) -> IOMUXC_XBAR_DIR_SEL_8R {
        IOMUXC_XBAR_DIR_SEL_8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - IOMUXC XBAR_INOUT9 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_9(&self) -> IOMUXC_XBAR_DIR_SEL_9R {
        IOMUXC_XBAR_DIR_SEL_9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - IOMUXC XBAR_INOUT10 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_10(&self) -> IOMUXC_XBAR_DIR_SEL_10R {
        IOMUXC_XBAR_DIR_SEL_10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - IOMUXC XBAR_INOUT11 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_11(&self) -> IOMUXC_XBAR_DIR_SEL_11R {
        IOMUXC_XBAR_DIR_SEL_11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - IOMUXC XBAR_INOUT12 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_12(&self) -> IOMUXC_XBAR_DIR_SEL_12R {
        IOMUXC_XBAR_DIR_SEL_12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - IOMUXC XBAR_INOUT13 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_13(&self) -> IOMUXC_XBAR_DIR_SEL_13R {
        IOMUXC_XBAR_DIR_SEL_13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - IOMUXC XBAR_INOUT14 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_14(&self) -> IOMUXC_XBAR_DIR_SEL_14R {
        IOMUXC_XBAR_DIR_SEL_14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - IOMUXC XBAR_INOUT15 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_15(&self) -> IOMUXC_XBAR_DIR_SEL_15R {
        IOMUXC_XBAR_DIR_SEL_15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - IOMUXC XBAR_INOUT16 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_16(&self) -> IOMUXC_XBAR_DIR_SEL_16R {
        IOMUXC_XBAR_DIR_SEL_16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - IOMUXC XBAR_INOUT17 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_17(&self) -> IOMUXC_XBAR_DIR_SEL_17R {
        IOMUXC_XBAR_DIR_SEL_17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - IOMUXC XBAR_INOUT18 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_18(&self) -> IOMUXC_XBAR_DIR_SEL_18R {
        IOMUXC_XBAR_DIR_SEL_18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - IOMUXC XBAR_INOUT19 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_19(&self) -> IOMUXC_XBAR_DIR_SEL_19R {
        IOMUXC_XBAR_DIR_SEL_19R::_from({
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
    #[doc = "Bit 0 - QTIMER1 TMR0 input select"]
    #[inline]
    pub fn qtimer1_trm0_input_sel(&mut self) -> _QTIMER1_TRM0_INPUT_SELW {
        _QTIMER1_TRM0_INPUT_SELW { w: self }
    }
    #[doc = "Bit 1 - QTIMER1 TMR1 input select"]
    #[inline]
    pub fn qtimer1_trm1_input_sel(&mut self) -> _QTIMER1_TRM1_INPUT_SELW {
        _QTIMER1_TRM1_INPUT_SELW { w: self }
    }
    #[doc = "Bit 2 - QTIMER1 TMR2 input select"]
    #[inline]
    pub fn qtimer1_trm2_input_sel(&mut self) -> _QTIMER1_TRM2_INPUT_SELW {
        _QTIMER1_TRM2_INPUT_SELW { w: self }
    }
    #[doc = "Bit 3 - QTIMER1 TMR3 input select"]
    #[inline]
    pub fn qtimer1_trm3_input_sel(&mut self) -> _QTIMER1_TRM3_INPUT_SELW {
        _QTIMER1_TRM3_INPUT_SELW { w: self }
    }
    #[doc = "Bit 4 - QTIMER2 TMR0 input select"]
    #[inline]
    pub fn qtimer2_trm0_input_sel(&mut self) -> _QTIMER2_TRM0_INPUT_SELW {
        _QTIMER2_TRM0_INPUT_SELW { w: self }
    }
    #[doc = "Bit 5 - QTIMER2 TMR1 input select"]
    #[inline]
    pub fn qtimer2_trm1_input_sel(&mut self) -> _QTIMER2_TRM1_INPUT_SELW {
        _QTIMER2_TRM1_INPUT_SELW { w: self }
    }
    #[doc = "Bit 6 - QTIMER2 TMR2 input select"]
    #[inline]
    pub fn qtimer2_trm2_input_sel(&mut self) -> _QTIMER2_TRM2_INPUT_SELW {
        _QTIMER2_TRM2_INPUT_SELW { w: self }
    }
    #[doc = "Bit 7 - QTIMER2 TMR3 input select"]
    #[inline]
    pub fn qtimer2_trm3_input_sel(&mut self) -> _QTIMER2_TRM3_INPUT_SELW {
        _QTIMER2_TRM3_INPUT_SELW { w: self }
    }
    #[doc = "Bit 8 - QTIMER3 TMR0 input select"]
    #[inline]
    pub fn qtimer3_trm0_input_sel(&mut self) -> _QTIMER3_TRM0_INPUT_SELW {
        _QTIMER3_TRM0_INPUT_SELW { w: self }
    }
    #[doc = "Bit 9 - QTIMER3 TMR1 input select"]
    #[inline]
    pub fn qtimer3_trm1_input_sel(&mut self) -> _QTIMER3_TRM1_INPUT_SELW {
        _QTIMER3_TRM1_INPUT_SELW { w: self }
    }
    #[doc = "Bit 10 - QTIMER3 TMR2 input select"]
    #[inline]
    pub fn qtimer3_trm2_input_sel(&mut self) -> _QTIMER3_TRM2_INPUT_SELW {
        _QTIMER3_TRM2_INPUT_SELW { w: self }
    }
    #[doc = "Bit 11 - QTIMER3 TMR3 input select"]
    #[inline]
    pub fn qtimer3_trm3_input_sel(&mut self) -> _QTIMER3_TRM3_INPUT_SELW {
        _QTIMER3_TRM3_INPUT_SELW { w: self }
    }
    #[doc = "Bit 12 - QTIMER4 TMR0 input select"]
    #[inline]
    pub fn qtimer4_trm0_input_sel(&mut self) -> _QTIMER4_TRM0_INPUT_SELW {
        _QTIMER4_TRM0_INPUT_SELW { w: self }
    }
    #[doc = "Bit 13 - QTIMER4 TMR1 input select"]
    #[inline]
    pub fn qtimer4_trm1_input_sel(&mut self) -> _QTIMER4_TRM1_INPUT_SELW {
        _QTIMER4_TRM1_INPUT_SELW { w: self }
    }
    #[doc = "Bit 14 - QTIMER4 TMR2 input select"]
    #[inline]
    pub fn qtimer4_trm2_input_sel(&mut self) -> _QTIMER4_TRM2_INPUT_SELW {
        _QTIMER4_TRM2_INPUT_SELW { w: self }
    }
    #[doc = "Bit 15 - QTIMER4 TMR3 input select"]
    #[inline]
    pub fn qtimer4_trm3_input_sel(&mut self) -> _QTIMER4_TRM3_INPUT_SELW {
        _QTIMER4_TRM3_INPUT_SELW { w: self }
    }
    #[doc = "Bit 16 - IOMUXC XBAR_INOUT4 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_4(&mut self) -> _IOMUXC_XBAR_DIR_SEL_4W {
        _IOMUXC_XBAR_DIR_SEL_4W { w: self }
    }
    #[doc = "Bit 17 - IOMUXC XBAR_INOUT5 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_5(&mut self) -> _IOMUXC_XBAR_DIR_SEL_5W {
        _IOMUXC_XBAR_DIR_SEL_5W { w: self }
    }
    #[doc = "Bit 18 - IOMUXC XBAR_INOUT6 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_6(&mut self) -> _IOMUXC_XBAR_DIR_SEL_6W {
        _IOMUXC_XBAR_DIR_SEL_6W { w: self }
    }
    #[doc = "Bit 19 - IOMUXC XBAR_INOUT7 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_7(&mut self) -> _IOMUXC_XBAR_DIR_SEL_7W {
        _IOMUXC_XBAR_DIR_SEL_7W { w: self }
    }
    #[doc = "Bit 20 - IOMUXC XBAR_INOUT8 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_8(&mut self) -> _IOMUXC_XBAR_DIR_SEL_8W {
        _IOMUXC_XBAR_DIR_SEL_8W { w: self }
    }
    #[doc = "Bit 21 - IOMUXC XBAR_INOUT9 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_9(&mut self) -> _IOMUXC_XBAR_DIR_SEL_9W {
        _IOMUXC_XBAR_DIR_SEL_9W { w: self }
    }
    #[doc = "Bit 22 - IOMUXC XBAR_INOUT10 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_10(&mut self) -> _IOMUXC_XBAR_DIR_SEL_10W {
        _IOMUXC_XBAR_DIR_SEL_10W { w: self }
    }
    #[doc = "Bit 23 - IOMUXC XBAR_INOUT11 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_11(&mut self) -> _IOMUXC_XBAR_DIR_SEL_11W {
        _IOMUXC_XBAR_DIR_SEL_11W { w: self }
    }
    #[doc = "Bit 24 - IOMUXC XBAR_INOUT12 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_12(&mut self) -> _IOMUXC_XBAR_DIR_SEL_12W {
        _IOMUXC_XBAR_DIR_SEL_12W { w: self }
    }
    #[doc = "Bit 25 - IOMUXC XBAR_INOUT13 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_13(&mut self) -> _IOMUXC_XBAR_DIR_SEL_13W {
        _IOMUXC_XBAR_DIR_SEL_13W { w: self }
    }
    #[doc = "Bit 26 - IOMUXC XBAR_INOUT14 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_14(&mut self) -> _IOMUXC_XBAR_DIR_SEL_14W {
        _IOMUXC_XBAR_DIR_SEL_14W { w: self }
    }
    #[doc = "Bit 27 - IOMUXC XBAR_INOUT15 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_15(&mut self) -> _IOMUXC_XBAR_DIR_SEL_15W {
        _IOMUXC_XBAR_DIR_SEL_15W { w: self }
    }
    #[doc = "Bit 28 - IOMUXC XBAR_INOUT16 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_16(&mut self) -> _IOMUXC_XBAR_DIR_SEL_16W {
        _IOMUXC_XBAR_DIR_SEL_16W { w: self }
    }
    #[doc = "Bit 29 - IOMUXC XBAR_INOUT17 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_17(&mut self) -> _IOMUXC_XBAR_DIR_SEL_17W {
        _IOMUXC_XBAR_DIR_SEL_17W { w: self }
    }
    #[doc = "Bit 30 - IOMUXC XBAR_INOUT18 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_18(&mut self) -> _IOMUXC_XBAR_DIR_SEL_18W {
        _IOMUXC_XBAR_DIR_SEL_18W { w: self }
    }
    #[doc = "Bit 31 - IOMUXC XBAR_INOUT19 function direction select"]
    #[inline]
    pub fn iomuxc_xbar_dir_sel_19(&mut self) -> _IOMUXC_XBAR_DIR_SEL_19W {
        _IOMUXC_XBAR_DIR_SEL_19W { w: self }
    }
}
