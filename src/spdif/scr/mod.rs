#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
#[doc = "Possible values of the field `USrc_Sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USRC_SELR {
    #[doc = "No embedded U channel"]
    USRC_SEL_0,
    #[doc = "U channel from SPDIF receive block (CD mode)"]
    USRC_SEL_1,
    #[doc = "U channel from on chip transmitter"]
    USRC_SEL_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USRC_SELR::USRC_SEL_0 => 0,
            USRC_SELR::USRC_SEL_1 => 1,
            USRC_SELR::USRC_SEL_3 => 3,
            USRC_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USRC_SELR {
        match value {
            0 => USRC_SELR::USRC_SEL_0,
            1 => USRC_SELR::USRC_SEL_1,
            3 => USRC_SELR::USRC_SEL_3,
            i => USRC_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_0`"]
    #[inline]
    pub fn is_usrc_sel_0(&self) -> bool {
        *self == USRC_SELR::USRC_SEL_0
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_1`"]
    #[inline]
    pub fn is_usrc_sel_1(&self) -> bool {
        *self == USRC_SELR::USRC_SEL_1
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_3`"]
    #[inline]
    pub fn is_usrc_sel_3(&self) -> bool {
        *self == USRC_SELR::USRC_SEL_3
    }
}
#[doc = "Possible values of the field `TxSel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSELR {
    #[doc = "Off and output 0"]
    TXSEL_0,
    #[doc = "Feed-through SPDIFIN"]
    TXSEL_1,
    #[doc = "Tx Normal operation"]
    TXSEL_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXSELR::TXSEL_0 => 0,
            TXSELR::TXSEL_1 => 1,
            TXSELR::TXSEL_5 => 5,
            TXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXSELR {
        match value {
            0 => TXSELR::TXSEL_0,
            1 => TXSELR::TXSEL_1,
            5 => TXSELR::TXSEL_5,
            i => TXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXSEL_0`"]
    #[inline]
    pub fn is_tx_sel_0(&self) -> bool {
        *self == TXSELR::TXSEL_0
    }
    #[doc = "Checks if the value of the field is `TXSEL_1`"]
    #[inline]
    pub fn is_tx_sel_1(&self) -> bool {
        *self == TXSELR::TXSEL_1
    }
    #[doc = "Checks if the value of the field is `TXSEL_5`"]
    #[inline]
    pub fn is_tx_sel_5(&self) -> bool {
        *self == TXSELR::TXSEL_5
    }
}
#[doc = "Possible values of the field `ValCtrl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALCTRLR {
    #[doc = "Outgoing Validity always set"]
    VALCTRL_0,
    #[doc = "Outgoing Validity always clear"]
    VALCTRL_1,
}
impl VALCTRLR {
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
            VALCTRLR::VALCTRL_0 => false,
            VALCTRLR::VALCTRL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALCTRLR {
        match value {
            false => VALCTRLR::VALCTRL_0,
            true => VALCTRLR::VALCTRL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALCTRL_0`"]
    #[inline]
    pub fn is_val_ctrl_0(&self) -> bool {
        *self == VALCTRLR::VALCTRL_0
    }
    #[doc = "Checks if the value of the field is `VALCTRL_1`"]
    #[inline]
    pub fn is_val_ctrl_1(&self) -> bool {
        *self == VALCTRLR::VALCTRL_1
    }
}
#[doc = r" Value of the field"]
pub struct DMA_TX_ENR {
    bits: bool,
}
impl DMA_TX_ENR {
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
pub struct DMA_RX_ENR {
    bits: bool,
}
impl DMA_RX_ENR {
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
#[doc = "Possible values of the field `TxFIFO_Ctrl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFO_CTRLR {
    #[doc = "Send out digital zero on SPDIF Tx"]
    TXFIFO_CTRL_0,
    #[doc = "Tx Normal operation"]
    TXFIFO_CTRL_1,
    #[doc = "Reset to 1 sample remaining"]
    TXFIFO_CTRL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXFIFO_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXFIFO_CTRLR::TXFIFO_CTRL_0 => 0,
            TXFIFO_CTRLR::TXFIFO_CTRL_1 => 1,
            TXFIFO_CTRLR::TXFIFO_CTRL_2 => 2,
            TXFIFO_CTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXFIFO_CTRLR {
        match value {
            0 => TXFIFO_CTRLR::TXFIFO_CTRL_0,
            1 => TXFIFO_CTRLR::TXFIFO_CTRL_1,
            2 => TXFIFO_CTRLR::TXFIFO_CTRL_2,
            i => TXFIFO_CTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFO_CTRL_0`"]
    #[inline]
    pub fn is_tx_fifo_ctrl_0(&self) -> bool {
        *self == TXFIFO_CTRLR::TXFIFO_CTRL_0
    }
    #[doc = "Checks if the value of the field is `TXFIFO_CTRL_1`"]
    #[inline]
    pub fn is_tx_fifo_ctrl_1(&self) -> bool {
        *self == TXFIFO_CTRLR::TXFIFO_CTRL_1
    }
    #[doc = "Checks if the value of the field is `TXFIFO_CTRL_2`"]
    #[inline]
    pub fn is_tx_fifo_ctrl_2(&self) -> bool {
        *self == TXFIFO_CTRLR::TXFIFO_CTRL_2
    }
}
#[doc = r" Value of the field"]
pub struct SOFT_RESETR {
    bits: bool,
}
impl SOFT_RESETR {
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
pub struct LOW_POWERR {
    bits: bool,
}
impl LOW_POWERR {
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
#[doc = "Possible values of the field `TxFIFOEmpty_Sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOEMPTY_SELR {
    #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_0,
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_1,
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_2,
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_3,
}
impl TXFIFOEMPTY_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_0 => 0,
            TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_1 => 1,
            TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_2 => 2,
            TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXFIFOEMPTY_SELR {
        match value {
            0 => TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_0,
            1 => TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_1,
            2 => TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_2,
            3 => TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_0`"]
    #[inline]
    pub fn is_tx_fifoempty_sel_0(&self) -> bool {
        *self == TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_0
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_1`"]
    #[inline]
    pub fn is_tx_fifoempty_sel_1(&self) -> bool {
        *self == TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_1
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_2`"]
    #[inline]
    pub fn is_tx_fifoempty_sel_2(&self) -> bool {
        *self == TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_2
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_3`"]
    #[inline]
    pub fn is_tx_fifoempty_sel_3(&self) -> bool {
        *self == TXFIFOEMPTY_SELR::TXFIFOEMPTY_SEL_3
    }
}
#[doc = "Possible values of the field `TxAutoSync`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXAUTOSYNCR {
    #[doc = "Tx FIFO auto sync off"]
    TXAUTOSYNC_0,
    #[doc = "Tx FIFO auto sync on"]
    TXAUTOSYNC_1,
}
impl TXAUTOSYNCR {
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
            TXAUTOSYNCR::TXAUTOSYNC_0 => false,
            TXAUTOSYNCR::TXAUTOSYNC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXAUTOSYNCR {
        match value {
            false => TXAUTOSYNCR::TXAUTOSYNC_0,
            true => TXAUTOSYNCR::TXAUTOSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXAUTOSYNC_0`"]
    #[inline]
    pub fn is_tx_auto_sync_0(&self) -> bool {
        *self == TXAUTOSYNCR::TXAUTOSYNC_0
    }
    #[doc = "Checks if the value of the field is `TXAUTOSYNC_1`"]
    #[inline]
    pub fn is_tx_auto_sync_1(&self) -> bool {
        *self == TXAUTOSYNCR::TXAUTOSYNC_1
    }
}
#[doc = "Possible values of the field `RxAutoSync`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXAUTOSYNCR {
    #[doc = "Rx FIFO auto sync off"]
    RXAUTOSYNC_0,
    #[doc = "RxFIFO auto sync on"]
    RXAUTOSYNC_1,
}
impl RXAUTOSYNCR {
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
            RXAUTOSYNCR::RXAUTOSYNC_0 => false,
            RXAUTOSYNCR::RXAUTOSYNC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXAUTOSYNCR {
        match value {
            false => RXAUTOSYNCR::RXAUTOSYNC_0,
            true => RXAUTOSYNCR::RXAUTOSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXAUTOSYNC_0`"]
    #[inline]
    pub fn is_rx_auto_sync_0(&self) -> bool {
        *self == RXAUTOSYNCR::RXAUTOSYNC_0
    }
    #[doc = "Checks if the value of the field is `RXAUTOSYNC_1`"]
    #[inline]
    pub fn is_rx_auto_sync_1(&self) -> bool {
        *self == RXAUTOSYNCR::RXAUTOSYNC_1
    }
}
#[doc = "Possible values of the field `RxFIFOFull_Sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOFULL_SELR {
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_0,
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_1,
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_2,
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
    RXFIFOFULL_SEL_3,
}
impl RXFIFOFULL_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXFIFOFULL_SELR::RXFIFOFULL_SEL_0 => 0,
            RXFIFOFULL_SELR::RXFIFOFULL_SEL_1 => 1,
            RXFIFOFULL_SELR::RXFIFOFULL_SEL_2 => 2,
            RXFIFOFULL_SELR::RXFIFOFULL_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXFIFOFULL_SELR {
        match value {
            0 => RXFIFOFULL_SELR::RXFIFOFULL_SEL_0,
            1 => RXFIFOFULL_SELR::RXFIFOFULL_SEL_1,
            2 => RXFIFOFULL_SELR::RXFIFOFULL_SEL_2,
            3 => RXFIFOFULL_SELR::RXFIFOFULL_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_0`"]
    #[inline]
    pub fn is_rx_fifofull_sel_0(&self) -> bool {
        *self == RXFIFOFULL_SELR::RXFIFOFULL_SEL_0
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_1`"]
    #[inline]
    pub fn is_rx_fifofull_sel_1(&self) -> bool {
        *self == RXFIFOFULL_SELR::RXFIFOFULL_SEL_1
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_2`"]
    #[inline]
    pub fn is_rx_fifofull_sel_2(&self) -> bool {
        *self == RXFIFOFULL_SELR::RXFIFOFULL_SEL_2
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_3`"]
    #[inline]
    pub fn is_rx_fifofull_sel_3(&self) -> bool {
        *self == RXFIFOFULL_SELR::RXFIFOFULL_SEL_3
    }
}
#[doc = "Possible values of the field `RxFIFO_Rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFO_RSTR {
    #[doc = "Normal operation"]
    RXFIFO_RST_0,
    #[doc = "Reset register to 1 sample remaining"]
    RXFIFO_RST_1,
}
impl RXFIFO_RSTR {
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
            RXFIFO_RSTR::RXFIFO_RST_0 => false,
            RXFIFO_RSTR::RXFIFO_RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFIFO_RSTR {
        match value {
            false => RXFIFO_RSTR::RXFIFO_RST_0,
            true => RXFIFO_RSTR::RXFIFO_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_RST_0`"]
    #[inline]
    pub fn is_rx_fifo_rst_0(&self) -> bool {
        *self == RXFIFO_RSTR::RXFIFO_RST_0
    }
    #[doc = "Checks if the value of the field is `RXFIFO_RST_1`"]
    #[inline]
    pub fn is_rx_fifo_rst_1(&self) -> bool {
        *self == RXFIFO_RSTR::RXFIFO_RST_1
    }
}
#[doc = "Possible values of the field `RxFIFO_Off_On`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFO_OFF_ONR {
    #[doc = "SPDIF Rx FIFO is on"]
    RXFIFO_OFF_ON_0,
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
    RXFIFO_OFF_ON_1,
}
impl RXFIFO_OFF_ONR {
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
            RXFIFO_OFF_ONR::RXFIFO_OFF_ON_0 => false,
            RXFIFO_OFF_ONR::RXFIFO_OFF_ON_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFIFO_OFF_ONR {
        match value {
            false => RXFIFO_OFF_ONR::RXFIFO_OFF_ON_0,
            true => RXFIFO_OFF_ONR::RXFIFO_OFF_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_OFF_ON_0`"]
    #[inline]
    pub fn is_rx_fifo_off_on_0(&self) -> bool {
        *self == RXFIFO_OFF_ONR::RXFIFO_OFF_ON_0
    }
    #[doc = "Checks if the value of the field is `RXFIFO_OFF_ON_1`"]
    #[inline]
    pub fn is_rx_fifo_off_on_1(&self) -> bool {
        *self == RXFIFO_OFF_ONR::RXFIFO_OFF_ON_1
    }
}
#[doc = "Possible values of the field `RxFIFO_Ctrl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFO_CTRLR {
    #[doc = "Normal operation"]
    RXFIFO_CTRL_0,
    #[doc = "Always read zero from Rx data register"]
    RXFIFO_CTRL_1,
}
impl RXFIFO_CTRLR {
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
            RXFIFO_CTRLR::RXFIFO_CTRL_0 => false,
            RXFIFO_CTRLR::RXFIFO_CTRL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFIFO_CTRLR {
        match value {
            false => RXFIFO_CTRLR::RXFIFO_CTRL_0,
            true => RXFIFO_CTRLR::RXFIFO_CTRL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_CTRL_0`"]
    #[inline]
    pub fn is_rx_fifo_ctrl_0(&self) -> bool {
        *self == RXFIFO_CTRLR::RXFIFO_CTRL_0
    }
    #[doc = "Checks if the value of the field is `RXFIFO_CTRL_1`"]
    #[inline]
    pub fn is_rx_fifo_ctrl_1(&self) -> bool {
        *self == RXFIFO_CTRLR::RXFIFO_CTRL_1
    }
}
#[doc = "Values that can be written to the field `USrc_Sel`"]
pub enum USRC_SELW {
    #[doc = "No embedded U channel"]
    USRC_SEL_0,
    #[doc = "U channel from SPDIF receive block (CD mode)"]
    USRC_SEL_1,
    #[doc = "U channel from on chip transmitter"]
    USRC_SEL_3,
}
impl USRC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USRC_SELW::USRC_SEL_0 => 0,
            USRC_SELW::USRC_SEL_1 => 1,
            USRC_SELW::USRC_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USRC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USRC_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No embedded U channel"]
    #[inline]
    pub fn usrc_sel_0(self) -> &'a mut W {
        self.variant(USRC_SELW::USRC_SEL_0)
    }
    #[doc = "U channel from SPDIF receive block (CD mode)"]
    #[inline]
    pub fn usrc_sel_1(self) -> &'a mut W {
        self.variant(USRC_SELW::USRC_SEL_1)
    }
    #[doc = "U channel from on chip transmitter"]
    #[inline]
    pub fn usrc_sel_3(self) -> &'a mut W {
        self.variant(USRC_SELW::USRC_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TxSel`"]
pub enum TXSELW {
    #[doc = "Off and output 0"]
    TXSEL_0,
    #[doc = "Feed-through SPDIFIN"]
    TXSEL_1,
    #[doc = "Tx Normal operation"]
    TXSEL_5,
}
impl TXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXSELW::TXSEL_0 => 0,
            TXSELW::TXSEL_1 => 1,
            TXSELW::TXSEL_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Off and output 0"]
    #[inline]
    pub fn tx_sel_0(self) -> &'a mut W {
        self.variant(TXSELW::TXSEL_0)
    }
    #[doc = "Feed-through SPDIFIN"]
    #[inline]
    pub fn tx_sel_1(self) -> &'a mut W {
        self.variant(TXSELW::TXSEL_1)
    }
    #[doc = "Tx Normal operation"]
    #[inline]
    pub fn tx_sel_5(self) -> &'a mut W {
        self.variant(TXSELW::TXSEL_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ValCtrl`"]
pub enum VALCTRLW {
    #[doc = "Outgoing Validity always set"]
    VALCTRL_0,
    #[doc = "Outgoing Validity always clear"]
    VALCTRL_1,
}
impl VALCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VALCTRLW::VALCTRL_0 => false,
            VALCTRLW::VALCTRL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _VALCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Outgoing Validity always set"]
    #[inline]
    pub fn val_ctrl_0(self) -> &'a mut W {
        self.variant(VALCTRLW::VALCTRL_0)
    }
    #[doc = "Outgoing Validity always clear"]
    #[inline]
    pub fn val_ctrl_1(self) -> &'a mut W {
        self.variant(VALCTRLW::VALCTRL_1)
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
pub struct _DMA_TX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TX_ENW<'a> {
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
pub struct _DMA_RX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_RX_ENW<'a> {
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
#[doc = "Values that can be written to the field `TxFIFO_Ctrl`"]
pub enum TXFIFO_CTRLW {
    #[doc = "Send out digital zero on SPDIF Tx"]
    TXFIFO_CTRL_0,
    #[doc = "Tx Normal operation"]
    TXFIFO_CTRL_1,
    #[doc = "Reset to 1 sample remaining"]
    TXFIFO_CTRL_2,
}
impl TXFIFO_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXFIFO_CTRLW::TXFIFO_CTRL_0 => 0,
            TXFIFO_CTRLW::TXFIFO_CTRL_1 => 1,
            TXFIFO_CTRLW::TXFIFO_CTRL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFIFO_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFIFO_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFIFO_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Send out digital zero on SPDIF Tx"]
    #[inline]
    pub fn tx_fifo_ctrl_0(self) -> &'a mut W {
        self.variant(TXFIFO_CTRLW::TXFIFO_CTRL_0)
    }
    #[doc = "Tx Normal operation"]
    #[inline]
    pub fn tx_fifo_ctrl_1(self) -> &'a mut W {
        self.variant(TXFIFO_CTRLW::TXFIFO_CTRL_1)
    }
    #[doc = "Reset to 1 sample remaining"]
    #[inline]
    pub fn tx_fifo_ctrl_2(self) -> &'a mut W {
        self.variant(TXFIFO_CTRLW::TXFIFO_CTRL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOFT_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFT_RESETW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOW_POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _LOW_POWERW<'a> {
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
#[doc = "Values that can be written to the field `TxFIFOEmpty_Sel`"]
pub enum TXFIFOEMPTY_SELW {
    #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_0,
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_1,
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_2,
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_3,
}
impl TXFIFOEMPTY_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_0 => 0,
            TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_1 => 1,
            TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_2 => 2,
            TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFIFOEMPTY_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFIFOEMPTY_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFIFOEMPTY_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
    #[inline]
    pub fn tx_fifoempty_sel_0(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_0)
    }
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    #[inline]
    pub fn tx_fifoempty_sel_1(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_1)
    }
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    #[inline]
    pub fn tx_fifoempty_sel_2(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_2)
    }
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    #[inline]
    pub fn tx_fifoempty_sel_3(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SELW::TXFIFOEMPTY_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TxAutoSync`"]
pub enum TXAUTOSYNCW {
    #[doc = "Tx FIFO auto sync off"]
    TXAUTOSYNC_0,
    #[doc = "Tx FIFO auto sync on"]
    TXAUTOSYNC_1,
}
impl TXAUTOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXAUTOSYNCW::TXAUTOSYNC_0 => false,
            TXAUTOSYNCW::TXAUTOSYNC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXAUTOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TXAUTOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXAUTOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tx FIFO auto sync off"]
    #[inline]
    pub fn tx_auto_sync_0(self) -> &'a mut W {
        self.variant(TXAUTOSYNCW::TXAUTOSYNC_0)
    }
    #[doc = "Tx FIFO auto sync on"]
    #[inline]
    pub fn tx_auto_sync_1(self) -> &'a mut W {
        self.variant(TXAUTOSYNCW::TXAUTOSYNC_1)
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
#[doc = "Values that can be written to the field `RxAutoSync`"]
pub enum RXAUTOSYNCW {
    #[doc = "Rx FIFO auto sync off"]
    RXAUTOSYNC_0,
    #[doc = "RxFIFO auto sync on"]
    RXAUTOSYNC_1,
}
impl RXAUTOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXAUTOSYNCW::RXAUTOSYNC_0 => false,
            RXAUTOSYNCW::RXAUTOSYNC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXAUTOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _RXAUTOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXAUTOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx FIFO auto sync off"]
    #[inline]
    pub fn rx_auto_sync_0(self) -> &'a mut W {
        self.variant(RXAUTOSYNCW::RXAUTOSYNC_0)
    }
    #[doc = "RxFIFO auto sync on"]
    #[inline]
    pub fn rx_auto_sync_1(self) -> &'a mut W {
        self.variant(RXAUTOSYNCW::RXAUTOSYNC_1)
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
#[doc = "Values that can be written to the field `RxFIFOFull_Sel`"]
pub enum RXFIFOFULL_SELW {
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_0,
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_1,
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_2,
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
    RXFIFOFULL_SEL_3,
}
impl RXFIFOFULL_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXFIFOFULL_SELW::RXFIFOFULL_SEL_0 => 0,
            RXFIFOFULL_SELW::RXFIFOFULL_SEL_1 => 1,
            RXFIFOFULL_SELW::RXFIFOFULL_SEL_2 => 2,
            RXFIFOFULL_SELW::RXFIFOFULL_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFOFULL_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFOFULL_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFIFOFULL_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    #[inline]
    pub fn rx_fifofull_sel_0(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SELW::RXFIFOFULL_SEL_0)
    }
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    #[inline]
    pub fn rx_fifofull_sel_1(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SELW::RXFIFOFULL_SEL_1)
    }
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    #[inline]
    pub fn rx_fifofull_sel_2(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SELW::RXFIFOFULL_SEL_2)
    }
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
    #[inline]
    pub fn rx_fifofull_sel_3(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SELW::RXFIFOFULL_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RxFIFO_Rst`"]
pub enum RXFIFO_RSTW {
    #[doc = "Normal operation"]
    RXFIFO_RST_0,
    #[doc = "Reset register to 1 sample remaining"]
    RXFIFO_RST_1,
}
impl RXFIFO_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFO_RSTW::RXFIFO_RST_0 => false,
            RXFIFO_RSTW::RXFIFO_RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFO_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFO_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFIFO_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn rx_fifo_rst_0(self) -> &'a mut W {
        self.variant(RXFIFO_RSTW::RXFIFO_RST_0)
    }
    #[doc = "Reset register to 1 sample remaining"]
    #[inline]
    pub fn rx_fifo_rst_1(self) -> &'a mut W {
        self.variant(RXFIFO_RSTW::RXFIFO_RST_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RxFIFO_Off_On`"]
pub enum RXFIFO_OFF_ONW {
    #[doc = "SPDIF Rx FIFO is on"]
    RXFIFO_OFF_ON_0,
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
    RXFIFO_OFF_ON_1,
}
impl RXFIFO_OFF_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFO_OFF_ONW::RXFIFO_OFF_ON_0 => false,
            RXFIFO_OFF_ONW::RXFIFO_OFF_ON_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFO_OFF_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFO_OFF_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFIFO_OFF_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPDIF Rx FIFO is on"]
    #[inline]
    pub fn rx_fifo_off_on_0(self) -> &'a mut W {
        self.variant(RXFIFO_OFF_ONW::RXFIFO_OFF_ON_0)
    }
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
    #[inline]
    pub fn rx_fifo_off_on_1(self) -> &'a mut W {
        self.variant(RXFIFO_OFF_ONW::RXFIFO_OFF_ON_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RxFIFO_Ctrl`"]
pub enum RXFIFO_CTRLW {
    #[doc = "Normal operation"]
    RXFIFO_CTRL_0,
    #[doc = "Always read zero from Rx data register"]
    RXFIFO_CTRL_1,
}
impl RXFIFO_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFO_CTRLW::RXFIFO_CTRL_0 => false,
            RXFIFO_CTRLW::RXFIFO_CTRL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFO_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFO_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFIFO_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn rx_fifo_ctrl_0(self) -> &'a mut W {
        self.variant(RXFIFO_CTRLW::RXFIFO_CTRL_0)
    }
    #[doc = "Always read zero from Rx data register"]
    #[inline]
    pub fn rx_fifo_ctrl_1(self) -> &'a mut W {
        self.variant(RXFIFO_CTRLW::RXFIFO_CTRL_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - no description available"]
    #[inline]
    pub fn usrc_sel(&self) -> USRC_SELR {
        USRC_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:4 - no description available"]
    #[inline]
    pub fn tx_sel(&self) -> TXSELR {
        TXSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn val_ctrl(&self) -> VALCTRLR {
        VALCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - DMA Transmit Request Enable (Tx FIFO empty)"]
    #[inline]
    pub fn dma_tx_en(&self) -> DMA_TX_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_TX_ENR { bits }
    }
    #[doc = "Bit 9 - DMA Receive Request Enable (RX FIFO full)"]
    #[inline]
    pub fn dma_rx_en(&self) -> DMA_RX_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_RX_ENR { bits }
    }
    #[doc = "Bits 10:11 - no description available"]
    #[inline]
    pub fn tx_fifo_ctrl(&self) -> TXFIFO_CTRLR {
        TXFIFO_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - When write 1 to this bit, it will cause SPDIF software reset"]
    #[inline]
    pub fn soft_reset(&self) -> SOFT_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOFT_RESETR { bits }
    }
    #[doc = "Bit 13 - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[inline]
    pub fn low_power(&self) -> LOW_POWERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOW_POWERR { bits }
    }
    #[doc = "Bits 15:16 - no description available"]
    #[inline]
    pub fn tx_fifoempty_sel(&self) -> TXFIFOEMPTY_SELR {
        TXFIFOEMPTY_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn tx_auto_sync(&self) -> TXAUTOSYNCR {
        TXAUTOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn rx_auto_sync(&self) -> RXAUTOSYNCR {
        RXAUTOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:20 - no description available"]
    #[inline]
    pub fn rx_fifofull_sel(&self) -> RXFIFOFULL_SELR {
        RXFIFOFULL_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - no description available"]
    #[inline]
    pub fn rx_fifo_rst(&self) -> RXFIFO_RSTR {
        RXFIFO_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - no description available"]
    #[inline]
    pub fn rx_fifo_off_on(&self) -> RXFIFO_OFF_ONR {
        RXFIFO_OFF_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - no description available"]
    #[inline]
    pub fn rx_fifo_ctrl(&self) -> RXFIFO_CTRLR {
        RXFIFO_CTRLR::_from({
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
        W { bits: 1024 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - no description available"]
    #[inline]
    pub fn usrc_sel(&mut self) -> _USRC_SELW {
        _USRC_SELW { w: self }
    }
    #[doc = "Bits 2:4 - no description available"]
    #[inline]
    pub fn tx_sel(&mut self) -> _TXSELW {
        _TXSELW { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn val_ctrl(&mut self) -> _VALCTRLW {
        _VALCTRLW { w: self }
    }
    #[doc = "Bit 8 - DMA Transmit Request Enable (Tx FIFO empty)"]
    #[inline]
    pub fn dma_tx_en(&mut self) -> _DMA_TX_ENW {
        _DMA_TX_ENW { w: self }
    }
    #[doc = "Bit 9 - DMA Receive Request Enable (RX FIFO full)"]
    #[inline]
    pub fn dma_rx_en(&mut self) -> _DMA_RX_ENW {
        _DMA_RX_ENW { w: self }
    }
    #[doc = "Bits 10:11 - no description available"]
    #[inline]
    pub fn tx_fifo_ctrl(&mut self) -> _TXFIFO_CTRLW {
        _TXFIFO_CTRLW { w: self }
    }
    #[doc = "Bit 12 - When write 1 to this bit, it will cause SPDIF software reset"]
    #[inline]
    pub fn soft_reset(&mut self) -> _SOFT_RESETW {
        _SOFT_RESETW { w: self }
    }
    #[doc = "Bit 13 - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[inline]
    pub fn low_power(&mut self) -> _LOW_POWERW {
        _LOW_POWERW { w: self }
    }
    #[doc = "Bits 15:16 - no description available"]
    #[inline]
    pub fn tx_fifoempty_sel(&mut self) -> _TXFIFOEMPTY_SELW {
        _TXFIFOEMPTY_SELW { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn tx_auto_sync(&mut self) -> _TXAUTOSYNCW {
        _TXAUTOSYNCW { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn rx_auto_sync(&mut self) -> _RXAUTOSYNCW {
        _RXAUTOSYNCW { w: self }
    }
    #[doc = "Bits 19:20 - no description available"]
    #[inline]
    pub fn rx_fifofull_sel(&mut self) -> _RXFIFOFULL_SELW {
        _RXFIFOFULL_SELW { w: self }
    }
    #[doc = "Bit 21 - no description available"]
    #[inline]
    pub fn rx_fifo_rst(&mut self) -> _RXFIFO_RSTW {
        _RXFIFO_RSTW { w: self }
    }
    #[doc = "Bit 22 - no description available"]
    #[inline]
    pub fn rx_fifo_off_on(&mut self) -> _RXFIFO_OFF_ONW {
        _RXFIFO_OFF_ONW { w: self }
    }
    #[doc = "Bit 23 - no description available"]
    #[inline]
    pub fn rx_fifo_ctrl(&mut self) -> _RXFIFO_CTRLW {
        _RXFIFO_CTRLW { w: self }
    }
}
