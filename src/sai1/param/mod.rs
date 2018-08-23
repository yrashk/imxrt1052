#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PARAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATALINER {
    bits: u8,
}
impl DATALINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIFOR {
    bits: u8,
}
impl FIFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRAMER {
    bits: u8,
}
impl FRAMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Number of Datalines"]
    #[inline]
    pub fn dataline(&self) -> DATALINER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATALINER { bits }
    }
    #[doc = "Bits 8:11 - FIFO Size"]
    #[inline]
    pub fn fifo(&self) -> FIFOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIFOR { bits }
    }
    #[doc = "Bits 16:19 - Frame Size"]
    #[inline]
    pub fn frame(&self) -> FRAMER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRAMER { bits }
    }
}
