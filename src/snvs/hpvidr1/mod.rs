#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HPVIDR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MINOR_REVR {
    bits: u8,
}
impl MINOR_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJOR_REVR {
    bits: u8,
}
impl MAJOR_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IP_IDR {
    bits: u16,
}
impl IP_IDR {
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
    #[doc = "Bits 0:7 - SNVS block minor version number"]
    #[inline]
    pub fn minor_rev(&self) -> MINOR_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINOR_REVR { bits }
    }
    #[doc = "Bits 8:15 - SNVS block major version number"]
    #[inline]
    pub fn major_rev(&self) -> MAJOR_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJOR_REVR { bits }
    }
    #[doc = "Bits 16:31 - SNVS block ID"]
    #[inline]
    pub fn ip_id(&self) -> IP_IDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IP_IDR { bits }
    }
}
