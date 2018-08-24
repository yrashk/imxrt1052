#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::DCIVERSION {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DCIVERSIONR {
    bits: u16,
}
impl DCIVERSIONR {
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
    #[doc = "Bits 0:15 - Device Controller Interface Version Number Default value is '01h', which means rev0.1."]
    #[inline]
    pub fn dciversion(&self) -> DCIVERSIONR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        DCIVERSIONR { bits }
    }
}
