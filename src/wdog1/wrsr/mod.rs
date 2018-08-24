#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::WRSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SFTW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTWR {
    #[doc = "Reset is not the result of a software reset."]
    SFTW_0,
    #[doc = "Reset is the result of a software reset."]
    SFTW_1,
}
impl SFTWR {
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
            SFTWR::SFTW_0 => false,
            SFTWR::SFTW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SFTWR {
        match value {
            false => SFTWR::SFTW_0,
            true => SFTWR::SFTW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SFTW_0`"]
    #[inline]
    pub fn is_sftw_0(&self) -> bool {
        *self == SFTWR::SFTW_0
    }
    #[doc = "Checks if the value of the field is `SFTW_1`"]
    #[inline]
    pub fn is_sftw_1(&self) -> bool {
        *self == SFTWR::SFTW_1
    }
}
#[doc = "Possible values of the field `TOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUTR {
    #[doc = "Reset is not the result of a WDOG timeout."]
    TOUT_0,
    #[doc = "Reset is the result of a WDOG timeout."]
    TOUT_1,
}
impl TOUTR {
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
            TOUTR::TOUT_0 => false,
            TOUTR::TOUT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOUTR {
        match value {
            false => TOUTR::TOUT_0,
            true => TOUTR::TOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOUT_0`"]
    #[inline]
    pub fn is_tout_0(&self) -> bool {
        *self == TOUTR::TOUT_0
    }
    #[doc = "Checks if the value of the field is `TOUT_1`"]
    #[inline]
    pub fn is_tout_1(&self) -> bool {
        *self == TOUTR::TOUT_1
    }
}
#[doc = "Possible values of the field `POR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR {
    #[doc = "Reset is not the result of a power on reset."]
    POR_0,
    #[doc = "Reset is the result of a power on reset."]
    POR_1,
}
impl PORR {
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
            PORR::POR_0 => false,
            PORR::POR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PORR {
        match value {
            false => PORR::POR_0,
            true => PORR::POR_1,
        }
    }
    #[doc = "Checks if the value of the field is `POR_0`"]
    #[inline]
    pub fn is_por_0(&self) -> bool {
        *self == PORR::POR_0
    }
    #[doc = "Checks if the value of the field is `POR_1`"]
    #[inline]
    pub fn is_por_1(&self) -> bool {
        *self == PORR::POR_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - SFTW"]
    #[inline]
    pub fn sftw(&self) -> SFTWR {
        SFTWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - TOUT"]
    #[inline]
    pub fn tout(&self) -> TOUTR {
        TOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - POR"]
    #[inline]
    pub fn por(&self) -> PORR {
        PORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
