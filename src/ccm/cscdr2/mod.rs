#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSCDR2 {
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
#[doc = "Possible values of the field `LCDIF_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDIF_CLK_SELR {
    #[doc = "derive clock from divided pre-muxed LCDIF clock"]
    LCDIF_CLK_SEL_0,
    #[doc = "derive clock from ipp_di0_clk"]
    LCDIF_CLK_SEL_1,
    #[doc = "derive clock from ipp_di1_clk"]
    LCDIF_CLK_SEL_2,
    #[doc = "derive clock from ldb_di0_clk"]
    LCDIF_CLK_SEL_3,
    #[doc = "derive clock from ldb_di1_clk"]
    LCDIF_CLK_SEL_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LCDIF_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCDIF_CLK_SELR::LCDIF_CLK_SEL_0 => 0,
            LCDIF_CLK_SELR::LCDIF_CLK_SEL_1 => 1,
            LCDIF_CLK_SELR::LCDIF_CLK_SEL_2 => 2,
            LCDIF_CLK_SELR::LCDIF_CLK_SEL_3 => 3,
            LCDIF_CLK_SELR::LCDIF_CLK_SEL_4 => 4,
            LCDIF_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCDIF_CLK_SELR {
        match value {
            0 => LCDIF_CLK_SELR::LCDIF_CLK_SEL_0,
            1 => LCDIF_CLK_SELR::LCDIF_CLK_SEL_1,
            2 => LCDIF_CLK_SELR::LCDIF_CLK_SEL_2,
            3 => LCDIF_CLK_SELR::LCDIF_CLK_SEL_3,
            4 => LCDIF_CLK_SELR::LCDIF_CLK_SEL_4,
            i => LCDIF_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LCDIF_CLK_SEL_0`"]
    #[inline]
    pub fn is_lcdif_clk_sel_0(&self) -> bool {
        *self == LCDIF_CLK_SELR::LCDIF_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `LCDIF_CLK_SEL_1`"]
    #[inline]
    pub fn is_lcdif_clk_sel_1(&self) -> bool {
        *self == LCDIF_CLK_SELR::LCDIF_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `LCDIF_CLK_SEL_2`"]
    #[inline]
    pub fn is_lcdif_clk_sel_2(&self) -> bool {
        *self == LCDIF_CLK_SELR::LCDIF_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `LCDIF_CLK_SEL_3`"]
    #[inline]
    pub fn is_lcdif_clk_sel_3(&self) -> bool {
        *self == LCDIF_CLK_SELR::LCDIF_CLK_SEL_3
    }
    #[doc = "Checks if the value of the field is `LCDIF_CLK_SEL_4`"]
    #[inline]
    pub fn is_lcdif_clk_sel_4(&self) -> bool {
        *self == LCDIF_CLK_SELR::LCDIF_CLK_SEL_4
    }
}
#[doc = "Possible values of the field `LCDIF_PRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDIF_PREDR {
    #[doc = "divide by 1"]
    LCDIF_PRED_0,
    #[doc = "divide by 2"]
    LCDIF_PRED_1,
    #[doc = "divide by 3"]
    LCDIF_PRED_2,
    #[doc = "divide by 4"]
    LCDIF_PRED_3,
    #[doc = "divide by 5"]
    LCDIF_PRED_4,
    #[doc = "divide by 6"]
    LCDIF_PRED_5,
    #[doc = "divide by 7"]
    LCDIF_PRED_6,
    #[doc = "divide by 8"]
    LCDIF_PRED_7,
}
impl LCDIF_PREDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCDIF_PREDR::LCDIF_PRED_0 => 0,
            LCDIF_PREDR::LCDIF_PRED_1 => 1,
            LCDIF_PREDR::LCDIF_PRED_2 => 2,
            LCDIF_PREDR::LCDIF_PRED_3 => 3,
            LCDIF_PREDR::LCDIF_PRED_4 => 4,
            LCDIF_PREDR::LCDIF_PRED_5 => 5,
            LCDIF_PREDR::LCDIF_PRED_6 => 6,
            LCDIF_PREDR::LCDIF_PRED_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCDIF_PREDR {
        match value {
            0 => LCDIF_PREDR::LCDIF_PRED_0,
            1 => LCDIF_PREDR::LCDIF_PRED_1,
            2 => LCDIF_PREDR::LCDIF_PRED_2,
            3 => LCDIF_PREDR::LCDIF_PRED_3,
            4 => LCDIF_PREDR::LCDIF_PRED_4,
            5 => LCDIF_PREDR::LCDIF_PRED_5,
            6 => LCDIF_PREDR::LCDIF_PRED_6,
            7 => LCDIF_PREDR::LCDIF_PRED_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_0`"]
    #[inline]
    pub fn is_lcdif_pred_0(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_0
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_1`"]
    #[inline]
    pub fn is_lcdif_pred_1(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_1
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_2`"]
    #[inline]
    pub fn is_lcdif_pred_2(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_2
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_3`"]
    #[inline]
    pub fn is_lcdif_pred_3(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_3
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_4`"]
    #[inline]
    pub fn is_lcdif_pred_4(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_4
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_5`"]
    #[inline]
    pub fn is_lcdif_pred_5(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_5
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_6`"]
    #[inline]
    pub fn is_lcdif_pred_6(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_6
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRED_7`"]
    #[inline]
    pub fn is_lcdif_pred_7(&self) -> bool {
        *self == LCDIF_PREDR::LCDIF_PRED_7
    }
}
#[doc = "Possible values of the field `LCDIF_PRE_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDIF_PRE_CLK_SELR {
    #[doc = "derive clock from PLL2"]
    LCDIF_PRE_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD3"]
    LCDIF_PRE_CLK_SEL_1,
    #[doc = "derive clock from PLL5"]
    LCDIF_PRE_CLK_SEL_2,
    #[doc = "derive clock from PLL2 PFD0"]
    LCDIF_PRE_CLK_SEL_3,
    #[doc = "derive clock from PLL2 PFD1"]
    LCDIF_PRE_CLK_SEL_4,
    #[doc = "derive clock from PLL3 PFD1"]
    LCDIF_PRE_CLK_SEL_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LCDIF_PRE_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_0 => 0,
            LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_1 => 1,
            LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_2 => 2,
            LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_3 => 3,
            LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_4 => 4,
            LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_5 => 5,
            LCDIF_PRE_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCDIF_PRE_CLK_SELR {
        match value {
            0 => LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_0,
            1 => LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_1,
            2 => LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_2,
            3 => LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_3,
            4 => LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_4,
            5 => LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_5,
            i => LCDIF_PRE_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRE_CLK_SEL_0`"]
    #[inline]
    pub fn is_lcdif_pre_clk_sel_0(&self) -> bool {
        *self == LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRE_CLK_SEL_1`"]
    #[inline]
    pub fn is_lcdif_pre_clk_sel_1(&self) -> bool {
        *self == LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRE_CLK_SEL_2`"]
    #[inline]
    pub fn is_lcdif_pre_clk_sel_2(&self) -> bool {
        *self == LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRE_CLK_SEL_3`"]
    #[inline]
    pub fn is_lcdif_pre_clk_sel_3(&self) -> bool {
        *self == LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_3
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRE_CLK_SEL_4`"]
    #[inline]
    pub fn is_lcdif_pre_clk_sel_4(&self) -> bool {
        *self == LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_4
    }
    #[doc = "Checks if the value of the field is `LCDIF_PRE_CLK_SEL_5`"]
    #[inline]
    pub fn is_lcdif_pre_clk_sel_5(&self) -> bool {
        *self == LCDIF_PRE_CLK_SELR::LCDIF_PRE_CLK_SEL_5
    }
}
#[doc = "Possible values of the field `LPI2C_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C_CLK_SELR {
    #[doc = "derive clock from pll3_60m"]
    LPI2C_CLK_SEL_0,
    #[doc = "derive clock from osc_clk"]
    LPI2C_CLK_SEL_1,
}
impl LPI2C_CLK_SELR {
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
            LPI2C_CLK_SELR::LPI2C_CLK_SEL_0 => false,
            LPI2C_CLK_SELR::LPI2C_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPI2C_CLK_SELR {
        match value {
            false => LPI2C_CLK_SELR::LPI2C_CLK_SEL_0,
            true => LPI2C_CLK_SELR::LPI2C_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C_CLK_SEL_0`"]
    #[inline]
    pub fn is_lpi2c_clk_sel_0(&self) -> bool {
        *self == LPI2C_CLK_SELR::LPI2C_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `LPI2C_CLK_SEL_1`"]
    #[inline]
    pub fn is_lpi2c_clk_sel_1(&self) -> bool {
        *self == LPI2C_CLK_SELR::LPI2C_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `LPI2C_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C_CLK_PODFR {
    #[doc = "divide by 1"]
    LPI2C_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    LPI2C_CLK_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPI2C_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPI2C_CLK_PODFR::LPI2C_CLK_PODF_0 => 0,
            LPI2C_CLK_PODFR::LPI2C_CLK_PODF_63 => 63,
            LPI2C_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPI2C_CLK_PODFR {
        match value {
            0 => LPI2C_CLK_PODFR::LPI2C_CLK_PODF_0,
            63 => LPI2C_CLK_PODFR::LPI2C_CLK_PODF_63,
            i => LPI2C_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C_CLK_PODF_0`"]
    #[inline]
    pub fn is_lpi2c_clk_podf_0(&self) -> bool {
        *self == LPI2C_CLK_PODFR::LPI2C_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `LPI2C_CLK_PODF_63`"]
    #[inline]
    pub fn is_lpi2c_clk_podf_63(&self) -> bool {
        *self == LPI2C_CLK_PODFR::LPI2C_CLK_PODF_63
    }
}
#[doc = "Values that can be written to the field `LCDIF_CLK_SEL`"]
pub enum LCDIF_CLK_SELW {
    #[doc = "derive clock from divided pre-muxed LCDIF clock"]
    LCDIF_CLK_SEL_0,
    #[doc = "derive clock from ipp_di0_clk"]
    LCDIF_CLK_SEL_1,
    #[doc = "derive clock from ipp_di1_clk"]
    LCDIF_CLK_SEL_2,
    #[doc = "derive clock from ldb_di0_clk"]
    LCDIF_CLK_SEL_3,
    #[doc = "derive clock from ldb_di1_clk"]
    LCDIF_CLK_SEL_4,
}
impl LCDIF_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCDIF_CLK_SELW::LCDIF_CLK_SEL_0 => 0,
            LCDIF_CLK_SELW::LCDIF_CLK_SEL_1 => 1,
            LCDIF_CLK_SELW::LCDIF_CLK_SEL_2 => 2,
            LCDIF_CLK_SELW::LCDIF_CLK_SEL_3 => 3,
            LCDIF_CLK_SELW::LCDIF_CLK_SEL_4 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCDIF_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDIF_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCDIF_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "derive clock from divided pre-muxed LCDIF clock"]
    #[inline]
    pub fn lcdif_clk_sel_0(self) -> &'a mut W {
        self.variant(LCDIF_CLK_SELW::LCDIF_CLK_SEL_0)
    }
    #[doc = "derive clock from ipp_di0_clk"]
    #[inline]
    pub fn lcdif_clk_sel_1(self) -> &'a mut W {
        self.variant(LCDIF_CLK_SELW::LCDIF_CLK_SEL_1)
    }
    #[doc = "derive clock from ipp_di1_clk"]
    #[inline]
    pub fn lcdif_clk_sel_2(self) -> &'a mut W {
        self.variant(LCDIF_CLK_SELW::LCDIF_CLK_SEL_2)
    }
    #[doc = "derive clock from ldb_di0_clk"]
    #[inline]
    pub fn lcdif_clk_sel_3(self) -> &'a mut W {
        self.variant(LCDIF_CLK_SELW::LCDIF_CLK_SEL_3)
    }
    #[doc = "derive clock from ldb_di1_clk"]
    #[inline]
    pub fn lcdif_clk_sel_4(self) -> &'a mut W {
        self.variant(LCDIF_CLK_SELW::LCDIF_CLK_SEL_4)
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
#[doc = "Values that can be written to the field `LCDIF_PRED`"]
pub enum LCDIF_PREDW {
    #[doc = "divide by 1"]
    LCDIF_PRED_0,
    #[doc = "divide by 2"]
    LCDIF_PRED_1,
    #[doc = "divide by 3"]
    LCDIF_PRED_2,
    #[doc = "divide by 4"]
    LCDIF_PRED_3,
    #[doc = "divide by 5"]
    LCDIF_PRED_4,
    #[doc = "divide by 6"]
    LCDIF_PRED_5,
    #[doc = "divide by 7"]
    LCDIF_PRED_6,
    #[doc = "divide by 8"]
    LCDIF_PRED_7,
}
impl LCDIF_PREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCDIF_PREDW::LCDIF_PRED_0 => 0,
            LCDIF_PREDW::LCDIF_PRED_1 => 1,
            LCDIF_PREDW::LCDIF_PRED_2 => 2,
            LCDIF_PREDW::LCDIF_PRED_3 => 3,
            LCDIF_PREDW::LCDIF_PRED_4 => 4,
            LCDIF_PREDW::LCDIF_PRED_5 => 5,
            LCDIF_PREDW::LCDIF_PRED_6 => 6,
            LCDIF_PREDW::LCDIF_PRED_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCDIF_PREDW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDIF_PREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCDIF_PREDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn lcdif_pred_0(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn lcdif_pred_1(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn lcdif_pred_2(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn lcdif_pred_3(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn lcdif_pred_4(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn lcdif_pred_5(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn lcdif_pred_6(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn lcdif_pred_7(self) -> &'a mut W {
        self.variant(LCDIF_PREDW::LCDIF_PRED_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCDIF_PRE_CLK_SEL`"]
pub enum LCDIF_PRE_CLK_SELW {
    #[doc = "derive clock from PLL2"]
    LCDIF_PRE_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD3"]
    LCDIF_PRE_CLK_SEL_1,
    #[doc = "derive clock from PLL5"]
    LCDIF_PRE_CLK_SEL_2,
    #[doc = "derive clock from PLL2 PFD0"]
    LCDIF_PRE_CLK_SEL_3,
    #[doc = "derive clock from PLL2 PFD1"]
    LCDIF_PRE_CLK_SEL_4,
    #[doc = "derive clock from PLL3 PFD1"]
    LCDIF_PRE_CLK_SEL_5,
}
impl LCDIF_PRE_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_0 => 0,
            LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_1 => 1,
            LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_2 => 2,
            LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_3 => 3,
            LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_4 => 4,
            LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCDIF_PRE_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDIF_PRE_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCDIF_PRE_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "derive clock from PLL2"]
    #[inline]
    pub fn lcdif_pre_clk_sel_0(self) -> &'a mut W {
        self.variant(LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD3"]
    #[inline]
    pub fn lcdif_pre_clk_sel_1(self) -> &'a mut W {
        self.variant(LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL5"]
    #[inline]
    pub fn lcdif_pre_clk_sel_2(self) -> &'a mut W {
        self.variant(LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline]
    pub fn lcdif_pre_clk_sel_3(self) -> &'a mut W {
        self.variant(LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_3)
    }
    #[doc = "derive clock from PLL2 PFD1"]
    #[inline]
    pub fn lcdif_pre_clk_sel_4(self) -> &'a mut W {
        self.variant(LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_4)
    }
    #[doc = "derive clock from PLL3 PFD1"]
    #[inline]
    pub fn lcdif_pre_clk_sel_5(self) -> &'a mut W {
        self.variant(LCDIF_PRE_CLK_SELW::LCDIF_PRE_CLK_SEL_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPI2C_CLK_SEL`"]
pub enum LPI2C_CLK_SELW {
    #[doc = "derive clock from pll3_60m"]
    LPI2C_CLK_SEL_0,
    #[doc = "derive clock from osc_clk"]
    LPI2C_CLK_SEL_1,
}
impl LPI2C_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPI2C_CLK_SELW::LPI2C_CLK_SEL_0 => false,
            LPI2C_CLK_SELW::LPI2C_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "derive clock from pll3_60m"]
    #[inline]
    pub fn lpi2c_clk_sel_0(self) -> &'a mut W {
        self.variant(LPI2C_CLK_SELW::LPI2C_CLK_SEL_0)
    }
    #[doc = "derive clock from osc_clk"]
    #[inline]
    pub fn lpi2c_clk_sel_1(self) -> &'a mut W {
        self.variant(LPI2C_CLK_SELW::LPI2C_CLK_SEL_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPI2C_CLK_PODF`"]
pub enum LPI2C_CLK_PODFW {
    #[doc = "divide by 1"]
    LPI2C_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    LPI2C_CLK_PODF_63,
}
impl LPI2C_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPI2C_CLK_PODFW::LPI2C_CLK_PODF_0 => 0,
            LPI2C_CLK_PODFW::LPI2C_CLK_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPI2C_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _LPI2C_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPI2C_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn lpi2c_clk_podf_0(self) -> &'a mut W {
        self.variant(LPI2C_CLK_PODFW::LPI2C_CLK_PODF_0)
    }
    #[doc = "divide by 2^6"]
    #[inline]
    pub fn lpi2c_clk_podf_63(self) -> &'a mut W {
        self.variant(LPI2C_CLK_PODFW::LPI2C_CLK_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 19;
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
    #[doc = "Bits 9:11 - Selector for LCDIF root clock multiplexer"]
    #[inline]
    pub fn lcdif_clk_sel(&self) -> LCDIF_CLK_SELR {
        LCDIF_CLK_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Pre-divider for lcdif clock. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn lcdif_pred(&self) -> LCDIF_PREDR {
        LCDIF_PREDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:17 - Selector for lcdif root clock pre-multiplexer"]
    #[inline]
    pub fn lcdif_pre_clk_sel(&self) -> LCDIF_PRE_CLK_SELR {
        LCDIF_PRE_CLK_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Selector for the LPI2C clock multiplexor"]
    #[inline]
    pub fn lpi2c_clk_sel(&self) -> LPI2C_CLK_SELR {
        LPI2C_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:24 - Divider for lpi2c clock podf"]
    #[inline]
    pub fn lpi2c_clk_podf(&self) -> LPI2C_CLK_PODFR {
        LPI2C_CLK_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 170824 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 9:11 - Selector for LCDIF root clock multiplexer"]
    #[inline]
    pub fn lcdif_clk_sel(&mut self) -> _LCDIF_CLK_SELW {
        _LCDIF_CLK_SELW { w: self }
    }
    #[doc = "Bits 12:14 - Pre-divider for lcdif clock. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn lcdif_pred(&mut self) -> _LCDIF_PREDW {
        _LCDIF_PREDW { w: self }
    }
    #[doc = "Bits 15:17 - Selector for lcdif root clock pre-multiplexer"]
    #[inline]
    pub fn lcdif_pre_clk_sel(&mut self) -> _LCDIF_PRE_CLK_SELW {
        _LCDIF_PRE_CLK_SELW { w: self }
    }
    #[doc = "Bit 18 - Selector for the LPI2C clock multiplexor"]
    #[inline]
    pub fn lpi2c_clk_sel(&mut self) -> _LPI2C_CLK_SELW {
        _LPI2C_CLK_SELW { w: self }
    }
    #[doc = "Bits 19:24 - Divider for lpi2c clock podf"]
    #[inline]
    pub fn lpi2c_clk_podf(&mut self) -> _LPI2C_CLK_PODFW {
        _LPI2C_CLK_PODFW { w: self }
    }
}
