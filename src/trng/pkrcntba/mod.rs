#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PKRCNTBA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PKR_A_CTR {
    bits: u16,
}
impl PKR_A_CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKR_B_CTR {
    bits: u16,
}
impl PKR_B_CTR {
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
    #[doc = "Bits 0:15 - Poker Ah Count"]
    #[inline]
    pub fn pkr_a_ct(&self) -> PKR_A_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_A_CTR { bits }
    }
    #[doc = "Bits 16:31 - Poker Bh Count"]
    #[inline]
    pub fn pkr_b_ct(&self) -> PKR_B_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_B_CTR { bits }
    }
}
