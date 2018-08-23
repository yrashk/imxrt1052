#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ESR1 {
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
#[doc = "Possible values of the field `WAKINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKINTR {
    #[doc = "No such occurrence"]
    WAKINT_0,
    #[doc = "Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode"]
    WAKINT_1,
}
impl WAKINTR {
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
            WAKINTR::WAKINT_0 => false,
            WAKINTR::WAKINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKINTR {
        match value {
            false => WAKINTR::WAKINT_0,
            true => WAKINTR::WAKINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKINT_0`"]
    #[inline]
    pub fn is_wakint_0(&self) -> bool {
        *self == WAKINTR::WAKINT_0
    }
    #[doc = "Checks if the value of the field is `WAKINT_1`"]
    #[inline]
    pub fn is_wakint_1(&self) -> bool {
        *self == WAKINTR::WAKINT_1
    }
}
#[doc = "Possible values of the field `ERRINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINTR {
    #[doc = "No such occurrence"]
    ERRINT_0,
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register"]
    ERRINT_1,
}
impl ERRINTR {
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
            ERRINTR::ERRINT_0 => false,
            ERRINTR::ERRINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRINTR {
        match value {
            false => ERRINTR::ERRINT_0,
            true => ERRINTR::ERRINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRINT_0`"]
    #[inline]
    pub fn is_errint_0(&self) -> bool {
        *self == ERRINTR::ERRINT_0
    }
    #[doc = "Checks if the value of the field is `ERRINT_1`"]
    #[inline]
    pub fn is_errint_1(&self) -> bool {
        *self == ERRINTR::ERRINT_1
    }
}
#[doc = "Possible values of the field `BOFFINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFINTR {
    #[doc = "No such occurrence"]
    BOFFINT_0,
    #[doc = "FLEXCAN module entered 'Bus Off' state"]
    BOFFINT_1,
}
impl BOFFINTR {
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
            BOFFINTR::BOFFINT_0 => false,
            BOFFINTR::BOFFINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFINTR {
        match value {
            false => BOFFINTR::BOFFINT_0,
            true => BOFFINTR::BOFFINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFINT_0`"]
    #[inline]
    pub fn is_boffint_0(&self) -> bool {
        *self == BOFFINTR::BOFFINT_0
    }
    #[doc = "Checks if the value of the field is `BOFFINT_1`"]
    #[inline]
    pub fn is_boffint_1(&self) -> bool {
        *self == BOFFINTR::BOFFINT_1
    }
}
#[doc = "Possible values of the field `RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXR {
    #[doc = "FLEXCAN is receiving a message"]
    RX_0,
    #[doc = "FLEXCAN is transmitting a message"]
    RX_1,
}
impl RXR {
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
            RXR::RX_0 => false,
            RXR::RX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXR {
        match value {
            false => RXR::RX_0,
            true => RXR::RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_0`"]
    #[inline]
    pub fn is_rx_0(&self) -> bool {
        *self == RXR::RX_0
    }
    #[doc = "Checks if the value of the field is `RX_1`"]
    #[inline]
    pub fn is_rx_1(&self) -> bool {
        *self == RXR::RX_1
    }
}
#[doc = "Possible values of the field `FLTCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTCONFR {
    #[doc = "Error Active"]
    FLTCONF_0,
    #[doc = "Error Passive"]
    FLTCONF_1,
    #[doc = "Bus off"]
    FLTCONF_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLTCONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLTCONFR::FLTCONF_0 => 0,
            FLTCONFR::FLTCONF_1 => 1,
            FLTCONFR::FLTCONF_2 => 2,
            FLTCONFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLTCONFR {
        match value {
            0 => FLTCONFR::FLTCONF_0,
            1 => FLTCONFR::FLTCONF_1,
            2 => FLTCONFR::FLTCONF_2,
            i => FLTCONFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLTCONF_0`"]
    #[inline]
    pub fn is_fltconf_0(&self) -> bool {
        *self == FLTCONFR::FLTCONF_0
    }
    #[doc = "Checks if the value of the field is `FLTCONF_1`"]
    #[inline]
    pub fn is_fltconf_1(&self) -> bool {
        *self == FLTCONFR::FLTCONF_1
    }
    #[doc = "Checks if the value of the field is `FLTCONF_2`"]
    #[inline]
    pub fn is_fltconf_2(&self) -> bool {
        *self == FLTCONFR::FLTCONF_2
    }
}
#[doc = "Possible values of the field `TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXR {
    #[doc = "FLEXCAN is receiving a message"]
    TX_0,
    #[doc = "FLEXCAN is transmitting a message"]
    TX_1,
}
impl TXR {
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
            TXR::TX_0 => false,
            TXR::TX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXR {
        match value {
            false => TXR::TX_0,
            true => TXR::TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `TX_0`"]
    #[inline]
    pub fn is_tx_0(&self) -> bool {
        *self == TXR::TX_0
    }
    #[doc = "Checks if the value of the field is `TX_1`"]
    #[inline]
    pub fn is_tx_1(&self) -> bool {
        *self == TXR::TX_1
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "No such occurrence"]
    IDLE_0,
    #[doc = "CAN bus is now IDLE"]
    IDLE_1,
}
impl IDLER {
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
            IDLER::IDLE_0 => false,
            IDLER::IDLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLER {
        match value {
            false => IDLER::IDLE_0,
            true => IDLER::IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_0`"]
    #[inline]
    pub fn is_idle_0(&self) -> bool {
        *self == IDLER::IDLE_0
    }
    #[doc = "Checks if the value of the field is `IDLE_1`"]
    #[inline]
    pub fn is_idle_1(&self) -> bool {
        *self == IDLER::IDLE_1
    }
}
#[doc = "Possible values of the field `RXWRN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXWRNR {
    #[doc = "No such occurrence"]
    RXWRN_0,
    #[doc = "Rx_Err_Counter >= 96"]
    RXWRN_1,
}
impl RXWRNR {
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
            RXWRNR::RXWRN_0 => false,
            RXWRNR::RXWRN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXWRNR {
        match value {
            false => RXWRNR::RXWRN_0,
            true => RXWRNR::RXWRN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXWRN_0`"]
    #[inline]
    pub fn is_rxwrn_0(&self) -> bool {
        *self == RXWRNR::RXWRN_0
    }
    #[doc = "Checks if the value of the field is `RXWRN_1`"]
    #[inline]
    pub fn is_rxwrn_1(&self) -> bool {
        *self == RXWRNR::RXWRN_1
    }
}
#[doc = "Possible values of the field `TXWRN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXWRNR {
    #[doc = "No such occurrence"]
    TXWRN_0,
    #[doc = "TX_Err_Counter >= 96"]
    TXWRN_1,
}
impl TXWRNR {
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
            TXWRNR::TXWRN_0 => false,
            TXWRNR::TXWRN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXWRNR {
        match value {
            false => TXWRNR::TXWRN_0,
            true => TXWRNR::TXWRN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXWRN_0`"]
    #[inline]
    pub fn is_txwrn_0(&self) -> bool {
        *self == TXWRNR::TXWRN_0
    }
    #[doc = "Checks if the value of the field is `TXWRN_1`"]
    #[inline]
    pub fn is_txwrn_1(&self) -> bool {
        *self == TXWRNR::TXWRN_1
    }
}
#[doc = "Possible values of the field `STFERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STFERRR {
    #[doc = "No such occurrence."]
    STFERR_0,
    #[doc = "A Stuffing Error occurred since last read of this register."]
    STFERR_1,
}
impl STFERRR {
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
            STFERRR::STFERR_0 => false,
            STFERRR::STFERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STFERRR {
        match value {
            false => STFERRR::STFERR_0,
            true => STFERRR::STFERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `STFERR_0`"]
    #[inline]
    pub fn is_stferr_0(&self) -> bool {
        *self == STFERRR::STFERR_0
    }
    #[doc = "Checks if the value of the field is `STFERR_1`"]
    #[inline]
    pub fn is_stferr_1(&self) -> bool {
        *self == STFERRR::STFERR_1
    }
}
#[doc = "Possible values of the field `FRMERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERRR {
    #[doc = "No such occurrence"]
    FRMERR_0,
    #[doc = "A Form Error occurred since last read of this register"]
    FRMERR_1,
}
impl FRMERRR {
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
            FRMERRR::FRMERR_0 => false,
            FRMERRR::FRMERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRMERRR {
        match value {
            false => FRMERRR::FRMERR_0,
            true => FRMERRR::FRMERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRMERR_0`"]
    #[inline]
    pub fn is_frmerr_0(&self) -> bool {
        *self == FRMERRR::FRMERR_0
    }
    #[doc = "Checks if the value of the field is `FRMERR_1`"]
    #[inline]
    pub fn is_frmerr_1(&self) -> bool {
        *self == FRMERRR::FRMERR_1
    }
}
#[doc = "Possible values of the field `CRCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERRR {
    #[doc = "No such occurrence"]
    CRCERR_0,
    #[doc = "A CRC error occurred since last read of this register."]
    CRCERR_1,
}
impl CRCERRR {
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
            CRCERRR::CRCERR_0 => false,
            CRCERRR::CRCERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCERRR {
        match value {
            false => CRCERRR::CRCERR_0,
            true => CRCERRR::CRCERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRCERR_0`"]
    #[inline]
    pub fn is_crcerr_0(&self) -> bool {
        *self == CRCERRR::CRCERR_0
    }
    #[doc = "Checks if the value of the field is `CRCERR_1`"]
    #[inline]
    pub fn is_crcerr_1(&self) -> bool {
        *self == CRCERRR::CRCERR_1
    }
}
#[doc = "Possible values of the field `ACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKERRR {
    #[doc = "No such occurrence"]
    ACKERR_0,
    #[doc = "An ACK error occurred since last read of this register"]
    ACKERR_1,
}
impl ACKERRR {
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
            ACKERRR::ACKERR_0 => false,
            ACKERRR::ACKERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKERRR {
        match value {
            false => ACKERRR::ACKERR_0,
            true => ACKERRR::ACKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACKERR_0`"]
    #[inline]
    pub fn is_ackerr_0(&self) -> bool {
        *self == ACKERRR::ACKERR_0
    }
    #[doc = "Checks if the value of the field is `ACKERR_1`"]
    #[inline]
    pub fn is_ackerr_1(&self) -> bool {
        *self == ACKERRR::ACKERR_1
    }
}
#[doc = "Possible values of the field `BIT0ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT0ERRR {
    #[doc = "No such occurrence"]
    BIT0ERR_0,
    #[doc = "At least one bit sent as dominant is received as recessive"]
    BIT0ERR_1,
}
impl BIT0ERRR {
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
            BIT0ERRR::BIT0ERR_0 => false,
            BIT0ERRR::BIT0ERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT0ERRR {
        match value {
            false => BIT0ERRR::BIT0ERR_0,
            true => BIT0ERRR::BIT0ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_0`"]
    #[inline]
    pub fn is_bit0err_0(&self) -> bool {
        *self == BIT0ERRR::BIT0ERR_0
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_1`"]
    #[inline]
    pub fn is_bit0err_1(&self) -> bool {
        *self == BIT0ERRR::BIT0ERR_1
    }
}
#[doc = "Possible values of the field `BIT1ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT1ERRR {
    #[doc = "No such occurrence"]
    BIT1ERR_0,
    #[doc = "At least one bit sent as recessive is received as dominant"]
    BIT1ERR_1,
}
impl BIT1ERRR {
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
            BIT1ERRR::BIT1ERR_0 => false,
            BIT1ERRR::BIT1ERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT1ERRR {
        match value {
            false => BIT1ERRR::BIT1ERR_0,
            true => BIT1ERRR::BIT1ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_0`"]
    #[inline]
    pub fn is_bit1err_0(&self) -> bool {
        *self == BIT1ERRR::BIT1ERR_0
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_1`"]
    #[inline]
    pub fn is_bit1err_1(&self) -> bool {
        *self == BIT1ERRR::BIT1ERR_1
    }
}
#[doc = "Possible values of the field `RWRNINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNINTR {
    #[doc = "No such occurrence"]
    RWRNINT_0,
    #[doc = "The Rx error counter transition from < 96 to >= 96"]
    RWRNINT_1,
}
impl RWRNINTR {
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
            RWRNINTR::RWRNINT_0 => false,
            RWRNINTR::RWRNINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWRNINTR {
        match value {
            false => RWRNINTR::RWRNINT_0,
            true => RWRNINTR::RWRNINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWRNINT_0`"]
    #[inline]
    pub fn is_rwrnint_0(&self) -> bool {
        *self == RWRNINTR::RWRNINT_0
    }
    #[doc = "Checks if the value of the field is `RWRNINT_1`"]
    #[inline]
    pub fn is_rwrnint_1(&self) -> bool {
        *self == RWRNINTR::RWRNINT_1
    }
}
#[doc = "Possible values of the field `TWRNINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNINTR {
    #[doc = "No such occurrence"]
    TWRNINT_0,
    #[doc = "The Tx error counter transition from < 96 to >= 96"]
    TWRNINT_1,
}
impl TWRNINTR {
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
            TWRNINTR::TWRNINT_0 => false,
            TWRNINTR::TWRNINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWRNINTR {
        match value {
            false => TWRNINTR::TWRNINT_0,
            true => TWRNINTR::TWRNINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TWRNINT_0`"]
    #[inline]
    pub fn is_twrnint_0(&self) -> bool {
        *self == TWRNINTR::TWRNINT_0
    }
    #[doc = "Checks if the value of the field is `TWRNINT_1`"]
    #[inline]
    pub fn is_twrnint_1(&self) -> bool {
        *self == TWRNINTR::TWRNINT_1
    }
}
#[doc = "Possible values of the field `SYNCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCHR {
    #[doc = "FlexCAN is not synchronized to the CAN bus"]
    SYNCH_0,
    #[doc = "FlexCAN is synchronized to the CAN bus"]
    SYNCH_1,
}
impl SYNCHR {
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
            SYNCHR::SYNCH_0 => false,
            SYNCHR::SYNCH_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCHR {
        match value {
            false => SYNCHR::SYNCH_0,
            true => SYNCHR::SYNCH_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCH_0`"]
    #[inline]
    pub fn is_synch_0(&self) -> bool {
        *self == SYNCHR::SYNCH_0
    }
    #[doc = "Checks if the value of the field is `SYNCH_1`"]
    #[inline]
    pub fn is_synch_1(&self) -> bool {
        *self == SYNCHR::SYNCH_1
    }
}
#[doc = "Values that can be written to the field `WAKINT`"]
pub enum WAKINTW {
    #[doc = "No such occurrence"]
    WAKINT_0,
    #[doc = "Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode"]
    WAKINT_1,
}
impl WAKINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKINTW::WAKINT_0 => false,
            WAKINTW::WAKINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn wakint_0(self) -> &'a mut W {
        self.variant(WAKINTW::WAKINT_0)
    }
    #[doc = "Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode"]
    #[inline]
    pub fn wakint_1(self) -> &'a mut W {
        self.variant(WAKINTW::WAKINT_1)
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
#[doc = "Values that can be written to the field `ERRINT`"]
pub enum ERRINTW {
    #[doc = "No such occurrence"]
    ERRINT_0,
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register"]
    ERRINT_1,
}
impl ERRINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRINTW::ERRINT_0 => false,
            ERRINTW::ERRINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRINTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn errint_0(self) -> &'a mut W {
        self.variant(ERRINTW::ERRINT_0)
    }
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register"]
    #[inline]
    pub fn errint_1(self) -> &'a mut W {
        self.variant(ERRINTW::ERRINT_1)
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
#[doc = "Values that can be written to the field `BOFFINT`"]
pub enum BOFFINTW {
    #[doc = "No such occurrence"]
    BOFFINT_0,
    #[doc = "FLEXCAN module entered 'Bus Off' state"]
    BOFFINT_1,
}
impl BOFFINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFINTW::BOFFINT_0 => false,
            BOFFINTW::BOFFINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFINTW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn boffint_0(self) -> &'a mut W {
        self.variant(BOFFINTW::BOFFINT_0)
    }
    #[doc = "FLEXCAN module entered 'Bus Off' state"]
    #[inline]
    pub fn boffint_1(self) -> &'a mut W {
        self.variant(BOFFINTW::BOFFINT_1)
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
#[doc = "Values that can be written to the field `RWRNINT`"]
pub enum RWRNINTW {
    #[doc = "No such occurrence"]
    RWRNINT_0,
    #[doc = "The Rx error counter transition from < 96 to >= 96"]
    RWRNINT_1,
}
impl RWRNINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWRNINTW::RWRNINT_0 => false,
            RWRNINTW::RWRNINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWRNINTW<'a> {
    w: &'a mut W,
}
impl<'a> _RWRNINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWRNINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn rwrnint_0(self) -> &'a mut W {
        self.variant(RWRNINTW::RWRNINT_0)
    }
    #[doc = "The Rx error counter transition from < 96 to >= 96"]
    #[inline]
    pub fn rwrnint_1(self) -> &'a mut W {
        self.variant(RWRNINTW::RWRNINT_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TWRNINT`"]
pub enum TWRNINTW {
    #[doc = "No such occurrence"]
    TWRNINT_0,
    #[doc = "The Tx error counter transition from < 96 to >= 96"]
    TWRNINT_1,
}
impl TWRNINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWRNINTW::TWRNINT_0 => false,
            TWRNINTW::TWRNINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWRNINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TWRNINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWRNINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn twrnint_0(self) -> &'a mut W {
        self.variant(TWRNINTW::TWRNINT_0)
    }
    #[doc = "The Tx error counter transition from < 96 to >= 96"]
    #[inline]
    pub fn twrnint_1(self) -> &'a mut W {
        self.variant(TWRNINTW::TWRNINT_1)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the ARM"]
    #[inline]
    pub fn wakint(&self) -> WAKINTR {
        WAKINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
    #[inline]
    pub fn errint(&self) -> ERRINTR {
        ERRINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - This bit is set when FLEXCAN enters 'Bus Off' state"]
    #[inline]
    pub fn boffint(&self) -> BOFFINTR {
        BOFFINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - This bit indicates if FlexCAN is receiving a message. Refer to ."]
    #[inline]
    pub fn rx(&self) -> RXR {
        RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate \"Error Passive\""]
    #[inline]
    pub fn fltconf(&self) -> FLTCONFR {
        FLTCONFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - This bit indicates if FLEXCAN is transmitting a message.Refer to ."]
    #[inline]
    pub fn tx(&self) -> TXR {
        TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - This bit indicates when CAN bus is in IDLE state.Refer to ."]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - This bit indicates when repetitive errors are occurring during message reception."]
    #[inline]
    pub fn rxwrn(&self) -> RXWRNR {
        RXWRNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - This bit indicates when repetitive errors are occurring during message transmission."]
    #[inline]
    pub fn txwrn(&self) -> TXWRNR {
        TXWRNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - This bit indicates that a Stuffing Error has been detected."]
    #[inline]
    pub fn stferr(&self) -> STFERRR {
        STFERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - This bit indicates that a Form Error has been detected by the receiver node, i"]
    #[inline]
    pub fn frmerr(&self) -> FRMERRR {
        FRMERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - This bit indicates that a CRC Error has been detected by the receiver node, i"]
    #[inline]
    pub fn crcerr(&self) -> CRCERRR {
        CRCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - This bit indicates that an Acknowledge Error has been detected by the transmitter node, i"]
    #[inline]
    pub fn ackerr(&self) -> ACKERRR {
        ACKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[inline]
    pub fn bit0err(&self) -> BIT0ERRR {
        BIT0ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[inline]
    pub fn bit1err(&self) -> BIT1ERRR {
        BIT1ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
    #[inline]
    pub fn rwrnint(&self) -> RWRNINTR {
        RWRNINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
    #[inline]
    pub fn twrnint(&self) -> TWRNINTR {
        TWRNINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process"]
    #[inline]
    pub fn synch(&self) -> SYNCHR {
        SYNCHR::_from({
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
    #[doc = "Bit 0 - When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the ARM"]
    #[inline]
    pub fn wakint(&mut self) -> _WAKINTW {
        _WAKINTW { w: self }
    }
    #[doc = "Bit 1 - This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
    #[inline]
    pub fn errint(&mut self) -> _ERRINTW {
        _ERRINTW { w: self }
    }
    #[doc = "Bit 2 - This bit is set when FLEXCAN enters 'Bus Off' state"]
    #[inline]
    pub fn boffint(&mut self) -> _BOFFINTW {
        _BOFFINTW { w: self }
    }
    #[doc = "Bit 16 - If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
    #[inline]
    pub fn rwrnint(&mut self) -> _RWRNINTW {
        _RWRNINTW { w: self }
    }
    #[doc = "Bit 17 - If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
    #[inline]
    pub fn twrnint(&mut self) -> _TWRNINTW {
        _TWRNINTW { w: self }
    }
}
