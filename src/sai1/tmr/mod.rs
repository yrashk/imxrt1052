#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TMR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TWM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWMR {
    #[doc = "Word N is enabled."]
    TWM_0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    TWM_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl TWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            TWMR::TWM_0 => 0,
            TWMR::TWM_1 => 1,
            TWMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> TWMR {
        match value {
            0 => TWMR::TWM_0,
            1 => TWMR::TWM_1,
            i => TWMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TWM_0`"]
    #[inline]
    pub fn is_twm_0(&self) -> bool {
        *self == TWMR::TWM_0
    }
    #[doc = "Checks if the value of the field is `TWM_1`"]
    #[inline]
    pub fn is_twm_1(&self) -> bool {
        *self == TWMR::TWM_1
    }
}
#[doc = "Values that can be written to the field `TWM`"]
pub enum TWMW {
    #[doc = "Word N is enabled."]
    TWM_0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    TWM_1,
}
impl TWMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            TWMW::TWM_0 => 0,
            TWMW::TWM_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWMW<'a> {
    w: &'a mut W,
}
impl<'a> _TWMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn twm_0(self) -> &'a mut W {
        self.variant(TWMW::TWM_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    #[inline]
    pub fn twm_1(self) -> &'a mut W {
        self.variant(TWMW::TWM_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Transmit Word Mask"]
    #[inline]
    pub fn twm(&self) -> TWMR {
        TWMR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Transmit Word Mask"]
    #[inline]
    pub fn twm(&mut self) -> _TWMW {
        _TWMW { w: self }
    }
}
