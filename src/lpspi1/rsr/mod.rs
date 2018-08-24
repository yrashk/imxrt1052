#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFR {
    #[doc = "Subsequent data word received after LPSPI_PCS assertion"]
    SOF_0,
    #[doc = "First data word received after LPSPI_PCS assertion"]
    SOF_1,
}
impl SOFR {
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
            SOFR::SOF_0 => false,
            SOFR::SOF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFR {
        match value {
            false => SOFR::SOF_0,
            true => SOFR::SOF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_0`"]
    #[inline]
    pub fn is_sof_0(&self) -> bool {
        *self == SOFR::SOF_0
    }
    #[doc = "Checks if the value of the field is `SOF_1`"]
    #[inline]
    pub fn is_sof_1(&self) -> bool {
        *self == SOFR::SOF_1
    }
}
#[doc = "Possible values of the field `RXEMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTYR {
    #[doc = "RX FIFO is not empty"]
    RXEMPTY_0,
    #[doc = "RX FIFO is empty"]
    RXEMPTY_1,
}
impl RXEMPTYR {
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
            RXEMPTYR::RXEMPTY_0 => false,
            RXEMPTYR::RXEMPTY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEMPTYR {
        match value {
            false => RXEMPTYR::RXEMPTY_0,
            true => RXEMPTYR::RXEMPTY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_0`"]
    #[inline]
    pub fn is_rxempty_0(&self) -> bool {
        *self == RXEMPTYR::RXEMPTY_0
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_1`"]
    #[inline]
    pub fn is_rxempty_1(&self) -> bool {
        *self == RXEMPTYR::RXEMPTY_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Start Of Frame"]
    #[inline]
    pub fn sof(&self) -> SOFR {
        SOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX FIFO Empty"]
    #[inline]
    pub fn rxempty(&self) -> RXEMPTYR {
        RXEMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
