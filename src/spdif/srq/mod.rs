#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SRQ {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXQCHANNELR {
    bits: u32,
}
impl RXQCHANNELR {
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
    #[doc = "Bits 0:23 - SPDIF receive Q channel register, contains next 3 Q channel bytes"]
    #[inline]
    pub fn rx_qchannel(&self) -> RXQCHANNELR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXQCHANNELR { bits }
    }
}
