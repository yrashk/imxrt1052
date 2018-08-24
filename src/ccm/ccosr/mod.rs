#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCOSR {
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
#[doc = "Possible values of the field `CLKO1_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO1_SELR {
    #[doc = "semc_clk_root"]
    CLKO1_SEL_5,
    #[doc = "enc_clk_root"]
    CLKO1_SEL_6,
    #[doc = "lcdif_pix_clk_root"]
    CLKO1_SEL_10,
    #[doc = "ahb_clk_root"]
    CLKO1_SEL_11,
    #[doc = "ipg_clk_root"]
    CLKO1_SEL_12,
    #[doc = "perclk_root"]
    CLKO1_SEL_13,
    #[doc = "ckil_sync_clk_root"]
    CLKO1_SEL_14,
    #[doc = "pll4_main_clk"]
    CLKO1_SEL_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKO1_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKO1_SELR::CLKO1_SEL_5 => 5,
            CLKO1_SELR::CLKO1_SEL_6 => 6,
            CLKO1_SELR::CLKO1_SEL_10 => 10,
            CLKO1_SELR::CLKO1_SEL_11 => 11,
            CLKO1_SELR::CLKO1_SEL_12 => 12,
            CLKO1_SELR::CLKO1_SEL_13 => 13,
            CLKO1_SELR::CLKO1_SEL_14 => 14,
            CLKO1_SELR::CLKO1_SEL_15 => 15,
            CLKO1_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKO1_SELR {
        match value {
            5 => CLKO1_SELR::CLKO1_SEL_5,
            6 => CLKO1_SELR::CLKO1_SEL_6,
            10 => CLKO1_SELR::CLKO1_SEL_10,
            11 => CLKO1_SELR::CLKO1_SEL_11,
            12 => CLKO1_SELR::CLKO1_SEL_12,
            13 => CLKO1_SELR::CLKO1_SEL_13,
            14 => CLKO1_SELR::CLKO1_SEL_14,
            15 => CLKO1_SELR::CLKO1_SEL_15,
            i => CLKO1_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_5`"]
    #[inline]
    pub fn is_clko1_sel_5(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_5
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_6`"]
    #[inline]
    pub fn is_clko1_sel_6(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_6
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_10`"]
    #[inline]
    pub fn is_clko1_sel_10(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_10
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_11`"]
    #[inline]
    pub fn is_clko1_sel_11(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_11
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_12`"]
    #[inline]
    pub fn is_clko1_sel_12(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_12
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_13`"]
    #[inline]
    pub fn is_clko1_sel_13(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_13
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_14`"]
    #[inline]
    pub fn is_clko1_sel_14(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_14
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_15`"]
    #[inline]
    pub fn is_clko1_sel_15(&self) -> bool {
        *self == CLKO1_SELR::CLKO1_SEL_15
    }
}
#[doc = "Possible values of the field `CLKO1_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO1_DIVR {
    #[doc = "divide by 1"]
    CLKO1_DIV_0,
    #[doc = "divide by 2"]
    CLKO1_DIV_1,
    #[doc = "divide by 3"]
    CLKO1_DIV_2,
    #[doc = "divide by 4"]
    CLKO1_DIV_3,
    #[doc = "divide by 5"]
    CLKO1_DIV_4,
    #[doc = "divide by 6"]
    CLKO1_DIV_5,
    #[doc = "divide by 7"]
    CLKO1_DIV_6,
    #[doc = "divide by 8"]
    CLKO1_DIV_7,
}
impl CLKO1_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKO1_DIVR::CLKO1_DIV_0 => 0,
            CLKO1_DIVR::CLKO1_DIV_1 => 1,
            CLKO1_DIVR::CLKO1_DIV_2 => 2,
            CLKO1_DIVR::CLKO1_DIV_3 => 3,
            CLKO1_DIVR::CLKO1_DIV_4 => 4,
            CLKO1_DIVR::CLKO1_DIV_5 => 5,
            CLKO1_DIVR::CLKO1_DIV_6 => 6,
            CLKO1_DIVR::CLKO1_DIV_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKO1_DIVR {
        match value {
            0 => CLKO1_DIVR::CLKO1_DIV_0,
            1 => CLKO1_DIVR::CLKO1_DIV_1,
            2 => CLKO1_DIVR::CLKO1_DIV_2,
            3 => CLKO1_DIVR::CLKO1_DIV_3,
            4 => CLKO1_DIVR::CLKO1_DIV_4,
            5 => CLKO1_DIVR::CLKO1_DIV_5,
            6 => CLKO1_DIVR::CLKO1_DIV_6,
            7 => CLKO1_DIVR::CLKO1_DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_0`"]
    #[inline]
    pub fn is_clko1_div_0(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_1`"]
    #[inline]
    pub fn is_clko1_div_1(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_2`"]
    #[inline]
    pub fn is_clko1_div_2(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_3`"]
    #[inline]
    pub fn is_clko1_div_3(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_4`"]
    #[inline]
    pub fn is_clko1_div_4(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_5`"]
    #[inline]
    pub fn is_clko1_div_5(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_6`"]
    #[inline]
    pub fn is_clko1_div_6(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_6
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_7`"]
    #[inline]
    pub fn is_clko1_div_7(&self) -> bool {
        *self == CLKO1_DIVR::CLKO1_DIV_7
    }
}
#[doc = "Possible values of the field `CLKO1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO1_ENR {
    #[doc = "CCM_CLKO1 disabled."]
    CLKO1_EN_0,
    #[doc = "CCM_CLKO1 enabled."]
    CLKO1_EN_1,
}
impl CLKO1_ENR {
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
            CLKO1_ENR::CLKO1_EN_0 => false,
            CLKO1_ENR::CLKO1_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKO1_ENR {
        match value {
            false => CLKO1_ENR::CLKO1_EN_0,
            true => CLKO1_ENR::CLKO1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_EN_0`"]
    #[inline]
    pub fn is_clko1_en_0(&self) -> bool {
        *self == CLKO1_ENR::CLKO1_EN_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_EN_1`"]
    #[inline]
    pub fn is_clko1_en_1(&self) -> bool {
        *self == CLKO1_ENR::CLKO1_EN_1
    }
}
#[doc = "Possible values of the field `CLK_OUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_OUT_SELR {
    #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock"]
    CLK_OUT_SEL_0,
    #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock"]
    CLK_OUT_SEL_1,
}
impl CLK_OUT_SELR {
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
            CLK_OUT_SELR::CLK_OUT_SEL_0 => false,
            CLK_OUT_SELR::CLK_OUT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLK_OUT_SELR {
        match value {
            false => CLK_OUT_SELR::CLK_OUT_SEL_0,
            true => CLK_OUT_SELR::CLK_OUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_OUT_SEL_0`"]
    #[inline]
    pub fn is_clk_out_sel_0(&self) -> bool {
        *self == CLK_OUT_SELR::CLK_OUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLK_OUT_SEL_1`"]
    #[inline]
    pub fn is_clk_out_sel_1(&self) -> bool {
        *self == CLK_OUT_SELR::CLK_OUT_SEL_1
    }
}
#[doc = "Possible values of the field `CLKO2_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO2_SELR {
    #[doc = "usdhc1_clk_root"]
    CLKO2_SEL_3,
    #[doc = "wrck_clk_root"]
    CLKO2_SEL_5,
    #[doc = "lpi2c_clk_root"]
    CLKO2_SEL_6,
    #[doc = "csi_core"]
    CLKO2_SEL_11,
    #[doc = "osc_clk"]
    CLKO2_SEL_14,
    #[doc = "usdhc2_clk_root"]
    CLKO2_SEL_17,
    #[doc = "sai1_clk_root"]
    CLKO2_SEL_18,
    #[doc = "sai2_clk_root"]
    CLKO2_SEL_19,
    #[doc = "sai3_clk_root"]
    CLKO2_SEL_20,
    #[doc = "can_clk_root"]
    CLKO2_SEL_23,
    #[doc = "flexspi_clk_root"]
    CLKO2_SEL_27,
    #[doc = "uart_clk_root"]
    CLKO2_SEL_28,
    #[doc = "spdif0_clk_root"]
    CLKO2_SEL_29,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKO2_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKO2_SELR::CLKO2_SEL_3 => 3,
            CLKO2_SELR::CLKO2_SEL_5 => 5,
            CLKO2_SELR::CLKO2_SEL_6 => 6,
            CLKO2_SELR::CLKO2_SEL_11 => 11,
            CLKO2_SELR::CLKO2_SEL_14 => 14,
            CLKO2_SELR::CLKO2_SEL_17 => 17,
            CLKO2_SELR::CLKO2_SEL_18 => 18,
            CLKO2_SELR::CLKO2_SEL_19 => 19,
            CLKO2_SELR::CLKO2_SEL_20 => 20,
            CLKO2_SELR::CLKO2_SEL_23 => 23,
            CLKO2_SELR::CLKO2_SEL_27 => 27,
            CLKO2_SELR::CLKO2_SEL_28 => 28,
            CLKO2_SELR::CLKO2_SEL_29 => 29,
            CLKO2_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKO2_SELR {
        match value {
            3 => CLKO2_SELR::CLKO2_SEL_3,
            5 => CLKO2_SELR::CLKO2_SEL_5,
            6 => CLKO2_SELR::CLKO2_SEL_6,
            11 => CLKO2_SELR::CLKO2_SEL_11,
            14 => CLKO2_SELR::CLKO2_SEL_14,
            17 => CLKO2_SELR::CLKO2_SEL_17,
            18 => CLKO2_SELR::CLKO2_SEL_18,
            19 => CLKO2_SELR::CLKO2_SEL_19,
            20 => CLKO2_SELR::CLKO2_SEL_20,
            23 => CLKO2_SELR::CLKO2_SEL_23,
            27 => CLKO2_SELR::CLKO2_SEL_27,
            28 => CLKO2_SELR::CLKO2_SEL_28,
            29 => CLKO2_SELR::CLKO2_SEL_29,
            i => CLKO2_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_3`"]
    #[inline]
    pub fn is_clko2_sel_3(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_3
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_5`"]
    #[inline]
    pub fn is_clko2_sel_5(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_5
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_6`"]
    #[inline]
    pub fn is_clko2_sel_6(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_6
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_11`"]
    #[inline]
    pub fn is_clko2_sel_11(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_11
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_14`"]
    #[inline]
    pub fn is_clko2_sel_14(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_14
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_17`"]
    #[inline]
    pub fn is_clko2_sel_17(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_17
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_18`"]
    #[inline]
    pub fn is_clko2_sel_18(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_18
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_19`"]
    #[inline]
    pub fn is_clko2_sel_19(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_19
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_20`"]
    #[inline]
    pub fn is_clko2_sel_20(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_20
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_23`"]
    #[inline]
    pub fn is_clko2_sel_23(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_23
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_27`"]
    #[inline]
    pub fn is_clko2_sel_27(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_27
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_28`"]
    #[inline]
    pub fn is_clko2_sel_28(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_28
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_29`"]
    #[inline]
    pub fn is_clko2_sel_29(&self) -> bool {
        *self == CLKO2_SELR::CLKO2_SEL_29
    }
}
#[doc = "Possible values of the field `CLKO2_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO2_DIVR {
    #[doc = "divide by 1"]
    CLKO2_DIV_0,
    #[doc = "divide by 2"]
    CLKO2_DIV_1,
    #[doc = "divide by 3"]
    CLKO2_DIV_2,
    #[doc = "divide by 4"]
    CLKO2_DIV_3,
    #[doc = "divide by 5"]
    CLKO2_DIV_4,
    #[doc = "divide by 6"]
    CLKO2_DIV_5,
    #[doc = "divide by 7"]
    CLKO2_DIV_6,
    #[doc = "divide by 8"]
    CLKO2_DIV_7,
}
impl CLKO2_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKO2_DIVR::CLKO2_DIV_0 => 0,
            CLKO2_DIVR::CLKO2_DIV_1 => 1,
            CLKO2_DIVR::CLKO2_DIV_2 => 2,
            CLKO2_DIVR::CLKO2_DIV_3 => 3,
            CLKO2_DIVR::CLKO2_DIV_4 => 4,
            CLKO2_DIVR::CLKO2_DIV_5 => 5,
            CLKO2_DIVR::CLKO2_DIV_6 => 6,
            CLKO2_DIVR::CLKO2_DIV_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKO2_DIVR {
        match value {
            0 => CLKO2_DIVR::CLKO2_DIV_0,
            1 => CLKO2_DIVR::CLKO2_DIV_1,
            2 => CLKO2_DIVR::CLKO2_DIV_2,
            3 => CLKO2_DIVR::CLKO2_DIV_3,
            4 => CLKO2_DIVR::CLKO2_DIV_4,
            5 => CLKO2_DIVR::CLKO2_DIV_5,
            6 => CLKO2_DIVR::CLKO2_DIV_6,
            7 => CLKO2_DIVR::CLKO2_DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_0`"]
    #[inline]
    pub fn is_clko2_div_0(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_1`"]
    #[inline]
    pub fn is_clko2_div_1(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_2`"]
    #[inline]
    pub fn is_clko2_div_2(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_3`"]
    #[inline]
    pub fn is_clko2_div_3(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_4`"]
    #[inline]
    pub fn is_clko2_div_4(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_5`"]
    #[inline]
    pub fn is_clko2_div_5(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_6`"]
    #[inline]
    pub fn is_clko2_div_6(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_6
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_7`"]
    #[inline]
    pub fn is_clko2_div_7(&self) -> bool {
        *self == CLKO2_DIVR::CLKO2_DIV_7
    }
}
#[doc = "Possible values of the field `CLKO2_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO2_ENR {
    #[doc = "CCM_CLKO2 disabled."]
    CLKO2_EN_0,
    #[doc = "CCM_CLKO2 enabled."]
    CLKO2_EN_1,
}
impl CLKO2_ENR {
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
            CLKO2_ENR::CLKO2_EN_0 => false,
            CLKO2_ENR::CLKO2_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKO2_ENR {
        match value {
            false => CLKO2_ENR::CLKO2_EN_0,
            true => CLKO2_ENR::CLKO2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_EN_0`"]
    #[inline]
    pub fn is_clko2_en_0(&self) -> bool {
        *self == CLKO2_ENR::CLKO2_EN_0
    }
    #[doc = "Checks if the value of the field is `CLKO2_EN_1`"]
    #[inline]
    pub fn is_clko2_en_1(&self) -> bool {
        *self == CLKO2_ENR::CLKO2_EN_1
    }
}
#[doc = "Values that can be written to the field `CLKO1_SEL`"]
pub enum CLKO1_SELW {
    #[doc = "semc_clk_root"]
    CLKO1_SEL_5,
    #[doc = "enc_clk_root"]
    CLKO1_SEL_6,
    #[doc = "lcdif_pix_clk_root"]
    CLKO1_SEL_10,
    #[doc = "ahb_clk_root"]
    CLKO1_SEL_11,
    #[doc = "ipg_clk_root"]
    CLKO1_SEL_12,
    #[doc = "perclk_root"]
    CLKO1_SEL_13,
    #[doc = "ckil_sync_clk_root"]
    CLKO1_SEL_14,
    #[doc = "pll4_main_clk"]
    CLKO1_SEL_15,
}
impl CLKO1_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKO1_SELW::CLKO1_SEL_5 => 5,
            CLKO1_SELW::CLKO1_SEL_6 => 6,
            CLKO1_SELW::CLKO1_SEL_10 => 10,
            CLKO1_SELW::CLKO1_SEL_11 => 11,
            CLKO1_SELW::CLKO1_SEL_12 => 12,
            CLKO1_SELW::CLKO1_SEL_13 => 13,
            CLKO1_SELW::CLKO1_SEL_14 => 14,
            CLKO1_SELW::CLKO1_SEL_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKO1_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKO1_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKO1_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "semc_clk_root"]
    #[inline]
    pub fn clko1_sel_5(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_5)
    }
    #[doc = "enc_clk_root"]
    #[inline]
    pub fn clko1_sel_6(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_6)
    }
    #[doc = "lcdif_pix_clk_root"]
    #[inline]
    pub fn clko1_sel_10(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_10)
    }
    #[doc = "ahb_clk_root"]
    #[inline]
    pub fn clko1_sel_11(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_11)
    }
    #[doc = "ipg_clk_root"]
    #[inline]
    pub fn clko1_sel_12(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_12)
    }
    #[doc = "perclk_root"]
    #[inline]
    pub fn clko1_sel_13(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_13)
    }
    #[doc = "ckil_sync_clk_root"]
    #[inline]
    pub fn clko1_sel_14(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_14)
    }
    #[doc = "pll4_main_clk"]
    #[inline]
    pub fn clko1_sel_15(self) -> &'a mut W {
        self.variant(CLKO1_SELW::CLKO1_SEL_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKO1_DIV`"]
pub enum CLKO1_DIVW {
    #[doc = "divide by 1"]
    CLKO1_DIV_0,
    #[doc = "divide by 2"]
    CLKO1_DIV_1,
    #[doc = "divide by 3"]
    CLKO1_DIV_2,
    #[doc = "divide by 4"]
    CLKO1_DIV_3,
    #[doc = "divide by 5"]
    CLKO1_DIV_4,
    #[doc = "divide by 6"]
    CLKO1_DIV_5,
    #[doc = "divide by 7"]
    CLKO1_DIV_6,
    #[doc = "divide by 8"]
    CLKO1_DIV_7,
}
impl CLKO1_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKO1_DIVW::CLKO1_DIV_0 => 0,
            CLKO1_DIVW::CLKO1_DIV_1 => 1,
            CLKO1_DIVW::CLKO1_DIV_2 => 2,
            CLKO1_DIVW::CLKO1_DIV_3 => 3,
            CLKO1_DIVW::CLKO1_DIV_4 => 4,
            CLKO1_DIVW::CLKO1_DIV_5 => 5,
            CLKO1_DIVW::CLKO1_DIV_6 => 6,
            CLKO1_DIVW::CLKO1_DIV_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKO1_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKO1_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKO1_DIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn clko1_div_0(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn clko1_div_1(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn clko1_div_2(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn clko1_div_3(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn clko1_div_4(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn clko1_div_5(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn clko1_div_6(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn clko1_div_7(self) -> &'a mut W {
        self.variant(CLKO1_DIVW::CLKO1_DIV_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKO1_EN`"]
pub enum CLKO1_ENW {
    #[doc = "CCM_CLKO1 disabled."]
    CLKO1_EN_0,
    #[doc = "CCM_CLKO1 enabled."]
    CLKO1_EN_1,
}
impl CLKO1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKO1_ENW::CLKO1_EN_0 => false,
            CLKO1_ENW::CLKO1_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKO1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKO1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKO1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCM_CLKO1 disabled."]
    #[inline]
    pub fn clko1_en_0(self) -> &'a mut W {
        self.variant(CLKO1_ENW::CLKO1_EN_0)
    }
    #[doc = "CCM_CLKO1 enabled."]
    #[inline]
    pub fn clko1_en_1(self) -> &'a mut W {
        self.variant(CLKO1_ENW::CLKO1_EN_1)
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
#[doc = "Values that can be written to the field `CLK_OUT_SEL`"]
pub enum CLK_OUT_SELW {
    #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock"]
    CLK_OUT_SEL_0,
    #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock"]
    CLK_OUT_SEL_1,
}
impl CLK_OUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLK_OUT_SELW::CLK_OUT_SEL_0 => false,
            CLK_OUT_SELW::CLK_OUT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_OUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_OUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_OUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock"]
    #[inline]
    pub fn clk_out_sel_0(self) -> &'a mut W {
        self.variant(CLK_OUT_SELW::CLK_OUT_SEL_0)
    }
    #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock"]
    #[inline]
    pub fn clk_out_sel_1(self) -> &'a mut W {
        self.variant(CLK_OUT_SELW::CLK_OUT_SEL_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKO2_SEL`"]
pub enum CLKO2_SELW {
    #[doc = "usdhc1_clk_root"]
    CLKO2_SEL_3,
    #[doc = "wrck_clk_root"]
    CLKO2_SEL_5,
    #[doc = "lpi2c_clk_root"]
    CLKO2_SEL_6,
    #[doc = "csi_core"]
    CLKO2_SEL_11,
    #[doc = "osc_clk"]
    CLKO2_SEL_14,
    #[doc = "usdhc2_clk_root"]
    CLKO2_SEL_17,
    #[doc = "sai1_clk_root"]
    CLKO2_SEL_18,
    #[doc = "sai2_clk_root"]
    CLKO2_SEL_19,
    #[doc = "sai3_clk_root"]
    CLKO2_SEL_20,
    #[doc = "can_clk_root"]
    CLKO2_SEL_23,
    #[doc = "flexspi_clk_root"]
    CLKO2_SEL_27,
    #[doc = "uart_clk_root"]
    CLKO2_SEL_28,
    #[doc = "spdif0_clk_root"]
    CLKO2_SEL_29,
}
impl CLKO2_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKO2_SELW::CLKO2_SEL_3 => 3,
            CLKO2_SELW::CLKO2_SEL_5 => 5,
            CLKO2_SELW::CLKO2_SEL_6 => 6,
            CLKO2_SELW::CLKO2_SEL_11 => 11,
            CLKO2_SELW::CLKO2_SEL_14 => 14,
            CLKO2_SELW::CLKO2_SEL_17 => 17,
            CLKO2_SELW::CLKO2_SEL_18 => 18,
            CLKO2_SELW::CLKO2_SEL_19 => 19,
            CLKO2_SELW::CLKO2_SEL_20 => 20,
            CLKO2_SELW::CLKO2_SEL_23 => 23,
            CLKO2_SELW::CLKO2_SEL_27 => 27,
            CLKO2_SELW::CLKO2_SEL_28 => 28,
            CLKO2_SELW::CLKO2_SEL_29 => 29,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKO2_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKO2_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKO2_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "usdhc1_clk_root"]
    #[inline]
    pub fn clko2_sel_3(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_3)
    }
    #[doc = "wrck_clk_root"]
    #[inline]
    pub fn clko2_sel_5(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_5)
    }
    #[doc = "lpi2c_clk_root"]
    #[inline]
    pub fn clko2_sel_6(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_6)
    }
    #[doc = "csi_core"]
    #[inline]
    pub fn clko2_sel_11(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_11)
    }
    #[doc = "osc_clk"]
    #[inline]
    pub fn clko2_sel_14(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_14)
    }
    #[doc = "usdhc2_clk_root"]
    #[inline]
    pub fn clko2_sel_17(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_17)
    }
    #[doc = "sai1_clk_root"]
    #[inline]
    pub fn clko2_sel_18(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_18)
    }
    #[doc = "sai2_clk_root"]
    #[inline]
    pub fn clko2_sel_19(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_19)
    }
    #[doc = "sai3_clk_root"]
    #[inline]
    pub fn clko2_sel_20(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_20)
    }
    #[doc = "can_clk_root"]
    #[inline]
    pub fn clko2_sel_23(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_23)
    }
    #[doc = "flexspi_clk_root"]
    #[inline]
    pub fn clko2_sel_27(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_27)
    }
    #[doc = "uart_clk_root"]
    #[inline]
    pub fn clko2_sel_28(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_28)
    }
    #[doc = "spdif0_clk_root"]
    #[inline]
    pub fn clko2_sel_29(self) -> &'a mut W {
        self.variant(CLKO2_SELW::CLKO2_SEL_29)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKO2_DIV`"]
pub enum CLKO2_DIVW {
    #[doc = "divide by 1"]
    CLKO2_DIV_0,
    #[doc = "divide by 2"]
    CLKO2_DIV_1,
    #[doc = "divide by 3"]
    CLKO2_DIV_2,
    #[doc = "divide by 4"]
    CLKO2_DIV_3,
    #[doc = "divide by 5"]
    CLKO2_DIV_4,
    #[doc = "divide by 6"]
    CLKO2_DIV_5,
    #[doc = "divide by 7"]
    CLKO2_DIV_6,
    #[doc = "divide by 8"]
    CLKO2_DIV_7,
}
impl CLKO2_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKO2_DIVW::CLKO2_DIV_0 => 0,
            CLKO2_DIVW::CLKO2_DIV_1 => 1,
            CLKO2_DIVW::CLKO2_DIV_2 => 2,
            CLKO2_DIVW::CLKO2_DIV_3 => 3,
            CLKO2_DIVW::CLKO2_DIV_4 => 4,
            CLKO2_DIVW::CLKO2_DIV_5 => 5,
            CLKO2_DIVW::CLKO2_DIV_6 => 6,
            CLKO2_DIVW::CLKO2_DIV_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKO2_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKO2_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKO2_DIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn clko2_div_0(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn clko2_div_1(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn clko2_div_2(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn clko2_div_3(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn clko2_div_4(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn clko2_div_5(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn clko2_div_6(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn clko2_div_7(self) -> &'a mut W {
        self.variant(CLKO2_DIVW::CLKO2_DIV_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKO2_EN`"]
pub enum CLKO2_ENW {
    #[doc = "CCM_CLKO2 disabled."]
    CLKO2_EN_0,
    #[doc = "CCM_CLKO2 enabled."]
    CLKO2_EN_1,
}
impl CLKO2_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKO2_ENW::CLKO2_EN_0 => false,
            CLKO2_ENW::CLKO2_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKO2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKO2_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKO2_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCM_CLKO2 disabled."]
    #[inline]
    pub fn clko2_en_0(self) -> &'a mut W {
        self.variant(CLKO2_ENW::CLKO2_EN_0)
    }
    #[doc = "CCM_CLKO2 enabled."]
    #[inline]
    pub fn clko2_en_1(self) -> &'a mut W {
        self.variant(CLKO2_ENW::CLKO2_EN_1)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - Selection of the clock to be generated on CCM_CLKO1"]
    #[inline]
    pub fn clko1_sel(&self) -> CLKO1_SELR {
        CLKO1_SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Setting the divider of CCM_CLKO1"]
    #[inline]
    pub fn clko1_div(&self) -> CLKO1_DIVR {
        CLKO1_DIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Enable of CCM_CLKO1 clock"]
    #[inline]
    pub fn clko1_en(&self) -> CLKO1_ENR {
        CLKO1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline]
    pub fn clk_out_sel(&self) -> CLK_OUT_SELR {
        CLK_OUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Selection of the clock to be generated on CCM_CLKO2"]
    #[inline]
    pub fn clko2_sel(&self) -> CLKO2_SELR {
        CLKO2_SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:23 - Setting the divider of CCM_CLKO2"]
    #[inline]
    pub fn clko2_div(&self) -> CLKO2_DIVR {
        CLKO2_DIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Enable of CCM_CLKO2 clock"]
    #[inline]
    pub fn clko2_en(&self) -> CLKO2_ENR {
        CLKO2_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 655361 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Selection of the clock to be generated on CCM_CLKO1"]
    #[inline]
    pub fn clko1_sel(&mut self) -> _CLKO1_SELW {
        _CLKO1_SELW { w: self }
    }
    #[doc = "Bits 4:6 - Setting the divider of CCM_CLKO1"]
    #[inline]
    pub fn clko1_div(&mut self) -> _CLKO1_DIVW {
        _CLKO1_DIVW { w: self }
    }
    #[doc = "Bit 7 - Enable of CCM_CLKO1 clock"]
    #[inline]
    pub fn clko1_en(&mut self) -> _CLKO1_ENW {
        _CLKO1_ENW { w: self }
    }
    #[doc = "Bit 8 - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline]
    pub fn clk_out_sel(&mut self) -> _CLK_OUT_SELW {
        _CLK_OUT_SELW { w: self }
    }
    #[doc = "Bits 16:20 - Selection of the clock to be generated on CCM_CLKO2"]
    #[inline]
    pub fn clko2_sel(&mut self) -> _CLKO2_SELW {
        _CLKO2_SELW { w: self }
    }
    #[doc = "Bits 21:23 - Setting the divider of CCM_CLKO2"]
    #[inline]
    pub fn clko2_div(&mut self) -> _CLKO2_DIVW {
        _CLKO2_DIVW { w: self }
    }
    #[doc = "Bit 24 - Enable of CCM_CLKO2 clock"]
    #[inline]
    pub fn clko2_en(&mut self) -> _CLKO2_ENW {
        _CLKO2_ENW { w: self }
    }
}
