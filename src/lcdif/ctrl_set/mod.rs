#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL_SET {
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
pub struct RUNR {
    bits: bool,
}
impl RUNR {
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
#[doc = "Possible values of the field `DATA_FORMAT_24_BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_FORMAT_24_BITR {
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    ALL_24_BITS_VALID,
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    DROP_UPPER_2_BITS_PER_BYTE,
}
impl DATA_FORMAT_24_BITR {
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
            DATA_FORMAT_24_BITR::ALL_24_BITS_VALID => false,
            DATA_FORMAT_24_BITR::DROP_UPPER_2_BITS_PER_BYTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_FORMAT_24_BITR {
        match value {
            false => DATA_FORMAT_24_BITR::ALL_24_BITS_VALID,
            true => DATA_FORMAT_24_BITR::DROP_UPPER_2_BITS_PER_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_24_BITS_VALID`"]
    #[inline]
    pub fn is_all_24_bits_valid(&self) -> bool {
        *self == DATA_FORMAT_24_BITR::ALL_24_BITS_VALID
    }
    #[doc = "Checks if the value of the field is `DROP_UPPER_2_BITS_PER_BYTE`"]
    #[inline]
    pub fn is_drop_upper_2_bits_per_byte(&self) -> bool {
        *self == DATA_FORMAT_24_BITR::DROP_UPPER_2_BITS_PER_BYTE
    }
}
#[doc = "Possible values of the field `DATA_FORMAT_18_BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_FORMAT_18_BITR {
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    LOWER_18_BITS_VALID,
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    UPPER_18_BITS_VALID,
}
impl DATA_FORMAT_18_BITR {
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
            DATA_FORMAT_18_BITR::LOWER_18_BITS_VALID => false,
            DATA_FORMAT_18_BITR::UPPER_18_BITS_VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_FORMAT_18_BITR {
        match value {
            false => DATA_FORMAT_18_BITR::LOWER_18_BITS_VALID,
            true => DATA_FORMAT_18_BITR::UPPER_18_BITS_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `LOWER_18_BITS_VALID`"]
    #[inline]
    pub fn is_lower_18_bits_valid(&self) -> bool {
        *self == DATA_FORMAT_18_BITR::LOWER_18_BITS_VALID
    }
    #[doc = "Checks if the value of the field is `UPPER_18_BITS_VALID`"]
    #[inline]
    pub fn is_upper_18_bits_valid(&self) -> bool {
        *self == DATA_FORMAT_18_BITR::UPPER_18_BITS_VALID
    }
}
#[doc = r" Value of the field"]
pub struct DATA_FORMAT_16_BITR {
    bits: bool,
}
impl DATA_FORMAT_16_BITR {
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
pub struct MASTERR {
    bits: bool,
}
impl MASTERR {
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
pub struct ENABLE_PXP_HANDSHAKER {
    bits: bool,
}
impl ENABLE_PXP_HANDSHAKER {
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
#[doc = "Possible values of the field `WORD_LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORD_LENGTHR {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT,
}
impl WORD_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WORD_LENGTHR::_16_BIT => 0,
            WORD_LENGTHR::_8_BIT => 1,
            WORD_LENGTHR::_18_BIT => 2,
            WORD_LENGTHR::_24_BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WORD_LENGTHR {
        match value {
            0 => WORD_LENGTHR::_16_BIT,
            1 => WORD_LENGTHR::_8_BIT,
            2 => WORD_LENGTHR::_18_BIT,
            3 => WORD_LENGTHR::_24_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == WORD_LENGTHR::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == WORD_LENGTHR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_18_BIT`"]
    #[inline]
    pub fn is_18_bit(&self) -> bool {
        *self == WORD_LENGTHR::_18_BIT
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline]
    pub fn is_24_bit(&self) -> bool {
        *self == WORD_LENGTHR::_24_BIT
    }
}
#[doc = "Possible values of the field `LCD_DATABUS_WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCD_DATABUS_WIDTHR {
    #[doc = "16-bit data bus mode."]
    _16_BIT,
    #[doc = "8-bit data bus mode."]
    _8_BIT,
    #[doc = "18-bit data bus mode."]
    _18_BIT,
    #[doc = "24-bit data bus mode."]
    _24_BIT,
}
impl LCD_DATABUS_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCD_DATABUS_WIDTHR::_16_BIT => 0,
            LCD_DATABUS_WIDTHR::_8_BIT => 1,
            LCD_DATABUS_WIDTHR::_18_BIT => 2,
            LCD_DATABUS_WIDTHR::_24_BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCD_DATABUS_WIDTHR {
        match value {
            0 => LCD_DATABUS_WIDTHR::_16_BIT,
            1 => LCD_DATABUS_WIDTHR::_8_BIT,
            2 => LCD_DATABUS_WIDTHR::_18_BIT,
            3 => LCD_DATABUS_WIDTHR::_24_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTHR::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTHR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_18_BIT`"]
    #[inline]
    pub fn is_18_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTHR::_18_BIT
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline]
    pub fn is_24_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTHR::_24_BIT
    }
}
#[doc = "Possible values of the field `CSC_DATA_SWIZZLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSC_DATA_SWIZZLER {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP,
    #[doc = "Swap half-words."]
    HWD_SWAP,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP,
}
impl CSC_DATA_SWIZZLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSC_DATA_SWIZZLER::NO_SWAP => 0,
            CSC_DATA_SWIZZLER::BIG_ENDIAN_SWAP => 1,
            CSC_DATA_SWIZZLER::HWD_SWAP => 2,
            CSC_DATA_SWIZZLER::HWD_BYTE_SWAP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSC_DATA_SWIZZLER {
        match value {
            0 => CSC_DATA_SWIZZLER::NO_SWAP,
            1 => CSC_DATA_SWIZZLER::BIG_ENDIAN_SWAP,
            2 => CSC_DATA_SWIZZLER::HWD_SWAP,
            3 => CSC_DATA_SWIZZLER::HWD_BYTE_SWAP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SWAP`"]
    #[inline]
    pub fn is_no_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLER::NO_SWAP
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_SWAP`"]
    #[inline]
    pub fn is_big_endian_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLER::BIG_ENDIAN_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_SWAP`"]
    #[inline]
    pub fn is_hwd_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLER::HWD_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_BYTE_SWAP`"]
    #[inline]
    pub fn is_hwd_byte_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLER::HWD_BYTE_SWAP
    }
}
#[doc = "Possible values of the field `INPUT_DATA_SWIZZLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT_DATA_SWIZZLER {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP,
    #[doc = "Swap half-words."]
    HWD_SWAP,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP,
}
impl INPUT_DATA_SWIZZLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT_DATA_SWIZZLER::NO_SWAP => 0,
            INPUT_DATA_SWIZZLER::BIG_ENDIAN_SWAP => 1,
            INPUT_DATA_SWIZZLER::HWD_SWAP => 2,
            INPUT_DATA_SWIZZLER::HWD_BYTE_SWAP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT_DATA_SWIZZLER {
        match value {
            0 => INPUT_DATA_SWIZZLER::NO_SWAP,
            1 => INPUT_DATA_SWIZZLER::BIG_ENDIAN_SWAP,
            2 => INPUT_DATA_SWIZZLER::HWD_SWAP,
            3 => INPUT_DATA_SWIZZLER::HWD_BYTE_SWAP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SWAP`"]
    #[inline]
    pub fn is_no_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLER::NO_SWAP
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_SWAP`"]
    #[inline]
    pub fn is_big_endian_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLER::BIG_ENDIAN_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_SWAP`"]
    #[inline]
    pub fn is_hwd_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLER::HWD_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_BYTE_SWAP`"]
    #[inline]
    pub fn is_hwd_byte_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLER::HWD_BYTE_SWAP
    }
}
#[doc = r" Value of the field"]
pub struct DOTCLK_MODER {
    bits: bool,
}
impl DOTCLK_MODER {
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
pub struct BYPASS_COUNTR {
    bits: bool,
}
impl BYPASS_COUNTR {
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
pub struct SHIFT_NUM_BITSR {
    bits: u8,
}
impl SHIFT_NUM_BITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DATA_SHIFT_DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_SHIFT_DIRR {
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_LEFT,
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_RIGHT,
}
impl DATA_SHIFT_DIRR {
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
            DATA_SHIFT_DIRR::TXDATA_SHIFT_LEFT => false,
            DATA_SHIFT_DIRR::TXDATA_SHIFT_RIGHT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_SHIFT_DIRR {
        match value {
            false => DATA_SHIFT_DIRR::TXDATA_SHIFT_LEFT,
            true => DATA_SHIFT_DIRR::TXDATA_SHIFT_RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `TXDATA_SHIFT_LEFT`"]
    #[inline]
    pub fn is_txdata_shift_left(&self) -> bool {
        *self == DATA_SHIFT_DIRR::TXDATA_SHIFT_LEFT
    }
    #[doc = "Checks if the value of the field is `TXDATA_SHIFT_RIGHT`"]
    #[inline]
    pub fn is_txdata_shift_right(&self) -> bool {
        *self == DATA_SHIFT_DIRR::TXDATA_SHIFT_RIGHT
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
pub struct _RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNW<'a> {
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
#[doc = "Values that can be written to the field `DATA_FORMAT_24_BIT`"]
pub enum DATA_FORMAT_24_BITW {
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    ALL_24_BITS_VALID,
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    DROP_UPPER_2_BITS_PER_BYTE,
}
impl DATA_FORMAT_24_BITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_FORMAT_24_BITW::ALL_24_BITS_VALID => false,
            DATA_FORMAT_24_BITW::DROP_UPPER_2_BITS_PER_BYTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_FORMAT_24_BITW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_FORMAT_24_BITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_FORMAT_24_BITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    #[inline]
    pub fn all_24_bits_valid(self) -> &'a mut W {
        self.variant(DATA_FORMAT_24_BITW::ALL_24_BITS_VALID)
    }
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    #[inline]
    pub fn drop_upper_2_bits_per_byte(self) -> &'a mut W {
        self.variant(DATA_FORMAT_24_BITW::DROP_UPPER_2_BITS_PER_BYTE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_FORMAT_18_BIT`"]
pub enum DATA_FORMAT_18_BITW {
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    LOWER_18_BITS_VALID,
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    UPPER_18_BITS_VALID,
}
impl DATA_FORMAT_18_BITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_FORMAT_18_BITW::LOWER_18_BITS_VALID => false,
            DATA_FORMAT_18_BITW::UPPER_18_BITS_VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_FORMAT_18_BITW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_FORMAT_18_BITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_FORMAT_18_BITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    #[inline]
    pub fn lower_18_bits_valid(self) -> &'a mut W {
        self.variant(DATA_FORMAT_18_BITW::LOWER_18_BITS_VALID)
    }
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    #[inline]
    pub fn upper_18_bits_valid(self) -> &'a mut W {
        self.variant(DATA_FORMAT_18_BITW::UPPER_18_BITS_VALID)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_FORMAT_16_BITW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_FORMAT_16_BITW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MASTERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTERW<'a> {
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
pub struct _ENABLE_PXP_HANDSHAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_PXP_HANDSHAKEW<'a> {
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
#[doc = "Values that can be written to the field `WORD_LENGTH`"]
pub enum WORD_LENGTHW {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT,
}
impl WORD_LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WORD_LENGTHW::_16_BIT => 0,
            WORD_LENGTHW::_8_BIT => 1,
            WORD_LENGTHW::_18_BIT => 2,
            WORD_LENGTHW::_24_BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WORD_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WORD_LENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WORD_LENGTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input data is 16 bits per pixel."]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTHW::_16_BIT)
    }
    #[doc = "Input data is 8 bits wide."]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTHW::_8_BIT)
    }
    #[doc = "Input data is 18 bits per pixel."]
    #[inline]
    pub fn _18_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTHW::_18_BIT)
    }
    #[doc = "Input data is 24 bits per pixel."]
    #[inline]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTHW::_24_BIT)
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
#[doc = "Values that can be written to the field `LCD_DATABUS_WIDTH`"]
pub enum LCD_DATABUS_WIDTHW {
    #[doc = "16-bit data bus mode."]
    _16_BIT,
    #[doc = "8-bit data bus mode."]
    _8_BIT,
    #[doc = "18-bit data bus mode."]
    _18_BIT,
    #[doc = "24-bit data bus mode."]
    _24_BIT,
}
impl LCD_DATABUS_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCD_DATABUS_WIDTHW::_16_BIT => 0,
            LCD_DATABUS_WIDTHW::_8_BIT => 1,
            LCD_DATABUS_WIDTHW::_18_BIT => 2,
            LCD_DATABUS_WIDTHW::_24_BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCD_DATABUS_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LCD_DATABUS_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCD_DATABUS_WIDTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "16-bit data bus mode."]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTHW::_16_BIT)
    }
    #[doc = "8-bit data bus mode."]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTHW::_8_BIT)
    }
    #[doc = "18-bit data bus mode."]
    #[inline]
    pub fn _18_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTHW::_18_BIT)
    }
    #[doc = "24-bit data bus mode."]
    #[inline]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTHW::_24_BIT)
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
#[doc = "Values that can be written to the field `CSC_DATA_SWIZZLE`"]
pub enum CSC_DATA_SWIZZLEW {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP,
    #[doc = "Swap half-words."]
    HWD_SWAP,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP,
}
impl CSC_DATA_SWIZZLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSC_DATA_SWIZZLEW::NO_SWAP => 0,
            CSC_DATA_SWIZZLEW::BIG_ENDIAN_SWAP => 1,
            CSC_DATA_SWIZZLEW::HWD_SWAP => 2,
            CSC_DATA_SWIZZLEW::HWD_BYTE_SWAP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSC_DATA_SWIZZLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSC_DATA_SWIZZLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSC_DATA_SWIZZLEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No byte swapping.(Little endian)"]
    #[inline]
    pub fn no_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLEW::NO_SWAP)
    }
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    #[inline]
    pub fn big_endian_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLEW::BIG_ENDIAN_SWAP)
    }
    #[doc = "Swap half-words."]
    #[inline]
    pub fn hwd_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLEW::HWD_SWAP)
    }
    #[doc = "Swap bytes within each half-word."]
    #[inline]
    pub fn hwd_byte_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLEW::HWD_BYTE_SWAP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT_DATA_SWIZZLE`"]
pub enum INPUT_DATA_SWIZZLEW {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP,
    #[doc = "Swap half-words."]
    HWD_SWAP,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP,
}
impl INPUT_DATA_SWIZZLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT_DATA_SWIZZLEW::NO_SWAP => 0,
            INPUT_DATA_SWIZZLEW::BIG_ENDIAN_SWAP => 1,
            INPUT_DATA_SWIZZLEW::HWD_SWAP => 2,
            INPUT_DATA_SWIZZLEW::HWD_BYTE_SWAP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT_DATA_SWIZZLEW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT_DATA_SWIZZLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT_DATA_SWIZZLEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No byte swapping.(Little endian)"]
    #[inline]
    pub fn no_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLEW::NO_SWAP)
    }
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    #[inline]
    pub fn big_endian_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLEW::BIG_ENDIAN_SWAP)
    }
    #[doc = "Swap half-words."]
    #[inline]
    pub fn hwd_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLEW::HWD_SWAP)
    }
    #[doc = "Swap bytes within each half-word."]
    #[inline]
    pub fn hwd_byte_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLEW::HWD_BYTE_SWAP)
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
pub struct _DOTCLK_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DOTCLK_MODEW<'a> {
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
pub struct _BYPASS_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_COUNTW<'a> {
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
pub struct _SHIFT_NUM_BITSW<'a> {
    w: &'a mut W,
}
impl<'a> _SHIFT_NUM_BITSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_SHIFT_DIR`"]
pub enum DATA_SHIFT_DIRW {
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_LEFT,
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_RIGHT,
}
impl DATA_SHIFT_DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_SHIFT_DIRW::TXDATA_SHIFT_LEFT => false,
            DATA_SHIFT_DIRW::TXDATA_SHIFT_RIGHT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_SHIFT_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_SHIFT_DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_SHIFT_DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    #[inline]
    pub fn txdata_shift_left(self) -> &'a mut W {
        self.variant(DATA_SHIFT_DIRW::TXDATA_SHIFT_LEFT)
    }
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    #[inline]
    pub fn txdata_shift_right(self) -> &'a mut W {
        self.variant(DATA_SHIFT_DIRW::TXDATA_SHIFT_RIGHT)
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
    #[doc = "Bit 0 - When this bit is set by software, the eLCDIF will begin transferring data between the SoC and the display"]
    #[inline]
    pub fn run(&self) -> RUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNR { bits }
    }
    #[doc = "Bit 1 - Used only when WORD_LENGTH = 3, i"]
    #[inline]
    pub fn data_format_24_bit(&self) -> DATA_FORMAT_24_BITR {
        DATA_FORMAT_24_BITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline]
    pub fn data_format_18_bit(&self) -> DATA_FORMAT_18_BITR {
        DATA_FORMAT_18_BITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline]
    pub fn data_format_16_bit(&self) -> DATA_FORMAT_16_BITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_FORMAT_16_BITR { bits }
    }
    #[doc = "Bit 5 - Set this bit to make the eLCDIF act as a bus master"]
    #[inline]
    pub fn master(&self) -> MASTERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MASTERR { bits }
    }
    #[doc = "Bit 6 - If this bit is set and LCDIF_MASTER bit is set, the eLCDIF will act as bus master and the handshake mechanism between eLCDIF and PXP will be turned on"]
    #[inline]
    pub fn enable_pxp_handshake(&self) -> ENABLE_PXP_HANDSHAKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_PXP_HANDSHAKER { bits }
    }
    #[doc = "Bits 8:9 - Input data format."]
    #[inline]
    pub fn word_length(&self) -> WORD_LENGTHR {
        WORD_LENGTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - LCD Data bus transfer width."]
    #[inline]
    pub fn lcd_databus_width(&self) -> LCD_DATABUS_WIDTHR {
        LCD_DATABUS_WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline]
    pub fn csc_data_swizzle(&self) -> CSC_DATA_SWIZZLER {
        CSC_DATA_SWIZZLER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline]
    pub fn input_data_swizzle(&self) -> INPUT_DATA_SWIZZLER {
        INPUT_DATA_SWIZZLER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline]
    pub fn dotclk_mode(&self) -> DOTCLK_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DOTCLK_MODER { bits }
    }
    #[doc = "Bit 19 - When this bit is 0, it means that eLCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline]
    pub fn bypass_count(&self) -> BYPASS_COUNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASS_COUNTR { bits }
    }
    #[doc = "Bits 21:25 - The data to be transmitted is shifted left or right by this number of bits."]
    #[inline]
    pub fn shift_num_bits(&self) -> SHIFT_NUM_BITSR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SHIFT_NUM_BITSR { bits }
    }
    #[doc = "Bit 26 - Use this bit to determine the direction of shift of transmit data."]
    #[inline]
    pub fn data_shift_dir(&self) -> DATA_SHIFT_DIRR {
        DATA_SHIFT_DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 31 - This bit must be set to zero to enable normal operation of the eLCDIF"]
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
    #[doc = "Bit 0 - When this bit is set by software, the eLCDIF will begin transferring data between the SoC and the display"]
    #[inline]
    pub fn run(&mut self) -> _RUNW {
        _RUNW { w: self }
    }
    #[doc = "Bit 1 - Used only when WORD_LENGTH = 3, i"]
    #[inline]
    pub fn data_format_24_bit(&mut self) -> _DATA_FORMAT_24_BITW {
        _DATA_FORMAT_24_BITW { w: self }
    }
    #[doc = "Bit 2 - Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline]
    pub fn data_format_18_bit(&mut self) -> _DATA_FORMAT_18_BITW {
        _DATA_FORMAT_18_BITW { w: self }
    }
    #[doc = "Bit 3 - When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline]
    pub fn data_format_16_bit(&mut self) -> _DATA_FORMAT_16_BITW {
        _DATA_FORMAT_16_BITW { w: self }
    }
    #[doc = "Bit 5 - Set this bit to make the eLCDIF act as a bus master"]
    #[inline]
    pub fn master(&mut self) -> _MASTERW {
        _MASTERW { w: self }
    }
    #[doc = "Bit 6 - If this bit is set and LCDIF_MASTER bit is set, the eLCDIF will act as bus master and the handshake mechanism between eLCDIF and PXP will be turned on"]
    #[inline]
    pub fn enable_pxp_handshake(&mut self) -> _ENABLE_PXP_HANDSHAKEW {
        _ENABLE_PXP_HANDSHAKEW { w: self }
    }
    #[doc = "Bits 8:9 - Input data format."]
    #[inline]
    pub fn word_length(&mut self) -> _WORD_LENGTHW {
        _WORD_LENGTHW { w: self }
    }
    #[doc = "Bits 10:11 - LCD Data bus transfer width."]
    #[inline]
    pub fn lcd_databus_width(&mut self) -> _LCD_DATABUS_WIDTHW {
        _LCD_DATABUS_WIDTHW { w: self }
    }
    #[doc = "Bits 12:13 - This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline]
    pub fn csc_data_swizzle(&mut self) -> _CSC_DATA_SWIZZLEW {
        _CSC_DATA_SWIZZLEW { w: self }
    }
    #[doc = "Bits 14:15 - This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline]
    pub fn input_data_swizzle(&mut self) -> _INPUT_DATA_SWIZZLEW {
        _INPUT_DATA_SWIZZLEW { w: self }
    }
    #[doc = "Bit 17 - Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline]
    pub fn dotclk_mode(&mut self) -> _DOTCLK_MODEW {
        _DOTCLK_MODEW { w: self }
    }
    #[doc = "Bit 19 - When this bit is 0, it means that eLCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline]
    pub fn bypass_count(&mut self) -> _BYPASS_COUNTW {
        _BYPASS_COUNTW { w: self }
    }
    #[doc = "Bits 21:25 - The data to be transmitted is shifted left or right by this number of bits."]
    #[inline]
    pub fn shift_num_bits(&mut self) -> _SHIFT_NUM_BITSW {
        _SHIFT_NUM_BITSW { w: self }
    }
    #[doc = "Bit 26 - Use this bit to determine the direction of shift of transmit data."]
    #[inline]
    pub fn data_shift_dir(&mut self) -> _DATA_SHIFT_DIRW {
        _DATA_SHIFT_DIRW { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - This bit must be set to zero to enable normal operation of the eLCDIF"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
