#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINCFG {
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
#[doc = "Possible values of the field `TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSELR {
    #[doc = "Input trigger is disabled."]
    TRGSEL_0,
    #[doc = "Input trigger is used instead of RXD pin input."]
    TRGSEL_1,
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    TRGSEL_2,
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    TRGSEL_3,
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGSELR::TRGSEL_0 => 0,
            TRGSELR::TRGSEL_1 => 1,
            TRGSELR::TRGSEL_2 => 2,
            TRGSELR::TRGSEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGSELR {
        match value {
            0 => TRGSELR::TRGSEL_0,
            1 => TRGSELR::TRGSEL_1,
            2 => TRGSELR::TRGSEL_2,
            3 => TRGSELR::TRGSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL_0`"]
    #[inline]
    pub fn is_trgsel_0(&self) -> bool {
        *self == TRGSELR::TRGSEL_0
    }
    #[doc = "Checks if the value of the field is `TRGSEL_1`"]
    #[inline]
    pub fn is_trgsel_1(&self) -> bool {
        *self == TRGSELR::TRGSEL_1
    }
    #[doc = "Checks if the value of the field is `TRGSEL_2`"]
    #[inline]
    pub fn is_trgsel_2(&self) -> bool {
        *self == TRGSELR::TRGSEL_2
    }
    #[doc = "Checks if the value of the field is `TRGSEL_3`"]
    #[inline]
    pub fn is_trgsel_3(&self) -> bool {
        *self == TRGSELR::TRGSEL_3
    }
}
#[doc = "Values that can be written to the field `TRGSEL`"]
pub enum TRGSELW {
    #[doc = "Input trigger is disabled."]
    TRGSEL_0,
    #[doc = "Input trigger is used instead of RXD pin input."]
    TRGSEL_1,
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    TRGSEL_2,
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    TRGSEL_3,
}
impl TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGSELW::TRGSEL_0 => 0,
            TRGSELW::TRGSEL_1 => 1,
            TRGSELW::TRGSEL_2 => 2,
            TRGSELW::TRGSEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input trigger is disabled."]
    #[inline]
    pub fn trgsel_0(self) -> &'a mut W {
        self.variant(TRGSELW::TRGSEL_0)
    }
    #[doc = "Input trigger is used instead of RXD pin input."]
    #[inline]
    pub fn trgsel_1(self) -> &'a mut W {
        self.variant(TRGSELW::TRGSEL_1)
    }
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    #[inline]
    pub fn trgsel_2(self) -> &'a mut W {
        self.variant(TRGSELW::TRGSEL_2)
    }
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    #[inline]
    pub fn trgsel_3(self) -> &'a mut W {
        self.variant(TRGSELW::TRGSEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        TRGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
}
