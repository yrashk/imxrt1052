#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PKRCNT10 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PKR_0_CTR {
    bits: u16,
}
impl PKR_0_CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKR_1_CTR {
    bits: u16,
}
impl PKR_1_CTR {
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
    #[doc = "Bits 0:15 - Poker 0h Count"]
    #[inline]
    pub fn pkr_0_ct(&self) -> PKR_0_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_0_CTR { bits }
    }
    #[doc = "Bits 16:31 - Poker 1h Count"]
    #[inline]
    pub fn pkr_1_ct(&self) -> PKR_1_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_1_CTR { bits }
    }
}
