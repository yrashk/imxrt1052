#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODIR {
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
#[doc = "Possible values of the field `TXCTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSER {
    #[doc = "CTS has no effect on the transmitter."]
    TXCTSE_0,
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    TXCTSE_1,
}
impl TXCTSER {
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
            TXCTSER::TXCTSE_0 => false,
            TXCTSER::TXCTSE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCTSER {
        match value {
            false => TXCTSER::TXCTSE_0,
            true => TXCTSER::TXCTSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXCTSE_0`"]
    #[inline]
    pub fn is_txctse_0(&self) -> bool {
        *self == TXCTSER::TXCTSE_0
    }
    #[doc = "Checks if the value of the field is `TXCTSE_1`"]
    #[inline]
    pub fn is_txctse_1(&self) -> bool {
        *self == TXCTSER::TXCTSE_1
    }
}
#[doc = "Possible values of the field `TXRTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSER {
    #[doc = "The transmitter has no effect on RTS."]
    TXRTSE_0,
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit."]
    TXRTSE_1,
}
impl TXRTSER {
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
            TXRTSER::TXRTSE_0 => false,
            TXRTSER::TXRTSE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRTSER {
        match value {
            false => TXRTSER::TXRTSE_0,
            true => TXRTSER::TXRTSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXRTSE_0`"]
    #[inline]
    pub fn is_txrtse_0(&self) -> bool {
        *self == TXRTSER::TXRTSE_0
    }
    #[doc = "Checks if the value of the field is `TXRTSE_1`"]
    #[inline]
    pub fn is_txrtse_1(&self) -> bool {
        *self == TXRTSER::TXRTSE_1
    }
}
#[doc = "Possible values of the field `TXRTSPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSPOLR {
    #[doc = "Transmitter RTS is active low."]
    TXRTSPOL_0,
    #[doc = "Transmitter RTS is active high."]
    TXRTSPOL_1,
}
impl TXRTSPOLR {
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
            TXRTSPOLR::TXRTSPOL_0 => false,
            TXRTSPOLR::TXRTSPOL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRTSPOLR {
        match value {
            false => TXRTSPOLR::TXRTSPOL_0,
            true => TXRTSPOLR::TXRTSPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXRTSPOL_0`"]
    #[inline]
    pub fn is_txrtspol_0(&self) -> bool {
        *self == TXRTSPOLR::TXRTSPOL_0
    }
    #[doc = "Checks if the value of the field is `TXRTSPOL_1`"]
    #[inline]
    pub fn is_txrtspol_1(&self) -> bool {
        *self == TXRTSPOLR::TXRTSPOL_1
    }
}
#[doc = "Possible values of the field `RXRTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRTSER {
    #[doc = "The receiver has no effect on RTS."]
    RXRTSE_0,
    #[doc = "no description available"]
    RXRTSE_1,
}
impl RXRTSER {
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
            RXRTSER::RXRTSE_0 => false,
            RXRTSER::RXRTSE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXRTSER {
        match value {
            false => RXRTSER::RXRTSE_0,
            true => RXRTSER::RXRTSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXRTSE_0`"]
    #[inline]
    pub fn is_rxrtse_0(&self) -> bool {
        *self == RXRTSER::RXRTSE_0
    }
    #[doc = "Checks if the value of the field is `RXRTSE_1`"]
    #[inline]
    pub fn is_rxrtse_1(&self) -> bool {
        *self == RXRTSER::RXRTSE_1
    }
}
#[doc = "Possible values of the field `TXCTSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSCR {
    #[doc = "CTS input is sampled at the start of each character."]
    TXCTSC_0,
    #[doc = "CTS input is sampled when the transmitter is idle."]
    TXCTSC_1,
}
impl TXCTSCR {
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
            TXCTSCR::TXCTSC_0 => false,
            TXCTSCR::TXCTSC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCTSCR {
        match value {
            false => TXCTSCR::TXCTSC_0,
            true => TXCTSCR::TXCTSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXCTSC_0`"]
    #[inline]
    pub fn is_txctsc_0(&self) -> bool {
        *self == TXCTSCR::TXCTSC_0
    }
    #[doc = "Checks if the value of the field is `TXCTSC_1`"]
    #[inline]
    pub fn is_txctsc_1(&self) -> bool {
        *self == TXCTSCR::TXCTSC_1
    }
}
#[doc = "Possible values of the field `TXCTSSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSSRCR {
    #[doc = "CTS input is the CTS_B pin."]
    TXCTSSRC_0,
    #[doc = "CTS input is the inverted Receiver Match result."]
    TXCTSSRC_1,
}
impl TXCTSSRCR {
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
            TXCTSSRCR::TXCTSSRC_0 => false,
            TXCTSSRCR::TXCTSSRC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCTSSRCR {
        match value {
            false => TXCTSSRCR::TXCTSSRC_0,
            true => TXCTSSRCR::TXCTSSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXCTSSRC_0`"]
    #[inline]
    pub fn is_txctssrc_0(&self) -> bool {
        *self == TXCTSSRCR::TXCTSSRC_0
    }
    #[doc = "Checks if the value of the field is `TXCTSSRC_1`"]
    #[inline]
    pub fn is_txctssrc_1(&self) -> bool {
        *self == TXCTSSRCR::TXCTSSRC_1
    }
}
#[doc = r" Value of the field"]
pub struct RTSWATERR {
    bits: u8,
}
impl RTSWATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TNP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNPR {
    #[doc = "1/OSR."]
    TNP_0,
    #[doc = "2/OSR."]
    TNP_1,
    #[doc = "3/OSR."]
    TNP_2,
    #[doc = "4/OSR."]
    TNP_3,
}
impl TNPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TNPR::TNP_0 => 0,
            TNPR::TNP_1 => 1,
            TNPR::TNP_2 => 2,
            TNPR::TNP_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TNPR {
        match value {
            0 => TNPR::TNP_0,
            1 => TNPR::TNP_1,
            2 => TNPR::TNP_2,
            3 => TNPR::TNP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TNP_0`"]
    #[inline]
    pub fn is_tnp_0(&self) -> bool {
        *self == TNPR::TNP_0
    }
    #[doc = "Checks if the value of the field is `TNP_1`"]
    #[inline]
    pub fn is_tnp_1(&self) -> bool {
        *self == TNPR::TNP_1
    }
    #[doc = "Checks if the value of the field is `TNP_2`"]
    #[inline]
    pub fn is_tnp_2(&self) -> bool {
        *self == TNPR::TNP_2
    }
    #[doc = "Checks if the value of the field is `TNP_3`"]
    #[inline]
    pub fn is_tnp_3(&self) -> bool {
        *self == TNPR::TNP_3
    }
}
#[doc = "Possible values of the field `IREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRENR {
    #[doc = "IR disabled."]
    IREN_0,
    #[doc = "IR enabled."]
    IREN_1,
}
impl IRENR {
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
            IRENR::IREN_0 => false,
            IRENR::IREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRENR {
        match value {
            false => IRENR::IREN_0,
            true => IRENR::IREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IREN_0`"]
    #[inline]
    pub fn is_iren_0(&self) -> bool {
        *self == IRENR::IREN_0
    }
    #[doc = "Checks if the value of the field is `IREN_1`"]
    #[inline]
    pub fn is_iren_1(&self) -> bool {
        *self == IRENR::IREN_1
    }
}
#[doc = "Values that can be written to the field `TXCTSE`"]
pub enum TXCTSEW {
    #[doc = "CTS has no effect on the transmitter."]
    TXCTSE_0,
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    TXCTSE_1,
}
impl TXCTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCTSEW::TXCTSE_0 => false,
            TXCTSEW::TXCTSE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS has no effect on the transmitter."]
    #[inline]
    pub fn txctse_0(self) -> &'a mut W {
        self.variant(TXCTSEW::TXCTSE_0)
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline]
    pub fn txctse_1(self) -> &'a mut W {
        self.variant(TXCTSEW::TXCTSE_1)
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
#[doc = "Values that can be written to the field `TXRTSE`"]
pub enum TXRTSEW {
    #[doc = "The transmitter has no effect on RTS."]
    TXRTSE_0,
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit."]
    TXRTSE_1,
}
impl TXRTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRTSEW::TXRTSE_0 => false,
            TXRTSEW::TXRTSE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmitter has no effect on RTS."]
    #[inline]
    pub fn txrtse_0(self) -> &'a mut W {
        self.variant(TXRTSEW::TXRTSE_0)
    }
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit."]
    #[inline]
    pub fn txrtse_1(self) -> &'a mut W {
        self.variant(TXRTSEW::TXRTSE_1)
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
#[doc = "Values that can be written to the field `TXRTSPOL`"]
pub enum TXRTSPOLW {
    #[doc = "Transmitter RTS is active low."]
    TXRTSPOL_0,
    #[doc = "Transmitter RTS is active high."]
    TXRTSPOL_1,
}
impl TXRTSPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRTSPOLW::TXRTSPOL_0 => false,
            TXRTSPOLW::TXRTSPOL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRTSPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRTSPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRTSPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitter RTS is active low."]
    #[inline]
    pub fn txrtspol_0(self) -> &'a mut W {
        self.variant(TXRTSPOLW::TXRTSPOL_0)
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline]
    pub fn txrtspol_1(self) -> &'a mut W {
        self.variant(TXRTSPOLW::TXRTSPOL_1)
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
#[doc = "Values that can be written to the field `RXRTSE`"]
pub enum RXRTSEW {
    #[doc = "The receiver has no effect on RTS."]
    RXRTSE_0,
    #[doc = "no description available"]
    RXRTSE_1,
}
impl RXRTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXRTSEW::RXRTSE_0 => false,
            RXRTSEW::RXRTSE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXRTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXRTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receiver has no effect on RTS."]
    #[inline]
    pub fn rxrtse_0(self) -> &'a mut W {
        self.variant(RXRTSEW::RXRTSE_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn rxrtse_1(self) -> &'a mut W {
        self.variant(RXRTSEW::RXRTSE_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXCTSC`"]
pub enum TXCTSCW {
    #[doc = "CTS input is sampled at the start of each character."]
    TXCTSC_0,
    #[doc = "CTS input is sampled when the transmitter is idle."]
    TXCTSC_1,
}
impl TXCTSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCTSCW::TXCTSC_0 => false,
            TXCTSCW::TXCTSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCTSCW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCTSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCTSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS input is sampled at the start of each character."]
    #[inline]
    pub fn txctsc_0(self) -> &'a mut W {
        self.variant(TXCTSCW::TXCTSC_0)
    }
    #[doc = "CTS input is sampled when the transmitter is idle."]
    #[inline]
    pub fn txctsc_1(self) -> &'a mut W {
        self.variant(TXCTSCW::TXCTSC_1)
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
#[doc = "Values that can be written to the field `TXCTSSRC`"]
pub enum TXCTSSRCW {
    #[doc = "CTS input is the CTS_B pin."]
    TXCTSSRC_0,
    #[doc = "CTS input is the inverted Receiver Match result."]
    TXCTSSRC_1,
}
impl TXCTSSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCTSSRCW::TXCTSSRC_0 => false,
            TXCTSSRCW::TXCTSSRC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCTSSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCTSSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCTSSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS input is the CTS_B pin."]
    #[inline]
    pub fn txctssrc_0(self) -> &'a mut W {
        self.variant(TXCTSSRCW::TXCTSSRC_0)
    }
    #[doc = "CTS input is the inverted Receiver Match result."]
    #[inline]
    pub fn txctssrc_1(self) -> &'a mut W {
        self.variant(TXCTSSRCW::TXCTSSRC_1)
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
pub struct _RTSWATERW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSWATERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TNP`"]
pub enum TNPW {
    #[doc = "1/OSR."]
    TNP_0,
    #[doc = "2/OSR."]
    TNP_1,
    #[doc = "3/OSR."]
    TNP_2,
    #[doc = "4/OSR."]
    TNP_3,
}
impl TNPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TNPW::TNP_0 => 0,
            TNPW::TNP_1 => 1,
            TNPW::TNP_2 => 2,
            TNPW::TNP_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNPW<'a> {
    w: &'a mut W,
}
impl<'a> _TNPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/OSR."]
    #[inline]
    pub fn tnp_0(self) -> &'a mut W {
        self.variant(TNPW::TNP_0)
    }
    #[doc = "2/OSR."]
    #[inline]
    pub fn tnp_1(self) -> &'a mut W {
        self.variant(TNPW::TNP_1)
    }
    #[doc = "3/OSR."]
    #[inline]
    pub fn tnp_2(self) -> &'a mut W {
        self.variant(TNPW::TNP_2)
    }
    #[doc = "4/OSR."]
    #[inline]
    pub fn tnp_3(self) -> &'a mut W {
        self.variant(TNPW::TNP_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IREN`"]
pub enum IRENW {
    #[doc = "IR disabled."]
    IREN_0,
    #[doc = "IR enabled."]
    IREN_1,
}
impl IRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRENW::IREN_0 => false,
            IRENW::IREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IR disabled."]
    #[inline]
    pub fn iren_0(self) -> &'a mut W {
        self.variant(IRENW::IREN_0)
    }
    #[doc = "IR enabled."]
    #[inline]
    pub fn iren_1(self) -> &'a mut W {
        self.variant(IRENW::IREN_1)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline]
    pub fn txctse(&self) -> TXCTSER {
        TXCTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline]
    pub fn txrtse(&self) -> TXRTSER {
        TXRTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline]
    pub fn txrtspol(&self) -> TXRTSPOLR {
        TXRTSPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline]
    pub fn rxrtse(&self) -> RXRTSER {
        RXRTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Transmit CTS Configuration"]
    #[inline]
    pub fn txctsc(&self) -> TXCTSCR {
        TXCTSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transmit CTS Source"]
    #[inline]
    pub fn txctssrc(&self) -> TXCTSSRCR {
        TXCTSSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Receive RTS Configuration"]
    #[inline]
    pub fn rtswater(&self) -> RTSWATERR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RTSWATERR { bits }
    }
    #[doc = "Bits 16:17 - Transmitter narrow pulse"]
    #[inline]
    pub fn tnp(&self) -> TNPR {
        TNPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Infrared enable"]
    #[inline]
    pub fn iren(&self) -> IRENR {
        IRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline]
    pub fn txctse(&mut self) -> _TXCTSEW {
        _TXCTSEW { w: self }
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline]
    pub fn txrtse(&mut self) -> _TXRTSEW {
        _TXRTSEW { w: self }
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline]
    pub fn txrtspol(&mut self) -> _TXRTSPOLW {
        _TXRTSPOLW { w: self }
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline]
    pub fn rxrtse(&mut self) -> _RXRTSEW {
        _RXRTSEW { w: self }
    }
    #[doc = "Bit 4 - Transmit CTS Configuration"]
    #[inline]
    pub fn txctsc(&mut self) -> _TXCTSCW {
        _TXCTSCW { w: self }
    }
    #[doc = "Bit 5 - Transmit CTS Source"]
    #[inline]
    pub fn txctssrc(&mut self) -> _TXCTSSRCW {
        _TXCTSSRCW { w: self }
    }
    #[doc = "Bits 8:9 - Receive RTS Configuration"]
    #[inline]
    pub fn rtswater(&mut self) -> _RTSWATERW {
        _RTSWATERW { w: self }
    }
    #[doc = "Bits 16:17 - Transmitter narrow pulse"]
    #[inline]
    pub fn tnp(&mut self) -> _TNPW {
        _TNPW { w: self }
    }
    #[doc = "Bit 18 - Infrared enable"]
    #[inline]
    pub fn iren(&mut self) -> _IRENW {
        _IRENW { w: self }
    }
}
