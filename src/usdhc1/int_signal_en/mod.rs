#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_SIGNAL_EN {
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
#[doc = "Possible values of the field `CCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIENR {
    #[doc = "Masked"]
    CCIEN_0,
    #[doc = "Enabled"]
    CCIEN_1,
}
impl CCIENR {
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
            CCIENR::CCIEN_0 => false,
            CCIENR::CCIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIENR {
        match value {
            false => CCIENR::CCIEN_0,
            true => CCIENR::CCIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIEN_0`"]
    #[inline]
    pub fn is_ccien_0(&self) -> bool {
        *self == CCIENR::CCIEN_0
    }
    #[doc = "Checks if the value of the field is `CCIEN_1`"]
    #[inline]
    pub fn is_ccien_1(&self) -> bool {
        *self == CCIENR::CCIEN_1
    }
}
#[doc = "Possible values of the field `TCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIENR {
    #[doc = "Masked"]
    TCIEN_0,
    #[doc = "Enabled"]
    TCIEN_1,
}
impl TCIENR {
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
            TCIENR::TCIEN_0 => false,
            TCIENR::TCIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIENR {
        match value {
            false => TCIENR::TCIEN_0,
            true => TCIENR::TCIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCIEN_0`"]
    #[inline]
    pub fn is_tcien_0(&self) -> bool {
        *self == TCIENR::TCIEN_0
    }
    #[doc = "Checks if the value of the field is `TCIEN_1`"]
    #[inline]
    pub fn is_tcien_1(&self) -> bool {
        *self == TCIENR::TCIEN_1
    }
}
#[doc = "Possible values of the field `BGEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGEIENR {
    #[doc = "Masked"]
    BGEIEN_0,
    #[doc = "Enabled"]
    BGEIEN_1,
}
impl BGEIENR {
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
            BGEIENR::BGEIEN_0 => false,
            BGEIENR::BGEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGEIENR {
        match value {
            false => BGEIENR::BGEIEN_0,
            true => BGEIENR::BGEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGEIEN_0`"]
    #[inline]
    pub fn is_bgeien_0(&self) -> bool {
        *self == BGEIENR::BGEIEN_0
    }
    #[doc = "Checks if the value of the field is `BGEIEN_1`"]
    #[inline]
    pub fn is_bgeien_1(&self) -> bool {
        *self == BGEIENR::BGEIEN_1
    }
}
#[doc = "Possible values of the field `DINTIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTIENR {
    #[doc = "Masked"]
    DINTIEN_0,
    #[doc = "Enabled"]
    DINTIEN_1,
}
impl DINTIENR {
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
            DINTIENR::DINTIEN_0 => false,
            DINTIENR::DINTIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DINTIENR {
        match value {
            false => DINTIENR::DINTIEN_0,
            true => DINTIENR::DINTIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DINTIEN_0`"]
    #[inline]
    pub fn is_dintien_0(&self) -> bool {
        *self == DINTIENR::DINTIEN_0
    }
    #[doc = "Checks if the value of the field is `DINTIEN_1`"]
    #[inline]
    pub fn is_dintien_1(&self) -> bool {
        *self == DINTIENR::DINTIEN_1
    }
}
#[doc = "Possible values of the field `BWRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRIENR {
    #[doc = "Masked"]
    BWRIEN_0,
    #[doc = "Enabled"]
    BWRIEN_1,
}
impl BWRIENR {
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
            BWRIENR::BWRIEN_0 => false,
            BWRIENR::BWRIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRIENR {
        match value {
            false => BWRIENR::BWRIEN_0,
            true => BWRIENR::BWRIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWRIEN_0`"]
    #[inline]
    pub fn is_bwrien_0(&self) -> bool {
        *self == BWRIENR::BWRIEN_0
    }
    #[doc = "Checks if the value of the field is `BWRIEN_1`"]
    #[inline]
    pub fn is_bwrien_1(&self) -> bool {
        *self == BWRIENR::BWRIEN_1
    }
}
#[doc = "Possible values of the field `BRRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRIENR {
    #[doc = "Masked"]
    BRRIEN_0,
    #[doc = "Enabled"]
    BRRIEN_1,
}
impl BRRIENR {
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
            BRRIENR::BRRIEN_0 => false,
            BRRIENR::BRRIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRRIENR {
        match value {
            false => BRRIENR::BRRIEN_0,
            true => BRRIENR::BRRIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRRIEN_0`"]
    #[inline]
    pub fn is_brrien_0(&self) -> bool {
        *self == BRRIENR::BRRIEN_0
    }
    #[doc = "Checks if the value of the field is `BRRIEN_1`"]
    #[inline]
    pub fn is_brrien_1(&self) -> bool {
        *self == BRRIENR::BRRIEN_1
    }
}
#[doc = "Possible values of the field `CINSIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSIENR {
    #[doc = "Masked"]
    CINSIEN_0,
    #[doc = "Enabled"]
    CINSIEN_1,
}
impl CINSIENR {
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
            CINSIENR::CINSIEN_0 => false,
            CINSIENR::CINSIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSIENR {
        match value {
            false => CINSIENR::CINSIEN_0,
            true => CINSIENR::CINSIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINSIEN_0`"]
    #[inline]
    pub fn is_cinsien_0(&self) -> bool {
        *self == CINSIENR::CINSIEN_0
    }
    #[doc = "Checks if the value of the field is `CINSIEN_1`"]
    #[inline]
    pub fn is_cinsien_1(&self) -> bool {
        *self == CINSIENR::CINSIEN_1
    }
}
#[doc = "Possible values of the field `CRMIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMIENR {
    #[doc = "Masked"]
    CRMIEN_0,
    #[doc = "Enabled"]
    CRMIEN_1,
}
impl CRMIENR {
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
            CRMIENR::CRMIEN_0 => false,
            CRMIENR::CRMIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRMIENR {
        match value {
            false => CRMIENR::CRMIEN_0,
            true => CRMIENR::CRMIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRMIEN_0`"]
    #[inline]
    pub fn is_crmien_0(&self) -> bool {
        *self == CRMIENR::CRMIEN_0
    }
    #[doc = "Checks if the value of the field is `CRMIEN_1`"]
    #[inline]
    pub fn is_crmien_1(&self) -> bool {
        *self == CRMIENR::CRMIEN_1
    }
}
#[doc = "Possible values of the field `CINTIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTIENR {
    #[doc = "Masked"]
    CINTIEN_0,
    #[doc = "Enabled"]
    CINTIEN_1,
}
impl CINTIENR {
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
            CINTIENR::CINTIEN_0 => false,
            CINTIENR::CINTIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTIENR {
        match value {
            false => CINTIENR::CINTIEN_0,
            true => CINTIENR::CINTIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINTIEN_0`"]
    #[inline]
    pub fn is_cintien_0(&self) -> bool {
        *self == CINTIENR::CINTIEN_0
    }
    #[doc = "Checks if the value of the field is `CINTIEN_1`"]
    #[inline]
    pub fn is_cintien_1(&self) -> bool {
        *self == CINTIENR::CINTIEN_1
    }
}
#[doc = "Possible values of the field `RTEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTEIENR {
    #[doc = "Masked"]
    RTEIEN_0,
    #[doc = "Enabled"]
    RTEIEN_1,
}
impl RTEIENR {
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
            RTEIENR::RTEIEN_0 => false,
            RTEIENR::RTEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTEIENR {
        match value {
            false => RTEIENR::RTEIEN_0,
            true => RTEIENR::RTEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTEIEN_0`"]
    #[inline]
    pub fn is_rteien_0(&self) -> bool {
        *self == RTEIENR::RTEIEN_0
    }
    #[doc = "Checks if the value of the field is `RTEIEN_1`"]
    #[inline]
    pub fn is_rteien_1(&self) -> bool {
        *self == RTEIENR::RTEIEN_1
    }
}
#[doc = "Possible values of the field `TPIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPIENR {
    #[doc = "Masked"]
    TPIEN_0,
    #[doc = "Enabled"]
    TPIEN_1,
}
impl TPIENR {
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
            TPIENR::TPIEN_0 => false,
            TPIENR::TPIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPIENR {
        match value {
            false => TPIENR::TPIEN_0,
            true => TPIENR::TPIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TPIEN_0`"]
    #[inline]
    pub fn is_tpien_0(&self) -> bool {
        *self == TPIENR::TPIEN_0
    }
    #[doc = "Checks if the value of the field is `TPIEN_1`"]
    #[inline]
    pub fn is_tpien_1(&self) -> bool {
        *self == TPIENR::TPIEN_1
    }
}
#[doc = "Possible values of the field `CTOEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOEIENR {
    #[doc = "Masked"]
    CTOEIEN_0,
    #[doc = "Enabled"]
    CTOEIEN_1,
}
impl CTOEIENR {
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
            CTOEIENR::CTOEIEN_0 => false,
            CTOEIENR::CTOEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTOEIENR {
        match value {
            false => CTOEIENR::CTOEIEN_0,
            true => CTOEIENR::CTOEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTOEIEN_0`"]
    #[inline]
    pub fn is_ctoeien_0(&self) -> bool {
        *self == CTOEIENR::CTOEIEN_0
    }
    #[doc = "Checks if the value of the field is `CTOEIEN_1`"]
    #[inline]
    pub fn is_ctoeien_1(&self) -> bool {
        *self == CTOEIENR::CTOEIEN_1
    }
}
#[doc = "Possible values of the field `CCEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCEIENR {
    #[doc = "Masked"]
    CCEIEN_0,
    #[doc = "Enabled"]
    CCEIEN_1,
}
impl CCEIENR {
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
            CCEIENR::CCEIEN_0 => false,
            CCEIENR::CCEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCEIENR {
        match value {
            false => CCEIENR::CCEIEN_0,
            true => CCEIENR::CCEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCEIEN_0`"]
    #[inline]
    pub fn is_cceien_0(&self) -> bool {
        *self == CCEIENR::CCEIEN_0
    }
    #[doc = "Checks if the value of the field is `CCEIEN_1`"]
    #[inline]
    pub fn is_cceien_1(&self) -> bool {
        *self == CCEIENR::CCEIEN_1
    }
}
#[doc = "Possible values of the field `CEBEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBEIENR {
    #[doc = "Masked"]
    CEBEIEN_0,
    #[doc = "Enabled"]
    CEBEIEN_1,
}
impl CEBEIENR {
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
            CEBEIENR::CEBEIEN_0 => false,
            CEBEIENR::CEBEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEBEIENR {
        match value {
            false => CEBEIENR::CEBEIEN_0,
            true => CEBEIENR::CEBEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEBEIEN_0`"]
    #[inline]
    pub fn is_cebeien_0(&self) -> bool {
        *self == CEBEIENR::CEBEIEN_0
    }
    #[doc = "Checks if the value of the field is `CEBEIEN_1`"]
    #[inline]
    pub fn is_cebeien_1(&self) -> bool {
        *self == CEBEIENR::CEBEIEN_1
    }
}
#[doc = "Possible values of the field `CIEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIEIENR {
    #[doc = "Masked"]
    CIEIEN_0,
    #[doc = "Enabled"]
    CIEIEN_1,
}
impl CIEIENR {
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
            CIEIENR::CIEIEN_0 => false,
            CIEIENR::CIEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIEIENR {
        match value {
            false => CIEIENR::CIEIEN_0,
            true => CIEIENR::CIEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIEIEN_0`"]
    #[inline]
    pub fn is_cieien_0(&self) -> bool {
        *self == CIEIENR::CIEIEN_0
    }
    #[doc = "Checks if the value of the field is `CIEIEN_1`"]
    #[inline]
    pub fn is_cieien_1(&self) -> bool {
        *self == CIEIENR::CIEIEN_1
    }
}
#[doc = "Possible values of the field `DTOEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOEIENR {
    #[doc = "Masked"]
    DTOEIEN_0,
    #[doc = "Enabled"]
    DTOEIEN_1,
}
impl DTOEIENR {
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
            DTOEIENR::DTOEIEN_0 => false,
            DTOEIENR::DTOEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTOEIENR {
        match value {
            false => DTOEIENR::DTOEIEN_0,
            true => DTOEIENR::DTOEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTOEIEN_0`"]
    #[inline]
    pub fn is_dtoeien_0(&self) -> bool {
        *self == DTOEIENR::DTOEIEN_0
    }
    #[doc = "Checks if the value of the field is `DTOEIEN_1`"]
    #[inline]
    pub fn is_dtoeien_1(&self) -> bool {
        *self == DTOEIENR::DTOEIEN_1
    }
}
#[doc = "Possible values of the field `DCEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEIENR {
    #[doc = "Masked"]
    DCEIEN_0,
    #[doc = "Enabled"]
    DCEIEN_1,
}
impl DCEIENR {
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
            DCEIENR::DCEIEN_0 => false,
            DCEIENR::DCEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCEIENR {
        match value {
            false => DCEIENR::DCEIEN_0,
            true => DCEIENR::DCEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCEIEN_0`"]
    #[inline]
    pub fn is_dceien_0(&self) -> bool {
        *self == DCEIENR::DCEIEN_0
    }
    #[doc = "Checks if the value of the field is `DCEIEN_1`"]
    #[inline]
    pub fn is_dceien_1(&self) -> bool {
        *self == DCEIENR::DCEIEN_1
    }
}
#[doc = "Possible values of the field `DEBEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBEIENR {
    #[doc = "Masked"]
    DEBEIEN_0,
    #[doc = "Enabled"]
    DEBEIEN_1,
}
impl DEBEIENR {
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
            DEBEIENR::DEBEIEN_0 => false,
            DEBEIENR::DEBEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBEIENR {
        match value {
            false => DEBEIENR::DEBEIEN_0,
            true => DEBEIENR::DEBEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBEIEN_0`"]
    #[inline]
    pub fn is_debeien_0(&self) -> bool {
        *self == DEBEIENR::DEBEIEN_0
    }
    #[doc = "Checks if the value of the field is `DEBEIEN_1`"]
    #[inline]
    pub fn is_debeien_1(&self) -> bool {
        *self == DEBEIENR::DEBEIEN_1
    }
}
#[doc = "Possible values of the field `AC12EIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EIENR {
    #[doc = "Masked"]
    AC12EIEN_0,
    #[doc = "Enabled"]
    AC12EIEN_1,
}
impl AC12EIENR {
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
            AC12EIENR::AC12EIEN_0 => false,
            AC12EIENR::AC12EIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12EIENR {
        match value {
            false => AC12EIENR::AC12EIEN_0,
            true => AC12EIENR::AC12EIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12EIEN_0`"]
    #[inline]
    pub fn is_ac12eien_0(&self) -> bool {
        *self == AC12EIENR::AC12EIEN_0
    }
    #[doc = "Checks if the value of the field is `AC12EIEN_1`"]
    #[inline]
    pub fn is_ac12eien_1(&self) -> bool {
        *self == AC12EIENR::AC12EIEN_1
    }
}
#[doc = "Possible values of the field `TNEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNEIENR {
    #[doc = "Masked"]
    TNEIEN_0,
    #[doc = "Enabled"]
    TNEIEN_1,
}
impl TNEIENR {
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
            TNEIENR::TNEIEN_0 => false,
            TNEIENR::TNEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNEIENR {
        match value {
            false => TNEIENR::TNEIEN_0,
            true => TNEIENR::TNEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TNEIEN_0`"]
    #[inline]
    pub fn is_tneien_0(&self) -> bool {
        *self == TNEIENR::TNEIEN_0
    }
    #[doc = "Checks if the value of the field is `TNEIEN_1`"]
    #[inline]
    pub fn is_tneien_1(&self) -> bool {
        *self == TNEIENR::TNEIEN_1
    }
}
#[doc = "Possible values of the field `DMAEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEIENR {
    #[doc = "Masked"]
    DMAEIEN_0,
    #[doc = "Enable"]
    DMAEIEN_1,
}
impl DMAEIENR {
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
            DMAEIENR::DMAEIEN_0 => false,
            DMAEIENR::DMAEIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAEIENR {
        match value {
            false => DMAEIENR::DMAEIEN_0,
            true => DMAEIENR::DMAEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEIEN_0`"]
    #[inline]
    pub fn is_dmaeien_0(&self) -> bool {
        *self == DMAEIENR::DMAEIEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEIEN_1`"]
    #[inline]
    pub fn is_dmaeien_1(&self) -> bool {
        *self == DMAEIENR::DMAEIEN_1
    }
}
#[doc = "Values that can be written to the field `CCIEN`"]
pub enum CCIENW {
    #[doc = "Masked"]
    CCIEN_0,
    #[doc = "Enabled"]
    CCIEN_1,
}
impl CCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCIENW::CCIEN_0 => false,
            CCIENW::CCIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ccien_0(self) -> &'a mut W {
        self.variant(CCIENW::CCIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ccien_1(self) -> &'a mut W {
        self.variant(CCIENW::CCIEN_1)
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
#[doc = "Values that can be written to the field `TCIEN`"]
pub enum TCIENW {
    #[doc = "Masked"]
    TCIEN_0,
    #[doc = "Enabled"]
    TCIEN_1,
}
impl TCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIENW::TCIEN_0 => false,
            TCIENW::TCIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn tcien_0(self) -> &'a mut W {
        self.variant(TCIENW::TCIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tcien_1(self) -> &'a mut W {
        self.variant(TCIENW::TCIEN_1)
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
#[doc = "Values that can be written to the field `BGEIEN`"]
pub enum BGEIENW {
    #[doc = "Masked"]
    BGEIEN_0,
    #[doc = "Enabled"]
    BGEIEN_1,
}
impl BGEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGEIENW::BGEIEN_0 => false,
            BGEIENW::BGEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _BGEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn bgeien_0(self) -> &'a mut W {
        self.variant(BGEIENW::BGEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn bgeien_1(self) -> &'a mut W {
        self.variant(BGEIENW::BGEIEN_1)
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
#[doc = "Values that can be written to the field `DINTIEN`"]
pub enum DINTIENW {
    #[doc = "Masked"]
    DINTIEN_0,
    #[doc = "Enabled"]
    DINTIEN_1,
}
impl DINTIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DINTIENW::DINTIEN_0 => false,
            DINTIENW::DINTIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINTIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DINTIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINTIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dintien_0(self) -> &'a mut W {
        self.variant(DINTIENW::DINTIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dintien_1(self) -> &'a mut W {
        self.variant(DINTIENW::DINTIEN_1)
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
#[doc = "Values that can be written to the field `BWRIEN`"]
pub enum BWRIENW {
    #[doc = "Masked"]
    BWRIEN_0,
    #[doc = "Enabled"]
    BWRIEN_1,
}
impl BWRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRIENW::BWRIEN_0 => false,
            BWRIENW::BWRIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _BWRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn bwrien_0(self) -> &'a mut W {
        self.variant(BWRIENW::BWRIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn bwrien_1(self) -> &'a mut W {
        self.variant(BWRIENW::BWRIEN_1)
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
#[doc = "Values that can be written to the field `BRRIEN`"]
pub enum BRRIENW {
    #[doc = "Masked"]
    BRRIEN_0,
    #[doc = "Enabled"]
    BRRIEN_1,
}
impl BRRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRRIENW::BRRIEN_0 => false,
            BRRIENW::BRRIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _BRRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn brrien_0(self) -> &'a mut W {
        self.variant(BRRIENW::BRRIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn brrien_1(self) -> &'a mut W {
        self.variant(BRRIENW::BRRIEN_1)
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
#[doc = "Values that can be written to the field `CINSIEN`"]
pub enum CINSIENW {
    #[doc = "Masked"]
    CINSIEN_0,
    #[doc = "Enabled"]
    CINSIEN_1,
}
impl CINSIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSIENW::CINSIEN_0 => false,
            CINSIENW::CINSIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cinsien_0(self) -> &'a mut W {
        self.variant(CINSIENW::CINSIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cinsien_1(self) -> &'a mut W {
        self.variant(CINSIENW::CINSIEN_1)
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
#[doc = "Values that can be written to the field `CRMIEN`"]
pub enum CRMIENW {
    #[doc = "Masked"]
    CRMIEN_0,
    #[doc = "Enabled"]
    CRMIEN_1,
}
impl CRMIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMIENW::CRMIEN_0 => false,
            CRMIENW::CRMIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn crmien_0(self) -> &'a mut W {
        self.variant(CRMIENW::CRMIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn crmien_1(self) -> &'a mut W {
        self.variant(CRMIENW::CRMIEN_1)
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
#[doc = "Values that can be written to the field `CINTIEN`"]
pub enum CINTIENW {
    #[doc = "Masked"]
    CINTIEN_0,
    #[doc = "Enabled"]
    CINTIEN_1,
}
impl CINTIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINTIENW::CINTIEN_0 => false,
            CINTIENW::CINTIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINTIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINTIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cintien_0(self) -> &'a mut W {
        self.variant(CINTIENW::CINTIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cintien_1(self) -> &'a mut W {
        self.variant(CINTIENW::CINTIEN_1)
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
#[doc = "Values that can be written to the field `RTEIEN`"]
pub enum RTEIENW {
    #[doc = "Masked"]
    RTEIEN_0,
    #[doc = "Enabled"]
    RTEIEN_1,
}
impl RTEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTEIENW::RTEIEN_0 => false,
            RTEIENW::RTEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn rteien_0(self) -> &'a mut W {
        self.variant(RTEIENW::RTEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn rteien_1(self) -> &'a mut W {
        self.variant(RTEIENW::RTEIEN_1)
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
#[doc = "Values that can be written to the field `TPIEN`"]
pub enum TPIENW {
    #[doc = "Masked"]
    TPIEN_0,
    #[doc = "Enabled"]
    TPIEN_1,
}
impl TPIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPIENW::TPIEN_0 => false,
            TPIENW::TPIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TPIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn tpien_0(self) -> &'a mut W {
        self.variant(TPIENW::TPIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tpien_1(self) -> &'a mut W {
        self.variant(TPIENW::TPIEN_1)
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
#[doc = "Values that can be written to the field `CTOEIEN`"]
pub enum CTOEIENW {
    #[doc = "Masked"]
    CTOEIEN_0,
    #[doc = "Enabled"]
    CTOEIEN_1,
}
impl CTOEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTOEIENW::CTOEIEN_0 => false,
            CTOEIENW::CTOEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTOEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTOEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ctoeien_0(self) -> &'a mut W {
        self.variant(CTOEIENW::CTOEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ctoeien_1(self) -> &'a mut W {
        self.variant(CTOEIENW::CTOEIEN_1)
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
#[doc = "Values that can be written to the field `CCEIEN`"]
pub enum CCEIENW {
    #[doc = "Masked"]
    CCEIEN_0,
    #[doc = "Enabled"]
    CCEIEN_1,
}
impl CCEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCEIENW::CCEIEN_0 => false,
            CCEIENW::CCEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cceien_0(self) -> &'a mut W {
        self.variant(CCEIENW::CCEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cceien_1(self) -> &'a mut W {
        self.variant(CCEIENW::CCEIEN_1)
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
#[doc = "Values that can be written to the field `CEBEIEN`"]
pub enum CEBEIENW {
    #[doc = "Masked"]
    CEBEIEN_0,
    #[doc = "Enabled"]
    CEBEIEN_1,
}
impl CEBEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEBEIENW::CEBEIEN_0 => false,
            CEBEIENW::CEBEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEBEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CEBEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEBEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cebeien_0(self) -> &'a mut W {
        self.variant(CEBEIENW::CEBEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cebeien_1(self) -> &'a mut W {
        self.variant(CEBEIENW::CEBEIEN_1)
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
#[doc = "Values that can be written to the field `CIEIEN`"]
pub enum CIEIENW {
    #[doc = "Masked"]
    CIEIEN_0,
    #[doc = "Enabled"]
    CIEIEN_1,
}
impl CIEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIEIENW::CIEIEN_0 => false,
            CIEIENW::CIEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CIEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn cieien_0(self) -> &'a mut W {
        self.variant(CIEIENW::CIEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn cieien_1(self) -> &'a mut W {
        self.variant(CIEIENW::CIEIEN_1)
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
#[doc = "Values that can be written to the field `DTOEIEN`"]
pub enum DTOEIENW {
    #[doc = "Masked"]
    DTOEIEN_0,
    #[doc = "Enabled"]
    DTOEIEN_1,
}
impl DTOEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTOEIENW::DTOEIEN_0 => false,
            DTOEIENW::DTOEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dtoeien_0(self) -> &'a mut W {
        self.variant(DTOEIENW::DTOEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dtoeien_1(self) -> &'a mut W {
        self.variant(DTOEIENW::DTOEIEN_1)
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
#[doc = "Values that can be written to the field `DCEIEN`"]
pub enum DCEIENW {
    #[doc = "Masked"]
    DCEIEN_0,
    #[doc = "Enabled"]
    DCEIEN_1,
}
impl DCEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEIENW::DCEIEN_0 => false,
            DCEIENW::DCEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dceien_0(self) -> &'a mut W {
        self.variant(DCEIENW::DCEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dceien_1(self) -> &'a mut W {
        self.variant(DCEIENW::DCEIEN_1)
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
#[doc = "Values that can be written to the field `DEBEIEN`"]
pub enum DEBEIENW {
    #[doc = "Masked"]
    DEBEIEN_0,
    #[doc = "Enabled"]
    DEBEIEN_1,
}
impl DEBEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBEIENW::DEBEIEN_0 => false,
            DEBEIENW::DEBEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn debeien_0(self) -> &'a mut W {
        self.variant(DEBEIENW::DEBEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn debeien_1(self) -> &'a mut W {
        self.variant(DEBEIENW::DEBEIEN_1)
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
#[doc = "Values that can be written to the field `AC12EIEN`"]
pub enum AC12EIENW {
    #[doc = "Masked"]
    AC12EIEN_0,
    #[doc = "Enabled"]
    AC12EIEN_1,
}
impl AC12EIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12EIENW::AC12EIEN_0 => false,
            AC12EIENW::AC12EIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12EIENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12EIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12EIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ac12eien_0(self) -> &'a mut W {
        self.variant(AC12EIENW::AC12EIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ac12eien_1(self) -> &'a mut W {
        self.variant(AC12EIENW::AC12EIEN_1)
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
#[doc = "Values that can be written to the field `TNEIEN`"]
pub enum TNEIENW {
    #[doc = "Masked"]
    TNEIEN_0,
    #[doc = "Enabled"]
    TNEIEN_1,
}
impl TNEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNEIENW::TNEIEN_0 => false,
            TNEIENW::TNEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TNEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn tneien_0(self) -> &'a mut W {
        self.variant(TNEIENW::TNEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tneien_1(self) -> &'a mut W {
        self.variant(TNEIENW::TNEIEN_1)
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
#[doc = "Values that can be written to the field `DMAEIEN`"]
pub enum DMAEIENW {
    #[doc = "Masked"]
    DMAEIEN_0,
    #[doc = "Enable"]
    DMAEIEN_1,
}
impl DMAEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAEIENW::DMAEIEN_0 => false,
            DMAEIENW::DMAEIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dmaeien_0(self) -> &'a mut W {
        self.variant(DMAEIENW::DMAEIEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn dmaeien_1(self) -> &'a mut W {
        self.variant(DMAEIENW::DMAEIEN_1)
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
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccien(&self) -> CCIENR {
        CCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcien(&self) -> TCIENR {
        TCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline]
    pub fn bgeien(&self) -> BGEIENR {
        BGEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline]
    pub fn dintien(&self) -> DINTIENR {
        DINTIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline]
    pub fn bwrien(&self) -> BWRIENR {
        BWRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline]
    pub fn brrien(&self) -> BRRIENR {
        BRRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline]
    pub fn cinsien(&self) -> CINSIENR {
        CINSIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline]
    pub fn crmien(&self) -> CRMIENR {
        CRMIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt Interrupt Enable"]
    #[inline]
    pub fn cintien(&self) -> CINTIENR {
        CINTIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Re-Tuning Event Interrupt Enable"]
    #[inline]
    pub fn rteien(&self) -> RTEIENR {
        RTEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Tuning Pass Interrupt Enable"]
    #[inline]
    pub fn tpien(&self) -> TPIENR {
        TPIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline]
    pub fn ctoeien(&self) -> CTOEIENR {
        CTOEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline]
    pub fn cceien(&self) -> CCEIENR {
        CCEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline]
    pub fn cebeien(&self) -> CEBEIENR {
        CEBEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline]
    pub fn cieien(&self) -> CIEIENR {
        CIEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline]
    pub fn dtoeien(&self) -> DTOEIENR {
        DTOEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline]
    pub fn dceien(&self) -> DCEIENR {
        DCEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline]
    pub fn debeien(&self) -> DEBEIENR {
        DEBEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline]
    pub fn ac12eien(&self) -> AC12EIENR {
        AC12EIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Tuning Error Interrupt Enable"]
    #[inline]
    pub fn tneien(&self) -> TNEIENR {
        TNEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline]
    pub fn dmaeien(&self) -> DMAEIENR {
        DMAEIENR::_from({
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
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccien(&mut self) -> _CCIENW {
        _CCIENW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcien(&mut self) -> _TCIENW {
        _TCIENW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline]
    pub fn bgeien(&mut self) -> _BGEIENW {
        _BGEIENW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline]
    pub fn dintien(&mut self) -> _DINTIENW {
        _DINTIENW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline]
    pub fn bwrien(&mut self) -> _BWRIENW {
        _BWRIENW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline]
    pub fn brrien(&mut self) -> _BRRIENW {
        _BRRIENW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline]
    pub fn cinsien(&mut self) -> _CINSIENW {
        _CINSIENW { w: self }
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline]
    pub fn crmien(&mut self) -> _CRMIENW {
        _CRMIENW { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Interrupt Enable"]
    #[inline]
    pub fn cintien(&mut self) -> _CINTIENW {
        _CINTIENW { w: self }
    }
    #[doc = "Bit 12 - Re-Tuning Event Interrupt Enable"]
    #[inline]
    pub fn rteien(&mut self) -> _RTEIENW {
        _RTEIENW { w: self }
    }
    #[doc = "Bit 14 - Tuning Pass Interrupt Enable"]
    #[inline]
    pub fn tpien(&mut self) -> _TPIENW {
        _TPIENW { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline]
    pub fn ctoeien(&mut self) -> _CTOEIENW {
        _CTOEIENW { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline]
    pub fn cceien(&mut self) -> _CCEIENW {
        _CCEIENW { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline]
    pub fn cebeien(&mut self) -> _CEBEIENW {
        _CEBEIENW { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline]
    pub fn cieien(&mut self) -> _CIEIENW {
        _CIEIENW { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline]
    pub fn dtoeien(&mut self) -> _DTOEIENW {
        _DTOEIENW { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline]
    pub fn dceien(&mut self) -> _DCEIENW {
        _DCEIENW { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline]
    pub fn debeien(&mut self) -> _DEBEIENW {
        _DEBEIENW { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline]
    pub fn ac12eien(&mut self) -> _AC12EIENW {
        _AC12EIENW { w: self }
    }
    #[doc = "Bit 26 - Tuning Error Interrupt Enable"]
    #[inline]
    pub fn tneien(&mut self) -> _TNEIENW {
        _TNEIENW { w: self }
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline]
    pub fn dmaeien(&mut self) -> _DMAEIENW {
        _DMAEIENW { w: self }
    }
}
