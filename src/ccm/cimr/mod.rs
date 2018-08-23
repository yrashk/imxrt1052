#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CIMR {
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
#[doc = "Possible values of the field `MASK_LRF_PLL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_LRF_PLLR {
    #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    MASK_LRF_PLL_0,
    #[doc = "mask interrupt due to lrf of PLLs"]
    MASK_LRF_PLL_1,
}
impl MASK_LRF_PLLR {
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
            MASK_LRF_PLLR::MASK_LRF_PLL_0 => false,
            MASK_LRF_PLLR::MASK_LRF_PLL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_LRF_PLLR {
        match value {
            false => MASK_LRF_PLLR::MASK_LRF_PLL_0,
            true => MASK_LRF_PLLR::MASK_LRF_PLL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_LRF_PLL_0`"]
    #[inline]
    pub fn is_mask_lrf_pll_0(&self) -> bool {
        *self == MASK_LRF_PLLR::MASK_LRF_PLL_0
    }
    #[doc = "Checks if the value of the field is `MASK_LRF_PLL_1`"]
    #[inline]
    pub fn is_mask_lrf_pll_1(&self) -> bool {
        *self == MASK_LRF_PLLR::MASK_LRF_PLL_1
    }
}
#[doc = "Possible values of the field `MASK_COSC_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_COSC_READYR {
    #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    MASK_COSC_READY_0,
    #[doc = "mask interrupt due to on board oscillator ready"]
    MASK_COSC_READY_1,
}
impl MASK_COSC_READYR {
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
            MASK_COSC_READYR::MASK_COSC_READY_0 => false,
            MASK_COSC_READYR::MASK_COSC_READY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_COSC_READYR {
        match value {
            false => MASK_COSC_READYR::MASK_COSC_READY_0,
            true => MASK_COSC_READYR::MASK_COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_COSC_READY_0`"]
    #[inline]
    pub fn is_mask_cosc_ready_0(&self) -> bool {
        *self == MASK_COSC_READYR::MASK_COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `MASK_COSC_READY_1`"]
    #[inline]
    pub fn is_mask_cosc_ready_1(&self) -> bool {
        *self == MASK_COSC_READYR::MASK_COSC_READY_1
    }
}
#[doc = "Possible values of the field `MASK_SEMC_PODF_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_SEMC_PODF_LOADEDR {
    #[doc = "don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    MASK_SEMC_PODF_LOADED_0,
    #[doc = "mask interrupt due to frequency change of semc_podf"]
    MASK_SEMC_PODF_LOADED_1,
}
impl MASK_SEMC_PODF_LOADEDR {
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
            MASK_SEMC_PODF_LOADEDR::MASK_SEMC_PODF_LOADED_0 => false,
            MASK_SEMC_PODF_LOADEDR::MASK_SEMC_PODF_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_SEMC_PODF_LOADEDR {
        match value {
            false => MASK_SEMC_PODF_LOADEDR::MASK_SEMC_PODF_LOADED_0,
            true => MASK_SEMC_PODF_LOADEDR::MASK_SEMC_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_SEMC_PODF_LOADED_0`"]
    #[inline]
    pub fn is_mask_semc_podf_loaded_0(&self) -> bool {
        *self == MASK_SEMC_PODF_LOADEDR::MASK_SEMC_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_SEMC_PODF_LOADED_1`"]
    #[inline]
    pub fn is_mask_semc_podf_loaded_1(&self) -> bool {
        *self == MASK_SEMC_PODF_LOADEDR::MASK_SEMC_PODF_LOADED_1
    }
}
#[doc = "Possible values of the field `MASK_PERIPH2_CLK_SEL_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_PERIPH2_CLK_SEL_LOADEDR {
    #[doc = "don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    MASK_PERIPH2_CLK_SEL_LOADED_0,
    #[doc = "mask interrupt due to update of periph2_clk_sel"]
    MASK_PERIPH2_CLK_SEL_LOADED_1,
}
impl MASK_PERIPH2_CLK_SEL_LOADEDR {
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
            MASK_PERIPH2_CLK_SEL_LOADEDR::MASK_PERIPH2_CLK_SEL_LOADED_0 => false,
            MASK_PERIPH2_CLK_SEL_LOADEDR::MASK_PERIPH2_CLK_SEL_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_PERIPH2_CLK_SEL_LOADEDR {
        match value {
            false => MASK_PERIPH2_CLK_SEL_LOADEDR::MASK_PERIPH2_CLK_SEL_LOADED_0,
            true => MASK_PERIPH2_CLK_SEL_LOADEDR::MASK_PERIPH2_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH2_CLK_SEL_LOADED_0`"]
    #[inline]
    pub fn is_mask_periph2_clk_sel_loaded_0(&self) -> bool {
        *self == MASK_PERIPH2_CLK_SEL_LOADEDR::MASK_PERIPH2_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH2_CLK_SEL_LOADED_1`"]
    #[inline]
    pub fn is_mask_periph2_clk_sel_loaded_1(&self) -> bool {
        *self == MASK_PERIPH2_CLK_SEL_LOADEDR::MASK_PERIPH2_CLK_SEL_LOADED_1
    }
}
#[doc = "Possible values of the field `MASK_AHB_PODF_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_AHB_PODF_LOADEDR {
    #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    MASK_AHB_PODF_LOADED_0,
    #[doc = "mask interrupt due to frequency change of ahb_podf"]
    MASK_AHB_PODF_LOADED_1,
}
impl MASK_AHB_PODF_LOADEDR {
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
            MASK_AHB_PODF_LOADEDR::MASK_AHB_PODF_LOADED_0 => false,
            MASK_AHB_PODF_LOADEDR::MASK_AHB_PODF_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_AHB_PODF_LOADEDR {
        match value {
            false => MASK_AHB_PODF_LOADEDR::MASK_AHB_PODF_LOADED_0,
            true => MASK_AHB_PODF_LOADEDR::MASK_AHB_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_AHB_PODF_LOADED_0`"]
    #[inline]
    pub fn is_mask_ahb_podf_loaded_0(&self) -> bool {
        *self == MASK_AHB_PODF_LOADEDR::MASK_AHB_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_AHB_PODF_LOADED_1`"]
    #[inline]
    pub fn is_mask_ahb_podf_loaded_1(&self) -> bool {
        *self == MASK_AHB_PODF_LOADEDR::MASK_AHB_PODF_LOADED_1
    }
}
#[doc = "Possible values of the field `MASK_PERIPH_CLK_SEL_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_PERIPH_CLK_SEL_LOADEDR {
    #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    MASK_PERIPH_CLK_SEL_LOADED_0,
    #[doc = "mask interrupt due to update of periph_clk_sel"]
    MASK_PERIPH_CLK_SEL_LOADED_1,
}
impl MASK_PERIPH_CLK_SEL_LOADEDR {
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
            MASK_PERIPH_CLK_SEL_LOADEDR::MASK_PERIPH_CLK_SEL_LOADED_0 => false,
            MASK_PERIPH_CLK_SEL_LOADEDR::MASK_PERIPH_CLK_SEL_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_PERIPH_CLK_SEL_LOADEDR {
        match value {
            false => MASK_PERIPH_CLK_SEL_LOADEDR::MASK_PERIPH_CLK_SEL_LOADED_0,
            true => MASK_PERIPH_CLK_SEL_LOADEDR::MASK_PERIPH_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH_CLK_SEL_LOADED_0`"]
    #[inline]
    pub fn is_mask_periph_clk_sel_loaded_0(&self) -> bool {
        *self == MASK_PERIPH_CLK_SEL_LOADEDR::MASK_PERIPH_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH_CLK_SEL_LOADED_1`"]
    #[inline]
    pub fn is_mask_periph_clk_sel_loaded_1(&self) -> bool {
        *self == MASK_PERIPH_CLK_SEL_LOADEDR::MASK_PERIPH_CLK_SEL_LOADED_1
    }
}
#[doc = "Possible values of the field `ARM_PODF_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_PODF_LOADEDR {
    #[doc = "don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    ARM_PODF_LOADED_0,
    #[doc = "mask interrupt due to frequency change of arm_podf"]
    ARM_PODF_LOADED_1,
}
impl ARM_PODF_LOADEDR {
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
            ARM_PODF_LOADEDR::ARM_PODF_LOADED_0 => false,
            ARM_PODF_LOADEDR::ARM_PODF_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARM_PODF_LOADEDR {
        match value {
            false => ARM_PODF_LOADEDR::ARM_PODF_LOADED_0,
            true => ARM_PODF_LOADEDR::ARM_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_LOADED_0`"]
    #[inline]
    pub fn is_arm_podf_loaded_0(&self) -> bool {
        *self == ARM_PODF_LOADEDR::ARM_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_LOADED_1`"]
    #[inline]
    pub fn is_arm_podf_loaded_1(&self) -> bool {
        *self == ARM_PODF_LOADEDR::ARM_PODF_LOADED_1
    }
}
#[doc = "Values that can be written to the field `MASK_LRF_PLL`"]
pub enum MASK_LRF_PLLW {
    #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    MASK_LRF_PLL_0,
    #[doc = "mask interrupt due to lrf of PLLs"]
    MASK_LRF_PLL_1,
}
impl MASK_LRF_PLLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_LRF_PLLW::MASK_LRF_PLL_0 => false,
            MASK_LRF_PLLW::MASK_LRF_PLL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_LRF_PLLW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_LRF_PLLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_LRF_PLLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    #[inline]
    pub fn mask_lrf_pll_0(self) -> &'a mut W {
        self.variant(MASK_LRF_PLLW::MASK_LRF_PLL_0)
    }
    #[doc = "mask interrupt due to lrf of PLLs"]
    #[inline]
    pub fn mask_lrf_pll_1(self) -> &'a mut W {
        self.variant(MASK_LRF_PLLW::MASK_LRF_PLL_1)
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
#[doc = "Values that can be written to the field `MASK_COSC_READY`"]
pub enum MASK_COSC_READYW {
    #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    MASK_COSC_READY_0,
    #[doc = "mask interrupt due to on board oscillator ready"]
    MASK_COSC_READY_1,
}
impl MASK_COSC_READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_COSC_READYW::MASK_COSC_READY_0 => false,
            MASK_COSC_READYW::MASK_COSC_READY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_COSC_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_COSC_READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_COSC_READYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    #[inline]
    pub fn mask_cosc_ready_0(self) -> &'a mut W {
        self.variant(MASK_COSC_READYW::MASK_COSC_READY_0)
    }
    #[doc = "mask interrupt due to on board oscillator ready"]
    #[inline]
    pub fn mask_cosc_ready_1(self) -> &'a mut W {
        self.variant(MASK_COSC_READYW::MASK_COSC_READY_1)
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
#[doc = "Values that can be written to the field `MASK_SEMC_PODF_LOADED`"]
pub enum MASK_SEMC_PODF_LOADEDW {
    #[doc = "don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    MASK_SEMC_PODF_LOADED_0,
    #[doc = "mask interrupt due to frequency change of semc_podf"]
    MASK_SEMC_PODF_LOADED_1,
}
impl MASK_SEMC_PODF_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_SEMC_PODF_LOADEDW::MASK_SEMC_PODF_LOADED_0 => false,
            MASK_SEMC_PODF_LOADEDW::MASK_SEMC_PODF_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_SEMC_PODF_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_SEMC_PODF_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_SEMC_PODF_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    #[inline]
    pub fn mask_semc_podf_loaded_0(self) -> &'a mut W {
        self.variant(MASK_SEMC_PODF_LOADEDW::MASK_SEMC_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of semc_podf"]
    #[inline]
    pub fn mask_semc_podf_loaded_1(self) -> &'a mut W {
        self.variant(MASK_SEMC_PODF_LOADEDW::MASK_SEMC_PODF_LOADED_1)
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
#[doc = "Values that can be written to the field `MASK_PERIPH2_CLK_SEL_LOADED`"]
pub enum MASK_PERIPH2_CLK_SEL_LOADEDW {
    #[doc = "don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    MASK_PERIPH2_CLK_SEL_LOADED_0,
    #[doc = "mask interrupt due to update of periph2_clk_sel"]
    MASK_PERIPH2_CLK_SEL_LOADED_1,
}
impl MASK_PERIPH2_CLK_SEL_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_PERIPH2_CLK_SEL_LOADEDW::MASK_PERIPH2_CLK_SEL_LOADED_0 => false,
            MASK_PERIPH2_CLK_SEL_LOADEDW::MASK_PERIPH2_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_PERIPH2_CLK_SEL_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_PERIPH2_CLK_SEL_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_PERIPH2_CLK_SEL_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    #[inline]
    pub fn mask_periph2_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(MASK_PERIPH2_CLK_SEL_LOADEDW::MASK_PERIPH2_CLK_SEL_LOADED_0)
    }
    #[doc = "mask interrupt due to update of periph2_clk_sel"]
    #[inline]
    pub fn mask_periph2_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(MASK_PERIPH2_CLK_SEL_LOADEDW::MASK_PERIPH2_CLK_SEL_LOADED_1)
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
#[doc = "Values that can be written to the field `MASK_AHB_PODF_LOADED`"]
pub enum MASK_AHB_PODF_LOADEDW {
    #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    MASK_AHB_PODF_LOADED_0,
    #[doc = "mask interrupt due to frequency change of ahb_podf"]
    MASK_AHB_PODF_LOADED_1,
}
impl MASK_AHB_PODF_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_AHB_PODF_LOADEDW::MASK_AHB_PODF_LOADED_0 => false,
            MASK_AHB_PODF_LOADEDW::MASK_AHB_PODF_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_AHB_PODF_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_AHB_PODF_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_AHB_PODF_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    #[inline]
    pub fn mask_ahb_podf_loaded_0(self) -> &'a mut W {
        self.variant(MASK_AHB_PODF_LOADEDW::MASK_AHB_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of ahb_podf"]
    #[inline]
    pub fn mask_ahb_podf_loaded_1(self) -> &'a mut W {
        self.variant(MASK_AHB_PODF_LOADEDW::MASK_AHB_PODF_LOADED_1)
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
#[doc = "Values that can be written to the field `MASK_PERIPH_CLK_SEL_LOADED`"]
pub enum MASK_PERIPH_CLK_SEL_LOADEDW {
    #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    MASK_PERIPH_CLK_SEL_LOADED_0,
    #[doc = "mask interrupt due to update of periph_clk_sel"]
    MASK_PERIPH_CLK_SEL_LOADED_1,
}
impl MASK_PERIPH_CLK_SEL_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_PERIPH_CLK_SEL_LOADEDW::MASK_PERIPH_CLK_SEL_LOADED_0 => false,
            MASK_PERIPH_CLK_SEL_LOADEDW::MASK_PERIPH_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_PERIPH_CLK_SEL_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_PERIPH_CLK_SEL_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_PERIPH_CLK_SEL_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    #[inline]
    pub fn mask_periph_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(MASK_PERIPH_CLK_SEL_LOADEDW::MASK_PERIPH_CLK_SEL_LOADED_0)
    }
    #[doc = "mask interrupt due to update of periph_clk_sel"]
    #[inline]
    pub fn mask_periph_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(MASK_PERIPH_CLK_SEL_LOADEDW::MASK_PERIPH_CLK_SEL_LOADED_1)
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
#[doc = "Values that can be written to the field `ARM_PODF_LOADED`"]
pub enum ARM_PODF_LOADEDW {
    #[doc = "don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    ARM_PODF_LOADED_0,
    #[doc = "mask interrupt due to frequency change of arm_podf"]
    ARM_PODF_LOADED_1,
}
impl ARM_PODF_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARM_PODF_LOADEDW::ARM_PODF_LOADED_0 => false,
            ARM_PODF_LOADEDW::ARM_PODF_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARM_PODF_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _ARM_PODF_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARM_PODF_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    #[inline]
    pub fn arm_podf_loaded_0(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADEDW::ARM_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of arm_podf"]
    #[inline]
    pub fn arm_podf_loaded_1(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADEDW::ARM_PODF_LOADED_1)
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
    #[doc = "Bit 0 - mask interrupt generation due to lrf of PLLs"]
    #[inline]
    pub fn mask_lrf_pll(&self) -> MASK_LRF_PLLR {
        MASK_LRF_PLLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - mask interrupt generation due to on board oscillator ready"]
    #[inline]
    pub fn mask_cosc_ready(&self) -> MASK_COSC_READYR {
        MASK_COSC_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - mask interrupt generation due to frequency change of semc_podf"]
    #[inline]
    pub fn mask_semc_podf_loaded(&self) -> MASK_SEMC_PODF_LOADEDR {
        MASK_SEMC_PODF_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - mask interrupt generation due to update of periph2_clk_sel."]
    #[inline]
    pub fn mask_periph2_clk_sel_loaded(&self) -> MASK_PERIPH2_CLK_SEL_LOADEDR {
        MASK_PERIPH2_CLK_SEL_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - mask interrupt generation due to frequency change of ahb_podf"]
    #[inline]
    pub fn mask_ahb_podf_loaded(&self) -> MASK_AHB_PODF_LOADEDR {
        MASK_AHB_PODF_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - mask interrupt generation due to update of periph_clk_sel."]
    #[inline]
    pub fn mask_periph_clk_sel_loaded(&self) -> MASK_PERIPH_CLK_SEL_LOADEDR {
        MASK_PERIPH_CLK_SEL_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - mask interrupt generation due to frequency change of arm_podf"]
    #[inline]
    pub fn arm_podf_loaded(&self) -> ARM_PODF_LOADEDR {
        ARM_PODF_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - mask interrupt generation due to lrf of PLLs"]
    #[inline]
    pub fn mask_lrf_pll(&mut self) -> _MASK_LRF_PLLW {
        _MASK_LRF_PLLW { w: self }
    }
    #[doc = "Bit 6 - mask interrupt generation due to on board oscillator ready"]
    #[inline]
    pub fn mask_cosc_ready(&mut self) -> _MASK_COSC_READYW {
        _MASK_COSC_READYW { w: self }
    }
    #[doc = "Bit 17 - mask interrupt generation due to frequency change of semc_podf"]
    #[inline]
    pub fn mask_semc_podf_loaded(&mut self) -> _MASK_SEMC_PODF_LOADEDW {
        _MASK_SEMC_PODF_LOADEDW { w: self }
    }
    #[doc = "Bit 19 - mask interrupt generation due to update of periph2_clk_sel."]
    #[inline]
    pub fn mask_periph2_clk_sel_loaded(&mut self) -> _MASK_PERIPH2_CLK_SEL_LOADEDW {
        _MASK_PERIPH2_CLK_SEL_LOADEDW { w: self }
    }
    #[doc = "Bit 20 - mask interrupt generation due to frequency change of ahb_podf"]
    #[inline]
    pub fn mask_ahb_podf_loaded(&mut self) -> _MASK_AHB_PODF_LOADEDW {
        _MASK_AHB_PODF_LOADEDW { w: self }
    }
    #[doc = "Bit 22 - mask interrupt generation due to update of periph_clk_sel."]
    #[inline]
    pub fn mask_periph_clk_sel_loaded(&mut self) -> _MASK_PERIPH_CLK_SEL_LOADEDW {
        _MASK_PERIPH_CLK_SEL_LOADEDW { w: self }
    }
    #[doc = "Bit 26 - mask interrupt generation due to frequency change of arm_podf"]
    #[inline]
    pub fn arm_podf_loaded(&mut self) -> _ARM_PODF_LOADEDW {
        _ARM_PODF_LOADEDW { w: self }
    }
}
