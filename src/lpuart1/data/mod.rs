#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DATA {
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
pub struct R0T0R {
    bits: bool,
}
impl R0T0R {
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
pub struct R1T1R {
    bits: bool,
}
impl R1T1R {
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
pub struct R2T2R {
    bits: bool,
}
impl R2T2R {
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
pub struct R3T3R {
    bits: bool,
}
impl R3T3R {
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
pub struct R4T4R {
    bits: bool,
}
impl R4T4R {
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
pub struct R5T5R {
    bits: bool,
}
impl R5T5R {
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
pub struct R6T6R {
    bits: bool,
}
impl R6T6R {
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
pub struct R7T7R {
    bits: bool,
}
impl R7T7R {
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
pub struct R8T8R {
    bits: bool,
}
impl R8T8R {
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
pub struct R9T9R {
    bits: bool,
}
impl R9T9R {
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
#[doc = "Possible values of the field `IDLINE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLINER {
    #[doc = "Receiver was not idle before receiving this character."]
    IDLINE_0,
    #[doc = "Receiver was idle before receiving this character."]
    IDLINE_1,
}
impl IDLINER {
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
            IDLINER::IDLINE_0 => false,
            IDLINER::IDLINE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLINER {
        match value {
            false => IDLINER::IDLINE_0,
            true => IDLINER::IDLINE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLINE_0`"]
    #[inline]
    pub fn is_idline_0(&self) -> bool {
        *self == IDLINER::IDLINE_0
    }
    #[doc = "Checks if the value of the field is `IDLINE_1`"]
    #[inline]
    pub fn is_idline_1(&self) -> bool {
        *self == IDLINER::IDLINE_1
    }
}
#[doc = "Possible values of the field `RXEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTR {
    #[doc = "Receive buffer contains valid data."]
    RXEMPT_0,
    #[doc = "Receive buffer is empty, data returned on read is not valid."]
    RXEMPT_1,
}
impl RXEMPTR {
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
            RXEMPTR::RXEMPT_0 => false,
            RXEMPTR::RXEMPT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEMPTR {
        match value {
            false => RXEMPTR::RXEMPT_0,
            true => RXEMPTR::RXEMPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEMPT_0`"]
    #[inline]
    pub fn is_rxempt_0(&self) -> bool {
        *self == RXEMPTR::RXEMPT_0
    }
    #[doc = "Checks if the value of the field is `RXEMPT_1`"]
    #[inline]
    pub fn is_rxempt_1(&self) -> bool {
        *self == RXEMPTR::RXEMPT_1
    }
}
#[doc = "Possible values of the field `FRETSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRETSCR {
    #[doc = "The dataword was received without a frame error on read, or transmit a normal character on write."]
    FRETSC_0,
    #[doc = "The dataword was received with a frame error, or transmit an idle or break character on transmit."]
    FRETSC_1,
}
impl FRETSCR {
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
            FRETSCR::FRETSC_0 => false,
            FRETSCR::FRETSC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRETSCR {
        match value {
            false => FRETSCR::FRETSC_0,
            true => FRETSCR::FRETSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRETSC_0`"]
    #[inline]
    pub fn is_fretsc_0(&self) -> bool {
        *self == FRETSCR::FRETSC_0
    }
    #[doc = "Checks if the value of the field is `FRETSC_1`"]
    #[inline]
    pub fn is_fretsc_1(&self) -> bool {
        *self == FRETSCR::FRETSC_1
    }
}
#[doc = "Possible values of the field `PARITYE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYER {
    #[doc = "The dataword was received without a parity error."]
    PARITYE_0,
    #[doc = "The dataword was received with a parity error."]
    PARITYE_1,
}
impl PARITYER {
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
            PARITYER::PARITYE_0 => false,
            PARITYER::PARITYE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARITYER {
        match value {
            false => PARITYER::PARITYE_0,
            true => PARITYER::PARITYE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PARITYE_0`"]
    #[inline]
    pub fn is_paritye_0(&self) -> bool {
        *self == PARITYER::PARITYE_0
    }
    #[doc = "Checks if the value of the field is `PARITYE_1`"]
    #[inline]
    pub fn is_paritye_1(&self) -> bool {
        *self == PARITYER::PARITYE_1
    }
}
#[doc = "Possible values of the field `NOISY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOISYR {
    #[doc = "The dataword was received without noise."]
    NOISY_0,
    #[doc = "The data was received with noise."]
    NOISY_1,
}
impl NOISYR {
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
            NOISYR::NOISY_0 => false,
            NOISYR::NOISY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOISYR {
        match value {
            false => NOISYR::NOISY_0,
            true => NOISYR::NOISY_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOISY_0`"]
    #[inline]
    pub fn is_noisy_0(&self) -> bool {
        *self == NOISYR::NOISY_0
    }
    #[doc = "Checks if the value of the field is `NOISY_1`"]
    #[inline]
    pub fn is_noisy_1(&self) -> bool {
        *self == NOISYR::NOISY_1
    }
}
#[doc = r" Proxy"]
pub struct _R0T0W<'a> {
    w: &'a mut W,
}
impl<'a> _R0T0W<'a> {
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
pub struct _R1T1W<'a> {
    w: &'a mut W,
}
impl<'a> _R1T1W<'a> {
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
pub struct _R2T2W<'a> {
    w: &'a mut W,
}
impl<'a> _R2T2W<'a> {
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
pub struct _R3T3W<'a> {
    w: &'a mut W,
}
impl<'a> _R3T3W<'a> {
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
pub struct _R4T4W<'a> {
    w: &'a mut W,
}
impl<'a> _R4T4W<'a> {
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
#[doc = r" Proxy"]
pub struct _R5T5W<'a> {
    w: &'a mut W,
}
impl<'a> _R5T5W<'a> {
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
pub struct _R6T6W<'a> {
    w: &'a mut W,
}
impl<'a> _R6T6W<'a> {
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
pub struct _R7T7W<'a> {
    w: &'a mut W,
}
impl<'a> _R7T7W<'a> {
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
pub struct _R8T8W<'a> {
    w: &'a mut W,
}
impl<'a> _R8T8W<'a> {
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
pub struct _R9T9W<'a> {
    w: &'a mut W,
}
impl<'a> _R9T9W<'a> {
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
#[doc = "Values that can be written to the field `FRETSC`"]
pub enum FRETSCW {
    #[doc = "The dataword was received without a frame error on read, or transmit a normal character on write."]
    FRETSC_0,
    #[doc = "The dataword was received with a frame error, or transmit an idle or break character on transmit."]
    FRETSC_1,
}
impl FRETSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRETSCW::FRETSC_0 => false,
            FRETSCW::FRETSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRETSCW<'a> {
    w: &'a mut W,
}
impl<'a> _FRETSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRETSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The dataword was received without a frame error on read, or transmit a normal character on write."]
    #[inline]
    pub fn fretsc_0(self) -> &'a mut W {
        self.variant(FRETSCW::FRETSC_0)
    }
    #[doc = "The dataword was received with a frame error, or transmit an idle or break character on transmit."]
    #[inline]
    pub fn fretsc_1(self) -> &'a mut W {
        self.variant(FRETSCW::FRETSC_1)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - R0T0"]
    #[inline]
    pub fn r0t0(&self) -> R0T0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R0T0R { bits }
    }
    #[doc = "Bit 1 - R1T1"]
    #[inline]
    pub fn r1t1(&self) -> R1T1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R1T1R { bits }
    }
    #[doc = "Bit 2 - R2T2"]
    #[inline]
    pub fn r2t2(&self) -> R2T2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R2T2R { bits }
    }
    #[doc = "Bit 3 - R3T3"]
    #[inline]
    pub fn r3t3(&self) -> R3T3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R3T3R { bits }
    }
    #[doc = "Bit 4 - R4T4"]
    #[inline]
    pub fn r4t4(&self) -> R4T4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R4T4R { bits }
    }
    #[doc = "Bit 5 - R5T5"]
    #[inline]
    pub fn r5t5(&self) -> R5T5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R5T5R { bits }
    }
    #[doc = "Bit 6 - R6T6"]
    #[inline]
    pub fn r6t6(&self) -> R6T6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R6T6R { bits }
    }
    #[doc = "Bit 7 - R7T7"]
    #[inline]
    pub fn r7t7(&self) -> R7T7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R7T7R { bits }
    }
    #[doc = "Bit 8 - R8T8"]
    #[inline]
    pub fn r8t8(&self) -> R8T8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R8T8R { bits }
    }
    #[doc = "Bit 9 - R9T9"]
    #[inline]
    pub fn r9t9(&self) -> R9T9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R9T9R { bits }
    }
    #[doc = "Bit 11 - Idle Line"]
    #[inline]
    pub fn idline(&self) -> IDLINER {
        IDLINER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Receive Buffer Empty"]
    #[inline]
    pub fn rxempt(&self) -> RXEMPTR {
        RXEMPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline]
    pub fn fretsc(&self) -> FRETSCR {
        FRETSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - PARITYE"]
    #[inline]
    pub fn paritye(&self) -> PARITYER {
        PARITYER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - NOISY"]
    #[inline]
    pub fn noisy(&self) -> NOISYR {
        NOISYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4096 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - R0T0"]
    #[inline]
    pub fn r0t0(&mut self) -> _R0T0W {
        _R0T0W { w: self }
    }
    #[doc = "Bit 1 - R1T1"]
    #[inline]
    pub fn r1t1(&mut self) -> _R1T1W {
        _R1T1W { w: self }
    }
    #[doc = "Bit 2 - R2T2"]
    #[inline]
    pub fn r2t2(&mut self) -> _R2T2W {
        _R2T2W { w: self }
    }
    #[doc = "Bit 3 - R3T3"]
    #[inline]
    pub fn r3t3(&mut self) -> _R3T3W {
        _R3T3W { w: self }
    }
    #[doc = "Bit 4 - R4T4"]
    #[inline]
    pub fn r4t4(&mut self) -> _R4T4W {
        _R4T4W { w: self }
    }
    #[doc = "Bit 5 - R5T5"]
    #[inline]
    pub fn r5t5(&mut self) -> _R5T5W {
        _R5T5W { w: self }
    }
    #[doc = "Bit 6 - R6T6"]
    #[inline]
    pub fn r6t6(&mut self) -> _R6T6W {
        _R6T6W { w: self }
    }
    #[doc = "Bit 7 - R7T7"]
    #[inline]
    pub fn r7t7(&mut self) -> _R7T7W {
        _R7T7W { w: self }
    }
    #[doc = "Bit 8 - R8T8"]
    #[inline]
    pub fn r8t8(&mut self) -> _R8T8W {
        _R8T8W { w: self }
    }
    #[doc = "Bit 9 - R9T9"]
    #[inline]
    pub fn r9t9(&mut self) -> _R9T9W {
        _R9T9W { w: self }
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline]
    pub fn fretsc(&mut self) -> _FRETSCW {
        _FRETSCW { w: self }
    }
}
