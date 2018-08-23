#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SCR4C {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct R4_0_CTR {
    bits: u16,
}
impl R4_0_CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct R4_1_CTR {
    bits: u16,
}
impl R4_1_CTR {
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
    #[doc = "Bits 0:11 - Runs of Zero, Length 4 Count"]
    #[inline]
    pub fn r4_0_ct(&self) -> R4_0_CTR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        R4_0_CTR { bits }
    }
    #[doc = "Bits 16:27 - Runs of One, Length 4 Count"]
    #[inline]
    pub fn r4_1_ct(&self) -> R4_1_CTR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        R4_1_CTR { bits }
    }
}
