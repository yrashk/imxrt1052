#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCSR {
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
#[doc = "Possible values of the field `PLL3_SW_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3_SW_CLK_SELR {
    #[doc = "pll3_main_clk"]
    PLL3_SW_CLK_SEL_0,
    #[doc = "pll3 bypass clock"]
    PLL3_SW_CLK_SEL_1,
}
impl PLL3_SW_CLK_SELR {
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
            PLL3_SW_CLK_SELR::PLL3_SW_CLK_SEL_0 => false,
            PLL3_SW_CLK_SELR::PLL3_SW_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL3_SW_CLK_SELR {
        match value {
            false => PLL3_SW_CLK_SELR::PLL3_SW_CLK_SEL_0,
            true => PLL3_SW_CLK_SELR::PLL3_SW_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLL3_SW_CLK_SEL_0`"]
    #[inline]
    pub fn is_pll3_sw_clk_sel_0(&self) -> bool {
        *self == PLL3_SW_CLK_SELR::PLL3_SW_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PLL3_SW_CLK_SEL_1`"]
    #[inline]
    pub fn is_pll3_sw_clk_sel_1(&self) -> bool {
        *self == PLL3_SW_CLK_SELR::PLL3_SW_CLK_SEL_1
    }
}
#[doc = "Values that can be written to the field `PLL3_SW_CLK_SEL`"]
pub enum PLL3_SW_CLK_SELW {
    #[doc = "pll3_main_clk"]
    PLL3_SW_CLK_SEL_0,
    #[doc = "pll3 bypass clock"]
    PLL3_SW_CLK_SEL_1,
}
impl PLL3_SW_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL3_SW_CLK_SELW::PLL3_SW_CLK_SEL_0 => false,
            PLL3_SW_CLK_SELW::PLL3_SW_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL3_SW_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL3_SW_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL3_SW_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "pll3_main_clk"]
    #[inline]
    pub fn pll3_sw_clk_sel_0(self) -> &'a mut W {
        self.variant(PLL3_SW_CLK_SELW::PLL3_SW_CLK_SEL_0)
    }
    #[doc = "pll3 bypass clock"]
    #[inline]
    pub fn pll3_sw_clk_sel_1(self) -> &'a mut W {
        self.variant(PLL3_SW_CLK_SELW::PLL3_SW_CLK_SEL_1)
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
    #[doc = "Bit 0 - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline]
    pub fn pll3_sw_clk_sel(&self) -> PLL3_SW_CLK_SELR {
        PLL3_SW_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline]
    pub fn pll3_sw_clk_sel(&mut self) -> _PLL3_SW_CLK_SELW {
        _PLL3_SW_CLK_SELW { w: self }
    }
}
