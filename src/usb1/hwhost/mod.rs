#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWHOST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCR {
    #[doc = "Not supported"]
    HC_0,
    #[doc = "Supported"]
    HC_1,
}
impl HCR {
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
            HCR::HC_0 => false,
            HCR::HC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCR {
        match value {
            false => HCR::HC_0,
            true => HCR::HC_1,
        }
    }
    #[doc = "Checks if the value of the field is `HC_0`"]
    #[inline]
    pub fn is_hc_0(&self) -> bool {
        *self == HCR::HC_0
    }
    #[doc = "Checks if the value of the field is `HC_1`"]
    #[inline]
    pub fn is_hc_1(&self) -> bool {
        *self == HCR::HC_1
    }
}
#[doc = r" Value of the field"]
pub struct NPORTR {
    bits: u8,
}
impl NPORTR {
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
    #[doc = "Bit 0 - Host Capable. Indicating whether host operation mode is supported or not."]
    #[inline]
    pub fn hc(&self) -> HCR {
        HCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - The Nmber of downstream ports supported by the host controller is (NPORT+1)"]
    #[inline]
    pub fn nport(&self) -> NPORTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPORTR { bits }
    }
}
