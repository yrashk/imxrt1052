#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::SM0CVAL4CYC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CVAL4CYCR {
    bits: u8,
}
impl CVAL4CYCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - CVAL4CYC"]
    #[inline]
    pub fn cval4cyc(&self) -> CVAL4CYCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CVAL4CYCR { bits }
    }
}
