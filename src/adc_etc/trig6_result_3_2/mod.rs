#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRIG6_RESULT_3_2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATA2R {
    bits: u16,
}
impl DATA2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA3R {
    bits: u16,
}
impl DATA3R {
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
    #[doc = "Bits 0:11 - Result DATA2"]
    #[inline]
    pub fn data2(&self) -> DATA2R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA2R { bits }
    }
    #[doc = "Bits 16:27 - Result DATA3"]
    #[inline]
    pub fn data3(&self) -> DATA3R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA3R { bits }
    }
}
