#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC1_CLR {
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
#[doc = "Possible values of the field `LVDS1_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDS1_CLK_SELR {
    #[doc = "Arm PLL"]
    ARM_PLL,
    #[doc = "System PLL"]
    SYS_PLL,
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4,
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5,
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6,
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7,
    #[doc = "Audio PLL"]
    AUDIO_PLL,
    #[doc = "Video PLL"]
    VIDEO_PLL,
    #[doc = "ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF,
    #[doc = "USB1 PLL clock"]
    USB1_PLL,
    #[doc = "USB2 PLL clock"]
    USB2_PLL,
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0,
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1,
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2,
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3,
    #[doc = "xtal (24M)"]
    XTAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LVDS1_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LVDS1_CLK_SELR::ARM_PLL => 0,
            LVDS1_CLK_SELR::SYS_PLL => 1,
            LVDS1_CLK_SELR::PFD4 => 2,
            LVDS1_CLK_SELR::PFD5 => 3,
            LVDS1_CLK_SELR::PFD6 => 4,
            LVDS1_CLK_SELR::PFD7 => 5,
            LVDS1_CLK_SELR::AUDIO_PLL => 6,
            LVDS1_CLK_SELR::VIDEO_PLL => 7,
            LVDS1_CLK_SELR::ETHERNET_REF => 9,
            LVDS1_CLK_SELR::USB1_PLL => 12,
            LVDS1_CLK_SELR::USB2_PLL => 13,
            LVDS1_CLK_SELR::PFD0 => 14,
            LVDS1_CLK_SELR::PFD1 => 15,
            LVDS1_CLK_SELR::PFD2 => 16,
            LVDS1_CLK_SELR::PFD3 => 17,
            LVDS1_CLK_SELR::XTAL => 18,
            LVDS1_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LVDS1_CLK_SELR {
        match value {
            0 => LVDS1_CLK_SELR::ARM_PLL,
            1 => LVDS1_CLK_SELR::SYS_PLL,
            2 => LVDS1_CLK_SELR::PFD4,
            3 => LVDS1_CLK_SELR::PFD5,
            4 => LVDS1_CLK_SELR::PFD6,
            5 => LVDS1_CLK_SELR::PFD7,
            6 => LVDS1_CLK_SELR::AUDIO_PLL,
            7 => LVDS1_CLK_SELR::VIDEO_PLL,
            9 => LVDS1_CLK_SELR::ETHERNET_REF,
            12 => LVDS1_CLK_SELR::USB1_PLL,
            13 => LVDS1_CLK_SELR::USB2_PLL,
            14 => LVDS1_CLK_SELR::PFD0,
            15 => LVDS1_CLK_SELR::PFD1,
            16 => LVDS1_CLK_SELR::PFD2,
            17 => LVDS1_CLK_SELR::PFD3,
            18 => LVDS1_CLK_SELR::XTAL,
            i => LVDS1_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PLL`"]
    #[inline]
    pub fn is_arm_pll(&self) -> bool {
        *self == LVDS1_CLK_SELR::ARM_PLL
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline]
    pub fn is_sys_pll(&self) -> bool {
        *self == LVDS1_CLK_SELR::SYS_PLL
    }
    #[doc = "Checks if the value of the field is `PFD4`"]
    #[inline]
    pub fn is_pfd4(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD4
    }
    #[doc = "Checks if the value of the field is `PFD5`"]
    #[inline]
    pub fn is_pfd5(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD5
    }
    #[doc = "Checks if the value of the field is `PFD6`"]
    #[inline]
    pub fn is_pfd6(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD6
    }
    #[doc = "Checks if the value of the field is `PFD7`"]
    #[inline]
    pub fn is_pfd7(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD7
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL`"]
    #[inline]
    pub fn is_audio_pll(&self) -> bool {
        *self == LVDS1_CLK_SELR::AUDIO_PLL
    }
    #[doc = "Checks if the value of the field is `VIDEO_PLL`"]
    #[inline]
    pub fn is_video_pll(&self) -> bool {
        *self == LVDS1_CLK_SELR::VIDEO_PLL
    }
    #[doc = "Checks if the value of the field is `ETHERNET_REF`"]
    #[inline]
    pub fn is_ethernet_ref(&self) -> bool {
        *self == LVDS1_CLK_SELR::ETHERNET_REF
    }
    #[doc = "Checks if the value of the field is `USB1_PLL`"]
    #[inline]
    pub fn is_usb1_pll(&self) -> bool {
        *self == LVDS1_CLK_SELR::USB1_PLL
    }
    #[doc = "Checks if the value of the field is `USB2_PLL`"]
    #[inline]
    pub fn is_usb2_pll(&self) -> bool {
        *self == LVDS1_CLK_SELR::USB2_PLL
    }
    #[doc = "Checks if the value of the field is `PFD0`"]
    #[inline]
    pub fn is_pfd0(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD0
    }
    #[doc = "Checks if the value of the field is `PFD1`"]
    #[inline]
    pub fn is_pfd1(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD1
    }
    #[doc = "Checks if the value of the field is `PFD2`"]
    #[inline]
    pub fn is_pfd2(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD2
    }
    #[doc = "Checks if the value of the field is `PFD3`"]
    #[inline]
    pub fn is_pfd3(&self) -> bool {
        *self == LVDS1_CLK_SELR::PFD3
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == LVDS1_CLK_SELR::XTAL
    }
}
#[doc = r" Value of the field"]
pub struct LVDSCLK1_OBENR {
    bits: bool,
}
impl LVDSCLK1_OBENR {
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
pub struct LVDSCLK1_IBENR {
    bits: bool,
}
impl LVDSCLK1_IBENR {
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
pub struct PFD_480_AUTOGATE_ENR {
    bits: bool,
}
impl PFD_480_AUTOGATE_ENR {
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
pub struct PFD_528_AUTOGATE_ENR {
    bits: bool,
}
impl PFD_528_AUTOGATE_ENR {
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
pub struct IRQ_TEMPPANICR {
    bits: bool,
}
impl IRQ_TEMPPANICR {
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
pub struct IRQ_TEMPLOWR {
    bits: bool,
}
impl IRQ_TEMPLOWR {
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
pub struct IRQ_TEMPHIGHR {
    bits: bool,
}
impl IRQ_TEMPHIGHR {
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
pub struct IRQ_ANA_BOR {
    bits: bool,
}
impl IRQ_ANA_BOR {
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
pub struct IRQ_DIG_BOR {
    bits: bool,
}
impl IRQ_DIG_BOR {
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
#[doc = "Values that can be written to the field `LVDS1_CLK_SEL`"]
pub enum LVDS1_CLK_SELW {
    #[doc = "Arm PLL"]
    ARM_PLL,
    #[doc = "System PLL"]
    SYS_PLL,
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4,
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5,
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6,
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7,
    #[doc = "Audio PLL"]
    AUDIO_PLL,
    #[doc = "Video PLL"]
    VIDEO_PLL,
    #[doc = "ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF,
    #[doc = "USB1 PLL clock"]
    USB1_PLL,
    #[doc = "USB2 PLL clock"]
    USB2_PLL,
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0,
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1,
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2,
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3,
    #[doc = "xtal (24M)"]
    XTAL,
}
impl LVDS1_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LVDS1_CLK_SELW::ARM_PLL => 0,
            LVDS1_CLK_SELW::SYS_PLL => 1,
            LVDS1_CLK_SELW::PFD4 => 2,
            LVDS1_CLK_SELW::PFD5 => 3,
            LVDS1_CLK_SELW::PFD6 => 4,
            LVDS1_CLK_SELW::PFD7 => 5,
            LVDS1_CLK_SELW::AUDIO_PLL => 6,
            LVDS1_CLK_SELW::VIDEO_PLL => 7,
            LVDS1_CLK_SELW::ETHERNET_REF => 9,
            LVDS1_CLK_SELW::USB1_PLL => 12,
            LVDS1_CLK_SELW::USB2_PLL => 13,
            LVDS1_CLK_SELW::PFD0 => 14,
            LVDS1_CLK_SELW::PFD1 => 15,
            LVDS1_CLK_SELW::PFD2 => 16,
            LVDS1_CLK_SELW::PFD3 => 17,
            LVDS1_CLK_SELW::XTAL => 18,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVDS1_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LVDS1_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVDS1_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Arm PLL"]
    #[inline]
    pub fn arm_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::ARM_PLL)
    }
    #[doc = "System PLL"]
    #[inline]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::SYS_PLL)
    }
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    #[inline]
    pub fn pfd4(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD4)
    }
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    #[inline]
    pub fn pfd5(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD5)
    }
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    #[inline]
    pub fn pfd6(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD6)
    }
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    #[inline]
    pub fn pfd7(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD7)
    }
    #[doc = "Audio PLL"]
    #[inline]
    pub fn audio_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::AUDIO_PLL)
    }
    #[doc = "Video PLL"]
    #[inline]
    pub fn video_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::VIDEO_PLL)
    }
    #[doc = "ethernet ref clock (ENET_PLL)"]
    #[inline]
    pub fn ethernet_ref(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::ETHERNET_REF)
    }
    #[doc = "USB1 PLL clock"]
    #[inline]
    pub fn usb1_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::USB1_PLL)
    }
    #[doc = "USB2 PLL clock"]
    #[inline]
    pub fn usb2_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::USB2_PLL)
    }
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    #[inline]
    pub fn pfd0(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD0)
    }
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    #[inline]
    pub fn pfd1(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD1)
    }
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    #[inline]
    pub fn pfd2(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD2)
    }
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    #[inline]
    pub fn pfd3(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::PFD3)
    }
    #[doc = "xtal (24M)"]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SELW::XTAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LVDSCLK1_OBENW<'a> {
    w: &'a mut W,
}
impl<'a> _LVDSCLK1_OBENW<'a> {
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
pub struct _LVDSCLK1_IBENW<'a> {
    w: &'a mut W,
}
impl<'a> _LVDSCLK1_IBENW<'a> {
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
pub struct _PFD_480_AUTOGATE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PFD_480_AUTOGATE_ENW<'a> {
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
pub struct _PFD_528_AUTOGATE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PFD_528_AUTOGATE_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _IRQ_TEMPPANICW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_TEMPPANICW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_TEMPLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_TEMPLOWW<'a> {
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
#[doc = r" Proxy"]
pub struct _IRQ_TEMPHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_TEMPHIGHW<'a> {
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
#[doc = r" Proxy"]
pub struct _IRQ_ANA_BOW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_ANA_BOW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_DIG_BOW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_DIG_BOW<'a> {
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
    #[doc = "Bits 0:4 - This field selects the clk to be routed to anaclk1/1b."]
    #[inline]
    pub fn lvds1_clk_sel(&self) -> LVDS1_CLK_SELR {
        LVDS1_CLK_SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - This enables the LVDS output buffer for anaclk1/1b"]
    #[inline]
    pub fn lvdsclk1_oben(&self) -> LVDSCLK1_OBENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LVDSCLK1_OBENR { bits }
    }
    #[doc = "Bit 12 - This enables the LVDS input buffer for anaclk1/1b"]
    #[inline]
    pub fn lvdsclk1_iben(&self) -> LVDSCLK1_IBENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LVDSCLK1_IBENR { bits }
    }
    #[doc = "Bit 16 - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline]
    pub fn pfd_480_autogate_en(&self) -> PFD_480_AUTOGATE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFD_480_AUTOGATE_ENR { bits }
    }
    #[doc = "Bit 17 - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline]
    pub fn pfd_528_autogate_en(&self) -> PFD_528_AUTOGATE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFD_528_AUTOGATE_ENR { bits }
    }
    #[doc = "Bit 27 - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline]
    pub fn irq_temppanic(&self) -> IRQ_TEMPPANICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ_TEMPPANICR { bits }
    }
    #[doc = "Bit 28 - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline]
    pub fn irq_templow(&self) -> IRQ_TEMPLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ_TEMPLOWR { bits }
    }
    #[doc = "Bit 29 - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline]
    pub fn irq_temphigh(&self) -> IRQ_TEMPHIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ_TEMPHIGHR { bits }
    }
    #[doc = "Bit 30 - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline]
    pub fn irq_ana_bo(&self) -> IRQ_ANA_BOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ_ANA_BOR { bits }
    }
    #[doc = "Bit 31 - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline]
    pub fn irq_dig_bo(&self) -> IRQ_DIG_BOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ_DIG_BOR { bits }
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
    #[doc = "Bits 0:4 - This field selects the clk to be routed to anaclk1/1b."]
    #[inline]
    pub fn lvds1_clk_sel(&mut self) -> _LVDS1_CLK_SELW {
        _LVDS1_CLK_SELW { w: self }
    }
    #[doc = "Bit 10 - This enables the LVDS output buffer for anaclk1/1b"]
    #[inline]
    pub fn lvdsclk1_oben(&mut self) -> _LVDSCLK1_OBENW {
        _LVDSCLK1_OBENW { w: self }
    }
    #[doc = "Bit 12 - This enables the LVDS input buffer for anaclk1/1b"]
    #[inline]
    pub fn lvdsclk1_iben(&mut self) -> _LVDSCLK1_IBENW {
        _LVDSCLK1_IBENW { w: self }
    }
    #[doc = "Bit 16 - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline]
    pub fn pfd_480_autogate_en(&mut self) -> _PFD_480_AUTOGATE_ENW {
        _PFD_480_AUTOGATE_ENW { w: self }
    }
    #[doc = "Bit 17 - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline]
    pub fn pfd_528_autogate_en(&mut self) -> _PFD_528_AUTOGATE_ENW {
        _PFD_528_AUTOGATE_ENW { w: self }
    }
    #[doc = "Bit 27 - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline]
    pub fn irq_temppanic(&mut self) -> _IRQ_TEMPPANICW {
        _IRQ_TEMPPANICW { w: self }
    }
    #[doc = "Bit 28 - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline]
    pub fn irq_templow(&mut self) -> _IRQ_TEMPLOWW {
        _IRQ_TEMPLOWW { w: self }
    }
    #[doc = "Bit 29 - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline]
    pub fn irq_temphigh(&mut self) -> _IRQ_TEMPHIGHW {
        _IRQ_TEMPHIGHW { w: self }
    }
    #[doc = "Bit 30 - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline]
    pub fn irq_ana_bo(&mut self) -> _IRQ_ANA_BOW {
        _IRQ_ANA_BOW { w: self }
    }
    #[doc = "Bit 31 - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline]
    pub fn irq_dig_bo(&mut self) -> _IRQ_DIG_BOW {
        _IRQ_DIG_BOW { w: self }
    }
}
