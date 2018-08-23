#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR5 {
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
#[doc = "Possible values of the field `WDOG1_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG1_MASKR {
    #[doc = "WDOG1 Timeout behaves normally"]
    WDOG1_MASK_0,
    #[doc = "WDOG1 Timeout is masked"]
    WDOG1_MASK_1,
}
impl WDOG1_MASKR {
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
            WDOG1_MASKR::WDOG1_MASK_0 => false,
            WDOG1_MASKR::WDOG1_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDOG1_MASKR {
        match value {
            false => WDOG1_MASKR::WDOG1_MASK_0,
            true => WDOG1_MASKR::WDOG1_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG1_MASK_0`"]
    #[inline]
    pub fn is_wdog1_mask_0(&self) -> bool {
        *self == WDOG1_MASKR::WDOG1_MASK_0
    }
    #[doc = "Checks if the value of the field is `WDOG1_MASK_1`"]
    #[inline]
    pub fn is_wdog1_mask_1(&self) -> bool {
        *self == WDOG1_MASKR::WDOG1_MASK_1
    }
}
#[doc = "Possible values of the field `WDOG2_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG2_MASKR {
    #[doc = "WDOG2 Timeout behaves normally"]
    WDOG2_MASK_0,
    #[doc = "WDOG2 Timeout is masked"]
    WDOG2_MASK_1,
}
impl WDOG2_MASKR {
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
            WDOG2_MASKR::WDOG2_MASK_0 => false,
            WDOG2_MASKR::WDOG2_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDOG2_MASKR {
        match value {
            false => WDOG2_MASKR::WDOG2_MASK_0,
            true => WDOG2_MASKR::WDOG2_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG2_MASK_0`"]
    #[inline]
    pub fn is_wdog2_mask_0(&self) -> bool {
        *self == WDOG2_MASKR::WDOG2_MASK_0
    }
    #[doc = "Checks if the value of the field is `WDOG2_MASK_1`"]
    #[inline]
    pub fn is_wdog2_mask_1(&self) -> bool {
        *self == WDOG2_MASKR::WDOG2_MASK_1
    }
}
#[doc = "Possible values of the field `GPT2_CAPIN1_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPT2_CAPIN1_SELR {
    #[doc = "source from pad"]
    GPT2_CAPIN1_SEL_0,
    #[doc = "source from enet1.ipp_do_mac0_timer[3]"]
    GPT2_CAPIN1_SEL_1,
}
impl GPT2_CAPIN1_SELR {
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
            GPT2_CAPIN1_SELR::GPT2_CAPIN1_SEL_0 => false,
            GPT2_CAPIN1_SELR::GPT2_CAPIN1_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPT2_CAPIN1_SELR {
        match value {
            false => GPT2_CAPIN1_SELR::GPT2_CAPIN1_SEL_0,
            true => GPT2_CAPIN1_SELR::GPT2_CAPIN1_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN1_SEL_0`"]
    #[inline]
    pub fn is_gpt2_capin1_sel_0(&self) -> bool {
        *self == GPT2_CAPIN1_SELR::GPT2_CAPIN1_SEL_0
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN1_SEL_1`"]
    #[inline]
    pub fn is_gpt2_capin1_sel_1(&self) -> bool {
        *self == GPT2_CAPIN1_SELR::GPT2_CAPIN1_SEL_1
    }
}
#[doc = "Possible values of the field `GPT2_CAPIN2_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPT2_CAPIN2_SELR {
    #[doc = "source from pad"]
    GPT2_CAPIN2_SEL_0,
    #[doc = "source from enet2.ipp_do_mac0_timer[3]"]
    GPT2_CAPIN2_SEL_1,
}
impl GPT2_CAPIN2_SELR {
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
            GPT2_CAPIN2_SELR::GPT2_CAPIN2_SEL_0 => false,
            GPT2_CAPIN2_SELR::GPT2_CAPIN2_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPT2_CAPIN2_SELR {
        match value {
            false => GPT2_CAPIN2_SELR::GPT2_CAPIN2_SEL_0,
            true => GPT2_CAPIN2_SELR::GPT2_CAPIN2_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN2_SEL_0`"]
    #[inline]
    pub fn is_gpt2_capin2_sel_0(&self) -> bool {
        *self == GPT2_CAPIN2_SELR::GPT2_CAPIN2_SEL_0
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN2_SEL_1`"]
    #[inline]
    pub fn is_gpt2_capin2_sel_1(&self) -> bool {
        *self == GPT2_CAPIN2_SELR::GPT2_CAPIN2_SEL_1
    }
}
#[doc = "Possible values of the field `ENET_EVENT3IN_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_EVENT3IN_SELR {
    #[doc = "event3 source input from pad"]
    ENET_EVENT3IN_SEL_0,
    #[doc = "event3 source input from gpt2.ipp_do_cmpout1"]
    ENET_EVENT3IN_SEL_1,
}
impl ENET_EVENT3IN_SELR {
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
            ENET_EVENT3IN_SELR::ENET_EVENT3IN_SEL_0 => false,
            ENET_EVENT3IN_SELR::ENET_EVENT3IN_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENET_EVENT3IN_SELR {
        match value {
            false => ENET_EVENT3IN_SELR::ENET_EVENT3IN_SEL_0,
            true => ENET_EVENT3IN_SELR::ENET_EVENT3IN_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_EVENT3IN_SEL_0`"]
    #[inline]
    pub fn is_enet_event3in_sel_0(&self) -> bool {
        *self == ENET_EVENT3IN_SELR::ENET_EVENT3IN_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET_EVENT3IN_SEL_1`"]
    #[inline]
    pub fn is_enet_event3in_sel_1(&self) -> bool {
        *self == ENET_EVENT3IN_SELR::ENET_EVENT3IN_SEL_1
    }
}
#[doc = "Possible values of the field `VREF_1M_CLK_GPT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF_1M_CLK_GPT1R {
    #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT1_0,
    #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT1_1,
}
impl VREF_1M_CLK_GPT1R {
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
            VREF_1M_CLK_GPT1R::VREF_1M_CLK_GPT1_0 => false,
            VREF_1M_CLK_GPT1R::VREF_1M_CLK_GPT1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VREF_1M_CLK_GPT1R {
        match value {
            false => VREF_1M_CLK_GPT1R::VREF_1M_CLK_GPT1_0,
            true => VREF_1M_CLK_GPT1R::VREF_1M_CLK_GPT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT1_0`"]
    #[inline]
    pub fn is_vref_1m_clk_gpt1_0(&self) -> bool {
        *self == VREF_1M_CLK_GPT1R::VREF_1M_CLK_GPT1_0
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT1_1`"]
    #[inline]
    pub fn is_vref_1m_clk_gpt1_1(&self) -> bool {
        *self == VREF_1M_CLK_GPT1R::VREF_1M_CLK_GPT1_1
    }
}
#[doc = "Possible values of the field `VREF_1M_CLK_GPT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF_1M_CLK_GPT2R {
    #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT2_0,
    #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT2_1,
}
impl VREF_1M_CLK_GPT2R {
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
            VREF_1M_CLK_GPT2R::VREF_1M_CLK_GPT2_0 => false,
            VREF_1M_CLK_GPT2R::VREF_1M_CLK_GPT2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VREF_1M_CLK_GPT2R {
        match value {
            false => VREF_1M_CLK_GPT2R::VREF_1M_CLK_GPT2_0,
            true => VREF_1M_CLK_GPT2R::VREF_1M_CLK_GPT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT2_0`"]
    #[inline]
    pub fn is_vref_1m_clk_gpt2_0(&self) -> bool {
        *self == VREF_1M_CLK_GPT2R::VREF_1M_CLK_GPT2_0
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT2_1`"]
    #[inline]
    pub fn is_vref_1m_clk_gpt2_1(&self) -> bool {
        *self == VREF_1M_CLK_GPT2R::VREF_1M_CLK_GPT2_1
    }
}
#[doc = "Values that can be written to the field `WDOG1_MASK`"]
pub enum WDOG1_MASKW {
    #[doc = "WDOG1 Timeout behaves normally"]
    WDOG1_MASK_0,
    #[doc = "WDOG1 Timeout is masked"]
    WDOG1_MASK_1,
}
impl WDOG1_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDOG1_MASKW::WDOG1_MASK_0 => false,
            WDOG1_MASKW::WDOG1_MASK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOG1_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG1_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOG1_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG1 Timeout behaves normally"]
    #[inline]
    pub fn wdog1_mask_0(self) -> &'a mut W {
        self.variant(WDOG1_MASKW::WDOG1_MASK_0)
    }
    #[doc = "WDOG1 Timeout is masked"]
    #[inline]
    pub fn wdog1_mask_1(self) -> &'a mut W {
        self.variant(WDOG1_MASKW::WDOG1_MASK_1)
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
#[doc = "Values that can be written to the field `WDOG2_MASK`"]
pub enum WDOG2_MASKW {
    #[doc = "WDOG2 Timeout behaves normally"]
    WDOG2_MASK_0,
    #[doc = "WDOG2 Timeout is masked"]
    WDOG2_MASK_1,
}
impl WDOG2_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDOG2_MASKW::WDOG2_MASK_0 => false,
            WDOG2_MASKW::WDOG2_MASK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOG2_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG2_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOG2_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG2 Timeout behaves normally"]
    #[inline]
    pub fn wdog2_mask_0(self) -> &'a mut W {
        self.variant(WDOG2_MASKW::WDOG2_MASK_0)
    }
    #[doc = "WDOG2 Timeout is masked"]
    #[inline]
    pub fn wdog2_mask_1(self) -> &'a mut W {
        self.variant(WDOG2_MASKW::WDOG2_MASK_1)
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
#[doc = "Values that can be written to the field `GPT2_CAPIN1_SEL`"]
pub enum GPT2_CAPIN1_SELW {
    #[doc = "source from pad"]
    GPT2_CAPIN1_SEL_0,
    #[doc = "source from enet1.ipp_do_mac0_timer[3]"]
    GPT2_CAPIN1_SEL_1,
}
impl GPT2_CAPIN1_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPT2_CAPIN1_SELW::GPT2_CAPIN1_SEL_0 => false,
            GPT2_CAPIN1_SELW::GPT2_CAPIN1_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPT2_CAPIN1_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _GPT2_CAPIN1_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPT2_CAPIN1_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "source from pad"]
    #[inline]
    pub fn gpt2_capin1_sel_0(self) -> &'a mut W {
        self.variant(GPT2_CAPIN1_SELW::GPT2_CAPIN1_SEL_0)
    }
    #[doc = "source from enet1.ipp_do_mac0_timer[3]"]
    #[inline]
    pub fn gpt2_capin1_sel_1(self) -> &'a mut W {
        self.variant(GPT2_CAPIN1_SELW::GPT2_CAPIN1_SEL_1)
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
#[doc = "Values that can be written to the field `GPT2_CAPIN2_SEL`"]
pub enum GPT2_CAPIN2_SELW {
    #[doc = "source from pad"]
    GPT2_CAPIN2_SEL_0,
    #[doc = "source from enet2.ipp_do_mac0_timer[3]"]
    GPT2_CAPIN2_SEL_1,
}
impl GPT2_CAPIN2_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPT2_CAPIN2_SELW::GPT2_CAPIN2_SEL_0 => false,
            GPT2_CAPIN2_SELW::GPT2_CAPIN2_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPT2_CAPIN2_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _GPT2_CAPIN2_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPT2_CAPIN2_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "source from pad"]
    #[inline]
    pub fn gpt2_capin2_sel_0(self) -> &'a mut W {
        self.variant(GPT2_CAPIN2_SELW::GPT2_CAPIN2_SEL_0)
    }
    #[doc = "source from enet2.ipp_do_mac0_timer[3]"]
    #[inline]
    pub fn gpt2_capin2_sel_1(self) -> &'a mut W {
        self.variant(GPT2_CAPIN2_SELW::GPT2_CAPIN2_SEL_1)
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
#[doc = "Values that can be written to the field `ENET_EVENT3IN_SEL`"]
pub enum ENET_EVENT3IN_SELW {
    #[doc = "event3 source input from pad"]
    ENET_EVENT3IN_SEL_0,
    #[doc = "event3 source input from gpt2.ipp_do_cmpout1"]
    ENET_EVENT3IN_SEL_1,
}
impl ENET_EVENT3IN_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENET_EVENT3IN_SELW::ENET_EVENT3IN_SEL_0 => false,
            ENET_EVENT3IN_SELW::ENET_EVENT3IN_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENET_EVENT3IN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET_EVENT3IN_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENET_EVENT3IN_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "event3 source input from pad"]
    #[inline]
    pub fn enet_event3in_sel_0(self) -> &'a mut W {
        self.variant(ENET_EVENT3IN_SELW::ENET_EVENT3IN_SEL_0)
    }
    #[doc = "event3 source input from gpt2.ipp_do_cmpout1"]
    #[inline]
    pub fn enet_event3in_sel_1(self) -> &'a mut W {
        self.variant(ENET_EVENT3IN_SELW::ENET_EVENT3IN_SEL_1)
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
#[doc = "Values that can be written to the field `VREF_1M_CLK_GPT1`"]
pub enum VREF_1M_CLK_GPT1W {
    #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT1_0,
    #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT1_1,
}
impl VREF_1M_CLK_GPT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VREF_1M_CLK_GPT1W::VREF_1M_CLK_GPT1_0 => false,
            VREF_1M_CLK_GPT1W::VREF_1M_CLK_GPT1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREF_1M_CLK_GPT1W<'a> {
    w: &'a mut W,
}
impl<'a> _VREF_1M_CLK_GPT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREF_1M_CLK_GPT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK"]
    #[inline]
    pub fn vref_1m_clk_gpt1_0(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT1W::VREF_1M_CLK_GPT1_0)
    }
    #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    #[inline]
    pub fn vref_1m_clk_gpt1_1(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT1W::VREF_1M_CLK_GPT1_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VREF_1M_CLK_GPT2`"]
pub enum VREF_1M_CLK_GPT2W {
    #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT2_0,
    #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT2_1,
}
impl VREF_1M_CLK_GPT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VREF_1M_CLK_GPT2W::VREF_1M_CLK_GPT2_0 => false,
            VREF_1M_CLK_GPT2W::VREF_1M_CLK_GPT2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREF_1M_CLK_GPT2W<'a> {
    w: &'a mut W,
}
impl<'a> _VREF_1M_CLK_GPT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREF_1M_CLK_GPT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK"]
    #[inline]
    pub fn vref_1m_clk_gpt2_0(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT2W::VREF_1M_CLK_GPT2_0)
    }
    #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    #[inline]
    pub fn vref_1m_clk_gpt2_1(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT2W::VREF_1M_CLK_GPT2_1)
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
    #[doc = "Bit 6 - WDOG1 Timeout Mask"]
    #[inline]
    pub fn wdog1_mask(&self) -> WDOG1_MASKR {
        WDOG1_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - WDOG2 Timeout Mask"]
    #[inline]
    pub fn wdog2_mask(&self) -> WDOG2_MASKR {
        WDOG2_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - GPT2 input capture channel 1 source select"]
    #[inline]
    pub fn gpt2_capin1_sel(&self) -> GPT2_CAPIN1_SELR {
        GPT2_CAPIN1_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - GPT2 input capture channel 2 source select"]
    #[inline]
    pub fn gpt2_capin2_sel(&self) -> GPT2_CAPIN2_SELR {
        GPT2_CAPIN2_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - ENET input timer event3 source select"]
    #[inline]
    pub fn enet_event3in_sel(&self) -> ENET_EVENT3IN_SELR {
        ENET_EVENT3IN_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - GPT1 1 MHz clock source select"]
    #[inline]
    pub fn vref_1m_clk_gpt1(&self) -> VREF_1M_CLK_GPT1R {
        VREF_1M_CLK_GPT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - GPT2 1 MHz clock source select"]
    #[inline]
    pub fn vref_1m_clk_gpt2(&self) -> VREF_1M_CLK_GPT2R {
        VREF_1M_CLK_GPT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
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
    #[doc = "Bit 6 - WDOG1 Timeout Mask"]
    #[inline]
    pub fn wdog1_mask(&mut self) -> _WDOG1_MASKW {
        _WDOG1_MASKW { w: self }
    }
    #[doc = "Bit 7 - WDOG2 Timeout Mask"]
    #[inline]
    pub fn wdog2_mask(&mut self) -> _WDOG2_MASKW {
        _WDOG2_MASKW { w: self }
    }
    #[doc = "Bit 23 - GPT2 input capture channel 1 source select"]
    #[inline]
    pub fn gpt2_capin1_sel(&mut self) -> _GPT2_CAPIN1_SELW {
        _GPT2_CAPIN1_SELW { w: self }
    }
    #[doc = "Bit 24 - GPT2 input capture channel 2 source select"]
    #[inline]
    pub fn gpt2_capin2_sel(&mut self) -> _GPT2_CAPIN2_SELW {
        _GPT2_CAPIN2_SELW { w: self }
    }
    #[doc = "Bit 25 - ENET input timer event3 source select"]
    #[inline]
    pub fn enet_event3in_sel(&mut self) -> _ENET_EVENT3IN_SELW {
        _ENET_EVENT3IN_SELW { w: self }
    }
    #[doc = "Bit 28 - GPT1 1 MHz clock source select"]
    #[inline]
    pub fn vref_1m_clk_gpt1(&mut self) -> _VREF_1M_CLK_GPT1W {
        _VREF_1M_CLK_GPT1W { w: self }
    }
    #[doc = "Bit 29 - GPT2 1 MHz clock source select"]
    #[inline]
    pub fn vref_1m_clk_gpt2(&mut self) -> _VREF_1M_CLK_GPT2W {
        _VREF_1M_CLK_GPT2W { w: self }
    }
}
