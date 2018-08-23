#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::KDDR {
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
#[doc = "Possible values of the field `KRDD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KRDDR {
    #[doc = "ROWn pin configured as an input."]
    INPUT,
    #[doc = "ROWn pin configured as an output."]
    OUTPUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KRDDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KRDDR::INPUT => 0,
            KRDDR::OUTPUT => 1,
            KRDDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KRDDR {
        match value {
            0 => KRDDR::INPUT,
            1 => KRDDR::OUTPUT,
            i => KRDDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == KRDDR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == KRDDR::OUTPUT
    }
}
#[doc = "Possible values of the field `KCDD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KCDDR {
    #[doc = "COLn pin is configured as an input."]
    INPUT,
    #[doc = "COLn pin is configured as an output."]
    OUTPUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KCDDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KCDDR::INPUT => 0,
            KCDDR::OUTPUT => 1,
            KCDDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KCDDR {
        match value {
            0 => KCDDR::INPUT,
            1 => KCDDR::OUTPUT,
            i => KCDDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == KCDDR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == KCDDR::OUTPUT
    }
}
#[doc = "Values that can be written to the field `KRDD`"]
pub enum KRDDW {
    #[doc = "ROWn pin configured as an input."]
    INPUT,
    #[doc = "ROWn pin configured as an output."]
    OUTPUT,
}
impl KRDDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KRDDW::INPUT => 0,
            KRDDW::OUTPUT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KRDDW<'a> {
    w: &'a mut W,
}
impl<'a> _KRDDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KRDDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ROWn pin configured as an input."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(KRDDW::INPUT)
    }
    #[doc = "ROWn pin configured as an output."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(KRDDW::OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KCDD`"]
pub enum KCDDW {
    #[doc = "COLn pin is configured as an input."]
    INPUT,
    #[doc = "COLn pin is configured as an output."]
    OUTPUT,
}
impl KCDDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KCDDW::INPUT => 0,
            KCDDW::OUTPUT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KCDDW<'a> {
    w: &'a mut W,
}
impl<'a> _KCDDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KCDDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "COLn pin is configured as an input."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(KCDDW::INPUT)
    }
    #[doc = "COLn pin is configured as an output."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(KCDDW::OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:7 - Keypad Row Data Direction"]
    #[inline]
    pub fn krdd(&self) -> KRDDR {
        KRDDR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:15 - Keypad Column Data Direction Register"]
    #[inline]
    pub fn kcdd(&self) -> KCDDR {
        KCDDR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Keypad Row Data Direction"]
    #[inline]
    pub fn krdd(&mut self) -> _KRDDW {
        _KRDDW { w: self }
    }
    #[doc = "Bits 8:15 - Keypad Column Data Direction Register"]
    #[inline]
    pub fn kcdd(&mut self) -> _KCDDW {
        _KCDDW { w: self }
    }
}
