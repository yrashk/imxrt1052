#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL_USB1_CLR {
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
#[doc = r" Value of the field"]
pub struct DIV_SELECTR {
    bits: u8,
}
impl DIV_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EN_USB_CLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_USB_CLKSR {
    #[doc = "PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0,
    #[doc = "PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1,
}
impl EN_USB_CLKSR {
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
            EN_USB_CLKSR::EN_USB_CLKS_0 => false,
            EN_USB_CLKSR::EN_USB_CLKS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_USB_CLKSR {
        match value {
            false => EN_USB_CLKSR::EN_USB_CLKS_0,
            true => EN_USB_CLKSR::EN_USB_CLKS_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_USB_CLKS_0`"]
    #[inline]
    pub fn is_en_usb_clks_0(&self) -> bool {
        *self == EN_USB_CLKSR::EN_USB_CLKS_0
    }
    #[doc = "Checks if the value of the field is `EN_USB_CLKS_1`"]
    #[inline]
    pub fn is_en_usb_clks_1(&self) -> bool {
        *self == EN_USB_CLKSR::EN_USB_CLKS_1
    }
}
#[doc = r" Value of the field"]
pub struct POWERR {
    bits: bool,
}
impl POWERR {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
    #[doc = "GPANAIO"]
    GPANAIO,
    #[doc = "CHRG_DET_B"]
    CHRG_DET_B,
}
impl BYPASS_CLK_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYPASS_CLK_SRCR::REF_CLK_24M => 0,
            BYPASS_CLK_SRCR::CLK1 => 1,
            BYPASS_CLK_SRCR::GPANAIO => 2,
            BYPASS_CLK_SRCR::CHRG_DET_B => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYPASS_CLK_SRCR {
        match value {
            0 => BYPASS_CLK_SRCR::REF_CLK_24M,
            1 => BYPASS_CLK_SRCR::CLK1,
            2 => BYPASS_CLK_SRCR::GPANAIO,
            3 => BYPASS_CLK_SRCR::CHRG_DET_B,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `GPANAIO`"]
    #[inline]
    pub fn is_gpanaio(&self) -> bool {
        *self == BYPASS_CLK_SRCR::GPANAIO
    }
    #[doc = "Checks if the value of the field is `CHRG_DET_B`"]
    #[inline]
    pub fn is_chrg_det_b(&self) -> bool {
        *self == BYPASS_CLK_SRCR::CHRG_DET_B
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
#[doc = r" Proxy"]
pub struct _DIV_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIV_SELECTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_USB_CLKS`"]
pub enum EN_USB_CLKSW {
    #[doc = "PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0,
    #[doc = "PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1,
}
impl EN_USB_CLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_USB_CLKSW::EN_USB_CLKS_0 => false,
            EN_USB_CLKSW::EN_USB_CLKS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_USB_CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_USB_CLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_USB_CLKSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL outputs for USBPHYn off."]
    #[inline]
    pub fn en_usb_clks_0(self) -> &'a mut W {
        self.variant(EN_USB_CLKSW::EN_USB_CLKS_0)
    }
    #[doc = "PLL outputs for USBPHYn on."]
    #[inline]
    pub fn en_usb_clks_1(self) -> &'a mut W {
        self.variant(EN_USB_CLKSW::EN_USB_CLKS_1)
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
#[doc = r" Proxy"]
pub struct _POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _POWERW<'a> {
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
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
    #[doc = "GPANAIO"]
    GPANAIO,
    #[doc = "CHRG_DET_B"]
    CHRG_DET_B,
}
impl BYPASS_CLK_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BYPASS_CLK_SRCW::REF_CLK_24M => 0,
            BYPASS_CLK_SRCW::CLK1 => 1,
            BYPASS_CLK_SRCW::GPANAIO => 2,
            BYPASS_CLK_SRCW::CHRG_DET_B => 3,
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
        {
            self.bits(variant._bits())
        }
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
    #[doc = "GPANAIO"]
    #[inline]
    pub fn gpanaio(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRCW::GPANAIO)
    }
    #[doc = "CHRG_DET_B"]
    #[inline]
    pub fn chrg_det_b(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRCW::CHRG_DET_B)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline]
    pub fn div_select(&self) -> DIV_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIV_SELECTR { bits }
    }
    #[doc = "Bit 6 - Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline]
    pub fn en_usb_clks(&self) -> EN_USB_CLKSR {
        EN_USB_CLKSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline]
    pub fn power(&self) -> POWERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POWERR { bits }
    }
    #[doc = "Bit 13 - Enable the PLL clock output."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
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
    #[doc = "Bit 31 - 1 - PLL is currently locked. 0 - PLL is not currently locked."]
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
        W { bits: 73728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline]
    pub fn div_select(&mut self) -> _DIV_SELECTW {
        _DIV_SELECTW { w: self }
    }
    #[doc = "Bit 6 - Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline]
    pub fn en_usb_clks(&mut self) -> _EN_USB_CLKSW {
        _EN_USB_CLKSW { w: self }
    }
    #[doc = "Bit 12 - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline]
    pub fn power(&mut self) -> _POWERW {
        _POWERW { w: self }
    }
    #[doc = "Bit 13 - Enable the PLL clock output."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
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
}
