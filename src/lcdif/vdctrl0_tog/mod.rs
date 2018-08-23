#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VDCTRL0_TOG {
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
pub struct VSYNC_PULSE_WIDTHR {
    bits: u32,
}
impl VSYNC_PULSE_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HALF_LINE_MODER {
    bits: bool,
}
impl HALF_LINE_MODER {
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
pub struct HALF_LINER {
    bits: bool,
}
impl HALF_LINER {
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
pub struct VSYNC_PULSE_WIDTH_UNITR {
    bits: bool,
}
impl VSYNC_PULSE_WIDTH_UNITR {
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
pub struct VSYNC_PERIOD_UNITR {
    bits: bool,
}
impl VSYNC_PERIOD_UNITR {
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
pub struct ENABLE_POLR {
    bits: bool,
}
impl ENABLE_POLR {
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
pub struct DOTCLK_POLR {
    bits: bool,
}
impl DOTCLK_POLR {
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
pub struct HSYNC_POLR {
    bits: bool,
}
impl HSYNC_POLR {
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
pub struct VSYNC_POLR {
    bits: bool,
}
impl VSYNC_POLR {
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
pub struct ENABLE_PRESENTR {
    bits: bool,
}
impl ENABLE_PRESENTR {
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
pub struct _VSYNC_PULSE_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_PULSE_WIDTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 262143;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HALF_LINE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _HALF_LINE_MODEW<'a> {
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
pub struct _HALF_LINEW<'a> {
    w: &'a mut W,
}
impl<'a> _HALF_LINEW<'a> {
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
pub struct _VSYNC_PULSE_WIDTH_UNITW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_PULSE_WIDTH_UNITW<'a> {
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
pub struct _VSYNC_PERIOD_UNITW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_PERIOD_UNITW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENABLE_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_POLW<'a> {
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
#[doc = r" Proxy"]
pub struct _DOTCLK_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _DOTCLK_POLW<'a> {
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
#[doc = r" Proxy"]
pub struct _HSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSYNC_POLW<'a> {
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
#[doc = r" Proxy"]
pub struct _VSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_POLW<'a> {
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
pub struct _ENABLE_PRESENTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_PRESENTW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:17 - Number of units for which VSYNC signal is active"]
    #[inline]
    pub fn vsync_pulse_width(&self) -> VSYNC_PULSE_WIDTHR {
        let bits = {
            const MASK: u32 = 262143;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        VSYNC_PULSE_WIDTHR { bits }
    }
    #[doc = "Bit 18 - When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline]
    pub fn half_line_mode(&self) -> HALF_LINE_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALF_LINE_MODER { bits }
    }
    #[doc = "Bit 19 - Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline]
    pub fn half_line(&self) -> HALF_LINER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALF_LINER { bits }
    }
    #[doc = "Bit 20 - Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline]
    pub fn vsync_pulse_width_unit(&self) -> VSYNC_PULSE_WIDTH_UNITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VSYNC_PULSE_WIDTH_UNITR { bits }
    }
    #[doc = "Bit 21 - Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline]
    pub fn vsync_period_unit(&self) -> VSYNC_PERIOD_UNITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VSYNC_PERIOD_UNITR { bits }
    }
    #[doc = "Bit 24 - Default 0 active low during valid data transfer on each horizontal line."]
    #[inline]
    pub fn enable_pol(&self) -> ENABLE_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_POLR { bits }
    }
    #[doc = "Bit 25 - Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline]
    pub fn dotclk_pol(&self) -> DOTCLK_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DOTCLK_POLR { bits }
    }
    #[doc = "Bit 26 - Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline]
    pub fn hsync_pol(&self) -> HSYNC_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSYNC_POLR { bits }
    }
    #[doc = "Bit 27 - Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline]
    pub fn vsync_pol(&self) -> VSYNC_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VSYNC_POLR { bits }
    }
    #[doc = "Bit 28 - Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline]
    pub fn enable_present(&self) -> ENABLE_PRESENTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_PRESENTR { bits }
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
    #[doc = "Bits 0:17 - Number of units for which VSYNC signal is active"]
    #[inline]
    pub fn vsync_pulse_width(&mut self) -> _VSYNC_PULSE_WIDTHW {
        _VSYNC_PULSE_WIDTHW { w: self }
    }
    #[doc = "Bit 18 - When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline]
    pub fn half_line_mode(&mut self) -> _HALF_LINE_MODEW {
        _HALF_LINE_MODEW { w: self }
    }
    #[doc = "Bit 19 - Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline]
    pub fn half_line(&mut self) -> _HALF_LINEW {
        _HALF_LINEW { w: self }
    }
    #[doc = "Bit 20 - Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline]
    pub fn vsync_pulse_width_unit(&mut self) -> _VSYNC_PULSE_WIDTH_UNITW {
        _VSYNC_PULSE_WIDTH_UNITW { w: self }
    }
    #[doc = "Bit 21 - Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline]
    pub fn vsync_period_unit(&mut self) -> _VSYNC_PERIOD_UNITW {
        _VSYNC_PERIOD_UNITW { w: self }
    }
    #[doc = "Bit 24 - Default 0 active low during valid data transfer on each horizontal line."]
    #[inline]
    pub fn enable_pol(&mut self) -> _ENABLE_POLW {
        _ENABLE_POLW { w: self }
    }
    #[doc = "Bit 25 - Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline]
    pub fn dotclk_pol(&mut self) -> _DOTCLK_POLW {
        _DOTCLK_POLW { w: self }
    }
    #[doc = "Bit 26 - Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline]
    pub fn hsync_pol(&mut self) -> _HSYNC_POLW {
        _HSYNC_POLW { w: self }
    }
    #[doc = "Bit 27 - Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline]
    pub fn vsync_pol(&mut self) -> _VSYNC_POLW {
        _VSYNC_POLW { w: self }
    }
    #[doc = "Bit 28 - Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline]
    pub fn enable_present(&mut self) -> _ENABLE_PRESENTW {
        _ENABLE_PRESENTW { w: self }
    }
}
