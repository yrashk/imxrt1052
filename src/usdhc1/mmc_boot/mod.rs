#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMC_BOOT {
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
#[doc = "Possible values of the field `DTOCV_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOCV_ACKR {
    #[doc = "SDCLK x 2^14"]
    DTOCV_ACK_0,
    #[doc = "SDCLK x 2^15"]
    DTOCV_ACK_1,
    #[doc = "SDCLK x 2^16"]
    DTOCV_ACK_2,
    #[doc = "SDCLK x 2^17"]
    DTOCV_ACK_3,
    #[doc = "SDCLK x 2^18"]
    DTOCV_ACK_4,
    #[doc = "SDCLK x 2^19"]
    DTOCV_ACK_5,
    #[doc = "SDCLK x 2^20"]
    DTOCV_ACK_6,
    #[doc = "SDCLK x 2^21"]
    DTOCV_ACK_7,
    #[doc = "SDCLK x 2^28"]
    DTOCV_ACK_14,
    #[doc = "SDCLK x 2^29"]
    DTOCV_ACK_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTOCV_ACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTOCV_ACKR::DTOCV_ACK_0 => 0,
            DTOCV_ACKR::DTOCV_ACK_1 => 1,
            DTOCV_ACKR::DTOCV_ACK_2 => 2,
            DTOCV_ACKR::DTOCV_ACK_3 => 3,
            DTOCV_ACKR::DTOCV_ACK_4 => 4,
            DTOCV_ACKR::DTOCV_ACK_5 => 5,
            DTOCV_ACKR::DTOCV_ACK_6 => 6,
            DTOCV_ACKR::DTOCV_ACK_7 => 7,
            DTOCV_ACKR::DTOCV_ACK_14 => 14,
            DTOCV_ACKR::DTOCV_ACK_15 => 15,
            DTOCV_ACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTOCV_ACKR {
        match value {
            0 => DTOCV_ACKR::DTOCV_ACK_0,
            1 => DTOCV_ACKR::DTOCV_ACK_1,
            2 => DTOCV_ACKR::DTOCV_ACK_2,
            3 => DTOCV_ACKR::DTOCV_ACK_3,
            4 => DTOCV_ACKR::DTOCV_ACK_4,
            5 => DTOCV_ACKR::DTOCV_ACK_5,
            6 => DTOCV_ACKR::DTOCV_ACK_6,
            7 => DTOCV_ACKR::DTOCV_ACK_7,
            14 => DTOCV_ACKR::DTOCV_ACK_14,
            15 => DTOCV_ACKR::DTOCV_ACK_15,
            i => DTOCV_ACKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_0`"]
    #[inline]
    pub fn is_dtocv_ack_0(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_0
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_1`"]
    #[inline]
    pub fn is_dtocv_ack_1(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_1
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_2`"]
    #[inline]
    pub fn is_dtocv_ack_2(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_2
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_3`"]
    #[inline]
    pub fn is_dtocv_ack_3(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_3
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_4`"]
    #[inline]
    pub fn is_dtocv_ack_4(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_4
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_5`"]
    #[inline]
    pub fn is_dtocv_ack_5(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_5
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_6`"]
    #[inline]
    pub fn is_dtocv_ack_6(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_6
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_7`"]
    #[inline]
    pub fn is_dtocv_ack_7(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_7
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_14`"]
    #[inline]
    pub fn is_dtocv_ack_14(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_14
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_15`"]
    #[inline]
    pub fn is_dtocv_ack_15(&self) -> bool {
        *self == DTOCV_ACKR::DTOCV_ACK_15
    }
}
#[doc = "Possible values of the field `BOOT_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_ACKR {
    #[doc = "No ack"]
    BOOT_ACK_0,
    #[doc = "Ack"]
    BOOT_ACK_1,
}
impl BOOT_ACKR {
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
            BOOT_ACKR::BOOT_ACK_0 => false,
            BOOT_ACKR::BOOT_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOT_ACKR {
        match value {
            false => BOOT_ACKR::BOOT_ACK_0,
            true => BOOT_ACKR::BOOT_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_ACK_0`"]
    #[inline]
    pub fn is_boot_ack_0(&self) -> bool {
        *self == BOOT_ACKR::BOOT_ACK_0
    }
    #[doc = "Checks if the value of the field is `BOOT_ACK_1`"]
    #[inline]
    pub fn is_boot_ack_1(&self) -> bool {
        *self == BOOT_ACKR::BOOT_ACK_1
    }
}
#[doc = "Possible values of the field `BOOT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_MODER {
    #[doc = "Normal boot"]
    BOOT_MODE_0,
    #[doc = "Alternative boot"]
    BOOT_MODE_1,
}
impl BOOT_MODER {
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
            BOOT_MODER::BOOT_MODE_0 => false,
            BOOT_MODER::BOOT_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOT_MODER {
        match value {
            false => BOOT_MODER::BOOT_MODE_0,
            true => BOOT_MODER::BOOT_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_MODE_0`"]
    #[inline]
    pub fn is_boot_mode_0(&self) -> bool {
        *self == BOOT_MODER::BOOT_MODE_0
    }
    #[doc = "Checks if the value of the field is `BOOT_MODE_1`"]
    #[inline]
    pub fn is_boot_mode_1(&self) -> bool {
        *self == BOOT_MODER::BOOT_MODE_1
    }
}
#[doc = "Possible values of the field `BOOT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_ENR {
    #[doc = "Fast boot disable"]
    BOOT_EN_0,
    #[doc = "Fast boot enable"]
    BOOT_EN_1,
}
impl BOOT_ENR {
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
            BOOT_ENR::BOOT_EN_0 => false,
            BOOT_ENR::BOOT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOT_ENR {
        match value {
            false => BOOT_ENR::BOOT_EN_0,
            true => BOOT_ENR::BOOT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_EN_0`"]
    #[inline]
    pub fn is_boot_en_0(&self) -> bool {
        *self == BOOT_ENR::BOOT_EN_0
    }
    #[doc = "Checks if the value of the field is `BOOT_EN_1`"]
    #[inline]
    pub fn is_boot_en_1(&self) -> bool {
        *self == BOOT_ENR::BOOT_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct AUTO_SABG_ENR {
    bits: bool,
}
impl AUTO_SABG_ENR {
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
#[doc = "Possible values of the field `DISABLE_TIME_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_TIME_OUTR {
    #[doc = "Enable time out"]
    DISABLE_TIME_OUT_0,
    #[doc = "Disable time out"]
    DISABLE_TIME_OUT_1,
}
impl DISABLE_TIME_OUTR {
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
            DISABLE_TIME_OUTR::DISABLE_TIME_OUT_0 => false,
            DISABLE_TIME_OUTR::DISABLE_TIME_OUT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLE_TIME_OUTR {
        match value {
            false => DISABLE_TIME_OUTR::DISABLE_TIME_OUT_0,
            true => DISABLE_TIME_OUTR::DISABLE_TIME_OUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_TIME_OUT_0`"]
    #[inline]
    pub fn is_disable_time_out_0(&self) -> bool {
        *self == DISABLE_TIME_OUTR::DISABLE_TIME_OUT_0
    }
    #[doc = "Checks if the value of the field is `DISABLE_TIME_OUT_1`"]
    #[inline]
    pub fn is_disable_time_out_1(&self) -> bool {
        *self == DISABLE_TIME_OUTR::DISABLE_TIME_OUT_1
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_BLK_CNTR {
    bits: u16,
}
impl BOOT_BLK_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DTOCV_ACK`"]
pub enum DTOCV_ACKW {
    #[doc = "SDCLK x 2^14"]
    DTOCV_ACK_0,
    #[doc = "SDCLK x 2^15"]
    DTOCV_ACK_1,
    #[doc = "SDCLK x 2^16"]
    DTOCV_ACK_2,
    #[doc = "SDCLK x 2^17"]
    DTOCV_ACK_3,
    #[doc = "SDCLK x 2^18"]
    DTOCV_ACK_4,
    #[doc = "SDCLK x 2^19"]
    DTOCV_ACK_5,
    #[doc = "SDCLK x 2^20"]
    DTOCV_ACK_6,
    #[doc = "SDCLK x 2^21"]
    DTOCV_ACK_7,
    #[doc = "SDCLK x 2^28"]
    DTOCV_ACK_14,
    #[doc = "SDCLK x 2^29"]
    DTOCV_ACK_15,
}
impl DTOCV_ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTOCV_ACKW::DTOCV_ACK_0 => 0,
            DTOCV_ACKW::DTOCV_ACK_1 => 1,
            DTOCV_ACKW::DTOCV_ACK_2 => 2,
            DTOCV_ACKW::DTOCV_ACK_3 => 3,
            DTOCV_ACKW::DTOCV_ACK_4 => 4,
            DTOCV_ACKW::DTOCV_ACK_5 => 5,
            DTOCV_ACKW::DTOCV_ACK_6 => 6,
            DTOCV_ACKW::DTOCV_ACK_7 => 7,
            DTOCV_ACKW::DTOCV_ACK_14 => 14,
            DTOCV_ACKW::DTOCV_ACK_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOCV_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOCV_ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOCV_ACKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDCLK x 2^14"]
    #[inline]
    pub fn dtocv_ack_0(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_0)
    }
    #[doc = "SDCLK x 2^15"]
    #[inline]
    pub fn dtocv_ack_1(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_1)
    }
    #[doc = "SDCLK x 2^16"]
    #[inline]
    pub fn dtocv_ack_2(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_2)
    }
    #[doc = "SDCLK x 2^17"]
    #[inline]
    pub fn dtocv_ack_3(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_3)
    }
    #[doc = "SDCLK x 2^18"]
    #[inline]
    pub fn dtocv_ack_4(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_4)
    }
    #[doc = "SDCLK x 2^19"]
    #[inline]
    pub fn dtocv_ack_5(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_5)
    }
    #[doc = "SDCLK x 2^20"]
    #[inline]
    pub fn dtocv_ack_6(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_6)
    }
    #[doc = "SDCLK x 2^21"]
    #[inline]
    pub fn dtocv_ack_7(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_7)
    }
    #[doc = "SDCLK x 2^28"]
    #[inline]
    pub fn dtocv_ack_14(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_14)
    }
    #[doc = "SDCLK x 2^29"]
    #[inline]
    pub fn dtocv_ack_15(self) -> &'a mut W {
        self.variant(DTOCV_ACKW::DTOCV_ACK_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOOT_ACK`"]
pub enum BOOT_ACKW {
    #[doc = "No ack"]
    BOOT_ACK_0,
    #[doc = "Ack"]
    BOOT_ACK_1,
}
impl BOOT_ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOT_ACKW::BOOT_ACK_0 => false,
            BOOT_ACKW::BOOT_ACK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOT_ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ack"]
    #[inline]
    pub fn boot_ack_0(self) -> &'a mut W {
        self.variant(BOOT_ACKW::BOOT_ACK_0)
    }
    #[doc = "Ack"]
    #[inline]
    pub fn boot_ack_1(self) -> &'a mut W {
        self.variant(BOOT_ACKW::BOOT_ACK_1)
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
#[doc = "Values that can be written to the field `BOOT_MODE`"]
pub enum BOOT_MODEW {
    #[doc = "Normal boot"]
    BOOT_MODE_0,
    #[doc = "Alternative boot"]
    BOOT_MODE_1,
}
impl BOOT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOT_MODEW::BOOT_MODE_0 => false,
            BOOT_MODEW::BOOT_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOT_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal boot"]
    #[inline]
    pub fn boot_mode_0(self) -> &'a mut W {
        self.variant(BOOT_MODEW::BOOT_MODE_0)
    }
    #[doc = "Alternative boot"]
    #[inline]
    pub fn boot_mode_1(self) -> &'a mut W {
        self.variant(BOOT_MODEW::BOOT_MODE_1)
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
#[doc = "Values that can be written to the field `BOOT_EN`"]
pub enum BOOT_ENW {
    #[doc = "Fast boot disable"]
    BOOT_EN_0,
    #[doc = "Fast boot enable"]
    BOOT_EN_1,
}
impl BOOT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOT_ENW::BOOT_EN_0 => false,
            BOOT_ENW::BOOT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast boot disable"]
    #[inline]
    pub fn boot_en_0(self) -> &'a mut W {
        self.variant(BOOT_ENW::BOOT_EN_0)
    }
    #[doc = "Fast boot enable"]
    #[inline]
    pub fn boot_en_1(self) -> &'a mut W {
        self.variant(BOOT_ENW::BOOT_EN_1)
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
pub struct _AUTO_SABG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_SABG_ENW<'a> {
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
#[doc = "Values that can be written to the field `DISABLE_TIME_OUT`"]
pub enum DISABLE_TIME_OUTW {
    #[doc = "Enable time out"]
    DISABLE_TIME_OUT_0,
    #[doc = "Disable time out"]
    DISABLE_TIME_OUT_1,
}
impl DISABLE_TIME_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISABLE_TIME_OUTW::DISABLE_TIME_OUT_0 => false,
            DISABLE_TIME_OUTW::DISABLE_TIME_OUT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_TIME_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_TIME_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLE_TIME_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable time out"]
    #[inline]
    pub fn disable_time_out_0(self) -> &'a mut W {
        self.variant(DISABLE_TIME_OUTW::DISABLE_TIME_OUT_0)
    }
    #[doc = "Disable time out"]
    #[inline]
    pub fn disable_time_out_1(self) -> &'a mut W {
        self.variant(DISABLE_TIME_OUTW::DISABLE_TIME_OUT_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_BLK_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_BLK_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:3 - DTOCV_ACK"]
    #[inline]
    pub fn dtocv_ack(&self) -> DTOCV_ACKR {
        DTOCV_ACKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - BOOT_ACK"]
    #[inline]
    pub fn boot_ack(&self) -> BOOT_ACKR {
        BOOT_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - BOOT_MODE"]
    #[inline]
    pub fn boot_mode(&self) -> BOOT_MODER {
        BOOT_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - BOOT_EN"]
    #[inline]
    pub fn boot_en(&self) -> BOOT_ENR {
        BOOT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - AUTO_SABG_EN"]
    #[inline]
    pub fn auto_sabg_en(&self) -> AUTO_SABG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTO_SABG_ENR { bits }
    }
    #[doc = "Bit 8 - Disable Time Out"]
    #[inline]
    pub fn disable_time_out(&self) -> DISABLE_TIME_OUTR {
        DISABLE_TIME_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - BOOT_BLK_CNT"]
    #[inline]
    pub fn boot_blk_cnt(&self) -> BOOT_BLK_CNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BOOT_BLK_CNTR { bits }
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
    #[doc = "Bits 0:3 - DTOCV_ACK"]
    #[inline]
    pub fn dtocv_ack(&mut self) -> _DTOCV_ACKW {
        _DTOCV_ACKW { w: self }
    }
    #[doc = "Bit 4 - BOOT_ACK"]
    #[inline]
    pub fn boot_ack(&mut self) -> _BOOT_ACKW {
        _BOOT_ACKW { w: self }
    }
    #[doc = "Bit 5 - BOOT_MODE"]
    #[inline]
    pub fn boot_mode(&mut self) -> _BOOT_MODEW {
        _BOOT_MODEW { w: self }
    }
    #[doc = "Bit 6 - BOOT_EN"]
    #[inline]
    pub fn boot_en(&mut self) -> _BOOT_ENW {
        _BOOT_ENW { w: self }
    }
    #[doc = "Bit 7 - AUTO_SABG_EN"]
    #[inline]
    pub fn auto_sabg_en(&mut self) -> _AUTO_SABG_ENW {
        _AUTO_SABG_ENW { w: self }
    }
    #[doc = "Bit 8 - Disable Time Out"]
    #[inline]
    pub fn disable_time_out(&mut self) -> _DISABLE_TIME_OUTW {
        _DISABLE_TIME_OUTW { w: self }
    }
    #[doc = "Bits 16:31 - BOOT_BLK_CNT"]
    #[inline]
    pub fn boot_blk_cnt(&mut self) -> _BOOT_BLK_CNTW {
        _BOOT_BLK_CNTW { w: self }
    }
}
