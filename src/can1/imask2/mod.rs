#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMASK2 {
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
#[doc = "Possible values of the field `BUFHM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFHMR {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    BUFHM_0,
    #[doc = "The corresponding buffer Interrupt is enabled"]
    BUFHM_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl BUFHMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            BUFHMR::BUFHM_0 => 0,
            BUFHMR::BUFHM_1 => 1,
            BUFHMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> BUFHMR {
        match value {
            0 => BUFHMR::BUFHM_0,
            1 => BUFHMR::BUFHM_1,
            i => BUFHMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUFHM_0`"]
    #[inline]
    pub fn is_bufhm_0(&self) -> bool {
        *self == BUFHMR::BUFHM_0
    }
    #[doc = "Checks if the value of the field is `BUFHM_1`"]
    #[inline]
    pub fn is_bufhm_1(&self) -> bool {
        *self == BUFHMR::BUFHM_1
    }
}
#[doc = "Values that can be written to the field `BUFHM`"]
pub enum BUFHMW {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    BUFHM_0,
    #[doc = "The corresponding buffer Interrupt is enabled"]
    BUFHM_1,
}
impl BUFHMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            BUFHMW::BUFHM_0 => 0,
            BUFHMW::BUFHM_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFHMW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFHMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFHMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The corresponding buffer Interrupt is disabled"]
    #[inline]
    pub fn bufhm_0(self) -> &'a mut W {
        self.variant(BUFHMW::BUFHM_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled"]
    #[inline]
    pub fn bufhm_1(self) -> &'a mut W {
        self.variant(BUFHMW::BUFHM_1)
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
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[inline]
    pub fn bufhm(&self) -> BUFHMR {
        BUFHMR::_from({
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
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[inline]
    pub fn bufhm(&mut self) -> _BUFHMW {
        _BUFHMW { w: self }
    }
}
