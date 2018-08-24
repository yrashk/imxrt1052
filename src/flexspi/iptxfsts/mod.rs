#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IPTXFSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FILLR {
    bits: u8,
}
impl FILLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WRCNTRR {
    bits: u16,
}
impl WRCNTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Fill level of IP TX FIFO."]
    #[inline]
    pub fn fill(&self) -> FILLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FILLR { bits }
    }
    #[doc = "Bits 16:31 - Total Write Data Counter: WRCNTR * 64 Bits."]
    #[inline]
    pub fn wrcntr(&self) -> WRCNTRR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WRCNTRR { bits }
    }
}
