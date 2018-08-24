#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_STATUS_EN {
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
#[doc = "Possible values of the field `CCSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCSENR {
    #[doc = "Masked"]
    CCSEN_0,
    #[doc = "Enabled"]
    CCSEN_1,
}
impl CCSENR {
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
            CCSENR::CCSEN_0 => false,
            CCSENR::CCSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCSENR {
        match value {
            false => CCSENR::CCSEN_0,
            true => CCSENR::CCSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCSEN_0`"]
    #[inline]
    pub fn is_ccsen_0(&self) -> bool {
        *self == CCSENR::CCSEN_0
    }
    #[doc = "Checks if the value of the field is `CCSEN_1`"]
    #[inline]
    pub fn is_ccsen_1(&self) -> bool {
        *self == CCSENR::CCSEN_1
    }
}
#[doc = "Possible values of the field `TCSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCSENR {
    #[doc = "Masked"]
    TCSEN_0,
    #[doc = "Enabled"]
    TCSEN_1,
}
impl TCSENR {
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
            TCSENR::TCSEN_0 => false,
            TCSENR::TCSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCSENR {
        match value {
            false => TCSENR::TCSEN_0,
            true => TCSENR::TCSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCSEN_0`"]
    #[inline]
    pub fn is_tcsen_0(&self) -> bool {
        *self == TCSENR::TCSEN_0
    }
    #[doc = "Checks if the value of the field is `TCSEN_1`"]
    #[inline]
    pub fn is_tcsen_1(&self) -> bool {
        *self == TCSENR::TCSEN_1
    }
}
#[doc = "Possible values of the field `BGESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGESENR {
    #[doc = "Masked"]
    BGESEN_0,
    #[doc = "Enabled"]
    BGESEN_1,
}
impl BGESENR {
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
            BGESENR::BGESEN_0 => false,
            BGESENR::BGESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGESENR {
        match value {
            false => BGESENR::BGESEN_0,
            true => BGESENR::BGESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGESEN_0`"]
    #[inline]
    pub fn is_bgesen_0(&self) -> bool {
        *self == BGESENR::BGESEN_0
    }
    #[doc = "Checks if the value of the field is `BGESEN_1`"]
    #[inline]
    pub fn is_bgesen_1(&self) -> bool {
        *self == BGESENR::BGESEN_1
    }
}
#[doc = "Possible values of the field `DINTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTSENR {
    #[doc = "Masked"]
    DINTSEN_0,
    #[doc = "Enabled"]
    DINTSEN_1,
}
impl DINTSENR {
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
            DINTSENR::DINTSEN_0 => false,
            DINTSENR::DINTSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DINTSENR {
        match value {
            false => DINTSENR::DINTSEN_0,
            true => DINTSENR::DINTSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DINTSEN_0`"]
    #[inline]
    pub fn is_dintsen_0(&self) -> bool {
        *self == DINTSENR::DINTSEN_0
    }
    #[doc = "Checks if the value of the field is `DINTSEN_1`"]
    #[inline]
    pub fn is_dintsen_1(&self) -> bool {
        *self == DINTSENR::DINTSEN_1
    }
}
#[doc = "Possible values of the field `BWRSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRSENR {
    #[doc = "Masked"]
    BWRSEN_0,
    #[doc = "Enabled"]
    BWRSEN_1,
}
impl BWRSENR {
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
            BWRSENR::BWRSEN_0 => false,
            BWRSENR::BWRSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRSENR {
        match value {
            false => BWRSENR::BWRSEN_0,
            true => BWRSENR::BWRSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWRSEN_0`"]
    #[inline]
    pub fn is_bwrsen_0(&self) -> bool {
        *self == BWRSENR::BWRSEN_0
    }
    #[doc = "Checks if the value of the field is `BWRSEN_1`"]
    #[inline]
    pub fn is_bwrsen_1(&self) -> bool {
        *self == BWRSENR::BWRSEN_1
    }
}
#[doc = "Possible values of the field `BRRSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRSENR {
    #[doc = "Masked"]
    BRRSEN_0,
    #[doc = "Enabled"]
    BRRSEN_1,
}
impl BRRSENR {
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
            BRRSENR::BRRSEN_0 => false,
            BRRSENR::BRRSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRRSENR {
        match value {
            false => BRRSENR::BRRSEN_0,
            true => BRRSENR::BRRSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRRSEN_0`"]
    #[inline]
    pub fn is_brrsen_0(&self) -> bool {
        *self == BRRSENR::BRRSEN_0
    }
    #[doc = "Checks if the value of the field is `BRRSEN_1`"]
    #[inline]
    pub fn is_brrsen_1(&self) -> bool {
        *self == BRRSENR::BRRSEN_1
    }
}
#[doc = "Possible values of the field `CINSSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSSENR {
    #[doc = "Masked"]
    CINSSEN_0,
    #[doc = "Enabled"]
    CINSSEN_1,
}
impl CINSSENR {
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
            CINSSENR::CINSSEN_0 => false,
            CINSSENR::CINSSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSSENR {
        match value {
            false => CINSSENR::CINSSEN_0,
            true => CINSSENR::CINSSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINSSEN_0`"]
    #[inline]
    pub fn is_cinssen_0(&self) -> bool {
        *self == CINSSENR::CINSSEN_0
    }
    #[doc = "Checks if the value of the field is `CINSSEN_1`"]
    #[inline]
    pub fn is_cinssen_1(&self) -> bool {
        *self == CINSSENR::CINSSEN_1
    }
}
#[doc = "Possible values of the field `CRMSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMSENR {
    #[doc = "Masked"]
    CRMSEN_0,
    #[doc = "Enabled"]
    CRMSEN_1,
}
impl CRMSENR {
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
            CRMSENR::CRMSEN_0 => false,
            CRMSENR::CRMSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRMSENR {
        match value {
            false => CRMSENR::CRMSEN_0,
            true => CRMSENR::CRMSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRMSEN_0`"]
    #[inline]
    pub fn is_crmsen_0(&self) -> bool {
        *self == CRMSENR::CRMSEN_0
    }
    #[doc = "Checks if the value of the field is `CRMSEN_1`"]
    #[inline]
    pub fn is_crmsen_1(&self) -> bool {
        *self == CRMSENR::CRMSEN_1
    }
}
#[doc = "Possible values of the field `CINTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTSENR {
    #[doc = "Masked"]
    CINTSEN_0,
    #[doc = "Enabled"]
    CINTSEN_1,
}
impl CINTSENR {
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
            CINTSENR::CINTSEN_0 => false,
            CINTSENR::CINTSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTSENR {
        match value {
            false => CINTSENR::CINTSEN_0,
            true => CINTSENR::CINTSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINTSEN_0`"]
    #[inline]
    pub fn is_cintsen_0(&self) -> bool {
        *self == CINTSENR::CINTSEN_0
    }
    #[doc = "Checks if the value of the field is `CINTSEN_1`"]
    #[inline]
    pub fn is_cintsen_1(&self) -> bool {
        *self == CINTSENR::CINTSEN_1
    }
}
#[doc = "Possible values of the field `RTESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTESENR {
    #[doc = "Masked"]
    RTESEN_0,
    #[doc = "Enabled"]
    RTESEN_1,
}
impl RTESENR {
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
            RTESENR::RTESEN_0 => false,
            RTESENR::RTESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTESENR {
        match value {
            false => RTESENR::RTESEN_0,
            true => RTESENR::RTESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTESEN_0`"]
    #[inline]
    pub fn is_rtesen_0(&self) -> bool {
        *self == RTESENR::RTESEN_0
    }
    #[doc = "Checks if the value of the field is `RTESEN_1`"]
    #[inline]
    pub fn is_rtesen_1(&self) -> bool {
        *self == RTESENR::RTESEN_1
    }
}
#[doc = "Possible values of the field `TPSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPSENR {
    #[doc = "Masked"]
    TPSEN_0,
    #[doc = "Enabled"]
    TPSEN_1,
}
impl TPSENR {
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
            TPSENR::TPSEN_0 => false,
            TPSENR::TPSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPSENR {
        match value {
            false => TPSENR::TPSEN_0,
            true => TPSENR::TPSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TPSEN_0`"]
    #[inline]
    pub fn is_tpsen_0(&self) -> bool {
        *self == TPSENR::TPSEN_0
    }
    #[doc = "Checks if the value of the field is `TPSEN_1`"]
    #[inline]
    pub fn is_tpsen_1(&self) -> bool {
        *self == TPSENR::TPSEN_1
    }
}
#[doc = "Possible values of the field `CTOESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOESENR {
    #[doc = "Masked"]
    CTOESEN_0,
    #[doc = "Enabled"]
    CTOESEN_1,
}
impl CTOESENR {
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
            CTOESENR::CTOESEN_0 => false,
            CTOESENR::CTOESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTOESENR {
        match value {
            false => CTOESENR::CTOESEN_0,
            true => CTOESENR::CTOESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTOESEN_0`"]
    #[inline]
    pub fn is_ctoesen_0(&self) -> bool {
        *self == CTOESENR::CTOESEN_0
    }
    #[doc = "Checks if the value of the field is `CTOESEN_1`"]
    #[inline]
    pub fn is_ctoesen_1(&self) -> bool {
        *self == CTOESENR::CTOESEN_1
    }
}
#[doc = "Possible values of the field `CCESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCESENR {
    #[doc = "Masked"]
    CCESEN_0,
    #[doc = "Enabled"]
    CCESEN_1,
}
impl CCESENR {
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
            CCESENR::CCESEN_0 => false,
            CCESENR::CCESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCESENR {
        match value {
            false => CCESENR::CCESEN_0,
            true => CCESENR::CCESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCESEN_0`"]
    #[inline]
    pub fn is_ccesen_0(&self) -> bool {
        *self == CCESENR::CCESEN_0
    }
    #[doc = "Checks if the value of the field is `CCESEN_1`"]
    #[inline]
    pub fn is_ccesen_1(&self) -> bool {
        *self == CCESENR::CCESEN_1
    }
}
#[doc = "Possible values of the field `CEBESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBESENR {
    #[doc = "Masked"]
    CEBESEN_0,
    #[doc = "Enabled"]
    CEBESEN_1,
}
impl CEBESENR {
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
            CEBESENR::CEBESEN_0 => false,
            CEBESENR::CEBESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEBESENR {
        match value {
            false => CEBESENR::CEBESEN_0,
            true => CEBESENR::CEBESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEBESEN_0`"]
    #[inline]
    pub fn is_cebesen_0(&self) -> bool {
        *self == CEBESENR::CEBESEN_0
    }
    #[doc = "Checks if the value of the field is `CEBESEN_1`"]
    #[inline]
    pub fn is_cebesen_1(&self) -> bool {
        *self == CEBESENR::CEBESEN_1
    }
}
#[doc = "Possible values of the field `CIESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIESENR {
    #[doc = "Masked"]
    CIESEN_0,
    #[doc = "Enabled"]
    CIESEN_1,
}
impl CIESENR {
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
            CIESENR::CIESEN_0 => false,
            CIESENR::CIESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIESENR {
        match value {
            false => CIESENR::CIESEN_0,
            true => CIESENR::CIESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIESEN_0`"]
    #[inline]
    pub fn is_ciesen_0(&self) -> bool {
        *self == CIESENR::CIESEN_0
    }
    #[doc = "Checks if the value of the field is `CIESEN_1`"]
    #[inline]
    pub fn is_ciesen_1(&self) -> bool {
        *self == CIESENR::CIESEN_1
    }
}
#[doc = "Possible values of the field `DTOESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOESENR {
    #[doc = "Masked"]
    DTOESEN_0,
    #[doc = "Enabled"]
    DTOESEN_1,
}
impl DTOESENR {
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
            DTOESENR::DTOESEN_0 => false,
            DTOESENR::DTOESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTOESENR {
        match value {
            false => DTOESENR::DTOESEN_0,
            true => DTOESENR::DTOESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTOESEN_0`"]
    #[inline]
    pub fn is_dtoesen_0(&self) -> bool {
        *self == DTOESENR::DTOESEN_0
    }
    #[doc = "Checks if the value of the field is `DTOESEN_1`"]
    #[inline]
    pub fn is_dtoesen_1(&self) -> bool {
        *self == DTOESENR::DTOESEN_1
    }
}
#[doc = "Possible values of the field `DCESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCESENR {
    #[doc = "Masked"]
    DCESEN_0,
    #[doc = "Enabled"]
    DCESEN_1,
}
impl DCESENR {
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
            DCESENR::DCESEN_0 => false,
            DCESENR::DCESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCESENR {
        match value {
            false => DCESENR::DCESEN_0,
            true => DCESENR::DCESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCESEN_0`"]
    #[inline]
    pub fn is_dcesen_0(&self) -> bool {
        *self == DCESENR::DCESEN_0
    }
    #[doc = "Checks if the value of the field is `DCESEN_1`"]
    #[inline]
    pub fn is_dcesen_1(&self) -> bool {
        *self == DCESENR::DCESEN_1
    }
}
#[doc = "Possible values of the field `DEBESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBESENR {
    #[doc = "Masked"]
    DEBESEN_0,
    #[doc = "Enabled"]
    DEBESEN_1,
}
impl DEBESENR {
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
            DEBESENR::DEBESEN_0 => false,
            DEBESENR::DEBESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBESENR {
        match value {
            false => DEBESENR::DEBESEN_0,
            true => DEBESENR::DEBESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBESEN_0`"]
    #[inline]
    pub fn is_debesen_0(&self) -> bool {
        *self == DEBESENR::DEBESEN_0
    }
    #[doc = "Checks if the value of the field is `DEBESEN_1`"]
    #[inline]
    pub fn is_debesen_1(&self) -> bool {
        *self == DEBESENR::DEBESEN_1
    }
}
#[doc = "Possible values of the field `AC12ESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12ESENR {
    #[doc = "Masked"]
    AC12ESEN_0,
    #[doc = "Enabled"]
    AC12ESEN_1,
}
impl AC12ESENR {
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
            AC12ESENR::AC12ESEN_0 => false,
            AC12ESENR::AC12ESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12ESENR {
        match value {
            false => AC12ESENR::AC12ESEN_0,
            true => AC12ESENR::AC12ESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12ESEN_0`"]
    #[inline]
    pub fn is_ac12esen_0(&self) -> bool {
        *self == AC12ESENR::AC12ESEN_0
    }
    #[doc = "Checks if the value of the field is `AC12ESEN_1`"]
    #[inline]
    pub fn is_ac12esen_1(&self) -> bool {
        *self == AC12ESENR::AC12ESEN_1
    }
}
#[doc = "Possible values of the field `TNESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNESENR {
    #[doc = "Masked"]
    TNESEN_0,
    #[doc = "Enabled"]
    TNESEN_1,
}
impl TNESENR {
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
            TNESENR::TNESEN_0 => false,
            TNESENR::TNESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNESENR {
        match value {
            false => TNESENR::TNESEN_0,
            true => TNESENR::TNESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TNESEN_0`"]
    #[inline]
    pub fn is_tnesen_0(&self) -> bool {
        *self == TNESENR::TNESEN_0
    }
    #[doc = "Checks if the value of the field is `TNESEN_1`"]
    #[inline]
    pub fn is_tnesen_1(&self) -> bool {
        *self == TNESENR::TNESEN_1
    }
}
#[doc = "Possible values of the field `DMAESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAESENR {
    #[doc = "Masked"]
    DMAESEN_0,
    #[doc = "Enabled"]
    DMAESEN_1,
}
impl DMAESENR {
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
            DMAESENR::DMAESEN_0 => false,
            DMAESENR::DMAESEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAESENR {
        match value {
            false => DMAESENR::DMAESEN_0,
            true => DMAESENR::DMAESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAESEN_0`"]
    #[inline]
    pub fn is_dmaesen_0(&self) -> bool {
        *self == DMAESENR::DMAESEN_0
    }
    #[doc = "Checks if the value of the field is `DMAESEN_1`"]
    #[inline]
    pub fn is_dmaesen_1(&self) -> bool {
        *self == DMAESENR::DMAESEN_1
    }
}
#[doc = "Values that can be written to the field `CCSEN`"]
pub enum CCSENW {
    #[doc = "Masked"]
    CCSEN_0,
    #[doc = "Enabled"]
    CCSEN_1,
}
impl CCSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCSENW::CCSEN_0 => false,
            CCSENW::CCSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ccsen_0(self) -> &'a mut W {
        self.variant(CCSENW::CCSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ccsen_1(self) -> &'a mut W {
        self.variant(CCSENW::CCSEN_1)
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
#[doc = "Values that can be written to the field `TCSEN`"]
pub enum TCSENW {
    #[doc = "Masked"]
    TCSEN_0,
    #[doc = "Enabled"]
    TCSEN_1,
}
impl TCSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCSENW::TCSEN_0 => false,
            TCSENW::TCSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCSENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn tcsen_0(self) -> &'a mut W {
        self.variant(TCSENW::TCSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tcsen_1(self) -> &'a mut W {
        self.variant(TCSENW::TCSEN_1)
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
#[doc = "Values that can be written to the field `BGESEN`"]
pub enum BGESENW {
    #[doc = "Masked"]
    BGESEN_0,
    #[doc = "Enabled"]
    BGESEN_1,
}
impl BGESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGESENW::BGESEN_0 => false,
            BGESENW::BGESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGESENW<'a> {
    w: &'a mut W,
}
impl<'a> _BGESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn bgesen_0(self) -> &'a mut W {
        self.variant(BGESENW::BGESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn bgesen_1(self) -> &'a mut W {
        self.variant(BGESENW::BGESEN_1)
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
#[doc = "Values that can be written to the field `DINTSEN`"]
pub enum DINTSENW {
    #[doc = "Masked"]
    DINTSEN_0,
    #[doc = "Enabled"]
    DINTSEN_1,
}
impl DINTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DINTSENW::DINTSEN_0 => false,
            DINTSENW::DINTSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _DINTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dintsen_0(self) -> &'a mut W {
        self.variant(DINTSENW::DINTSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dintsen_1(self) -> &'a mut W {
        self.variant(DINTSENW::DINTSEN_1)
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
#[doc = "Values that can be written to the field `BWRSEN`"]
pub enum BWRSENW {
    #[doc = "Masked"]
    BWRSEN_0,
    #[doc = "Enabled"]
    BWRSEN_1,
}
impl BWRSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRSENW::BWRSEN_0 => false,
            BWRSENW::BWRSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _BWRSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWRSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn bwrsen_0(self) -> &'a mut W {
        self.variant(BWRSENW::BWRSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn bwrsen_1(self) -> &'a mut W {
        self.variant(BWRSENW::BWRSEN_1)
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
#[doc = "Values that can be written to the field `BRRSEN`"]
pub enum BRRSENW {
    #[doc = "Masked"]
    BRRSEN_0,
    #[doc = "Enabled"]
    BRRSEN_1,
}
impl BRRSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRRSENW::BRRSEN_0 => false,
            BRRSENW::BRRSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _BRRSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRRSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn brrsen_0(self) -> &'a mut W {
        self.variant(BRRSENW::BRRSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn brrsen_1(self) -> &'a mut W {
        self.variant(BRRSENW::BRRSEN_1)
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
#[doc = "Values that can be written to the field `CINSSEN`"]
pub enum CINSSENW {
    #[doc = "Masked"]
    CINSSEN_0,
    #[doc = "Enabled"]
    CINSSEN_1,
}
impl CINSSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSSENW::CINSSEN_0 => false,
            CINSSENW::CINSSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cinssen_0(self) -> &'a mut W {
        self.variant(CINSSENW::CINSSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cinssen_1(self) -> &'a mut W {
        self.variant(CINSSENW::CINSSEN_1)
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
#[doc = "Values that can be written to the field `CRMSEN`"]
pub enum CRMSENW {
    #[doc = "Masked"]
    CRMSEN_0,
    #[doc = "Enabled"]
    CRMSEN_1,
}
impl CRMSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMSENW::CRMSEN_0 => false,
            CRMSENW::CRMSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn crmsen_0(self) -> &'a mut W {
        self.variant(CRMSENW::CRMSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn crmsen_1(self) -> &'a mut W {
        self.variant(CRMSENW::CRMSEN_1)
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
#[doc = "Values that can be written to the field `CINTSEN`"]
pub enum CINTSENW {
    #[doc = "Masked"]
    CINTSEN_0,
    #[doc = "Enabled"]
    CINTSEN_1,
}
impl CINTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINTSENW::CINTSEN_0 => false,
            CINTSENW::CINTSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cintsen_0(self) -> &'a mut W {
        self.variant(CINTSENW::CINTSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cintsen_1(self) -> &'a mut W {
        self.variant(CINTSENW::CINTSEN_1)
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
#[doc = "Values that can be written to the field `RTESEN`"]
pub enum RTESENW {
    #[doc = "Masked"]
    RTESEN_0,
    #[doc = "Enabled"]
    RTESEN_1,
}
impl RTESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTESENW::RTESEN_0 => false,
            RTESENW::RTESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTESENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn rtesen_0(self) -> &'a mut W {
        self.variant(RTESENW::RTESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn rtesen_1(self) -> &'a mut W {
        self.variant(RTESENW::RTESEN_1)
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
#[doc = "Values that can be written to the field `TPSEN`"]
pub enum TPSENW {
    #[doc = "Masked"]
    TPSEN_0,
    #[doc = "Enabled"]
    TPSEN_1,
}
impl TPSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPSENW::TPSEN_0 => false,
            TPSENW::TPSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPSENW<'a> {
    w: &'a mut W,
}
impl<'a> _TPSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn tpsen_0(self) -> &'a mut W {
        self.variant(TPSENW::TPSEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tpsen_1(self) -> &'a mut W {
        self.variant(TPSENW::TPSEN_1)
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
#[doc = "Values that can be written to the field `CTOESEN`"]
pub enum CTOESENW {
    #[doc = "Masked"]
    CTOESEN_0,
    #[doc = "Enabled"]
    CTOESEN_1,
}
impl CTOESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTOESENW::CTOESEN_0 => false,
            CTOESENW::CTOESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTOESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTOESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ctoesen_0(self) -> &'a mut W {
        self.variant(CTOESENW::CTOESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ctoesen_1(self) -> &'a mut W {
        self.variant(CTOESENW::CTOESEN_1)
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
#[doc = "Values that can be written to the field `CCESEN`"]
pub enum CCESENW {
    #[doc = "Masked"]
    CCESEN_0,
    #[doc = "Enabled"]
    CCESEN_1,
}
impl CCESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCESENW::CCESEN_0 => false,
            CCESENW::CCESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ccesen_0(self) -> &'a mut W {
        self.variant(CCESENW::CCESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ccesen_1(self) -> &'a mut W {
        self.variant(CCESENW::CCESEN_1)
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
#[doc = "Values that can be written to the field `CEBESEN`"]
pub enum CEBESENW {
    #[doc = "Masked"]
    CEBESEN_0,
    #[doc = "Enabled"]
    CEBESEN_1,
}
impl CEBESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEBESENW::CEBESEN_0 => false,
            CEBESENW::CEBESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEBESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CEBESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEBESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cebesen_0(self) -> &'a mut W {
        self.variant(CEBESENW::CEBESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cebesen_1(self) -> &'a mut W {
        self.variant(CEBESENW::CEBESEN_1)
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
#[doc = "Values that can be written to the field `CIESEN`"]
pub enum CIESENW {
    #[doc = "Masked"]
    CIESEN_0,
    #[doc = "Enabled"]
    CIESEN_1,
}
impl CIESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIESENW::CIESEN_0 => false,
            CIESENW::CIESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CIESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ciesen_0(self) -> &'a mut W {
        self.variant(CIESENW::CIESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ciesen_1(self) -> &'a mut W {
        self.variant(CIESENW::CIESEN_1)
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
#[doc = "Values that can be written to the field `DTOESEN`"]
pub enum DTOESENW {
    #[doc = "Masked"]
    DTOESEN_0,
    #[doc = "Enabled"]
    DTOESEN_1,
}
impl DTOESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTOESENW::DTOESEN_0 => false,
            DTOESENW::DTOESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dtoesen_0(self) -> &'a mut W {
        self.variant(DTOESENW::DTOESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dtoesen_1(self) -> &'a mut W {
        self.variant(DTOESENW::DTOESEN_1)
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
#[doc = "Values that can be written to the field `DCESEN`"]
pub enum DCESENW {
    #[doc = "Masked"]
    DCESEN_0,
    #[doc = "Enabled"]
    DCESEN_1,
}
impl DCESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCESENW::DCESEN_0 => false,
            DCESENW::DCESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dcesen_0(self) -> &'a mut W {
        self.variant(DCESENW::DCESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dcesen_1(self) -> &'a mut W {
        self.variant(DCESENW::DCESEN_1)
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
#[doc = "Values that can be written to the field `DEBESEN`"]
pub enum DEBESENW {
    #[doc = "Masked"]
    DEBESEN_0,
    #[doc = "Enabled"]
    DEBESEN_1,
}
impl DEBESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBESENW::DEBESEN_0 => false,
            DEBESENW::DEBESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn debesen_0(self) -> &'a mut W {
        self.variant(DEBESENW::DEBESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn debesen_1(self) -> &'a mut W {
        self.variant(DEBESENW::DEBESEN_1)
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
#[doc = "Values that can be written to the field `AC12ESEN`"]
pub enum AC12ESENW {
    #[doc = "Masked"]
    AC12ESEN_0,
    #[doc = "Enabled"]
    AC12ESEN_1,
}
impl AC12ESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12ESENW::AC12ESEN_0 => false,
            AC12ESENW::AC12ESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12ESENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12ESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12ESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ac12esen_0(self) -> &'a mut W {
        self.variant(AC12ESENW::AC12ESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ac12esen_1(self) -> &'a mut W {
        self.variant(AC12ESENW::AC12ESEN_1)
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
#[doc = "Values that can be written to the field `TNESEN`"]
pub enum TNESENW {
    #[doc = "Masked"]
    TNESEN_0,
    #[doc = "Enabled"]
    TNESEN_1,
}
impl TNESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNESENW::TNESEN_0 => false,
            TNESENW::TNESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNESENW<'a> {
    w: &'a mut W,
}
impl<'a> _TNESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn tnesen_0(self) -> &'a mut W {
        self.variant(TNESENW::TNESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tnesen_1(self) -> &'a mut W {
        self.variant(TNESENW::TNESEN_1)
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
#[doc = "Values that can be written to the field `DMAESEN`"]
pub enum DMAESENW {
    #[doc = "Masked"]
    DMAESEN_0,
    #[doc = "Enabled"]
    DMAESEN_1,
}
impl DMAESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAESENW::DMAESEN_0 => false,
            DMAESENW::DMAESEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dmaesen_0(self) -> &'a mut W {
        self.variant(DMAESENW::DMAESEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dmaesen_1(self) -> &'a mut W {
        self.variant(DMAESENW::DMAESEN_1)
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
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn ccsen(&self) -> CCSENR {
        CCSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn tcsen(&self) -> TCSENR {
        TCSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn bgesen(&self) -> BGESENR {
        BGESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline]
    pub fn dintsen(&self) -> DINTSENR {
        DINTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn bwrsen(&self) -> BWRSENR {
        BWRSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn brrsen(&self) -> BRRSENR {
        BRRSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn cinssen(&self) -> CINSSENR {
        CINSSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn crmsen(&self) -> CRMSENR {
        CRMSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn cintsen(&self) -> CINTSENR {
        CINTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable"]
    #[inline]
    pub fn rtesen(&self) -> RTESENR {
        RTESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Tuning Pass Status Enable"]
    #[inline]
    pub fn tpsen(&self) -> TPSENR {
        TPSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline]
    pub fn ctoesen(&self) -> CTOESENR {
        CTOESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline]
    pub fn ccesen(&self) -> CCESENR {
        CCESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline]
    pub fn cebesen(&self) -> CEBESENR {
        CEBESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline]
    pub fn ciesen(&self) -> CIESENR {
        CIESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline]
    pub fn dtoesen(&self) -> DTOESENR {
        DTOESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline]
    pub fn dcesen(&self) -> DCESENR {
        DCESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline]
    pub fn debesen(&self) -> DEBESENR {
        DEBESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline]
    pub fn ac12esen(&self) -> AC12ESENR {
        AC12ESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline]
    pub fn tnesen(&self) -> TNESENR {
        TNESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline]
    pub fn dmaesen(&self) -> DMAESENR {
        DMAESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn ccsen(&mut self) -> _CCSENW {
        _CCSENW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn tcsen(&mut self) -> _TCSENW {
        _TCSENW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn bgesen(&mut self) -> _BGESENW {
        _BGESENW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline]
    pub fn dintsen(&mut self) -> _DINTSENW {
        _DINTSENW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn bwrsen(&mut self) -> _BWRSENW {
        _BWRSENW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn brrsen(&mut self) -> _BRRSENW {
        _BRRSENW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn cinssen(&mut self) -> _CINSSENW {
        _CINSSENW { w: self }
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn crmsen(&mut self) -> _CRMSENW {
        _CRMSENW { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn cintsen(&mut self) -> _CINTSENW {
        _CINTSENW { w: self }
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable"]
    #[inline]
    pub fn rtesen(&mut self) -> _RTESENW {
        _RTESENW { w: self }
    }
    #[doc = "Bit 14 - Tuning Pass Status Enable"]
    #[inline]
    pub fn tpsen(&mut self) -> _TPSENW {
        _TPSENW { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline]
    pub fn ctoesen(&mut self) -> _CTOESENW {
        _CTOESENW { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline]
    pub fn ccesen(&mut self) -> _CCESENW {
        _CCESENW { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline]
    pub fn cebesen(&mut self) -> _CEBESENW {
        _CEBESENW { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline]
    pub fn ciesen(&mut self) -> _CIESENW {
        _CIESENW { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline]
    pub fn dtoesen(&mut self) -> _DTOESENW {
        _DTOESENW { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline]
    pub fn dcesen(&mut self) -> _DCESENW {
        _DCESENW { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline]
    pub fn debesen(&mut self) -> _DEBESENW {
        _DEBESENW { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline]
    pub fn ac12esen(&mut self) -> _AC12ESENW {
        _AC12ESENW { w: self }
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline]
    pub fn tnesen(&mut self) -> _TNESENW {
        _TNESENW { w: self }
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline]
    pub fn dmaesen(&mut self) -> _DMAESENW {
        _DMAESENW { w: self }
    }
}
