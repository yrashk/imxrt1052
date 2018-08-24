#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SASR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RADDRR {
    bits: u16,
}
impl RADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ANV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANVR {
    #[doc = "Received Address (RADDR) is valid"]
    ANV_0,
    #[doc = "Received Address (RADDR) is not valid"]
    ANV_1,
}
impl ANVR {
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
            ANVR::ANV_0 => false,
            ANVR::ANV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANVR {
        match value {
            false => ANVR::ANV_0,
            true => ANVR::ANV_1,
        }
    }
    #[doc = "Checks if the value of the field is `ANV_0`"]
    #[inline]
    pub fn is_anv_0(&self) -> bool {
        *self == ANVR::ANV_0
    }
    #[doc = "Checks if the value of the field is `ANV_1`"]
    #[inline]
    pub fn is_anv_1(&self) -> bool {
        *self == ANVR::ANV_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Received Address"]
    #[inline]
    pub fn raddr(&self) -> RADDRR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RADDRR { bits }
    }
    #[doc = "Bit 14 - Address Not Valid"]
    #[inline]
    pub fn anv(&self) -> ANVR {
        ANVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
