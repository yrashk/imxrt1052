#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSCDR3 {
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
#[doc = "Possible values of the field `CSI_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSI_CLK_SELR {
    #[doc = "derive clock from osc_clk (24M)"]
    CSI_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD2"]
    CSI_CLK_SEL_1,
    #[doc = "derive clock from pll3_120M"]
    CSI_CLK_SEL_2,
    #[doc = "derive clock from PLL3 PFD1"]
    CSI_CLK_SEL_3,
}
impl CSI_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSI_CLK_SELR::CSI_CLK_SEL_0 => 0,
            CSI_CLK_SELR::CSI_CLK_SEL_1 => 1,
            CSI_CLK_SELR::CSI_CLK_SEL_2 => 2,
            CSI_CLK_SELR::CSI_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSI_CLK_SELR {
        match value {
            0 => CSI_CLK_SELR::CSI_CLK_SEL_0,
            1 => CSI_CLK_SELR::CSI_CLK_SEL_1,
            2 => CSI_CLK_SELR::CSI_CLK_SEL_2,
            3 => CSI_CLK_SELR::CSI_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_0`"]
    #[inline]
    pub fn is_csi_clk_sel_0(&self) -> bool {
        *self == CSI_CLK_SELR::CSI_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_1`"]
    #[inline]
    pub fn is_csi_clk_sel_1(&self) -> bool {
        *self == CSI_CLK_SELR::CSI_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_2`"]
    #[inline]
    pub fn is_csi_clk_sel_2(&self) -> bool {
        *self == CSI_CLK_SELR::CSI_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_3`"]
    #[inline]
    pub fn is_csi_clk_sel_3(&self) -> bool {
        *self == CSI_CLK_SELR::CSI_CLK_SEL_3
    }
}
#[doc = "Possible values of the field `CSI_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSI_PODFR {
    #[doc = "divide by 1"]
    CSI_PODF_0,
    #[doc = "divide by 2"]
    CSI_PODF_1,
    #[doc = "divide by 3"]
    CSI_PODF_2,
    #[doc = "divide by 4"]
    CSI_PODF_3,
    #[doc = "divide by 5"]
    CSI_PODF_4,
    #[doc = "divide by 6"]
    CSI_PODF_5,
    #[doc = "divide by 7"]
    CSI_PODF_6,
    #[doc = "divide by 8"]
    CSI_PODF_7,
}
impl CSI_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSI_PODFR::CSI_PODF_0 => 0,
            CSI_PODFR::CSI_PODF_1 => 1,
            CSI_PODFR::CSI_PODF_2 => 2,
            CSI_PODFR::CSI_PODF_3 => 3,
            CSI_PODFR::CSI_PODF_4 => 4,
            CSI_PODFR::CSI_PODF_5 => 5,
            CSI_PODFR::CSI_PODF_6 => 6,
            CSI_PODFR::CSI_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSI_PODFR {
        match value {
            0 => CSI_PODFR::CSI_PODF_0,
            1 => CSI_PODFR::CSI_PODF_1,
            2 => CSI_PODFR::CSI_PODF_2,
            3 => CSI_PODFR::CSI_PODF_3,
            4 => CSI_PODFR::CSI_PODF_4,
            5 => CSI_PODFR::CSI_PODF_5,
            6 => CSI_PODFR::CSI_PODF_6,
            7 => CSI_PODFR::CSI_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_0`"]
    #[inline]
    pub fn is_csi_podf_0(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_0
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_1`"]
    #[inline]
    pub fn is_csi_podf_1(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_1
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_2`"]
    #[inline]
    pub fn is_csi_podf_2(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_2
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_3`"]
    #[inline]
    pub fn is_csi_podf_3(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_3
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_4`"]
    #[inline]
    pub fn is_csi_podf_4(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_4
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_5`"]
    #[inline]
    pub fn is_csi_podf_5(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_5
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_6`"]
    #[inline]
    pub fn is_csi_podf_6(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_6
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_7`"]
    #[inline]
    pub fn is_csi_podf_7(&self) -> bool {
        *self == CSI_PODFR::CSI_PODF_7
    }
}
#[doc = "Values that can be written to the field `CSI_CLK_SEL`"]
pub enum CSI_CLK_SELW {
    #[doc = "derive clock from osc_clk (24M)"]
    CSI_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD2"]
    CSI_CLK_SEL_1,
    #[doc = "derive clock from pll3_120M"]
    CSI_CLK_SEL_2,
    #[doc = "derive clock from PLL3 PFD1"]
    CSI_CLK_SEL_3,
}
impl CSI_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSI_CLK_SELW::CSI_CLK_SEL_0 => 0,
            CSI_CLK_SELW::CSI_CLK_SEL_1 => 1,
            CSI_CLK_SELW::CSI_CLK_SEL_2 => 2,
            CSI_CLK_SELW::CSI_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSI_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSI_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from osc_clk (24M)"]
    #[inline]
    pub fn csi_clk_sel_0(self) -> &'a mut W {
        self.variant(CSI_CLK_SELW::CSI_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline]
    pub fn csi_clk_sel_1(self) -> &'a mut W {
        self.variant(CSI_CLK_SELW::CSI_CLK_SEL_1)
    }
    #[doc = "derive clock from pll3_120M"]
    #[inline]
    pub fn csi_clk_sel_2(self) -> &'a mut W {
        self.variant(CSI_CLK_SELW::CSI_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL3 PFD1"]
    #[inline]
    pub fn csi_clk_sel_3(self) -> &'a mut W {
        self.variant(CSI_CLK_SELW::CSI_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSI_PODF`"]
pub enum CSI_PODFW {
    #[doc = "divide by 1"]
    CSI_PODF_0,
    #[doc = "divide by 2"]
    CSI_PODF_1,
    #[doc = "divide by 3"]
    CSI_PODF_2,
    #[doc = "divide by 4"]
    CSI_PODF_3,
    #[doc = "divide by 5"]
    CSI_PODF_4,
    #[doc = "divide by 6"]
    CSI_PODF_5,
    #[doc = "divide by 7"]
    CSI_PODF_6,
    #[doc = "divide by 8"]
    CSI_PODF_7,
}
impl CSI_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSI_PODFW::CSI_PODF_0 => 0,
            CSI_PODFW::CSI_PODF_1 => 1,
            CSI_PODFW::CSI_PODF_2 => 2,
            CSI_PODFW::CSI_PODF_3 => 3,
            CSI_PODFW::CSI_PODF_4 => 4,
            CSI_PODFW::CSI_PODF_5 => 5,
            CSI_PODFW::CSI_PODF_6 => 6,
            CSI_PODFW::CSI_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSI_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSI_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn csi_podf_0(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn csi_podf_1(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn csi_podf_2(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn csi_podf_3(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn csi_podf_4(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn csi_podf_5(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn csi_podf_6(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn csi_podf_7(self) -> &'a mut W {
        self.variant(CSI_PODFW::CSI_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
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
    #[doc = "Bits 9:10 - Selector for csi_mclk multiplexer"]
    #[inline]
    pub fn csi_clk_sel(&self) -> CSI_CLK_SELR {
        CSI_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Post divider for csi_mclk. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn csi_podf(&self) -> CSI_PODFR {
        CSI_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 84033 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 9:10 - Selector for csi_mclk multiplexer"]
    #[inline]
    pub fn csi_clk_sel(&mut self) -> _CSI_CLK_SELW {
        _CSI_CLK_SELW { w: self }
    }
    #[doc = "Bits 11:13 - Post divider for csi_mclk. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn csi_podf(&mut self) -> _CSI_PODFW {
        _CSI_PODFW { w: self }
    }
}
