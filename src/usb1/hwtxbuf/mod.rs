#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWTXBUF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXBURSTR {
    bits: u8,
}
impl TXBURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXCHANADDR {
    bits: u8,
}
impl TXCHANADDR {
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
    #[doc = "Bits 0:7 - Default burst size for memory to TX buffer transfer"]
    #[inline]
    pub fn txburst(&self) -> TXBURSTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXBURSTR { bits }
    }
    #[doc = "Bits 16:23 - TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes"]
    #[inline]
    pub fn txchanadd(&self) -> TXCHANADDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCHANADDR { bits }
    }
}
