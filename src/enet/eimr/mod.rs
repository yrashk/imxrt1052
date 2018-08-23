#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EIMR {
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
pub struct TS_TIMERR {
    bits: bool,
}
impl TS_TIMERR {
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
pub struct TS_AVAILR {
    bits: bool,
}
impl TS_AVAILR {
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
pub struct WAKEUPR {
    bits: bool,
}
impl WAKEUPR {
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
pub struct PLRR {
    bits: bool,
}
impl PLRR {
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
pub struct UNR {
    bits: bool,
}
impl UNR {
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
pub struct RLR {
    bits: bool,
}
impl RLR {
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
pub struct LCR {
    bits: bool,
}
impl LCR {
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
pub struct EBERRR {
    bits: bool,
}
impl EBERRR {
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
pub struct MIIR {
    bits: bool,
}
impl MIIR {
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
pub struct RXBR {
    bits: bool,
}
impl RXBR {
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
pub struct RXFR {
    bits: bool,
}
impl RXFR {
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
#[doc = "Possible values of the field `TXB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBR {
    #[doc = "The corresponding interrupt source is masked."]
    TXB_0,
    #[doc = "The corresponding interrupt source is not masked."]
    TXB_1,
}
impl TXBR {
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
            TXBR::TXB_0 => false,
            TXBR::TXB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXBR {
        match value {
            false => TXBR::TXB_0,
            true => TXBR::TXB_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXB_0`"]
    #[inline]
    pub fn is_txb_0(&self) -> bool {
        *self == TXBR::TXB_0
    }
    #[doc = "Checks if the value of the field is `TXB_1`"]
    #[inline]
    pub fn is_txb_1(&self) -> bool {
        *self == TXBR::TXB_1
    }
}
#[doc = "Possible values of the field `TXF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFR {
    #[doc = "The corresponding interrupt source is masked."]
    TXF_0,
    #[doc = "The corresponding interrupt source is not masked."]
    TXF_1,
}
impl TXFR {
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
            TXFR::TXF_0 => false,
            TXFR::TXF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFR {
        match value {
            false => TXFR::TXF_0,
            true => TXFR::TXF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXF_0`"]
    #[inline]
    pub fn is_txf_0(&self) -> bool {
        *self == TXFR::TXF_0
    }
    #[doc = "Checks if the value of the field is `TXF_1`"]
    #[inline]
    pub fn is_txf_1(&self) -> bool {
        *self == TXFR::TXF_1
    }
}
#[doc = "Possible values of the field `GRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GRAR {
    #[doc = "The corresponding interrupt source is masked."]
    GRA_0,
    #[doc = "The corresponding interrupt source is not masked."]
    GRA_1,
}
impl GRAR {
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
            GRAR::GRA_0 => false,
            GRAR::GRA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GRAR {
        match value {
            false => GRAR::GRA_0,
            true => GRAR::GRA_1,
        }
    }
    #[doc = "Checks if the value of the field is `GRA_0`"]
    #[inline]
    pub fn is_gra_0(&self) -> bool {
        *self == GRAR::GRA_0
    }
    #[doc = "Checks if the value of the field is `GRA_1`"]
    #[inline]
    pub fn is_gra_1(&self) -> bool {
        *self == GRAR::GRA_1
    }
}
#[doc = "Possible values of the field `BABT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BABTR {
    #[doc = "The corresponding interrupt source is masked."]
    BABT_0,
    #[doc = "The corresponding interrupt source is not masked."]
    BABT_1,
}
impl BABTR {
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
            BABTR::BABT_0 => false,
            BABTR::BABT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BABTR {
        match value {
            false => BABTR::BABT_0,
            true => BABTR::BABT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BABT_0`"]
    #[inline]
    pub fn is_babt_0(&self) -> bool {
        *self == BABTR::BABT_0
    }
    #[doc = "Checks if the value of the field is `BABT_1`"]
    #[inline]
    pub fn is_babt_1(&self) -> bool {
        *self == BABTR::BABT_1
    }
}
#[doc = "Possible values of the field `BABR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BABRR {
    #[doc = "The corresponding interrupt source is masked."]
    BABR_0,
    #[doc = "The corresponding interrupt source is not masked."]
    BABR_1,
}
impl BABRR {
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
            BABRR::BABR_0 => false,
            BABRR::BABR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BABRR {
        match value {
            false => BABRR::BABR_0,
            true => BABRR::BABR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BABR_0`"]
    #[inline]
    pub fn is_babr_0(&self) -> bool {
        *self == BABRR::BABR_0
    }
    #[doc = "Checks if the value of the field is `BABR_1`"]
    #[inline]
    pub fn is_babr_1(&self) -> bool {
        *self == BABRR::BABR_1
    }
}
#[doc = r" Proxy"]
pub struct _TS_TIMERW<'a> {
    w: &'a mut W,
}
impl<'a> _TS_TIMERW<'a> {
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
pub struct _TS_AVAILW<'a> {
    w: &'a mut W,
}
impl<'a> _TS_AVAILW<'a> {
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
pub struct _WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPW<'a> {
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
pub struct _PLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PLRW<'a> {
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
pub struct _UNW<'a> {
    w: &'a mut W,
}
impl<'a> _UNW<'a> {
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
pub struct _RLW<'a> {
    w: &'a mut W,
}
impl<'a> _RLW<'a> {
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
pub struct _LCW<'a> {
    w: &'a mut W,
}
impl<'a> _LCW<'a> {
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
pub struct _EBERRW<'a> {
    w: &'a mut W,
}
impl<'a> _EBERRW<'a> {
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
#[doc = r" Proxy"]
pub struct _MIIW<'a> {
    w: &'a mut W,
}
impl<'a> _MIIW<'a> {
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
pub struct _RXBW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBW<'a> {
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
pub struct _RXFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFW<'a> {
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
#[doc = "Values that can be written to the field `TXB`"]
pub enum TXBW {
    #[doc = "The corresponding interrupt source is masked."]
    TXB_0,
    #[doc = "The corresponding interrupt source is not masked."]
    TXB_1,
}
impl TXBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXBW::TXB_0 => false,
            TXBW::TXB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXBW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline]
    pub fn txb_0(self) -> &'a mut W {
        self.variant(TXBW::TXB_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline]
    pub fn txb_1(self) -> &'a mut W {
        self.variant(TXBW::TXB_1)
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
#[doc = "Values that can be written to the field `TXF`"]
pub enum TXFW {
    #[doc = "The corresponding interrupt source is masked."]
    TXF_0,
    #[doc = "The corresponding interrupt source is not masked."]
    TXF_1,
}
impl TXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFW::TXF_0 => false,
            TXFW::TXF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline]
    pub fn txf_0(self) -> &'a mut W {
        self.variant(TXFW::TXF_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline]
    pub fn txf_1(self) -> &'a mut W {
        self.variant(TXFW::TXF_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GRA`"]
pub enum GRAW {
    #[doc = "The corresponding interrupt source is masked."]
    GRA_0,
    #[doc = "The corresponding interrupt source is not masked."]
    GRA_1,
}
impl GRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GRAW::GRA_0 => false,
            GRAW::GRA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GRAW<'a> {
    w: &'a mut W,
}
impl<'a> _GRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline]
    pub fn gra_0(self) -> &'a mut W {
        self.variant(GRAW::GRA_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline]
    pub fn gra_1(self) -> &'a mut W {
        self.variant(GRAW::GRA_1)
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
#[doc = "Values that can be written to the field `BABT`"]
pub enum BABTW {
    #[doc = "The corresponding interrupt source is masked."]
    BABT_0,
    #[doc = "The corresponding interrupt source is not masked."]
    BABT_1,
}
impl BABTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BABTW::BABT_0 => false,
            BABTW::BABT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BABTW<'a> {
    w: &'a mut W,
}
impl<'a> _BABTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BABTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline]
    pub fn babt_0(self) -> &'a mut W {
        self.variant(BABTW::BABT_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline]
    pub fn babt_1(self) -> &'a mut W {
        self.variant(BABTW::BABT_1)
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
#[doc = "Values that can be written to the field `BABR`"]
pub enum BABRW {
    #[doc = "The corresponding interrupt source is masked."]
    BABR_0,
    #[doc = "The corresponding interrupt source is not masked."]
    BABR_1,
}
impl BABRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BABRW::BABR_0 => false,
            BABRW::BABR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BABRW<'a> {
    w: &'a mut W,
}
impl<'a> _BABRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BABRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline]
    pub fn babr_0(self) -> &'a mut W {
        self.variant(BABRW::BABR_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline]
    pub fn babr_1(self) -> &'a mut W {
        self.variant(BABRW::BABR_1)
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
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline]
    pub fn ts_timer(&self) -> TS_TIMERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TS_TIMERR { bits }
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline]
    pub fn ts_avail(&self) -> TS_AVAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TS_AVAILR { bits }
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline]
    pub fn wakeup(&self) -> WAKEUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAKEUPR { bits }
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline]
    pub fn plr(&self) -> PLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLRR { bits }
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline]
    pub fn un(&self) -> UNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNR { bits }
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline]
    pub fn rl(&self) -> RLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RLR { bits }
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline]
    pub fn lc(&self) -> LCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCR { bits }
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline]
    pub fn eberr(&self) -> EBERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EBERRR { bits }
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline]
    pub fn mii(&self) -> MIIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIIR { bits }
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline]
    pub fn rxb(&self) -> RXBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXBR { bits }
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline]
    pub fn rxf(&self) -> RXFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFR { bits }
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline]
    pub fn txb(&self) -> TXBR {
        TXBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline]
    pub fn txf(&self) -> TXFR {
        TXFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline]
    pub fn gra(&self) -> GRAR {
        GRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline]
    pub fn babt(&self) -> BABTR {
        BABTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline]
    pub fn babr(&self) -> BABRR {
        BABRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline]
    pub fn ts_timer(&mut self) -> _TS_TIMERW {
        _TS_TIMERW { w: self }
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline]
    pub fn ts_avail(&mut self) -> _TS_AVAILW {
        _TS_AVAILW { w: self }
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline]
    pub fn wakeup(&mut self) -> _WAKEUPW {
        _WAKEUPW { w: self }
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline]
    pub fn plr(&mut self) -> _PLRW {
        _PLRW { w: self }
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline]
    pub fn un(&mut self) -> _UNW {
        _UNW { w: self }
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline]
    pub fn rl(&mut self) -> _RLW {
        _RLW { w: self }
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline]
    pub fn lc(&mut self) -> _LCW {
        _LCW { w: self }
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline]
    pub fn eberr(&mut self) -> _EBERRW {
        _EBERRW { w: self }
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline]
    pub fn mii(&mut self) -> _MIIW {
        _MIIW { w: self }
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline]
    pub fn rxb(&mut self) -> _RXBW {
        _RXBW { w: self }
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline]
    pub fn rxf(&mut self) -> _RXFW {
        _RXFW { w: self }
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline]
    pub fn txb(&mut self) -> _TXBW {
        _TXBW { w: self }
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline]
    pub fn txf(&mut self) -> _TXFW {
        _TXFW { w: self }
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline]
    pub fn gra(&mut self) -> _GRAW {
        _GRAW { w: self }
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline]
    pub fn babt(&mut self) -> _BABTW {
        _BABTW { w: self }
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline]
    pub fn babr(&mut self) -> _BABRW {
        _BABRW { w: self }
    }
}
