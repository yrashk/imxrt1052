#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROMPATCHENL {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Address comparator disabled"]
    ENABLE_0,
    #[doc = "Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address"]
    ENABLE_1,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            ENABLER::ENABLE_0 => 0,
            ENABLER::ENABLE_1 => 1,
            ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> ENABLER {
        match value {
            0 => ENABLER::ENABLE_0,
            1 => ENABLER::ENABLE_1,
            i => ENABLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLER::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLER::ENABLE_1
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "Address comparator disabled"]
    ENABLE_0,
    #[doc = "Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address"]
    ENABLE_1,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            ENABLEW::ENABLE_0 => 0,
            ENABLEW::ENABLE_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Address comparator disabled"]
    #[inline]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_0)
    }
    #[doc = "Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address"]
    #[inline]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:15 - Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
}
