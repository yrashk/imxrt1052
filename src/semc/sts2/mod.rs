#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STS2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `NDWRPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDWRPENDR {
    #[doc = "No pending"]
    NDWRPEND_0,
    #[doc = "Pending"]
    NDWRPEND_1,
}
impl NDWRPENDR {
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
            NDWRPENDR::NDWRPEND_0 => false,
            NDWRPENDR::NDWRPEND_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDWRPENDR {
        match value {
            false => NDWRPENDR::NDWRPEND_0,
            true => NDWRPENDR::NDWRPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDWRPEND_0`"]
    #[inline]
    pub fn is_ndwrpend_0(&self) -> bool {
        *self == NDWRPENDR::NDWRPEND_0
    }
    #[doc = "Checks if the value of the field is `NDWRPEND_1`"]
    #[inline]
    pub fn is_ndwrpend_1(&self) -> bool {
        *self == NDWRPENDR::NDWRPEND_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - This field indicating whether there is pending AXI command (write) to NAND device."]
    #[inline]
    pub fn ndwrpend(&self) -> NDWRPENDR {
        NDWRPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
