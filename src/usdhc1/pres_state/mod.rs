#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRES_STATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CIHB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIHBR {
    #[doc = "Can issue command using only CMD line"]
    CIHB_0,
    #[doc = "Cannot issue command"]
    CIHB_1,
}
impl CIHBR {
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
            CIHBR::CIHB_0 => false,
            CIHBR::CIHB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIHBR {
        match value {
            false => CIHBR::CIHB_0,
            true => CIHBR::CIHB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIHB_0`"]
    #[inline]
    pub fn is_cihb_0(&self) -> bool {
        *self == CIHBR::CIHB_0
    }
    #[doc = "Checks if the value of the field is `CIHB_1`"]
    #[inline]
    pub fn is_cihb_1(&self) -> bool {
        *self == CIHBR::CIHB_1
    }
}
#[doc = "Possible values of the field `CDIHB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIHBR {
    #[doc = "Can issue command which uses the DATA line"]
    CDIHB_0,
    #[doc = "Cannot issue command which uses the DATA line"]
    CDIHB_1,
}
impl CDIHBR {
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
            CDIHBR::CDIHB_0 => false,
            CDIHBR::CDIHB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDIHBR {
        match value {
            false => CDIHBR::CDIHB_0,
            true => CDIHBR::CDIHB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDIHB_0`"]
    #[inline]
    pub fn is_cdihb_0(&self) -> bool {
        *self == CDIHBR::CDIHB_0
    }
    #[doc = "Checks if the value of the field is `CDIHB_1`"]
    #[inline]
    pub fn is_cdihb_1(&self) -> bool {
        *self == CDIHBR::CDIHB_1
    }
}
#[doc = "Possible values of the field `DLA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLAR {
    #[doc = "DATA Line Inactive"]
    DLA_0,
    #[doc = "DATA Line Active"]
    DLA_1,
}
impl DLAR {
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
            DLAR::DLA_0 => false,
            DLAR::DLA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLAR {
        match value {
            false => DLAR::DLA_0,
            true => DLAR::DLA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DLA_0`"]
    #[inline]
    pub fn is_dla_0(&self) -> bool {
        *self == DLAR::DLA_0
    }
    #[doc = "Checks if the value of the field is `DLA_1`"]
    #[inline]
    pub fn is_dla_1(&self) -> bool {
        *self == DLAR::DLA_1
    }
}
#[doc = "Possible values of the field `SDSTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDSTBR {
    #[doc = "Clock is changing frequency and not stable."]
    SDSTB_0,
    #[doc = "Clock is stable."]
    SDSTB_1,
}
impl SDSTBR {
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
            SDSTBR::SDSTB_0 => false,
            SDSTBR::SDSTB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDSTBR {
        match value {
            false => SDSTBR::SDSTB_0,
            true => SDSTBR::SDSTB_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDSTB_0`"]
    #[inline]
    pub fn is_sdstb_0(&self) -> bool {
        *self == SDSTBR::SDSTB_0
    }
    #[doc = "Checks if the value of the field is `SDSTB_1`"]
    #[inline]
    pub fn is_sdstb_1(&self) -> bool {
        *self == SDSTBR::SDSTB_1
    }
}
#[doc = "Possible values of the field `IPGOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGOFFR {
    #[doc = "IPG_CLK is active."]
    IPGOFF_0,
    #[doc = "IPG_CLK is gated off."]
    IPGOFF_1,
}
impl IPGOFFR {
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
            IPGOFFR::IPGOFF_0 => false,
            IPGOFFR::IPGOFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPGOFFR {
        match value {
            false => IPGOFFR::IPGOFF_0,
            true => IPGOFFR::IPGOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPGOFF_0`"]
    #[inline]
    pub fn is_ipgoff_0(&self) -> bool {
        *self == IPGOFFR::IPGOFF_0
    }
    #[doc = "Checks if the value of the field is `IPGOFF_1`"]
    #[inline]
    pub fn is_ipgoff_1(&self) -> bool {
        *self == IPGOFFR::IPGOFF_1
    }
}
#[doc = "Possible values of the field `HCKOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCKOFFR {
    #[doc = "HCLK is active."]
    HCKOFF_0,
    #[doc = "HCLK is gated off."]
    HCKOFF_1,
}
impl HCKOFFR {
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
            HCKOFFR::HCKOFF_0 => false,
            HCKOFFR::HCKOFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCKOFFR {
        match value {
            false => HCKOFFR::HCKOFF_0,
            true => HCKOFFR::HCKOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HCKOFF_0`"]
    #[inline]
    pub fn is_hckoff_0(&self) -> bool {
        *self == HCKOFFR::HCKOFF_0
    }
    #[doc = "Checks if the value of the field is `HCKOFF_1`"]
    #[inline]
    pub fn is_hckoff_1(&self) -> bool {
        *self == HCKOFFR::HCKOFF_1
    }
}
#[doc = "Possible values of the field `PEROFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEROFFR {
    #[doc = "IPG_PERCLK is active."]
    PEROFF_0,
    #[doc = "IPG_PERCLK is gated off."]
    PEROFF_1,
}
impl PEROFFR {
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
            PEROFFR::PEROFF_0 => false,
            PEROFFR::PEROFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEROFFR {
        match value {
            false => PEROFFR::PEROFF_0,
            true => PEROFFR::PEROFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PEROFF_0`"]
    #[inline]
    pub fn is_peroff_0(&self) -> bool {
        *self == PEROFFR::PEROFF_0
    }
    #[doc = "Checks if the value of the field is `PEROFF_1`"]
    #[inline]
    pub fn is_peroff_1(&self) -> bool {
        *self == PEROFFR::PEROFF_1
    }
}
#[doc = "Possible values of the field `SDOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDOFFR {
    #[doc = "SD Clock is active."]
    SDOFF_0,
    #[doc = "SD Clock is gated off."]
    SDOFF_1,
}
impl SDOFFR {
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
            SDOFFR::SDOFF_0 => false,
            SDOFFR::SDOFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDOFFR {
        match value {
            false => SDOFFR::SDOFF_0,
            true => SDOFFR::SDOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDOFF_0`"]
    #[inline]
    pub fn is_sdoff_0(&self) -> bool {
        *self == SDOFFR::SDOFF_0
    }
    #[doc = "Checks if the value of the field is `SDOFF_1`"]
    #[inline]
    pub fn is_sdoff_1(&self) -> bool {
        *self == SDOFFR::SDOFF_1
    }
}
#[doc = "Possible values of the field `WTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTAR {
    #[doc = "No valid data"]
    WTA_0,
    #[doc = "Transferring data"]
    WTA_1,
}
impl WTAR {
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
            WTAR::WTA_0 => false,
            WTAR::WTA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTAR {
        match value {
            false => WTAR::WTA_0,
            true => WTAR::WTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WTA_0`"]
    #[inline]
    pub fn is_wta_0(&self) -> bool {
        *self == WTAR::WTA_0
    }
    #[doc = "Checks if the value of the field is `WTA_1`"]
    #[inline]
    pub fn is_wta_1(&self) -> bool {
        *self == WTAR::WTA_1
    }
}
#[doc = "Possible values of the field `RTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTAR {
    #[doc = "No valid data"]
    RTA_0,
    #[doc = "Transferring data"]
    RTA_1,
}
impl RTAR {
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
            RTAR::RTA_0 => false,
            RTAR::RTA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTAR {
        match value {
            false => RTAR::RTA_0,
            true => RTAR::RTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTA_0`"]
    #[inline]
    pub fn is_rta_0(&self) -> bool {
        *self == RTAR::RTA_0
    }
    #[doc = "Checks if the value of the field is `RTA_1`"]
    #[inline]
    pub fn is_rta_1(&self) -> bool {
        *self == RTAR::RTA_1
    }
}
#[doc = "Possible values of the field `BWEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWENR {
    #[doc = "Write disable"]
    BWEN_0,
    #[doc = "Write enable"]
    BWEN_1,
}
impl BWENR {
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
            BWENR::BWEN_0 => false,
            BWENR::BWEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWENR {
        match value {
            false => BWENR::BWEN_0,
            true => BWENR::BWEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWEN_0`"]
    #[inline]
    pub fn is_bwen_0(&self) -> bool {
        *self == BWENR::BWEN_0
    }
    #[doc = "Checks if the value of the field is `BWEN_1`"]
    #[inline]
    pub fn is_bwen_1(&self) -> bool {
        *self == BWENR::BWEN_1
    }
}
#[doc = "Possible values of the field `BREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRENR {
    #[doc = "Read disable"]
    BREN_0,
    #[doc = "Read enable"]
    BREN_1,
}
impl BRENR {
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
            BRENR::BREN_0 => false,
            BRENR::BREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRENR {
        match value {
            false => BRENR::BREN_0,
            true => BRENR::BREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BREN_0`"]
    #[inline]
    pub fn is_bren_0(&self) -> bool {
        *self == BRENR::BREN_0
    }
    #[doc = "Checks if the value of the field is `BREN_1`"]
    #[inline]
    pub fn is_bren_1(&self) -> bool {
        *self == BRENR::BREN_1
    }
}
#[doc = "Possible values of the field `RTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTRR {
    #[doc = "Fixed or well tuned sampling clock"]
    RTR_0,
    #[doc = "Sampling clock needs re-tuning"]
    RTR_1,
}
impl RTRR {
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
            RTRR::RTR_0 => false,
            RTRR::RTR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTRR {
        match value {
            false => RTRR::RTR_0,
            true => RTRR::RTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTR_0`"]
    #[inline]
    pub fn is_rtr_0(&self) -> bool {
        *self == RTRR::RTR_0
    }
    #[doc = "Checks if the value of the field is `RTR_1`"]
    #[inline]
    pub fn is_rtr_1(&self) -> bool {
        *self == RTRR::RTR_1
    }
}
#[doc = "Possible values of the field `TSCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCDR {
    #[doc = "Delay cell select change is not finished."]
    TSCD_0,
    #[doc = "Delay cell select change is finished."]
    TSCD_1,
}
impl TSCDR {
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
            TSCDR::TSCD_0 => false,
            TSCDR::TSCD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSCDR {
        match value {
            false => TSCDR::TSCD_0,
            true => TSCDR::TSCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `TSCD_0`"]
    #[inline]
    pub fn is_tscd_0(&self) -> bool {
        *self == TSCDR::TSCD_0
    }
    #[doc = "Checks if the value of the field is `TSCD_1`"]
    #[inline]
    pub fn is_tscd_1(&self) -> bool {
        *self == TSCDR::TSCD_1
    }
}
#[doc = "Possible values of the field `CINST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSTR {
    #[doc = "Power on Reset or No Card"]
    CINST_0,
    #[doc = "Card Inserted"]
    CINST_1,
}
impl CINSTR {
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
            CINSTR::CINST_0 => false,
            CINSTR::CINST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSTR {
        match value {
            false => CINSTR::CINST_0,
            true => CINSTR::CINST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINST_0`"]
    #[inline]
    pub fn is_cinst_0(&self) -> bool {
        *self == CINSTR::CINST_0
    }
    #[doc = "Checks if the value of the field is `CINST_1`"]
    #[inline]
    pub fn is_cinst_1(&self) -> bool {
        *self == CINSTR::CINST_1
    }
}
#[doc = "Possible values of the field `CDPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDPLR {
    #[doc = "No card present (CD_B = 1)"]
    CDPL_0,
    #[doc = "Card present (CD_B = 0)"]
    CDPL_1,
}
impl CDPLR {
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
            CDPLR::CDPL_0 => false,
            CDPLR::CDPL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDPLR {
        match value {
            false => CDPLR::CDPL_0,
            true => CDPLR::CDPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDPL_0`"]
    #[inline]
    pub fn is_cdpl_0(&self) -> bool {
        *self == CDPLR::CDPL_0
    }
    #[doc = "Checks if the value of the field is `CDPL_1`"]
    #[inline]
    pub fn is_cdpl_1(&self) -> bool {
        *self == CDPLR::CDPL_1
    }
}
#[doc = "Possible values of the field `WPSPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSPLR {
    #[doc = "Write protected (WP = 1)"]
    WPSPL_0,
    #[doc = "Write enabled (WP = 0)"]
    WPSPL_1,
}
impl WPSPLR {
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
            WPSPLR::WPSPL_0 => false,
            WPSPLR::WPSPL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPSPLR {
        match value {
            false => WPSPLR::WPSPL_0,
            true => WPSPLR::WPSPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPSPL_0`"]
    #[inline]
    pub fn is_wpspl_0(&self) -> bool {
        *self == WPSPLR::WPSPL_0
    }
    #[doc = "Checks if the value of the field is `WPSPL_1`"]
    #[inline]
    pub fn is_wpspl_1(&self) -> bool {
        *self == WPSPLR::WPSPL_1
    }
}
#[doc = r" Value of the field"]
pub struct CLSLR {
    bits: bool,
}
impl CLSLR {
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
#[doc = "Possible values of the field `DLSL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLSLR {
    #[doc = "Data 0 line signal level"]
    DATA0,
    #[doc = "Data 1 line signal level"]
    DATA1,
    #[doc = "Data 2 line signal level"]
    DATA2,
    #[doc = "Data 3 line signal level"]
    DATA3,
    #[doc = "Data 4 line signal level"]
    DATA4,
    #[doc = "Data 5 line signal level"]
    DATA5,
    #[doc = "Data 6 line signal level"]
    DATA6,
    #[doc = "Data 7 line signal level"]
    DATA7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DLSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DLSLR::DATA0 => 0,
            DLSLR::DATA1 => 1,
            DLSLR::DATA2 => 2,
            DLSLR::DATA3 => 3,
            DLSLR::DATA4 => 4,
            DLSLR::DATA5 => 5,
            DLSLR::DATA6 => 6,
            DLSLR::DATA7 => 7,
            DLSLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DLSLR {
        match value {
            0 => DLSLR::DATA0,
            1 => DLSLR::DATA1,
            2 => DLSLR::DATA2,
            3 => DLSLR::DATA3,
            4 => DLSLR::DATA4,
            5 => DLSLR::DATA5,
            6 => DLSLR::DATA6,
            7 => DLSLR::DATA7,
            i => DLSLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == DLSLR::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == DLSLR::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == DLSLR::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA3`"]
    #[inline]
    pub fn is_data3(&self) -> bool {
        *self == DLSLR::DATA3
    }
    #[doc = "Checks if the value of the field is `DATA4`"]
    #[inline]
    pub fn is_data4(&self) -> bool {
        *self == DLSLR::DATA4
    }
    #[doc = "Checks if the value of the field is `DATA5`"]
    #[inline]
    pub fn is_data5(&self) -> bool {
        *self == DLSLR::DATA5
    }
    #[doc = "Checks if the value of the field is `DATA6`"]
    #[inline]
    pub fn is_data6(&self) -> bool {
        *self == DLSLR::DATA6
    }
    #[doc = "Checks if the value of the field is `DATA7`"]
    #[inline]
    pub fn is_data7(&self) -> bool {
        *self == DLSLR::DATA7
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline]
    pub fn cihb(&self) -> CIHBR {
        CIHBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Command Inhibit (DATA)"]
    #[inline]
    pub fn cdihb(&self) -> CDIHBR {
        CDIHBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Data Line Active"]
    #[inline]
    pub fn dla(&self) -> DLAR {
        DLAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - SD Clock Stable"]
    #[inline]
    pub fn sdstb(&self) -> SDSTBR {
        SDSTBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IPG_CLK Gated Off Internally"]
    #[inline]
    pub fn ipgoff(&self) -> IPGOFFR {
        IPGOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - HCLK Gated Off Internally"]
    #[inline]
    pub fn hckoff(&self) -> HCKOFFR {
        HCKOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - IPG_PERCLK Gated Off Internally"]
    #[inline]
    pub fn peroff(&self) -> PEROFFR {
        PEROFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SD Clock Gated Off Internally"]
    #[inline]
    pub fn sdoff(&self) -> SDOFFR {
        SDOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline]
    pub fn wta(&self) -> WTAR {
        WTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline]
    pub fn rta(&self) -> RTAR {
        RTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline]
    pub fn bwen(&self) -> BWENR {
        BWENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline]
    pub fn bren(&self) -> BRENR {
        BRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn rtr(&self) -> RTRR {
        RTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Tape Select Change Done"]
    #[inline]
    pub fn tscd(&self) -> TSCDR {
        TSCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline]
    pub fn cinst(&self) -> CINSTR {
        CINSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline]
    pub fn cdpl(&self) -> CDPLR {
        CDPLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline]
    pub fn wpspl(&self) -> WPSPLR {
        WPSPLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - CMD Line Signal Level"]
    #[inline]
    pub fn clsl(&self) -> CLSLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLSLR { bits }
    }
    #[doc = "Bits 24:31 - DATA[7:0] Line Signal Level"]
    #[inline]
    pub fn dlsl(&self) -> DLSLR {
        DLSLR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
