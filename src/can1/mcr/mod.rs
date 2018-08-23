#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
pub struct MAXMBR {
    bits: u8,
}
impl MAXMBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IDAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDAMR {
    #[doc = "Format A One full ID (standard or extended) per ID filter Table element."]
    IDAM_0,
    #[doc = "Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
    IDAM_1,
    #[doc = "Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
    IDAM_2,
    #[doc = "Format D All frames rejected."]
    IDAM_3,
}
impl IDAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDAMR::IDAM_0 => 0,
            IDAMR::IDAM_1 => 1,
            IDAMR::IDAM_2 => 2,
            IDAMR::IDAM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDAMR {
        match value {
            0 => IDAMR::IDAM_0,
            1 => IDAMR::IDAM_1,
            2 => IDAMR::IDAM_2,
            3 => IDAMR::IDAM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDAM_0`"]
    #[inline]
    pub fn is_idam_0(&self) -> bool {
        *self == IDAMR::IDAM_0
    }
    #[doc = "Checks if the value of the field is `IDAM_1`"]
    #[inline]
    pub fn is_idam_1(&self) -> bool {
        *self == IDAMR::IDAM_1
    }
    #[doc = "Checks if the value of the field is `IDAM_2`"]
    #[inline]
    pub fn is_idam_2(&self) -> bool {
        *self == IDAMR::IDAM_2
    }
    #[doc = "Checks if the value of the field is `IDAM_3`"]
    #[inline]
    pub fn is_idam_3(&self) -> bool {
        *self == IDAMR::IDAM_3
    }
}
#[doc = "Possible values of the field `AEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AENR {
    #[doc = "Abort disabled"]
    AEN_0,
    #[doc = "Abort enabled"]
    AEN_1,
}
impl AENR {
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
            AENR::AEN_0 => false,
            AENR::AEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AENR {
        match value {
            false => AENR::AEN_0,
            true => AENR::AEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AEN_0`"]
    #[inline]
    pub fn is_aen_0(&self) -> bool {
        *self == AENR::AEN_0
    }
    #[doc = "Checks if the value of the field is `AEN_1`"]
    #[inline]
    pub fn is_aen_1(&self) -> bool {
        *self == AENR::AEN_1
    }
}
#[doc = "Possible values of the field `LPRIOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRIOENR {
    #[doc = "Local Priority disabled"]
    LPRIOEN_0,
    #[doc = "Local Priority enabled"]
    LPRIOEN_1,
}
impl LPRIOENR {
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
            LPRIOENR::LPRIOEN_0 => false,
            LPRIOENR::LPRIOEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPRIOENR {
        match value {
            false => LPRIOENR::LPRIOEN_0,
            true => LPRIOENR::LPRIOEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPRIOEN_0`"]
    #[inline]
    pub fn is_lprioen_0(&self) -> bool {
        *self == LPRIOENR::LPRIOEN_0
    }
    #[doc = "Checks if the value of the field is `LPRIOEN_1`"]
    #[inline]
    pub fn is_lprioen_1(&self) -> bool {
        *self == LPRIOENR::LPRIOEN_1
    }
}
#[doc = "Possible values of the field `IRMQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRMQR {
    #[doc = "Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    IRMQ_0,
    #[doc = "Individual Rx masking and queue feature are enabled."]
    IRMQ_1,
}
impl IRMQR {
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
            IRMQR::IRMQ_0 => false,
            IRMQR::IRMQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRMQR {
        match value {
            false => IRMQR::IRMQ_0,
            true => IRMQR::IRMQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRMQ_0`"]
    #[inline]
    pub fn is_irmq_0(&self) -> bool {
        *self == IRMQR::IRMQ_0
    }
    #[doc = "Checks if the value of the field is `IRMQ_1`"]
    #[inline]
    pub fn is_irmq_1(&self) -> bool {
        *self == IRMQR::IRMQ_1
    }
}
#[doc = "Possible values of the field `SRXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRXDISR {
    #[doc = "Self reception enabled"]
    SRXDIS_0,
    #[doc = "Self reception disabled"]
    SRXDIS_1,
}
impl SRXDISR {
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
            SRXDISR::SRXDIS_0 => false,
            SRXDISR::SRXDIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRXDISR {
        match value {
            false => SRXDISR::SRXDIS_0,
            true => SRXDISR::SRXDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRXDIS_0`"]
    #[inline]
    pub fn is_srxdis_0(&self) -> bool {
        *self == SRXDISR::SRXDIS_0
    }
    #[doc = "Checks if the value of the field is `SRXDIS_1`"]
    #[inline]
    pub fn is_srxdis_1(&self) -> bool {
        *self == SRXDISR::SRXDIS_1
    }
}
#[doc = "Possible values of the field `WAKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKSRCR {
    #[doc = "FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
    WAKSRC_0,
    #[doc = "FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus"]
    WAKSRC_1,
}
impl WAKSRCR {
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
            WAKSRCR::WAKSRC_0 => false,
            WAKSRCR::WAKSRC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKSRCR {
        match value {
            false => WAKSRCR::WAKSRC_0,
            true => WAKSRCR::WAKSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKSRC_0`"]
    #[inline]
    pub fn is_waksrc_0(&self) -> bool {
        *self == WAKSRCR::WAKSRC_0
    }
    #[doc = "Checks if the value of the field is `WAKSRC_1`"]
    #[inline]
    pub fn is_waksrc_1(&self) -> bool {
        *self == WAKSRCR::WAKSRC_1
    }
}
#[doc = "Possible values of the field `LPMACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACKR {
    #[doc = "FLEXCAN not in any of the low power modes"]
    LPMACK_0,
    #[doc = "FLEXCAN is either in Disable Mode, or Stop mode"]
    LPMACK_1,
}
impl LPMACKR {
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
            LPMACKR::LPMACK_0 => false,
            LPMACKR::LPMACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPMACKR {
        match value {
            false => LPMACKR::LPMACK_0,
            true => LPMACKR::LPMACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPMACK_0`"]
    #[inline]
    pub fn is_lpmack_0(&self) -> bool {
        *self == LPMACKR::LPMACK_0
    }
    #[doc = "Checks if the value of the field is `LPMACK_1`"]
    #[inline]
    pub fn is_lpmack_1(&self) -> bool {
        *self == LPMACKR::LPMACK_1
    }
}
#[doc = "Possible values of the field `WRNEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRNENR {
    #[doc = "TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters."]
    WRNEN_0,
    #[doc = "TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96."]
    WRNEN_1,
}
impl WRNENR {
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
            WRNENR::WRNEN_0 => false,
            WRNENR::WRNEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRNENR {
        match value {
            false => WRNENR::WRNEN_0,
            true => WRNENR::WRNEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WRNEN_0`"]
    #[inline]
    pub fn is_wrnen_0(&self) -> bool {
        *self == WRNENR::WRNEN_0
    }
    #[doc = "Checks if the value of the field is `WRNEN_1`"]
    #[inline]
    pub fn is_wrnen_1(&self) -> bool {
        *self == WRNENR::WRNEN_1
    }
}
#[doc = "Possible values of the field `SLFWAK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLFWAKR {
    #[doc = "FLEXCAN Self Wake Up feature is disabled"]
    SLFWAK_0,
    #[doc = "FLEXCAN Self Wake Up feature is enabled"]
    SLFWAK_1,
}
impl SLFWAKR {
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
            SLFWAKR::SLFWAK_0 => false,
            SLFWAKR::SLFWAK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLFWAKR {
        match value {
            false => SLFWAKR::SLFWAK_0,
            true => SLFWAKR::SLFWAK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLFWAK_0`"]
    #[inline]
    pub fn is_slfwak_0(&self) -> bool {
        *self == SLFWAKR::SLFWAK_0
    }
    #[doc = "Checks if the value of the field is `SLFWAK_1`"]
    #[inline]
    pub fn is_slfwak_1(&self) -> bool {
        *self == SLFWAKR::SLFWAK_1
    }
}
#[doc = "Possible values of the field `SUPV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPVR {
    #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses"]
    SUPV_0,
    #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location"]
    SUPV_1,
}
impl SUPVR {
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
            SUPVR::SUPV_0 => false,
            SUPVR::SUPV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUPVR {
        match value {
            false => SUPVR::SUPV_0,
            true => SUPVR::SUPV_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUPV_0`"]
    #[inline]
    pub fn is_supv_0(&self) -> bool {
        *self == SUPVR::SUPV_0
    }
    #[doc = "Checks if the value of the field is `SUPV_1`"]
    #[inline]
    pub fn is_supv_1(&self) -> bool {
        *self == SUPVR::SUPV_1
    }
}
#[doc = "Possible values of the field `FRZACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZACKR {
    #[doc = "FLEXCAN not in Freeze Mode, prescaler running"]
    FRZACK_0,
    #[doc = "FLEXCAN in Freeze Mode, prescaler stopped"]
    FRZACK_1,
}
impl FRZACKR {
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
            FRZACKR::FRZACK_0 => false,
            FRZACKR::FRZACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRZACKR {
        match value {
            false => FRZACKR::FRZACK_0,
            true => FRZACKR::FRZACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRZACK_0`"]
    #[inline]
    pub fn is_frzack_0(&self) -> bool {
        *self == FRZACKR::FRZACK_0
    }
    #[doc = "Checks if the value of the field is `FRZACK_1`"]
    #[inline]
    pub fn is_frzack_1(&self) -> bool {
        *self == FRZACKR::FRZACK_1
    }
}
#[doc = "Possible values of the field `SOFTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRSTR {
    #[doc = "No reset request"]
    SOFTRST_0,
    #[doc = "Reset the registers"]
    SOFTRST_1,
}
impl SOFTRSTR {
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
            SOFTRSTR::SOFTRST_0 => false,
            SOFTRSTR::SOFTRST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFTRSTR {
        match value {
            false => SOFTRSTR::SOFTRST_0,
            true => SOFTRSTR::SOFTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTRST_0`"]
    #[inline]
    pub fn is_softrst_0(&self) -> bool {
        *self == SOFTRSTR::SOFTRST_0
    }
    #[doc = "Checks if the value of the field is `SOFTRST_1`"]
    #[inline]
    pub fn is_softrst_1(&self) -> bool {
        *self == SOFTRSTR::SOFTRST_1
    }
}
#[doc = "Possible values of the field `WAKMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKMSKR {
    #[doc = "Wake Up Interrupt is disabled"]
    WAKMSK_0,
    #[doc = "Wake Up Interrupt is enabled"]
    WAKMSK_1,
}
impl WAKMSKR {
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
            WAKMSKR::WAKMSK_0 => false,
            WAKMSKR::WAKMSK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKMSKR {
        match value {
            false => WAKMSKR::WAKMSK_0,
            true => WAKMSKR::WAKMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKMSK_0`"]
    #[inline]
    pub fn is_wakmsk_0(&self) -> bool {
        *self == WAKMSKR::WAKMSK_0
    }
    #[doc = "Checks if the value of the field is `WAKMSK_1`"]
    #[inline]
    pub fn is_wakmsk_1(&self) -> bool {
        *self == WAKMSKR::WAKMSK_1
    }
}
#[doc = "Possible values of the field `NOTRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRDYR {
    #[doc = "FLEXCAN module is either in Normal Mode, Listen-Only Mode or Loop-Back Mode"]
    NOTRDY_0,
    #[doc = "FLEXCAN module is either in Disable Mode, Stop Mode or Freeze Mode"]
    NOTRDY_1,
}
impl NOTRDYR {
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
            NOTRDYR::NOTRDY_0 => false,
            NOTRDYR::NOTRDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOTRDYR {
        match value {
            false => NOTRDYR::NOTRDY_0,
            true => NOTRDYR::NOTRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRDY_0`"]
    #[inline]
    pub fn is_notrdy_0(&self) -> bool {
        *self == NOTRDYR::NOTRDY_0
    }
    #[doc = "Checks if the value of the field is `NOTRDY_1`"]
    #[inline]
    pub fn is_notrdy_1(&self) -> bool {
        *self == NOTRDYR::NOTRDY_1
    }
}
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTR {
    #[doc = "No Freeze Mode request."]
    HALT_0,
    #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
    HALT_1,
}
impl HALTR {
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
            HALTR::HALT_0 => false,
            HALTR::HALT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HALTR {
        match value {
            false => HALTR::HALT_0,
            true => HALTR::HALT_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_0`"]
    #[inline]
    pub fn is_halt_0(&self) -> bool {
        *self == HALTR::HALT_0
    }
    #[doc = "Checks if the value of the field is `HALT_1`"]
    #[inline]
    pub fn is_halt_1(&self) -> bool {
        *self == HALTR::HALT_1
    }
}
#[doc = "Possible values of the field `RFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFENR {
    #[doc = "FIFO not enabled"]
    RFEN_0,
    #[doc = "FIFO enabled"]
    RFEN_1,
}
impl RFENR {
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
            RFENR::RFEN_0 => false,
            RFENR::RFEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFENR {
        match value {
            false => RFENR::RFEN_0,
            true => RFENR::RFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RFEN_0`"]
    #[inline]
    pub fn is_rfen_0(&self) -> bool {
        *self == RFENR::RFEN_0
    }
    #[doc = "Checks if the value of the field is `RFEN_1`"]
    #[inline]
    pub fn is_rfen_1(&self) -> bool {
        *self == RFENR::RFEN_1
    }
}
#[doc = "Possible values of the field `FRZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZR {
    #[doc = "Not enabled to enter Freeze Mode"]
    FRZ_0,
    #[doc = "Enabled to enter Freeze Mode"]
    FRZ_1,
}
impl FRZR {
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
            FRZR::FRZ_0 => false,
            FRZR::FRZ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRZR {
        match value {
            false => FRZR::FRZ_0,
            true => FRZR::FRZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRZ_0`"]
    #[inline]
    pub fn is_frz_0(&self) -> bool {
        *self == FRZR::FRZ_0
    }
    #[doc = "Checks if the value of the field is `FRZ_1`"]
    #[inline]
    pub fn is_frz_1(&self) -> bool {
        *self == FRZR::FRZ_1
    }
}
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Enable the FLEXCAN module"]
    MDIS_0,
    #[doc = "Disable the FLEXCAN module"]
    MDIS_1,
}
impl MDISR {
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
            MDISR::MDIS_0 => false,
            MDISR::MDIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDISR {
        match value {
            false => MDISR::MDIS_0,
            true => MDISR::MDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDIS_0`"]
    #[inline]
    pub fn is_mdis_0(&self) -> bool {
        *self == MDISR::MDIS_0
    }
    #[doc = "Checks if the value of the field is `MDIS_1`"]
    #[inline]
    pub fn is_mdis_1(&self) -> bool {
        *self == MDISR::MDIS_1
    }
}
#[doc = r" Proxy"]
pub struct _MAXMBW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXMBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDAM`"]
pub enum IDAMW {
    #[doc = "Format A One full ID (standard or extended) per ID filter Table element."]
    IDAM_0,
    #[doc = "Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
    IDAM_1,
    #[doc = "Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
    IDAM_2,
    #[doc = "Format D All frames rejected."]
    IDAM_3,
}
impl IDAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDAMW::IDAM_0 => 0,
            IDAMW::IDAM_1 => 1,
            IDAMW::IDAM_2 => 2,
            IDAMW::IDAM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDAMW<'a> {
    w: &'a mut W,
}
impl<'a> _IDAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Format A One full ID (standard or extended) per ID filter Table element."]
    #[inline]
    pub fn idam_0(self) -> &'a mut W {
        self.variant(IDAMW::IDAM_0)
    }
    #[doc = "Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
    #[inline]
    pub fn idam_1(self) -> &'a mut W {
        self.variant(IDAMW::IDAM_1)
    }
    #[doc = "Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
    #[inline]
    pub fn idam_2(self) -> &'a mut W {
        self.variant(IDAMW::IDAM_2)
    }
    #[doc = "Format D All frames rejected."]
    #[inline]
    pub fn idam_3(self) -> &'a mut W {
        self.variant(IDAMW::IDAM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AEN`"]
pub enum AENW {
    #[doc = "Abort disabled"]
    AEN_0,
    #[doc = "Abort enabled"]
    AEN_1,
}
impl AENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AENW::AEN_0 => false,
            AENW::AEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AENW<'a> {
    w: &'a mut W,
}
impl<'a> _AENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Abort disabled"]
    #[inline]
    pub fn aen_0(self) -> &'a mut W {
        self.variant(AENW::AEN_0)
    }
    #[doc = "Abort enabled"]
    #[inline]
    pub fn aen_1(self) -> &'a mut W {
        self.variant(AENW::AEN_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPRIOEN`"]
pub enum LPRIOENW {
    #[doc = "Local Priority disabled"]
    LPRIOEN_0,
    #[doc = "Local Priority enabled"]
    LPRIOEN_1,
}
impl LPRIOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPRIOENW::LPRIOEN_0 => false,
            LPRIOENW::LPRIOEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPRIOENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPRIOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPRIOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Local Priority disabled"]
    #[inline]
    pub fn lprioen_0(self) -> &'a mut W {
        self.variant(LPRIOENW::LPRIOEN_0)
    }
    #[doc = "Local Priority enabled"]
    #[inline]
    pub fn lprioen_1(self) -> &'a mut W {
        self.variant(LPRIOENW::LPRIOEN_1)
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
#[doc = "Values that can be written to the field `IRMQ`"]
pub enum IRMQW {
    #[doc = "Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    IRMQ_0,
    #[doc = "Individual Rx masking and queue feature are enabled."]
    IRMQ_1,
}
impl IRMQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRMQW::IRMQ_0 => false,
            IRMQW::IRMQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRMQW<'a> {
    w: &'a mut W,
}
impl<'a> _IRMQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRMQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline]
    pub fn irmq_0(self) -> &'a mut W {
        self.variant(IRMQW::IRMQ_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline]
    pub fn irmq_1(self) -> &'a mut W {
        self.variant(IRMQW::IRMQ_1)
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
#[doc = "Values that can be written to the field `SRXDIS`"]
pub enum SRXDISW {
    #[doc = "Self reception enabled"]
    SRXDIS_0,
    #[doc = "Self reception disabled"]
    SRXDIS_1,
}
impl SRXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRXDISW::SRXDIS_0 => false,
            SRXDISW::SRXDIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SRXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Self reception enabled"]
    #[inline]
    pub fn srxdis_0(self) -> &'a mut W {
        self.variant(SRXDISW::SRXDIS_0)
    }
    #[doc = "Self reception disabled"]
    #[inline]
    pub fn srxdis_1(self) -> &'a mut W {
        self.variant(SRXDISW::SRXDIS_1)
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
#[doc = "Values that can be written to the field `WAKSRC`"]
pub enum WAKSRCW {
    #[doc = "FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
    WAKSRC_0,
    #[doc = "FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus"]
    WAKSRC_1,
}
impl WAKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKSRCW::WAKSRC_0 => false,
            WAKSRCW::WAKSRC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
    #[inline]
    pub fn waksrc_0(self) -> &'a mut W {
        self.variant(WAKSRCW::WAKSRC_0)
    }
    #[doc = "FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus"]
    #[inline]
    pub fn waksrc_1(self) -> &'a mut W {
        self.variant(WAKSRCW::WAKSRC_1)
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
#[doc = "Values that can be written to the field `WRNEN`"]
pub enum WRNENW {
    #[doc = "TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters."]
    WRNEN_0,
    #[doc = "TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96."]
    WRNEN_1,
}
impl WRNENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRNENW::WRNEN_0 => false,
            WRNENW::WRNEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRNENW<'a> {
    w: &'a mut W,
}
impl<'a> _WRNENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRNENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters."]
    #[inline]
    pub fn wrnen_0(self) -> &'a mut W {
        self.variant(WRNENW::WRNEN_0)
    }
    #[doc = "TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96."]
    #[inline]
    pub fn wrnen_1(self) -> &'a mut W {
        self.variant(WRNENW::WRNEN_1)
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
#[doc = "Values that can be written to the field `SLFWAK`"]
pub enum SLFWAKW {
    #[doc = "FLEXCAN Self Wake Up feature is disabled"]
    SLFWAK_0,
    #[doc = "FLEXCAN Self Wake Up feature is enabled"]
    SLFWAK_1,
}
impl SLFWAKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLFWAKW::SLFWAK_0 => false,
            SLFWAKW::SLFWAK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLFWAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SLFWAKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLFWAKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLEXCAN Self Wake Up feature is disabled"]
    #[inline]
    pub fn slfwak_0(self) -> &'a mut W {
        self.variant(SLFWAKW::SLFWAK_0)
    }
    #[doc = "FLEXCAN Self Wake Up feature is enabled"]
    #[inline]
    pub fn slfwak_1(self) -> &'a mut W {
        self.variant(SLFWAKW::SLFWAK_1)
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
#[doc = "Values that can be written to the field `SUPV`"]
pub enum SUPVW {
    #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses"]
    SUPV_0,
    #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location"]
    SUPV_1,
}
impl SUPVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUPVW::SUPV_0 => false,
            SUPVW::SUPV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUPVW<'a> {
    w: &'a mut W,
}
impl<'a> _SUPVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUPVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses"]
    #[inline]
    pub fn supv_0(self) -> &'a mut W {
        self.variant(SUPVW::SUPV_0)
    }
    #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location"]
    #[inline]
    pub fn supv_1(self) -> &'a mut W {
        self.variant(SUPVW::SUPV_1)
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
#[doc = "Values that can be written to the field `SOFTRST`"]
pub enum SOFTRSTW {
    #[doc = "No reset request"]
    SOFTRST_0,
    #[doc = "Reset the registers"]
    SOFTRST_1,
}
impl SOFTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFTRSTW::SOFTRST_0 => false,
            SOFTRSTW::SOFTRST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset request"]
    #[inline]
    pub fn softrst_0(self) -> &'a mut W {
        self.variant(SOFTRSTW::SOFTRST_0)
    }
    #[doc = "Reset the registers"]
    #[inline]
    pub fn softrst_1(self) -> &'a mut W {
        self.variant(SOFTRSTW::SOFTRST_1)
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
#[doc = "Values that can be written to the field `WAKMSK`"]
pub enum WAKMSKW {
    #[doc = "Wake Up Interrupt is disabled"]
    WAKMSK_0,
    #[doc = "Wake Up Interrupt is enabled"]
    WAKMSK_1,
}
impl WAKMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKMSKW::WAKMSK_0 => false,
            WAKMSKW::WAKMSK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake Up Interrupt is disabled"]
    #[inline]
    pub fn wakmsk_0(self) -> &'a mut W {
        self.variant(WAKMSKW::WAKMSK_0)
    }
    #[doc = "Wake Up Interrupt is enabled"]
    #[inline]
    pub fn wakmsk_1(self) -> &'a mut W {
        self.variant(WAKMSKW::WAKMSK_1)
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
#[doc = "Values that can be written to the field `HALT`"]
pub enum HALTW {
    #[doc = "No Freeze Mode request."]
    HALT_0,
    #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
    HALT_1,
}
impl HALTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HALTW::HALT_0 => false,
            HALTW::HALT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Freeze Mode request."]
    #[inline]
    pub fn halt_0(self) -> &'a mut W {
        self.variant(HALTW::HALT_0)
    }
    #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
    #[inline]
    pub fn halt_1(self) -> &'a mut W {
        self.variant(HALTW::HALT_1)
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
#[doc = "Values that can be written to the field `RFEN`"]
pub enum RFENW {
    #[doc = "FIFO not enabled"]
    RFEN_0,
    #[doc = "FIFO enabled"]
    RFEN_1,
}
impl RFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFENW::RFEN_0 => false,
            RFENW::RFEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO not enabled"]
    #[inline]
    pub fn rfen_0(self) -> &'a mut W {
        self.variant(RFENW::RFEN_0)
    }
    #[doc = "FIFO enabled"]
    #[inline]
    pub fn rfen_1(self) -> &'a mut W {
        self.variant(RFENW::RFEN_1)
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
#[doc = "Values that can be written to the field `FRZ`"]
pub enum FRZW {
    #[doc = "Not enabled to enter Freeze Mode"]
    FRZ_0,
    #[doc = "Enabled to enter Freeze Mode"]
    FRZ_1,
}
impl FRZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRZW::FRZ_0 => false,
            FRZW::FRZ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not enabled to enter Freeze Mode"]
    #[inline]
    pub fn frz_0(self) -> &'a mut W {
        self.variant(FRZW::FRZ_0)
    }
    #[doc = "Enabled to enter Freeze Mode"]
    #[inline]
    pub fn frz_1(self) -> &'a mut W {
        self.variant(FRZW::FRZ_1)
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
#[doc = "Values that can be written to the field `MDIS`"]
pub enum MDISW {
    #[doc = "Enable the FLEXCAN module"]
    MDIS_0,
    #[doc = "Disable the FLEXCAN module"]
    MDIS_1,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::MDIS_0 => false,
            MDISW::MDIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the FLEXCAN module"]
    #[inline]
    pub fn mdis_0(self) -> &'a mut W {
        self.variant(MDISW::MDIS_0)
    }
    #[doc = "Disable the FLEXCAN module"]
    #[inline]
    pub fn mdis_1(self) -> &'a mut W {
        self.variant(MDISW::MDIS_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:6 - This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
    #[inline]
    pub fn maxmb(&self) -> MAXMBR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXMBR { bits }
    }
    #[doc = "Bits 8:9 - This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
    #[inline]
    pub fn idam(&self) -> IDAMR {
        IDAMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - This bit is supplied for backwards compatibility reasons"]
    #[inline]
    pub fn aen(&self) -> AENR {
        AENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - This bit is provided for backwards compatibility reasons"]
    #[inline]
    pub fn lprioen(&self) -> LPRIOENR {
        LPRIOENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
    #[inline]
    pub fn irmq(&self) -> IRMQR {
        IRMQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
    #[inline]
    pub fn srxdis(&self) -> SRXDISR {
        SRXDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
    #[inline]
    pub fn waksrc(&self) -> WAKSRCR {
        WAKSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode"]
    #[inline]
    pub fn lpmack(&self) -> LPMACKR {
        LPMACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
    #[inline]
    pub fn wrnen(&self) -> WRNENR {
        WRNENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
    #[inline]
    pub fn slfwak(&self) -> SLFWAKR {
        SLFWAKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
    #[inline]
    pub fn supv(&self) -> SUPVR {
        SUPVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped"]
    #[inline]
    pub fn frzack(&self) -> FRZACKR {
        FRZACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
    #[inline]
    pub fn softrst(&self) -> SOFTRSTR {
        SOFTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - This bit enables the Wake Up Interrupt generation."]
    #[inline]
    pub fn wakmsk(&self) -> WAKMSKR {
        WAKMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode"]
    #[inline]
    pub fn notrdy(&self) -> NOTRDYR {
        NOTRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
    #[inline]
    pub fn halt(&self) -> HALTR {
        HALTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - This bit controls whether the Rx FIFO feature is enabled or not"]
    #[inline]
    pub fn rfen(&self) -> RFENR {
        RFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at ARM level"]
    #[inline]
    pub fn frz(&self) -> FRZR {
        FRZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - This bit controls whether FLEXCAN is enabled or not"]
    #[inline]
    pub fn mdis(&self) -> MDISR {
        MDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1501560847 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
    #[inline]
    pub fn maxmb(&mut self) -> _MAXMBW {
        _MAXMBW { w: self }
    }
    #[doc = "Bits 8:9 - This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
    #[inline]
    pub fn idam(&mut self) -> _IDAMW {
        _IDAMW { w: self }
    }
    #[doc = "Bit 12 - This bit is supplied for backwards compatibility reasons"]
    #[inline]
    pub fn aen(&mut self) -> _AENW {
        _AENW { w: self }
    }
    #[doc = "Bit 13 - This bit is provided for backwards compatibility reasons"]
    #[inline]
    pub fn lprioen(&mut self) -> _LPRIOENW {
        _LPRIOENW { w: self }
    }
    #[doc = "Bit 16 - This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
    #[inline]
    pub fn irmq(&mut self) -> _IRMQW {
        _IRMQW { w: self }
    }
    #[doc = "Bit 17 - This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
    #[inline]
    pub fn srxdis(&mut self) -> _SRXDISW {
        _SRXDISW { w: self }
    }
    #[doc = "Bit 19 - This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
    #[inline]
    pub fn waksrc(&mut self) -> _WAKSRCW {
        _WAKSRCW { w: self }
    }
    #[doc = "Bit 21 - When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
    #[inline]
    pub fn wrnen(&mut self) -> _WRNENW {
        _WRNENW { w: self }
    }
    #[doc = "Bit 22 - This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
    #[inline]
    pub fn slfwak(&mut self) -> _SLFWAKW {
        _SLFWAKW { w: self }
    }
    #[doc = "Bit 23 - This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
    #[inline]
    pub fn supv(&mut self) -> _SUPVW {
        _SUPVW { w: self }
    }
    #[doc = "Bit 25 - When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
    #[inline]
    pub fn softrst(&mut self) -> _SOFTRSTW {
        _SOFTRSTW { w: self }
    }
    #[doc = "Bit 26 - This bit enables the Wake Up Interrupt generation."]
    #[inline]
    pub fn wakmsk(&mut self) -> _WAKMSKW {
        _WAKMSKW { w: self }
    }
    #[doc = "Bit 28 - Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
    #[doc = "Bit 29 - This bit controls whether the Rx FIFO feature is enabled or not"]
    #[inline]
    pub fn rfen(&mut self) -> _RFENW {
        _RFENW { w: self }
    }
    #[doc = "Bit 30 - The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at ARM level"]
    #[inline]
    pub fn frz(&mut self) -> _FRZW {
        _FRZW { w: self }
    }
    #[doc = "Bit 31 - This bit controls whether FLEXCAN is enabled or not"]
    #[inline]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
}
