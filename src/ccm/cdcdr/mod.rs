#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CDCDR {
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
#[doc = "Possible values of the field `FLEXIO1_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_CLK_SELR {
    #[doc = "derive clock from PLL4"]
    FLEXIO1_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD2"]
    FLEXIO1_CLK_SEL_1,
    #[doc = "derive clock from PLL5"]
    FLEXIO1_CLK_SEL_2,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO1_CLK_SEL_3,
}
impl FLEXIO1_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_0 => 0,
            FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_1 => 1,
            FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_2 => 2,
            FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXIO1_CLK_SELR {
        match value {
            0 => FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_0,
            1 => FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_1,
            2 => FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_2,
            3 => FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_0`"]
    #[inline]
    pub fn is_flexio1_clk_sel_0(&self) -> bool {
        *self == FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_1`"]
    #[inline]
    pub fn is_flexio1_clk_sel_1(&self) -> bool {
        *self == FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_2`"]
    #[inline]
    pub fn is_flexio1_clk_sel_2(&self) -> bool {
        *self == FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_3`"]
    #[inline]
    pub fn is_flexio1_clk_sel_3(&self) -> bool {
        *self == FLEXIO1_CLK_SELR::FLEXIO1_CLK_SEL_3
    }
}
#[doc = "Possible values of the field `FLEXIO1_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_CLK_PODFR {
    #[doc = "divide by 1"]
    FLEXIO1_CLK_PODF_0,
    #[doc = "divide by 8"]
    FLEXIO1_CLK_PODF_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLEXIO1_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXIO1_CLK_PODFR::FLEXIO1_CLK_PODF_0 => 0,
            FLEXIO1_CLK_PODFR::FLEXIO1_CLK_PODF_7 => 7,
            FLEXIO1_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXIO1_CLK_PODFR {
        match value {
            0 => FLEXIO1_CLK_PODFR::FLEXIO1_CLK_PODF_0,
            7 => FLEXIO1_CLK_PODFR::FLEXIO1_CLK_PODF_7,
            i => FLEXIO1_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_PODF_0`"]
    #[inline]
    pub fn is_flexio1_clk_podf_0(&self) -> bool {
        *self == FLEXIO1_CLK_PODFR::FLEXIO1_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_PODF_7`"]
    #[inline]
    pub fn is_flexio1_clk_podf_7(&self) -> bool {
        *self == FLEXIO1_CLK_PODFR::FLEXIO1_CLK_PODF_7
    }
}
#[doc = "Possible values of the field `FLEXIO1_CLK_PRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_CLK_PREDR {
    #[doc = "divide by 1 (do not use with high input frequencies)"]
    FLEXIO1_CLK_PRED_0,
    #[doc = "divide by 2"]
    FLEXIO1_CLK_PRED_1,
    #[doc = "divide by 3"]
    FLEXIO1_CLK_PRED_2,
    #[doc = "divide by 8"]
    FLEXIO1_CLK_PRED_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLEXIO1_CLK_PREDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_0 => 0,
            FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_1 => 1,
            FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_2 => 2,
            FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_7 => 7,
            FLEXIO1_CLK_PREDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXIO1_CLK_PREDR {
        match value {
            0 => FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_0,
            1 => FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_1,
            2 => FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_2,
            7 => FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_7,
            i => FLEXIO1_CLK_PREDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_PRED_0`"]
    #[inline]
    pub fn is_flexio1_clk_pred_0(&self) -> bool {
        *self == FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_PRED_1`"]
    #[inline]
    pub fn is_flexio1_clk_pred_1(&self) -> bool {
        *self == FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_1
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_PRED_2`"]
    #[inline]
    pub fn is_flexio1_clk_pred_2(&self) -> bool {
        *self == FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_2
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_PRED_7`"]
    #[inline]
    pub fn is_flexio1_clk_pred_7(&self) -> bool {
        *self == FLEXIO1_CLK_PREDR::FLEXIO1_CLK_PRED_7
    }
}
#[doc = "Possible values of the field `SPDIF0_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF0_CLK_SELR {
    #[doc = "derive clock from PLL4"]
    SPDIF0_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD2"]
    SPDIF0_CLK_SEL_1,
    #[doc = "derive clock from PLL5"]
    SPDIF0_CLK_SEL_2,
    #[doc = "derive clock from pll3_sw_clk"]
    SPDIF0_CLK_SEL_3,
}
impl SPDIF0_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_0 => 0,
            SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_1 => 1,
            SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_2 => 2,
            SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPDIF0_CLK_SELR {
        match value {
            0 => SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_0,
            1 => SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_1,
            2 => SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_2,
            3 => SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_0`"]
    #[inline]
    pub fn is_spdif0_clk_sel_0(&self) -> bool {
        *self == SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_1`"]
    #[inline]
    pub fn is_spdif0_clk_sel_1(&self) -> bool {
        *self == SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_2`"]
    #[inline]
    pub fn is_spdif0_clk_sel_2(&self) -> bool {
        *self == SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_3`"]
    #[inline]
    pub fn is_spdif0_clk_sel_3(&self) -> bool {
        *self == SPDIF0_CLK_SELR::SPDIF0_CLK_SEL_3
    }
}
#[doc = "Possible values of the field `SPDIF0_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF0_CLK_PODFR {
    #[doc = "divide by 1"]
    SPDIF0_CLK_PODF_0,
    #[doc = "divide by 8"]
    SPDIF0_CLK_PODF_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPDIF0_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPDIF0_CLK_PODFR::SPDIF0_CLK_PODF_0 => 0,
            SPDIF0_CLK_PODFR::SPDIF0_CLK_PODF_7 => 7,
            SPDIF0_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPDIF0_CLK_PODFR {
        match value {
            0 => SPDIF0_CLK_PODFR::SPDIF0_CLK_PODF_0,
            7 => SPDIF0_CLK_PODFR::SPDIF0_CLK_PODF_7,
            i => SPDIF0_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_PODF_0`"]
    #[inline]
    pub fn is_spdif0_clk_podf_0(&self) -> bool {
        *self == SPDIF0_CLK_PODFR::SPDIF0_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_PODF_7`"]
    #[inline]
    pub fn is_spdif0_clk_podf_7(&self) -> bool {
        *self == SPDIF0_CLK_PODFR::SPDIF0_CLK_PODF_7
    }
}
#[doc = "Possible values of the field `SPDIF0_CLK_PRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF0_CLK_PREDR {
    #[doc = "divide by 1 (do not use with high input frequencies)"]
    SPDIF0_CLK_PRED_0,
    #[doc = "divide by 2"]
    SPDIF0_CLK_PRED_1,
    #[doc = "divide by 3"]
    SPDIF0_CLK_PRED_2,
    #[doc = "divide by 8"]
    SPDIF0_CLK_PRED_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPDIF0_CLK_PREDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_0 => 0,
            SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_1 => 1,
            SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_2 => 2,
            SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_7 => 7,
            SPDIF0_CLK_PREDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPDIF0_CLK_PREDR {
        match value {
            0 => SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_0,
            1 => SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_1,
            2 => SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_2,
            7 => SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_7,
            i => SPDIF0_CLK_PREDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_PRED_0`"]
    #[inline]
    pub fn is_spdif0_clk_pred_0(&self) -> bool {
        *self == SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_0
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_PRED_1`"]
    #[inline]
    pub fn is_spdif0_clk_pred_1(&self) -> bool {
        *self == SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_1
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_PRED_2`"]
    #[inline]
    pub fn is_spdif0_clk_pred_2(&self) -> bool {
        *self == SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_2
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_PRED_7`"]
    #[inline]
    pub fn is_spdif0_clk_pred_7(&self) -> bool {
        *self == SPDIF0_CLK_PREDR::SPDIF0_CLK_PRED_7
    }
}
#[doc = "Values that can be written to the field `FLEXIO1_CLK_SEL`"]
pub enum FLEXIO1_CLK_SELW {
    #[doc = "derive clock from PLL4"]
    FLEXIO1_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD2"]
    FLEXIO1_CLK_SEL_1,
    #[doc = "derive clock from PLL5"]
    FLEXIO1_CLK_SEL_2,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO1_CLK_SEL_3,
}
impl FLEXIO1_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_0 => 0,
            FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_1 => 1,
            FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_2 => 2,
            FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO1_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO1_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO1_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from PLL4"]
    #[inline]
    pub fn flexio1_clk_sel_0(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline]
    pub fn flexio1_clk_sel_1(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL5"]
    #[inline]
    pub fn flexio1_clk_sel_2(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_2)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline]
    pub fn flexio1_clk_sel_3(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SELW::FLEXIO1_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLEXIO1_CLK_PODF`"]
pub enum FLEXIO1_CLK_PODFW {
    #[doc = "divide by 1"]
    FLEXIO1_CLK_PODF_0,
    #[doc = "divide by 8"]
    FLEXIO1_CLK_PODF_7,
}
impl FLEXIO1_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXIO1_CLK_PODFW::FLEXIO1_CLK_PODF_0 => 0,
            FLEXIO1_CLK_PODFW::FLEXIO1_CLK_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO1_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO1_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO1_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn flexio1_clk_podf_0(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODFW::FLEXIO1_CLK_PODF_0)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn flexio1_clk_podf_7(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODFW::FLEXIO1_CLK_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLEXIO1_CLK_PRED`"]
pub enum FLEXIO1_CLK_PREDW {
    #[doc = "divide by 1 (do not use with high input frequencies)"]
    FLEXIO1_CLK_PRED_0,
    #[doc = "divide by 2"]
    FLEXIO1_CLK_PRED_1,
    #[doc = "divide by 3"]
    FLEXIO1_CLK_PRED_2,
    #[doc = "divide by 8"]
    FLEXIO1_CLK_PRED_7,
}
impl FLEXIO1_CLK_PREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_0 => 0,
            FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_1 => 1,
            FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_2 => 2,
            FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO1_CLK_PREDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO1_CLK_PREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO1_CLK_PREDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1 (do not use with high input frequencies)"]
    #[inline]
    pub fn flexio1_clk_pred_0(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn flexio1_clk_pred_1(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn flexio1_clk_pred_2(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_2)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn flexio1_clk_pred_7(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PREDW::FLEXIO1_CLK_PRED_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDIF0_CLK_SEL`"]
pub enum SPDIF0_CLK_SELW {
    #[doc = "derive clock from PLL4"]
    SPDIF0_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD2"]
    SPDIF0_CLK_SEL_1,
    #[doc = "derive clock from PLL5"]
    SPDIF0_CLK_SEL_2,
    #[doc = "derive clock from pll3_sw_clk"]
    SPDIF0_CLK_SEL_3,
}
impl SPDIF0_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_0 => 0,
            SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_1 => 1,
            SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_2 => 2,
            SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDIF0_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDIF0_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDIF0_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from PLL4"]
    #[inline]
    pub fn spdif0_clk_sel_0(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline]
    pub fn spdif0_clk_sel_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL5"]
    #[inline]
    pub fn spdif0_clk_sel_2(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_2)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline]
    pub fn spdif0_clk_sel_3(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SELW::SPDIF0_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDIF0_CLK_PODF`"]
pub enum SPDIF0_CLK_PODFW {
    #[doc = "divide by 1"]
    SPDIF0_CLK_PODF_0,
    #[doc = "divide by 8"]
    SPDIF0_CLK_PODF_7,
}
impl SPDIF0_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPDIF0_CLK_PODFW::SPDIF0_CLK_PODF_0 => 0,
            SPDIF0_CLK_PODFW::SPDIF0_CLK_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDIF0_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDIF0_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDIF0_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn spdif0_clk_podf_0(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODFW::SPDIF0_CLK_PODF_0)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn spdif0_clk_podf_7(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODFW::SPDIF0_CLK_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDIF0_CLK_PRED`"]
pub enum SPDIF0_CLK_PREDW {
    #[doc = "divide by 1 (do not use with high input frequencies)"]
    SPDIF0_CLK_PRED_0,
    #[doc = "divide by 2"]
    SPDIF0_CLK_PRED_1,
    #[doc = "divide by 3"]
    SPDIF0_CLK_PRED_2,
    #[doc = "divide by 8"]
    SPDIF0_CLK_PRED_7,
}
impl SPDIF0_CLK_PREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_0 => 0,
            SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_1 => 1,
            SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_2 => 2,
            SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDIF0_CLK_PREDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDIF0_CLK_PREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDIF0_CLK_PREDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1 (do not use with high input frequencies)"]
    #[inline]
    pub fn spdif0_clk_pred_0(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn spdif0_clk_pred_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn spdif0_clk_pred_2(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_2)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn spdif0_clk_pred_7(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PREDW::SPDIF0_CLK_PRED_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
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
    #[doc = "Bits 7:8 - Selector for flexio1 clock multiplexer"]
    #[inline]
    pub fn flexio1_clk_sel(&self) -> FLEXIO1_CLK_SELR {
        FLEXIO1_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - Divider for flexio1 clock podf. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn flexio1_clk_podf(&self) -> FLEXIO1_CLK_PODFR {
        FLEXIO1_CLK_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Divider for flexio1 clock pred. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn flexio1_clk_pred(&self) -> FLEXIO1_CLK_PREDR {
        FLEXIO1_CLK_PREDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Selector for spdif0 clock multiplexer"]
    #[inline]
    pub fn spdif0_clk_sel(&self) -> SPDIF0_CLK_SELR {
        SPDIF0_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:24 - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn spdif0_clk_podf(&self) -> SPDIF0_CLK_PODFR {
        SPDIF0_CLK_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:27 - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn spdif0_clk_pred(&self) -> SPDIF0_CLK_PREDR {
        SPDIF0_CLK_PREDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 871833490 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 7:8 - Selector for flexio1 clock multiplexer"]
    #[inline]
    pub fn flexio1_clk_sel(&mut self) -> _FLEXIO1_CLK_SELW {
        _FLEXIO1_CLK_SELW { w: self }
    }
    #[doc = "Bits 9:11 - Divider for flexio1 clock podf. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn flexio1_clk_podf(&mut self) -> _FLEXIO1_CLK_PODFW {
        _FLEXIO1_CLK_PODFW { w: self }
    }
    #[doc = "Bits 12:14 - Divider for flexio1 clock pred. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn flexio1_clk_pred(&mut self) -> _FLEXIO1_CLK_PREDW {
        _FLEXIO1_CLK_PREDW { w: self }
    }
    #[doc = "Bits 20:21 - Selector for spdif0 clock multiplexer"]
    #[inline]
    pub fn spdif0_clk_sel(&mut self) -> _SPDIF0_CLK_SELW {
        _SPDIF0_CLK_SELW { w: self }
    }
    #[doc = "Bits 22:24 - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn spdif0_clk_podf(&mut self) -> _SPDIF0_CLK_PODFW {
        _SPDIF0_CLK_PODFW { w: self }
    }
    #[doc = "Bits 25:27 - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn spdif0_clk_pred(&mut self) -> _SPDIF0_CLK_PREDW {
        _SPDIF0_CLK_PREDW { w: self }
    }
}
