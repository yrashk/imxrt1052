#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::POSDH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct POSDHR {
    bits: u16,
}
impl POSDHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:15 - This read-only register contains a snapshot of the value of the POSD register"]
    #[inline]
    pub fn posdh(&self) -> POSDHR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        POSDHR { bits }
    }
}
