#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RMON_T_P65TO127 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXPKTSR {
    bits: u16,
}
impl TXPKTSR {
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
    #[doc = "Bits 0:15 - Number of 65- to 127-byte transmit packets"]
    #[inline]
    pub fn txpkts(&self) -> TXPKTSR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TXPKTSR { bits }
    }
}
