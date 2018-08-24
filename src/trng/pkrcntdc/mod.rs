#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PKRCNTDC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PKR_C_CTR {
    bits: u16,
}
impl PKR_C_CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKR_D_CTR {
    bits: u16,
}
impl PKR_D_CTR {
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
    #[doc = "Bits 0:15 - Poker Ch Count"]
    #[inline]
    pub fn pkr_c_ct(&self) -> PKR_C_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_C_CTR { bits }
    }
    #[doc = "Bits 16:31 - Poker Dh Count"]
    #[inline]
    pub fn pkr_d_ct(&self) -> PKR_D_CTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKR_D_CTR { bits }
    }
}
