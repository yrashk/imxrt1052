#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWRXBUF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXBURSTR {
    bits: u8,
}
impl RXBURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXADDR {
    bits: u8,
}
impl RXADDR {
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
    #[doc = "Bits 0:7 - Default burst size for memory to RX buffer transfer"]
    #[inline]
    pub fn rxburst(&self) -> RXBURSTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXBURSTR { bits }
    }
    #[doc = "Bits 8:15 - Buffer total size for all receive endpoints is (2^RXADD)"]
    #[inline]
    pub fn rxadd(&self) -> RXADDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXADDR { bits }
    }
}
