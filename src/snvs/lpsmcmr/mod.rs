#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LPSMCMR {
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
    bits: u16,
}
impl MON_COUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MC_ERA_BITSR {
    bits: u16,
}
impl MC_ERA_BITSR {
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
    #[doc = "Bits 0:15 - Monotonic Counter most-significant 16 Bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR register is detected"]
    #[inline]
    pub fn mon_counter(&self) -> MON_COUNTERR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MON_COUNTERR { bits }
    }
    #[doc = "Bits 16:31 - Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses."]
    #[inline]
    pub fn mc_era_bits(&self) -> MC_ERA_BITSR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MC_ERA_BITSR { bits }
    }
}
