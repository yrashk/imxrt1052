#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FCTRL20 {
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
#[doc = "Possible values of the field `NOCOMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCOMBR {
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    NOCOMB_0,
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    NOCOMB_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NOCOMBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NOCOMBR::NOCOMB_0 => 0,
            NOCOMBR::NOCOMB_1 => 1,
            NOCOMBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NOCOMBR {
        match value {
            0 => NOCOMBR::NOCOMB_0,
            1 => NOCOMBR::NOCOMB_1,
            i => NOCOMBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCOMB_0`"]
    #[inline]
    pub fn is_nocomb_0(&self) -> bool {
        *self == NOCOMBR::NOCOMB_0
    }
    #[doc = "Checks if the value of the field is `NOCOMB_1`"]
    #[inline]
    pub fn is_nocomb_1(&self) -> bool {
        *self == NOCOMBR::NOCOMB_1
    }
}
#[doc = "Values that can be written to the field `NOCOMB`"]
pub enum NOCOMBW {
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    NOCOMB_0,
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    NOCOMB_1,
}
impl NOCOMBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NOCOMBW::NOCOMB_0 => 0,
            NOCOMBW::NOCOMB_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOCOMBW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCOMBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOCOMBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    #[inline]
    pub fn nocomb_0(self) -> &'a mut W {
        self.variant(NOCOMBW::NOCOMB_0)
    }
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    #[inline]
    pub fn nocomb_1(self) -> &'a mut W {
        self.variant(NOCOMBW::NOCOMB_1)
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
    #[doc = "Bits 0:3 - No Combinational Path From Fault Input To PWM Output"]
    #[inline]
    pub fn nocomb(&self) -> NOCOMBR {
        NOCOMBR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - No Combinational Path From Fault Input To PWM Output"]
    #[inline]
    pub fn nocomb(&mut self) -> _NOCOMBW {
        _NOCOMBW { w: self }
    }
}
