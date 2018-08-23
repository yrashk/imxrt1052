#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC2_TOG {
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
#[doc = "Possible values of the field `REG0_BO_OFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_BO_OFFSETR {
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REG0_BO_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG0_BO_OFFSETR::REG0_BO_OFFSET_4 => 4,
            REG0_BO_OFFSETR::REG0_BO_OFFSET_7 => 7,
            REG0_BO_OFFSETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG0_BO_OFFSETR {
        match value {
            4 => REG0_BO_OFFSETR::REG0_BO_OFFSET_4,
            7 => REG0_BO_OFFSETR::REG0_BO_OFFSET_7,
            i => REG0_BO_OFFSETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_BO_OFFSET_4`"]
    #[inline]
    pub fn is_reg0_bo_offset_4(&self) -> bool {
        *self == REG0_BO_OFFSETR::REG0_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG0_BO_OFFSET_7`"]
    #[inline]
    pub fn is_reg0_bo_offset_7(&self) -> bool {
        *self == REG0_BO_OFFSETR::REG0_BO_OFFSET_7
    }
}
#[doc = "Possible values of the field `REG0_BO_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_BO_STATUSR {
    #[doc = "Brownout, supply is below target minus brownout offset."]
    REG0_BO_STATUS_1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl REG0_BO_STATUSR {
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
            REG0_BO_STATUSR::REG0_BO_STATUS_1 => true,
            REG0_BO_STATUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REG0_BO_STATUSR {
        match value {
            true => REG0_BO_STATUSR::REG0_BO_STATUS_1,
            i => REG0_BO_STATUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_BO_STATUS_1`"]
    #[inline]
    pub fn is_reg0_bo_status_1(&self) -> bool {
        *self == REG0_BO_STATUSR::REG0_BO_STATUS_1
    }
}
#[doc = r" Value of the field"]
pub struct REG0_ENABLE_BOR {
    bits: bool,
}
impl REG0_ENABLE_BOR {
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
pub struct PLL3_DISABLER {
    bits: bool,
}
impl PLL3_DISABLER {
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
#[doc = "Possible values of the field `AUDIO_DIV_LSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_DIV_LSBR {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_LSB_0,
    #[doc = "divide by 2"]
    AUDIO_DIV_LSB_1,
}
impl AUDIO_DIV_LSBR {
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
            AUDIO_DIV_LSBR::AUDIO_DIV_LSB_0 => false,
            AUDIO_DIV_LSBR::AUDIO_DIV_LSB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUDIO_DIV_LSBR {
        match value {
            false => AUDIO_DIV_LSBR::AUDIO_DIV_LSB_0,
            true => AUDIO_DIV_LSBR::AUDIO_DIV_LSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_LSB_0`"]
    #[inline]
    pub fn is_audio_div_lsb_0(&self) -> bool {
        *self == AUDIO_DIV_LSBR::AUDIO_DIV_LSB_0
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_LSB_1`"]
    #[inline]
    pub fn is_audio_div_lsb_1(&self) -> bool {
        *self == AUDIO_DIV_LSBR::AUDIO_DIV_LSB_1
    }
}
#[doc = "Possible values of the field `REG2_BO_OFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG2_BO_OFFSETR {
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REG2_BO_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG2_BO_OFFSETR::REG2_BO_OFFSET_4 => 4,
            REG2_BO_OFFSETR::REG2_BO_OFFSET_7 => 7,
            REG2_BO_OFFSETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG2_BO_OFFSETR {
        match value {
            4 => REG2_BO_OFFSETR::REG2_BO_OFFSET_4,
            7 => REG2_BO_OFFSETR::REG2_BO_OFFSET_7,
            i => REG2_BO_OFFSETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG2_BO_OFFSET_4`"]
    #[inline]
    pub fn is_reg2_bo_offset_4(&self) -> bool {
        *self == REG2_BO_OFFSETR::REG2_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG2_BO_OFFSET_7`"]
    #[inline]
    pub fn is_reg2_bo_offset_7(&self) -> bool {
        *self == REG2_BO_OFFSETR::REG2_BO_OFFSET_7
    }
}
#[doc = r" Value of the field"]
pub struct REG2_BO_STATUSR {
    bits: bool,
}
impl REG2_BO_STATUSR {
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
pub struct REG2_ENABLE_BOR {
    bits: bool,
}
impl REG2_ENABLE_BOR {
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
pub struct REG2_OKR {
    bits: bool,
}
impl REG2_OKR {
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
#[doc = "Possible values of the field `AUDIO_DIV_MSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_DIV_MSBR {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_MSB_0,
    #[doc = "divide by 2"]
    AUDIO_DIV_MSB_1,
}
impl AUDIO_DIV_MSBR {
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
            AUDIO_DIV_MSBR::AUDIO_DIV_MSB_0 => false,
            AUDIO_DIV_MSBR::AUDIO_DIV_MSB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUDIO_DIV_MSBR {
        match value {
            false => AUDIO_DIV_MSBR::AUDIO_DIV_MSB_0,
            true => AUDIO_DIV_MSBR::AUDIO_DIV_MSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_MSB_0`"]
    #[inline]
    pub fn is_audio_div_msb_0(&self) -> bool {
        *self == AUDIO_DIV_MSBR::AUDIO_DIV_MSB_0
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_MSB_1`"]
    #[inline]
    pub fn is_audio_div_msb_1(&self) -> bool {
        *self == AUDIO_DIV_MSBR::AUDIO_DIV_MSB_1
    }
}
#[doc = "Possible values of the field `REG0_STEP_TIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_STEP_TIMER {
    #[doc = "64"]
    _64_CLOCKS,
    #[doc = "128"]
    _128_CLOCKS,
    #[doc = "256"]
    _256_CLOCKS,
    #[doc = "512"]
    _512_CLOCKS,
}
impl REG0_STEP_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG0_STEP_TIMER::_64_CLOCKS => 0,
            REG0_STEP_TIMER::_128_CLOCKS => 1,
            REG0_STEP_TIMER::_256_CLOCKS => 2,
            REG0_STEP_TIMER::_512_CLOCKS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG0_STEP_TIMER {
        match value {
            0 => REG0_STEP_TIMER::_64_CLOCKS,
            1 => REG0_STEP_TIMER::_128_CLOCKS,
            2 => REG0_STEP_TIMER::_256_CLOCKS,
            3 => REG0_STEP_TIMER::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG0_STEP_TIMER::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG0_STEP_TIMER::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG0_STEP_TIMER::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG0_STEP_TIMER::_512_CLOCKS
    }
}
#[doc = "Possible values of the field `REG2_STEP_TIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG2_STEP_TIMER {
    #[doc = "64"]
    _64_CLOCKS,
    #[doc = "128"]
    _128_CLOCKS,
    #[doc = "256"]
    _256_CLOCKS,
    #[doc = "512"]
    _512_CLOCKS,
}
impl REG2_STEP_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG2_STEP_TIMER::_64_CLOCKS => 0,
            REG2_STEP_TIMER::_128_CLOCKS => 1,
            REG2_STEP_TIMER::_256_CLOCKS => 2,
            REG2_STEP_TIMER::_512_CLOCKS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG2_STEP_TIMER {
        match value {
            0 => REG2_STEP_TIMER::_64_CLOCKS,
            1 => REG2_STEP_TIMER::_128_CLOCKS,
            2 => REG2_STEP_TIMER::_256_CLOCKS,
            3 => REG2_STEP_TIMER::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG2_STEP_TIMER::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG2_STEP_TIMER::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG2_STEP_TIMER::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG2_STEP_TIMER::_512_CLOCKS
    }
}
#[doc = "Possible values of the field `VIDEO_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIDEO_DIVR {
    #[doc = "divide by 1 (Default)"]
    VIDEO_DIV_0,
    #[doc = "divide by 2"]
    VIDEO_DIV_1,
    #[doc = "divide by 1"]
    VIDEO_DIV_2,
    #[doc = "divide by 4"]
    VIDEO_DIV_3,
}
impl VIDEO_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VIDEO_DIVR::VIDEO_DIV_0 => 0,
            VIDEO_DIVR::VIDEO_DIV_1 => 1,
            VIDEO_DIVR::VIDEO_DIV_2 => 2,
            VIDEO_DIVR::VIDEO_DIV_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VIDEO_DIVR {
        match value {
            0 => VIDEO_DIVR::VIDEO_DIV_0,
            1 => VIDEO_DIVR::VIDEO_DIV_1,
            2 => VIDEO_DIVR::VIDEO_DIV_2,
            3 => VIDEO_DIVR::VIDEO_DIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_0`"]
    #[inline]
    pub fn is_video_div_0(&self) -> bool {
        *self == VIDEO_DIVR::VIDEO_DIV_0
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_1`"]
    #[inline]
    pub fn is_video_div_1(&self) -> bool {
        *self == VIDEO_DIVR::VIDEO_DIV_1
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_2`"]
    #[inline]
    pub fn is_video_div_2(&self) -> bool {
        *self == VIDEO_DIVR::VIDEO_DIV_2
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_3`"]
    #[inline]
    pub fn is_video_div_3(&self) -> bool {
        *self == VIDEO_DIVR::VIDEO_DIV_3
    }
}
#[doc = r" Proxy"]
pub struct _REG0_ENABLE_BOW<'a> {
    w: &'a mut W,
}
impl<'a> _REG0_ENABLE_BOW<'a> {
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
pub struct _PLL3_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL3_DISABLEW<'a> {
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
#[doc = "Values that can be written to the field `AUDIO_DIV_LSB`"]
pub enum AUDIO_DIV_LSBW {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_LSB_0,
    #[doc = "divide by 2"]
    AUDIO_DIV_LSB_1,
}
impl AUDIO_DIV_LSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUDIO_DIV_LSBW::AUDIO_DIV_LSB_0 => false,
            AUDIO_DIV_LSBW::AUDIO_DIV_LSB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUDIO_DIV_LSBW<'a> {
    w: &'a mut W,
}
impl<'a> _AUDIO_DIV_LSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUDIO_DIV_LSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "divide by 1 (Default)"]
    #[inline]
    pub fn audio_div_lsb_0(self) -> &'a mut W {
        self.variant(AUDIO_DIV_LSBW::AUDIO_DIV_LSB_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn audio_div_lsb_1(self) -> &'a mut W {
        self.variant(AUDIO_DIV_LSBW::AUDIO_DIV_LSB_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REG2_ENABLE_BOW<'a> {
    w: &'a mut W,
}
impl<'a> _REG2_ENABLE_BOW<'a> {
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
#[doc = "Values that can be written to the field `AUDIO_DIV_MSB`"]
pub enum AUDIO_DIV_MSBW {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_MSB_0,
    #[doc = "divide by 2"]
    AUDIO_DIV_MSB_1,
}
impl AUDIO_DIV_MSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUDIO_DIV_MSBW::AUDIO_DIV_MSB_0 => false,
            AUDIO_DIV_MSBW::AUDIO_DIV_MSB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUDIO_DIV_MSBW<'a> {
    w: &'a mut W,
}
impl<'a> _AUDIO_DIV_MSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUDIO_DIV_MSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "divide by 1 (Default)"]
    #[inline]
    pub fn audio_div_msb_0(self) -> &'a mut W {
        self.variant(AUDIO_DIV_MSBW::AUDIO_DIV_MSB_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn audio_div_msb_1(self) -> &'a mut W {
        self.variant(AUDIO_DIV_MSBW::AUDIO_DIV_MSB_1)
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
#[doc = "Values that can be written to the field `REG0_STEP_TIME`"]
pub enum REG0_STEP_TIMEW {
    #[doc = "64"]
    _64_CLOCKS,
    #[doc = "128"]
    _128_CLOCKS,
    #[doc = "256"]
    _256_CLOCKS,
    #[doc = "512"]
    _512_CLOCKS,
}
impl REG0_STEP_TIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG0_STEP_TIMEW::_64_CLOCKS => 0,
            REG0_STEP_TIMEW::_128_CLOCKS => 1,
            REG0_STEP_TIMEW::_256_CLOCKS => 2,
            REG0_STEP_TIMEW::_512_CLOCKS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG0_STEP_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _REG0_STEP_TIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG0_STEP_TIMEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64"]
    #[inline]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIMEW::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIMEW::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIMEW::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIMEW::_512_CLOCKS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REG2_STEP_TIME`"]
pub enum REG2_STEP_TIMEW {
    #[doc = "64"]
    _64_CLOCKS,
    #[doc = "128"]
    _128_CLOCKS,
    #[doc = "256"]
    _256_CLOCKS,
    #[doc = "512"]
    _512_CLOCKS,
}
impl REG2_STEP_TIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG2_STEP_TIMEW::_64_CLOCKS => 0,
            REG2_STEP_TIMEW::_128_CLOCKS => 1,
            REG2_STEP_TIMEW::_256_CLOCKS => 2,
            REG2_STEP_TIMEW::_512_CLOCKS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG2_STEP_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _REG2_STEP_TIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG2_STEP_TIMEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64"]
    #[inline]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIMEW::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIMEW::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIMEW::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIMEW::_512_CLOCKS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VIDEO_DIV`"]
pub enum VIDEO_DIVW {
    #[doc = "divide by 1 (Default)"]
    VIDEO_DIV_0,
    #[doc = "divide by 2"]
    VIDEO_DIV_1,
    #[doc = "divide by 1"]
    VIDEO_DIV_2,
    #[doc = "divide by 4"]
    VIDEO_DIV_3,
}
impl VIDEO_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VIDEO_DIVW::VIDEO_DIV_0 => 0,
            VIDEO_DIVW::VIDEO_DIV_1 => 1,
            VIDEO_DIVW::VIDEO_DIV_2 => 2,
            VIDEO_DIVW::VIDEO_DIV_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIDEO_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _VIDEO_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIDEO_DIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1 (Default)"]
    #[inline]
    pub fn video_div_0(self) -> &'a mut W {
        self.variant(VIDEO_DIVW::VIDEO_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn video_div_1(self) -> &'a mut W {
        self.variant(VIDEO_DIVW::VIDEO_DIV_1)
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn video_div_2(self) -> &'a mut W {
        self.variant(VIDEO_DIVW::VIDEO_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn video_div_3(self) -> &'a mut W {
        self.variant(VIDEO_DIVW::VIDEO_DIV_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:2 - This field defines the brown out voltage offset for the CORE power domain"]
    #[inline]
    pub fn reg0_bo_offset(&self) -> REG0_BO_OFFSETR {
        REG0_BO_OFFSETR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Reg0 brownout status bit."]
    #[inline]
    pub fn reg0_bo_status(&self) -> REG0_BO_STATUSR {
        REG0_BO_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enables the brownout detection."]
    #[inline]
    pub fn reg0_enable_bo(&self) -> REG0_ENABLE_BOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REG0_ENABLE_BOR { bits }
    }
    #[doc = "Bit 7 - Default value of \"0\""]
    #[inline]
    pub fn pll3_disable(&self) -> PLL3_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL3_DISABLER { bits }
    }
    #[doc = "Bit 15 - LSB of Post-divider for Audio PLL"]
    #[inline]
    pub fn audio_div_lsb(&self) -> AUDIO_DIV_LSBR {
        AUDIO_DIV_LSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - This field defines the brown out voltage offset for the xPU power domain"]
    #[inline]
    pub fn reg2_bo_offset(&self) -> REG2_BO_OFFSETR {
        REG2_BO_OFFSETR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Reg2 brownout status bit."]
    #[inline]
    pub fn reg2_bo_status(&self) -> REG2_BO_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REG2_BO_STATUSR { bits }
    }
    #[doc = "Bit 21 - Enables the brownout detection."]
    #[inline]
    pub fn reg2_enable_bo(&self) -> REG2_ENABLE_BOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REG2_ENABLE_BOR { bits }
    }
    #[doc = "Bit 22 - Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline]
    pub fn reg2_ok(&self) -> REG2_OKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REG2_OKR { bits }
    }
    #[doc = "Bit 23 - MSB of Post-divider for Audio PLL"]
    #[inline]
    pub fn audio_div_msb(&self) -> AUDIO_DIV_MSBR {
        AUDIO_DIV_MSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Number of clock periods (24MHz clock)."]
    #[inline]
    pub fn reg0_step_time(&self) -> REG0_STEP_TIMER {
        REG0_STEP_TIMER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Number of clock periods (24MHz clock)."]
    #[inline]
    pub fn reg2_step_time(&self) -> REG2_STEP_TIMER {
        REG2_STEP_TIMER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Post-divider for video"]
    #[inline]
    pub fn video_div(&self) -> VIDEO_DIVR {
        VIDEO_DIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2565927 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Enables the brownout detection."]
    #[inline]
    pub fn reg0_enable_bo(&mut self) -> _REG0_ENABLE_BOW {
        _REG0_ENABLE_BOW { w: self }
    }
    #[doc = "Bit 7 - Default value of \"0\""]
    #[inline]
    pub fn pll3_disable(&mut self) -> _PLL3_DISABLEW {
        _PLL3_DISABLEW { w: self }
    }
    #[doc = "Bit 15 - LSB of Post-divider for Audio PLL"]
    #[inline]
    pub fn audio_div_lsb(&mut self) -> _AUDIO_DIV_LSBW {
        _AUDIO_DIV_LSBW { w: self }
    }
    #[doc = "Bit 21 - Enables the brownout detection."]
    #[inline]
    pub fn reg2_enable_bo(&mut self) -> _REG2_ENABLE_BOW {
        _REG2_ENABLE_BOW { w: self }
    }
    #[doc = "Bit 23 - MSB of Post-divider for Audio PLL"]
    #[inline]
    pub fn audio_div_msb(&mut self) -> _AUDIO_DIV_MSBW {
        _AUDIO_DIV_MSBW { w: self }
    }
    #[doc = "Bits 24:25 - Number of clock periods (24MHz clock)."]
    #[inline]
    pub fn reg0_step_time(&mut self) -> _REG0_STEP_TIMEW {
        _REG0_STEP_TIMEW { w: self }
    }
    #[doc = "Bits 28:29 - Number of clock periods (24MHz clock)."]
    #[inline]
    pub fn reg2_step_time(&mut self) -> _REG2_STEP_TIMEW {
        _REG2_STEP_TIMEW { w: self }
    }
    #[doc = "Bits 30:31 - Post-divider for video"]
    #[inline]
    pub fn video_div(&mut self) -> _VIDEO_DIVW {
        _VIDEO_DIVW { w: self }
    }
}
