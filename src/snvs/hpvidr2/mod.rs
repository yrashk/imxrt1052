#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HPVIDR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CONFIG_OPTR {
    bits: u8,
}
impl CONFIG_OPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ECO_REVR {
    bits: u8,
}
impl ECO_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTG_OPTR {
    bits: u8,
}
impl INTG_OPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IP_ERAR {
    bits: u8,
}
impl IP_ERAR {
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
    #[doc = "Bits 0:7 - SNVS Configuration Options"]
    #[inline]
    pub fn config_opt(&self) -> CONFIG_OPTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CONFIG_OPTR { bits }
    }
    #[doc = "Bits 8:15 - SNVS ECO Revision"]
    #[inline]
    pub fn eco_rev(&self) -> ECO_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECO_REVR { bits }
    }
    #[doc = "Bits 16:23 - SNVS Integration Options"]
    #[inline]
    pub fn intg_opt(&self) -> INTG_OPTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTG_OPTR { bits }
    }
    #[doc = "Bits 24:31 - IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5"]
    #[inline]
    pub fn ip_era(&self) -> IP_ERAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IP_ERAR { bits }
    }
}
