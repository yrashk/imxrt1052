#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ESR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `IMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMBR {
    #[doc = "If ESR2[VPS] is asserted, the ESR2[LPTM] is not an inactive Mailbox."]
    IMB_0,
    #[doc = "If ESR2[VPS] is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    IMB_1,
}
impl IMBR {
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
            IMBR::IMB_0 => false,
            IMBR::IMB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMBR {
        match value {
            false => IMBR::IMB_0,
            true => IMBR::IMB_1,
        }
    }
    #[doc = "Checks if the value of the field is `IMB_0`"]
    #[inline]
    pub fn is_imb_0(&self) -> bool {
        *self == IMBR::IMB_0
    }
    #[doc = "Checks if the value of the field is `IMB_1`"]
    #[inline]
    pub fn is_imb_1(&self) -> bool {
        *self == IMBR::IMB_1
    }
}
#[doc = "Possible values of the field `VPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VPSR {
    #[doc = "Contents of IMB and LPTM are invalid"]
    VPS_0,
    #[doc = "Contents of IMB and LPTM are valid"]
    VPS_1,
}
impl VPSR {
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
            VPSR::VPS_0 => false,
            VPSR::VPS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VPSR {
        match value {
            false => VPSR::VPS_0,
            true => VPSR::VPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `VPS_0`"]
    #[inline]
    pub fn is_vps_0(&self) -> bool {
        *self == VPSR::VPS_0
    }
    #[doc = "Checks if the value of the field is `VPS_1`"]
    #[inline]
    pub fn is_vps_1(&self) -> bool {
        *self == VPSR::VPS_1
    }
}
#[doc = r" Value of the field"]
pub struct LPTMR {
    bits: u8,
}
impl LPTMR {
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
    #[doc = "Bit 13 - If ESR2[VPS] is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)"]
    #[inline]
    pub fn imb(&self) -> IMBR {
        IMBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - This bit indicates whether IMB and LPTM contents are currently valid or not"]
    #[inline]
    pub fn vps(&self) -> VPSR {
        VPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:22 - If ESR2[VPS] is asserted, his 7-bit field indicates the lowest number inactive Mailbox (refer to IMB bit description)"]
    #[inline]
    pub fn lptm(&self) -> LPTMR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPTMR { bits }
    }
}
