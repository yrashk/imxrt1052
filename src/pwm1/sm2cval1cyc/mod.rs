#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::SM2CVAL1CYC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CVAL1CYCR {
    bits: u8,
}
impl CVAL1CYCR {
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
    #[doc = "Bits 0:3 - CVAL1CYC"]
    #[inline]
    pub fn cval1cyc(&self) -> CVAL1CYCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CVAL1CYCR { bits }
    }
}
