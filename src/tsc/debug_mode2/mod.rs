#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG_MODE2 {
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
#[doc = "Possible values of the field `XPUL_PULL_DOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XPUL_PULL_DOWNR {
    #[doc = "Close the switch"]
    XPUL_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    XPUL_PULL_DOWN_1,
}
impl XPUL_PULL_DOWNR {
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
            XPUL_PULL_DOWNR::XPUL_PULL_DOWN_0 => false,
            XPUL_PULL_DOWNR::XPUL_PULL_DOWN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XPUL_PULL_DOWNR {
        match value {
            false => XPUL_PULL_DOWNR::XPUL_PULL_DOWN_0,
            true => XPUL_PULL_DOWNR::XPUL_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_DOWN_0`"]
    #[inline]
    pub fn is_xpul_pull_down_0(&self) -> bool {
        *self == XPUL_PULL_DOWNR::XPUL_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_DOWN_1`"]
    #[inline]
    pub fn is_xpul_pull_down_1(&self) -> bool {
        *self == XPUL_PULL_DOWNR::XPUL_PULL_DOWN_1
    }
}
#[doc = "Possible values of the field `XPUL_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XPUL_PULL_UPR {
    #[doc = "Close the switch"]
    XPUL_PULL_UP_0,
    #[doc = "Open up the switch"]
    XPUL_PULL_UP_1,
}
impl XPUL_PULL_UPR {
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
            XPUL_PULL_UPR::XPUL_PULL_UP_0 => false,
            XPUL_PULL_UPR::XPUL_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XPUL_PULL_UPR {
        match value {
            false => XPUL_PULL_UPR::XPUL_PULL_UP_0,
            true => XPUL_PULL_UPR::XPUL_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_UP_0`"]
    #[inline]
    pub fn is_xpul_pull_up_0(&self) -> bool {
        *self == XPUL_PULL_UPR::XPUL_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_UP_1`"]
    #[inline]
    pub fn is_xpul_pull_up_1(&self) -> bool {
        *self == XPUL_PULL_UPR::XPUL_PULL_UP_1
    }
}
#[doc = "Possible values of the field `XPUL_200K_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XPUL_200K_PULL_UPR {
    #[doc = "Close the switch"]
    XPUL_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    XPUL_200K_PULL_UP_1,
}
impl XPUL_200K_PULL_UPR {
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
            XPUL_200K_PULL_UPR::XPUL_200K_PULL_UP_0 => false,
            XPUL_200K_PULL_UPR::XPUL_200K_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XPUL_200K_PULL_UPR {
        match value {
            false => XPUL_200K_PULL_UPR::XPUL_200K_PULL_UP_0,
            true => XPUL_200K_PULL_UPR::XPUL_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XPUL_200K_PULL_UP_0`"]
    #[inline]
    pub fn is_xpul_200k_pull_up_0(&self) -> bool {
        *self == XPUL_200K_PULL_UPR::XPUL_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XPUL_200K_PULL_UP_1`"]
    #[inline]
    pub fn is_xpul_200k_pull_up_1(&self) -> bool {
        *self == XPUL_200K_PULL_UPR::XPUL_200K_PULL_UP_1
    }
}
#[doc = "Possible values of the field `XNUR_PULL_DOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNUR_PULL_DOWNR {
    #[doc = "Close the switch"]
    XNUR_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    XNUR_PULL_DOWN_1,
}
impl XNUR_PULL_DOWNR {
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
            XNUR_PULL_DOWNR::XNUR_PULL_DOWN_0 => false,
            XNUR_PULL_DOWNR::XNUR_PULL_DOWN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XNUR_PULL_DOWNR {
        match value {
            false => XNUR_PULL_DOWNR::XNUR_PULL_DOWN_0,
            true => XNUR_PULL_DOWNR::XNUR_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_DOWN_0`"]
    #[inline]
    pub fn is_xnur_pull_down_0(&self) -> bool {
        *self == XNUR_PULL_DOWNR::XNUR_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_DOWN_1`"]
    #[inline]
    pub fn is_xnur_pull_down_1(&self) -> bool {
        *self == XNUR_PULL_DOWNR::XNUR_PULL_DOWN_1
    }
}
#[doc = "Possible values of the field `XNUR_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNUR_PULL_UPR {
    #[doc = "Close the switch"]
    XNUR_PULL_UP_0,
    #[doc = "Open up the switch"]
    XNUR_PULL_UP_1,
}
impl XNUR_PULL_UPR {
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
            XNUR_PULL_UPR::XNUR_PULL_UP_0 => false,
            XNUR_PULL_UPR::XNUR_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XNUR_PULL_UPR {
        match value {
            false => XNUR_PULL_UPR::XNUR_PULL_UP_0,
            true => XNUR_PULL_UPR::XNUR_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_UP_0`"]
    #[inline]
    pub fn is_xnur_pull_up_0(&self) -> bool {
        *self == XNUR_PULL_UPR::XNUR_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_UP_1`"]
    #[inline]
    pub fn is_xnur_pull_up_1(&self) -> bool {
        *self == XNUR_PULL_UPR::XNUR_PULL_UP_1
    }
}
#[doc = "Possible values of the field `XNUR_200K_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNUR_200K_PULL_UPR {
    #[doc = "Close the switch"]
    XNUR_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    XNUR_200K_PULL_UP_1,
}
impl XNUR_200K_PULL_UPR {
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
            XNUR_200K_PULL_UPR::XNUR_200K_PULL_UP_0 => false,
            XNUR_200K_PULL_UPR::XNUR_200K_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XNUR_200K_PULL_UPR {
        match value {
            false => XNUR_200K_PULL_UPR::XNUR_200K_PULL_UP_0,
            true => XNUR_200K_PULL_UPR::XNUR_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNUR_200K_PULL_UP_0`"]
    #[inline]
    pub fn is_xnur_200k_pull_up_0(&self) -> bool {
        *self == XNUR_200K_PULL_UPR::XNUR_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XNUR_200K_PULL_UP_1`"]
    #[inline]
    pub fn is_xnur_200k_pull_up_1(&self) -> bool {
        *self == XNUR_200K_PULL_UPR::XNUR_200K_PULL_UP_1
    }
}
#[doc = "Possible values of the field `YPLL_PULL_DOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YPLL_PULL_DOWNR {
    #[doc = "Close the switch"]
    YPLL_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    YPLL_PULL_DOWN_1,
}
impl YPLL_PULL_DOWNR {
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
            YPLL_PULL_DOWNR::YPLL_PULL_DOWN_0 => false,
            YPLL_PULL_DOWNR::YPLL_PULL_DOWN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> YPLL_PULL_DOWNR {
        match value {
            false => YPLL_PULL_DOWNR::YPLL_PULL_DOWN_0,
            true => YPLL_PULL_DOWNR::YPLL_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_DOWN_0`"]
    #[inline]
    pub fn is_ypll_pull_down_0(&self) -> bool {
        *self == YPLL_PULL_DOWNR::YPLL_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_DOWN_1`"]
    #[inline]
    pub fn is_ypll_pull_down_1(&self) -> bool {
        *self == YPLL_PULL_DOWNR::YPLL_PULL_DOWN_1
    }
}
#[doc = "Possible values of the field `YPLL_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YPLL_PULL_UPR {
    #[doc = "Close the switch"]
    YPLL_PULL_UP_0,
    #[doc = "Open the switch"]
    YPLL_PULL_UP_1,
}
impl YPLL_PULL_UPR {
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
            YPLL_PULL_UPR::YPLL_PULL_UP_0 => false,
            YPLL_PULL_UPR::YPLL_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> YPLL_PULL_UPR {
        match value {
            false => YPLL_PULL_UPR::YPLL_PULL_UP_0,
            true => YPLL_PULL_UPR::YPLL_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_UP_0`"]
    #[inline]
    pub fn is_ypll_pull_up_0(&self) -> bool {
        *self == YPLL_PULL_UPR::YPLL_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_UP_1`"]
    #[inline]
    pub fn is_ypll_pull_up_1(&self) -> bool {
        *self == YPLL_PULL_UPR::YPLL_PULL_UP_1
    }
}
#[doc = "Possible values of the field `YPLL_200K_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YPLL_200K_PULL_UPR {
    #[doc = "Close the switch"]
    YPLL_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    YPLL_200K_PULL_UP_1,
}
impl YPLL_200K_PULL_UPR {
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
            YPLL_200K_PULL_UPR::YPLL_200K_PULL_UP_0 => false,
            YPLL_200K_PULL_UPR::YPLL_200K_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> YPLL_200K_PULL_UPR {
        match value {
            false => YPLL_200K_PULL_UPR::YPLL_200K_PULL_UP_0,
            true => YPLL_200K_PULL_UPR::YPLL_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YPLL_200K_PULL_UP_0`"]
    #[inline]
    pub fn is_ypll_200k_pull_up_0(&self) -> bool {
        *self == YPLL_200K_PULL_UPR::YPLL_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YPLL_200K_PULL_UP_1`"]
    #[inline]
    pub fn is_ypll_200k_pull_up_1(&self) -> bool {
        *self == YPLL_200K_PULL_UPR::YPLL_200K_PULL_UP_1
    }
}
#[doc = "Possible values of the field `YNLR_PULL_DOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YNLR_PULL_DOWNR {
    #[doc = "Close the switch"]
    YNLR_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    YNLR_PULL_DOWN_1,
}
impl YNLR_PULL_DOWNR {
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
            YNLR_PULL_DOWNR::YNLR_PULL_DOWN_0 => false,
            YNLR_PULL_DOWNR::YNLR_PULL_DOWN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> YNLR_PULL_DOWNR {
        match value {
            false => YNLR_PULL_DOWNR::YNLR_PULL_DOWN_0,
            true => YNLR_PULL_DOWNR::YNLR_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_DOWN_0`"]
    #[inline]
    pub fn is_ynlr_pull_down_0(&self) -> bool {
        *self == YNLR_PULL_DOWNR::YNLR_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_DOWN_1`"]
    #[inline]
    pub fn is_ynlr_pull_down_1(&self) -> bool {
        *self == YNLR_PULL_DOWNR::YNLR_PULL_DOWN_1
    }
}
#[doc = "Possible values of the field `YNLR_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YNLR_PULL_UPR {
    #[doc = "Close the switch"]
    YNLR_PULL_UP_0,
    #[doc = "Open up the switch"]
    YNLR_PULL_UP_1,
}
impl YNLR_PULL_UPR {
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
            YNLR_PULL_UPR::YNLR_PULL_UP_0 => false,
            YNLR_PULL_UPR::YNLR_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> YNLR_PULL_UPR {
        match value {
            false => YNLR_PULL_UPR::YNLR_PULL_UP_0,
            true => YNLR_PULL_UPR::YNLR_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_UP_0`"]
    #[inline]
    pub fn is_ynlr_pull_up_0(&self) -> bool {
        *self == YNLR_PULL_UPR::YNLR_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_UP_1`"]
    #[inline]
    pub fn is_ynlr_pull_up_1(&self) -> bool {
        *self == YNLR_PULL_UPR::YNLR_PULL_UP_1
    }
}
#[doc = "Possible values of the field `YNLR_200K_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YNLR_200K_PULL_UPR {
    #[doc = "Close the switch"]
    YNLR_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    YNLR_200K_PULL_UP_1,
}
impl YNLR_200K_PULL_UPR {
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
            YNLR_200K_PULL_UPR::YNLR_200K_PULL_UP_0 => false,
            YNLR_200K_PULL_UPR::YNLR_200K_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> YNLR_200K_PULL_UPR {
        match value {
            false => YNLR_200K_PULL_UPR::YNLR_200K_PULL_UP_0,
            true => YNLR_200K_PULL_UPR::YNLR_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YNLR_200K_PULL_UP_0`"]
    #[inline]
    pub fn is_ynlr_200k_pull_up_0(&self) -> bool {
        *self == YNLR_200K_PULL_UPR::YNLR_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YNLR_200K_PULL_UP_1`"]
    #[inline]
    pub fn is_ynlr_200k_pull_up_1(&self) -> bool {
        *self == YNLR_200K_PULL_UPR::YNLR_200K_PULL_UP_1
    }
}
#[doc = "Possible values of the field `WIPER_PULL_DOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPER_PULL_DOWNR {
    #[doc = "Close the switch"]
    WIPER_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    WIPER_PULL_DOWN_1,
}
impl WIPER_PULL_DOWNR {
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
            WIPER_PULL_DOWNR::WIPER_PULL_DOWN_0 => false,
            WIPER_PULL_DOWNR::WIPER_PULL_DOWN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIPER_PULL_DOWNR {
        match value {
            false => WIPER_PULL_DOWNR::WIPER_PULL_DOWN_0,
            true => WIPER_PULL_DOWNR::WIPER_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_DOWN_0`"]
    #[inline]
    pub fn is_wiper_pull_down_0(&self) -> bool {
        *self == WIPER_PULL_DOWNR::WIPER_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_DOWN_1`"]
    #[inline]
    pub fn is_wiper_pull_down_1(&self) -> bool {
        *self == WIPER_PULL_DOWNR::WIPER_PULL_DOWN_1
    }
}
#[doc = "Possible values of the field `WIPER_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPER_PULL_UPR {
    #[doc = "Close the switch"]
    WIPER_PULL_UP_0,
    #[doc = "Open up the switch"]
    WIPER_PULL_UP_1,
}
impl WIPER_PULL_UPR {
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
            WIPER_PULL_UPR::WIPER_PULL_UP_0 => false,
            WIPER_PULL_UPR::WIPER_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIPER_PULL_UPR {
        match value {
            false => WIPER_PULL_UPR::WIPER_PULL_UP_0,
            true => WIPER_PULL_UPR::WIPER_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_UP_0`"]
    #[inline]
    pub fn is_wiper_pull_up_0(&self) -> bool {
        *self == WIPER_PULL_UPR::WIPER_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_UP_1`"]
    #[inline]
    pub fn is_wiper_pull_up_1(&self) -> bool {
        *self == WIPER_PULL_UPR::WIPER_PULL_UP_1
    }
}
#[doc = "Possible values of the field `WIPER_200K_PULL_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPER_200K_PULL_UPR {
    #[doc = "Close the switch"]
    WIPER_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    WIPER_200K_PULL_UP_1,
}
impl WIPER_200K_PULL_UPR {
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
            WIPER_200K_PULL_UPR::WIPER_200K_PULL_UP_0 => false,
            WIPER_200K_PULL_UPR::WIPER_200K_PULL_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIPER_200K_PULL_UPR {
        match value {
            false => WIPER_200K_PULL_UPR::WIPER_200K_PULL_UP_0,
            true => WIPER_200K_PULL_UPR::WIPER_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIPER_200K_PULL_UP_0`"]
    #[inline]
    pub fn is_wiper_200k_pull_up_0(&self) -> bool {
        *self == WIPER_200K_PULL_UPR::WIPER_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `WIPER_200K_PULL_UP_1`"]
    #[inline]
    pub fn is_wiper_200k_pull_up_1(&self) -> bool {
        *self == WIPER_200K_PULL_UPR::WIPER_200K_PULL_UP_1
    }
}
#[doc = "Possible values of the field `DETECT_FOUR_WIRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_FOUR_WIRER {
    #[doc = "No detect signal"]
    DETECT_FOUR_WIRE_0,
    #[doc = "Yes, there is a detect on the touch screen."]
    DETECT_FOUR_WIRE_1,
}
impl DETECT_FOUR_WIRER {
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
            DETECT_FOUR_WIRER::DETECT_FOUR_WIRE_0 => false,
            DETECT_FOUR_WIRER::DETECT_FOUR_WIRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DETECT_FOUR_WIRER {
        match value {
            false => DETECT_FOUR_WIRER::DETECT_FOUR_WIRE_0,
            true => DETECT_FOUR_WIRER::DETECT_FOUR_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_FOUR_WIRE_0`"]
    #[inline]
    pub fn is_detect_four_wire_0(&self) -> bool {
        *self == DETECT_FOUR_WIRER::DETECT_FOUR_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_FOUR_WIRE_1`"]
    #[inline]
    pub fn is_detect_four_wire_1(&self) -> bool {
        *self == DETECT_FOUR_WIRER::DETECT_FOUR_WIRE_1
    }
}
#[doc = "Possible values of the field `DETECT_FIVE_WIRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_FIVE_WIRER {
    #[doc = "No detect signal"]
    DETECT_FIVE_WIRE_0,
    #[doc = "Yes, there is a detect on the touch screen."]
    DETECT_FIVE_WIRE_1,
}
impl DETECT_FIVE_WIRER {
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
            DETECT_FIVE_WIRER::DETECT_FIVE_WIRE_0 => false,
            DETECT_FIVE_WIRER::DETECT_FIVE_WIRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DETECT_FIVE_WIRER {
        match value {
            false => DETECT_FIVE_WIRER::DETECT_FIVE_WIRE_0,
            true => DETECT_FIVE_WIRER::DETECT_FIVE_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_FIVE_WIRE_0`"]
    #[inline]
    pub fn is_detect_five_wire_0(&self) -> bool {
        *self == DETECT_FIVE_WIRER::DETECT_FIVE_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_FIVE_WIRE_1`"]
    #[inline]
    pub fn is_detect_five_wire_1(&self) -> bool {
        *self == DETECT_FIVE_WIRER::DETECT_FIVE_WIRE_1
    }
}
#[doc = "Possible values of the field `STATE_MACHINE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_MACHINER {
    #[doc = "Idle"]
    STATE_MACHINE_0,
    #[doc = "Pre-charge"]
    STATE_MACHINE_1,
    #[doc = "Detect"]
    STATE_MACHINE_2,
    #[doc = "X-measure"]
    STATE_MACHINE_3,
    #[doc = "Y-measure"]
    STATE_MACHINE_4,
    #[doc = "Pre-charge"]
    STATE_MACHINE_5,
    #[doc = "Detect"]
    STATE_MACHINE_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATE_MACHINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATE_MACHINER::STATE_MACHINE_0 => 0,
            STATE_MACHINER::STATE_MACHINE_1 => 1,
            STATE_MACHINER::STATE_MACHINE_2 => 2,
            STATE_MACHINER::STATE_MACHINE_3 => 3,
            STATE_MACHINER::STATE_MACHINE_4 => 4,
            STATE_MACHINER::STATE_MACHINE_5 => 5,
            STATE_MACHINER::STATE_MACHINE_6 => 6,
            STATE_MACHINER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATE_MACHINER {
        match value {
            0 => STATE_MACHINER::STATE_MACHINE_0,
            1 => STATE_MACHINER::STATE_MACHINE_1,
            2 => STATE_MACHINER::STATE_MACHINE_2,
            3 => STATE_MACHINER::STATE_MACHINE_3,
            4 => STATE_MACHINER::STATE_MACHINE_4,
            5 => STATE_MACHINER::STATE_MACHINE_5,
            6 => STATE_MACHINER::STATE_MACHINE_6,
            i => STATE_MACHINER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_0`"]
    #[inline]
    pub fn is_state_machine_0(&self) -> bool {
        *self == STATE_MACHINER::STATE_MACHINE_0
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_1`"]
    #[inline]
    pub fn is_state_machine_1(&self) -> bool {
        *self == STATE_MACHINER::STATE_MACHINE_1
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_2`"]
    #[inline]
    pub fn is_state_machine_2(&self) -> bool {
        *self == STATE_MACHINER::STATE_MACHINE_2
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_3`"]
    #[inline]
    pub fn is_state_machine_3(&self) -> bool {
        *self == STATE_MACHINER::STATE_MACHINE_3
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_4`"]
    #[inline]
    pub fn is_state_machine_4(&self) -> bool {
        *self == STATE_MACHINER::STATE_MACHINE_4
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_5`"]
    #[inline]
    pub fn is_state_machine_5(&self) -> bool {
        *self == STATE_MACHINER::STATE_MACHINE_5
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_6`"]
    #[inline]
    pub fn is_state_machine_6(&self) -> bool {
        *self == STATE_MACHINER::STATE_MACHINE_6
    }
}
#[doc = "Possible values of the field `INTERMEDIATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERMEDIATER {
    #[doc = "Not in intermedia"]
    INTERMEDIATE_0,
    #[doc = "Intermedia"]
    INTERMEDIATE_1,
}
impl INTERMEDIATER {
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
            INTERMEDIATER::INTERMEDIATE_0 => false,
            INTERMEDIATER::INTERMEDIATE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTERMEDIATER {
        match value {
            false => INTERMEDIATER::INTERMEDIATE_0,
            true => INTERMEDIATER::INTERMEDIATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE_0`"]
    #[inline]
    pub fn is_intermediate_0(&self) -> bool {
        *self == INTERMEDIATER::INTERMEDIATE_0
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE_1`"]
    #[inline]
    pub fn is_intermediate_1(&self) -> bool {
        *self == INTERMEDIATER::INTERMEDIATE_1
    }
}
#[doc = "Possible values of the field `DETECT_ENABLE_FOUR_WIRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_ENABLE_FOUR_WIRER {
    #[doc = "Do not read four wire detect value, read default value from analogue"]
    DETECT_ENABLE_FOUR_WIRE_0,
    #[doc = "Read four wire detect status from analogue"]
    DETECT_ENABLE_FOUR_WIRE_1,
}
impl DETECT_ENABLE_FOUR_WIRER {
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
            DETECT_ENABLE_FOUR_WIRER::DETECT_ENABLE_FOUR_WIRE_0 => false,
            DETECT_ENABLE_FOUR_WIRER::DETECT_ENABLE_FOUR_WIRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DETECT_ENABLE_FOUR_WIRER {
        match value {
            false => DETECT_ENABLE_FOUR_WIRER::DETECT_ENABLE_FOUR_WIRE_0,
            true => DETECT_ENABLE_FOUR_WIRER::DETECT_ENABLE_FOUR_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FOUR_WIRE_0`"]
    #[inline]
    pub fn is_detect_enable_four_wire_0(&self) -> bool {
        *self == DETECT_ENABLE_FOUR_WIRER::DETECT_ENABLE_FOUR_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FOUR_WIRE_1`"]
    #[inline]
    pub fn is_detect_enable_four_wire_1(&self) -> bool {
        *self == DETECT_ENABLE_FOUR_WIRER::DETECT_ENABLE_FOUR_WIRE_1
    }
}
#[doc = "Possible values of the field `DETECT_ENABLE_FIVE_WIRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_ENABLE_FIVE_WIRER {
    #[doc = "Do not read five wire detect value, read default value from analogue"]
    DETECT_ENABLE_FIVE_WIRE_0,
    #[doc = "Read five wire detect status from analogue"]
    DETECT_ENABLE_FIVE_WIRE_1,
}
impl DETECT_ENABLE_FIVE_WIRER {
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
            DETECT_ENABLE_FIVE_WIRER::DETECT_ENABLE_FIVE_WIRE_0 => false,
            DETECT_ENABLE_FIVE_WIRER::DETECT_ENABLE_FIVE_WIRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DETECT_ENABLE_FIVE_WIRER {
        match value {
            false => DETECT_ENABLE_FIVE_WIRER::DETECT_ENABLE_FIVE_WIRE_0,
            true => DETECT_ENABLE_FIVE_WIRER::DETECT_ENABLE_FIVE_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FIVE_WIRE_0`"]
    #[inline]
    pub fn is_detect_enable_five_wire_0(&self) -> bool {
        *self == DETECT_ENABLE_FIVE_WIRER::DETECT_ENABLE_FIVE_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FIVE_WIRE_1`"]
    #[inline]
    pub fn is_detect_enable_five_wire_1(&self) -> bool {
        *self == DETECT_ENABLE_FIVE_WIRER::DETECT_ENABLE_FIVE_WIRE_1
    }
}
#[doc = "Possible values of the field `DE_GLITCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DE_GLITCHR {
    #[doc = "Normal function: 0x1fff ipg clock cycles; Low power mode: 0x9 low power clock cycles"]
    DE_GLITCH_0,
    #[doc = "Normal function: 0xfff ipg clock cycles; Low power mode: :0x7 low power clock cycles"]
    DE_GLITCH_1,
    #[doc = "Normal function: 0x7ff ipg clock cycles; Low power mode:0x5 low power clock cycles"]
    DE_GLITCH_2,
    #[doc = "Normal function: 0x3 ipg clock cycles; Low power mode:0x3 low power clock cycles"]
    DE_GLITCH_3,
}
impl DE_GLITCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DE_GLITCHR::DE_GLITCH_0 => 0,
            DE_GLITCHR::DE_GLITCH_1 => 1,
            DE_GLITCHR::DE_GLITCH_2 => 2,
            DE_GLITCHR::DE_GLITCH_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DE_GLITCHR {
        match value {
            0 => DE_GLITCHR::DE_GLITCH_0,
            1 => DE_GLITCHR::DE_GLITCH_1,
            2 => DE_GLITCHR::DE_GLITCH_2,
            3 => DE_GLITCHR::DE_GLITCH_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_0`"]
    #[inline]
    pub fn is_de_glitch_0(&self) -> bool {
        *self == DE_GLITCHR::DE_GLITCH_0
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_1`"]
    #[inline]
    pub fn is_de_glitch_1(&self) -> bool {
        *self == DE_GLITCHR::DE_GLITCH_1
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_2`"]
    #[inline]
    pub fn is_de_glitch_2(&self) -> bool {
        *self == DE_GLITCHR::DE_GLITCH_2
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_3`"]
    #[inline]
    pub fn is_de_glitch_3(&self) -> bool {
        *self == DE_GLITCHR::DE_GLITCH_3
    }
}
#[doc = "Values that can be written to the field `XPUL_PULL_DOWN`"]
pub enum XPUL_PULL_DOWNW {
    #[doc = "Close the switch"]
    XPUL_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    XPUL_PULL_DOWN_1,
}
impl XPUL_PULL_DOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XPUL_PULL_DOWNW::XPUL_PULL_DOWN_0 => false,
            XPUL_PULL_DOWNW::XPUL_PULL_DOWN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XPUL_PULL_DOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _XPUL_PULL_DOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XPUL_PULL_DOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn xpul_pull_down_0(self) -> &'a mut W {
        self.variant(XPUL_PULL_DOWNW::XPUL_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn xpul_pull_down_1(self) -> &'a mut W {
        self.variant(XPUL_PULL_DOWNW::XPUL_PULL_DOWN_1)
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
#[doc = "Values that can be written to the field `XPUL_PULL_UP`"]
pub enum XPUL_PULL_UPW {
    #[doc = "Close the switch"]
    XPUL_PULL_UP_0,
    #[doc = "Open up the switch"]
    XPUL_PULL_UP_1,
}
impl XPUL_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XPUL_PULL_UPW::XPUL_PULL_UP_0 => false,
            XPUL_PULL_UPW::XPUL_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XPUL_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _XPUL_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XPUL_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn xpul_pull_up_0(self) -> &'a mut W {
        self.variant(XPUL_PULL_UPW::XPUL_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn xpul_pull_up_1(self) -> &'a mut W {
        self.variant(XPUL_PULL_UPW::XPUL_PULL_UP_1)
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
#[doc = "Values that can be written to the field `XPUL_200K_PULL_UP`"]
pub enum XPUL_200K_PULL_UPW {
    #[doc = "Close the switch"]
    XPUL_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    XPUL_200K_PULL_UP_1,
}
impl XPUL_200K_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XPUL_200K_PULL_UPW::XPUL_200K_PULL_UP_0 => false,
            XPUL_200K_PULL_UPW::XPUL_200K_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XPUL_200K_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _XPUL_200K_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XPUL_200K_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn xpul_200k_pull_up_0(self) -> &'a mut W {
        self.variant(XPUL_200K_PULL_UPW::XPUL_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn xpul_200k_pull_up_1(self) -> &'a mut W {
        self.variant(XPUL_200K_PULL_UPW::XPUL_200K_PULL_UP_1)
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
#[doc = "Values that can be written to the field `XNUR_PULL_DOWN`"]
pub enum XNUR_PULL_DOWNW {
    #[doc = "Close the switch"]
    XNUR_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    XNUR_PULL_DOWN_1,
}
impl XNUR_PULL_DOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XNUR_PULL_DOWNW::XNUR_PULL_DOWN_0 => false,
            XNUR_PULL_DOWNW::XNUR_PULL_DOWN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XNUR_PULL_DOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _XNUR_PULL_DOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XNUR_PULL_DOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn xnur_pull_down_0(self) -> &'a mut W {
        self.variant(XNUR_PULL_DOWNW::XNUR_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn xnur_pull_down_1(self) -> &'a mut W {
        self.variant(XNUR_PULL_DOWNW::XNUR_PULL_DOWN_1)
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
#[doc = "Values that can be written to the field `XNUR_PULL_UP`"]
pub enum XNUR_PULL_UPW {
    #[doc = "Close the switch"]
    XNUR_PULL_UP_0,
    #[doc = "Open up the switch"]
    XNUR_PULL_UP_1,
}
impl XNUR_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XNUR_PULL_UPW::XNUR_PULL_UP_0 => false,
            XNUR_PULL_UPW::XNUR_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XNUR_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _XNUR_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XNUR_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn xnur_pull_up_0(self) -> &'a mut W {
        self.variant(XNUR_PULL_UPW::XNUR_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn xnur_pull_up_1(self) -> &'a mut W {
        self.variant(XNUR_PULL_UPW::XNUR_PULL_UP_1)
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
#[doc = "Values that can be written to the field `XNUR_200K_PULL_UP`"]
pub enum XNUR_200K_PULL_UPW {
    #[doc = "Close the switch"]
    XNUR_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    XNUR_200K_PULL_UP_1,
}
impl XNUR_200K_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XNUR_200K_PULL_UPW::XNUR_200K_PULL_UP_0 => false,
            XNUR_200K_PULL_UPW::XNUR_200K_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XNUR_200K_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _XNUR_200K_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XNUR_200K_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn xnur_200k_pull_up_0(self) -> &'a mut W {
        self.variant(XNUR_200K_PULL_UPW::XNUR_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn xnur_200k_pull_up_1(self) -> &'a mut W {
        self.variant(XNUR_200K_PULL_UPW::XNUR_200K_PULL_UP_1)
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
#[doc = "Values that can be written to the field `YPLL_PULL_DOWN`"]
pub enum YPLL_PULL_DOWNW {
    #[doc = "Close the switch"]
    YPLL_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    YPLL_PULL_DOWN_1,
}
impl YPLL_PULL_DOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            YPLL_PULL_DOWNW::YPLL_PULL_DOWN_0 => false,
            YPLL_PULL_DOWNW::YPLL_PULL_DOWN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _YPLL_PULL_DOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _YPLL_PULL_DOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: YPLL_PULL_DOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn ypll_pull_down_0(self) -> &'a mut W {
        self.variant(YPLL_PULL_DOWNW::YPLL_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn ypll_pull_down_1(self) -> &'a mut W {
        self.variant(YPLL_PULL_DOWNW::YPLL_PULL_DOWN_1)
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
#[doc = "Values that can be written to the field `YPLL_PULL_UP`"]
pub enum YPLL_PULL_UPW {
    #[doc = "Close the switch"]
    YPLL_PULL_UP_0,
    #[doc = "Open the switch"]
    YPLL_PULL_UP_1,
}
impl YPLL_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            YPLL_PULL_UPW::YPLL_PULL_UP_0 => false,
            YPLL_PULL_UPW::YPLL_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _YPLL_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _YPLL_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: YPLL_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn ypll_pull_up_0(self) -> &'a mut W {
        self.variant(YPLL_PULL_UPW::YPLL_PULL_UP_0)
    }
    #[doc = "Open the switch"]
    #[inline]
    pub fn ypll_pull_up_1(self) -> &'a mut W {
        self.variant(YPLL_PULL_UPW::YPLL_PULL_UP_1)
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
#[doc = "Values that can be written to the field `YPLL_200K_PULL_UP`"]
pub enum YPLL_200K_PULL_UPW {
    #[doc = "Close the switch"]
    YPLL_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    YPLL_200K_PULL_UP_1,
}
impl YPLL_200K_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            YPLL_200K_PULL_UPW::YPLL_200K_PULL_UP_0 => false,
            YPLL_200K_PULL_UPW::YPLL_200K_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _YPLL_200K_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _YPLL_200K_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: YPLL_200K_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn ypll_200k_pull_up_0(self) -> &'a mut W {
        self.variant(YPLL_200K_PULL_UPW::YPLL_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn ypll_200k_pull_up_1(self) -> &'a mut W {
        self.variant(YPLL_200K_PULL_UPW::YPLL_200K_PULL_UP_1)
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
#[doc = "Values that can be written to the field `YNLR_PULL_DOWN`"]
pub enum YNLR_PULL_DOWNW {
    #[doc = "Close the switch"]
    YNLR_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    YNLR_PULL_DOWN_1,
}
impl YNLR_PULL_DOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            YNLR_PULL_DOWNW::YNLR_PULL_DOWN_0 => false,
            YNLR_PULL_DOWNW::YNLR_PULL_DOWN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _YNLR_PULL_DOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _YNLR_PULL_DOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: YNLR_PULL_DOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn ynlr_pull_down_0(self) -> &'a mut W {
        self.variant(YNLR_PULL_DOWNW::YNLR_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn ynlr_pull_down_1(self) -> &'a mut W {
        self.variant(YNLR_PULL_DOWNW::YNLR_PULL_DOWN_1)
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
#[doc = "Values that can be written to the field `YNLR_PULL_UP`"]
pub enum YNLR_PULL_UPW {
    #[doc = "Close the switch"]
    YNLR_PULL_UP_0,
    #[doc = "Open up the switch"]
    YNLR_PULL_UP_1,
}
impl YNLR_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            YNLR_PULL_UPW::YNLR_PULL_UP_0 => false,
            YNLR_PULL_UPW::YNLR_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _YNLR_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _YNLR_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: YNLR_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn ynlr_pull_up_0(self) -> &'a mut W {
        self.variant(YNLR_PULL_UPW::YNLR_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn ynlr_pull_up_1(self) -> &'a mut W {
        self.variant(YNLR_PULL_UPW::YNLR_PULL_UP_1)
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
#[doc = "Values that can be written to the field `YNLR_200K_PULL_UP`"]
pub enum YNLR_200K_PULL_UPW {
    #[doc = "Close the switch"]
    YNLR_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    YNLR_200K_PULL_UP_1,
}
impl YNLR_200K_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            YNLR_200K_PULL_UPW::YNLR_200K_PULL_UP_0 => false,
            YNLR_200K_PULL_UPW::YNLR_200K_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _YNLR_200K_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _YNLR_200K_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: YNLR_200K_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn ynlr_200k_pull_up_0(self) -> &'a mut W {
        self.variant(YNLR_200K_PULL_UPW::YNLR_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn ynlr_200k_pull_up_1(self) -> &'a mut W {
        self.variant(YNLR_200K_PULL_UPW::YNLR_200K_PULL_UP_1)
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
#[doc = "Values that can be written to the field `WIPER_PULL_DOWN`"]
pub enum WIPER_PULL_DOWNW {
    #[doc = "Close the switch"]
    WIPER_PULL_DOWN_0,
    #[doc = "Open up the switch"]
    WIPER_PULL_DOWN_1,
}
impl WIPER_PULL_DOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WIPER_PULL_DOWNW::WIPER_PULL_DOWN_0 => false,
            WIPER_PULL_DOWNW::WIPER_PULL_DOWN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIPER_PULL_DOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _WIPER_PULL_DOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIPER_PULL_DOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn wiper_pull_down_0(self) -> &'a mut W {
        self.variant(WIPER_PULL_DOWNW::WIPER_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn wiper_pull_down_1(self) -> &'a mut W {
        self.variant(WIPER_PULL_DOWNW::WIPER_PULL_DOWN_1)
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
#[doc = "Values that can be written to the field `WIPER_PULL_UP`"]
pub enum WIPER_PULL_UPW {
    #[doc = "Close the switch"]
    WIPER_PULL_UP_0,
    #[doc = "Open up the switch"]
    WIPER_PULL_UP_1,
}
impl WIPER_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WIPER_PULL_UPW::WIPER_PULL_UP_0 => false,
            WIPER_PULL_UPW::WIPER_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIPER_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _WIPER_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIPER_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn wiper_pull_up_0(self) -> &'a mut W {
        self.variant(WIPER_PULL_UPW::WIPER_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn wiper_pull_up_1(self) -> &'a mut W {
        self.variant(WIPER_PULL_UPW::WIPER_PULL_UP_1)
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
#[doc = "Values that can be written to the field `WIPER_200K_PULL_UP`"]
pub enum WIPER_200K_PULL_UPW {
    #[doc = "Close the switch"]
    WIPER_200K_PULL_UP_0,
    #[doc = "Open up the switch"]
    WIPER_200K_PULL_UP_1,
}
impl WIPER_200K_PULL_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WIPER_200K_PULL_UPW::WIPER_200K_PULL_UP_0 => false,
            WIPER_200K_PULL_UPW::WIPER_200K_PULL_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIPER_200K_PULL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _WIPER_200K_PULL_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIPER_200K_PULL_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Close the switch"]
    #[inline]
    pub fn wiper_200k_pull_up_0(self) -> &'a mut W {
        self.variant(WIPER_200K_PULL_UPW::WIPER_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline]
    pub fn wiper_200k_pull_up_1(self) -> &'a mut W {
        self.variant(WIPER_200K_PULL_UPW::WIPER_200K_PULL_UP_1)
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
#[doc = "Values that can be written to the field `DETECT_ENABLE_FOUR_WIRE`"]
pub enum DETECT_ENABLE_FOUR_WIREW {
    #[doc = "Do not read four wire detect value, read default value from analogue"]
    DETECT_ENABLE_FOUR_WIRE_0,
    #[doc = "Read four wire detect status from analogue"]
    DETECT_ENABLE_FOUR_WIRE_1,
}
impl DETECT_ENABLE_FOUR_WIREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DETECT_ENABLE_FOUR_WIREW::DETECT_ENABLE_FOUR_WIRE_0 => false,
            DETECT_ENABLE_FOUR_WIREW::DETECT_ENABLE_FOUR_WIRE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DETECT_ENABLE_FOUR_WIREW<'a> {
    w: &'a mut W,
}
impl<'a> _DETECT_ENABLE_FOUR_WIREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DETECT_ENABLE_FOUR_WIREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not read four wire detect value, read default value from analogue"]
    #[inline]
    pub fn detect_enable_four_wire_0(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FOUR_WIREW::DETECT_ENABLE_FOUR_WIRE_0)
    }
    #[doc = "Read four wire detect status from analogue"]
    #[inline]
    pub fn detect_enable_four_wire_1(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FOUR_WIREW::DETECT_ENABLE_FOUR_WIRE_1)
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
#[doc = "Values that can be written to the field `DETECT_ENABLE_FIVE_WIRE`"]
pub enum DETECT_ENABLE_FIVE_WIREW {
    #[doc = "Do not read five wire detect value, read default value from analogue"]
    DETECT_ENABLE_FIVE_WIRE_0,
    #[doc = "Read five wire detect status from analogue"]
    DETECT_ENABLE_FIVE_WIRE_1,
}
impl DETECT_ENABLE_FIVE_WIREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DETECT_ENABLE_FIVE_WIREW::DETECT_ENABLE_FIVE_WIRE_0 => false,
            DETECT_ENABLE_FIVE_WIREW::DETECT_ENABLE_FIVE_WIRE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DETECT_ENABLE_FIVE_WIREW<'a> {
    w: &'a mut W,
}
impl<'a> _DETECT_ENABLE_FIVE_WIREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DETECT_ENABLE_FIVE_WIREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not read five wire detect value, read default value from analogue"]
    #[inline]
    pub fn detect_enable_five_wire_0(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FIVE_WIREW::DETECT_ENABLE_FIVE_WIRE_0)
    }
    #[doc = "Read five wire detect status from analogue"]
    #[inline]
    pub fn detect_enable_five_wire_1(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FIVE_WIREW::DETECT_ENABLE_FIVE_WIRE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - XPUL Wire Pull Down Switch"]
    #[inline]
    pub fn xpul_pull_down(&self) -> XPUL_PULL_DOWNR {
        XPUL_PULL_DOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - XPUL Wire Pull Up Switch"]
    #[inline]
    pub fn xpul_pull_up(&self) -> XPUL_PULL_UPR {
        XPUL_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - XPUL Wire 200K Pull Up Switch"]
    #[inline]
    pub fn xpul_200k_pull_up(&self) -> XPUL_200K_PULL_UPR {
        XPUL_200K_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - XNUR Wire Pull Down Switch"]
    #[inline]
    pub fn xnur_pull_down(&self) -> XNUR_PULL_DOWNR {
        XNUR_PULL_DOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - XNUR Wire Pull Up Switch"]
    #[inline]
    pub fn xnur_pull_up(&self) -> XNUR_PULL_UPR {
        XNUR_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - XNUR Wire 200K Pull Up Switch"]
    #[inline]
    pub fn xnur_200k_pull_up(&self) -> XNUR_200K_PULL_UPR {
        XNUR_200K_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - YPLL Wire Pull Down Switch"]
    #[inline]
    pub fn ypll_pull_down(&self) -> YPLL_PULL_DOWNR {
        YPLL_PULL_DOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - YPLL Wire Pull Up Switch"]
    #[inline]
    pub fn ypll_pull_up(&self) -> YPLL_PULL_UPR {
        YPLL_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - YPLL Wire 200K Pull Up Switch"]
    #[inline]
    pub fn ypll_200k_pull_up(&self) -> YPLL_200K_PULL_UPR {
        YPLL_200K_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - YNLR Wire Pull Down Switch"]
    #[inline]
    pub fn ynlr_pull_down(&self) -> YNLR_PULL_DOWNR {
        YNLR_PULL_DOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - YNLR Wire Pull Up Switch"]
    #[inline]
    pub fn ynlr_pull_up(&self) -> YNLR_PULL_UPR {
        YNLR_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - YNLR Wire 200K Pull Up Switch"]
    #[inline]
    pub fn ynlr_200k_pull_up(&self) -> YNLR_200K_PULL_UPR {
        YNLR_200K_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wiper Wire Pull Down Switch"]
    #[inline]
    pub fn wiper_pull_down(&self) -> WIPER_PULL_DOWNR {
        WIPER_PULL_DOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Wiper Wire Pull Up Switch"]
    #[inline]
    pub fn wiper_pull_up(&self) -> WIPER_PULL_UPR {
        WIPER_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Wiper Wire 200K Pull Up Switch"]
    #[inline]
    pub fn wiper_200k_pull_up(&self) -> WIPER_200K_PULL_UPR {
        WIPER_200K_PULL_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Detect Four Wire"]
    #[inline]
    pub fn detect_four_wire(&self) -> DETECT_FOUR_WIRER {
        DETECT_FOUR_WIRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Detect Five Wire"]
    #[inline]
    pub fn detect_five_wire(&self) -> DETECT_FIVE_WIRER {
        DETECT_FIVE_WIRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:22 - State Machine"]
    #[inline]
    pub fn state_machine(&self) -> STATE_MACHINER {
        STATE_MACHINER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Intermediate State"]
    #[inline]
    pub fn intermediate(&self) -> INTERMEDIATER {
        INTERMEDIATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Detect Enable Four Wire"]
    #[inline]
    pub fn detect_enable_four_wire(&self) -> DETECT_ENABLE_FOUR_WIRER {
        DETECT_ENABLE_FOUR_WIRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Detect Enable Five Wire"]
    #[inline]
    pub fn detect_enable_five_wire(&self) -> DETECT_ENABLE_FIVE_WIRER {
        DETECT_ENABLE_FIVE_WIRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:30 - This field indicates glitch threshold"]
    #[inline]
    pub fn de_glitch(&self) -> DE_GLITCHR {
        DE_GLITCHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - XPUL Wire Pull Down Switch"]
    #[inline]
    pub fn xpul_pull_down(&mut self) -> _XPUL_PULL_DOWNW {
        _XPUL_PULL_DOWNW { w: self }
    }
    #[doc = "Bit 1 - XPUL Wire Pull Up Switch"]
    #[inline]
    pub fn xpul_pull_up(&mut self) -> _XPUL_PULL_UPW {
        _XPUL_PULL_UPW { w: self }
    }
    #[doc = "Bit 2 - XPUL Wire 200K Pull Up Switch"]
    #[inline]
    pub fn xpul_200k_pull_up(&mut self) -> _XPUL_200K_PULL_UPW {
        _XPUL_200K_PULL_UPW { w: self }
    }
    #[doc = "Bit 3 - XNUR Wire Pull Down Switch"]
    #[inline]
    pub fn xnur_pull_down(&mut self) -> _XNUR_PULL_DOWNW {
        _XNUR_PULL_DOWNW { w: self }
    }
    #[doc = "Bit 4 - XNUR Wire Pull Up Switch"]
    #[inline]
    pub fn xnur_pull_up(&mut self) -> _XNUR_PULL_UPW {
        _XNUR_PULL_UPW { w: self }
    }
    #[doc = "Bit 5 - XNUR Wire 200K Pull Up Switch"]
    #[inline]
    pub fn xnur_200k_pull_up(&mut self) -> _XNUR_200K_PULL_UPW {
        _XNUR_200K_PULL_UPW { w: self }
    }
    #[doc = "Bit 6 - YPLL Wire Pull Down Switch"]
    #[inline]
    pub fn ypll_pull_down(&mut self) -> _YPLL_PULL_DOWNW {
        _YPLL_PULL_DOWNW { w: self }
    }
    #[doc = "Bit 7 - YPLL Wire Pull Up Switch"]
    #[inline]
    pub fn ypll_pull_up(&mut self) -> _YPLL_PULL_UPW {
        _YPLL_PULL_UPW { w: self }
    }
    #[doc = "Bit 8 - YPLL Wire 200K Pull Up Switch"]
    #[inline]
    pub fn ypll_200k_pull_up(&mut self) -> _YPLL_200K_PULL_UPW {
        _YPLL_200K_PULL_UPW { w: self }
    }
    #[doc = "Bit 9 - YNLR Wire Pull Down Switch"]
    #[inline]
    pub fn ynlr_pull_down(&mut self) -> _YNLR_PULL_DOWNW {
        _YNLR_PULL_DOWNW { w: self }
    }
    #[doc = "Bit 10 - YNLR Wire Pull Up Switch"]
    #[inline]
    pub fn ynlr_pull_up(&mut self) -> _YNLR_PULL_UPW {
        _YNLR_PULL_UPW { w: self }
    }
    #[doc = "Bit 11 - YNLR Wire 200K Pull Up Switch"]
    #[inline]
    pub fn ynlr_200k_pull_up(&mut self) -> _YNLR_200K_PULL_UPW {
        _YNLR_200K_PULL_UPW { w: self }
    }
    #[doc = "Bit 12 - Wiper Wire Pull Down Switch"]
    #[inline]
    pub fn wiper_pull_down(&mut self) -> _WIPER_PULL_DOWNW {
        _WIPER_PULL_DOWNW { w: self }
    }
    #[doc = "Bit 13 - Wiper Wire Pull Up Switch"]
    #[inline]
    pub fn wiper_pull_up(&mut self) -> _WIPER_PULL_UPW {
        _WIPER_PULL_UPW { w: self }
    }
    #[doc = "Bit 14 - Wiper Wire 200K Pull Up Switch"]
    #[inline]
    pub fn wiper_200k_pull_up(&mut self) -> _WIPER_200K_PULL_UPW {
        _WIPER_200K_PULL_UPW { w: self }
    }
    #[doc = "Bit 24 - Detect Enable Four Wire"]
    #[inline]
    pub fn detect_enable_four_wire(&mut self) -> _DETECT_ENABLE_FOUR_WIREW {
        _DETECT_ENABLE_FOUR_WIREW { w: self }
    }
    #[doc = "Bit 28 - Detect Enable Five Wire"]
    #[inline]
    pub fn detect_enable_five_wire(&mut self) -> _DETECT_ENABLE_FIVE_WIREW {
        _DETECT_ENABLE_FIVE_WIREW { w: self }
    }
}
