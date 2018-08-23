#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ADMA_ERR_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ADMAESR {
    bits: u8,
}
impl ADMAESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ADMALME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMALMER {
    #[doc = "No Error"]
    ADMALME_0,
    #[doc = "Error"]
    ADMALME_1,
}
impl ADMALMER {
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
            ADMALMER::ADMALME_0 => false,
            ADMALMER::ADMALME_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMALMER {
        match value {
            false => ADMALMER::ADMALME_0,
            true => ADMALMER::ADMALME_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMALME_0`"]
    #[inline]
    pub fn is_admalme_0(&self) -> bool {
        *self == ADMALMER::ADMALME_0
    }
    #[doc = "Checks if the value of the field is `ADMALME_1`"]
    #[inline]
    pub fn is_admalme_1(&self) -> bool {
        *self == ADMALMER::ADMALME_1
    }
}
#[doc = "Possible values of the field `ADMADCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMADCER {
    #[doc = "No Error"]
    ADMADCE_0,
    #[doc = "Error"]
    ADMADCE_1,
}
impl ADMADCER {
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
            ADMADCER::ADMADCE_0 => false,
            ADMADCER::ADMADCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMADCER {
        match value {
            false => ADMADCER::ADMADCE_0,
            true => ADMADCER::ADMADCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMADCE_0`"]
    #[inline]
    pub fn is_admadce_0(&self) -> bool {
        *self == ADMADCER::ADMADCE_0
    }
    #[doc = "Checks if the value of the field is `ADMADCE_1`"]
    #[inline]
    pub fn is_admadce_1(&self) -> bool {
        *self == ADMADCER::ADMADCE_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ADMA Error State (when ADMA Error is occurred)"]
    #[inline]
    pub fn admaes(&self) -> ADMAESR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADMAESR { bits }
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline]
    pub fn admalme(&self) -> ADMALMER {
        ADMALMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ADMA Descriptor Error"]
    #[inline]
    pub fn admadce(&self) -> ADMADCER {
        ADMADCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
