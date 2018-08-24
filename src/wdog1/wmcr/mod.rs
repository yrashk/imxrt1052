#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::WMCR {
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
#[doc = "Possible values of the field `PDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDER {
    #[doc = "Power Down Counter of WDOG is disabled."]
    PDE_0,
    #[doc = "Power Down Counter of WDOG is enabled (Default)."]
    PDE_1,
}
impl PDER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PDER::PDE_0 => false,
            PDER::PDE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDER {
        match value {
            false => PDER::PDE_0,
            true => PDER::PDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDE_0`"]
    #[inline]
    pub fn is_pde_0(&self) -> bool {
        *self == PDER::PDE_0
    }
    #[doc = "Checks if the value of the field is `PDE_1`"]
    #[inline]
    pub fn is_pde_1(&self) -> bool {
        *self == PDER::PDE_1
    }
}
#[doc = "Values that can be written to the field `PDE`"]
pub enum PDEW {
    #[doc = "Power Down Counter of WDOG is disabled."]
    PDE_0,
    #[doc = "Power Down Counter of WDOG is enabled (Default)."]
    PDE_1,
}
impl PDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEW::PDE_0 => false,
            PDEW::PDE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power Down Counter of WDOG is disabled."]
    #[inline]
    pub fn pde_0(self) -> &'a mut W {
        self.variant(PDEW::PDE_0)
    }
    #[doc = "Power Down Counter of WDOG is enabled (Default)."]
    #[inline]
    pub fn pde_1(self) -> &'a mut W {
        self.variant(PDEW::PDE_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bit 0 - PDE"]
    #[inline]
    pub fn pde(&self) -> PDER {
        PDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PDE"]
    #[inline]
    pub fn pde(&mut self) -> _PDEW {
        _PDEW { w: self }
    }
}
