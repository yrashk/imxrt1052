#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SRCSH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXCCHANNEL_HR {
    bits: u32,
}
impl RXCCHANNEL_HR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - SPDIF receive C channel register, contains first 24 bits of C channel without interpretation"]
    #[inline]
    pub fn rx_cchannel_h(&self) -> RXCCHANNEL_HR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXCCHANNEL_HR { bits }
    }
}
