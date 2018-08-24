#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TFR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RFPR {
    bits: u8,
}
impl RFPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WFPR {
    bits: u8,
}
impl WFPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCPR {
    #[doc = "No effect."]
    WCP_0,
    #[doc = "FIFO combine is enabled for FIFO writes and this FIFO will be written on the next FIFO write."]
    WCP_1,
}
impl WCPR {
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
            WCPR::WCP_0 => false,
            WCPR::WCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCPR {
        match value {
            false => WCPR::WCP_0,
            true => WCPR::WCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `WCP_0`"]
    #[inline]
    pub fn is_wcp_0(&self) -> bool {
        *self == WCPR::WCP_0
    }
    #[doc = "Checks if the value of the field is `WCP_1`"]
    #[inline]
    pub fn is_wcp_1(&self) -> bool {
        *self == WCPR::WCP_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Read FIFO Pointer"]
    #[inline]
    pub fn rfp(&self) -> RFPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFPR { bits }
    }
    #[doc = "Bits 16:21 - Write FIFO Pointer"]
    #[inline]
    pub fn wfp(&self) -> WFPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WFPR { bits }
    }
    #[doc = "Bit 31 - Write Channel Pointer"]
    #[inline]
    pub fn wcp(&self) -> WCPR {
        WCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
