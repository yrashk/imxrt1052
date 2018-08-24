#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PROT_CTRL {
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
#[doc = "Possible values of the field `LCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCTLR {
    #[doc = "LED off"]
    LCTL_0,
    #[doc = "LED on"]
    LCTL_1,
}
impl LCTLR {
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
            LCTLR::LCTL_0 => false,
            LCTLR::LCTL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCTLR {
        match value {
            false => LCTLR::LCTL_0,
            true => LCTLR::LCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCTL_0`"]
    #[inline]
    pub fn is_lctl_0(&self) -> bool {
        *self == LCTLR::LCTL_0
    }
    #[doc = "Checks if the value of the field is `LCTL_1`"]
    #[inline]
    pub fn is_lctl_1(&self) -> bool {
        *self == LCTLR::LCTL_1
    }
}
#[doc = "Possible values of the field `DTW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTWR {
    #[doc = "1-bit mode"]
    DTW_0,
    #[doc = "4-bit mode"]
    DTW_1,
    #[doc = "8-bit mode"]
    DTW_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTWR::DTW_0 => 0,
            DTWR::DTW_1 => 1,
            DTWR::DTW_2 => 2,
            DTWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTWR {
        match value {
            0 => DTWR::DTW_0,
            1 => DTWR::DTW_1,
            2 => DTWR::DTW_2,
            i => DTWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DTW_0`"]
    #[inline]
    pub fn is_dtw_0(&self) -> bool {
        *self == DTWR::DTW_0
    }
    #[doc = "Checks if the value of the field is `DTW_1`"]
    #[inline]
    pub fn is_dtw_1(&self) -> bool {
        *self == DTWR::DTW_1
    }
    #[doc = "Checks if the value of the field is `DTW_2`"]
    #[inline]
    pub fn is_dtw_2(&self) -> bool {
        *self == DTWR::DTW_2
    }
}
#[doc = "Possible values of the field `D3CD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D3CDR {
    #[doc = "DATA3 does not monitor Card Insertion"]
    D3CD_0,
    #[doc = "DATA3 as Card Detection Pin"]
    D3CD_1,
}
impl D3CDR {
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
            D3CDR::D3CD_0 => false,
            D3CDR::D3CD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D3CDR {
        match value {
            false => D3CDR::D3CD_0,
            true => D3CDR::D3CD_1,
        }
    }
    #[doc = "Checks if the value of the field is `D3CD_0`"]
    #[inline]
    pub fn is_d3cd_0(&self) -> bool {
        *self == D3CDR::D3CD_0
    }
    #[doc = "Checks if the value of the field is `D3CD_1`"]
    #[inline]
    pub fn is_d3cd_1(&self) -> bool {
        *self == D3CDR::D3CD_1
    }
}
#[doc = "Possible values of the field `EMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMODER {
    #[doc = "Big Endian Mode"]
    EMODE_0,
    #[doc = "Half Word Big Endian Mode"]
    EMODE_1,
    #[doc = "Little Endian Mode"]
    EMODE_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMODER::EMODE_0 => 0,
            EMODER::EMODE_1 => 1,
            EMODER::EMODE_2 => 2,
            EMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMODER {
        match value {
            0 => EMODER::EMODE_0,
            1 => EMODER::EMODE_1,
            2 => EMODER::EMODE_2,
            i => EMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMODE_0`"]
    #[inline]
    pub fn is_emode_0(&self) -> bool {
        *self == EMODER::EMODE_0
    }
    #[doc = "Checks if the value of the field is `EMODE_1`"]
    #[inline]
    pub fn is_emode_1(&self) -> bool {
        *self == EMODER::EMODE_1
    }
    #[doc = "Checks if the value of the field is `EMODE_2`"]
    #[inline]
    pub fn is_emode_2(&self) -> bool {
        *self == EMODER::EMODE_2
    }
}
#[doc = "Possible values of the field `CDTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTLR {
    #[doc = "Card Detect Test Level is 0, no card inserted"]
    CDTL_0,
    #[doc = "Card Detect Test Level is 1, card inserted"]
    CDTL_1,
}
impl CDTLR {
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
            CDTLR::CDTL_0 => false,
            CDTLR::CDTL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDTLR {
        match value {
            false => CDTLR::CDTL_0,
            true => CDTLR::CDTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDTL_0`"]
    #[inline]
    pub fn is_cdtl_0(&self) -> bool {
        *self == CDTLR::CDTL_0
    }
    #[doc = "Checks if the value of the field is `CDTL_1`"]
    #[inline]
    pub fn is_cdtl_1(&self) -> bool {
        *self == CDTLR::CDTL_1
    }
}
#[doc = "Possible values of the field `CDSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSSR {
    #[doc = "Card Detection Level is selected (for normal purpose)."]
    CDSS_0,
    #[doc = "Card Detection Test Level is selected (for test purpose)."]
    CDSS_1,
}
impl CDSSR {
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
            CDSSR::CDSS_0 => false,
            CDSSR::CDSS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDSSR {
        match value {
            false => CDSSR::CDSS_0,
            true => CDSSR::CDSS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDSS_0`"]
    #[inline]
    pub fn is_cdss_0(&self) -> bool {
        *self == CDSSR::CDSS_0
    }
    #[doc = "Checks if the value of the field is `CDSS_1`"]
    #[inline]
    pub fn is_cdss_1(&self) -> bool {
        *self == CDSSR::CDSS_1
    }
}
#[doc = "Possible values of the field `DMASEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASELR {
    #[doc = "No DMA or Simple DMA is selected"]
    DMASEL_0,
    #[doc = "ADMA1 is selected"]
    DMASEL_1,
    #[doc = "ADMA2 is selected"]
    DMASEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMASELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMASELR::DMASEL_0 => 0,
            DMASELR::DMASEL_1 => 1,
            DMASELR::DMASEL_2 => 2,
            DMASELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMASELR {
        match value {
            0 => DMASELR::DMASEL_0,
            1 => DMASELR::DMASEL_1,
            2 => DMASELR::DMASEL_2,
            i => DMASELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMASEL_0`"]
    #[inline]
    pub fn is_dmasel_0(&self) -> bool {
        *self == DMASELR::DMASEL_0
    }
    #[doc = "Checks if the value of the field is `DMASEL_1`"]
    #[inline]
    pub fn is_dmasel_1(&self) -> bool {
        *self == DMASELR::DMASEL_1
    }
    #[doc = "Checks if the value of the field is `DMASEL_2`"]
    #[inline]
    pub fn is_dmasel_2(&self) -> bool {
        *self == DMASELR::DMASEL_2
    }
}
#[doc = "Possible values of the field `SABGREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABGREQR {
    #[doc = "Transfer"]
    SABGREQ_0,
    #[doc = "Stop"]
    SABGREQ_1,
}
impl SABGREQR {
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
            SABGREQR::SABGREQ_0 => false,
            SABGREQR::SABGREQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SABGREQR {
        match value {
            false => SABGREQR::SABGREQ_0,
            true => SABGREQR::SABGREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SABGREQ_0`"]
    #[inline]
    pub fn is_sabgreq_0(&self) -> bool {
        *self == SABGREQR::SABGREQ_0
    }
    #[doc = "Checks if the value of the field is `SABGREQ_1`"]
    #[inline]
    pub fn is_sabgreq_1(&self) -> bool {
        *self == SABGREQR::SABGREQ_1
    }
}
#[doc = "Possible values of the field `CREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREQR {
    #[doc = "No effect"]
    CREQ_0,
    #[doc = "Restart"]
    CREQ_1,
}
impl CREQR {
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
            CREQR::CREQ_0 => false,
            CREQR::CREQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CREQR {
        match value {
            false => CREQR::CREQ_0,
            true => CREQR::CREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CREQ_0`"]
    #[inline]
    pub fn is_creq_0(&self) -> bool {
        *self == CREQR::CREQ_0
    }
    #[doc = "Checks if the value of the field is `CREQ_1`"]
    #[inline]
    pub fn is_creq_1(&self) -> bool {
        *self == CREQR::CREQ_1
    }
}
#[doc = "Possible values of the field `RWCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWCTLR {
    #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_0,
    #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_1,
}
impl RWCTLR {
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
            RWCTLR::RWCTL_0 => false,
            RWCTLR::RWCTL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWCTLR {
        match value {
            false => RWCTLR::RWCTL_0,
            true => RWCTLR::RWCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWCTL_0`"]
    #[inline]
    pub fn is_rwctl_0(&self) -> bool {
        *self == RWCTLR::RWCTL_0
    }
    #[doc = "Checks if the value of the field is `RWCTL_1`"]
    #[inline]
    pub fn is_rwctl_1(&self) -> bool {
        *self == RWCTLR::RWCTL_1
    }
}
#[doc = "Possible values of the field `IABG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IABGR {
    #[doc = "Disabled"]
    IABG_0,
    #[doc = "Enabled"]
    IABG_1,
}
impl IABGR {
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
            IABGR::IABG_0 => false,
            IABGR::IABG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IABGR {
        match value {
            false => IABGR::IABG_0,
            true => IABGR::IABG_1,
        }
    }
    #[doc = "Checks if the value of the field is `IABG_0`"]
    #[inline]
    pub fn is_iabg_0(&self) -> bool {
        *self == IABGR::IABG_0
    }
    #[doc = "Checks if the value of the field is `IABG_1`"]
    #[inline]
    pub fn is_iabg_1(&self) -> bool {
        *self == IABGR::IABG_1
    }
}
#[doc = r" Value of the field"]
pub struct RD_DONE_NO_8CLKR {
    bits: bool,
}
impl RD_DONE_NO_8CLKR {
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
#[doc = "Possible values of the field `WECINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINTR {
    #[doc = "Disable"]
    WECINT_0,
    #[doc = "Enable"]
    WECINT_1,
}
impl WECINTR {
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
            WECINTR::WECINT_0 => false,
            WECINTR::WECINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WECINTR {
        match value {
            false => WECINTR::WECINT_0,
            true => WECINTR::WECINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECINT_0`"]
    #[inline]
    pub fn is_wecint_0(&self) -> bool {
        *self == WECINTR::WECINT_0
    }
    #[doc = "Checks if the value of the field is `WECINT_1`"]
    #[inline]
    pub fn is_wecint_1(&self) -> bool {
        *self == WECINTR::WECINT_1
    }
}
#[doc = "Possible values of the field `WECINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINSR {
    #[doc = "Disable"]
    WECINS_0,
    #[doc = "Enable"]
    WECINS_1,
}
impl WECINSR {
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
            WECINSR::WECINS_0 => false,
            WECINSR::WECINS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WECINSR {
        match value {
            false => WECINSR::WECINS_0,
            true => WECINSR::WECINS_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECINS_0`"]
    #[inline]
    pub fn is_wecins_0(&self) -> bool {
        *self == WECINSR::WECINS_0
    }
    #[doc = "Checks if the value of the field is `WECINS_1`"]
    #[inline]
    pub fn is_wecins_1(&self) -> bool {
        *self == WECINSR::WECINS_1
    }
}
#[doc = "Possible values of the field `WECRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECRMR {
    #[doc = "Disable"]
    WECRM_0,
    #[doc = "Enable"]
    WECRM_1,
}
impl WECRMR {
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
            WECRMR::WECRM_0 => false,
            WECRMR::WECRM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WECRMR {
        match value {
            false => WECRMR::WECRM_0,
            true => WECRMR::WECRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECRM_0`"]
    #[inline]
    pub fn is_wecrm_0(&self) -> bool {
        *self == WECRMR::WECRM_0
    }
    #[doc = "Checks if the value of the field is `WECRM_1`"]
    #[inline]
    pub fn is_wecrm_1(&self) -> bool {
        *self == WECRMR::WECRM_1
    }
}
#[doc = "Possible values of the field `BURST_LEN_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_LEN_ENR {
    #[doc = "Burst length is enabled for INCR"]
    BURST_LEN_EN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BURST_LEN_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BURST_LEN_ENR::BURST_LEN_EN_1 => 1,
            BURST_LEN_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BURST_LEN_ENR {
        match value {
            1 => BURST_LEN_ENR::BURST_LEN_EN_1,
            i => BURST_LEN_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BURST_LEN_EN_1`"]
    #[inline]
    pub fn is_burst_len_en_1(&self) -> bool {
        *self == BURST_LEN_ENR::BURST_LEN_EN_1
    }
}
#[doc = "Possible values of the field `NON_EXACT_BLK_RD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NON_EXACT_BLK_RDR {
    #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_0,
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_1,
}
impl NON_EXACT_BLK_RDR {
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
            NON_EXACT_BLK_RDR::NON_EXACT_BLK_RD_0 => false,
            NON_EXACT_BLK_RDR::NON_EXACT_BLK_RD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NON_EXACT_BLK_RDR {
        match value {
            false => NON_EXACT_BLK_RDR::NON_EXACT_BLK_RD_0,
            true => NON_EXACT_BLK_RDR::NON_EXACT_BLK_RD_1,
        }
    }
    #[doc = "Checks if the value of the field is `NON_EXACT_BLK_RD_0`"]
    #[inline]
    pub fn is_non_exact_blk_rd_0(&self) -> bool {
        *self == NON_EXACT_BLK_RDR::NON_EXACT_BLK_RD_0
    }
    #[doc = "Checks if the value of the field is `NON_EXACT_BLK_RD_1`"]
    #[inline]
    pub fn is_non_exact_blk_rd_1(&self) -> bool {
        *self == NON_EXACT_BLK_RDR::NON_EXACT_BLK_RD_1
    }
}
#[doc = "Values that can be written to the field `LCTL`"]
pub enum LCTLW {
    #[doc = "LED off"]
    LCTL_0,
    #[doc = "LED on"]
    LCTL_1,
}
impl LCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCTLW::LCTL_0 => false,
            LCTLW::LCTL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _LCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCTLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LED off"]
    #[inline]
    pub fn lctl_0(self) -> &'a mut W {
        self.variant(LCTLW::LCTL_0)
    }
    #[doc = "LED on"]
    #[inline]
    pub fn lctl_1(self) -> &'a mut W {
        self.variant(LCTLW::LCTL_1)
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
#[doc = "Values that can be written to the field `DTW`"]
pub enum DTWW {
    #[doc = "1-bit mode"]
    DTW_0,
    #[doc = "4-bit mode"]
    DTW_1,
    #[doc = "8-bit mode"]
    DTW_2,
}
impl DTWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTWW::DTW_0 => 0,
            DTWW::DTW_1 => 1,
            DTWW::DTW_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTWW<'a> {
    w: &'a mut W,
}
impl<'a> _DTWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1-bit mode"]
    #[inline]
    pub fn dtw_0(self) -> &'a mut W {
        self.variant(DTWW::DTW_0)
    }
    #[doc = "4-bit mode"]
    #[inline]
    pub fn dtw_1(self) -> &'a mut W {
        self.variant(DTWW::DTW_1)
    }
    #[doc = "8-bit mode"]
    #[inline]
    pub fn dtw_2(self) -> &'a mut W {
        self.variant(DTWW::DTW_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `D3CD`"]
pub enum D3CDW {
    #[doc = "DATA3 does not monitor Card Insertion"]
    D3CD_0,
    #[doc = "DATA3 as Card Detection Pin"]
    D3CD_1,
}
impl D3CDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D3CDW::D3CD_0 => false,
            D3CDW::D3CD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D3CDW<'a> {
    w: &'a mut W,
}
impl<'a> _D3CDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D3CDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DATA3 does not monitor Card Insertion"]
    #[inline]
    pub fn d3cd_0(self) -> &'a mut W {
        self.variant(D3CDW::D3CD_0)
    }
    #[doc = "DATA3 as Card Detection Pin"]
    #[inline]
    pub fn d3cd_1(self) -> &'a mut W {
        self.variant(D3CDW::D3CD_1)
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
#[doc = "Values that can be written to the field `EMODE`"]
pub enum EMODEW {
    #[doc = "Big Endian Mode"]
    EMODE_0,
    #[doc = "Half Word Big Endian Mode"]
    EMODE_1,
    #[doc = "Little Endian Mode"]
    EMODE_2,
}
impl EMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMODEW::EMODE_0 => 0,
            EMODEW::EMODE_1 => 1,
            EMODEW::EMODE_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Big Endian Mode"]
    #[inline]
    pub fn emode_0(self) -> &'a mut W {
        self.variant(EMODEW::EMODE_0)
    }
    #[doc = "Half Word Big Endian Mode"]
    #[inline]
    pub fn emode_1(self) -> &'a mut W {
        self.variant(EMODEW::EMODE_1)
    }
    #[doc = "Little Endian Mode"]
    #[inline]
    pub fn emode_2(self) -> &'a mut W {
        self.variant(EMODEW::EMODE_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDTL`"]
pub enum CDTLW {
    #[doc = "Card Detect Test Level is 0, no card inserted"]
    CDTL_0,
    #[doc = "Card Detect Test Level is 1, card inserted"]
    CDTL_1,
}
impl CDTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDTLW::CDTL_0 => false,
            CDTLW::CDTL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDTLW<'a> {
    w: &'a mut W,
}
impl<'a> _CDTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDTLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card Detect Test Level is 0, no card inserted"]
    #[inline]
    pub fn cdtl_0(self) -> &'a mut W {
        self.variant(CDTLW::CDTL_0)
    }
    #[doc = "Card Detect Test Level is 1, card inserted"]
    #[inline]
    pub fn cdtl_1(self) -> &'a mut W {
        self.variant(CDTLW::CDTL_1)
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
#[doc = "Values that can be written to the field `CDSS`"]
pub enum CDSSW {
    #[doc = "Card Detection Level is selected (for normal purpose)."]
    CDSS_0,
    #[doc = "Card Detection Test Level is selected (for test purpose)."]
    CDSS_1,
}
impl CDSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDSSW::CDSS_0 => false,
            CDSSW::CDSS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDSSW<'a> {
    w: &'a mut W,
}
impl<'a> _CDSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card Detection Level is selected (for normal purpose)."]
    #[inline]
    pub fn cdss_0(self) -> &'a mut W {
        self.variant(CDSSW::CDSS_0)
    }
    #[doc = "Card Detection Test Level is selected (for test purpose)."]
    #[inline]
    pub fn cdss_1(self) -> &'a mut W {
        self.variant(CDSSW::CDSS_1)
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
#[doc = "Values that can be written to the field `DMASEL`"]
pub enum DMASELW {
    #[doc = "No DMA or Simple DMA is selected"]
    DMASEL_0,
    #[doc = "ADMA1 is selected"]
    DMASEL_1,
    #[doc = "ADMA2 is selected"]
    DMASEL_2,
}
impl DMASELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMASELW::DMASEL_0 => 0,
            DMASELW::DMASEL_1 => 1,
            DMASELW::DMASEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMASELW<'a> {
    w: &'a mut W,
}
impl<'a> _DMASELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMASELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No DMA or Simple DMA is selected"]
    #[inline]
    pub fn dmasel_0(self) -> &'a mut W {
        self.variant(DMASELW::DMASEL_0)
    }
    #[doc = "ADMA1 is selected"]
    #[inline]
    pub fn dmasel_1(self) -> &'a mut W {
        self.variant(DMASELW::DMASEL_1)
    }
    #[doc = "ADMA2 is selected"]
    #[inline]
    pub fn dmasel_2(self) -> &'a mut W {
        self.variant(DMASELW::DMASEL_2)
    }
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
#[doc = "Values that can be written to the field `SABGREQ`"]
pub enum SABGREQW {
    #[doc = "Transfer"]
    SABGREQ_0,
    #[doc = "Stop"]
    SABGREQ_1,
}
impl SABGREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SABGREQW::SABGREQ_0 => false,
            SABGREQW::SABGREQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SABGREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SABGREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SABGREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer"]
    #[inline]
    pub fn sabgreq_0(self) -> &'a mut W {
        self.variant(SABGREQW::SABGREQ_0)
    }
    #[doc = "Stop"]
    #[inline]
    pub fn sabgreq_1(self) -> &'a mut W {
        self.variant(SABGREQW::SABGREQ_1)
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
#[doc = "Values that can be written to the field `CREQ`"]
pub enum CREQW {
    #[doc = "No effect"]
    CREQ_0,
    #[doc = "Restart"]
    CREQ_1,
}
impl CREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CREQW::CREQ_0 => false,
            CREQW::CREQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CREQW<'a> {
    w: &'a mut W,
}
impl<'a> _CREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn creq_0(self) -> &'a mut W {
        self.variant(CREQW::CREQ_0)
    }
    #[doc = "Restart"]
    #[inline]
    pub fn creq_1(self) -> &'a mut W {
        self.variant(CREQW::CREQ_1)
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
#[doc = "Values that can be written to the field `RWCTL`"]
pub enum RWCTLW {
    #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_0,
    #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_1,
}
impl RWCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWCTLW::RWCTL_0 => false,
            RWCTLW::RWCTL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _RWCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWCTLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    #[inline]
    pub fn rwctl_0(self) -> &'a mut W {
        self.variant(RWCTLW::RWCTL_0)
    }
    #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    #[inline]
    pub fn rwctl_1(self) -> &'a mut W {
        self.variant(RWCTLW::RWCTL_1)
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
#[doc = "Values that can be written to the field `IABG`"]
pub enum IABGW {
    #[doc = "Disabled"]
    IABG_0,
    #[doc = "Enabled"]
    IABG_1,
}
impl IABGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IABGW::IABG_0 => false,
            IABGW::IABG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IABGW<'a> {
    w: &'a mut W,
}
impl<'a> _IABGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IABGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn iabg_0(self) -> &'a mut W {
        self.variant(IABGW::IABG_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn iabg_1(self) -> &'a mut W {
        self.variant(IABGW::IABG_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RD_DONE_NO_8CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _RD_DONE_NO_8CLKW<'a> {
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
#[doc = "Values that can be written to the field `WECINT`"]
pub enum WECINTW {
    #[doc = "Disable"]
    WECINT_0,
    #[doc = "Enable"]
    WECINT_1,
}
impl WECINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WECINTW::WECINT_0 => false,
            WECINTW::WECINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WECINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WECINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WECINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn wecint_0(self) -> &'a mut W {
        self.variant(WECINTW::WECINT_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn wecint_1(self) -> &'a mut W {
        self.variant(WECINTW::WECINT_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WECINS`"]
pub enum WECINSW {
    #[doc = "Disable"]
    WECINS_0,
    #[doc = "Enable"]
    WECINS_1,
}
impl WECINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WECINSW::WECINS_0 => false,
            WECINSW::WECINS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WECINSW<'a> {
    w: &'a mut W,
}
impl<'a> _WECINSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WECINSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn wecins_0(self) -> &'a mut W {
        self.variant(WECINSW::WECINS_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn wecins_1(self) -> &'a mut W {
        self.variant(WECINSW::WECINS_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WECRM`"]
pub enum WECRMW {
    #[doc = "Disable"]
    WECRM_0,
    #[doc = "Enable"]
    WECRM_1,
}
impl WECRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WECRMW::WECRM_0 => false,
            WECRMW::WECRM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WECRMW<'a> {
    w: &'a mut W,
}
impl<'a> _WECRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WECRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn wecrm_0(self) -> &'a mut W {
        self.variant(WECRMW::WECRM_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn wecrm_1(self) -> &'a mut W {
        self.variant(WECRMW::WECRM_1)
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
#[doc = "Values that can be written to the field `BURST_LEN_EN`"]
pub enum BURST_LEN_ENW {
    #[doc = "Burst length is enabled for INCR"]
    BURST_LEN_EN_1,
}
impl BURST_LEN_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BURST_LEN_ENW::BURST_LEN_EN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURST_LEN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BURST_LEN_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURST_LEN_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Burst length is enabled for INCR"]
    #[inline]
    pub fn burst_len_en_1(self) -> &'a mut W {
        self.variant(BURST_LEN_ENW::BURST_LEN_EN_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NON_EXACT_BLK_RD`"]
pub enum NON_EXACT_BLK_RDW {
    #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_0,
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_1,
}
impl NON_EXACT_BLK_RDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NON_EXACT_BLK_RDW::NON_EXACT_BLK_RD_0 => false,
            NON_EXACT_BLK_RDW::NON_EXACT_BLK_RD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NON_EXACT_BLK_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _NON_EXACT_BLK_RDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NON_EXACT_BLK_RDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    #[inline]
    pub fn non_exact_blk_rd_0(self) -> &'a mut W {
        self.variant(NON_EXACT_BLK_RDW::NON_EXACT_BLK_RD_0)
    }
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    #[inline]
    pub fn non_exact_blk_rd_1(self) -> &'a mut W {
        self.variant(NON_EXACT_BLK_RDW::NON_EXACT_BLK_RD_1)
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
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn lctl(&self) -> LCTLR {
        LCTLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline]
    pub fn dtw(&self) -> DTWR {
        DTWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - DATA3 as Card Detection Pin"]
    #[inline]
    pub fn d3cd(&self) -> D3CDR {
        D3CDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline]
    pub fn emode(&self) -> EMODER {
        EMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn cdtl(&self) -> CDTLR {
        CDTLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline]
    pub fn cdss(&self) -> CDSSR {
        CDSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline]
    pub fn dmasel(&self) -> DMASELR {
        DMASELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline]
    pub fn sabgreq(&self) -> SABGREQR {
        SABGREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline]
    pub fn creq(&self) -> CREQR {
        CREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline]
    pub fn rwctl(&self) -> RWCTLR {
        RWCTLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline]
    pub fn iabg(&self) -> IABGR {
        IABGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - RD_DONE_NO_8CLK"]
    #[inline]
    pub fn rd_done_no_8clk(&self) -> RD_DONE_NO_8CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RD_DONE_NO_8CLKR { bits }
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline]
    pub fn wecint(&self) -> WECINTR {
        WECINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline]
    pub fn wecins(&self) -> WECINSR {
        WECINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline]
    pub fn wecrm(&self) -> WECRMR {
        WECRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline]
    pub fn burst_len_en(&self) -> BURST_LEN_ENR {
        BURST_LEN_ENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - NON_EXACT_BLK_RD"]
    #[inline]
    pub fn non_exact_blk_rd(&self) -> NON_EXACT_BLK_RDR {
        NON_EXACT_BLK_RDR::_from({
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
        W { bits: 142606368 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn lctl(&mut self) -> _LCTLW {
        _LCTLW { w: self }
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline]
    pub fn dtw(&mut self) -> _DTWW {
        _DTWW { w: self }
    }
    #[doc = "Bit 3 - DATA3 as Card Detection Pin"]
    #[inline]
    pub fn d3cd(&mut self) -> _D3CDW {
        _D3CDW { w: self }
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline]
    pub fn emode(&mut self) -> _EMODEW {
        _EMODEW { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn cdtl(&mut self) -> _CDTLW {
        _CDTLW { w: self }
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline]
    pub fn cdss(&mut self) -> _CDSSW {
        _CDSSW { w: self }
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline]
    pub fn dmasel(&mut self) -> _DMASELW {
        _DMASELW { w: self }
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline]
    pub fn sabgreq(&mut self) -> _SABGREQW {
        _SABGREQW { w: self }
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline]
    pub fn creq(&mut self) -> _CREQW {
        _CREQW { w: self }
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline]
    pub fn rwctl(&mut self) -> _RWCTLW {
        _RWCTLW { w: self }
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline]
    pub fn iabg(&mut self) -> _IABGW {
        _IABGW { w: self }
    }
    #[doc = "Bit 20 - RD_DONE_NO_8CLK"]
    #[inline]
    pub fn rd_done_no_8clk(&mut self) -> _RD_DONE_NO_8CLKW {
        _RD_DONE_NO_8CLKW { w: self }
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline]
    pub fn wecint(&mut self) -> _WECINTW {
        _WECINTW { w: self }
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline]
    pub fn wecins(&mut self) -> _WECINSW {
        _WECINSW { w: self }
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline]
    pub fn wecrm(&mut self) -> _WECRMW {
        _WECRMW { w: self }
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline]
    pub fn burst_len_en(&mut self) -> _BURST_LEN_ENW {
        _BURST_LEN_ENW { w: self }
    }
    #[doc = "Bit 30 - NON_EXACT_BLK_RD"]
    #[inline]
    pub fn non_exact_blk_rd(&mut self) -> _NON_EXACT_BLK_RDW {
        _NON_EXACT_BLK_RDW { w: self }
    }
}
