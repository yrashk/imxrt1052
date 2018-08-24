#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RFR {
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
#[doc = "Possible values of the field `RCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCPR {
    #[doc = "No effect."]
    RCP_0,
    #[doc = "FIFO combine is enabled for FIFO reads and this FIFO will be read on the next FIFO read."]
    RCP_1,
}
impl RCPR {
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
            RCPR::RCP_0 => false,
            RCPR::RCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCPR {
        match value {
            false => RCPR::RCP_0,
            true => RCPR::RCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `RCP_0`"]
    #[inline]
    pub fn is_rcp_0(&self) -> bool {
        *self == RCPR::RCP_0
    }
    #[doc = "Checks if the value of the field is `RCP_1`"]
    #[inline]
    pub fn is_rcp_1(&self) -> bool {
        *self == RCPR::RCP_1
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
    #[doc = "Bit 15 - Receive Channel Pointer"]
    #[inline]
    pub fn rcp(&self) -> RCPR {
        RCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
}
