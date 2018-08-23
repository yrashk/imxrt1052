#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CBCMR {
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
#[doc = "Possible values of the field `LPSPI_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI_CLK_SELR {
    #[doc = "derive clock from PLL3 PFD1 clk"]
    LPSPI_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD0"]
    LPSPI_CLK_SEL_1,
    #[doc = "derive clock from PLL2"]
    LPSPI_CLK_SEL_2,
    #[doc = "derive clock from PLL2 PFD2"]
    LPSPI_CLK_SEL_3,
}
impl LPSPI_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPSPI_CLK_SELR::LPSPI_CLK_SEL_0 => 0,
            LPSPI_CLK_SELR::LPSPI_CLK_SEL_1 => 1,
            LPSPI_CLK_SELR::LPSPI_CLK_SEL_2 => 2,
            LPSPI_CLK_SELR::LPSPI_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPSPI_CLK_SELR {
        match value {
            0 => LPSPI_CLK_SELR::LPSPI_CLK_SEL_0,
            1 => LPSPI_CLK_SELR::LPSPI_CLK_SEL_1,
            2 => LPSPI_CLK_SELR::LPSPI_CLK_SEL_2,
            3 => LPSPI_CLK_SELR::LPSPI_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_0`"]
    #[inline]
    pub fn is_lpspi_clk_sel_0(&self) -> bool {
        *self == LPSPI_CLK_SELR::LPSPI_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_1`"]
    #[inline]
    pub fn is_lpspi_clk_sel_1(&self) -> bool {
        *self == LPSPI_CLK_SELR::LPSPI_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_2`"]
    #[inline]
    pub fn is_lpspi_clk_sel_2(&self) -> bool {
        *self == LPSPI_CLK_SELR::LPSPI_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_3`"]
    #[inline]
    pub fn is_lpspi_clk_sel_3(&self) -> bool {
        *self == LPSPI_CLK_SELR::LPSPI_CLK_SEL_3
    }
}
#[doc = "Possible values of the field `PERIPH_CLK2_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK2_SELR {
    #[doc = "derive clock from pll3_sw_clk"]
    PERIPH_CLK2_SEL_0,
    #[doc = "derive clock from osc_clk (pll1_ref_clk)"]
    PERIPH_CLK2_SEL_1,
    #[doc = "derive clock from pll2_bypass_clk"]
    PERIPH_CLK2_SEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PERIPH_CLK2_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_0 => 0,
            PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_1 => 1,
            PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_2 => 2,
            PERIPH_CLK2_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERIPH_CLK2_SELR {
        match value {
            0 => PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_0,
            1 => PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_1,
            2 => PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_2,
            i => PERIPH_CLK2_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_0`"]
    #[inline]
    pub fn is_periph_clk2_sel_0(&self) -> bool {
        *self == PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_1`"]
    #[inline]
    pub fn is_periph_clk2_sel_1(&self) -> bool {
        *self == PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_1
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_2`"]
    #[inline]
    pub fn is_periph_clk2_sel_2(&self) -> bool {
        *self == PERIPH_CLK2_SELR::PERIPH_CLK2_SEL_2
    }
}
#[doc = "Possible values of the field `TRACE_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACE_CLK_SELR {
    #[doc = "derive clock from PLL2"]
    TRACE_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD2"]
    TRACE_CLK_SEL_1,
    #[doc = "derive clock from PLL2 PFD0"]
    TRACE_CLK_SEL_2,
    #[doc = "derive clock from PLL2 PFD1"]
    TRACE_CLK_SEL_3,
}
impl TRACE_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRACE_CLK_SELR::TRACE_CLK_SEL_0 => 0,
            TRACE_CLK_SELR::TRACE_CLK_SEL_1 => 1,
            TRACE_CLK_SELR::TRACE_CLK_SEL_2 => 2,
            TRACE_CLK_SELR::TRACE_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRACE_CLK_SELR {
        match value {
            0 => TRACE_CLK_SELR::TRACE_CLK_SEL_0,
            1 => TRACE_CLK_SELR::TRACE_CLK_SEL_1,
            2 => TRACE_CLK_SELR::TRACE_CLK_SEL_2,
            3 => TRACE_CLK_SELR::TRACE_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_0`"]
    #[inline]
    pub fn is_trace_clk_sel_0(&self) -> bool {
        *self == TRACE_CLK_SELR::TRACE_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_1`"]
    #[inline]
    pub fn is_trace_clk_sel_1(&self) -> bool {
        *self == TRACE_CLK_SELR::TRACE_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_2`"]
    #[inline]
    pub fn is_trace_clk_sel_2(&self) -> bool {
        *self == TRACE_CLK_SELR::TRACE_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_3`"]
    #[inline]
    pub fn is_trace_clk_sel_3(&self) -> bool {
        *self == TRACE_CLK_SELR::TRACE_CLK_SEL_3
    }
}
#[doc = "Possible values of the field `PRE_PERIPH_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_PERIPH_CLK_SELR {
    #[doc = "derive clock from PLL2"]
    PRE_PERIPH_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD2"]
    PRE_PERIPH_CLK_SEL_1,
    #[doc = "derive clock from PLL2 PFD0"]
    PRE_PERIPH_CLK_SEL_2,
    #[doc = "derive clock from divided PLL1"]
    PRE_PERIPH_CLK_SEL_3,
}
impl PRE_PERIPH_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_0 => 0,
            PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_1 => 1,
            PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_2 => 2,
            PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRE_PERIPH_CLK_SELR {
        match value {
            0 => PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_0,
            1 => PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_1,
            2 => PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_2,
            3 => PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_0`"]
    #[inline]
    pub fn is_pre_periph_clk_sel_0(&self) -> bool {
        *self == PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_1`"]
    #[inline]
    pub fn is_pre_periph_clk_sel_1(&self) -> bool {
        *self == PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_2`"]
    #[inline]
    pub fn is_pre_periph_clk_sel_2(&self) -> bool {
        *self == PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_3`"]
    #[inline]
    pub fn is_pre_periph_clk_sel_3(&self) -> bool {
        *self == PRE_PERIPH_CLK_SELR::PRE_PERIPH_CLK_SEL_3
    }
}
#[doc = "Possible values of the field `LCDIF_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDIF_PODFR {
    #[doc = "divide by 1"]
    LCDIF_PODF_0,
    #[doc = "divide by 2"]
    LCDIF_PODF_1,
    #[doc = "divide by 3"]
    LCDIF_PODF_2,
    #[doc = "divide by 4"]
    LCDIF_PODF_3,
    #[doc = "divide by 5"]
    LCDIF_PODF_4,
    #[doc = "divide by 6"]
    LCDIF_PODF_5,
    #[doc = "divide by 7"]
    LCDIF_PODF_6,
    #[doc = "divide by 8"]
    LCDIF_PODF_7,
}
impl LCDIF_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCDIF_PODFR::LCDIF_PODF_0 => 0,
            LCDIF_PODFR::LCDIF_PODF_1 => 1,
            LCDIF_PODFR::LCDIF_PODF_2 => 2,
            LCDIF_PODFR::LCDIF_PODF_3 => 3,
            LCDIF_PODFR::LCDIF_PODF_4 => 4,
            LCDIF_PODFR::LCDIF_PODF_5 => 5,
            LCDIF_PODFR::LCDIF_PODF_6 => 6,
            LCDIF_PODFR::LCDIF_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCDIF_PODFR {
        match value {
            0 => LCDIF_PODFR::LCDIF_PODF_0,
            1 => LCDIF_PODFR::LCDIF_PODF_1,
            2 => LCDIF_PODFR::LCDIF_PODF_2,
            3 => LCDIF_PODFR::LCDIF_PODF_3,
            4 => LCDIF_PODFR::LCDIF_PODF_4,
            5 => LCDIF_PODFR::LCDIF_PODF_5,
            6 => LCDIF_PODFR::LCDIF_PODF_6,
            7 => LCDIF_PODFR::LCDIF_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_0`"]
    #[inline]
    pub fn is_lcdif_podf_0(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_0
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_1`"]
    #[inline]
    pub fn is_lcdif_podf_1(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_1
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_2`"]
    #[inline]
    pub fn is_lcdif_podf_2(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_2
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_3`"]
    #[inline]
    pub fn is_lcdif_podf_3(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_3
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_4`"]
    #[inline]
    pub fn is_lcdif_podf_4(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_4
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_5`"]
    #[inline]
    pub fn is_lcdif_podf_5(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_5
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_6`"]
    #[inline]
    pub fn is_lcdif_podf_6(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_6
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_7`"]
    #[inline]
    pub fn is_lcdif_podf_7(&self) -> bool {
        *self == LCDIF_PODFR::LCDIF_PODF_7
    }
}
#[doc = "Possible values of the field `LPSPI_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI_PODFR {
    #[doc = "divide by 1"]
    LPSPI_PODF_0,
    #[doc = "divide by 2"]
    LPSPI_PODF_1,
    #[doc = "divide by 3"]
    LPSPI_PODF_2,
    #[doc = "divide by 4"]
    LPSPI_PODF_3,
    #[doc = "divide by 5"]
    LPSPI_PODF_4,
    #[doc = "divide by 6"]
    LPSPI_PODF_5,
    #[doc = "divide by 7"]
    LPSPI_PODF_6,
    #[doc = "divide by 8"]
    LPSPI_PODF_7,
}
impl LPSPI_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPSPI_PODFR::LPSPI_PODF_0 => 0,
            LPSPI_PODFR::LPSPI_PODF_1 => 1,
            LPSPI_PODFR::LPSPI_PODF_2 => 2,
            LPSPI_PODFR::LPSPI_PODF_3 => 3,
            LPSPI_PODFR::LPSPI_PODF_4 => 4,
            LPSPI_PODFR::LPSPI_PODF_5 => 5,
            LPSPI_PODFR::LPSPI_PODF_6 => 6,
            LPSPI_PODFR::LPSPI_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPSPI_PODFR {
        match value {
            0 => LPSPI_PODFR::LPSPI_PODF_0,
            1 => LPSPI_PODFR::LPSPI_PODF_1,
            2 => LPSPI_PODFR::LPSPI_PODF_2,
            3 => LPSPI_PODFR::LPSPI_PODF_3,
            4 => LPSPI_PODFR::LPSPI_PODF_4,
            5 => LPSPI_PODFR::LPSPI_PODF_5,
            6 => LPSPI_PODFR::LPSPI_PODF_6,
            7 => LPSPI_PODFR::LPSPI_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_0`"]
    #[inline]
    pub fn is_lpspi_podf_0(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_0
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_1`"]
    #[inline]
    pub fn is_lpspi_podf_1(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_1
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_2`"]
    #[inline]
    pub fn is_lpspi_podf_2(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_2
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_3`"]
    #[inline]
    pub fn is_lpspi_podf_3(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_3
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_4`"]
    #[inline]
    pub fn is_lpspi_podf_4(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_4
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_5`"]
    #[inline]
    pub fn is_lpspi_podf_5(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_5
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_6`"]
    #[inline]
    pub fn is_lpspi_podf_6(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_6
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_7`"]
    #[inline]
    pub fn is_lpspi_podf_7(&self) -> bool {
        *self == LPSPI_PODFR::LPSPI_PODF_7
    }
}
#[doc = "Values that can be written to the field `LPSPI_CLK_SEL`"]
pub enum LPSPI_CLK_SELW {
    #[doc = "derive clock from PLL3 PFD1 clk"]
    LPSPI_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD0"]
    LPSPI_CLK_SEL_1,
    #[doc = "derive clock from PLL2"]
    LPSPI_CLK_SEL_2,
    #[doc = "derive clock from PLL2 PFD2"]
    LPSPI_CLK_SEL_3,
}
impl LPSPI_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPSPI_CLK_SELW::LPSPI_CLK_SEL_0 => 0,
            LPSPI_CLK_SELW::LPSPI_CLK_SEL_1 => 1,
            LPSPI_CLK_SELW::LPSPI_CLK_SEL_2 => 2,
            LPSPI_CLK_SELW::LPSPI_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from PLL3 PFD1 clk"]
    #[inline]
    pub fn lpspi_clk_sel_0(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SELW::LPSPI_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD0"]
    #[inline]
    pub fn lpspi_clk_sel_1(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SELW::LPSPI_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2"]
    #[inline]
    pub fn lpspi_clk_sel_2(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SELW::LPSPI_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline]
    pub fn lpspi_clk_sel_3(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SELW::LPSPI_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PERIPH_CLK2_SEL`"]
pub enum PERIPH_CLK2_SELW {
    #[doc = "derive clock from pll3_sw_clk"]
    PERIPH_CLK2_SEL_0,
    #[doc = "derive clock from osc_clk (pll1_ref_clk)"]
    PERIPH_CLK2_SEL_1,
    #[doc = "derive clock from pll2_bypass_clk"]
    PERIPH_CLK2_SEL_2,
}
impl PERIPH_CLK2_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERIPH_CLK2_SELW::PERIPH_CLK2_SEL_0 => 0,
            PERIPH_CLK2_SELW::PERIPH_CLK2_SEL_1 => 1,
            PERIPH_CLK2_SELW::PERIPH_CLK2_SEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERIPH_CLK2_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPH_CLK2_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERIPH_CLK2_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline]
    pub fn periph_clk2_sel_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SELW::PERIPH_CLK2_SEL_0)
    }
    #[doc = "derive clock from osc_clk (pll1_ref_clk)"]
    #[inline]
    pub fn periph_clk2_sel_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SELW::PERIPH_CLK2_SEL_1)
    }
    #[doc = "derive clock from pll2_bypass_clk"]
    #[inline]
    pub fn periph_clk2_sel_2(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SELW::PERIPH_CLK2_SEL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRACE_CLK_SEL`"]
pub enum TRACE_CLK_SELW {
    #[doc = "derive clock from PLL2"]
    TRACE_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD2"]
    TRACE_CLK_SEL_1,
    #[doc = "derive clock from PLL2 PFD0"]
    TRACE_CLK_SEL_2,
    #[doc = "derive clock from PLL2 PFD1"]
    TRACE_CLK_SEL_3,
}
impl TRACE_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRACE_CLK_SELW::TRACE_CLK_SEL_0 => 0,
            TRACE_CLK_SELW::TRACE_CLK_SEL_1 => 1,
            TRACE_CLK_SELW::TRACE_CLK_SEL_2 => 2,
            TRACE_CLK_SELW::TRACE_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACE_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACE_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACE_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from PLL2"]
    #[inline]
    pub fn trace_clk_sel_0(self) -> &'a mut W {
        self.variant(TRACE_CLK_SELW::TRACE_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline]
    pub fn trace_clk_sel_1(self) -> &'a mut W {
        self.variant(TRACE_CLK_SELW::TRACE_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline]
    pub fn trace_clk_sel_2(self) -> &'a mut W {
        self.variant(TRACE_CLK_SELW::TRACE_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 PFD1"]
    #[inline]
    pub fn trace_clk_sel_3(self) -> &'a mut W {
        self.variant(TRACE_CLK_SELW::TRACE_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRE_PERIPH_CLK_SEL`"]
pub enum PRE_PERIPH_CLK_SELW {
    #[doc = "derive clock from PLL2"]
    PRE_PERIPH_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD2"]
    PRE_PERIPH_CLK_SEL_1,
    #[doc = "derive clock from PLL2 PFD0"]
    PRE_PERIPH_CLK_SEL_2,
    #[doc = "derive clock from divided PLL1"]
    PRE_PERIPH_CLK_SEL_3,
}
impl PRE_PERIPH_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_0 => 0,
            PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_1 => 1,
            PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_2 => 2,
            PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRE_PERIPH_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_PERIPH_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRE_PERIPH_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from PLL2"]
    #[inline]
    pub fn pre_periph_clk_sel_0(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline]
    pub fn pre_periph_clk_sel_1(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline]
    pub fn pre_periph_clk_sel_2(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_2)
    }
    #[doc = "derive clock from divided PLL1"]
    #[inline]
    pub fn pre_periph_clk_sel_3(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SELW::PRE_PERIPH_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCDIF_PODF`"]
pub enum LCDIF_PODFW {
    #[doc = "divide by 1"]
    LCDIF_PODF_0,
    #[doc = "divide by 2"]
    LCDIF_PODF_1,
    #[doc = "divide by 3"]
    LCDIF_PODF_2,
    #[doc = "divide by 4"]
    LCDIF_PODF_3,
    #[doc = "divide by 5"]
    LCDIF_PODF_4,
    #[doc = "divide by 6"]
    LCDIF_PODF_5,
    #[doc = "divide by 7"]
    LCDIF_PODF_6,
    #[doc = "divide by 8"]
    LCDIF_PODF_7,
}
impl LCDIF_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCDIF_PODFW::LCDIF_PODF_0 => 0,
            LCDIF_PODFW::LCDIF_PODF_1 => 1,
            LCDIF_PODFW::LCDIF_PODF_2 => 2,
            LCDIF_PODFW::LCDIF_PODF_3 => 3,
            LCDIF_PODFW::LCDIF_PODF_4 => 4,
            LCDIF_PODFW::LCDIF_PODF_5 => 5,
            LCDIF_PODFW::LCDIF_PODF_6 => 6,
            LCDIF_PODFW::LCDIF_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCDIF_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDIF_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCDIF_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn lcdif_podf_0(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn lcdif_podf_1(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn lcdif_podf_2(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn lcdif_podf_3(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn lcdif_podf_4(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn lcdif_podf_5(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn lcdif_podf_6(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn lcdif_podf_7(self) -> &'a mut W {
        self.variant(LCDIF_PODFW::LCDIF_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSPI_PODF`"]
pub enum LPSPI_PODFW {
    #[doc = "divide by 1"]
    LPSPI_PODF_0,
    #[doc = "divide by 2"]
    LPSPI_PODF_1,
    #[doc = "divide by 3"]
    LPSPI_PODF_2,
    #[doc = "divide by 4"]
    LPSPI_PODF_3,
    #[doc = "divide by 5"]
    LPSPI_PODF_4,
    #[doc = "divide by 6"]
    LPSPI_PODF_5,
    #[doc = "divide by 7"]
    LPSPI_PODF_6,
    #[doc = "divide by 8"]
    LPSPI_PODF_7,
}
impl LPSPI_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPSPI_PODFW::LPSPI_PODF_0 => 0,
            LPSPI_PODFW::LPSPI_PODF_1 => 1,
            LPSPI_PODFW::LPSPI_PODF_2 => 2,
            LPSPI_PODFW::LPSPI_PODF_3 => 3,
            LPSPI_PODFW::LPSPI_PODF_4 => 4,
            LPSPI_PODFW::LPSPI_PODF_5 => 5,
            LPSPI_PODFW::LPSPI_PODF_6 => 6,
            LPSPI_PODFW::LPSPI_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPI_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPI_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPI_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn lpspi_podf_0(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn lpspi_podf_1(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn lpspi_podf_2(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn lpspi_podf_3(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn lpspi_podf_4(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn lpspi_podf_5(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn lpspi_podf_6(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn lpspi_podf_7(self) -> &'a mut W {
        self.variant(LPSPI_PODFW::LPSPI_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 4:5 - Selector for lpspi clock multiplexer"]
    #[inline]
    pub fn lpspi_clk_sel(&self) -> LPSPI_CLK_SELR {
        LPSPI_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Selector for peripheral clk2 clock multiplexer"]
    #[inline]
    pub fn periph_clk2_sel(&self) -> PERIPH_CLK2_SELR {
        PERIPH_CLK2_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Selector for Trace clock multiplexer"]
    #[inline]
    pub fn trace_clk_sel(&self) -> TRACE_CLK_SELR {
        TRACE_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Selector for pre_periph clock multiplexer"]
    #[inline]
    pub fn pre_periph_clk_sel(&self) -> PRE_PERIPH_CLK_SELR {
        PRE_PERIPH_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:25 - Post-divider for LCDIF clock."]
    #[inline]
    pub fn lcdif_podf(&self) -> LCDIF_PODFR {
        LCDIF_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:28 - Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn lpspi_podf(&self) -> LPSPI_PODFR {
        LPSPI_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 765625124 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:5 - Selector for lpspi clock multiplexer"]
    #[inline]
    pub fn lpspi_clk_sel(&mut self) -> _LPSPI_CLK_SELW {
        _LPSPI_CLK_SELW { w: self }
    }
    #[doc = "Bits 12:13 - Selector for peripheral clk2 clock multiplexer"]
    #[inline]
    pub fn periph_clk2_sel(&mut self) -> _PERIPH_CLK2_SELW {
        _PERIPH_CLK2_SELW { w: self }
    }
    #[doc = "Bits 14:15 - Selector for Trace clock multiplexer"]
    #[inline]
    pub fn trace_clk_sel(&mut self) -> _TRACE_CLK_SELW {
        _TRACE_CLK_SELW { w: self }
    }
    #[doc = "Bits 18:19 - Selector for pre_periph clock multiplexer"]
    #[inline]
    pub fn pre_periph_clk_sel(&mut self) -> _PRE_PERIPH_CLK_SELW {
        _PRE_PERIPH_CLK_SELW { w: self }
    }
    #[doc = "Bits 23:25 - Post-divider for LCDIF clock."]
    #[inline]
    pub fn lcdif_podf(&mut self) -> _LCDIF_PODFW {
        _LCDIF_PODFW { w: self }
    }
    #[doc = "Bits 26:28 - Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn lpspi_podf(&mut self) -> _LPSPI_PODFW {
        _LPSPI_PODFW { w: self }
    }
}
