#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPSR {
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
#[doc = "Possible values of the field `LPTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTAR {
    #[doc = "No time alarm interrupt occurred."]
    LPTA_0,
    #[doc = "A time alarm interrupt occurred."]
    LPTA_1,
}
impl LPTAR {
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
            LPTAR::LPTA_0 => false,
            LPTAR::LPTA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPTAR {
        match value {
            false => LPTAR::LPTA_0,
            true => LPTAR::LPTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTA_0`"]
    #[inline]
    pub fn is_lpta_0(&self) -> bool {
        *self == LPTAR::LPTA_0
    }
    #[doc = "Checks if the value of the field is `LPTA_1`"]
    #[inline]
    pub fn is_lpta_1(&self) -> bool {
        *self == LPTAR::LPTA_1
    }
}
#[doc = "Possible values of the field `SRTCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTCRR {
    #[doc = "SRTC has not reached its maximum value."]
    SRTCR_0,
    #[doc = "SRTC has reached its maximum value."]
    SRTCR_1,
}
impl SRTCRR {
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
            SRTCRR::SRTCR_0 => false,
            SRTCRR::SRTCR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRTCRR {
        match value {
            false => SRTCRR::SRTCR_0,
            true => SRTCRR::SRTCR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTCR_0`"]
    #[inline]
    pub fn is_srtcr_0(&self) -> bool {
        *self == SRTCRR::SRTCR_0
    }
    #[doc = "Checks if the value of the field is `SRTCR_1`"]
    #[inline]
    pub fn is_srtcr_1(&self) -> bool {
        *self == SRTCRR::SRTCR_1
    }
}
#[doc = "Possible values of the field `MCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCRR {
    #[doc = "MC has not reached its maximum value."]
    MCR_0,
    #[doc = "MC has reached its maximum value."]
    MCR_1,
}
impl MCRR {
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
            MCRR::MCR_0 => false,
            MCRR::MCR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCRR {
        match value {
            false => MCRR::MCR_0,
            true => MCRR::MCR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCR_0`"]
    #[inline]
    pub fn is_mcr_0(&self) -> bool {
        *self == MCRR::MCR_0
    }
    #[doc = "Checks if the value of the field is `MCR_1`"]
    #[inline]
    pub fn is_mcr_1(&self) -> bool {
        *self == MCRR::MCR_1
    }
}
#[doc = r" Value of the field"]
pub struct PGDR {
    bits: bool,
}
impl PGDR {
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
#[doc = "Possible values of the field `ET1D`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET1DR {
    #[doc = "External tampering 1 not detected."]
    ET1D_0,
    #[doc = "External tampering 1 detected."]
    ET1D_1,
}
impl ET1DR {
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
            ET1DR::ET1D_0 => false,
            ET1DR::ET1D_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET1DR {
        match value {
            false => ET1DR::ET1D_0,
            true => ET1DR::ET1D_1,
        }
    }
    #[doc = "Checks if the value of the field is `ET1D_0`"]
    #[inline]
    pub fn is_et1d_0(&self) -> bool {
        *self == ET1DR::ET1D_0
    }
    #[doc = "Checks if the value of the field is `ET1D_1`"]
    #[inline]
    pub fn is_et1d_1(&self) -> bool {
        *self == ET1DR::ET1D_1
    }
}
#[doc = "Possible values of the field `ESVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESVDR {
    #[doc = "No external security violation."]
    ESVD_0,
    #[doc = "External security violation is detected."]
    ESVD_1,
}
impl ESVDR {
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
            ESVDR::ESVD_0 => false,
            ESVDR::ESVD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESVDR {
        match value {
            false => ESVDR::ESVD_0,
            true => ESVDR::ESVD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESVD_0`"]
    #[inline]
    pub fn is_esvd_0(&self) -> bool {
        *self == ESVDR::ESVD_0
    }
    #[doc = "Checks if the value of the field is `ESVD_1`"]
    #[inline]
    pub fn is_esvd_1(&self) -> bool {
        *self == ESVDR::ESVD_1
    }
}
#[doc = "Possible values of the field `EO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOR {
    #[doc = "Emergency off was not detected."]
    EO_0,
    #[doc = "Emergency off was detected."]
    EO_1,
}
impl EOR {
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
            EOR::EO_0 => false,
            EOR::EO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOR {
        match value {
            false => EOR::EO_0,
            true => EOR::EO_1,
        }
    }
    #[doc = "Checks if the value of the field is `EO_0`"]
    #[inline]
    pub fn is_eo_0(&self) -> bool {
        *self == EOR::EO_0
    }
    #[doc = "Checks if the value of the field is `EO_1`"]
    #[inline]
    pub fn is_eo_1(&self) -> bool {
        *self == EOR::EO_1
    }
}
#[doc = "Possible values of the field `SPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOR {
    #[doc = "Set Power Off was not detected."]
    SPO_0,
    #[doc = "Set Power Off was detected."]
    SPO_1,
}
impl SPOR {
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
            SPOR::SPO_0 => false,
            SPOR::SPO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPOR {
        match value {
            false => SPOR::SPO_0,
            true => SPOR::SPO_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPO_0`"]
    #[inline]
    pub fn is_spo_0(&self) -> bool {
        *self == SPOR::SPO_0
    }
    #[doc = "Checks if the value of the field is `SPO_1`"]
    #[inline]
    pub fn is_spo_1(&self) -> bool {
        *self == SPOR::SPO_1
    }
}
#[doc = "Possible values of the field `SED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEDR {
    #[doc = "Scan exit was not detected."]
    SED_0,
    #[doc = "Scan exit was detected."]
    SED_1,
}
impl SEDR {
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
            SEDR::SED_0 => false,
            SEDR::SED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEDR {
        match value {
            false => SEDR::SED_0,
            true => SEDR::SED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SED_0`"]
    #[inline]
    pub fn is_sed_0(&self) -> bool {
        *self == SEDR::SED_0
    }
    #[doc = "Checks if the value of the field is `SED_1`"]
    #[inline]
    pub fn is_sed_1(&self) -> bool {
        *self == SEDR::SED_1
    }
}
#[doc = "Possible values of the field `LPNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPNSR {
    #[doc = "LP section was not programmed in the non-secure state."]
    LPNS_0,
    #[doc = "LP section was programmed in the non-secure state."]
    LPNS_1,
}
impl LPNSR {
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
            LPNSR::LPNS_0 => false,
            LPNSR::LPNS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPNSR {
        match value {
            false => LPNSR::LPNS_0,
            true => LPNSR::LPNS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPNS_0`"]
    #[inline]
    pub fn is_lpns_0(&self) -> bool {
        *self == LPNSR::LPNS_0
    }
    #[doc = "Checks if the value of the field is `LPNS_1`"]
    #[inline]
    pub fn is_lpns_1(&self) -> bool {
        *self == LPNSR::LPNS_1
    }
}
#[doc = "Possible values of the field `LPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSR {
    #[doc = "LP section was not programmed in secure or trusted state."]
    LPS_0,
    #[doc = "LP section was programmed in secure or trusted state."]
    LPS_1,
}
impl LPSR {
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
            LPSR::LPS_0 => false,
            LPSR::LPS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSR {
        match value {
            false => LPSR::LPS_0,
            true => LPSR::LPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPS_0`"]
    #[inline]
    pub fn is_lps_0(&self) -> bool {
        *self == LPSR::LPS_0
    }
    #[doc = "Checks if the value of the field is `LPS_1`"]
    #[inline]
    pub fn is_lps_1(&self) -> bool {
        *self == LPSR::LPS_1
    }
}
#[doc = "Values that can be written to the field `LPTA`"]
pub enum LPTAW {
    #[doc = "No time alarm interrupt occurred."]
    LPTA_0,
    #[doc = "A time alarm interrupt occurred."]
    LPTA_1,
}
impl LPTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPTAW::LPTA_0 => false,
            LPTAW::LPTA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPTAW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time alarm interrupt occurred."]
    #[inline]
    pub fn lpta_0(self) -> &'a mut W {
        self.variant(LPTAW::LPTA_0)
    }
    #[doc = "A time alarm interrupt occurred."]
    #[inline]
    pub fn lpta_1(self) -> &'a mut W {
        self.variant(LPTAW::LPTA_1)
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
#[doc = "Values that can be written to the field `SRTCR`"]
pub enum SRTCRW {
    #[doc = "SRTC has not reached its maximum value."]
    SRTCR_0,
    #[doc = "SRTC has reached its maximum value."]
    SRTCR_1,
}
impl SRTCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRTCRW::SRTCR_0 => false,
            SRTCRW::SRTCR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRTCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRTCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRTCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRTC has not reached its maximum value."]
    #[inline]
    pub fn srtcr_0(self) -> &'a mut W {
        self.variant(SRTCRW::SRTCR_0)
    }
    #[doc = "SRTC has reached its maximum value."]
    #[inline]
    pub fn srtcr_1(self) -> &'a mut W {
        self.variant(SRTCRW::SRTCR_1)
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
#[doc = "Values that can be written to the field `MCR`"]
pub enum MCRW {
    #[doc = "MC has not reached its maximum value."]
    MCR_0,
    #[doc = "MC has reached its maximum value."]
    MCR_1,
}
impl MCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCRW::MCR_0 => false,
            MCRW::MCR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCRW<'a> {
    w: &'a mut W,
}
impl<'a> _MCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MC has not reached its maximum value."]
    #[inline]
    pub fn mcr_0(self) -> &'a mut W {
        self.variant(MCRW::MCR_0)
    }
    #[doc = "MC has reached its maximum value."]
    #[inline]
    pub fn mcr_1(self) -> &'a mut W {
        self.variant(MCRW::MCR_1)
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
#[doc = r" Proxy"]
pub struct _PGDW<'a> {
    w: &'a mut W,
}
impl<'a> _PGDW<'a> {
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
#[doc = "Values that can be written to the field `ET1D`"]
pub enum ET1DW {
    #[doc = "External tampering 1 not detected."]
    ET1D_0,
    #[doc = "External tampering 1 detected."]
    ET1D_1,
}
impl ET1DW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET1DW::ET1D_0 => false,
            ET1DW::ET1D_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET1DW<'a> {
    w: &'a mut W,
}
impl<'a> _ET1DW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET1DW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External tampering 1 not detected."]
    #[inline]
    pub fn et1d_0(self) -> &'a mut W {
        self.variant(ET1DW::ET1D_0)
    }
    #[doc = "External tampering 1 detected."]
    #[inline]
    pub fn et1d_1(self) -> &'a mut W {
        self.variant(ET1DW::ET1D_1)
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
#[doc = "Values that can be written to the field `ESVD`"]
pub enum ESVDW {
    #[doc = "No external security violation."]
    ESVD_0,
    #[doc = "External security violation is detected."]
    ESVD_1,
}
impl ESVDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESVDW::ESVD_0 => false,
            ESVDW::ESVD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESVDW<'a> {
    w: &'a mut W,
}
impl<'a> _ESVDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESVDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No external security violation."]
    #[inline]
    pub fn esvd_0(self) -> &'a mut W {
        self.variant(ESVDW::ESVD_0)
    }
    #[doc = "External security violation is detected."]
    #[inline]
    pub fn esvd_1(self) -> &'a mut W {
        self.variant(ESVDW::ESVD_1)
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
#[doc = "Values that can be written to the field `EO`"]
pub enum EOW {
    #[doc = "Emergency off was not detected."]
    EO_0,
    #[doc = "Emergency off was detected."]
    EO_1,
}
impl EOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOW::EO_0 => false,
            EOW::EO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOW<'a> {
    w: &'a mut W,
}
impl<'a> _EOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Emergency off was not detected."]
    #[inline]
    pub fn eo_0(self) -> &'a mut W {
        self.variant(EOW::EO_0)
    }
    #[doc = "Emergency off was detected."]
    #[inline]
    pub fn eo_1(self) -> &'a mut W {
        self.variant(EOW::EO_1)
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
#[doc = "Values that can be written to the field `SPO`"]
pub enum SPOW {
    #[doc = "Set Power Off was not detected."]
    SPO_0,
    #[doc = "Set Power Off was detected."]
    SPO_1,
}
impl SPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOW::SPO_0 => false,
            SPOW::SPO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPOW<'a> {
    w: &'a mut W,
}
impl<'a> _SPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set Power Off was not detected."]
    #[inline]
    pub fn spo_0(self) -> &'a mut W {
        self.variant(SPOW::SPO_0)
    }
    #[doc = "Set Power Off was detected."]
    #[inline]
    pub fn spo_1(self) -> &'a mut W {
        self.variant(SPOW::SPO_1)
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
#[doc = "Values that can be written to the field `SED`"]
pub enum SEDW {
    #[doc = "Scan exit was not detected."]
    SED_0,
    #[doc = "Scan exit was detected."]
    SED_1,
}
impl SEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEDW::SED_0 => false,
            SEDW::SED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Scan exit was not detected."]
    #[inline]
    pub fn sed_0(self) -> &'a mut W {
        self.variant(SEDW::SED_0)
    }
    #[doc = "Scan exit was detected."]
    #[inline]
    pub fn sed_1(self) -> &'a mut W {
        self.variant(SEDW::SED_1)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - LP Time Alarm"]
    #[inline]
    pub fn lpta(&self) -> LPTAR {
        LPTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Secure Real Time Counter Rollover"]
    #[inline]
    pub fn srtcr(&self) -> SRTCRR {
        SRTCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Monotonic Counter Rollover"]
    #[inline]
    pub fn mcr(&self) -> MCRR {
        MCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Power Supply Glitch Detected 0 No power supply glitch. 1 Power supply glitch is detected."]
    #[inline]
    pub fn pgd(&self) -> PGDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PGDR { bits }
    }
    #[doc = "Bit 9 - External Tampering 1 Detected"]
    #[inline]
    pub fn et1d(&self) -> ET1DR {
        ET1DR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline]
    pub fn esvd(&self) -> ESVDR {
        ESVDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Emergency Off This bit is set when a power off is requested."]
    #[inline]
    pub fn eo(&self) -> EOR {
        EOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline]
    pub fn spo(&self) -> SPOR {
        SPOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Scan Exit Detected"]
    #[inline]
    pub fn sed(&self) -> SEDR {
        SEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    #[inline]
    pub fn lpns(&self) -> LPNSR {
        LPNSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    #[inline]
    pub fn lps(&self) -> LPSR {
        LPSR::_from({
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
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LP Time Alarm"]
    #[inline]
    pub fn lpta(&mut self) -> _LPTAW {
        _LPTAW { w: self }
    }
    #[doc = "Bit 1 - Secure Real Time Counter Rollover"]
    #[inline]
    pub fn srtcr(&mut self) -> _SRTCRW {
        _SRTCRW { w: self }
    }
    #[doc = "Bit 2 - Monotonic Counter Rollover"]
    #[inline]
    pub fn mcr(&mut self) -> _MCRW {
        _MCRW { w: self }
    }
    #[doc = "Bit 3 - Power Supply Glitch Detected 0 No power supply glitch. 1 Power supply glitch is detected."]
    #[inline]
    pub fn pgd(&mut self) -> _PGDW {
        _PGDW { w: self }
    }
    #[doc = "Bit 9 - External Tampering 1 Detected"]
    #[inline]
    pub fn et1d(&mut self) -> _ET1DW {
        _ET1DW { w: self }
    }
    #[doc = "Bit 16 - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline]
    pub fn esvd(&mut self) -> _ESVDW {
        _ESVDW { w: self }
    }
    #[doc = "Bit 17 - Emergency Off This bit is set when a power off is requested."]
    #[inline]
    pub fn eo(&mut self) -> _EOW {
        _EOW { w: self }
    }
    #[doc = "Bit 18 - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline]
    pub fn spo(&mut self) -> _SPOW {
        _SPOW { w: self }
    }
    #[doc = "Bit 20 - Scan Exit Detected"]
    #[inline]
    pub fn sed(&mut self) -> _SEDW {
        _SEDW { w: self }
    }
}
