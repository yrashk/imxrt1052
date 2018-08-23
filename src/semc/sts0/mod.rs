#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STS0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IDLER {
    bits: bool,
}
impl IDLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `NARDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NARDYR {
    #[doc = "NAND device is not ready"]
    NARDY_0,
    #[doc = "NAND device is ready"]
    NARDY_1,
}
impl NARDYR {
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
            NARDYR::NARDY_0 => false,
            NARDYR::NARDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NARDYR {
        match value {
            false => NARDYR::NARDY_0,
            true => NARDYR::NARDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `NARDY_0`"]
    #[inline]
    pub fn is_nardy_0(&self) -> bool {
        *self == NARDYR::NARDY_0
    }
    #[doc = "Checks if the value of the field is `NARDY_1`"]
    #[inline]
    pub fn is_nardy_1(&self) -> bool {
        *self == NARDYR::NARDY_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicating whether SEMC is in IDLE state."]
    #[inline]
    pub fn idle(&self) -> IDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDLER { bits }
    }
    #[doc = "Bit 1 - Indicating NAND device Ready/WAIT# pin level."]
    #[inline]
    pub fn nardy(&self) -> NARDYR {
        NARDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
