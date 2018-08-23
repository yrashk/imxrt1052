#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DLL_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DLL_STS_SLV_LOCKR {
    bits: bool,
}
impl DLL_STS_SLV_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DLL_STS_REF_LOCKR {
    bits: bool,
}
impl DLL_STS_REF_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DLL_STS_SLV_SELR {
    bits: u8,
}
impl DLL_STS_SLV_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLL_STS_REF_SELR {
    bits: u8,
}
impl DLL_STS_REF_SELR {
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
    #[doc = "Bit 0 - DLL_STS_SLV_LOCK"]
    #[inline]
    pub fn dll_sts_slv_lock(&self) -> DLL_STS_SLV_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_STS_SLV_LOCKR { bits }
    }
    #[doc = "Bit 1 - DLL_STS_REF_LOCK"]
    #[inline]
    pub fn dll_sts_ref_lock(&self) -> DLL_STS_REF_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_STS_REF_LOCKR { bits }
    }
    #[doc = "Bits 2:8 - DLL_STS_SLV_SEL"]
    #[inline]
    pub fn dll_sts_slv_sel(&self) -> DLL_STS_SLV_SELR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLL_STS_SLV_SELR { bits }
    }
    #[doc = "Bits 9:15 - DLL_STS_REF_SEL"]
    #[inline]
    pub fn dll_sts_ref_sel(&self) -> DLL_STS_REF_SELR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLL_STS_REF_SELR { bits }
    }
}
