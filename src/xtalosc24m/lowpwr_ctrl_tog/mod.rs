#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOWPWR_CTRL_TOG {
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
#[doc = "Possible values of the field `RC_OSC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RC_OSC_ENR {
    #[doc = "Use XTAL OSC to source the 24MHz clock"]
    RC_OSC_EN_0,
    #[doc = "Use RC OSC"]
    RC_OSC_EN_1,
}
impl RC_OSC_ENR {
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
            RC_OSC_ENR::RC_OSC_EN_0 => false,
            RC_OSC_ENR::RC_OSC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RC_OSC_ENR {
        match value {
            false => RC_OSC_ENR::RC_OSC_EN_0,
            true => RC_OSC_ENR::RC_OSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_EN_0`"]
    #[inline]
    pub fn is_rc_osc_en_0(&self) -> bool {
        *self == RC_OSC_ENR::RC_OSC_EN_0
    }
    #[doc = "Checks if the value of the field is `RC_OSC_EN_1`"]
    #[inline]
    pub fn is_rc_osc_en_1(&self) -> bool {
        *self == RC_OSC_ENR::RC_OSC_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct RC_OSC_PROGR {
    bits: u8,
}
impl RC_OSC_PROGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OSC_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_SELR {
    #[doc = "XTAL OSC"]
    OSC_SEL_0,
    #[doc = "RC OSC"]
    OSC_SEL_1,
}
impl OSC_SELR {
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
            OSC_SELR::OSC_SEL_0 => false,
            OSC_SELR::OSC_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_SELR {
        match value {
            false => OSC_SELR::OSC_SEL_0,
            true => OSC_SELR::OSC_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_SEL_0`"]
    #[inline]
    pub fn is_osc_sel_0(&self) -> bool {
        *self == OSC_SELR::OSC_SEL_0
    }
    #[doc = "Checks if the value of the field is `OSC_SEL_1`"]
    #[inline]
    pub fn is_osc_sel_1(&self) -> bool {
        *self == OSC_SELR::OSC_SEL_1
    }
}
#[doc = "Possible values of the field `LPBG_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBG_SELR {
    #[doc = "Normal power bandgap"]
    LPBG_SEL_0,
    #[doc = "Low power bandgap"]
    LPBG_SEL_1,
}
impl LPBG_SELR {
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
            LPBG_SELR::LPBG_SEL_0 => false,
            LPBG_SELR::LPBG_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBG_SELR {
        match value {
            false => LPBG_SELR::LPBG_SEL_0,
            true => LPBG_SELR::LPBG_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPBG_SEL_0`"]
    #[inline]
    pub fn is_lpbg_sel_0(&self) -> bool {
        *self == LPBG_SELR::LPBG_SEL_0
    }
    #[doc = "Checks if the value of the field is `LPBG_SEL_1`"]
    #[inline]
    pub fn is_lpbg_sel_1(&self) -> bool {
        *self == LPBG_SELR::LPBG_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct LPBG_TESTR {
    bits: bool,
}
impl LPBG_TESTR {
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
pub struct REFTOP_IBIAS_OFFR {
    bits: bool,
}
impl REFTOP_IBIAS_OFFR {
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
pub struct L1_PWRGATER {
    bits: bool,
}
impl L1_PWRGATER {
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
pub struct L2_PWRGATER {
    bits: bool,
}
impl L2_PWRGATER {
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
pub struct CPU_PWRGATER {
    bits: bool,
}
impl CPU_PWRGATER {
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
pub struct DISPLAY_PWRGATER {
    bits: bool,
}
impl DISPLAY_PWRGATER {
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
pub struct RCOSC_CG_OVERRIDER {
    bits: bool,
}
impl RCOSC_CG_OVERRIDER {
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
#[doc = "Possible values of the field `XTALOSC_PWRUP_DELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOSC_PWRUP_DELAYR {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3,
}
impl XTALOSC_PWRUP_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_0 => 0,
            XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_1 => 1,
            XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_2 => 2,
            XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XTALOSC_PWRUP_DELAYR {
        match value {
            0 => XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_0,
            1 => XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_1,
            2 => XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_2,
            3 => XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_0`"]
    #[inline]
    pub fn is_xtalosc_pwrup_delay_0(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_0
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_1`"]
    #[inline]
    pub fn is_xtalosc_pwrup_delay_1(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_1
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_2`"]
    #[inline]
    pub fn is_xtalosc_pwrup_delay_2(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_2
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_3`"]
    #[inline]
    pub fn is_xtalosc_pwrup_delay_3(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAYR::XTALOSC_PWRUP_DELAY_3
    }
}
#[doc = "Possible values of the field `XTALOSC_PWRUP_STAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOSC_PWRUP_STATR {
    #[doc = "Not stable"]
    XTALOSC_PWRUP_STAT_0,
    #[doc = "Stable and ready to use"]
    XTALOSC_PWRUP_STAT_1,
}
impl XTALOSC_PWRUP_STATR {
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
            XTALOSC_PWRUP_STATR::XTALOSC_PWRUP_STAT_0 => false,
            XTALOSC_PWRUP_STATR::XTALOSC_PWRUP_STAT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALOSC_PWRUP_STATR {
        match value {
            false => XTALOSC_PWRUP_STATR::XTALOSC_PWRUP_STAT_0,
            true => XTALOSC_PWRUP_STATR::XTALOSC_PWRUP_STAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_STAT_0`"]
    #[inline]
    pub fn is_xtalosc_pwrup_stat_0(&self) -> bool {
        *self == XTALOSC_PWRUP_STATR::XTALOSC_PWRUP_STAT_0
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_STAT_1`"]
    #[inline]
    pub fn is_xtalosc_pwrup_stat_1(&self) -> bool {
        *self == XTALOSC_PWRUP_STATR::XTALOSC_PWRUP_STAT_1
    }
}
#[doc = r" Value of the field"]
pub struct MIX_PWRGATER {
    bits: bool,
}
impl MIX_PWRGATER {
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
#[doc = "Values that can be written to the field `RC_OSC_EN`"]
pub enum RC_OSC_ENW {
    #[doc = "Use XTAL OSC to source the 24MHz clock"]
    RC_OSC_EN_0,
    #[doc = "Use RC OSC"]
    RC_OSC_EN_1,
}
impl RC_OSC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RC_OSC_ENW::RC_OSC_EN_0 => false,
            RC_OSC_ENW::RC_OSC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RC_OSC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RC_OSC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RC_OSC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use XTAL OSC to source the 24MHz clock"]
    #[inline]
    pub fn rc_osc_en_0(self) -> &'a mut W {
        self.variant(RC_OSC_ENW::RC_OSC_EN_0)
    }
    #[doc = "Use RC OSC"]
    #[inline]
    pub fn rc_osc_en_1(self) -> &'a mut W {
        self.variant(RC_OSC_ENW::RC_OSC_EN_1)
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
#[doc = r" Proxy"]
pub struct _RC_OSC_PROGW<'a> {
    w: &'a mut W,
}
impl<'a> _RC_OSC_PROGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSC_SEL`"]
pub enum OSC_SELW {
    #[doc = "XTAL OSC"]
    OSC_SEL_0,
    #[doc = "RC OSC"]
    OSC_SEL_1,
}
impl OSC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSC_SELW::OSC_SEL_0 => false,
            OSC_SELW::OSC_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XTAL OSC"]
    #[inline]
    pub fn osc_sel_0(self) -> &'a mut W {
        self.variant(OSC_SELW::OSC_SEL_0)
    }
    #[doc = "RC OSC"]
    #[inline]
    pub fn osc_sel_1(self) -> &'a mut W {
        self.variant(OSC_SELW::OSC_SEL_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPBG_SEL`"]
pub enum LPBG_SELW {
    #[doc = "Normal power bandgap"]
    LPBG_SEL_0,
    #[doc = "Low power bandgap"]
    LPBG_SEL_1,
}
impl LPBG_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPBG_SELW::LPBG_SEL_0 => false,
            LPBG_SELW::LPBG_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPBG_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPBG_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPBG_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal power bandgap"]
    #[inline]
    pub fn lpbg_sel_0(self) -> &'a mut W {
        self.variant(LPBG_SELW::LPBG_SEL_0)
    }
    #[doc = "Low power bandgap"]
    #[inline]
    pub fn lpbg_sel_1(self) -> &'a mut W {
        self.variant(LPBG_SELW::LPBG_SEL_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPBG_TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _LPBG_TESTW<'a> {
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
#[doc = r" Proxy"]
pub struct _REFTOP_IBIAS_OFFW<'a> {
    w: &'a mut W,
}
impl<'a> _REFTOP_IBIAS_OFFW<'a> {
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
#[doc = r" Proxy"]
pub struct _L1_PWRGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _L1_PWRGATEW<'a> {
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
#[doc = r" Proxy"]
pub struct _L2_PWRGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _L2_PWRGATEW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPU_PWRGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU_PWRGATEW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISPLAY_PWRGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPLAY_PWRGATEW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSC_CG_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_CG_OVERRIDEW<'a> {
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
#[doc = "Values that can be written to the field `XTALOSC_PWRUP_DELAY`"]
pub enum XTALOSC_PWRUP_DELAYW {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3,
}
impl XTALOSC_PWRUP_DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_0 => 0,
            XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_1 => 1,
            XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_2 => 2,
            XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALOSC_PWRUP_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALOSC_PWRUP_DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALOSC_PWRUP_DELAYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.25ms"]
    #[inline]
    pub fn xtalosc_pwrup_delay_0(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_0)
    }
    #[doc = "0.5ms"]
    #[inline]
    pub fn xtalosc_pwrup_delay_1(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_1)
    }
    #[doc = "1ms"]
    #[inline]
    pub fn xtalosc_pwrup_delay_2(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_2)
    }
    #[doc = "2ms"]
    #[inline]
    pub fn xtalosc_pwrup_delay_3(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAYW::XTALOSC_PWRUP_DELAY_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MIX_PWRGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _MIX_PWRGATEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RC Osc. enable control."]
    #[inline]
    pub fn rc_osc_en(&self) -> RC_OSC_ENR {
        RC_OSC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - RC osc. tuning values."]
    #[inline]
    pub fn rc_osc_prog(&self) -> RC_OSC_PROGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RC_OSC_PROGR { bits }
    }
    #[doc = "Bit 4 - Select the source for the 24MHz clock."]
    #[inline]
    pub fn osc_sel(&self) -> OSC_SELR {
        OSC_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Bandgap select. Not related to oscillator."]
    #[inline]
    pub fn lpbg_sel(&self) -> LPBG_SELR {
        LPBG_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Low power bandgap test bit. Not related to oscillator."]
    #[inline]
    pub fn lpbg_test(&self) -> LPBG_TESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPBG_TESTR { bits }
    }
    #[doc = "Bit 7 - Low power reftop ibias disable. Not related to oscillator."]
    #[inline]
    pub fn reftop_ibias_off(&self) -> REFTOP_IBIAS_OFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REFTOP_IBIAS_OFFR { bits }
    }
    #[doc = "Bit 8 - L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline]
    pub fn l1_pwrgate(&self) -> L1_PWRGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        L1_PWRGATER { bits }
    }
    #[doc = "Bit 9 - L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline]
    pub fn l2_pwrgate(&self) -> L2_PWRGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        L2_PWRGATER { bits }
    }
    #[doc = "Bit 10 - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline]
    pub fn cpu_pwrgate(&self) -> CPU_PWRGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPU_PWRGATER { bits }
    }
    #[doc = "Bit 11 - Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline]
    pub fn display_pwrgate(&self) -> DISPLAY_PWRGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPLAY_PWRGATER { bits }
    }
    #[doc = "Bit 13 - For debug purposes only"]
    #[inline]
    pub fn rcosc_cg_override(&self) -> RCOSC_CG_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSC_CG_OVERRIDER { bits }
    }
    #[doc = "Bits 14:15 - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline]
    pub fn xtalosc_pwrup_delay(&self) -> XTALOSC_PWRUP_DELAYR {
        XTALOSC_PWRUP_DELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Status of the 24MHz xtal oscillator."]
    #[inline]
    pub fn xtalosc_pwrup_stat(&self) -> XTALOSC_PWRUP_STATR {
        XTALOSC_PWRUP_STATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline]
    pub fn mix_pwrgate(&self) -> MIX_PWRGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIX_PWRGATER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16393 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RC Osc. enable control."]
    #[inline]
    pub fn rc_osc_en(&mut self) -> _RC_OSC_ENW {
        _RC_OSC_ENW { w: self }
    }
    #[doc = "Bits 1:3 - RC osc. tuning values."]
    #[inline]
    pub fn rc_osc_prog(&mut self) -> _RC_OSC_PROGW {
        _RC_OSC_PROGW { w: self }
    }
    #[doc = "Bit 4 - Select the source for the 24MHz clock."]
    #[inline]
    pub fn osc_sel(&mut self) -> _OSC_SELW {
        _OSC_SELW { w: self }
    }
    #[doc = "Bit 5 - Bandgap select. Not related to oscillator."]
    #[inline]
    pub fn lpbg_sel(&mut self) -> _LPBG_SELW {
        _LPBG_SELW { w: self }
    }
    #[doc = "Bit 6 - Low power bandgap test bit. Not related to oscillator."]
    #[inline]
    pub fn lpbg_test(&mut self) -> _LPBG_TESTW {
        _LPBG_TESTW { w: self }
    }
    #[doc = "Bit 7 - Low power reftop ibias disable. Not related to oscillator."]
    #[inline]
    pub fn reftop_ibias_off(&mut self) -> _REFTOP_IBIAS_OFFW {
        _REFTOP_IBIAS_OFFW { w: self }
    }
    #[doc = "Bit 8 - L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline]
    pub fn l1_pwrgate(&mut self) -> _L1_PWRGATEW {
        _L1_PWRGATEW { w: self }
    }
    #[doc = "Bit 9 - L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline]
    pub fn l2_pwrgate(&mut self) -> _L2_PWRGATEW {
        _L2_PWRGATEW { w: self }
    }
    #[doc = "Bit 10 - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline]
    pub fn cpu_pwrgate(&mut self) -> _CPU_PWRGATEW {
        _CPU_PWRGATEW { w: self }
    }
    #[doc = "Bit 11 - Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline]
    pub fn display_pwrgate(&mut self) -> _DISPLAY_PWRGATEW {
        _DISPLAY_PWRGATEW { w: self }
    }
    #[doc = "Bit 13 - For debug purposes only"]
    #[inline]
    pub fn rcosc_cg_override(&mut self) -> _RCOSC_CG_OVERRIDEW {
        _RCOSC_CG_OVERRIDEW { w: self }
    }
    #[doc = "Bits 14:15 - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline]
    pub fn xtalosc_pwrup_delay(&mut self) -> _XTALOSC_PWRUP_DELAYW {
        _XTALOSC_PWRUP_DELAYW { w: self }
    }
    #[doc = "Bit 17 - Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline]
    pub fn mix_pwrgate(&mut self) -> _MIX_PWRGATEW {
        _MIX_PWRGATEW { w: self }
    }
}
