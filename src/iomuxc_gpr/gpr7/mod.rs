#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR7 {
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
#[doc = "Possible values of the field `LPI2C1_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_STOP_REQR {
    #[doc = "stop request off"]
    LPI2C1_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C1_STOP_REQ_1,
}
impl LPI2C1_STOP_REQR {
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
            LPI2C1_STOP_REQR::LPI2C1_STOP_REQ_0 => false,
            LPI2C1_STOP_REQR::LPI2C1_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C1_STOP_REQR {
        match value {
            false => LPI2C1_STOP_REQR::LPI2C1_STOP_REQ_0,
            true => LPI2C1_STOP_REQR::LPI2C1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpi2c1_stop_req_0(&self) -> bool {
        *self == LPI2C1_STOP_REQR::LPI2C1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpi2c1_stop_req_1(&self) -> bool {
        *self == LPI2C1_STOP_REQR::LPI2C1_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPI2C2_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_STOP_REQR {
    #[doc = "stop request off"]
    LPI2C2_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C2_STOP_REQ_1,
}
impl LPI2C2_STOP_REQR {
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
            LPI2C2_STOP_REQR::LPI2C2_STOP_REQ_0 => false,
            LPI2C2_STOP_REQR::LPI2C2_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C2_STOP_REQR {
        match value {
            false => LPI2C2_STOP_REQR::LPI2C2_STOP_REQ_0,
            true => LPI2C2_STOP_REQR::LPI2C2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpi2c2_stop_req_0(&self) -> bool {
        *self == LPI2C2_STOP_REQR::LPI2C2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpi2c2_stop_req_1(&self) -> bool {
        *self == LPI2C2_STOP_REQR::LPI2C2_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPI2C3_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_STOP_REQR {
    #[doc = "stop request off"]
    LPI2C3_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C3_STOP_REQ_1,
}
impl LPI2C3_STOP_REQR {
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
            LPI2C3_STOP_REQR::LPI2C3_STOP_REQ_0 => false,
            LPI2C3_STOP_REQR::LPI2C3_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C3_STOP_REQR {
        match value {
            false => LPI2C3_STOP_REQR::LPI2C3_STOP_REQ_0,
            true => LPI2C3_STOP_REQR::LPI2C3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpi2c3_stop_req_0(&self) -> bool {
        *self == LPI2C3_STOP_REQR::LPI2C3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpi2c3_stop_req_1(&self) -> bool {
        *self == LPI2C3_STOP_REQR::LPI2C3_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPI2C4_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_STOP_REQR {
    #[doc = "stop request off"]
    LPI2C4_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C4_STOP_REQ_1,
}
impl LPI2C4_STOP_REQR {
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
            LPI2C4_STOP_REQR::LPI2C4_STOP_REQ_0 => false,
            LPI2C4_STOP_REQR::LPI2C4_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C4_STOP_REQR {
        match value {
            false => LPI2C4_STOP_REQR::LPI2C4_STOP_REQ_0,
            true => LPI2C4_STOP_REQR::LPI2C4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpi2c4_stop_req_0(&self) -> bool {
        *self == LPI2C4_STOP_REQR::LPI2C4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpi2c4_stop_req_1(&self) -> bool {
        *self == LPI2C4_STOP_REQR::LPI2C4_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPSPI1_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_STOP_REQR {
    #[doc = "stop request off"]
    LPSPI1_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI1_STOP_REQ_1,
}
impl LPSPI1_STOP_REQR {
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
            LPSPI1_STOP_REQR::LPSPI1_STOP_REQ_0 => false,
            LPSPI1_STOP_REQR::LPSPI1_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI1_STOP_REQR {
        match value {
            false => LPSPI1_STOP_REQR::LPSPI1_STOP_REQ_0,
            true => LPSPI1_STOP_REQR::LPSPI1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpspi1_stop_req_0(&self) -> bool {
        *self == LPSPI1_STOP_REQR::LPSPI1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpspi1_stop_req_1(&self) -> bool {
        *self == LPSPI1_STOP_REQR::LPSPI1_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPSPI2_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_STOP_REQR {
    #[doc = "stop request off"]
    LPSPI2_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI2_STOP_REQ_1,
}
impl LPSPI2_STOP_REQR {
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
            LPSPI2_STOP_REQR::LPSPI2_STOP_REQ_0 => false,
            LPSPI2_STOP_REQR::LPSPI2_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI2_STOP_REQR {
        match value {
            false => LPSPI2_STOP_REQR::LPSPI2_STOP_REQ_0,
            true => LPSPI2_STOP_REQR::LPSPI2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpspi2_stop_req_0(&self) -> bool {
        *self == LPSPI2_STOP_REQR::LPSPI2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpspi2_stop_req_1(&self) -> bool {
        *self == LPSPI2_STOP_REQR::LPSPI2_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPSPI3_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_STOP_REQR {
    #[doc = "stop request off"]
    LPSPI3_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI3_STOP_REQ_1,
}
impl LPSPI3_STOP_REQR {
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
            LPSPI3_STOP_REQR::LPSPI3_STOP_REQ_0 => false,
            LPSPI3_STOP_REQR::LPSPI3_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI3_STOP_REQR {
        match value {
            false => LPSPI3_STOP_REQR::LPSPI3_STOP_REQ_0,
            true => LPSPI3_STOP_REQR::LPSPI3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpspi3_stop_req_0(&self) -> bool {
        *self == LPSPI3_STOP_REQR::LPSPI3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpspi3_stop_req_1(&self) -> bool {
        *self == LPSPI3_STOP_REQR::LPSPI3_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPSPI4_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_STOP_REQR {
    #[doc = "stop request off"]
    LPSPI4_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI4_STOP_REQ_1,
}
impl LPSPI4_STOP_REQR {
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
            LPSPI4_STOP_REQR::LPSPI4_STOP_REQ_0 => false,
            LPSPI4_STOP_REQR::LPSPI4_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI4_STOP_REQR {
        match value {
            false => LPSPI4_STOP_REQR::LPSPI4_STOP_REQ_0,
            true => LPSPI4_STOP_REQR::LPSPI4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpspi4_stop_req_0(&self) -> bool {
        *self == LPSPI4_STOP_REQR::LPSPI4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpspi4_stop_req_1(&self) -> bool {
        *self == LPSPI4_STOP_REQR::LPSPI4_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART1_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_STOP_REQR {
    #[doc = "stop request off"]
    LPUART1_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART1_STOP_REQ_1,
}
impl LPUART1_STOP_REQR {
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
            LPUART1_STOP_REQR::LPUART1_STOP_REQ_0 => false,
            LPUART1_STOP_REQR::LPUART1_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART1_STOP_REQR {
        match value {
            false => LPUART1_STOP_REQR::LPUART1_STOP_REQ_0,
            true => LPUART1_STOP_REQR::LPUART1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart1_stop_req_0(&self) -> bool {
        *self == LPUART1_STOP_REQR::LPUART1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart1_stop_req_1(&self) -> bool {
        *self == LPUART1_STOP_REQR::LPUART1_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART2_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_STOP_REQR {
    #[doc = "stop request off"]
    LPUART2_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART2_STOP_REQ_1,
}
impl LPUART2_STOP_REQR {
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
            LPUART2_STOP_REQR::LPUART2_STOP_REQ_0 => false,
            LPUART2_STOP_REQR::LPUART2_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART2_STOP_REQR {
        match value {
            false => LPUART2_STOP_REQR::LPUART2_STOP_REQ_0,
            true => LPUART2_STOP_REQR::LPUART2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart2_stop_req_0(&self) -> bool {
        *self == LPUART2_STOP_REQR::LPUART2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart2_stop_req_1(&self) -> bool {
        *self == LPUART2_STOP_REQR::LPUART2_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART3_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_STOP_REQR {
    #[doc = "stop request off"]
    LPUART3_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART3_STOP_REQ_1,
}
impl LPUART3_STOP_REQR {
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
            LPUART3_STOP_REQR::LPUART3_STOP_REQ_0 => false,
            LPUART3_STOP_REQR::LPUART3_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART3_STOP_REQR {
        match value {
            false => LPUART3_STOP_REQR::LPUART3_STOP_REQ_0,
            true => LPUART3_STOP_REQR::LPUART3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart3_stop_req_0(&self) -> bool {
        *self == LPUART3_STOP_REQR::LPUART3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart3_stop_req_1(&self) -> bool {
        *self == LPUART3_STOP_REQR::LPUART3_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART4_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_STOP_REQR {
    #[doc = "stop request off"]
    LPUART4_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART4_STOP_REQ_1,
}
impl LPUART4_STOP_REQR {
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
            LPUART4_STOP_REQR::LPUART4_STOP_REQ_0 => false,
            LPUART4_STOP_REQR::LPUART4_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART4_STOP_REQR {
        match value {
            false => LPUART4_STOP_REQR::LPUART4_STOP_REQ_0,
            true => LPUART4_STOP_REQR::LPUART4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart4_stop_req_0(&self) -> bool {
        *self == LPUART4_STOP_REQR::LPUART4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart4_stop_req_1(&self) -> bool {
        *self == LPUART4_STOP_REQR::LPUART4_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART5_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_STOP_REQR {
    #[doc = "stop request off"]
    LPUART5_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART5_STOP_REQ_1,
}
impl LPUART5_STOP_REQR {
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
            LPUART5_STOP_REQR::LPUART5_STOP_REQ_0 => false,
            LPUART5_STOP_REQR::LPUART5_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART5_STOP_REQR {
        match value {
            false => LPUART5_STOP_REQR::LPUART5_STOP_REQ_0,
            true => LPUART5_STOP_REQR::LPUART5_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart5_stop_req_0(&self) -> bool {
        *self == LPUART5_STOP_REQR::LPUART5_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart5_stop_req_1(&self) -> bool {
        *self == LPUART5_STOP_REQR::LPUART5_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART6_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_STOP_REQR {
    #[doc = "stop request off"]
    LPUART6_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART6_STOP_REQ_1,
}
impl LPUART6_STOP_REQR {
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
            LPUART6_STOP_REQR::LPUART6_STOP_REQ_0 => false,
            LPUART6_STOP_REQR::LPUART6_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART6_STOP_REQR {
        match value {
            false => LPUART6_STOP_REQR::LPUART6_STOP_REQ_0,
            true => LPUART6_STOP_REQR::LPUART6_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart6_stop_req_0(&self) -> bool {
        *self == LPUART6_STOP_REQR::LPUART6_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart6_stop_req_1(&self) -> bool {
        *self == LPUART6_STOP_REQR::LPUART6_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART7_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_STOP_REQR {
    #[doc = "stop request off"]
    LPUART7_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART7_STOP_REQ_1,
}
impl LPUART7_STOP_REQR {
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
            LPUART7_STOP_REQR::LPUART7_STOP_REQ_0 => false,
            LPUART7_STOP_REQR::LPUART7_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART7_STOP_REQR {
        match value {
            false => LPUART7_STOP_REQR::LPUART7_STOP_REQ_0,
            true => LPUART7_STOP_REQR::LPUART7_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart7_stop_req_0(&self) -> bool {
        *self == LPUART7_STOP_REQR::LPUART7_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart7_stop_req_1(&self) -> bool {
        *self == LPUART7_STOP_REQR::LPUART7_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPUART8_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_STOP_REQR {
    #[doc = "stop request off"]
    LPUART8_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART8_STOP_REQ_1,
}
impl LPUART8_STOP_REQR {
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
            LPUART8_STOP_REQR::LPUART8_STOP_REQ_0 => false,
            LPUART8_STOP_REQR::LPUART8_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART8_STOP_REQR {
        match value {
            false => LPUART8_STOP_REQR::LPUART8_STOP_REQ_0,
            true => LPUART8_STOP_REQR::LPUART8_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_REQ_0`"]
    #[inline]
    pub fn is_lpuart8_stop_req_0(&self) -> bool {
        *self == LPUART8_STOP_REQR::LPUART8_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_REQ_1`"]
    #[inline]
    pub fn is_lpuart8_stop_req_1(&self) -> bool {
        *self == LPUART8_STOP_REQR::LPUART8_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `LPI2C1_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPI2C1_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted (the module is in Stop mode)"]
    LPI2C1_STOP_ACK_1,
}
impl LPI2C1_STOP_ACKR {
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
            LPI2C1_STOP_ACKR::LPI2C1_STOP_ACK_0 => false,
            LPI2C1_STOP_ACKR::LPI2C1_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C1_STOP_ACKR {
        match value {
            false => LPI2C1_STOP_ACKR::LPI2C1_STOP_ACK_0,
            true => LPI2C1_STOP_ACKR::LPI2C1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpi2c1_stop_ack_0(&self) -> bool {
        *self == LPI2C1_STOP_ACKR::LPI2C1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpi2c1_stop_ack_1(&self) -> bool {
        *self == LPI2C1_STOP_ACKR::LPI2C1_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPI2C2_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPI2C2_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPI2C2_STOP_ACK_1,
}
impl LPI2C2_STOP_ACKR {
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
            LPI2C2_STOP_ACKR::LPI2C2_STOP_ACK_0 => false,
            LPI2C2_STOP_ACKR::LPI2C2_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C2_STOP_ACKR {
        match value {
            false => LPI2C2_STOP_ACKR::LPI2C2_STOP_ACK_0,
            true => LPI2C2_STOP_ACKR::LPI2C2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpi2c2_stop_ack_0(&self) -> bool {
        *self == LPI2C2_STOP_ACKR::LPI2C2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpi2c2_stop_ack_1(&self) -> bool {
        *self == LPI2C2_STOP_ACKR::LPI2C2_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPI2C3_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPI2C3_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPI2C3_STOP_ACK_1,
}
impl LPI2C3_STOP_ACKR {
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
            LPI2C3_STOP_ACKR::LPI2C3_STOP_ACK_0 => false,
            LPI2C3_STOP_ACKR::LPI2C3_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C3_STOP_ACKR {
        match value {
            false => LPI2C3_STOP_ACKR::LPI2C3_STOP_ACK_0,
            true => LPI2C3_STOP_ACKR::LPI2C3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpi2c3_stop_ack_0(&self) -> bool {
        *self == LPI2C3_STOP_ACKR::LPI2C3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpi2c3_stop_ack_1(&self) -> bool {
        *self == LPI2C3_STOP_ACKR::LPI2C3_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPI2C4_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPI2C4_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPI2C4_STOP_ACK_1,
}
impl LPI2C4_STOP_ACKR {
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
            LPI2C4_STOP_ACKR::LPI2C4_STOP_ACK_0 => false,
            LPI2C4_STOP_ACKR::LPI2C4_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C4_STOP_ACKR {
        match value {
            false => LPI2C4_STOP_ACKR::LPI2C4_STOP_ACK_0,
            true => LPI2C4_STOP_ACKR::LPI2C4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpi2c4_stop_ack_0(&self) -> bool {
        *self == LPI2C4_STOP_ACKR::LPI2C4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpi2c4_stop_ack_1(&self) -> bool {
        *self == LPI2C4_STOP_ACKR::LPI2C4_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPSPI1_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPSPI1_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPSPI1_STOP_ACK_1,
}
impl LPSPI1_STOP_ACKR {
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
            LPSPI1_STOP_ACKR::LPSPI1_STOP_ACK_0 => false,
            LPSPI1_STOP_ACKR::LPSPI1_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI1_STOP_ACKR {
        match value {
            false => LPSPI1_STOP_ACKR::LPSPI1_STOP_ACK_0,
            true => LPSPI1_STOP_ACKR::LPSPI1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpspi1_stop_ack_0(&self) -> bool {
        *self == LPSPI1_STOP_ACKR::LPSPI1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpspi1_stop_ack_1(&self) -> bool {
        *self == LPSPI1_STOP_ACKR::LPSPI1_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPSPI2_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPSPI2_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPSPI2_STOP_ACK_1,
}
impl LPSPI2_STOP_ACKR {
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
            LPSPI2_STOP_ACKR::LPSPI2_STOP_ACK_0 => false,
            LPSPI2_STOP_ACKR::LPSPI2_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI2_STOP_ACKR {
        match value {
            false => LPSPI2_STOP_ACKR::LPSPI2_STOP_ACK_0,
            true => LPSPI2_STOP_ACKR::LPSPI2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpspi2_stop_ack_0(&self) -> bool {
        *self == LPSPI2_STOP_ACKR::LPSPI2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpspi2_stop_ack_1(&self) -> bool {
        *self == LPSPI2_STOP_ACKR::LPSPI2_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPSPI3_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPSPI3_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPSPI3_STOP_ACK_1,
}
impl LPSPI3_STOP_ACKR {
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
            LPSPI3_STOP_ACKR::LPSPI3_STOP_ACK_0 => false,
            LPSPI3_STOP_ACKR::LPSPI3_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI3_STOP_ACKR {
        match value {
            false => LPSPI3_STOP_ACKR::LPSPI3_STOP_ACK_0,
            true => LPSPI3_STOP_ACKR::LPSPI3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpspi3_stop_ack_0(&self) -> bool {
        *self == LPSPI3_STOP_ACKR::LPSPI3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpspi3_stop_ack_1(&self) -> bool {
        *self == LPSPI3_STOP_ACKR::LPSPI3_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPSPI4_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPSPI4_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPSPI4_STOP_ACK_1,
}
impl LPSPI4_STOP_ACKR {
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
            LPSPI4_STOP_ACKR::LPSPI4_STOP_ACK_0 => false,
            LPSPI4_STOP_ACKR::LPSPI4_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSPI4_STOP_ACKR {
        match value {
            false => LPSPI4_STOP_ACKR::LPSPI4_STOP_ACK_0,
            true => LPSPI4_STOP_ACKR::LPSPI4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpspi4_stop_ack_0(&self) -> bool {
        *self == LPSPI4_STOP_ACKR::LPSPI4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpspi4_stop_ack_1(&self) -> bool {
        *self == LPSPI4_STOP_ACKR::LPSPI4_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART1_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART1_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPUART1_STOP_ACK_1,
}
impl LPUART1_STOP_ACKR {
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
            LPUART1_STOP_ACKR::LPUART1_STOP_ACK_0 => false,
            LPUART1_STOP_ACKR::LPUART1_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART1_STOP_ACKR {
        match value {
            false => LPUART1_STOP_ACKR::LPUART1_STOP_ACK_0,
            true => LPUART1_STOP_ACKR::LPUART1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart1_stop_ack_0(&self) -> bool {
        *self == LPUART1_STOP_ACKR::LPUART1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart1_stop_ack_1(&self) -> bool {
        *self == LPUART1_STOP_ACKR::LPUART1_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART2_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART2_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPUART2_STOP_ACK_1,
}
impl LPUART2_STOP_ACKR {
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
            LPUART2_STOP_ACKR::LPUART2_STOP_ACK_0 => false,
            LPUART2_STOP_ACKR::LPUART2_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART2_STOP_ACKR {
        match value {
            false => LPUART2_STOP_ACKR::LPUART2_STOP_ACK_0,
            true => LPUART2_STOP_ACKR::LPUART2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart2_stop_ack_0(&self) -> bool {
        *self == LPUART2_STOP_ACKR::LPUART2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart2_stop_ack_1(&self) -> bool {
        *self == LPUART2_STOP_ACKR::LPUART2_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART3_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART3_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPUART3_STOP_ACK_1,
}
impl LPUART3_STOP_ACKR {
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
            LPUART3_STOP_ACKR::LPUART3_STOP_ACK_0 => false,
            LPUART3_STOP_ACKR::LPUART3_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART3_STOP_ACKR {
        match value {
            false => LPUART3_STOP_ACKR::LPUART3_STOP_ACK_0,
            true => LPUART3_STOP_ACKR::LPUART3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart3_stop_ack_0(&self) -> bool {
        *self == LPUART3_STOP_ACKR::LPUART3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart3_stop_ack_1(&self) -> bool {
        *self == LPUART3_STOP_ACKR::LPUART3_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART4_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART4_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPUART4_STOP_ACK_1,
}
impl LPUART4_STOP_ACKR {
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
            LPUART4_STOP_ACKR::LPUART4_STOP_ACK_0 => false,
            LPUART4_STOP_ACKR::LPUART4_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART4_STOP_ACKR {
        match value {
            false => LPUART4_STOP_ACKR::LPUART4_STOP_ACK_0,
            true => LPUART4_STOP_ACKR::LPUART4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart4_stop_ack_0(&self) -> bool {
        *self == LPUART4_STOP_ACKR::LPUART4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart4_stop_ack_1(&self) -> bool {
        *self == LPUART4_STOP_ACKR::LPUART4_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART5_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART5_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPUART5_STOP_ACK_1,
}
impl LPUART5_STOP_ACKR {
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
            LPUART5_STOP_ACKR::LPUART5_STOP_ACK_0 => false,
            LPUART5_STOP_ACKR::LPUART5_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART5_STOP_ACKR {
        match value {
            false => LPUART5_STOP_ACKR::LPUART5_STOP_ACK_0,
            true => LPUART5_STOP_ACKR::LPUART5_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart5_stop_ack_0(&self) -> bool {
        *self == LPUART5_STOP_ACKR::LPUART5_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart5_stop_ack_1(&self) -> bool {
        *self == LPUART5_STOP_ACKR::LPUART5_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART6_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART6_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPUART6_STOP_ACK_1,
}
impl LPUART6_STOP_ACKR {
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
            LPUART6_STOP_ACKR::LPUART6_STOP_ACK_0 => false,
            LPUART6_STOP_ACKR::LPUART6_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART6_STOP_ACKR {
        match value {
            false => LPUART6_STOP_ACKR::LPUART6_STOP_ACK_0,
            true => LPUART6_STOP_ACKR::LPUART6_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart6_stop_ack_0(&self) -> bool {
        *self == LPUART6_STOP_ACKR::LPUART6_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart6_stop_ack_1(&self) -> bool {
        *self == LPUART6_STOP_ACKR::LPUART6_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART7_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART7_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted"]
    LPUART7_STOP_ACK_1,
}
impl LPUART7_STOP_ACKR {
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
            LPUART7_STOP_ACKR::LPUART7_STOP_ACK_0 => false,
            LPUART7_STOP_ACKR::LPUART7_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART7_STOP_ACKR {
        match value {
            false => LPUART7_STOP_ACKR::LPUART7_STOP_ACK_0,
            true => LPUART7_STOP_ACKR::LPUART7_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart7_stop_ack_0(&self) -> bool {
        *self == LPUART7_STOP_ACKR::LPUART7_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart7_stop_ack_1(&self) -> bool {
        *self == LPUART7_STOP_ACKR::LPUART7_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `LPUART8_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_STOP_ACKR {
    #[doc = "stop acknowledge is not asserted"]
    LPUART8_STOP_ACK_0,
    #[doc = "stop acknowledge is asserted (the module is in Stop mode)"]
    LPUART8_STOP_ACK_1,
}
impl LPUART8_STOP_ACKR {
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
            LPUART8_STOP_ACKR::LPUART8_STOP_ACK_0 => false,
            LPUART8_STOP_ACKR::LPUART8_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART8_STOP_ACKR {
        match value {
            false => LPUART8_STOP_ACKR::LPUART8_STOP_ACK_0,
            true => LPUART8_STOP_ACKR::LPUART8_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_ACK_0`"]
    #[inline]
    pub fn is_lpuart8_stop_ack_0(&self) -> bool {
        *self == LPUART8_STOP_ACKR::LPUART8_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_ACK_1`"]
    #[inline]
    pub fn is_lpuart8_stop_ack_1(&self) -> bool {
        *self == LPUART8_STOP_ACKR::LPUART8_STOP_ACK_1
    }
}
#[doc = "Values that can be written to the field `LPI2C1_STOP_REQ`"]
pub enum LPI2C1_STOP_REQW {
    #[doc = "stop request off"]
    LPI2C1_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C1_STOP_REQ_1,
}
impl LPI2C1_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C1_STOP_REQW::LPI2C1_STOP_REQ_0 => false,
            LPI2C1_STOP_REQW::LPI2C1_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C1_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C1_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C1_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpi2c1_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C1_STOP_REQW::LPI2C1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpi2c1_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C1_STOP_REQW::LPI2C1_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPI2C2_STOP_REQ`"]
pub enum LPI2C2_STOP_REQW {
    #[doc = "stop request off"]
    LPI2C2_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C2_STOP_REQ_1,
}
impl LPI2C2_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C2_STOP_REQW::LPI2C2_STOP_REQ_0 => false,
            LPI2C2_STOP_REQW::LPI2C2_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C2_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C2_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C2_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpi2c2_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C2_STOP_REQW::LPI2C2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpi2c2_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C2_STOP_REQW::LPI2C2_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPI2C3_STOP_REQ`"]
pub enum LPI2C3_STOP_REQW {
    #[doc = "stop request off"]
    LPI2C3_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C3_STOP_REQ_1,
}
impl LPI2C3_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C3_STOP_REQW::LPI2C3_STOP_REQ_0 => false,
            LPI2C3_STOP_REQW::LPI2C3_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C3_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C3_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C3_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpi2c3_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C3_STOP_REQW::LPI2C3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpi2c3_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C3_STOP_REQW::LPI2C3_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPI2C4_STOP_REQ`"]
pub enum LPI2C4_STOP_REQW {
    #[doc = "stop request off"]
    LPI2C4_STOP_REQ_0,
    #[doc = "stop request on"]
    LPI2C4_STOP_REQ_1,
}
impl LPI2C4_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C4_STOP_REQW::LPI2C4_STOP_REQ_0 => false,
            LPI2C4_STOP_REQW::LPI2C4_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C4_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C4_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C4_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpi2c4_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C4_STOP_REQW::LPI2C4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpi2c4_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C4_STOP_REQW::LPI2C4_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPSPI1_STOP_REQ`"]
pub enum LPSPI1_STOP_REQW {
    #[doc = "stop request off"]
    LPSPI1_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI1_STOP_REQ_1,
}
impl LPSPI1_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI1_STOP_REQW::LPSPI1_STOP_REQ_0 => false,
            LPSPI1_STOP_REQW::LPSPI1_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI1_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI1_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI1_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpspi1_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI1_STOP_REQW::LPSPI1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpspi1_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI1_STOP_REQW::LPSPI1_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPSPI2_STOP_REQ`"]
pub enum LPSPI2_STOP_REQW {
    #[doc = "stop request off"]
    LPSPI2_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI2_STOP_REQ_1,
}
impl LPSPI2_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI2_STOP_REQW::LPSPI2_STOP_REQ_0 => false,
            LPSPI2_STOP_REQW::LPSPI2_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI2_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI2_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI2_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpspi2_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI2_STOP_REQW::LPSPI2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpspi2_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI2_STOP_REQW::LPSPI2_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPSPI3_STOP_REQ`"]
pub enum LPSPI3_STOP_REQW {
    #[doc = "stop request off"]
    LPSPI3_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI3_STOP_REQ_1,
}
impl LPSPI3_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI3_STOP_REQW::LPSPI3_STOP_REQ_0 => false,
            LPSPI3_STOP_REQW::LPSPI3_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI3_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI3_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI3_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpspi3_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI3_STOP_REQW::LPSPI3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpspi3_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI3_STOP_REQW::LPSPI3_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPSPI4_STOP_REQ`"]
pub enum LPSPI4_STOP_REQW {
    #[doc = "stop request off"]
    LPSPI4_STOP_REQ_0,
    #[doc = "stop request on"]
    LPSPI4_STOP_REQ_1,
}
impl LPSPI4_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSPI4_STOP_REQW::LPSPI4_STOP_REQ_0 => false,
            LPSPI4_STOP_REQW::LPSPI4_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI4_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI4_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI4_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpspi4_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI4_STOP_REQW::LPSPI4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpspi4_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI4_STOP_REQW::LPSPI4_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART1_STOP_REQ`"]
pub enum LPUART1_STOP_REQW {
    #[doc = "stop request off"]
    LPUART1_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART1_STOP_REQ_1,
}
impl LPUART1_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART1_STOP_REQW::LPUART1_STOP_REQ_0 => false,
            LPUART1_STOP_REQW::LPUART1_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART1_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART1_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART1_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart1_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART1_STOP_REQW::LPUART1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart1_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART1_STOP_REQW::LPUART1_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART2_STOP_REQ`"]
pub enum LPUART2_STOP_REQW {
    #[doc = "stop request off"]
    LPUART2_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART2_STOP_REQ_1,
}
impl LPUART2_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART2_STOP_REQW::LPUART2_STOP_REQ_0 => false,
            LPUART2_STOP_REQW::LPUART2_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART2_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART2_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART2_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart2_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART2_STOP_REQW::LPUART2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart2_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART2_STOP_REQW::LPUART2_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART3_STOP_REQ`"]
pub enum LPUART3_STOP_REQW {
    #[doc = "stop request off"]
    LPUART3_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART3_STOP_REQ_1,
}
impl LPUART3_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART3_STOP_REQW::LPUART3_STOP_REQ_0 => false,
            LPUART3_STOP_REQW::LPUART3_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART3_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART3_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART3_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart3_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART3_STOP_REQW::LPUART3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart3_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART3_STOP_REQW::LPUART3_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART4_STOP_REQ`"]
pub enum LPUART4_STOP_REQW {
    #[doc = "stop request off"]
    LPUART4_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART4_STOP_REQ_1,
}
impl LPUART4_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART4_STOP_REQW::LPUART4_STOP_REQ_0 => false,
            LPUART4_STOP_REQW::LPUART4_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART4_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART4_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART4_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart4_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART4_STOP_REQW::LPUART4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart4_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART4_STOP_REQW::LPUART4_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART5_STOP_REQ`"]
pub enum LPUART5_STOP_REQW {
    #[doc = "stop request off"]
    LPUART5_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART5_STOP_REQ_1,
}
impl LPUART5_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART5_STOP_REQW::LPUART5_STOP_REQ_0 => false,
            LPUART5_STOP_REQW::LPUART5_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART5_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART5_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART5_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart5_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART5_STOP_REQW::LPUART5_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart5_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART5_STOP_REQW::LPUART5_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART6_STOP_REQ`"]
pub enum LPUART6_STOP_REQW {
    #[doc = "stop request off"]
    LPUART6_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART6_STOP_REQ_1,
}
impl LPUART6_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART6_STOP_REQW::LPUART6_STOP_REQ_0 => false,
            LPUART6_STOP_REQW::LPUART6_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART6_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART6_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART6_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart6_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART6_STOP_REQW::LPUART6_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart6_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART6_STOP_REQW::LPUART6_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART7_STOP_REQ`"]
pub enum LPUART7_STOP_REQW {
    #[doc = "stop request off"]
    LPUART7_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART7_STOP_REQ_1,
}
impl LPUART7_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART7_STOP_REQW::LPUART7_STOP_REQ_0 => false,
            LPUART7_STOP_REQW::LPUART7_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART7_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART7_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART7_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart7_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART7_STOP_REQW::LPUART7_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart7_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART7_STOP_REQW::LPUART7_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `LPUART8_STOP_REQ`"]
pub enum LPUART8_STOP_REQW {
    #[doc = "stop request off"]
    LPUART8_STOP_REQ_0,
    #[doc = "stop request on"]
    LPUART8_STOP_REQ_1,
}
impl LPUART8_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART8_STOP_REQW::LPUART8_STOP_REQ_0 => false,
            LPUART8_STOP_REQW::LPUART8_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART8_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART8_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART8_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn lpuart8_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART8_STOP_REQW::LPUART8_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn lpuart8_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART8_STOP_REQW::LPUART8_STOP_REQ_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - LPI2C1 stop request"]
    #[inline]
    pub fn lpi2c1_stop_req(&self) -> LPI2C1_STOP_REQR {
        LPI2C1_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - LPI2C2 stop request"]
    #[inline]
    pub fn lpi2c2_stop_req(&self) -> LPI2C2_STOP_REQR {
        LPI2C2_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - LPI2C3 stop request"]
    #[inline]
    pub fn lpi2c3_stop_req(&self) -> LPI2C3_STOP_REQR {
        LPI2C3_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - LPI2C4 stop request"]
    #[inline]
    pub fn lpi2c4_stop_req(&self) -> LPI2C4_STOP_REQR {
        LPI2C4_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - LPSPI1 stop request"]
    #[inline]
    pub fn lpspi1_stop_req(&self) -> LPSPI1_STOP_REQR {
        LPSPI1_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - LPSPI2 stop request"]
    #[inline]
    pub fn lpspi2_stop_req(&self) -> LPSPI2_STOP_REQR {
        LPSPI2_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LPSPI3 stop request"]
    #[inline]
    pub fn lpspi3_stop_req(&self) -> LPSPI3_STOP_REQR {
        LPSPI3_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - LPSPI4 stop request"]
    #[inline]
    pub fn lpspi4_stop_req(&self) -> LPSPI4_STOP_REQR {
        LPSPI4_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - LPUART1 stop request"]
    #[inline]
    pub fn lpuart1_stop_req(&self) -> LPUART1_STOP_REQR {
        LPUART1_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - LPUART1 stop request"]
    #[inline]
    pub fn lpuart2_stop_req(&self) -> LPUART2_STOP_REQR {
        LPUART2_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - LPUART3 stop request"]
    #[inline]
    pub fn lpuart3_stop_req(&self) -> LPUART3_STOP_REQR {
        LPUART3_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - LPUART4 stop request"]
    #[inline]
    pub fn lpuart4_stop_req(&self) -> LPUART4_STOP_REQR {
        LPUART4_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - LPUART5 stop request"]
    #[inline]
    pub fn lpuart5_stop_req(&self) -> LPUART5_STOP_REQR {
        LPUART5_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - LPUART6 stop request"]
    #[inline]
    pub fn lpuart6_stop_req(&self) -> LPUART6_STOP_REQR {
        LPUART6_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - LPUART7 stop request"]
    #[inline]
    pub fn lpuart7_stop_req(&self) -> LPUART7_STOP_REQR {
        LPUART7_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - LPUART8 stop request"]
    #[inline]
    pub fn lpuart8_stop_req(&self) -> LPUART8_STOP_REQR {
        LPUART8_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - LPI2C1 stop acknowledge"]
    #[inline]
    pub fn lpi2c1_stop_ack(&self) -> LPI2C1_STOP_ACKR {
        LPI2C1_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - LPI2C2 stop acknowledge"]
    #[inline]
    pub fn lpi2c2_stop_ack(&self) -> LPI2C2_STOP_ACKR {
        LPI2C2_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - LPI2C3 stop acknowledge"]
    #[inline]
    pub fn lpi2c3_stop_ack(&self) -> LPI2C3_STOP_ACKR {
        LPI2C3_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - LPI2C4 stop acknowledge"]
    #[inline]
    pub fn lpi2c4_stop_ack(&self) -> LPI2C4_STOP_ACKR {
        LPI2C4_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - LPSPI1 stop acknowledge"]
    #[inline]
    pub fn lpspi1_stop_ack(&self) -> LPSPI1_STOP_ACKR {
        LPSPI1_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - LPSPI2 stop acknowledge"]
    #[inline]
    pub fn lpspi2_stop_ack(&self) -> LPSPI2_STOP_ACKR {
        LPSPI2_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - LPSPI3 stop acknowledge"]
    #[inline]
    pub fn lpspi3_stop_ack(&self) -> LPSPI3_STOP_ACKR {
        LPSPI3_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - LPSPI4 stop acknowledge"]
    #[inline]
    pub fn lpspi4_stop_ack(&self) -> LPSPI4_STOP_ACKR {
        LPSPI4_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - LPUART1 stop acknowledge"]
    #[inline]
    pub fn lpuart1_stop_ack(&self) -> LPUART1_STOP_ACKR {
        LPUART1_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - LPUART1 stop acknowledge"]
    #[inline]
    pub fn lpuart2_stop_ack(&self) -> LPUART2_STOP_ACKR {
        LPUART2_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - LPUART3 stop acknowledge"]
    #[inline]
    pub fn lpuart3_stop_ack(&self) -> LPUART3_STOP_ACKR {
        LPUART3_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - LPUART4 stop acknowledge"]
    #[inline]
    pub fn lpuart4_stop_ack(&self) -> LPUART4_STOP_ACKR {
        LPUART4_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - LPUART5 stop acknowledge"]
    #[inline]
    pub fn lpuart5_stop_ack(&self) -> LPUART5_STOP_ACKR {
        LPUART5_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - LPUART6 stop acknowledge"]
    #[inline]
    pub fn lpuart6_stop_ack(&self) -> LPUART6_STOP_ACKR {
        LPUART6_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - LPUART7 stop acknowledge"]
    #[inline]
    pub fn lpuart7_stop_ack(&self) -> LPUART7_STOP_ACKR {
        LPUART7_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - LPUART8 stop acknowledge"]
    #[inline]
    pub fn lpuart8_stop_ack(&self) -> LPUART8_STOP_ACKR {
        LPUART8_STOP_ACKR::_from({
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
    #[doc = "Bit 0 - LPI2C1 stop request"]
    #[inline]
    pub fn lpi2c1_stop_req(&mut self) -> _LPI2C1_STOP_REQW {
        _LPI2C1_STOP_REQW { w: self }
    }
    #[doc = "Bit 1 - LPI2C2 stop request"]
    #[inline]
    pub fn lpi2c2_stop_req(&mut self) -> _LPI2C2_STOP_REQW {
        _LPI2C2_STOP_REQW { w: self }
    }
    #[doc = "Bit 2 - LPI2C3 stop request"]
    #[inline]
    pub fn lpi2c3_stop_req(&mut self) -> _LPI2C3_STOP_REQW {
        _LPI2C3_STOP_REQW { w: self }
    }
    #[doc = "Bit 3 - LPI2C4 stop request"]
    #[inline]
    pub fn lpi2c4_stop_req(&mut self) -> _LPI2C4_STOP_REQW {
        _LPI2C4_STOP_REQW { w: self }
    }
    #[doc = "Bit 4 - LPSPI1 stop request"]
    #[inline]
    pub fn lpspi1_stop_req(&mut self) -> _LPSPI1_STOP_REQW {
        _LPSPI1_STOP_REQW { w: self }
    }
    #[doc = "Bit 5 - LPSPI2 stop request"]
    #[inline]
    pub fn lpspi2_stop_req(&mut self) -> _LPSPI2_STOP_REQW {
        _LPSPI2_STOP_REQW { w: self }
    }
    #[doc = "Bit 6 - LPSPI3 stop request"]
    #[inline]
    pub fn lpspi3_stop_req(&mut self) -> _LPSPI3_STOP_REQW {
        _LPSPI3_STOP_REQW { w: self }
    }
    #[doc = "Bit 7 - LPSPI4 stop request"]
    #[inline]
    pub fn lpspi4_stop_req(&mut self) -> _LPSPI4_STOP_REQW {
        _LPSPI4_STOP_REQW { w: self }
    }
    #[doc = "Bit 8 - LPUART1 stop request"]
    #[inline]
    pub fn lpuart1_stop_req(&mut self) -> _LPUART1_STOP_REQW {
        _LPUART1_STOP_REQW { w: self }
    }
    #[doc = "Bit 9 - LPUART1 stop request"]
    #[inline]
    pub fn lpuart2_stop_req(&mut self) -> _LPUART2_STOP_REQW {
        _LPUART2_STOP_REQW { w: self }
    }
    #[doc = "Bit 10 - LPUART3 stop request"]
    #[inline]
    pub fn lpuart3_stop_req(&mut self) -> _LPUART3_STOP_REQW {
        _LPUART3_STOP_REQW { w: self }
    }
    #[doc = "Bit 11 - LPUART4 stop request"]
    #[inline]
    pub fn lpuart4_stop_req(&mut self) -> _LPUART4_STOP_REQW {
        _LPUART4_STOP_REQW { w: self }
    }
    #[doc = "Bit 12 - LPUART5 stop request"]
    #[inline]
    pub fn lpuart5_stop_req(&mut self) -> _LPUART5_STOP_REQW {
        _LPUART5_STOP_REQW { w: self }
    }
    #[doc = "Bit 13 - LPUART6 stop request"]
    #[inline]
    pub fn lpuart6_stop_req(&mut self) -> _LPUART6_STOP_REQW {
        _LPUART6_STOP_REQW { w: self }
    }
    #[doc = "Bit 14 - LPUART7 stop request"]
    #[inline]
    pub fn lpuart7_stop_req(&mut self) -> _LPUART7_STOP_REQW {
        _LPUART7_STOP_REQW { w: self }
    }
    #[doc = "Bit 15 - LPUART8 stop request"]
    #[inline]
    pub fn lpuart8_stop_req(&mut self) -> _LPUART8_STOP_REQW {
        _LPUART8_STOP_REQW { w: self }
    }
}
