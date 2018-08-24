#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::CAPLENGTH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CAPLENGTHR {
    bits: u8,
}
impl CAPLENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - These bits are used as an offset to add to register base to find the beginning of the Operational Register"]
    #[inline]
    pub fn caplength(&self) -> CAPLENGTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        CAPLENGTHR { bits }
    }
}
