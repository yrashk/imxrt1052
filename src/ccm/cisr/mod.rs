#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CISR {
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
#[doc = "Possible values of the field `LRF_PLL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRF_PLLR {
    #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_0,
    #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_1,
}
impl LRF_PLLR {
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
            LRF_PLLR::LRF_PLL_0 => false,
            LRF_PLLR::LRF_PLL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRF_PLLR {
        match value {
            false => LRF_PLLR::LRF_PLL_0,
            true => LRF_PLLR::LRF_PLL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LRF_PLL_0`"]
    #[inline]
    pub fn is_lrf_pll_0(&self) -> bool {
        *self == LRF_PLLR::LRF_PLL_0
    }
    #[doc = "Checks if the value of the field is `LRF_PLL_1`"]
    #[inline]
    pub fn is_lrf_pll_1(&self) -> bool {
        *self == LRF_PLLR::LRF_PLL_1
    }
}
#[doc = "Possible values of the field `COSC_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_READYR {
    #[doc = "interrupt is not generated due to on board oscillator ready"]
    COSC_READY_0,
    #[doc = "interrupt generated due to on board oscillator ready"]
    COSC_READY_1,
}
impl COSC_READYR {
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
            COSC_READYR::COSC_READY_0 => false,
            COSC_READYR::COSC_READY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSC_READYR {
        match value {
            false => COSC_READYR::COSC_READY_0,
            true => COSC_READYR::COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_READY_0`"]
    #[inline]
    pub fn is_cosc_ready_0(&self) -> bool {
        *self == COSC_READYR::COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `COSC_READY_1`"]
    #[inline]
    pub fn is_cosc_ready_1(&self) -> bool {
        *self == COSC_READYR::COSC_READY_1
    }
}
#[doc = "Possible values of the field `SEMC_PODF_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_PODF_LOADEDR {
    #[doc = "interrupt is not generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_0,
    #[doc = "interrupt generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_1,
}
impl SEMC_PODF_LOADEDR {
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
            SEMC_PODF_LOADEDR::SEMC_PODF_LOADED_0 => false,
            SEMC_PODF_LOADEDR::SEMC_PODF_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEMC_PODF_LOADEDR {
        match value {
            false => SEMC_PODF_LOADEDR::SEMC_PODF_LOADED_0,
            true => SEMC_PODF_LOADEDR::SEMC_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_LOADED_0`"]
    #[inline]
    pub fn is_semc_podf_loaded_0(&self) -> bool {
        *self == SEMC_PODF_LOADEDR::SEMC_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_LOADED_1`"]
    #[inline]
    pub fn is_semc_podf_loaded_1(&self) -> bool {
        *self == SEMC_PODF_LOADEDR::SEMC_PODF_LOADED_1
    }
}
#[doc = "Possible values of the field `PERIPH2_CLK_SEL_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH2_CLK_SEL_LOADEDR {
    #[doc = "interrupt is not generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_0,
    #[doc = "interrupt generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_1,
}
impl PERIPH2_CLK_SEL_LOADEDR {
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
            PERIPH2_CLK_SEL_LOADEDR::PERIPH2_CLK_SEL_LOADED_0 => false,
            PERIPH2_CLK_SEL_LOADEDR::PERIPH2_CLK_SEL_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERIPH2_CLK_SEL_LOADEDR {
        match value {
            false => PERIPH2_CLK_SEL_LOADEDR::PERIPH2_CLK_SEL_LOADED_0,
            true => PERIPH2_CLK_SEL_LOADEDR::PERIPH2_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_LOADED_0`"]
    #[inline]
    pub fn is_periph2_clk_sel_loaded_0(&self) -> bool {
        *self == PERIPH2_CLK_SEL_LOADEDR::PERIPH2_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_LOADED_1`"]
    #[inline]
    pub fn is_periph2_clk_sel_loaded_1(&self) -> bool {
        *self == PERIPH2_CLK_SEL_LOADEDR::PERIPH2_CLK_SEL_LOADED_1
    }
}
#[doc = "Possible values of the field `AHB_PODF_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_PODF_LOADEDR {
    #[doc = "interrupt is not generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_0,
    #[doc = "interrupt generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_1,
}
impl AHB_PODF_LOADEDR {
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
            AHB_PODF_LOADEDR::AHB_PODF_LOADED_0 => false,
            AHB_PODF_LOADEDR::AHB_PODF_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_PODF_LOADEDR {
        match value {
            false => AHB_PODF_LOADEDR::AHB_PODF_LOADED_0,
            true => AHB_PODF_LOADEDR::AHB_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_LOADED_0`"]
    #[inline]
    pub fn is_ahb_podf_loaded_0(&self) -> bool {
        *self == AHB_PODF_LOADEDR::AHB_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_LOADED_1`"]
    #[inline]
    pub fn is_ahb_podf_loaded_1(&self) -> bool {
        *self == AHB_PODF_LOADEDR::AHB_PODF_LOADED_1
    }
}
#[doc = "Possible values of the field `PERIPH_CLK_SEL_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK_SEL_LOADEDR {
    #[doc = "interrupt is not generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_0,
    #[doc = "interrupt generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_1,
}
impl PERIPH_CLK_SEL_LOADEDR {
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
            PERIPH_CLK_SEL_LOADEDR::PERIPH_CLK_SEL_LOADED_0 => false,
            PERIPH_CLK_SEL_LOADEDR::PERIPH_CLK_SEL_LOADED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERIPH_CLK_SEL_LOADEDR {
        match value {
            false => PERIPH_CLK_SEL_LOADEDR::PERIPH_CLK_SEL_LOADED_0,
            true => PERIPH_CLK_SEL_LOADEDR::PERIPH_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_LOADED_0`"]
    #[inline]
    pub fn is_periph_clk_sel_loaded_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_LOADEDR::PERIPH_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_LOADED_1`"]
    #[inline]
    pub fn is_periph_clk_sel_loaded_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_LOADEDR::PERIPH_CLK_SEL_LOADED_1
    }
}
#[doc = "Possible values of the field `ARM_PODF_LOADED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_PODF_LOADEDR {
    #[doc = "interrupt is not generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_0,
    #[doc = "interrupt generated due to frequency change of arm_podf"]
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
#[doc = "Values that can be written to the field `LRF_PLL`"]
pub enum LRF_PLLW {
    #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_0,
    #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_1,
}
impl LRF_PLLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRF_PLLW::LRF_PLL_0 => false,
            LRF_PLLW::LRF_PLL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRF_PLLW<'a> {
    w: &'a mut W,
}
impl<'a> _LRF_PLLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRF_PLLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    #[inline]
    pub fn lrf_pll_0(self) -> &'a mut W {
        self.variant(LRF_PLLW::LRF_PLL_0)
    }
    #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    #[inline]
    pub fn lrf_pll_1(self) -> &'a mut W {
        self.variant(LRF_PLLW::LRF_PLL_1)
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
#[doc = "Values that can be written to the field `COSC_READY`"]
pub enum COSC_READYW {
    #[doc = "interrupt is not generated due to on board oscillator ready"]
    COSC_READY_0,
    #[doc = "interrupt generated due to on board oscillator ready"]
    COSC_READY_1,
}
impl COSC_READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSC_READYW::COSC_READY_0 => false,
            COSC_READYW::COSC_READY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSC_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _COSC_READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSC_READYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt is not generated due to on board oscillator ready"]
    #[inline]
    pub fn cosc_ready_0(self) -> &'a mut W {
        self.variant(COSC_READYW::COSC_READY_0)
    }
    #[doc = "interrupt generated due to on board oscillator ready"]
    #[inline]
    pub fn cosc_ready_1(self) -> &'a mut W {
        self.variant(COSC_READYW::COSC_READY_1)
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
#[doc = "Values that can be written to the field `SEMC_PODF_LOADED`"]
pub enum SEMC_PODF_LOADEDW {
    #[doc = "interrupt is not generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_0,
    #[doc = "interrupt generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_1,
}
impl SEMC_PODF_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEMC_PODF_LOADEDW::SEMC_PODF_LOADED_0 => false,
            SEMC_PODF_LOADEDW::SEMC_PODF_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEMC_PODF_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEMC_PODF_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEMC_PODF_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt is not generated due to frequency change of semc_podf"]
    #[inline]
    pub fn semc_podf_loaded_0(self) -> &'a mut W {
        self.variant(SEMC_PODF_LOADEDW::SEMC_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of semc_podf"]
    #[inline]
    pub fn semc_podf_loaded_1(self) -> &'a mut W {
        self.variant(SEMC_PODF_LOADEDW::SEMC_PODF_LOADED_1)
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
#[doc = "Values that can be written to the field `PERIPH2_CLK_SEL_LOADED`"]
pub enum PERIPH2_CLK_SEL_LOADEDW {
    #[doc = "interrupt is not generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_0,
    #[doc = "interrupt generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_1,
}
impl PERIPH2_CLK_SEL_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERIPH2_CLK_SEL_LOADEDW::PERIPH2_CLK_SEL_LOADED_0 => false,
            PERIPH2_CLK_SEL_LOADEDW::PERIPH2_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERIPH2_CLK_SEL_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPH2_CLK_SEL_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERIPH2_CLK_SEL_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt is not generated due to frequency change of periph2_clk_sel"]
    #[inline]
    pub fn periph2_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(PERIPH2_CLK_SEL_LOADEDW::PERIPH2_CLK_SEL_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of periph2_clk_sel"]
    #[inline]
    pub fn periph2_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(PERIPH2_CLK_SEL_LOADEDW::PERIPH2_CLK_SEL_LOADED_1)
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
#[doc = "Values that can be written to the field `AHB_PODF_LOADED`"]
pub enum AHB_PODF_LOADEDW {
    #[doc = "interrupt is not generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_0,
    #[doc = "interrupt generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_1,
}
impl AHB_PODF_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHB_PODF_LOADEDW::AHB_PODF_LOADED_0 => false,
            AHB_PODF_LOADEDW::AHB_PODF_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_PODF_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_PODF_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_PODF_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt is not generated due to frequency change of ahb_podf"]
    #[inline]
    pub fn ahb_podf_loaded_0(self) -> &'a mut W {
        self.variant(AHB_PODF_LOADEDW::AHB_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of ahb_podf"]
    #[inline]
    pub fn ahb_podf_loaded_1(self) -> &'a mut W {
        self.variant(AHB_PODF_LOADEDW::AHB_PODF_LOADED_1)
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
#[doc = "Values that can be written to the field `PERIPH_CLK_SEL_LOADED`"]
pub enum PERIPH_CLK_SEL_LOADEDW {
    #[doc = "interrupt is not generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_0,
    #[doc = "interrupt generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_1,
}
impl PERIPH_CLK_SEL_LOADEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERIPH_CLK_SEL_LOADEDW::PERIPH_CLK_SEL_LOADED_0 => false,
            PERIPH_CLK_SEL_LOADEDW::PERIPH_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERIPH_CLK_SEL_LOADEDW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPH_CLK_SEL_LOADEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERIPH_CLK_SEL_LOADEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt is not generated due to update of periph_clk_sel."]
    #[inline]
    pub fn periph_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_LOADEDW::PERIPH_CLK_SEL_LOADED_0)
    }
    #[doc = "interrupt generated due to update of periph_clk_sel."]
    #[inline]
    pub fn periph_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_LOADEDW::PERIPH_CLK_SEL_LOADED_1)
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
    #[doc = "interrupt is not generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_0,
    #[doc = "interrupt generated due to frequency change of arm_podf"]
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
    #[doc = "interrupt is not generated due to frequency change of arm_podf"]
    #[inline]
    pub fn arm_podf_loaded_0(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADEDW::ARM_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of arm_podf"]
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
    #[doc = "Bit 0 - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline]
    pub fn lrf_pll(&self) -> LRF_PLLR {
        LRF_PLLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline]
    pub fn cosc_ready(&self) -> COSC_READYR {
        COSC_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[inline]
    pub fn semc_podf_loaded(&self) -> SEMC_PODF_LOADEDR {
        SEMC_PODF_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[inline]
    pub fn periph2_clk_sel_loaded(&self) -> PERIPH2_CLK_SEL_LOADEDR {
        PERIPH2_CLK_SEL_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline]
    pub fn ahb_podf_loaded(&self) -> AHB_PODF_LOADEDR {
        AHB_PODF_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline]
    pub fn periph_clk_sel_loaded(&self) -> PERIPH_CLK_SEL_LOADEDR {
        PERIPH_CLK_SEL_LOADEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - CCM interrupt request 1 generated due to frequency change of arm_podf"]
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline]
    pub fn lrf_pll(&mut self) -> _LRF_PLLW {
        _LRF_PLLW { w: self }
    }
    #[doc = "Bit 6 - CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline]
    pub fn cosc_ready(&mut self) -> _COSC_READYW {
        _COSC_READYW { w: self }
    }
    #[doc = "Bit 17 - CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[inline]
    pub fn semc_podf_loaded(&mut self) -> _SEMC_PODF_LOADEDW {
        _SEMC_PODF_LOADEDW { w: self }
    }
    #[doc = "Bit 19 - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[inline]
    pub fn periph2_clk_sel_loaded(&mut self) -> _PERIPH2_CLK_SEL_LOADEDW {
        _PERIPH2_CLK_SEL_LOADEDW { w: self }
    }
    #[doc = "Bit 20 - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline]
    pub fn ahb_podf_loaded(&mut self) -> _AHB_PODF_LOADEDW {
        _AHB_PODF_LOADEDW { w: self }
    }
    #[doc = "Bit 22 - CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline]
    pub fn periph_clk_sel_loaded(&mut self) -> _PERIPH_CLK_SEL_LOADEDW {
        _PERIPH_CLK_SEL_LOADEDW { w: self }
    }
    #[doc = "Bit 26 - CCM interrupt request 1 generated due to frequency change of arm_podf"]
    #[inline]
    pub fn arm_podf_loaded(&mut self) -> _ARM_PODF_LOADEDW {
        _ARM_PODF_LOADEDW { w: self }
    }
}
