#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STS2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ASLVLOCKR {
    bits: bool,
}
impl ASLVLOCKR {
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
pub struct AREFLOCKR {
    bits: bool,
}
impl AREFLOCKR {
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
pub struct ASLVSELR {
    bits: u8,
}
impl ASLVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AREFSELR {
    bits: u8,
}
impl AREFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BSLVLOCKR {
    bits: bool,
}
impl BSLVLOCKR {
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
pub struct BREFLOCKR {
    bits: bool,
}
impl BREFLOCKR {
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
pub struct BSLVSELR {
    bits: u8,
}
impl BSLVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BREFSELR {
    bits: u8,
}
impl BREFSELR {
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
    #[doc = "Bit 0 - Flash A sample clock slave delay line locked."]
    #[inline]
    pub fn aslvlock(&self) -> ASLVLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASLVLOCKR { bits }
    }
    #[doc = "Bit 1 - Flash A sample clock reference delay line locked."]
    #[inline]
    pub fn areflock(&self) -> AREFLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AREFLOCKR { bits }
    }
    #[doc = "Bits 2:7 - Flash A sample clock slave delay line delay cell number selection ."]
    #[inline]
    pub fn aslvsel(&self) -> ASLVSELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ASLVSELR { bits }
    }
    #[doc = "Bits 8:13 - Flash A sample clock reference delay line delay cell number selection."]
    #[inline]
    pub fn arefsel(&self) -> AREFSELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AREFSELR { bits }
    }
    #[doc = "Bit 16 - Flash B sample clock slave delay line locked."]
    #[inline]
    pub fn bslvlock(&self) -> BSLVLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSLVLOCKR { bits }
    }
    #[doc = "Bit 17 - Flash B sample clock reference delay line locked."]
    #[inline]
    pub fn breflock(&self) -> BREFLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BREFLOCKR { bits }
    }
    #[doc = "Bits 18:23 - Flash B sample clock slave delay line delay cell number selection."]
    #[inline]
    pub fn bslvsel(&self) -> BSLVSELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BSLVSELR { bits }
    }
    #[doc = "Bits 24:29 - Flash B sample clock reference delay line delay cell number selection."]
    #[inline]
    pub fn brefsel(&self) -> BREFSELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BREFSELR { bits }
    }
}
