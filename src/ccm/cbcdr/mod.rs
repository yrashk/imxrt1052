#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CBCDR {
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
#[doc = "Possible values of the field `SEMC_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_CLK_SELR {
    #[doc = "Periph_clk output will be used as SEMC clock root"]
    SEMC_CLK_SEL_0,
    #[doc = "SEMC alternative clock will be used as SEMC clock root"]
    SEMC_CLK_SEL_1,
}
impl SEMC_CLK_SELR {
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
            SEMC_CLK_SELR::SEMC_CLK_SEL_0 => false,
            SEMC_CLK_SELR::SEMC_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEMC_CLK_SELR {
        match value {
            false => SEMC_CLK_SELR::SEMC_CLK_SEL_0,
            true => SEMC_CLK_SELR::SEMC_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_CLK_SEL_0`"]
    #[inline]
    pub fn is_semc_clk_sel_0(&self) -> bool {
        *self == SEMC_CLK_SELR::SEMC_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SEMC_CLK_SEL_1`"]
    #[inline]
    pub fn is_semc_clk_sel_1(&self) -> bool {
        *self == SEMC_CLK_SELR::SEMC_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `SEMC_ALT_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_ALT_CLK_SELR {
    #[doc = "PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_0,
    #[doc = "PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_1,
}
impl SEMC_ALT_CLK_SELR {
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
            SEMC_ALT_CLK_SELR::SEMC_ALT_CLK_SEL_0 => false,
            SEMC_ALT_CLK_SELR::SEMC_ALT_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEMC_ALT_CLK_SELR {
        match value {
            false => SEMC_ALT_CLK_SELR::SEMC_ALT_CLK_SEL_0,
            true => SEMC_ALT_CLK_SELR::SEMC_ALT_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_ALT_CLK_SEL_0`"]
    #[inline]
    pub fn is_semc_alt_clk_sel_0(&self) -> bool {
        *self == SEMC_ALT_CLK_SELR::SEMC_ALT_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SEMC_ALT_CLK_SEL_1`"]
    #[inline]
    pub fn is_semc_alt_clk_sel_1(&self) -> bool {
        *self == SEMC_ALT_CLK_SELR::SEMC_ALT_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `IPG_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPG_PODFR {
    #[doc = "divide by 1"]
    IPG_PODF_0,
    #[doc = "divide by 2"]
    IPG_PODF_1,
    #[doc = "divide by 3"]
    IPG_PODF_2,
    #[doc = "divide by 4"]
    IPG_PODF_3,
}
impl IPG_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IPG_PODFR::IPG_PODF_0 => 0,
            IPG_PODFR::IPG_PODF_1 => 1,
            IPG_PODFR::IPG_PODF_2 => 2,
            IPG_PODFR::IPG_PODF_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IPG_PODFR {
        match value {
            0 => IPG_PODFR::IPG_PODF_0,
            1 => IPG_PODFR::IPG_PODF_1,
            2 => IPG_PODFR::IPG_PODF_2,
            3 => IPG_PODFR::IPG_PODF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_0`"]
    #[inline]
    pub fn is_ipg_podf_0(&self) -> bool {
        *self == IPG_PODFR::IPG_PODF_0
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_1`"]
    #[inline]
    pub fn is_ipg_podf_1(&self) -> bool {
        *self == IPG_PODFR::IPG_PODF_1
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_2`"]
    #[inline]
    pub fn is_ipg_podf_2(&self) -> bool {
        *self == IPG_PODFR::IPG_PODF_2
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_3`"]
    #[inline]
    pub fn is_ipg_podf_3(&self) -> bool {
        *self == IPG_PODFR::IPG_PODF_3
    }
}
#[doc = "Possible values of the field `AHB_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_PODFR {
    #[doc = "divide by 1"]
    AHB_PODF_0,
    #[doc = "divide by 2"]
    AHB_PODF_1,
    #[doc = "divide by 3"]
    AHB_PODF_2,
    #[doc = "divide by 4"]
    AHB_PODF_3,
    #[doc = "divide by 5"]
    AHB_PODF_4,
    #[doc = "divide by 6"]
    AHB_PODF_5,
    #[doc = "divide by 7"]
    AHB_PODF_6,
    #[doc = "divide by 8"]
    AHB_PODF_7,
}
impl AHB_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AHB_PODFR::AHB_PODF_0 => 0,
            AHB_PODFR::AHB_PODF_1 => 1,
            AHB_PODFR::AHB_PODF_2 => 2,
            AHB_PODFR::AHB_PODF_3 => 3,
            AHB_PODFR::AHB_PODF_4 => 4,
            AHB_PODFR::AHB_PODF_5 => 5,
            AHB_PODFR::AHB_PODF_6 => 6,
            AHB_PODFR::AHB_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AHB_PODFR {
        match value {
            0 => AHB_PODFR::AHB_PODF_0,
            1 => AHB_PODFR::AHB_PODF_1,
            2 => AHB_PODFR::AHB_PODF_2,
            3 => AHB_PODFR::AHB_PODF_3,
            4 => AHB_PODFR::AHB_PODF_4,
            5 => AHB_PODFR::AHB_PODF_5,
            6 => AHB_PODFR::AHB_PODF_6,
            7 => AHB_PODFR::AHB_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_0`"]
    #[inline]
    pub fn is_ahb_podf_0(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_1`"]
    #[inline]
    pub fn is_ahb_podf_1(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_1
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_2`"]
    #[inline]
    pub fn is_ahb_podf_2(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_2
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_3`"]
    #[inline]
    pub fn is_ahb_podf_3(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_3
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_4`"]
    #[inline]
    pub fn is_ahb_podf_4(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_4
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_5`"]
    #[inline]
    pub fn is_ahb_podf_5(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_5
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_6`"]
    #[inline]
    pub fn is_ahb_podf_6(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_6
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_7`"]
    #[inline]
    pub fn is_ahb_podf_7(&self) -> bool {
        *self == AHB_PODFR::AHB_PODF_7
    }
}
#[doc = "Possible values of the field `SEMC_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_PODFR {
    #[doc = "divide by 1"]
    SEMC_PODF_0,
    #[doc = "divide by 2"]
    SEMC_PODF_1,
    #[doc = "divide by 3"]
    SEMC_PODF_2,
    #[doc = "divide by 4"]
    SEMC_PODF_3,
    #[doc = "divide by 5"]
    SEMC_PODF_4,
    #[doc = "divide by 6"]
    SEMC_PODF_5,
    #[doc = "divide by 7"]
    SEMC_PODF_6,
    #[doc = "divide by 8"]
    SEMC_PODF_7,
}
impl SEMC_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEMC_PODFR::SEMC_PODF_0 => 0,
            SEMC_PODFR::SEMC_PODF_1 => 1,
            SEMC_PODFR::SEMC_PODF_2 => 2,
            SEMC_PODFR::SEMC_PODF_3 => 3,
            SEMC_PODFR::SEMC_PODF_4 => 4,
            SEMC_PODFR::SEMC_PODF_5 => 5,
            SEMC_PODFR::SEMC_PODF_6 => 6,
            SEMC_PODFR::SEMC_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEMC_PODFR {
        match value {
            0 => SEMC_PODFR::SEMC_PODF_0,
            1 => SEMC_PODFR::SEMC_PODF_1,
            2 => SEMC_PODFR::SEMC_PODF_2,
            3 => SEMC_PODFR::SEMC_PODF_3,
            4 => SEMC_PODFR::SEMC_PODF_4,
            5 => SEMC_PODFR::SEMC_PODF_5,
            6 => SEMC_PODFR::SEMC_PODF_6,
            7 => SEMC_PODFR::SEMC_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_0`"]
    #[inline]
    pub fn is_semc_podf_0(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_1`"]
    #[inline]
    pub fn is_semc_podf_1(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_1
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_2`"]
    #[inline]
    pub fn is_semc_podf_2(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_2
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_3`"]
    #[inline]
    pub fn is_semc_podf_3(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_3
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_4`"]
    #[inline]
    pub fn is_semc_podf_4(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_4
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_5`"]
    #[inline]
    pub fn is_semc_podf_5(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_5
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_6`"]
    #[inline]
    pub fn is_semc_podf_6(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_6
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_7`"]
    #[inline]
    pub fn is_semc_podf_7(&self) -> bool {
        *self == SEMC_PODFR::SEMC_PODF_7
    }
}
#[doc = "Possible values of the field `PERIPH_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK_SELR {
    #[doc = "derive clock from pre_periph_clk_sel"]
    PERIPH_CLK_SEL_0,
    #[doc = "derive clock from periph_clk2_clk_divided"]
    PERIPH_CLK_SEL_1,
}
impl PERIPH_CLK_SELR {
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
            PERIPH_CLK_SELR::PERIPH_CLK_SEL_0 => false,
            PERIPH_CLK_SELR::PERIPH_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERIPH_CLK_SELR {
        match value {
            false => PERIPH_CLK_SELR::PERIPH_CLK_SEL_0,
            true => PERIPH_CLK_SELR::PERIPH_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_0`"]
    #[inline]
    pub fn is_periph_clk_sel_0(&self) -> bool {
        *self == PERIPH_CLK_SELR::PERIPH_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_1`"]
    #[inline]
    pub fn is_periph_clk_sel_1(&self) -> bool {
        *self == PERIPH_CLK_SELR::PERIPH_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `PERIPH_CLK2_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK2_PODFR {
    #[doc = "divide by 1"]
    PERIPH_CLK2_PODF_0,
    #[doc = "divide by 2"]
    PERIPH_CLK2_PODF_1,
    #[doc = "divide by 3"]
    PERIPH_CLK2_PODF_2,
    #[doc = "divide by 4"]
    PERIPH_CLK2_PODF_3,
    #[doc = "divide by 5"]
    PERIPH_CLK2_PODF_4,
    #[doc = "divide by 6"]
    PERIPH_CLK2_PODF_5,
    #[doc = "divide by 7"]
    PERIPH_CLK2_PODF_6,
    #[doc = "divide by 8"]
    PERIPH_CLK2_PODF_7,
}
impl PERIPH_CLK2_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_0 => 0,
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_1 => 1,
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_2 => 2,
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_3 => 3,
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_4 => 4,
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_5 => 5,
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_6 => 6,
            PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERIPH_CLK2_PODFR {
        match value {
            0 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_0,
            1 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_1,
            2 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_2,
            3 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_3,
            4 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_4,
            5 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_5,
            6 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_6,
            7 => PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_0`"]
    #[inline]
    pub fn is_periph_clk2_podf_0(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_1`"]
    #[inline]
    pub fn is_periph_clk2_podf_1(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_1
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_2`"]
    #[inline]
    pub fn is_periph_clk2_podf_2(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_2
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_3`"]
    #[inline]
    pub fn is_periph_clk2_podf_3(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_3
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_4`"]
    #[inline]
    pub fn is_periph_clk2_podf_4(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_4
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_5`"]
    #[inline]
    pub fn is_periph_clk2_podf_5(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_5
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_6`"]
    #[inline]
    pub fn is_periph_clk2_podf_6(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_6
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_7`"]
    #[inline]
    pub fn is_periph_clk2_podf_7(&self) -> bool {
        *self == PERIPH_CLK2_PODFR::PERIPH_CLK2_PODF_7
    }
}
#[doc = "Values that can be written to the field `SEMC_CLK_SEL`"]
pub enum SEMC_CLK_SELW {
    #[doc = "Periph_clk output will be used as SEMC clock root"]
    SEMC_CLK_SEL_0,
    #[doc = "SEMC alternative clock will be used as SEMC clock root"]
    SEMC_CLK_SEL_1,
}
impl SEMC_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEMC_CLK_SELW::SEMC_CLK_SEL_0 => false,
            SEMC_CLK_SELW::SEMC_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEMC_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SEMC_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEMC_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Periph_clk output will be used as SEMC clock root"]
    #[inline]
    pub fn semc_clk_sel_0(self) -> &'a mut W {
        self.variant(SEMC_CLK_SELW::SEMC_CLK_SEL_0)
    }
    #[doc = "SEMC alternative clock will be used as SEMC clock root"]
    #[inline]
    pub fn semc_clk_sel_1(self) -> &'a mut W {
        self.variant(SEMC_CLK_SELW::SEMC_CLK_SEL_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEMC_ALT_CLK_SEL`"]
pub enum SEMC_ALT_CLK_SELW {
    #[doc = "PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_0,
    #[doc = "PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_1,
}
impl SEMC_ALT_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEMC_ALT_CLK_SELW::SEMC_ALT_CLK_SEL_0 => false,
            SEMC_ALT_CLK_SELW::SEMC_ALT_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEMC_ALT_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SEMC_ALT_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEMC_ALT_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    #[inline]
    pub fn semc_alt_clk_sel_0(self) -> &'a mut W {
        self.variant(SEMC_ALT_CLK_SELW::SEMC_ALT_CLK_SEL_0)
    }
    #[doc = "PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    #[inline]
    pub fn semc_alt_clk_sel_1(self) -> &'a mut W {
        self.variant(SEMC_ALT_CLK_SELW::SEMC_ALT_CLK_SEL_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IPG_PODF`"]
pub enum IPG_PODFW {
    #[doc = "divide by 1"]
    IPG_PODF_0,
    #[doc = "divide by 2"]
    IPG_PODF_1,
    #[doc = "divide by 3"]
    IPG_PODF_2,
    #[doc = "divide by 4"]
    IPG_PODF_3,
}
impl IPG_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IPG_PODFW::IPG_PODF_0 => 0,
            IPG_PODFW::IPG_PODF_1 => 1,
            IPG_PODFW::IPG_PODF_2 => 2,
            IPG_PODFW::IPG_PODF_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPG_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _IPG_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPG_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn ipg_podf_0(self) -> &'a mut W {
        self.variant(IPG_PODFW::IPG_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn ipg_podf_1(self) -> &'a mut W {
        self.variant(IPG_PODFW::IPG_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn ipg_podf_2(self) -> &'a mut W {
        self.variant(IPG_PODFW::IPG_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn ipg_podf_3(self) -> &'a mut W {
        self.variant(IPG_PODFW::IPG_PODF_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHB_PODF`"]
pub enum AHB_PODFW {
    #[doc = "divide by 1"]
    AHB_PODF_0,
    #[doc = "divide by 2"]
    AHB_PODF_1,
    #[doc = "divide by 3"]
    AHB_PODF_2,
    #[doc = "divide by 4"]
    AHB_PODF_3,
    #[doc = "divide by 5"]
    AHB_PODF_4,
    #[doc = "divide by 6"]
    AHB_PODF_5,
    #[doc = "divide by 7"]
    AHB_PODF_6,
    #[doc = "divide by 8"]
    AHB_PODF_7,
}
impl AHB_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AHB_PODFW::AHB_PODF_0 => 0,
            AHB_PODFW::AHB_PODF_1 => 1,
            AHB_PODFW::AHB_PODF_2 => 2,
            AHB_PODFW::AHB_PODF_3 => 3,
            AHB_PODFW::AHB_PODF_4 => 4,
            AHB_PODFW::AHB_PODF_5 => 5,
            AHB_PODFW::AHB_PODF_6 => 6,
            AHB_PODFW::AHB_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn ahb_podf_0(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn ahb_podf_1(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn ahb_podf_2(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn ahb_podf_3(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn ahb_podf_4(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn ahb_podf_5(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn ahb_podf_6(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn ahb_podf_7(self) -> &'a mut W {
        self.variant(AHB_PODFW::AHB_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEMC_PODF`"]
pub enum SEMC_PODFW {
    #[doc = "divide by 1"]
    SEMC_PODF_0,
    #[doc = "divide by 2"]
    SEMC_PODF_1,
    #[doc = "divide by 3"]
    SEMC_PODF_2,
    #[doc = "divide by 4"]
    SEMC_PODF_3,
    #[doc = "divide by 5"]
    SEMC_PODF_4,
    #[doc = "divide by 6"]
    SEMC_PODF_5,
    #[doc = "divide by 7"]
    SEMC_PODF_6,
    #[doc = "divide by 8"]
    SEMC_PODF_7,
}
impl SEMC_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEMC_PODFW::SEMC_PODF_0 => 0,
            SEMC_PODFW::SEMC_PODF_1 => 1,
            SEMC_PODFW::SEMC_PODF_2 => 2,
            SEMC_PODFW::SEMC_PODF_3 => 3,
            SEMC_PODFW::SEMC_PODF_4 => 4,
            SEMC_PODFW::SEMC_PODF_5 => 5,
            SEMC_PODFW::SEMC_PODF_6 => 6,
            SEMC_PODFW::SEMC_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEMC_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _SEMC_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEMC_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn semc_podf_0(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn semc_podf_1(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn semc_podf_2(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn semc_podf_3(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn semc_podf_4(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn semc_podf_5(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn semc_podf_6(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn semc_podf_7(self) -> &'a mut W {
        self.variant(SEMC_PODFW::SEMC_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PERIPH_CLK_SEL`"]
pub enum PERIPH_CLK_SELW {
    #[doc = "derive clock from pre_periph_clk_sel"]
    PERIPH_CLK_SEL_0,
    #[doc = "derive clock from periph_clk2_clk_divided"]
    PERIPH_CLK_SEL_1,
}
impl PERIPH_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERIPH_CLK_SELW::PERIPH_CLK_SEL_0 => false,
            PERIPH_CLK_SELW::PERIPH_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERIPH_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPH_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERIPH_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "derive clock from pre_periph_clk_sel"]
    #[inline]
    pub fn periph_clk_sel_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SELW::PERIPH_CLK_SEL_0)
    }
    #[doc = "derive clock from periph_clk2_clk_divided"]
    #[inline]
    pub fn periph_clk_sel_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SELW::PERIPH_CLK_SEL_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PERIPH_CLK2_PODF`"]
pub enum PERIPH_CLK2_PODFW {
    #[doc = "divide by 1"]
    PERIPH_CLK2_PODF_0,
    #[doc = "divide by 2"]
    PERIPH_CLK2_PODF_1,
    #[doc = "divide by 3"]
    PERIPH_CLK2_PODF_2,
    #[doc = "divide by 4"]
    PERIPH_CLK2_PODF_3,
    #[doc = "divide by 5"]
    PERIPH_CLK2_PODF_4,
    #[doc = "divide by 6"]
    PERIPH_CLK2_PODF_5,
    #[doc = "divide by 7"]
    PERIPH_CLK2_PODF_6,
    #[doc = "divide by 8"]
    PERIPH_CLK2_PODF_7,
}
impl PERIPH_CLK2_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_0 => 0,
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_1 => 1,
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_2 => 2,
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_3 => 3,
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_4 => 4,
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_5 => 5,
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_6 => 6,
            PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERIPH_CLK2_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPH_CLK2_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERIPH_CLK2_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn periph_clk2_podf_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn periph_clk2_podf_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn periph_clk2_podf_2(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn periph_clk2_podf_3(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn periph_clk2_podf_4(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn periph_clk2_podf_5(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn periph_clk2_podf_6(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn periph_clk2_podf_7(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODFW::PERIPH_CLK2_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 6 - SEMC clock source select"]
    #[inline]
    pub fn semc_clk_sel(&self) -> SEMC_CLK_SELR {
        SEMC_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SEMC alternative clock select"]
    #[inline]
    pub fn semc_alt_clk_sel(&self) -> SEMC_ALT_CLK_SELR {
        SEMC_ALT_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Divider for ipg podf"]
    #[inline]
    pub fn ipg_podf(&self) -> IPG_PODFR {
        IPG_PODFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:12 - Divider for AHB PODF"]
    #[inline]
    pub fn ahb_podf(&self) -> AHB_PODFR {
        AHB_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Post divider for SEMC clock"]
    #[inline]
    pub fn semc_podf(&self) -> SEMC_PODFR {
        SEMC_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - Selector for peripheral main clock"]
    #[inline]
    pub fn periph_clk_sel(&self) -> PERIPH_CLK_SELR {
        PERIPH_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 27:29 - Divider for periph_clk2_podf."]
    #[inline]
    pub fn periph_clk2_podf(&self) -> PERIPH_CLK2_PODFR {
        PERIPH_CLK2_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 755200 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 6 - SEMC clock source select"]
    #[inline]
    pub fn semc_clk_sel(&mut self) -> _SEMC_CLK_SELW {
        _SEMC_CLK_SELW { w: self }
    }
    #[doc = "Bit 7 - SEMC alternative clock select"]
    #[inline]
    pub fn semc_alt_clk_sel(&mut self) -> _SEMC_ALT_CLK_SELW {
        _SEMC_ALT_CLK_SELW { w: self }
    }
    #[doc = "Bits 8:9 - Divider for ipg podf"]
    #[inline]
    pub fn ipg_podf(&mut self) -> _IPG_PODFW {
        _IPG_PODFW { w: self }
    }
    #[doc = "Bits 10:12 - Divider for AHB PODF"]
    #[inline]
    pub fn ahb_podf(&mut self) -> _AHB_PODFW {
        _AHB_PODFW { w: self }
    }
    #[doc = "Bits 16:18 - Post divider for SEMC clock"]
    #[inline]
    pub fn semc_podf(&mut self) -> _SEMC_PODFW {
        _SEMC_PODFW { w: self }
    }
    #[doc = "Bit 25 - Selector for peripheral main clock"]
    #[inline]
    pub fn periph_clk_sel(&mut self) -> _PERIPH_CLK_SELW {
        _PERIPH_CLK_SELW { w: self }
    }
    #[doc = "Bits 27:29 - Divider for periph_clk2_podf."]
    #[inline]
    pub fn periph_clk2_podf(&mut self) -> _PERIPH_CLK2_PODFW {
        _PERIPH_CLK2_PODFW { w: self }
    }
}
