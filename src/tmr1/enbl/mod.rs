#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ENBL {
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
#[doc = "Possible values of the field `ENBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBLR {
    #[doc = "Timer channel is disabled."]
    ENBL_0,
    #[doc = "Timer channel is enabled. (default)"]
    ENBL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENBLR::ENBL_0 => 0,
            ENBLR::ENBL_1 => 1,
            ENBLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENBLR {
        match value {
            0 => ENBLR::ENBL_0,
            1 => ENBLR::ENBL_1,
            i => ENBLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENBL_0`"]
    #[inline]
    pub fn is_enbl_0(&self) -> bool {
        *self == ENBLR::ENBL_0
    }
    #[doc = "Checks if the value of the field is `ENBL_1`"]
    #[inline]
    pub fn is_enbl_1(&self) -> bool {
        *self == ENBLR::ENBL_1
    }
}
#[doc = "Values that can be written to the field `ENBL`"]
pub enum ENBLW {
    #[doc = "Timer channel is disabled."]
    ENBL_0,
    #[doc = "Timer channel is enabled. (default)"]
    ENBL_1,
}
impl ENBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENBLW::ENBL_0 => 0,
            ENBLW::ENBL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENBLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENBLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer channel is disabled."]
    #[inline]
    pub fn enbl_0(self) -> &'a mut W {
        self.variant(ENBLW::ENBL_0)
    }
    #[doc = "Timer channel is enabled. (default)"]
    #[inline]
    pub fn enbl_1(self) -> &'a mut W {
        self.variant(ENBLW::ENBL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - Timer Channel Enable"]
    #[inline]
    pub fn enbl(&self) -> ENBLR {
        ENBLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Timer Channel Enable"]
    #[inline]
    pub fn enbl(&mut self) -> _ENBLW {
        _ENBLW { w: self }
    }
}
