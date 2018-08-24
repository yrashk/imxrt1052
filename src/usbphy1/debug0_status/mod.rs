#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEBUG0_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LOOP_BACK_FAIL_COUNTR {
    bits: u16,
}
impl LOOP_BACK_FAIL_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UTMI_RXERROR_FAIL_COUNTR {
    bits: u16,
}
impl UTMI_RXERROR_FAIL_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SQUELCH_COUNTR {
    bits: u8,
}
impl SQUELCH_COUNTR {
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
    #[doc = "Bits 0:15 - Running count of the failed pseudo-random generator loopback"]
    #[inline]
    pub fn loop_back_fail_count(&self) -> LOOP_BACK_FAIL_COUNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LOOP_BACK_FAIL_COUNTR { bits }
    }
    #[doc = "Bits 16:25 - Running count of the UTMI_RXERROR."]
    #[inline]
    pub fn utmi_rxerror_fail_count(&self) -> UTMI_RXERROR_FAIL_COUNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        UTMI_RXERROR_FAIL_COUNTR { bits }
    }
    #[doc = "Bits 26:31 - Running count of the squelch reset instead of normal end for HS RX."]
    #[inline]
    pub fn squelch_count(&self) -> SQUELCH_COUNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQUELCH_COUNTR { bits }
    }
}
