#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SBMR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_CFG1R {
    bits: u8,
}
impl BOOT_CFG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_CFG2R {
    bits: u8,
}
impl BOOT_CFG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_CFG3R {
    bits: u8,
}
impl BOOT_CFG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_CFG4R {
    bits: u8,
}
impl BOOT_CFG4R {
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
    #[doc = "Bits 0:7 - Refer to fusemap."]
    #[inline]
    pub fn boot_cfg1(&self) -> BOOT_CFG1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOOT_CFG1R { bits }
    }
    #[doc = "Bits 8:15 - Refer to fusemap."]
    #[inline]
    pub fn boot_cfg2(&self) -> BOOT_CFG2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOOT_CFG2R { bits }
    }
    #[doc = "Bits 16:23 - Refer to fusemap."]
    #[inline]
    pub fn boot_cfg3(&self) -> BOOT_CFG3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOOT_CFG3R { bits }
    }
    #[doc = "Bits 24:31 - Refer to fusemap."]
    #[inline]
    pub fn boot_cfg4(&self) -> BOOT_CFG4R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOOT_CFG4R { bits }
    }
}
