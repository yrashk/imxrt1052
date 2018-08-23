#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::SM3CVAL0CYC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CVAL0CYCR {
    bits: u8,
}
impl CVAL0CYCR {
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
    #[doc = "Bits 0:3 - CVAL0CYC"]
    #[inline]
    pub fn cval0cyc(&self) -> CVAL0CYCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CVAL0CYCR { bits }
    }
}
