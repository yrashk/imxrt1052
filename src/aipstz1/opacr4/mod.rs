#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACR4 {
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
#[doc = "Possible values of the field `OPAC33`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC33R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC33R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC33R::TP0 => 0,
            OPAC33R::TP1 => 1,
            OPAC33R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC33R {
        match value {
            0 => OPAC33R::TP0,
            1 => OPAC33R::TP1,
            i => OPAC33R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC33R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC33R::TP1
    }
}
#[doc = "Possible values of the field `OPAC32`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC32R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC32R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC32R::TP0 => 0,
            OPAC32R::TP1 => 1,
            OPAC32R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC32R {
        match value {
            0 => OPAC32R::TP0,
            1 => OPAC32R::TP1,
            i => OPAC32R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC32R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC32R::TP1
    }
}
#[doc = "Values that can be written to the field `OPAC33`"]
pub enum OPAC33W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC33W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC33W::TP0 => 0,
            OPAC33W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC33W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC33W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC33W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC33W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC33W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPAC32`"]
pub enum OPAC32W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC32W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC32W::TP0 => 0,
            OPAC32W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC32W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC32W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC32W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC32W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC32W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 33"]
    #[inline]
    pub fn opac33(&self) -> OPAC33R {
        OPAC33R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 32"]
    #[inline]
    pub fn opac32(&self) -> OPAC32R {
        OPAC32R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1145324612 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 33"]
    #[inline]
    pub fn opac33(&mut self) -> _OPAC33W {
        _OPAC33W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 32"]
    #[inline]
    pub fn opac32(&mut self) -> _OPAC32W {
        _OPAC32W { w: self }
    }
}
