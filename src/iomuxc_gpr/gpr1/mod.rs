#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR1 {
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
#[doc = "Possible values of the field `SAI1_MCLK1_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_MCLK1_SELR {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0,
    #[doc = "ccm.ssi2_clk_root"]
    SAI1_MCLK1_SEL_1,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK1_SEL_3,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK1_SEL_4,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK1_SEL_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI1_MCLK1_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_0 => 0,
            SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_1 => 1,
            SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_2 => 2,
            SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_3 => 3,
            SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_4 => 4,
            SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_5 => 5,
            SAI1_MCLK1_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI1_MCLK1_SELR {
        match value {
            0 => SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_0,
            1 => SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_1,
            2 => SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_2,
            3 => SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_3,
            4 => SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_4,
            5 => SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_5,
            i => SAI1_MCLK1_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_0`"]
    #[inline]
    pub fn is_sai1_mclk1_sel_0(&self) -> bool {
        *self == SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_1`"]
    #[inline]
    pub fn is_sai1_mclk1_sel_1(&self) -> bool {
        *self == SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_2`"]
    #[inline]
    pub fn is_sai1_mclk1_sel_2(&self) -> bool {
        *self == SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_3`"]
    #[inline]
    pub fn is_sai1_mclk1_sel_3(&self) -> bool {
        *self == SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_3
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_4`"]
    #[inline]
    pub fn is_sai1_mclk1_sel_4(&self) -> bool {
        *self == SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_4
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_5`"]
    #[inline]
    pub fn is_sai1_mclk1_sel_5(&self) -> bool {
        *self == SAI1_MCLK1_SELR::SAI1_MCLK1_SEL_5
    }
}
#[doc = "Possible values of the field `SAI1_MCLK2_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_MCLK2_SELR {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0,
    #[doc = "ccm.ssi2_clk_root"]
    SAI1_MCLK2_SEL_1,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK2_SEL_3,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK2_SEL_4,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK2_SEL_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI1_MCLK2_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_0 => 0,
            SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_1 => 1,
            SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_2 => 2,
            SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_3 => 3,
            SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_4 => 4,
            SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_5 => 5,
            SAI1_MCLK2_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI1_MCLK2_SELR {
        match value {
            0 => SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_0,
            1 => SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_1,
            2 => SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_2,
            3 => SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_3,
            4 => SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_4,
            5 => SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_5,
            i => SAI1_MCLK2_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_0`"]
    #[inline]
    pub fn is_sai1_mclk2_sel_0(&self) -> bool {
        *self == SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_1`"]
    #[inline]
    pub fn is_sai1_mclk2_sel_1(&self) -> bool {
        *self == SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_2`"]
    #[inline]
    pub fn is_sai1_mclk2_sel_2(&self) -> bool {
        *self == SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_3`"]
    #[inline]
    pub fn is_sai1_mclk2_sel_3(&self) -> bool {
        *self == SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_3
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_4`"]
    #[inline]
    pub fn is_sai1_mclk2_sel_4(&self) -> bool {
        *self == SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_4
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_5`"]
    #[inline]
    pub fn is_sai1_mclk2_sel_5(&self) -> bool {
        *self == SAI1_MCLK2_SELR::SAI1_MCLK2_SEL_5
    }
}
#[doc = "Possible values of the field `SAI1_MCLK3_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_MCLK3_SELR {
    #[doc = "ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI1_MCLK3_SEL_1,
    #[doc = "spdif.spdif_srclk"]
    SAI1_MCLK3_SEL_2,
    #[doc = "spdif.spdif_outclock"]
    SAI1_MCLK3_SEL_3,
}
impl SAI1_MCLK3_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_0 => 0,
            SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_1 => 1,
            SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_2 => 2,
            SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI1_MCLK3_SELR {
        match value {
            0 => SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_0,
            1 => SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_1,
            2 => SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_2,
            3 => SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_0`"]
    #[inline]
    pub fn is_sai1_mclk3_sel_0(&self) -> bool {
        *self == SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_1`"]
    #[inline]
    pub fn is_sai1_mclk3_sel_1(&self) -> bool {
        *self == SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_2`"]
    #[inline]
    pub fn is_sai1_mclk3_sel_2(&self) -> bool {
        *self == SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_3`"]
    #[inline]
    pub fn is_sai1_mclk3_sel_3(&self) -> bool {
        *self == SAI1_MCLK3_SELR::SAI1_MCLK3_SEL_3
    }
}
#[doc = "Possible values of the field `SAI2_MCLK3_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_MCLK3_SELR {
    #[doc = "ccm.spdif0_clk_root"]
    SAI2_MCLK3_SEL_0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI2_MCLK3_SEL_1,
    #[doc = "spdif.spdif_srclk"]
    SAI2_MCLK3_SEL_2,
    #[doc = "spdif.spdif_outclock"]
    SAI2_MCLK3_SEL_3,
}
impl SAI2_MCLK3_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_0 => 0,
            SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_1 => 1,
            SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_2 => 2,
            SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI2_MCLK3_SELR {
        match value {
            0 => SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_0,
            1 => SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_1,
            2 => SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_2,
            3 => SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_0`"]
    #[inline]
    pub fn is_sai2_mclk3_sel_0(&self) -> bool {
        *self == SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_1`"]
    #[inline]
    pub fn is_sai2_mclk3_sel_1(&self) -> bool {
        *self == SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_2`"]
    #[inline]
    pub fn is_sai2_mclk3_sel_2(&self) -> bool {
        *self == SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_3`"]
    #[inline]
    pub fn is_sai2_mclk3_sel_3(&self) -> bool {
        *self == SAI2_MCLK3_SELR::SAI2_MCLK3_SEL_3
    }
}
#[doc = "Possible values of the field `SAI3_MCLK3_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_MCLK3_SELR {
    #[doc = "ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI3_MCLK3_SEL_1,
    #[doc = "spdif.spdif_srclk"]
    SAI3_MCLK3_SEL_2,
    #[doc = "spdif.spdif_outclock"]
    SAI3_MCLK3_SEL_3,
}
impl SAI3_MCLK3_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_0 => 0,
            SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_1 => 1,
            SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_2 => 2,
            SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI3_MCLK3_SELR {
        match value {
            0 => SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_0,
            1 => SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_1,
            2 => SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_2,
            3 => SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_0`"]
    #[inline]
    pub fn is_sai3_mclk3_sel_0(&self) -> bool {
        *self == SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_1`"]
    #[inline]
    pub fn is_sai3_mclk3_sel_1(&self) -> bool {
        *self == SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_2`"]
    #[inline]
    pub fn is_sai3_mclk3_sel_2(&self) -> bool {
        *self == SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_3`"]
    #[inline]
    pub fn is_sai3_mclk3_sel_3(&self) -> bool {
        *self == SAI3_MCLK3_SELR::SAI3_MCLK3_SEL_3
    }
}
#[doc = "Possible values of the field `GINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINTR {
    #[doc = "Global interrupt request is not asserted."]
    GINT_0,
    #[doc = "Global interrupt request is asserted."]
    GINT_1,
}
impl GINTR {
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
            GINTR::GINT_0 => false,
            GINTR::GINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GINTR {
        match value {
            false => GINTR::GINT_0,
            true => GINTR::GINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `GINT_0`"]
    #[inline]
    pub fn is_gint_0(&self) -> bool {
        *self == GINTR::GINT_0
    }
    #[doc = "Checks if the value of the field is `GINT_1`"]
    #[inline]
    pub fn is_gint_1(&self) -> bool {
        *self == GINTR::GINT_1
    }
}
#[doc = "Possible values of the field `ENET1_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET1_CLK_SELR {
    #[doc = "ENET1 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET_REF_CLK1 function."]
    ENET1_CLK_SEL_0,
    #[doc = "Gets ENET1 TX reference clock from the ENET1_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    ENET1_CLK_SEL_1,
}
impl ENET1_CLK_SELR {
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
            ENET1_CLK_SELR::ENET1_CLK_SEL_0 => false,
            ENET1_CLK_SELR::ENET1_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENET1_CLK_SELR {
        match value {
            false => ENET1_CLK_SELR::ENET1_CLK_SEL_0,
            true => ENET1_CLK_SELR::ENET1_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET1_CLK_SEL_0`"]
    #[inline]
    pub fn is_enet1_clk_sel_0(&self) -> bool {
        *self == ENET1_CLK_SELR::ENET1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET1_CLK_SEL_1`"]
    #[inline]
    pub fn is_enet1_clk_sel_1(&self) -> bool {
        *self == ENET1_CLK_SELR::ENET1_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `USB_EXP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EXP_MODER {
    #[doc = "Exposure mode is disabled."]
    USB_EXP_MODE_0,
    #[doc = "Exposure mode is enabled."]
    USB_EXP_MODE_1,
}
impl USB_EXP_MODER {
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
            USB_EXP_MODER::USB_EXP_MODE_0 => false,
            USB_EXP_MODER::USB_EXP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_EXP_MODER {
        match value {
            false => USB_EXP_MODER::USB_EXP_MODE_0,
            true => USB_EXP_MODER::USB_EXP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `USB_EXP_MODE_0`"]
    #[inline]
    pub fn is_usb_exp_mode_0(&self) -> bool {
        *self == USB_EXP_MODER::USB_EXP_MODE_0
    }
    #[doc = "Checks if the value of the field is `USB_EXP_MODE_1`"]
    #[inline]
    pub fn is_usb_exp_mode_1(&self) -> bool {
        *self == USB_EXP_MODER::USB_EXP_MODE_1
    }
}
#[doc = "Possible values of the field `ENET1_TX_CLK_DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET1_TX_CLK_DIRR {
    #[doc = "ENET1_TX_CLK output driver is disabled when configured for ALT1"]
    ENET1_TX_CLK_DIR_0,
    #[doc = "ENET1_TX_CLK output driver is enabled when configured for ALT1"]
    ENET1_TX_CLK_DIR_1,
}
impl ENET1_TX_CLK_DIRR {
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
            ENET1_TX_CLK_DIRR::ENET1_TX_CLK_DIR_0 => false,
            ENET1_TX_CLK_DIRR::ENET1_TX_CLK_DIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENET1_TX_CLK_DIRR {
        match value {
            false => ENET1_TX_CLK_DIRR::ENET1_TX_CLK_DIR_0,
            true => ENET1_TX_CLK_DIRR::ENET1_TX_CLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET1_TX_CLK_DIR_0`"]
    #[inline]
    pub fn is_enet1_tx_clk_dir_0(&self) -> bool {
        *self == ENET1_TX_CLK_DIRR::ENET1_TX_CLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `ENET1_TX_CLK_DIR_1`"]
    #[inline]
    pub fn is_enet1_tx_clk_dir_1(&self) -> bool {
        *self == ENET1_TX_CLK_DIRR::ENET1_TX_CLK_DIR_1
    }
}
#[doc = "Possible values of the field `SAI1_MCLK_DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_MCLK_DIRR {
    #[doc = "LCD_DATA00 output driver is disabled when configured for ALT8"]
    SAI1_MCLK_DIR_0,
    #[doc = "LCD_DATA00 output driver is enabled when configured for ALT8"]
    SAI1_MCLK_DIR_1,
}
impl SAI1_MCLK_DIRR {
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
            SAI1_MCLK_DIRR::SAI1_MCLK_DIR_0 => false,
            SAI1_MCLK_DIRR::SAI1_MCLK_DIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI1_MCLK_DIRR {
        match value {
            false => SAI1_MCLK_DIRR::SAI1_MCLK_DIR_0,
            true => SAI1_MCLK_DIRR::SAI1_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK_DIR_0`"]
    #[inline]
    pub fn is_sai1_mclk_dir_0(&self) -> bool {
        *self == SAI1_MCLK_DIRR::SAI1_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK_DIR_1`"]
    #[inline]
    pub fn is_sai1_mclk_dir_1(&self) -> bool {
        *self == SAI1_MCLK_DIRR::SAI1_MCLK_DIR_1
    }
}
#[doc = "Possible values of the field `SAI2_MCLK_DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_MCLK_DIRR {
    #[doc = "SD1_CLK output driver is disabled when configured for ALT2"]
    SAI2_MCLK_DIR_0,
    #[doc = "SD1_CLK output driver is enabled when configured for ALT2"]
    SAI2_MCLK_DIR_1,
}
impl SAI2_MCLK_DIRR {
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
            SAI2_MCLK_DIRR::SAI2_MCLK_DIR_0 => false,
            SAI2_MCLK_DIRR::SAI2_MCLK_DIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI2_MCLK_DIRR {
        match value {
            false => SAI2_MCLK_DIRR::SAI2_MCLK_DIR_0,
            true => SAI2_MCLK_DIRR::SAI2_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK_DIR_0`"]
    #[inline]
    pub fn is_sai2_mclk_dir_0(&self) -> bool {
        *self == SAI2_MCLK_DIRR::SAI2_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK_DIR_1`"]
    #[inline]
    pub fn is_sai2_mclk_dir_1(&self) -> bool {
        *self == SAI2_MCLK_DIRR::SAI2_MCLK_DIR_1
    }
}
#[doc = "Possible values of the field `SAI3_MCLK_DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_MCLK_DIRR {
    #[doc = "LCD_CLK output driver is disabled when configured for ALT3"]
    SAI3_MCLK_DIR_0,
    #[doc = "LCD_CLK output driver is enabled when configured for ALT3"]
    SAI3_MCLK_DIR_1,
}
impl SAI3_MCLK_DIRR {
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
            SAI3_MCLK_DIRR::SAI3_MCLK_DIR_0 => false,
            SAI3_MCLK_DIRR::SAI3_MCLK_DIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI3_MCLK_DIRR {
        match value {
            false => SAI3_MCLK_DIRR::SAI3_MCLK_DIR_0,
            true => SAI3_MCLK_DIRR::SAI3_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK_DIR_0`"]
    #[inline]
    pub fn is_sai3_mclk_dir_0(&self) -> bool {
        *self == SAI3_MCLK_DIRR::SAI3_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK_DIR_1`"]
    #[inline]
    pub fn is_sai3_mclk_dir_1(&self) -> bool {
        *self == SAI3_MCLK_DIRR::SAI3_MCLK_DIR_1
    }
}
#[doc = "Possible values of the field `EXC_MON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXC_MONR {
    #[doc = "OKAY response"]
    EXC_MON_0,
    #[doc = "SLVError response (default)"]
    EXC_MON_1,
}
impl EXC_MONR {
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
            EXC_MONR::EXC_MON_0 => false,
            EXC_MONR::EXC_MON_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXC_MONR {
        match value {
            false => EXC_MONR::EXC_MON_0,
            true => EXC_MONR::EXC_MON_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXC_MON_0`"]
    #[inline]
    pub fn is_exc_mon_0(&self) -> bool {
        *self == EXC_MONR::EXC_MON_0
    }
    #[doc = "Checks if the value of the field is `EXC_MON_1`"]
    #[inline]
    pub fn is_exc_mon_1(&self) -> bool {
        *self == EXC_MONR::EXC_MON_1
    }
}
#[doc = "Possible values of the field `ENET_IPG_CLK_S_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_IPG_CLK_S_ENR {
    #[doc = "ipg_clk_s is gated when there is no IPS access"]
    ENET_IPG_CLK_S_EN_0,
    #[doc = "ipg_clk_s is always on"]
    ENET_IPG_CLK_S_EN_1,
}
impl ENET_IPG_CLK_S_ENR {
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
            ENET_IPG_CLK_S_ENR::ENET_IPG_CLK_S_EN_0 => false,
            ENET_IPG_CLK_S_ENR::ENET_IPG_CLK_S_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENET_IPG_CLK_S_ENR {
        match value {
            false => ENET_IPG_CLK_S_ENR::ENET_IPG_CLK_S_EN_0,
            true => ENET_IPG_CLK_S_ENR::ENET_IPG_CLK_S_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_IPG_CLK_S_EN_0`"]
    #[inline]
    pub fn is_enet_ipg_clk_s_en_0(&self) -> bool {
        *self == ENET_IPG_CLK_S_ENR::ENET_IPG_CLK_S_EN_0
    }
    #[doc = "Checks if the value of the field is `ENET_IPG_CLK_S_EN_1`"]
    #[inline]
    pub fn is_enet_ipg_clk_s_en_1(&self) -> bool {
        *self == ENET_IPG_CLK_S_ENR::ENET_IPG_CLK_S_EN_1
    }
}
#[doc = "Possible values of the field `CM7_FORCE_HCLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM7_FORCE_HCLK_ENR {
    #[doc = "AHB clock is not running (gated)"]
    CM7_FORCE_HCLK_EN_0,
    #[doc = "AHB clock is running (enabled)"]
    CM7_FORCE_HCLK_EN_1,
}
impl CM7_FORCE_HCLK_ENR {
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
            CM7_FORCE_HCLK_ENR::CM7_FORCE_HCLK_EN_0 => false,
            CM7_FORCE_HCLK_ENR::CM7_FORCE_HCLK_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CM7_FORCE_HCLK_ENR {
        match value {
            false => CM7_FORCE_HCLK_ENR::CM7_FORCE_HCLK_EN_0,
            true => CM7_FORCE_HCLK_ENR::CM7_FORCE_HCLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CM7_FORCE_HCLK_EN_0`"]
    #[inline]
    pub fn is_cm7_force_hclk_en_0(&self) -> bool {
        *self == CM7_FORCE_HCLK_ENR::CM7_FORCE_HCLK_EN_0
    }
    #[doc = "Checks if the value of the field is `CM7_FORCE_HCLK_EN_1`"]
    #[inline]
    pub fn is_cm7_force_hclk_en_1(&self) -> bool {
        *self == CM7_FORCE_HCLK_ENR::CM7_FORCE_HCLK_EN_1
    }
}
#[doc = "Values that can be written to the field `SAI1_MCLK1_SEL`"]
pub enum SAI1_MCLK1_SELW {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0,
    #[doc = "ccm.ssi2_clk_root"]
    SAI1_MCLK1_SEL_1,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK1_SEL_3,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK1_SEL_4,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK1_SEL_5,
}
impl SAI1_MCLK1_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_0 => 0,
            SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_1 => 1,
            SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_2 => 2,
            SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_3 => 3,
            SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_4 => 4,
            SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_MCLK1_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_MCLK1_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_MCLK1_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ccm.ssi1_clk_root"]
    #[inline]
    pub fn sai1_mclk1_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_0)
    }
    #[doc = "ccm.ssi2_clk_root"]
    #[inline]
    pub fn sai1_mclk1_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_1)
    }
    #[doc = "ccm.ssi3_clk_root"]
    #[inline]
    pub fn sai1_mclk1_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_2)
    }
    #[doc = "iomux.sai1_ipg_clk_sai_mclk[2]"]
    #[inline]
    pub fn sai1_mclk1_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_3)
    }
    #[doc = "iomux.sai2_ipg_clk_sai_mclk[2]"]
    #[inline]
    pub fn sai1_mclk1_sel_4(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_4)
    }
    #[doc = "iomux.sai3_ipg_clk_sai_mclk[2]"]
    #[inline]
    pub fn sai1_mclk1_sel_5(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SELW::SAI1_MCLK1_SEL_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI1_MCLK2_SEL`"]
pub enum SAI1_MCLK2_SELW {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0,
    #[doc = "ccm.ssi2_clk_root"]
    SAI1_MCLK2_SEL_1,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK2_SEL_3,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK2_SEL_4,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk[2]"]
    SAI1_MCLK2_SEL_5,
}
impl SAI1_MCLK2_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_0 => 0,
            SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_1 => 1,
            SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_2 => 2,
            SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_3 => 3,
            SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_4 => 4,
            SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_MCLK2_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_MCLK2_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_MCLK2_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ccm.ssi1_clk_root"]
    #[inline]
    pub fn sai1_mclk2_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_0)
    }
    #[doc = "ccm.ssi2_clk_root"]
    #[inline]
    pub fn sai1_mclk2_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_1)
    }
    #[doc = "ccm.ssi3_clk_root"]
    #[inline]
    pub fn sai1_mclk2_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_2)
    }
    #[doc = "iomux.sai1_ipg_clk_sai_mclk[2]"]
    #[inline]
    pub fn sai1_mclk2_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_3)
    }
    #[doc = "iomux.sai2_ipg_clk_sai_mclk[2]"]
    #[inline]
    pub fn sai1_mclk2_sel_4(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_4)
    }
    #[doc = "iomux.sai3_ipg_clk_sai_mclk[2]"]
    #[inline]
    pub fn sai1_mclk2_sel_5(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SELW::SAI1_MCLK2_SEL_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI1_MCLK3_SEL`"]
pub enum SAI1_MCLK3_SELW {
    #[doc = "ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI1_MCLK3_SEL_1,
    #[doc = "spdif.spdif_srclk"]
    SAI1_MCLK3_SEL_2,
    #[doc = "spdif.spdif_outclock"]
    SAI1_MCLK3_SEL_3,
}
impl SAI1_MCLK3_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_0 => 0,
            SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_1 => 1,
            SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_2 => 2,
            SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_MCLK3_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_MCLK3_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_MCLK3_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ccm.spdif0_clk_root"]
    #[inline]
    pub fn sai1_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_0)
    }
    #[doc = "iomux.spdif_tx_clk2"]
    #[inline]
    pub fn sai1_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline]
    pub fn sai1_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline]
    pub fn sai1_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SELW::SAI1_MCLK3_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI2_MCLK3_SEL`"]
pub enum SAI2_MCLK3_SELW {
    #[doc = "ccm.spdif0_clk_root"]
    SAI2_MCLK3_SEL_0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI2_MCLK3_SEL_1,
    #[doc = "spdif.spdif_srclk"]
    SAI2_MCLK3_SEL_2,
    #[doc = "spdif.spdif_outclock"]
    SAI2_MCLK3_SEL_3,
}
impl SAI2_MCLK3_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_0 => 0,
            SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_1 => 1,
            SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_2 => 2,
            SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI2_MCLK3_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2_MCLK3_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2_MCLK3_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ccm.spdif0_clk_root"]
    #[inline]
    pub fn sai2_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_0)
    }
    #[doc = "iomux.spdif_tx_clk2"]
    #[inline]
    pub fn sai2_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline]
    pub fn sai2_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline]
    pub fn sai2_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SELW::SAI2_MCLK3_SEL_3)
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
#[doc = "Values that can be written to the field `SAI3_MCLK3_SEL`"]
pub enum SAI3_MCLK3_SELW {
    #[doc = "ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI3_MCLK3_SEL_1,
    #[doc = "spdif.spdif_srclk"]
    SAI3_MCLK3_SEL_2,
    #[doc = "spdif.spdif_outclock"]
    SAI3_MCLK3_SEL_3,
}
impl SAI3_MCLK3_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_0 => 0,
            SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_1 => 1,
            SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_2 => 2,
            SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI3_MCLK3_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI3_MCLK3_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI3_MCLK3_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ccm.spdif0_clk_root"]
    #[inline]
    pub fn sai3_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_0)
    }
    #[doc = "iomux.spdif_tx_clk2"]
    #[inline]
    pub fn sai3_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline]
    pub fn sai3_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline]
    pub fn sai3_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SELW::SAI3_MCLK3_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GINT`"]
pub enum GINTW {
    #[doc = "Global interrupt request is not asserted."]
    GINT_0,
    #[doc = "Global interrupt request is asserted."]
    GINT_1,
}
impl GINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GINTW::GINT_0 => false,
            GINTW::GINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GINTW<'a> {
    w: &'a mut W,
}
impl<'a> _GINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Global interrupt request is not asserted."]
    #[inline]
    pub fn gint_0(self) -> &'a mut W {
        self.variant(GINTW::GINT_0)
    }
    #[doc = "Global interrupt request is asserted."]
    #[inline]
    pub fn gint_1(self) -> &'a mut W {
        self.variant(GINTW::GINT_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENET1_CLK_SEL`"]
pub enum ENET1_CLK_SELW {
    #[doc = "ENET1 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET_REF_CLK1 function."]
    ENET1_CLK_SEL_0,
    #[doc = "Gets ENET1 TX reference clock from the ENET1_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    ENET1_CLK_SEL_1,
}
impl ENET1_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENET1_CLK_SELW::ENET1_CLK_SEL_0 => false,
            ENET1_CLK_SELW::ENET1_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENET1_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET1_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENET1_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ENET1 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET_REF_CLK1 function."]
    #[inline]
    pub fn enet1_clk_sel_0(self) -> &'a mut W {
        self.variant(ENET1_CLK_SELW::ENET1_CLK_SEL_0)
    }
    #[doc = "Gets ENET1 TX reference clock from the ENET1_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    #[inline]
    pub fn enet1_clk_sel_1(self) -> &'a mut W {
        self.variant(ENET1_CLK_SELW::ENET1_CLK_SEL_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USB_EXP_MODE`"]
pub enum USB_EXP_MODEW {
    #[doc = "Exposure mode is disabled."]
    USB_EXP_MODE_0,
    #[doc = "Exposure mode is enabled."]
    USB_EXP_MODE_1,
}
impl USB_EXP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_EXP_MODEW::USB_EXP_MODE_0 => false,
            USB_EXP_MODEW::USB_EXP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB_EXP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EXP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_EXP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exposure mode is disabled."]
    #[inline]
    pub fn usb_exp_mode_0(self) -> &'a mut W {
        self.variant(USB_EXP_MODEW::USB_EXP_MODE_0)
    }
    #[doc = "Exposure mode is enabled."]
    #[inline]
    pub fn usb_exp_mode_1(self) -> &'a mut W {
        self.variant(USB_EXP_MODEW::USB_EXP_MODE_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENET1_TX_CLK_DIR`"]
pub enum ENET1_TX_CLK_DIRW {
    #[doc = "ENET1_TX_CLK output driver is disabled when configured for ALT1"]
    ENET1_TX_CLK_DIR_0,
    #[doc = "ENET1_TX_CLK output driver is enabled when configured for ALT1"]
    ENET1_TX_CLK_DIR_1,
}
impl ENET1_TX_CLK_DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENET1_TX_CLK_DIRW::ENET1_TX_CLK_DIR_0 => false,
            ENET1_TX_CLK_DIRW::ENET1_TX_CLK_DIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENET1_TX_CLK_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET1_TX_CLK_DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENET1_TX_CLK_DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ENET1_TX_CLK output driver is disabled when configured for ALT1"]
    #[inline]
    pub fn enet1_tx_clk_dir_0(self) -> &'a mut W {
        self.variant(ENET1_TX_CLK_DIRW::ENET1_TX_CLK_DIR_0)
    }
    #[doc = "ENET1_TX_CLK output driver is enabled when configured for ALT1"]
    #[inline]
    pub fn enet1_tx_clk_dir_1(self) -> &'a mut W {
        self.variant(ENET1_TX_CLK_DIRW::ENET1_TX_CLK_DIR_1)
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
#[doc = "Values that can be written to the field `SAI1_MCLK_DIR`"]
pub enum SAI1_MCLK_DIRW {
    #[doc = "LCD_DATA00 output driver is disabled when configured for ALT8"]
    SAI1_MCLK_DIR_0,
    #[doc = "LCD_DATA00 output driver is enabled when configured for ALT8"]
    SAI1_MCLK_DIR_1,
}
impl SAI1_MCLK_DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAI1_MCLK_DIRW::SAI1_MCLK_DIR_0 => false,
            SAI1_MCLK_DIRW::SAI1_MCLK_DIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_MCLK_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_MCLK_DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_MCLK_DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LCD_DATA00 output driver is disabled when configured for ALT8"]
    #[inline]
    pub fn sai1_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK_DIRW::SAI1_MCLK_DIR_0)
    }
    #[doc = "LCD_DATA00 output driver is enabled when configured for ALT8"]
    #[inline]
    pub fn sai1_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK_DIRW::SAI1_MCLK_DIR_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI2_MCLK_DIR`"]
pub enum SAI2_MCLK_DIRW {
    #[doc = "SD1_CLK output driver is disabled when configured for ALT2"]
    SAI2_MCLK_DIR_0,
    #[doc = "SD1_CLK output driver is enabled when configured for ALT2"]
    SAI2_MCLK_DIR_1,
}
impl SAI2_MCLK_DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAI2_MCLK_DIRW::SAI2_MCLK_DIR_0 => false,
            SAI2_MCLK_DIRW::SAI2_MCLK_DIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI2_MCLK_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2_MCLK_DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2_MCLK_DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SD1_CLK output driver is disabled when configured for ALT2"]
    #[inline]
    pub fn sai2_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI2_MCLK_DIRW::SAI2_MCLK_DIR_0)
    }
    #[doc = "SD1_CLK output driver is enabled when configured for ALT2"]
    #[inline]
    pub fn sai2_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI2_MCLK_DIRW::SAI2_MCLK_DIR_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI3_MCLK_DIR`"]
pub enum SAI3_MCLK_DIRW {
    #[doc = "LCD_CLK output driver is disabled when configured for ALT3"]
    SAI3_MCLK_DIR_0,
    #[doc = "LCD_CLK output driver is enabled when configured for ALT3"]
    SAI3_MCLK_DIR_1,
}
impl SAI3_MCLK_DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAI3_MCLK_DIRW::SAI3_MCLK_DIR_0 => false,
            SAI3_MCLK_DIRW::SAI3_MCLK_DIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI3_MCLK_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI3_MCLK_DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI3_MCLK_DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LCD_CLK output driver is disabled when configured for ALT3"]
    #[inline]
    pub fn sai3_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI3_MCLK_DIRW::SAI3_MCLK_DIR_0)
    }
    #[doc = "LCD_CLK output driver is enabled when configured for ALT3"]
    #[inline]
    pub fn sai3_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI3_MCLK_DIRW::SAI3_MCLK_DIR_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXC_MON`"]
pub enum EXC_MONW {
    #[doc = "OKAY response"]
    EXC_MON_0,
    #[doc = "SLVError response (default)"]
    EXC_MON_1,
}
impl EXC_MONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXC_MONW::EXC_MON_0 => false,
            EXC_MONW::EXC_MON_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXC_MONW<'a> {
    w: &'a mut W,
}
impl<'a> _EXC_MONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXC_MONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OKAY response"]
    #[inline]
    pub fn exc_mon_0(self) -> &'a mut W {
        self.variant(EXC_MONW::EXC_MON_0)
    }
    #[doc = "SLVError response (default)"]
    #[inline]
    pub fn exc_mon_1(self) -> &'a mut W {
        self.variant(EXC_MONW::EXC_MON_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENET_IPG_CLK_S_EN`"]
pub enum ENET_IPG_CLK_S_ENW {
    #[doc = "ipg_clk_s is gated when there is no IPS access"]
    ENET_IPG_CLK_S_EN_0,
    #[doc = "ipg_clk_s is always on"]
    ENET_IPG_CLK_S_EN_1,
}
impl ENET_IPG_CLK_S_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENET_IPG_CLK_S_ENW::ENET_IPG_CLK_S_EN_0 => false,
            ENET_IPG_CLK_S_ENW::ENET_IPG_CLK_S_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENET_IPG_CLK_S_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET_IPG_CLK_S_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENET_IPG_CLK_S_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ipg_clk_s is gated when there is no IPS access"]
    #[inline]
    pub fn enet_ipg_clk_s_en_0(self) -> &'a mut W {
        self.variant(ENET_IPG_CLK_S_ENW::ENET_IPG_CLK_S_EN_0)
    }
    #[doc = "ipg_clk_s is always on"]
    #[inline]
    pub fn enet_ipg_clk_s_en_1(self) -> &'a mut W {
        self.variant(ENET_IPG_CLK_S_ENW::ENET_IPG_CLK_S_EN_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CM7_FORCE_HCLK_EN`"]
pub enum CM7_FORCE_HCLK_ENW {
    #[doc = "AHB clock is not running (gated)"]
    CM7_FORCE_HCLK_EN_0,
    #[doc = "AHB clock is running (enabled)"]
    CM7_FORCE_HCLK_EN_1,
}
impl CM7_FORCE_HCLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CM7_FORCE_HCLK_ENW::CM7_FORCE_HCLK_EN_0 => false,
            CM7_FORCE_HCLK_ENW::CM7_FORCE_HCLK_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM7_FORCE_HCLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CM7_FORCE_HCLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM7_FORCE_HCLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHB clock is not running (gated)"]
    #[inline]
    pub fn cm7_force_hclk_en_0(self) -> &'a mut W {
        self.variant(CM7_FORCE_HCLK_ENW::CM7_FORCE_HCLK_EN_0)
    }
    #[doc = "AHB clock is running (enabled)"]
    #[inline]
    pub fn cm7_force_hclk_en_1(self) -> &'a mut W {
        self.variant(CM7_FORCE_HCLK_ENW::CM7_FORCE_HCLK_EN_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:2 - SAI1 MCLK1 source select"]
    #[inline]
    pub fn sai1_mclk1_sel(&self) -> SAI1_MCLK1_SELR {
        SAI1_MCLK1_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - SAI1 MCLK2 source select"]
    #[inline]
    pub fn sai1_mclk2_sel(&self) -> SAI1_MCLK2_SELR {
        SAI1_MCLK2_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - SAI1 MCLK3 source select"]
    #[inline]
    pub fn sai1_mclk3_sel(&self) -> SAI1_MCLK3_SELR {
        SAI1_MCLK3_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - SAI2 MCLK3 source select"]
    #[inline]
    pub fn sai2_mclk3_sel(&self) -> SAI2_MCLK3_SELR {
        SAI2_MCLK3_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - SAI3 MCLK3 source select"]
    #[inline]
    pub fn sai3_mclk3_sel(&self) -> SAI3_MCLK3_SELR {
        SAI3_MCLK3_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Global interrupt \"0\" bit (connected to ARM M7 IRQ#0 and GPC)"]
    #[inline]
    pub fn gint(&self) -> GINTR {
        GINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - ENET1 reference clock mode select."]
    #[inline]
    pub fn enet1_clk_sel(&self) -> ENET1_CLK_SELR {
        ENET1_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - USB Exposure mode"]
    #[inline]
    pub fn usb_exp_mode(&self) -> USB_EXP_MODER {
        USB_EXP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - ENET1_TX_CLK data direction control when anatop. ENET_REF_CLK1 is selected (ALT1)"]
    #[inline]
    pub fn enet1_tx_clk_dir(&self) -> ENET1_TX_CLK_DIRR {
        ENET1_TX_CLK_DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - LCD_DATA00 data direction control when sai1.MCLK is selected (ALT8)"]
    #[inline]
    pub fn sai1_mclk_dir(&self) -> SAI1_MCLK_DIRR {
        SAI1_MCLK_DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SD1_CLK data direction control when sai2.MCLK is selected (ALT2)"]
    #[inline]
    pub fn sai2_mclk_dir(&self) -> SAI2_MCLK_DIRR {
        SAI2_MCLK_DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - LCD_CLK data direction control when sai3.MCLK is selected (ALT3)"]
    #[inline]
    pub fn sai3_mclk_dir(&self) -> SAI3_MCLK_DIRR {
        SAI3_MCLK_DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Exclusive monitor response select of illegal command"]
    #[inline]
    pub fn exc_mon(&self) -> EXC_MONR {
        EXC_MONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - ENET ipg_clk_s clock gating enable"]
    #[inline]
    pub fn enet_ipg_clk_s_en(&self) -> ENET_IPG_CLK_S_ENR {
        ENET_IPG_CLK_S_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - ARM CM7 platform AHB clock enable"]
    #[inline]
    pub fn cm7_force_hclk_en(&self) -> CM7_FORCE_HCLK_ENR {
        CM7_FORCE_HCLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:2 - SAI1 MCLK1 source select"]
    #[inline]
    pub fn sai1_mclk1_sel(&mut self) -> _SAI1_MCLK1_SELW {
        _SAI1_MCLK1_SELW { w: self }
    }
    #[doc = "Bits 3:5 - SAI1 MCLK2 source select"]
    #[inline]
    pub fn sai1_mclk2_sel(&mut self) -> _SAI1_MCLK2_SELW {
        _SAI1_MCLK2_SELW { w: self }
    }
    #[doc = "Bits 6:7 - SAI1 MCLK3 source select"]
    #[inline]
    pub fn sai1_mclk3_sel(&mut self) -> _SAI1_MCLK3_SELW {
        _SAI1_MCLK3_SELW { w: self }
    }
    #[doc = "Bits 8:9 - SAI2 MCLK3 source select"]
    #[inline]
    pub fn sai2_mclk3_sel(&mut self) -> _SAI2_MCLK3_SELW {
        _SAI2_MCLK3_SELW { w: self }
    }
    #[doc = "Bits 10:11 - SAI3 MCLK3 source select"]
    #[inline]
    pub fn sai3_mclk3_sel(&mut self) -> _SAI3_MCLK3_SELW {
        _SAI3_MCLK3_SELW { w: self }
    }
    #[doc = "Bit 12 - Global interrupt \"0\" bit (connected to ARM M7 IRQ#0 and GPC)"]
    #[inline]
    pub fn gint(&mut self) -> _GINTW {
        _GINTW { w: self }
    }
    #[doc = "Bit 13 - ENET1 reference clock mode select."]
    #[inline]
    pub fn enet1_clk_sel(&mut self) -> _ENET1_CLK_SELW {
        _ENET1_CLK_SELW { w: self }
    }
    #[doc = "Bit 15 - USB Exposure mode"]
    #[inline]
    pub fn usb_exp_mode(&mut self) -> _USB_EXP_MODEW {
        _USB_EXP_MODEW { w: self }
    }
    #[doc = "Bit 17 - ENET1_TX_CLK data direction control when anatop. ENET_REF_CLK1 is selected (ALT1)"]
    #[inline]
    pub fn enet1_tx_clk_dir(&mut self) -> _ENET1_TX_CLK_DIRW {
        _ENET1_TX_CLK_DIRW { w: self }
    }
    #[doc = "Bit 19 - LCD_DATA00 data direction control when sai1.MCLK is selected (ALT8)"]
    #[inline]
    pub fn sai1_mclk_dir(&mut self) -> _SAI1_MCLK_DIRW {
        _SAI1_MCLK_DIRW { w: self }
    }
    #[doc = "Bit 20 - SD1_CLK data direction control when sai2.MCLK is selected (ALT2)"]
    #[inline]
    pub fn sai2_mclk_dir(&mut self) -> _SAI2_MCLK_DIRW {
        _SAI2_MCLK_DIRW { w: self }
    }
    #[doc = "Bit 21 - LCD_CLK data direction control when sai3.MCLK is selected (ALT3)"]
    #[inline]
    pub fn sai3_mclk_dir(&mut self) -> _SAI3_MCLK_DIRW {
        _SAI3_MCLK_DIRW { w: self }
    }
    #[doc = "Bit 22 - Exclusive monitor response select of illegal command"]
    #[inline]
    pub fn exc_mon(&mut self) -> _EXC_MONW {
        _EXC_MONW { w: self }
    }
    #[doc = "Bit 23 - ENET ipg_clk_s clock gating enable"]
    #[inline]
    pub fn enet_ipg_clk_s_en(&mut self) -> _ENET_IPG_CLK_S_ENW {
        _ENET_IPG_CLK_S_ENW { w: self }
    }
    #[doc = "Bit 31 - ARM CM7 platform AHB clock enable"]
    #[inline]
    pub fn cm7_force_hclk_en(&mut self) -> _CM7_FORCE_HCLK_ENW {
        _CM7_FORCE_HCLK_ENW { w: self }
    }
}
