#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MEASEURE_VALUE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct Y_VALUER {
    bits: u16,
}
impl Y_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct X_VALUER {
    bits: u16,
}
impl X_VALUER {
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
    #[doc = "Bits 0:11 - Y Value"]
    #[inline]
    pub fn y_value(&self) -> Y_VALUER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        Y_VALUER { bits }
    }
    #[doc = "Bits 16:27 - X Value"]
    #[inline]
    pub fn x_value(&self) -> X_VALUER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        X_VALUER { bits }
    }
}
