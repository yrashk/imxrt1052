#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PKRCNT98 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PKR_8_CTR {
    bits: u16,
}
impl PKR_8_CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKR_9_CTR {
    bits: u16,
}
impl PKR_9_CTR {
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
    #[doc = "Bits 0:15 - Poker 8h Count"]
    #[inline]
    pub fn pkr_8_ct(&self) -> PKR_8_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_8_CTR { bits }
    }
    #[doc = "Bits 16:31 - Poker 9h Count"]
    #[inline]
    pub fn pkr_9_ct(&self) -> PKR_9_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_9_CTR { bits }
    }
}
