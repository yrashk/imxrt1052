#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TOTSAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TOT_SAMR {
    bits: u32,
}
impl TOT_SAMR {
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
    #[doc = "Bits 0:19 - Total Samples"]
    #[inline]
    pub fn tot_sam(&self) -> TOT_SAMR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TOT_SAMR { bits }
    }
}
