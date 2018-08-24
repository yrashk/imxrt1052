#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRIG6_RESULT_7_6 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATA6R {
    bits: u16,
}
impl DATA6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA7R {
    bits: u16,
}
impl DATA7R {
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
    #[doc = "Bits 0:11 - Result DATA6"]
    #[inline]
    pub fn data6(&self) -> DATA6R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA6R { bits }
    }
    #[doc = "Bits 16:27 - Result DATA7"]
    #[inline]
    pub fn data7(&self) -> DATA7R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA7R { bits }
    }
}
