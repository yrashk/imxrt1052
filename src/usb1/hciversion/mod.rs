#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::HCIVERSION {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct HCIVERSIONR {
    bits: u16,
}
impl HCIVERSIONR {
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
    #[doc = "Bits 0:15 - Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0."]
    #[inline]
    pub fn hciversion(&self) -> HCIVERSIONR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        HCIVERSIONR { bits }
    }
}
