#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = r" Value of the field"]
pub struct IRQ_ENABLER {
    bits: bool,
}
impl IRQ_ENABLER {
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
pub struct NEXT_IRQ_ENABLER {
    bits: bool,
}
impl NEXT_IRQ_ENABLER {
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
pub struct ENABLE_LCD_HANDSHAKER {
    bits: bool,
}
impl ENABLE_LCD_HANDSHAKER {
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
pub struct RSVD0R {
    bits: u8,
}
impl RSVD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ROTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROTATER {
    #[doc = "ROT_0"]
    ROT_0,
    #[doc = "ROT_90"]
    ROT_90,
    #[doc = "ROT_180"]
    ROT_180,
    #[doc = "ROT_270"]
    ROT_270,
}
impl ROTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ROTATER::ROT_0 => 0,
            ROTATER::ROT_90 => 1,
            ROTATER::ROT_180 => 2,
            ROTATER::ROT_270 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ROTATER {
        match value {
            0 => ROTATER::ROT_0,
            1 => ROTATER::ROT_90,
            2 => ROTATER::ROT_180,
            3 => ROTATER::ROT_270,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ROT_0`"]
    #[inline]
    pub fn is_rot_0(&self) -> bool {
        *self == ROTATER::ROT_0
    }
    #[doc = "Checks if the value of the field is `ROT_90`"]
    #[inline]
    pub fn is_rot_90(&self) -> bool {
        *self == ROTATER::ROT_90
    }
    #[doc = "Checks if the value of the field is `ROT_180`"]
    #[inline]
    pub fn is_rot_180(&self) -> bool {
        *self == ROTATER::ROT_180
    }
    #[doc = "Checks if the value of the field is `ROT_270`"]
    #[inline]
    pub fn is_rot_270(&self) -> bool {
        *self == ROTATER::ROT_270
    }
}
#[doc = r" Value of the field"]
pub struct HFLIPR {
    bits: bool,
}
impl HFLIPR {
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
pub struct VFLIPR {
    bits: bool,
}
impl VFLIPR {
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
pub struct RSVD1R {
    bits: u16,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROT_POSR {
    bits: bool,
}
impl ROT_POSR {
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
#[doc = "Possible values of the field `BLOCK_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_SIZER {
    #[doc = "Process 8x8 pixel blocks."]
    _8X8,
    #[doc = "Process 16x16 pixel blocks."]
    _16X16,
}
impl BLOCK_SIZER {
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
            BLOCK_SIZER::_8X8 => false,
            BLOCK_SIZER::_16X16 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLOCK_SIZER {
        match value {
            false => BLOCK_SIZER::_8X8,
            true => BLOCK_SIZER::_16X16,
        }
    }
    #[doc = "Checks if the value of the field is `_8X8`"]
    #[inline]
    pub fn is_8x8(&self) -> bool {
        *self == BLOCK_SIZER::_8X8
    }
    #[doc = "Checks if the value of the field is `_16X16`"]
    #[inline]
    pub fn is_16x16(&self) -> bool {
        *self == BLOCK_SIZER::_16X16
    }
}
#[doc = r" Value of the field"]
pub struct RSVD3R {
    bits: u8,
}
impl RSVD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EN_REPEATR {
    bits: bool,
}
impl EN_REPEATR {
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
pub struct RSVD4R {
    bits: bool,
}
impl RSVD4R {
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
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
pub struct SFTRSTR {
    bits: bool,
}
impl SFTRSTR {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_ENABLEW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NEXT_IRQ_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _NEXT_IRQ_ENABLEW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_LCD_HANDSHAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_LCD_HANDSHAKEW<'a> {
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
#[doc = "Values that can be written to the field `ROTATE`"]
pub enum ROTATEW {
    #[doc = "ROT_0"]
    ROT_0,
    #[doc = "ROT_90"]
    ROT_90,
    #[doc = "ROT_180"]
    ROT_180,
    #[doc = "ROT_270"]
    ROT_270,
}
impl ROTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ROTATEW::ROT_0 => 0,
            ROTATEW::ROT_90 => 1,
            ROTATEW::ROT_180 => 2,
            ROTATEW::ROT_270 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _ROTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROTATEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ROT_0"]
    #[inline]
    pub fn rot_0(self) -> &'a mut W {
        self.variant(ROTATEW::ROT_0)
    }
    #[doc = "ROT_90"]
    #[inline]
    pub fn rot_90(self) -> &'a mut W {
        self.variant(ROTATEW::ROT_90)
    }
    #[doc = "ROT_180"]
    #[inline]
    pub fn rot_180(self) -> &'a mut W {
        self.variant(ROTATEW::ROT_180)
    }
    #[doc = "ROT_270"]
    #[inline]
    pub fn rot_270(self) -> &'a mut W {
        self.variant(ROTATEW::ROT_270)
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
#[doc = r" Proxy"]
pub struct _HFLIPW<'a> {
    w: &'a mut W,
}
impl<'a> _HFLIPW<'a> {
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
pub struct _VFLIPW<'a> {
    w: &'a mut W,
}
impl<'a> _VFLIPW<'a> {
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
pub struct _ROT_POSW<'a> {
    w: &'a mut W,
}
impl<'a> _ROT_POSW<'a> {
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
#[doc = "Values that can be written to the field `BLOCK_SIZE`"]
pub enum BLOCK_SIZEW {
    #[doc = "Process 8x8 pixel blocks."]
    _8X8,
    #[doc = "Process 16x16 pixel blocks."]
    _16X16,
}
impl BLOCK_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLOCK_SIZEW::_8X8 => false,
            BLOCK_SIZEW::_16X16 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLOCK_SIZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Process 8x8 pixel blocks."]
    #[inline]
    pub fn _8x8(self) -> &'a mut W {
        self.variant(BLOCK_SIZEW::_8X8)
    }
    #[doc = "Process 16x16 pixel blocks."]
    #[inline]
    pub fn _16x16(self) -> &'a mut W {
        self.variant(BLOCK_SIZEW::_16X16)
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
#[doc = r" Proxy"]
pub struct _EN_REPEATW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_REPEATW<'a> {
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
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
pub struct _SFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTW<'a> {
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
    #[doc = "Bit 0 - Enables PXP operation with specified parameters"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline]
    pub fn irq_enable(&self) -> IRQ_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ_ENABLER { bits }
    }
    #[doc = "Bit 2 - Next command interrupt enable"]
    #[inline]
    pub fn next_irq_enable(&self) -> NEXT_IRQ_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NEXT_IRQ_ENABLER { bits }
    }
    #[doc = "Bit 4 - Enable handshake with LCD controller"]
    #[inline]
    pub fn enable_lcd_handshake(&self) -> ENABLE_LCD_HANDSHAKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_LCD_HANDSHAKER { bits }
    }
    #[doc = "Bits 5:7 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD0R { bits }
    }
    #[doc = "Bits 8:9 - Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline]
    pub fn rotate(&self) -> ROTATER {
        ROTATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline]
    pub fn hflip(&self) -> HFLIPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HFLIPR { bits }
    }
    #[doc = "Bit 11 - Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline]
    pub fn vflip(&self) -> VFLIPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VFLIPR { bits }
    }
    #[doc = "Bits 12:21 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RSVD1R { bits }
    }
    #[doc = "Bit 22 - This bit controls where rotation will occur in the PXP datapath"]
    #[inline]
    pub fn rot_pos(&self) -> ROT_POSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ROT_POSR { bits }
    }
    #[doc = "Bit 23 - Select the block size to process."]
    #[inline]
    pub fn block_size(&self) -> BLOCK_SIZER {
        BLOCK_SIZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd3(&self) -> RSVD3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD3R { bits }
    }
    #[doc = "Bit 28 - Enable the PXP to run continuously"]
    #[inline]
    pub fn en_repeat(&self) -> EN_REPEATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_REPEATR { bits }
    }
    #[doc = "Bit 29 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd4(&self) -> RSVD4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD4R { bits }
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable normal PXP operation"]
    #[inline]
    pub fn sftrst(&self) -> SFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables PXP operation with specified parameters"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline]
    pub fn irq_enable(&mut self) -> _IRQ_ENABLEW {
        _IRQ_ENABLEW { w: self }
    }
    #[doc = "Bit 2 - Next command interrupt enable"]
    #[inline]
    pub fn next_irq_enable(&mut self) -> _NEXT_IRQ_ENABLEW {
        _NEXT_IRQ_ENABLEW { w: self }
    }
    #[doc = "Bit 4 - Enable handshake with LCD controller"]
    #[inline]
    pub fn enable_lcd_handshake(&mut self) -> _ENABLE_LCD_HANDSHAKEW {
        _ENABLE_LCD_HANDSHAKEW { w: self }
    }
    #[doc = "Bits 8:9 - Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline]
    pub fn rotate(&mut self) -> _ROTATEW {
        _ROTATEW { w: self }
    }
    #[doc = "Bit 10 - Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline]
    pub fn hflip(&mut self) -> _HFLIPW {
        _HFLIPW { w: self }
    }
    #[doc = "Bit 11 - Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline]
    pub fn vflip(&mut self) -> _VFLIPW {
        _VFLIPW { w: self }
    }
    #[doc = "Bit 22 - This bit controls where rotation will occur in the PXP datapath"]
    #[inline]
    pub fn rot_pos(&mut self) -> _ROT_POSW {
        _ROT_POSW { w: self }
    }
    #[doc = "Bit 23 - Select the block size to process."]
    #[inline]
    pub fn block_size(&mut self) -> _BLOCK_SIZEW {
        _BLOCK_SIZEW { w: self }
    }
    #[doc = "Bit 28 - Enable the PXP to run continuously"]
    #[inline]
    pub fn en_repeat(&mut self) -> _EN_REPEATW {
        _EN_REPEATW { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable normal PXP operation"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
