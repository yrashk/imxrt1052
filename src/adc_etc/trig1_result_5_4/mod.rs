#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRIG1_RESULT_5_4 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATA4R {
    bits: u16,
}
impl DATA4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA5R {
    bits: u16,
}
impl DATA5R {
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
    #[doc = "Bits 0:11 - Result DATA4"]
    #[inline]
    pub fn data4(&self) -> DATA4R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA4R { bits }
    }
    #[doc = "Bits 16:27 - Result DATA5"]
    #[inline]
    pub fn data5(&self) -> DATA5R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA5R { bits }
    }
}
