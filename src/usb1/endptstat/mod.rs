#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ENDPTSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ERBRR {
    bits: u8,
}
impl ERBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETBRR {
    bits: u8,
}
impl ETBRR {
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
    #[doc = "Bits 0:7 - Endpoint Receive Buffer Ready -- Read Only"]
    #[inline]
    pub fn erbr(&self) -> ERBRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERBRR { bits }
    }
    #[doc = "Bits 16:23 - Endpoint Transmit Buffer Ready -- Read Only"]
    #[inline]
    pub fn etbr(&self) -> ETBRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETBRR { bits }
    }
}
