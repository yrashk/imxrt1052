#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SCR1C {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct R1_0_CTR {
    bits: u16,
}
impl R1_0_CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct R1_1_CTR {
    bits: u16,
}
impl R1_1_CTR {
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
    #[doc = "Bits 0:14 - Runs of Zero, Length 1 Count"]
    #[inline]
    pub fn r1_0_ct(&self) -> R1_0_CTR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        R1_0_CTR { bits }
    }
    #[doc = "Bits 16:30 - Runs of One, Length 1 Count"]
    #[inline]
    pub fn r1_1_ct(&self) -> R1_1_CTR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        R1_1_CTR { bits }
    }
}
