#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LPSMCLR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MON_COUNTERR {
    bits: u32,
}
impl MON_COUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Monotonic Counter bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR Register is detected"]
    #[inline]
    pub fn mon_counter(&self) -> MON_COUNTERR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        MON_COUNTERR { bits }
    }
}
