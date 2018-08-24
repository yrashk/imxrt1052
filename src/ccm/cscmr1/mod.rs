#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSCMR1 {
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
#[doc = "Possible values of the field `PERCLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERCLK_PODFR {
    #[doc = "divide by 1"]
    PERCLK_PODF_0,
    #[doc = "divide by 2"]
    PERCLK_PODF_1,
    #[doc = "divide by 3"]
    PERCLK_PODF_2,
    #[doc = "divide by 4"]
    PERCLK_PODF_3,
    #[doc = "divide by 5"]
    PERCLK_PODF_4,
    #[doc = "divide by 6"]
    PERCLK_PODF_5,
    #[doc = "divide by 7"]
    PERCLK_PODF_6,
    #[doc = "divide by 64"]
    PERCLK_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PERCLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERCLK_PODFR::PERCLK_PODF_0 => 0,
            PERCLK_PODFR::PERCLK_PODF_1 => 1,
            PERCLK_PODFR::PERCLK_PODF_2 => 2,
            PERCLK_PODFR::PERCLK_PODF_3 => 3,
            PERCLK_PODFR::PERCLK_PODF_4 => 4,
            PERCLK_PODFR::PERCLK_PODF_5 => 5,
            PERCLK_PODFR::PERCLK_PODF_6 => 6,
            PERCLK_PODFR::PERCLK_PODF_63 => 63,
            PERCLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERCLK_PODFR {
        match value {
            0 => PERCLK_PODFR::PERCLK_PODF_0,
            1 => PERCLK_PODFR::PERCLK_PODF_1,
            2 => PERCLK_PODFR::PERCLK_PODF_2,
            3 => PERCLK_PODFR::PERCLK_PODF_3,
            4 => PERCLK_PODFR::PERCLK_PODF_4,
            5 => PERCLK_PODFR::PERCLK_PODF_5,
            6 => PERCLK_PODFR::PERCLK_PODF_6,
            63 => PERCLK_PODFR::PERCLK_PODF_63,
            i => PERCLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_0`"]
    #[inline]
    pub fn is_perclk_podf_0(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_1`"]
    #[inline]
    pub fn is_perclk_podf_1(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_1
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_2`"]
    #[inline]
    pub fn is_perclk_podf_2(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_2
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_3`"]
    #[inline]
    pub fn is_perclk_podf_3(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_3
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_4`"]
    #[inline]
    pub fn is_perclk_podf_4(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_4
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_5`"]
    #[inline]
    pub fn is_perclk_podf_5(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_5
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_6`"]
    #[inline]
    pub fn is_perclk_podf_6(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_6
    }
    #[doc = "Checks if the value of the field is `PERCLK_PODF_63`"]
    #[inline]
    pub fn is_perclk_podf_63(&self) -> bool {
        *self == PERCLK_PODFR::PERCLK_PODF_63
    }
}
#[doc = "Possible values of the field `PERCLK_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERCLK_CLK_SELR {
    #[doc = "derive clock from ipg clk root"]
    PERCLK_CLK_SEL_0,
    #[doc = "derive clock from osc_clk"]
    PERCLK_CLK_SEL_1,
}
impl PERCLK_CLK_SELR {
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
            PERCLK_CLK_SELR::PERCLK_CLK_SEL_0 => false,
            PERCLK_CLK_SELR::PERCLK_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERCLK_CLK_SELR {
        match value {
            false => PERCLK_CLK_SELR::PERCLK_CLK_SEL_0,
            true => PERCLK_CLK_SELR::PERCLK_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERCLK_CLK_SEL_0`"]
    #[inline]
    pub fn is_perclk_clk_sel_0(&self) -> bool {
        *self == PERCLK_CLK_SELR::PERCLK_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERCLK_CLK_SEL_1`"]
    #[inline]
    pub fn is_perclk_clk_sel_1(&self) -> bool {
        *self == PERCLK_CLK_SELR::PERCLK_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `SAI1_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_CLK_SELR {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI1_CLK_SEL_0,
    #[doc = "derive clock from PLL5"]
    SAI1_CLK_SEL_1,
    #[doc = "derive clock from PLL4"]
    SAI1_CLK_SEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI1_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI1_CLK_SELR::SAI1_CLK_SEL_0 => 0,
            SAI1_CLK_SELR::SAI1_CLK_SEL_1 => 1,
            SAI1_CLK_SELR::SAI1_CLK_SEL_2 => 2,
            SAI1_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI1_CLK_SELR {
        match value {
            0 => SAI1_CLK_SELR::SAI1_CLK_SEL_0,
            1 => SAI1_CLK_SELR::SAI1_CLK_SEL_1,
            2 => SAI1_CLK_SELR::SAI1_CLK_SEL_2,
            i => SAI1_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_SEL_0`"]
    #[inline]
    pub fn is_sai1_clk_sel_0(&self) -> bool {
        *self == SAI1_CLK_SELR::SAI1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_SEL_1`"]
    #[inline]
    pub fn is_sai1_clk_sel_1(&self) -> bool {
        *self == SAI1_CLK_SELR::SAI1_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_SEL_2`"]
    #[inline]
    pub fn is_sai1_clk_sel_2(&self) -> bool {
        *self == SAI1_CLK_SELR::SAI1_CLK_SEL_2
    }
}
#[doc = "Possible values of the field `SAI2_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_CLK_SELR {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI2_CLK_SEL_0,
    #[doc = "derive clock from PLL5"]
    SAI2_CLK_SEL_1,
    #[doc = "derive clock from PLL4"]
    SAI2_CLK_SEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI2_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI2_CLK_SELR::SAI2_CLK_SEL_0 => 0,
            SAI2_CLK_SELR::SAI2_CLK_SEL_1 => 1,
            SAI2_CLK_SELR::SAI2_CLK_SEL_2 => 2,
            SAI2_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI2_CLK_SELR {
        match value {
            0 => SAI2_CLK_SELR::SAI2_CLK_SEL_0,
            1 => SAI2_CLK_SELR::SAI2_CLK_SEL_1,
            2 => SAI2_CLK_SELR::SAI2_CLK_SEL_2,
            i => SAI2_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_SEL_0`"]
    #[inline]
    pub fn is_sai2_clk_sel_0(&self) -> bool {
        *self == SAI2_CLK_SELR::SAI2_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_SEL_1`"]
    #[inline]
    pub fn is_sai2_clk_sel_1(&self) -> bool {
        *self == SAI2_CLK_SELR::SAI2_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_SEL_2`"]
    #[inline]
    pub fn is_sai2_clk_sel_2(&self) -> bool {
        *self == SAI2_CLK_SELR::SAI2_CLK_SEL_2
    }
}
#[doc = "Possible values of the field `SAI3_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_CLK_SELR {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI3_CLK_SEL_0,
    #[doc = "derive clock from PLL5"]
    SAI3_CLK_SEL_1,
    #[doc = "derive clock from PLL4"]
    SAI3_CLK_SEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI3_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI3_CLK_SELR::SAI3_CLK_SEL_0 => 0,
            SAI3_CLK_SELR::SAI3_CLK_SEL_1 => 1,
            SAI3_CLK_SELR::SAI3_CLK_SEL_2 => 2,
            SAI3_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI3_CLK_SELR {
        match value {
            0 => SAI3_CLK_SELR::SAI3_CLK_SEL_0,
            1 => SAI3_CLK_SELR::SAI3_CLK_SEL_1,
            2 => SAI3_CLK_SELR::SAI3_CLK_SEL_2,
            i => SAI3_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_SEL_0`"]
    #[inline]
    pub fn is_sai3_clk_sel_0(&self) -> bool {
        *self == SAI3_CLK_SELR::SAI3_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_SEL_1`"]
    #[inline]
    pub fn is_sai3_clk_sel_1(&self) -> bool {
        *self == SAI3_CLK_SELR::SAI3_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_SEL_2`"]
    #[inline]
    pub fn is_sai3_clk_sel_2(&self) -> bool {
        *self == SAI3_CLK_SELR::SAI3_CLK_SEL_2
    }
}
#[doc = "Possible values of the field `USDHC1_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC1_CLK_SELR {
    #[doc = "derive clock from PLL2 PFD2"]
    USDHC1_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD0"]
    USDHC1_CLK_SEL_1,
}
impl USDHC1_CLK_SELR {
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
            USDHC1_CLK_SELR::USDHC1_CLK_SEL_0 => false,
            USDHC1_CLK_SELR::USDHC1_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USDHC1_CLK_SELR {
        match value {
            false => USDHC1_CLK_SELR::USDHC1_CLK_SEL_0,
            true => USDHC1_CLK_SELR::USDHC1_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC1_CLK_SEL_0`"]
    #[inline]
    pub fn is_usdhc1_clk_sel_0(&self) -> bool {
        *self == USDHC1_CLK_SELR::USDHC1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `USDHC1_CLK_SEL_1`"]
    #[inline]
    pub fn is_usdhc1_clk_sel_1(&self) -> bool {
        *self == USDHC1_CLK_SELR::USDHC1_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `USDHC2_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC2_CLK_SELR {
    #[doc = "derive clock from PLL2 PFD2"]
    USDHC2_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD0"]
    USDHC2_CLK_SEL_1,
}
impl USDHC2_CLK_SELR {
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
            USDHC2_CLK_SELR::USDHC2_CLK_SEL_0 => false,
            USDHC2_CLK_SELR::USDHC2_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USDHC2_CLK_SELR {
        match value {
            false => USDHC2_CLK_SELR::USDHC2_CLK_SEL_0,
            true => USDHC2_CLK_SELR::USDHC2_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC2_CLK_SEL_0`"]
    #[inline]
    pub fn is_usdhc2_clk_sel_0(&self) -> bool {
        *self == USDHC2_CLK_SELR::USDHC2_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `USDHC2_CLK_SEL_1`"]
    #[inline]
    pub fn is_usdhc2_clk_sel_1(&self) -> bool {
        *self == USDHC2_CLK_SELR::USDHC2_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `FLEXSPI_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_PODFR {
    #[doc = "divide by 1"]
    FLEXSPI_PODF_0,
    #[doc = "divide by 2"]
    FLEXSPI_PODF_1,
    #[doc = "divide by 3"]
    FLEXSPI_PODF_2,
    #[doc = "divide by 4"]
    FLEXSPI_PODF_3,
    #[doc = "divide by 5"]
    FLEXSPI_PODF_4,
    #[doc = "divide by 6"]
    FLEXSPI_PODF_5,
    #[doc = "divide by 7"]
    FLEXSPI_PODF_6,
    #[doc = "divide by 8"]
    FLEXSPI_PODF_7,
}
impl FLEXSPI_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXSPI_PODFR::FLEXSPI_PODF_0 => 0,
            FLEXSPI_PODFR::FLEXSPI_PODF_1 => 1,
            FLEXSPI_PODFR::FLEXSPI_PODF_2 => 2,
            FLEXSPI_PODFR::FLEXSPI_PODF_3 => 3,
            FLEXSPI_PODFR::FLEXSPI_PODF_4 => 4,
            FLEXSPI_PODFR::FLEXSPI_PODF_5 => 5,
            FLEXSPI_PODFR::FLEXSPI_PODF_6 => 6,
            FLEXSPI_PODFR::FLEXSPI_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXSPI_PODFR {
        match value {
            0 => FLEXSPI_PODFR::FLEXSPI_PODF_0,
            1 => FLEXSPI_PODFR::FLEXSPI_PODF_1,
            2 => FLEXSPI_PODFR::FLEXSPI_PODF_2,
            3 => FLEXSPI_PODFR::FLEXSPI_PODF_3,
            4 => FLEXSPI_PODFR::FLEXSPI_PODF_4,
            5 => FLEXSPI_PODFR::FLEXSPI_PODF_5,
            6 => FLEXSPI_PODFR::FLEXSPI_PODF_6,
            7 => FLEXSPI_PODFR::FLEXSPI_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_0`"]
    #[inline]
    pub fn is_flexspi_podf_0(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_1`"]
    #[inline]
    pub fn is_flexspi_podf_1(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_1
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_2`"]
    #[inline]
    pub fn is_flexspi_podf_2(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_2
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_3`"]
    #[inline]
    pub fn is_flexspi_podf_3(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_3
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_4`"]
    #[inline]
    pub fn is_flexspi_podf_4(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_4
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_5`"]
    #[inline]
    pub fn is_flexspi_podf_5(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_5
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_6`"]
    #[inline]
    pub fn is_flexspi_podf_6(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_6
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_7`"]
    #[inline]
    pub fn is_flexspi_podf_7(&self) -> bool {
        *self == FLEXSPI_PODFR::FLEXSPI_PODF_7
    }
}
#[doc = "Possible values of the field `FLEXSPI_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_CLK_SELR {
    #[doc = "derive clock from semc_clk_root_pre"]
    FLEXSPI_CLK_SEL_0,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXSPI_CLK_SEL_1,
    #[doc = "derive clock from PLL2 PFD2"]
    FLEXSPI_CLK_SEL_2,
    #[doc = "derive clock from PLL3 PFD0"]
    FLEXSPI_CLK_SEL_3,
}
impl FLEXSPI_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_0 => 0,
            FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_1 => 1,
            FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_2 => 2,
            FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXSPI_CLK_SELR {
        match value {
            0 => FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_0,
            1 => FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_1,
            2 => FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_2,
            3 => FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_0`"]
    #[inline]
    pub fn is_flexspi_clk_sel_0(&self) -> bool {
        *self == FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_1`"]
    #[inline]
    pub fn is_flexspi_clk_sel_1(&self) -> bool {
        *self == FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_2`"]
    #[inline]
    pub fn is_flexspi_clk_sel_2(&self) -> bool {
        *self == FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_3`"]
    #[inline]
    pub fn is_flexspi_clk_sel_3(&self) -> bool {
        *self == FLEXSPI_CLK_SELR::FLEXSPI_CLK_SEL_3
    }
}
#[doc = "Values that can be written to the field `PERCLK_PODF`"]
pub enum PERCLK_PODFW {
    #[doc = "divide by 1"]
    PERCLK_PODF_0,
    #[doc = "divide by 2"]
    PERCLK_PODF_1,
    #[doc = "divide by 3"]
    PERCLK_PODF_2,
    #[doc = "divide by 4"]
    PERCLK_PODF_3,
    #[doc = "divide by 5"]
    PERCLK_PODF_4,
    #[doc = "divide by 6"]
    PERCLK_PODF_5,
    #[doc = "divide by 7"]
    PERCLK_PODF_6,
    #[doc = "divide by 64"]
    PERCLK_PODF_63,
}
impl PERCLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERCLK_PODFW::PERCLK_PODF_0 => 0,
            PERCLK_PODFW::PERCLK_PODF_1 => 1,
            PERCLK_PODFW::PERCLK_PODF_2 => 2,
            PERCLK_PODFW::PERCLK_PODF_3 => 3,
            PERCLK_PODFW::PERCLK_PODF_4 => 4,
            PERCLK_PODFW::PERCLK_PODF_5 => 5,
            PERCLK_PODFW::PERCLK_PODF_6 => 6,
            PERCLK_PODFW::PERCLK_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERCLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _PERCLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERCLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn perclk_podf_0(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn perclk_podf_1(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn perclk_podf_2(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn perclk_podf_3(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn perclk_podf_4(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn perclk_podf_5(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn perclk_podf_6(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_6)
    }
    #[doc = "divide by 64"]
    #[inline]
    pub fn perclk_podf_63(self) -> &'a mut W {
        self.variant(PERCLK_PODFW::PERCLK_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PERCLK_CLK_SEL`"]
pub enum PERCLK_CLK_SELW {
    #[doc = "derive clock from ipg clk root"]
    PERCLK_CLK_SEL_0,
    #[doc = "derive clock from osc_clk"]
    PERCLK_CLK_SEL_1,
}
impl PERCLK_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERCLK_CLK_SELW::PERCLK_CLK_SEL_0 => false,
            PERCLK_CLK_SELW::PERCLK_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERCLK_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERCLK_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERCLK_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "derive clock from ipg clk root"]
    #[inline]
    pub fn perclk_clk_sel_0(self) -> &'a mut W {
        self.variant(PERCLK_CLK_SELW::PERCLK_CLK_SEL_0)
    }
    #[doc = "derive clock from osc_clk"]
    #[inline]
    pub fn perclk_clk_sel_1(self) -> &'a mut W {
        self.variant(PERCLK_CLK_SELW::PERCLK_CLK_SEL_1)
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
#[doc = "Values that can be written to the field `SAI1_CLK_SEL`"]
pub enum SAI1_CLK_SELW {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI1_CLK_SEL_0,
    #[doc = "derive clock from PLL5"]
    SAI1_CLK_SEL_1,
    #[doc = "derive clock from PLL4"]
    SAI1_CLK_SEL_2,
}
impl SAI1_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI1_CLK_SELW::SAI1_CLK_SEL_0 => 0,
            SAI1_CLK_SELW::SAI1_CLK_SEL_1 => 1,
            SAI1_CLK_SELW::SAI1_CLK_SEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline]
    pub fn sai1_clk_sel_0(self) -> &'a mut W {
        self.variant(SAI1_CLK_SELW::SAI1_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL5"]
    #[inline]
    pub fn sai1_clk_sel_1(self) -> &'a mut W {
        self.variant(SAI1_CLK_SELW::SAI1_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL4"]
    #[inline]
    pub fn sai1_clk_sel_2(self) -> &'a mut W {
        self.variant(SAI1_CLK_SELW::SAI1_CLK_SEL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI2_CLK_SEL`"]
pub enum SAI2_CLK_SELW {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI2_CLK_SEL_0,
    #[doc = "derive clock from PLL5"]
    SAI2_CLK_SEL_1,
    #[doc = "derive clock from PLL4"]
    SAI2_CLK_SEL_2,
}
impl SAI2_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI2_CLK_SELW::SAI2_CLK_SEL_0 => 0,
            SAI2_CLK_SELW::SAI2_CLK_SEL_1 => 1,
            SAI2_CLK_SELW::SAI2_CLK_SEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI2_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline]
    pub fn sai2_clk_sel_0(self) -> &'a mut W {
        self.variant(SAI2_CLK_SELW::SAI2_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL5"]
    #[inline]
    pub fn sai2_clk_sel_1(self) -> &'a mut W {
        self.variant(SAI2_CLK_SELW::SAI2_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL4"]
    #[inline]
    pub fn sai2_clk_sel_2(self) -> &'a mut W {
        self.variant(SAI2_CLK_SELW::SAI2_CLK_SEL_2)
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
#[doc = "Values that can be written to the field `SAI3_CLK_SEL`"]
pub enum SAI3_CLK_SELW {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI3_CLK_SEL_0,
    #[doc = "derive clock from PLL5"]
    SAI3_CLK_SEL_1,
    #[doc = "derive clock from PLL4"]
    SAI3_CLK_SEL_2,
}
impl SAI3_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI3_CLK_SELW::SAI3_CLK_SEL_0 => 0,
            SAI3_CLK_SELW::SAI3_CLK_SEL_1 => 1,
            SAI3_CLK_SELW::SAI3_CLK_SEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI3_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI3_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI3_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline]
    pub fn sai3_clk_sel_0(self) -> &'a mut W {
        self.variant(SAI3_CLK_SELW::SAI3_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL5"]
    #[inline]
    pub fn sai3_clk_sel_1(self) -> &'a mut W {
        self.variant(SAI3_CLK_SELW::SAI3_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL4"]
    #[inline]
    pub fn sai3_clk_sel_2(self) -> &'a mut W {
        self.variant(SAI3_CLK_SELW::SAI3_CLK_SEL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USDHC1_CLK_SEL`"]
pub enum USDHC1_CLK_SELW {
    #[doc = "derive clock from PLL2 PFD2"]
    USDHC1_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD0"]
    USDHC1_CLK_SEL_1,
}
impl USDHC1_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USDHC1_CLK_SELW::USDHC1_CLK_SEL_0 => false,
            USDHC1_CLK_SELW::USDHC1_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USDHC1_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USDHC1_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USDHC1_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline]
    pub fn usdhc1_clk_sel_0(self) -> &'a mut W {
        self.variant(USDHC1_CLK_SELW::USDHC1_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline]
    pub fn usdhc1_clk_sel_1(self) -> &'a mut W {
        self.variant(USDHC1_CLK_SELW::USDHC1_CLK_SEL_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USDHC2_CLK_SEL`"]
pub enum USDHC2_CLK_SELW {
    #[doc = "derive clock from PLL2 PFD2"]
    USDHC2_CLK_SEL_0,
    #[doc = "derive clock from PLL2 PFD0"]
    USDHC2_CLK_SEL_1,
}
impl USDHC2_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USDHC2_CLK_SELW::USDHC2_CLK_SEL_0 => false,
            USDHC2_CLK_SELW::USDHC2_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USDHC2_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USDHC2_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USDHC2_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline]
    pub fn usdhc2_clk_sel_0(self) -> &'a mut W {
        self.variant(USDHC2_CLK_SELW::USDHC2_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline]
    pub fn usdhc2_clk_sel_1(self) -> &'a mut W {
        self.variant(USDHC2_CLK_SELW::USDHC2_CLK_SEL_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLEXSPI_PODF`"]
pub enum FLEXSPI_PODFW {
    #[doc = "divide by 1"]
    FLEXSPI_PODF_0,
    #[doc = "divide by 2"]
    FLEXSPI_PODF_1,
    #[doc = "divide by 3"]
    FLEXSPI_PODF_2,
    #[doc = "divide by 4"]
    FLEXSPI_PODF_3,
    #[doc = "divide by 5"]
    FLEXSPI_PODF_4,
    #[doc = "divide by 6"]
    FLEXSPI_PODF_5,
    #[doc = "divide by 7"]
    FLEXSPI_PODF_6,
    #[doc = "divide by 8"]
    FLEXSPI_PODF_7,
}
impl FLEXSPI_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXSPI_PODFW::FLEXSPI_PODF_0 => 0,
            FLEXSPI_PODFW::FLEXSPI_PODF_1 => 1,
            FLEXSPI_PODFW::FLEXSPI_PODF_2 => 2,
            FLEXSPI_PODFW::FLEXSPI_PODF_3 => 3,
            FLEXSPI_PODFW::FLEXSPI_PODF_4 => 4,
            FLEXSPI_PODFW::FLEXSPI_PODF_5 => 5,
            FLEXSPI_PODFW::FLEXSPI_PODF_6 => 6,
            FLEXSPI_PODFW::FLEXSPI_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXSPI_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXSPI_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXSPI_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn flexspi_podf_0(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn flexspi_podf_1(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn flexspi_podf_2(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn flexspi_podf_3(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn flexspi_podf_4(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn flexspi_podf_5(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn flexspi_podf_6(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn flexspi_podf_7(self) -> &'a mut W {
        self.variant(FLEXSPI_PODFW::FLEXSPI_PODF_7)
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
#[doc = "Values that can be written to the field `FLEXSPI_CLK_SEL`"]
pub enum FLEXSPI_CLK_SELW {
    #[doc = "derive clock from semc_clk_root_pre"]
    FLEXSPI_CLK_SEL_0,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXSPI_CLK_SEL_1,
    #[doc = "derive clock from PLL2 PFD2"]
    FLEXSPI_CLK_SEL_2,
    #[doc = "derive clock from PLL3 PFD0"]
    FLEXSPI_CLK_SEL_3,
}
impl FLEXSPI_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_0 => 0,
            FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_1 => 1,
            FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_2 => 2,
            FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXSPI_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXSPI_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXSPI_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from semc_clk_root_pre"]
    #[inline]
    pub fn flexspi_clk_sel_0(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_0)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline]
    pub fn flexspi_clk_sel_1(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline]
    pub fn flexspi_clk_sel_2(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL3 PFD0"]
    #[inline]
    pub fn flexspi_clk_sel_3(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SELW::FLEXSPI_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:5 - Divider for perclk podf."]
    #[inline]
    pub fn perclk_podf(&self) -> PERCLK_PODFR {
        PERCLK_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Selector for the perclk clock multiplexor"]
    #[inline]
    pub fn perclk_clk_sel(&self) -> PERCLK_CLK_SELR {
        PERCLK_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Selector for sai1 clock multiplexer"]
    #[inline]
    pub fn sai1_clk_sel(&self) -> SAI1_CLK_SELR {
        SAI1_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Selector for sai2 clock multiplexer"]
    #[inline]
    pub fn sai2_clk_sel(&self) -> SAI2_CLK_SELR {
        SAI2_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Selector for sai3 clock multiplexer"]
    #[inline]
    pub fn sai3_clk_sel(&self) -> SAI3_CLK_SELR {
        SAI3_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Selector for usdhc1 clock multiplexer"]
    #[inline]
    pub fn usdhc1_clk_sel(&self) -> USDHC1_CLK_SELR {
        USDHC1_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Selector for usdhc2 clock multiplexer"]
    #[inline]
    pub fn usdhc2_clk_sel(&self) -> USDHC2_CLK_SELR {
        USDHC2_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:25 - Divider for flexspi clock root."]
    #[inline]
    pub fn flexspi_podf(&self) -> FLEXSPI_PODFR {
        FLEXSPI_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 29:30 - Selector for flexspi clock multiplexer"]
    #[inline]
    pub fn flexspi_clk_sel(&self) -> FLEXSPI_CLK_SELR {
        FLEXSPI_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 76546176 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Divider for perclk podf."]
    #[inline]
    pub fn perclk_podf(&mut self) -> _PERCLK_PODFW {
        _PERCLK_PODFW { w: self }
    }
    #[doc = "Bit 6 - Selector for the perclk clock multiplexor"]
    #[inline]
    pub fn perclk_clk_sel(&mut self) -> _PERCLK_CLK_SELW {
        _PERCLK_CLK_SELW { w: self }
    }
    #[doc = "Bits 10:11 - Selector for sai1 clock multiplexer"]
    #[inline]
    pub fn sai1_clk_sel(&mut self) -> _SAI1_CLK_SELW {
        _SAI1_CLK_SELW { w: self }
    }
    #[doc = "Bits 12:13 - Selector for sai2 clock multiplexer"]
    #[inline]
    pub fn sai2_clk_sel(&mut self) -> _SAI2_CLK_SELW {
        _SAI2_CLK_SELW { w: self }
    }
    #[doc = "Bits 14:15 - Selector for sai3 clock multiplexer"]
    #[inline]
    pub fn sai3_clk_sel(&mut self) -> _SAI3_CLK_SELW {
        _SAI3_CLK_SELW { w: self }
    }
    #[doc = "Bit 16 - Selector for usdhc1 clock multiplexer"]
    #[inline]
    pub fn usdhc1_clk_sel(&mut self) -> _USDHC1_CLK_SELW {
        _USDHC1_CLK_SELW { w: self }
    }
    #[doc = "Bit 17 - Selector for usdhc2 clock multiplexer"]
    #[inline]
    pub fn usdhc2_clk_sel(&mut self) -> _USDHC2_CLK_SELW {
        _USDHC2_CLK_SELW { w: self }
    }
    #[doc = "Bits 23:25 - Divider for flexspi clock root."]
    #[inline]
    pub fn flexspi_podf(&mut self) -> _FLEXSPI_PODFW {
        _FLEXSPI_PODFW { w: self }
    }
    #[doc = "Bits 29:30 - Selector for flexspi clock multiplexer"]
    #[inline]
    pub fn flexspi_clk_sel(&mut self) -> _FLEXSPI_CLK_SELW {
        _FLEXSPI_CLK_SELW { w: self }
    }
}
