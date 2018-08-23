#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PKRCNT76 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PKR_6_CTR {
    bits: u16,
}
impl PKR_6_CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKR_7_CTR {
    bits: u16,
}
impl PKR_7_CTR {
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
    #[doc = "Bits 0:15 - Poker 6h Count"]
    #[inline]
    pub fn pkr_6_ct(&self) -> PKR_6_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_6_CTR { bits }
    }
    #[doc = "Bits 16:31 - Poker 7h Count"]
    #[inline]
    pub fn pkr_7_ct(&self) -> PKR_7_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_7_CTR { bits }
    }
}
