#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWDEVICE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCR {
    #[doc = "Not supported"]
    DC_0,
    #[doc = "Supported"]
    DC_1,
}
impl DCR {
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
            DCR::DC_0 => false,
            DCR::DC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCR {
        match value {
            false => DCR::DC_0,
            true => DCR::DC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DC_0`"]
    #[inline]
    pub fn is_dc_0(&self) -> bool {
        *self == DCR::DC_0
    }
    #[doc = "Checks if the value of the field is `DC_1`"]
    #[inline]
    pub fn is_dc_1(&self) -> bool {
        *self == DCR::DC_1
    }
}
#[doc = r" Value of the field"]
pub struct DEVEPR {
    bits: u8,
}
impl DEVEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Device Capable. Indicating whether device operation mode is supported or not."]
    #[inline]
    pub fn dc(&self) -> DCR {
        DCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:5 - Device Endpoint Number"]
    #[inline]
    pub fn devep(&self) -> DEVEPR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVEPR { bits }
    }
}
