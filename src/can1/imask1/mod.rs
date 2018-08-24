#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMASK1 {
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
#[doc = "Possible values of the field `BUFLM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLMR {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    BUFLM_0,
    #[doc = "The corresponding buffer Interrupt is enabled"]
    BUFLM_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl BUFLMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            BUFLMR::BUFLM_0 => 0,
            BUFLMR::BUFLM_1 => 1,
            BUFLMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> BUFLMR {
        match value {
            0 => BUFLMR::BUFLM_0,
            1 => BUFLMR::BUFLM_1,
            i => BUFLMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUFLM_0`"]
    #[inline]
    pub fn is_buflm_0(&self) -> bool {
        *self == BUFLMR::BUFLM_0
    }
    #[doc = "Checks if the value of the field is `BUFLM_1`"]
    #[inline]
    pub fn is_buflm_1(&self) -> bool {
        *self == BUFLMR::BUFLM_1
    }
}
#[doc = "Values that can be written to the field `BUFLM`"]
pub enum BUFLMW {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    BUFLM_0,
    #[doc = "The corresponding buffer Interrupt is enabled"]
    BUFLM_1,
}
impl BUFLMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            BUFLMW::BUFLM_0 => 0,
            BUFLMW::BUFLM_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLMW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The corresponding buffer Interrupt is disabled"]
    #[inline]
    pub fn buflm_0(self) -> &'a mut W {
        self.variant(BUFLMW::BUFLM_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled"]
    #[inline]
    pub fn buflm_1(self) -> &'a mut W {
        self.variant(BUFLMW::BUFLM_1)
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
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
    #[inline]
    pub fn buflm(&self) -> BUFLMR {
        BUFLMR::_from({
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
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
    #[inline]
    pub fn buflm(&mut self) -> _BUFLMW {
        _BUFLMW { w: self }
    }
}
