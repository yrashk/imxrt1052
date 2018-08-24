#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL_ENET_TOG {
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
#[doc = "Possible values of the field `ENET0_DIV_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET0_DIV_SELECTR {
    #[doc = "25MHz"]
    ENET0_DIV_SELECT_0,
    #[doc = "50MHz"]
    ENET0_DIV_SELECT_1,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET0_DIV_SELECT_2,
    #[doc = "125MHz"]
    ENET0_DIV_SELECT_3,
}
impl ENET0_DIV_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENET0_DIV_SELECTR::ENET0_DIV_SELECT_0 => 0,
            ENET0_DIV_SELECTR::ENET0_DIV_SELECT_1 => 1,
            ENET0_DIV_SELECTR::ENET0_DIV_SELECT_2 => 2,
            ENET0_DIV_SELECTR::ENET0_DIV_SELECT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENET0_DIV_SELECTR {
        match value {
            0 => ENET0_DIV_SELECTR::ENET0_DIV_SELECT_0,
            1 => ENET0_DIV_SELECTR::ENET0_DIV_SELECT_1,
            2 => ENET0_DIV_SELECTR::ENET0_DIV_SELECT_2,
            3 => ENET0_DIV_SELECTR::ENET0_DIV_SELECT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENET0_DIV_SELECT_0`"]
    #[inline]
    pub fn is_enet0_div_select_0(&self) -> bool {
        *self == ENET0_DIV_SELECTR::ENET0_DIV_SELECT_0
    }
    #[doc = "Checks if the value of the field is `ENET0_DIV_SELECT_1`"]
    #[inline]
    pub fn is_enet0_div_select_1(&self) -> bool {
        *self == ENET0_DIV_SELECTR::ENET0_DIV_SELECT_1
    }
    #[doc = "Checks if the value of the field is `ENET0_DIV_SELECT_2`"]
    #[inline]
    pub fn is_enet0_div_select_2(&self) -> bool {
        *self == ENET0_DIV_SELECTR::ENET0_DIV_SELECT_2
    }
    #[doc = "Checks if the value of the field is `ENET0_DIV_SELECT_3`"]
    #[inline]
    pub fn is_enet0_div_select_3(&self) -> bool {
        *self == ENET0_DIV_SELECTR::ENET0_DIV_SELECT_3
    }
}
#[doc = "Possible values of the field `ENET1_DIV_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET1_DIV_SELECTR {
    #[doc = "25MHz"]
    ENET1_DIV_SELECT_0,
    #[doc = "50MHz"]
    ENET1_DIV_SELECT_1,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET1_DIV_SELECT_2,
    #[doc = "125MHz"]
    ENET1_DIV_SELECT_3,
}
impl ENET1_DIV_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENET1_DIV_SELECTR::ENET1_DIV_SELECT_0 => 0,
            ENET1_DIV_SELECTR::ENET1_DIV_SELECT_1 => 1,
            ENET1_DIV_SELECTR::ENET1_DIV_SELECT_2 => 2,
            ENET1_DIV_SELECTR::ENET1_DIV_SELECT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENET1_DIV_SELECTR {
        match value {
            0 => ENET1_DIV_SELECTR::ENET1_DIV_SELECT_0,
            1 => ENET1_DIV_SELECTR::ENET1_DIV_SELECT_1,
            2 => ENET1_DIV_SELECTR::ENET1_DIV_SELECT_2,
            3 => ENET1_DIV_SELECTR::ENET1_DIV_SELECT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENET1_DIV_SELECT_0`"]
    #[inline]
    pub fn is_enet1_div_select_0(&self) -> bool {
        *self == ENET1_DIV_SELECTR::ENET1_DIV_SELECT_0
    }
    #[doc = "Checks if the value of the field is `ENET1_DIV_SELECT_1`"]
    #[inline]
    pub fn is_enet1_div_select_1(&self) -> bool {
        *self == ENET1_DIV_SELECTR::ENET1_DIV_SELECT_1
    }
    #[doc = "Checks if the value of the field is `ENET1_DIV_SELECT_2`"]
    #[inline]
    pub fn is_enet1_div_select_2(&self) -> bool {
        *self == ENET1_DIV_SELECTR::ENET1_DIV_SELECT_2
    }
    #[doc = "Checks if the value of the field is `ENET1_DIV_SELECT_3`"]
    #[inline]
    pub fn is_enet1_div_select_3(&self) -> bool {
        *self == ENET1_DIV_SELECTR::ENET1_DIV_SELECT_3
    }
}
#[doc = r" Value of the field"]
pub struct POWERDOWNR {
    bits: bool,
}
impl POWERDOWNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENET1_125M_ENR {
    bits: bool,
}
impl ENET1_125M_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `BYPASS_CLK_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_CLK_SRCR {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYPASS_CLK_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYPASS_CLK_SRCR::REF_CLK_24M => 0,
            BYPASS_CLK_SRCR::CLK1 => 1,
            BYPASS_CLK_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYPASS_CLK_SRCR {
        match value {
            0 => BYPASS_CLK_SRCR::REF_CLK_24M,
            1 => BYPASS_CLK_SRCR::CLK1,
            i => BYPASS_CLK_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REF_CLK_24M`"]
    #[inline]
    pub fn is_ref_clk_24m(&self) -> bool {
        *self == BYPASS_CLK_SRCR::REF_CLK_24M
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline]
    pub fn is_clk1(&self) -> bool {
        *self == BYPASS_CLK_SRCR::CLK1
    }
}
#[doc = r" Value of the field"]
pub struct BYPASSR {
    bits: bool,
}
impl BYPASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PFD_OFFSET_ENR {
    bits: bool,
}
impl PFD_OFFSET_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENABLE_125MR {
    bits: bool,
}
impl ENABLE_125MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENET2_125M_ENR {
    bits: bool,
}
impl ENET2_125M_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENET_25M_REF_ENR {
    bits: bool,
}
impl ENET_25M_REF_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct LOCKR {
    bits: bool,
}
impl LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `ENET0_DIV_SELECT`"]
pub enum ENET0_DIV_SELECTW {
    #[doc = "25MHz"]
    ENET0_DIV_SELECT_0,
    #[doc = "50MHz"]
    ENET0_DIV_SELECT_1,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET0_DIV_SELECT_2,
    #[doc = "125MHz"]
    ENET0_DIV_SELECT_3,
}
impl ENET0_DIV_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENET0_DIV_SELECTW::ENET0_DIV_SELECT_0 => 0,
            ENET0_DIV_SELECTW::ENET0_DIV_SELECT_1 => 1,
            ENET0_DIV_SELECTW::ENET0_DIV_SELECT_2 => 2,
            ENET0_DIV_SELECTW::ENET0_DIV_SELECT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENET0_DIV_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET0_DIV_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENET0_DIV_SELECTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "25MHz"]
    #[inline]
    pub fn enet0_div_select_0(self) -> &'a mut W {
        self.variant(ENET0_DIV_SELECTW::ENET0_DIV_SELECT_0)
    }
    #[doc = "50MHz"]
    #[inline]
    pub fn enet0_div_select_1(self) -> &'a mut W {
        self.variant(ENET0_DIV_SELECTW::ENET0_DIV_SELECT_1)
    }
    #[doc = "100MHz (not 50% duty cycle)"]
    #[inline]
    pub fn enet0_div_select_2(self) -> &'a mut W {
        self.variant(ENET0_DIV_SELECTW::ENET0_DIV_SELECT_2)
    }
    #[doc = "125MHz"]
    #[inline]
    pub fn enet0_div_select_3(self) -> &'a mut W {
        self.variant(ENET0_DIV_SELECTW::ENET0_DIV_SELECT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENET1_DIV_SELECT`"]
pub enum ENET1_DIV_SELECTW {
    #[doc = "25MHz"]
    ENET1_DIV_SELECT_0,
    #[doc = "50MHz"]
    ENET1_DIV_SELECT_1,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET1_DIV_SELECT_2,
    #[doc = "125MHz"]
    ENET1_DIV_SELECT_3,
}
impl ENET1_DIV_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENET1_DIV_SELECTW::ENET1_DIV_SELECT_0 => 0,
            ENET1_DIV_SELECTW::ENET1_DIV_SELECT_1 => 1,
            ENET1_DIV_SELECTW::ENET1_DIV_SELECT_2 => 2,
            ENET1_DIV_SELECTW::ENET1_DIV_SELECT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENET1_DIV_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET1_DIV_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENET1_DIV_SELECTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "25MHz"]
    #[inline]
    pub fn enet1_div_select_0(self) -> &'a mut W {
        self.variant(ENET1_DIV_SELECTW::ENET1_DIV_SELECT_0)
    }
    #[doc = "50MHz"]
    #[inline]
    pub fn enet1_div_select_1(self) -> &'a mut W {
        self.variant(ENET1_DIV_SELECTW::ENET1_DIV_SELECT_1)
    }
    #[doc = "100MHz (not 50% duty cycle)"]
    #[inline]
    pub fn enet1_div_select_2(self) -> &'a mut W {
        self.variant(ENET1_DIV_SELECTW::ENET1_DIV_SELECT_2)
    }
    #[doc = "125MHz"]
    #[inline]
    pub fn enet1_div_select_3(self) -> &'a mut W {
        self.variant(ENET1_DIV_SELECTW::ENET1_DIV_SELECT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POWERDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _POWERDOWNW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENET1_125M_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET1_125M_ENW<'a> {
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
#[doc = "Values that can be written to the field `BYPASS_CLK_SRC`"]
pub enum BYPASS_CLK_SRCW {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1,
}
impl BYPASS_CLK_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BYPASS_CLK_SRCW::REF_CLK_24M => 0,
            BYPASS_CLK_SRCW::CLK1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASS_CLK_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_CLK_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASS_CLK_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select the 24MHz oscillator as source."]
    #[inline]
    pub fn ref_clk_24m(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRCW::REF_CLK_24M)
    }
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    #[inline]
    pub fn clk1(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRCW::CLK1)
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
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
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
#[doc = r" Proxy"]
pub struct _PFD_OFFSET_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PFD_OFFSET_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENABLE_125MW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_125MW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENET2_125M_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET2_125M_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENET_25M_REF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET_25M_REF_ENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Controls the frequency of the ethernet0 reference clock."]
    #[inline]
    pub fn enet0_div_select(&self) -> ENET0_DIV_SELECTR {
        ENET0_DIV_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Controls the frequency of the ethernet1 reference clock."]
    #[inline]
    pub fn enet1_div_select(&self) -> ENET1_DIV_SELECTR {
        ENET1_DIV_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Powers down the PLL."]
    #[inline]
    pub fn powerdown(&self) -> POWERDOWNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POWERDOWNR { bits }
    }
    #[doc = "Bit 13 - Enable the PLL providing the ENET1 125 MHz reference clock."]
    #[inline]
    pub fn enet1_125m_en(&self) -> ENET1_125M_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENET1_125M_ENR { bits }
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline]
    pub fn bypass_clk_src(&self) -> BYPASS_CLK_SRCR {
        BYPASS_CLK_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASSR { bits }
    }
    #[doc = "Bit 18 - Enables an offset in the phase frequency detector."]
    #[inline]
    pub fn pfd_offset_en(&self) -> PFD_OFFSET_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFD_OFFSET_ENR { bits }
    }
    #[doc = "Bit 19 - Enables an offset in the phase frequency detector."]
    #[inline]
    pub fn enable_125m(&self) -> ENABLE_125MR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_125MR { bits }
    }
    #[doc = "Bit 20 - Enable the PLL providing the ENET2 125 MHz reference clock"]
    #[inline]
    pub fn enet2_125m_en(&self) -> ENET2_125M_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENET2_125M_ENR { bits }
    }
    #[doc = "Bit 21 - Enable the PLL providing ENET 25 MHz reference clock"]
    #[inline]
    pub fn enet_25m_ref_en(&self) -> ENET_25M_REF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENET_25M_REF_ENR { bits }
    }
    #[doc = "Bit 31 - 1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 69633 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Controls the frequency of the ethernet0 reference clock."]
    #[inline]
    pub fn enet0_div_select(&mut self) -> _ENET0_DIV_SELECTW {
        _ENET0_DIV_SELECTW { w: self }
    }
    #[doc = "Bits 2:3 - Controls the frequency of the ethernet1 reference clock."]
    #[inline]
    pub fn enet1_div_select(&mut self) -> _ENET1_DIV_SELECTW {
        _ENET1_DIV_SELECTW { w: self }
    }
    #[doc = "Bit 12 - Powers down the PLL."]
    #[inline]
    pub fn powerdown(&mut self) -> _POWERDOWNW {
        _POWERDOWNW { w: self }
    }
    #[doc = "Bit 13 - Enable the PLL providing the ENET1 125 MHz reference clock."]
    #[inline]
    pub fn enet1_125m_en(&mut self) -> _ENET1_125M_ENW {
        _ENET1_125M_ENW { w: self }
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline]
    pub fn bypass_clk_src(&mut self) -> _BYPASS_CLK_SRCW {
        _BYPASS_CLK_SRCW { w: self }
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 18 - Enables an offset in the phase frequency detector."]
    #[inline]
    pub fn pfd_offset_en(&mut self) -> _PFD_OFFSET_ENW {
        _PFD_OFFSET_ENW { w: self }
    }
    #[doc = "Bit 19 - Enables an offset in the phase frequency detector."]
    #[inline]
    pub fn enable_125m(&mut self) -> _ENABLE_125MW {
        _ENABLE_125MW { w: self }
    }
    #[doc = "Bit 20 - Enable the PLL providing the ENET2 125 MHz reference clock"]
    #[inline]
    pub fn enet2_125m_en(&mut self) -> _ENET2_125M_ENW {
        _ENET2_125M_ENW { w: self }
    }
    #[doc = "Bit 21 - Enable the PLL providing ENET 25 MHz reference clock"]
    #[inline]
    pub fn enet_25m_ref_en(&mut self) -> _ENET_25M_REF_ENW {
        _ENET_25M_REF_ENW { w: self }
    }
}
