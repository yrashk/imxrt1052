#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::WSR {
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
#[doc = "Possible values of the field `WSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSRR {
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_21845,
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_43690,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl WSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            WSRR::WSR_21845 => 21845,
            WSRR::WSR_43690 => 43690,
            WSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> WSRR {
        match value {
            21845 => WSRR::WSR_21845,
            43690 => WSRR::WSR_43690,
            i => WSRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WSR_21845`"]
    #[inline]
    pub fn is_wsr_21845(&self) -> bool {
        *self == WSRR::WSR_21845
    }
    #[doc = "Checks if the value of the field is `WSR_43690`"]
    #[inline]
    pub fn is_wsr_43690(&self) -> bool {
        *self == WSRR::WSR_43690
    }
}
#[doc = "Values that can be written to the field `WSR`"]
pub enum WSRW {
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_21845,
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_43690,
}
impl WSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            WSRW::WSR_21845 => 21845,
            WSRW::WSR_43690 => 43690,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSRW<'a> {
    w: &'a mut W,
}
impl<'a> _WSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    #[inline]
    pub fn wsr_21845(self) -> &'a mut W {
        self.variant(WSRW::WSR_21845)
    }
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    #[inline]
    pub fn wsr_43690(self) -> &'a mut W {
        self.variant(WSRW::WSR_43690)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - WSR"]
    #[inline]
    pub fn wsr(&self) -> WSRR {
        WSRR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
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
    #[doc = "Bits 0:15 - WSR"]
    #[inline]
    pub fn wsr(&mut self) -> _WSRW {
        _WSRW { w: self }
    }
}
