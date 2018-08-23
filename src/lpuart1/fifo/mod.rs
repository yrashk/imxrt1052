#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFO {
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
#[doc = "Possible values of the field `RXFIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOSIZER {
    #[doc = "Receive FIFO/Buffer depth = 1 dataword."]
    RXFIFOSIZE_0,
    #[doc = "Receive FIFO/Buffer depth = 4 datawords."]
    RXFIFOSIZE_1,
    #[doc = "Receive FIFO/Buffer depth = 8 datawords."]
    RXFIFOSIZE_2,
    #[doc = "Receive FIFO/Buffer depth = 16 datawords."]
    RXFIFOSIZE_3,
    #[doc = "Receive FIFO/Buffer depth = 32 datawords."]
    RXFIFOSIZE_4,
    #[doc = "Receive FIFO/Buffer depth = 64 datawords."]
    RXFIFOSIZE_5,
    #[doc = "Receive FIFO/Buffer depth = 128 datawords."]
    RXFIFOSIZE_6,
    #[doc = "Receive FIFO/Buffer depth = 256 datawords."]
    RXFIFOSIZE_7,
}
impl RXFIFOSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXFIFOSIZER::RXFIFOSIZE_0 => 0,
            RXFIFOSIZER::RXFIFOSIZE_1 => 1,
            RXFIFOSIZER::RXFIFOSIZE_2 => 2,
            RXFIFOSIZER::RXFIFOSIZE_3 => 3,
            RXFIFOSIZER::RXFIFOSIZE_4 => 4,
            RXFIFOSIZER::RXFIFOSIZE_5 => 5,
            RXFIFOSIZER::RXFIFOSIZE_6 => 6,
            RXFIFOSIZER::RXFIFOSIZE_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXFIFOSIZER {
        match value {
            0 => RXFIFOSIZER::RXFIFOSIZE_0,
            1 => RXFIFOSIZER::RXFIFOSIZE_1,
            2 => RXFIFOSIZER::RXFIFOSIZE_2,
            3 => RXFIFOSIZER::RXFIFOSIZE_3,
            4 => RXFIFOSIZER::RXFIFOSIZE_4,
            5 => RXFIFOSIZER::RXFIFOSIZE_5,
            6 => RXFIFOSIZER::RXFIFOSIZE_6,
            7 => RXFIFOSIZER::RXFIFOSIZE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_0`"]
    #[inline]
    pub fn is_rxfifosize_0(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_0
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_1`"]
    #[inline]
    pub fn is_rxfifosize_1(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_1
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_2`"]
    #[inline]
    pub fn is_rxfifosize_2(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_2
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_3`"]
    #[inline]
    pub fn is_rxfifosize_3(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_3
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_4`"]
    #[inline]
    pub fn is_rxfifosize_4(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_4
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_5`"]
    #[inline]
    pub fn is_rxfifosize_5(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_5
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_6`"]
    #[inline]
    pub fn is_rxfifosize_6(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_6
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_7`"]
    #[inline]
    pub fn is_rxfifosize_7(&self) -> bool {
        *self == RXFIFOSIZER::RXFIFOSIZE_7
    }
}
#[doc = "Possible values of the field `RXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFER {
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    RXFE_0,
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    RXFE_1,
}
impl RXFER {
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
            RXFER::RXFE_0 => false,
            RXFER::RXFE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFER {
        match value {
            false => RXFER::RXFE_0,
            true => RXFER::RXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFE_0`"]
    #[inline]
    pub fn is_rxfe_0(&self) -> bool {
        *self == RXFER::RXFE_0
    }
    #[doc = "Checks if the value of the field is `RXFE_1`"]
    #[inline]
    pub fn is_rxfe_1(&self) -> bool {
        *self == RXFER::RXFE_1
    }
}
#[doc = "Possible values of the field `TXFIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOSIZER {
    #[doc = "Transmit FIFO/Buffer depth = 1 dataword."]
    TXFIFOSIZE_0,
    #[doc = "Transmit FIFO/Buffer depth = 4 datawords."]
    TXFIFOSIZE_1,
    #[doc = "Transmit FIFO/Buffer depth = 8 datawords."]
    TXFIFOSIZE_2,
    #[doc = "Transmit FIFO/Buffer depth = 16 datawords."]
    TXFIFOSIZE_3,
    #[doc = "Transmit FIFO/Buffer depth = 32 datawords."]
    TXFIFOSIZE_4,
    #[doc = "Transmit FIFO/Buffer depth = 64 datawords."]
    TXFIFOSIZE_5,
    #[doc = "Transmit FIFO/Buffer depth = 128 datawords."]
    TXFIFOSIZE_6,
    #[doc = "Transmit FIFO/Buffer depth = 256 datawords"]
    TXFIFOSIZE_7,
}
impl TXFIFOSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXFIFOSIZER::TXFIFOSIZE_0 => 0,
            TXFIFOSIZER::TXFIFOSIZE_1 => 1,
            TXFIFOSIZER::TXFIFOSIZE_2 => 2,
            TXFIFOSIZER::TXFIFOSIZE_3 => 3,
            TXFIFOSIZER::TXFIFOSIZE_4 => 4,
            TXFIFOSIZER::TXFIFOSIZE_5 => 5,
            TXFIFOSIZER::TXFIFOSIZE_6 => 6,
            TXFIFOSIZER::TXFIFOSIZE_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXFIFOSIZER {
        match value {
            0 => TXFIFOSIZER::TXFIFOSIZE_0,
            1 => TXFIFOSIZER::TXFIFOSIZE_1,
            2 => TXFIFOSIZER::TXFIFOSIZE_2,
            3 => TXFIFOSIZER::TXFIFOSIZE_3,
            4 => TXFIFOSIZER::TXFIFOSIZE_4,
            5 => TXFIFOSIZER::TXFIFOSIZE_5,
            6 => TXFIFOSIZER::TXFIFOSIZE_6,
            7 => TXFIFOSIZER::TXFIFOSIZE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_0`"]
    #[inline]
    pub fn is_txfifosize_0(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_0
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_1`"]
    #[inline]
    pub fn is_txfifosize_1(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_1
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_2`"]
    #[inline]
    pub fn is_txfifosize_2(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_2
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_3`"]
    #[inline]
    pub fn is_txfifosize_3(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_3
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_4`"]
    #[inline]
    pub fn is_txfifosize_4(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_4
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_5`"]
    #[inline]
    pub fn is_txfifosize_5(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_5
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_6`"]
    #[inline]
    pub fn is_txfifosize_6(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_6
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_7`"]
    #[inline]
    pub fn is_txfifosize_7(&self) -> bool {
        *self == TXFIFOSIZER::TXFIFOSIZE_7
    }
}
#[doc = "Possible values of the field `TXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFER {
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    TXFE_0,
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    TXFE_1,
}
impl TXFER {
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
            TXFER::TXFE_0 => false,
            TXFER::TXFE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFER {
        match value {
            false => TXFER::TXFE_0,
            true => TXFER::TXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXFE_0`"]
    #[inline]
    pub fn is_txfe_0(&self) -> bool {
        *self == TXFER::TXFE_0
    }
    #[doc = "Checks if the value of the field is `TXFE_1`"]
    #[inline]
    pub fn is_txfe_1(&self) -> bool {
        *self == TXFER::TXFE_1
    }
}
#[doc = "Possible values of the field `RXUFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFER {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    RXUFE_0,
    #[doc = "RXUF flag generates an interrupt to the host."]
    RXUFE_1,
}
impl RXUFER {
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
            RXUFER::RXUFE_0 => false,
            RXUFER::RXUFE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXUFER {
        match value {
            false => RXUFER::RXUFE_0,
            true => RXUFER::RXUFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXUFE_0`"]
    #[inline]
    pub fn is_rxufe_0(&self) -> bool {
        *self == RXUFER::RXUFE_0
    }
    #[doc = "Checks if the value of the field is `RXUFE_1`"]
    #[inline]
    pub fn is_rxufe_1(&self) -> bool {
        *self == RXUFER::RXUFE_1
    }
}
#[doc = "Possible values of the field `TXOFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFER {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    TXOFE_0,
    #[doc = "TXOF flag generates an interrupt to the host."]
    TXOFE_1,
}
impl TXOFER {
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
            TXOFER::TXOFE_0 => false,
            TXOFER::TXOFE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOFER {
        match value {
            false => TXOFER::TXOFE_0,
            true => TXOFER::TXOFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXOFE_0`"]
    #[inline]
    pub fn is_txofe_0(&self) -> bool {
        *self == TXOFER::TXOFE_0
    }
    #[doc = "Checks if the value of the field is `TXOFE_1`"]
    #[inline]
    pub fn is_txofe_1(&self) -> bool {
        *self == TXOFER::TXOFE_1
    }
}
#[doc = "Possible values of the field `RXIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIDENR {
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    RXIDEN_0,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    RXIDEN_1,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    RXIDEN_2,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    RXIDEN_3,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    RXIDEN_4,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    RXIDEN_5,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    RXIDEN_6,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    RXIDEN_7,
}
impl RXIDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXIDENR::RXIDEN_0 => 0,
            RXIDENR::RXIDEN_1 => 1,
            RXIDENR::RXIDEN_2 => 2,
            RXIDENR::RXIDEN_3 => 3,
            RXIDENR::RXIDEN_4 => 4,
            RXIDENR::RXIDEN_5 => 5,
            RXIDENR::RXIDEN_6 => 6,
            RXIDENR::RXIDEN_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXIDENR {
        match value {
            0 => RXIDENR::RXIDEN_0,
            1 => RXIDENR::RXIDEN_1,
            2 => RXIDENR::RXIDEN_2,
            3 => RXIDENR::RXIDEN_3,
            4 => RXIDENR::RXIDEN_4,
            5 => RXIDENR::RXIDEN_5,
            6 => RXIDENR::RXIDEN_6,
            7 => RXIDENR::RXIDEN_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXIDEN_0`"]
    #[inline]
    pub fn is_rxiden_0(&self) -> bool {
        *self == RXIDENR::RXIDEN_0
    }
    #[doc = "Checks if the value of the field is `RXIDEN_1`"]
    #[inline]
    pub fn is_rxiden_1(&self) -> bool {
        *self == RXIDENR::RXIDEN_1
    }
    #[doc = "Checks if the value of the field is `RXIDEN_2`"]
    #[inline]
    pub fn is_rxiden_2(&self) -> bool {
        *self == RXIDENR::RXIDEN_2
    }
    #[doc = "Checks if the value of the field is `RXIDEN_3`"]
    #[inline]
    pub fn is_rxiden_3(&self) -> bool {
        *self == RXIDENR::RXIDEN_3
    }
    #[doc = "Checks if the value of the field is `RXIDEN_4`"]
    #[inline]
    pub fn is_rxiden_4(&self) -> bool {
        *self == RXIDENR::RXIDEN_4
    }
    #[doc = "Checks if the value of the field is `RXIDEN_5`"]
    #[inline]
    pub fn is_rxiden_5(&self) -> bool {
        *self == RXIDENR::RXIDEN_5
    }
    #[doc = "Checks if the value of the field is `RXIDEN_6`"]
    #[inline]
    pub fn is_rxiden_6(&self) -> bool {
        *self == RXIDENR::RXIDEN_6
    }
    #[doc = "Checks if the value of the field is `RXIDEN_7`"]
    #[inline]
    pub fn is_rxiden_7(&self) -> bool {
        *self == RXIDENR::RXIDEN_7
    }
}
#[doc = "Possible values of the field `RXUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFR {
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    RXUF_0,
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    RXUF_1,
}
impl RXUFR {
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
            RXUFR::RXUF_0 => false,
            RXUFR::RXUF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXUFR {
        match value {
            false => RXUFR::RXUF_0,
            true => RXUFR::RXUF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXUF_0`"]
    #[inline]
    pub fn is_rxuf_0(&self) -> bool {
        *self == RXUFR::RXUF_0
    }
    #[doc = "Checks if the value of the field is `RXUF_1`"]
    #[inline]
    pub fn is_rxuf_1(&self) -> bool {
        *self == RXUFR::RXUF_1
    }
}
#[doc = "Possible values of the field `TXOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFR {
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    TXOF_0,
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    TXOF_1,
}
impl TXOFR {
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
            TXOFR::TXOF_0 => false,
            TXOFR::TXOF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOFR {
        match value {
            false => TXOFR::TXOF_0,
            true => TXOFR::TXOF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXOF_0`"]
    #[inline]
    pub fn is_txof_0(&self) -> bool {
        *self == TXOFR::TXOF_0
    }
    #[doc = "Checks if the value of the field is `TXOF_1`"]
    #[inline]
    pub fn is_txof_1(&self) -> bool {
        *self == TXOFR::TXOF_1
    }
}
#[doc = "Possible values of the field `RXEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTR {
    #[doc = "Receive buffer is not empty."]
    RXEMPT_0,
    #[doc = "Receive buffer is empty."]
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
#[doc = "Possible values of the field `TXEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTR {
    #[doc = "Transmit buffer is not empty."]
    TXEMPT_0,
    #[doc = "Transmit buffer is empty."]
    TXEMPT_1,
}
impl TXEMPTR {
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
            TXEMPTR::TXEMPT_0 => false,
            TXEMPTR::TXEMPT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEMPTR {
        match value {
            false => TXEMPTR::TXEMPT_0,
            true => TXEMPTR::TXEMPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXEMPT_0`"]
    #[inline]
    pub fn is_txempt_0(&self) -> bool {
        *self == TXEMPTR::TXEMPT_0
    }
    #[doc = "Checks if the value of the field is `TXEMPT_1`"]
    #[inline]
    pub fn is_txempt_1(&self) -> bool {
        *self == TXEMPTR::TXEMPT_1
    }
}
#[doc = "Values that can be written to the field `RXFE`"]
pub enum RXFEW {
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    RXFE_0,
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    RXFE_1,
}
impl RXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFEW::RXFE_0 => false,
            RXFEW::RXFE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    #[inline]
    pub fn rxfe_0(self) -> &'a mut W {
        self.variant(RXFEW::RXFE_0)
    }
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    #[inline]
    pub fn rxfe_1(self) -> &'a mut W {
        self.variant(RXFEW::RXFE_1)
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
#[doc = "Values that can be written to the field `TXFE`"]
pub enum TXFEW {
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    TXFE_0,
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    TXFE_1,
}
impl TXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFEW::TXFE_0 => false,
            TXFEW::TXFE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    #[inline]
    pub fn txfe_0(self) -> &'a mut W {
        self.variant(TXFEW::TXFE_0)
    }
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    #[inline]
    pub fn txfe_1(self) -> &'a mut W {
        self.variant(TXFEW::TXFE_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXUFE`"]
pub enum RXUFEW {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    RXUFE_0,
    #[doc = "RXUF flag generates an interrupt to the host."]
    RXUFE_1,
}
impl RXUFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXUFEW::RXUFE_0 => false,
            RXUFEW::RXUFE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXUFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXUFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXUFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline]
    pub fn rxufe_0(self) -> &'a mut W {
        self.variant(RXUFEW::RXUFE_0)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline]
    pub fn rxufe_1(self) -> &'a mut W {
        self.variant(RXUFEW::RXUFE_1)
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
#[doc = "Values that can be written to the field `TXOFE`"]
pub enum TXOFEW {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    TXOFE_0,
    #[doc = "TXOF flag generates an interrupt to the host."]
    TXOFE_1,
}
impl TXOFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOFEW::TXOFE_0 => false,
            TXOFEW::TXOFE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline]
    pub fn txofe_0(self) -> &'a mut W {
        self.variant(TXOFEW::TXOFE_0)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline]
    pub fn txofe_1(self) -> &'a mut W {
        self.variant(TXOFEW::TXOFE_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIDEN`"]
pub enum RXIDENW {
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    RXIDEN_0,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    RXIDEN_1,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    RXIDEN_2,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    RXIDEN_3,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    RXIDEN_4,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    RXIDEN_5,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    RXIDEN_6,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    RXIDEN_7,
}
impl RXIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXIDENW::RXIDEN_0 => 0,
            RXIDENW::RXIDEN_1 => 1,
            RXIDENW::RXIDEN_2 => 2,
            RXIDENW::RXIDEN_3 => 3,
            RXIDENW::RXIDEN_4 => 4,
            RXIDENW::RXIDEN_5 => 5,
            RXIDENW::RXIDEN_6 => 6,
            RXIDENW::RXIDEN_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIDENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    #[inline]
    pub fn rxiden_0(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_0)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    #[inline]
    pub fn rxiden_1(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_1)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    #[inline]
    pub fn rxiden_2(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_2)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    #[inline]
    pub fn rxiden_3(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_3)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    #[inline]
    pub fn rxiden_4(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_4)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    #[inline]
    pub fn rxiden_5(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_5)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    #[inline]
    pub fn rxiden_6(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_6)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    #[inline]
    pub fn rxiden_7(self) -> &'a mut W {
        self.variant(RXIDENW::RXIDEN_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXFLUSH`"]
pub enum RXFLUSHW {
    #[doc = "No flush operation occurs."]
    RXFLUSH_0,
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    RXFLUSH_1,
}
impl RXFLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFLUSHW::RXFLUSH_0 => false,
            RXFLUSHW::RXFLUSH_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline]
    pub fn rxflush_0(self) -> &'a mut W {
        self.variant(RXFLUSHW::RXFLUSH_0)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline]
    pub fn rxflush_1(self) -> &'a mut W {
        self.variant(RXFLUSHW::RXFLUSH_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXFLUSH`"]
pub enum TXFLUSHW {
    #[doc = "No flush operation occurs."]
    TXFLUSH_0,
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    TXFLUSH_1,
}
impl TXFLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFLUSHW::TXFLUSH_0 => false,
            TXFLUSHW::TXFLUSH_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline]
    pub fn txflush_0(self) -> &'a mut W {
        self.variant(TXFLUSHW::TXFLUSH_0)
    }
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    #[inline]
    pub fn txflush_1(self) -> &'a mut W {
        self.variant(TXFLUSHW::TXFLUSH_1)
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
#[doc = "Values that can be written to the field `RXUF`"]
pub enum RXUFW {
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    RXUF_0,
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    RXUF_1,
}
impl RXUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXUFW::RXUF_0 => false,
            RXUFW::RXUF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXUFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn rxuf_0(self) -> &'a mut W {
        self.variant(RXUFW::RXUF_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn rxuf_1(self) -> &'a mut W {
        self.variant(RXUFW::RXUF_1)
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
#[doc = "Values that can be written to the field `TXOF`"]
pub enum TXOFW {
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    TXOF_0,
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    TXOF_1,
}
impl TXOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOFW::TXOF_0 => false,
            TXOFW::TXOF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOFW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn txof_0(self) -> &'a mut W {
        self.variant(TXOFW::TXOF_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn txof_1(self) -> &'a mut W {
        self.variant(TXOFW::TXOF_1)
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
    #[doc = "Bits 0:2 - Receive FIFO. Buffer Depth"]
    #[inline]
    pub fn rxfifosize(&self) -> RXFIFOSIZER {
        RXFIFOSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        RXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Transmit FIFO. Buffer Depth"]
    #[inline]
    pub fn txfifosize(&self) -> TXFIFOSIZER {
        TXFIFOSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        TXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline]
    pub fn rxufe(&self) -> RXUFER {
        RXUFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn txofe(&self) -> TXOFER {
        TXOFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline]
    pub fn rxiden(&self) -> RXIDENR {
        RXIDENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline]
    pub fn rxuf(&self) -> RXUFR {
        RXUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline]
    pub fn txof(&self) -> TXOFR {
        TXOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Receive Buffer/FIFO Empty"]
    #[inline]
    pub fn rxempt(&self) -> RXEMPTR {
        RXEMPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit Buffer/FIFO Empty"]
    #[inline]
    pub fn txempt(&self) -> TXEMPTR {
        TXEMPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12582929 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline]
    pub fn rxfe(&mut self) -> _RXFEW {
        _RXFEW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline]
    pub fn txfe(&mut self) -> _TXFEW {
        _TXFEW { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline]
    pub fn rxufe(&mut self) -> _RXUFEW {
        _RXUFEW { w: self }
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn txofe(&mut self) -> _TXOFEW {
        _TXOFEW { w: self }
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline]
    pub fn rxiden(&mut self) -> _RXIDENW {
        _RXIDENW { w: self }
    }
    #[doc = "Bit 14 - Receive FIFO/Buffer Flush"]
    #[inline]
    pub fn rxflush(&mut self) -> _RXFLUSHW {
        _RXFLUSHW { w: self }
    }
    #[doc = "Bit 15 - Transmit FIFO/Buffer Flush"]
    #[inline]
    pub fn txflush(&mut self) -> _TXFLUSHW {
        _TXFLUSHW { w: self }
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline]
    pub fn rxuf(&mut self) -> _RXUFW {
        _RXUFW { w: self }
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline]
    pub fn txof(&mut self) -> _TXOFW {
        _TXOFW { w: self }
    }
}
