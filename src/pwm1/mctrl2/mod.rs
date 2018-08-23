#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::MCTRL2 {
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
#[doc = "Possible values of the field `MONPLL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONPLLR {
    #[doc = "Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    MONPLL_0,
    #[doc = "Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    MONPLL_1,
    #[doc = "Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    MONPLL_2,
    #[doc = "Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    MONPLL_3,
}
impl MONPLLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MONPLLR::MONPLL_0 => 0,
            MONPLLR::MONPLL_1 => 1,
            MONPLLR::MONPLL_2 => 2,
            MONPLLR::MONPLL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MONPLLR {
        match value {
            0 => MONPLLR::MONPLL_0,
            1 => MONPLLR::MONPLL_1,
            2 => MONPLLR::MONPLL_2,
            3 => MONPLLR::MONPLL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MONPLL_0`"]
    #[inline]
    pub fn is_monpll_0(&self) -> bool {
        *self == MONPLLR::MONPLL_0
    }
    #[doc = "Checks if the value of the field is `MONPLL_1`"]
    #[inline]
    pub fn is_monpll_1(&self) -> bool {
        *self == MONPLLR::MONPLL_1
    }
    #[doc = "Checks if the value of the field is `MONPLL_2`"]
    #[inline]
    pub fn is_monpll_2(&self) -> bool {
        *self == MONPLLR::MONPLL_2
    }
    #[doc = "Checks if the value of the field is `MONPLL_3`"]
    #[inline]
    pub fn is_monpll_3(&self) -> bool {
        *self == MONPLLR::MONPLL_3
    }
}
#[doc = "Values that can be written to the field `MONPLL`"]
pub enum MONPLLW {
    #[doc = "Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    MONPLL_0,
    #[doc = "Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    MONPLL_1,
    #[doc = "Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    MONPLL_2,
    #[doc = "Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    MONPLL_3,
}
impl MONPLLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MONPLLW::MONPLL_0 => 0,
            MONPLLW::MONPLL_1 => 1,
            MONPLLW::MONPLL_2 => 2,
            MONPLLW::MONPLL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONPLLW<'a> {
    w: &'a mut W,
}
impl<'a> _MONPLLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONPLLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    #[inline]
    pub fn monpll_0(self) -> &'a mut W {
        self.variant(MONPLLW::MONPLL_0)
    }
    #[doc = "Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    #[inline]
    pub fn monpll_1(self) -> &'a mut W {
        self.variant(MONPLLW::MONPLL_1)
    }
    #[doc = "Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    #[inline]
    pub fn monpll_2(self) -> &'a mut W {
        self.variant(MONPLLW::MONPLL_2)
    }
    #[doc = "Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    #[inline]
    pub fn monpll_3(self) -> &'a mut W {
        self.variant(MONPLLW::MONPLL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Monitor PLL State"]
    #[inline]
    pub fn monpll(&self) -> MONPLLR {
        MONPLLR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Monitor PLL State"]
    #[inline]
    pub fn monpll(&mut self) -> _MONPLLW {
        _MONPLLW { w: self }
    }
}
