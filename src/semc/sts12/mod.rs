#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STS12 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NDADDRR {
    bits: u32,
}
impl NDADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    #[inline]
    pub fn ndaddr(&self) -> NDADDRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        NDADDRR { bits }
    }
}
