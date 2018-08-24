#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPLR {
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
#[doc = "Possible values of the field `ZMK_WHL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_WHLR {
    #[doc = "Write access is allowed."]
    ZMK_WHL_0,
    #[doc = "Write access is not allowed."]
    ZMK_WHL_1,
}
impl ZMK_WHLR {
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
            ZMK_WHLR::ZMK_WHL_0 => false,
            ZMK_WHLR::ZMK_WHL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_WHLR {
        match value {
            false => ZMK_WHLR::ZMK_WHL_0,
            true => ZMK_WHLR::ZMK_WHL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_WHL_0`"]
    #[inline]
    pub fn is_zmk_whl_0(&self) -> bool {
        *self == ZMK_WHLR::ZMK_WHL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_WHL_1`"]
    #[inline]
    pub fn is_zmk_whl_1(&self) -> bool {
        *self == ZMK_WHLR::ZMK_WHL_1
    }
}
#[doc = "Possible values of the field `ZMK_RHL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_RHLR {
    #[doc = "Read access is allowed (only in software programming mode)."]
    ZMK_RHL_0,
    #[doc = "Read access is not allowed."]
    ZMK_RHL_1,
}
impl ZMK_RHLR {
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
            ZMK_RHLR::ZMK_RHL_0 => false,
            ZMK_RHLR::ZMK_RHL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_RHLR {
        match value {
            false => ZMK_RHLR::ZMK_RHL_0,
            true => ZMK_RHLR::ZMK_RHL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_RHL_0`"]
    #[inline]
    pub fn is_zmk_rhl_0(&self) -> bool {
        *self == ZMK_RHLR::ZMK_RHL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_RHL_1`"]
    #[inline]
    pub fn is_zmk_rhl_1(&self) -> bool {
        *self == ZMK_RHLR::ZMK_RHL_1
    }
}
#[doc = "Possible values of the field `SRTC_HL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_HLR {
    #[doc = "Write access is allowed."]
    SRTC_HL_0,
    #[doc = "Write access is not allowed."]
    SRTC_HL_1,
}
impl SRTC_HLR {
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
            SRTC_HLR::SRTC_HL_0 => false,
            SRTC_HLR::SRTC_HL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRTC_HLR {
        match value {
            false => SRTC_HLR::SRTC_HL_0,
            true => SRTC_HLR::SRTC_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_HL_0`"]
    #[inline]
    pub fn is_srtc_hl_0(&self) -> bool {
        *self == SRTC_HLR::SRTC_HL_0
    }
    #[doc = "Checks if the value of the field is `SRTC_HL_1`"]
    #[inline]
    pub fn is_srtc_hl_1(&self) -> bool {
        *self == SRTC_HLR::SRTC_HL_1
    }
}
#[doc = "Possible values of the field `LPCALB_HL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCALB_HLR {
    #[doc = "Write access is allowed."]
    LPCALB_HL_0,
    #[doc = "Write access is not allowed."]
    LPCALB_HL_1,
}
impl LPCALB_HLR {
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
            LPCALB_HLR::LPCALB_HL_0 => false,
            LPCALB_HLR::LPCALB_HL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPCALB_HLR {
        match value {
            false => LPCALB_HLR::LPCALB_HL_0,
            true => LPCALB_HLR::LPCALB_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_HL_0`"]
    #[inline]
    pub fn is_lpcalb_hl_0(&self) -> bool {
        *self == LPCALB_HLR::LPCALB_HL_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_HL_1`"]
    #[inline]
    pub fn is_lpcalb_hl_1(&self) -> bool {
        *self == LPCALB_HLR::LPCALB_HL_1
    }
}
#[doc = "Possible values of the field `MC_HL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MC_HLR {
    #[doc = "Write access (increment) is allowed."]
    MC_HL_0,
    #[doc = "Write access (increment) is not allowed."]
    MC_HL_1,
}
impl MC_HLR {
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
            MC_HLR::MC_HL_0 => false,
            MC_HLR::MC_HL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MC_HLR {
        match value {
            false => MC_HLR::MC_HL_0,
            true => MC_HLR::MC_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MC_HL_0`"]
    #[inline]
    pub fn is_mc_hl_0(&self) -> bool {
        *self == MC_HLR::MC_HL_0
    }
    #[doc = "Checks if the value of the field is `MC_HL_1`"]
    #[inline]
    pub fn is_mc_hl_1(&self) -> bool {
        *self == MC_HLR::MC_HL_1
    }
}
#[doc = "Possible values of the field `GPR_HL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPR_HLR {
    #[doc = "Write access is allowed."]
    GPR_HL_0,
    #[doc = "Write access is not allowed."]
    GPR_HL_1,
}
impl GPR_HLR {
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
            GPR_HLR::GPR_HL_0 => false,
            GPR_HLR::GPR_HL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPR_HLR {
        match value {
            false => GPR_HLR::GPR_HL_0,
            true => GPR_HLR::GPR_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPR_HL_0`"]
    #[inline]
    pub fn is_gpr_hl_0(&self) -> bool {
        *self == GPR_HLR::GPR_HL_0
    }
    #[doc = "Checks if the value of the field is `GPR_HL_1`"]
    #[inline]
    pub fn is_gpr_hl_1(&self) -> bool {
        *self == GPR_HLR::GPR_HL_1
    }
}
#[doc = "Possible values of the field `LPSVCR_HL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSVCR_HLR {
    #[doc = "Write access is allowed."]
    LPSVCR_HL_0,
    #[doc = "Write access is not allowed."]
    LPSVCR_HL_1,
}
impl LPSVCR_HLR {
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
            LPSVCR_HLR::LPSVCR_HL_0 => false,
            LPSVCR_HLR::LPSVCR_HL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSVCR_HLR {
        match value {
            false => LPSVCR_HLR::LPSVCR_HL_0,
            true => LPSVCR_HLR::LPSVCR_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSVCR_HL_0`"]
    #[inline]
    pub fn is_lpsvcr_hl_0(&self) -> bool {
        *self == LPSVCR_HLR::LPSVCR_HL_0
    }
    #[doc = "Checks if the value of the field is `LPSVCR_HL_1`"]
    #[inline]
    pub fn is_lpsvcr_hl_1(&self) -> bool {
        *self == LPSVCR_HLR::LPSVCR_HL_1
    }
}
#[doc = "Possible values of the field `LPTDCR_HL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTDCR_HLR {
    #[doc = "Write access is allowed."]
    LPTDCR_HL_0,
    #[doc = "Write access is not allowed."]
    LPTDCR_HL_1,
}
impl LPTDCR_HLR {
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
            LPTDCR_HLR::LPTDCR_HL_0 => false,
            LPTDCR_HLR::LPTDCR_HL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPTDCR_HLR {
        match value {
            false => LPTDCR_HLR::LPTDCR_HL_0,
            true => LPTDCR_HLR::LPTDCR_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTDCR_HL_0`"]
    #[inline]
    pub fn is_lptdcr_hl_0(&self) -> bool {
        *self == LPTDCR_HLR::LPTDCR_HL_0
    }
    #[doc = "Checks if the value of the field is `LPTDCR_HL_1`"]
    #[inline]
    pub fn is_lptdcr_hl_1(&self) -> bool {
        *self == LPTDCR_HLR::LPTDCR_HL_1
    }
}
#[doc = "Possible values of the field `MKS_HL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MKS_HLR {
    #[doc = "Write access is allowed."]
    MKS_HL_0,
    #[doc = "Write access is not allowed."]
    MKS_HL_1,
}
impl MKS_HLR {
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
            MKS_HLR::MKS_HL_0 => false,
            MKS_HLR::MKS_HL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MKS_HLR {
        match value {
            false => MKS_HLR::MKS_HL_0,
            true => MKS_HLR::MKS_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MKS_HL_0`"]
    #[inline]
    pub fn is_mks_hl_0(&self) -> bool {
        *self == MKS_HLR::MKS_HL_0
    }
    #[doc = "Checks if the value of the field is `MKS_HL_1`"]
    #[inline]
    pub fn is_mks_hl_1(&self) -> bool {
        *self == MKS_HLR::MKS_HL_1
    }
}
#[doc = "Values that can be written to the field `ZMK_WHL`"]
pub enum ZMK_WHLW {
    #[doc = "Write access is allowed."]
    ZMK_WHL_0,
    #[doc = "Write access is not allowed."]
    ZMK_WHL_1,
}
impl ZMK_WHLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_WHLW::ZMK_WHL_0 => false,
            ZMK_WHLW::ZMK_WHL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_WHLW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_WHLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_WHLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline]
    pub fn zmk_whl_0(self) -> &'a mut W {
        self.variant(ZMK_WHLW::ZMK_WHL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline]
    pub fn zmk_whl_1(self) -> &'a mut W {
        self.variant(ZMK_WHLW::ZMK_WHL_1)
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
#[doc = "Values that can be written to the field `ZMK_RHL`"]
pub enum ZMK_RHLW {
    #[doc = "Read access is allowed (only in software programming mode)."]
    ZMK_RHL_0,
    #[doc = "Read access is not allowed."]
    ZMK_RHL_1,
}
impl ZMK_RHLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_RHLW::ZMK_RHL_0 => false,
            ZMK_RHLW::ZMK_RHL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_RHLW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_RHLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_RHLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access is allowed (only in software programming mode)."]
    #[inline]
    pub fn zmk_rhl_0(self) -> &'a mut W {
        self.variant(ZMK_RHLW::ZMK_RHL_0)
    }
    #[doc = "Read access is not allowed."]
    #[inline]
    pub fn zmk_rhl_1(self) -> &'a mut W {
        self.variant(ZMK_RHLW::ZMK_RHL_1)
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
#[doc = "Values that can be written to the field `SRTC_HL`"]
pub enum SRTC_HLW {
    #[doc = "Write access is allowed."]
    SRTC_HL_0,
    #[doc = "Write access is not allowed."]
    SRTC_HL_1,
}
impl SRTC_HLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRTC_HLW::SRTC_HL_0 => false,
            SRTC_HLW::SRTC_HL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRTC_HLW<'a> {
    w: &'a mut W,
}
impl<'a> _SRTC_HLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRTC_HLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline]
    pub fn srtc_hl_0(self) -> &'a mut W {
        self.variant(SRTC_HLW::SRTC_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline]
    pub fn srtc_hl_1(self) -> &'a mut W {
        self.variant(SRTC_HLW::SRTC_HL_1)
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
#[doc = "Values that can be written to the field `LPCALB_HL`"]
pub enum LPCALB_HLW {
    #[doc = "Write access is allowed."]
    LPCALB_HL_0,
    #[doc = "Write access is not allowed."]
    LPCALB_HL_1,
}
impl LPCALB_HLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPCALB_HLW::LPCALB_HL_0 => false,
            LPCALB_HLW::LPCALB_HL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCALB_HLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCALB_HLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCALB_HLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline]
    pub fn lpcalb_hl_0(self) -> &'a mut W {
        self.variant(LPCALB_HLW::LPCALB_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline]
    pub fn lpcalb_hl_1(self) -> &'a mut W {
        self.variant(LPCALB_HLW::LPCALB_HL_1)
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
#[doc = "Values that can be written to the field `MC_HL`"]
pub enum MC_HLW {
    #[doc = "Write access (increment) is allowed."]
    MC_HL_0,
    #[doc = "Write access (increment) is not allowed."]
    MC_HL_1,
}
impl MC_HLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MC_HLW::MC_HL_0 => false,
            MC_HLW::MC_HL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MC_HLW<'a> {
    w: &'a mut W,
}
impl<'a> _MC_HLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MC_HLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access (increment) is allowed."]
    #[inline]
    pub fn mc_hl_0(self) -> &'a mut W {
        self.variant(MC_HLW::MC_HL_0)
    }
    #[doc = "Write access (increment) is not allowed."]
    #[inline]
    pub fn mc_hl_1(self) -> &'a mut W {
        self.variant(MC_HLW::MC_HL_1)
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
#[doc = "Values that can be written to the field `GPR_HL`"]
pub enum GPR_HLW {
    #[doc = "Write access is allowed."]
    GPR_HL_0,
    #[doc = "Write access is not allowed."]
    GPR_HL_1,
}
impl GPR_HLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPR_HLW::GPR_HL_0 => false,
            GPR_HLW::GPR_HL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPR_HLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPR_HLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPR_HLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline]
    pub fn gpr_hl_0(self) -> &'a mut W {
        self.variant(GPR_HLW::GPR_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline]
    pub fn gpr_hl_1(self) -> &'a mut W {
        self.variant(GPR_HLW::GPR_HL_1)
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
#[doc = "Values that can be written to the field `LPSVCR_HL`"]
pub enum LPSVCR_HLW {
    #[doc = "Write access is allowed."]
    LPSVCR_HL_0,
    #[doc = "Write access is not allowed."]
    LPSVCR_HL_1,
}
impl LPSVCR_HLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSVCR_HLW::LPSVCR_HL_0 => false,
            LPSVCR_HLW::LPSVCR_HL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSVCR_HLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSVCR_HLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSVCR_HLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline]
    pub fn lpsvcr_hl_0(self) -> &'a mut W {
        self.variant(LPSVCR_HLW::LPSVCR_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline]
    pub fn lpsvcr_hl_1(self) -> &'a mut W {
        self.variant(LPSVCR_HLW::LPSVCR_HL_1)
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
#[doc = "Values that can be written to the field `LPTDCR_HL`"]
pub enum LPTDCR_HLW {
    #[doc = "Write access is allowed."]
    LPTDCR_HL_0,
    #[doc = "Write access is not allowed."]
    LPTDCR_HL_1,
}
impl LPTDCR_HLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPTDCR_HLW::LPTDCR_HL_0 => false,
            LPTDCR_HLW::LPTDCR_HL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPTDCR_HLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTDCR_HLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTDCR_HLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline]
    pub fn lptdcr_hl_0(self) -> &'a mut W {
        self.variant(LPTDCR_HLW::LPTDCR_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline]
    pub fn lptdcr_hl_1(self) -> &'a mut W {
        self.variant(LPTDCR_HLW::LPTDCR_HL_1)
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
#[doc = "Values that can be written to the field `MKS_HL`"]
pub enum MKS_HLW {
    #[doc = "Write access is allowed."]
    MKS_HL_0,
    #[doc = "Write access is not allowed."]
    MKS_HL_1,
}
impl MKS_HLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MKS_HLW::MKS_HL_0 => false,
            MKS_HLW::MKS_HL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MKS_HLW<'a> {
    w: &'a mut W,
}
impl<'a> _MKS_HLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MKS_HLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline]
    pub fn mks_hl_0(self) -> &'a mut W {
        self.variant(MKS_HLW::MKS_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline]
    pub fn mks_hl_1(self) -> &'a mut W {
        self.variant(MKS_HLW::MKS_HL_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline]
    pub fn zmk_whl(&self) -> ZMK_WHLR {
        ZMK_WHLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline]
    pub fn zmk_rhl(&self) -> ZMK_RHLR {
        ZMK_RHLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline]
    pub fn srtc_hl(&self) -> SRTC_HLR {
        SRTC_HLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline]
    pub fn lpcalb_hl(&self) -> LPCALB_HLR {
        LPCALB_HLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline]
    pub fn mc_hl(&self) -> MC_HLR {
        MC_HLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline]
    pub fn gpr_hl(&self) -> GPR_HLR {
        GPR_HLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline]
    pub fn lpsvcr_hl(&self) -> LPSVCR_HLR {
        LPSVCR_HLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Hard Lock When set, prevents any writes to the LPTDCR"]
    #[inline]
    pub fn lptdcr_hl(&self) -> LPTDCR_HLR {
        LPTDCR_HLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline]
    pub fn mks_hl(&self) -> MKS_HLR {
        MKS_HLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline]
    pub fn zmk_whl(&mut self) -> _ZMK_WHLW {
        _ZMK_WHLW { w: self }
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline]
    pub fn zmk_rhl(&mut self) -> _ZMK_RHLW {
        _ZMK_RHLW { w: self }
    }
    #[doc = "Bit 2 - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline]
    pub fn srtc_hl(&mut self) -> _SRTC_HLW {
        _SRTC_HLW { w: self }
    }
    #[doc = "Bit 3 - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline]
    pub fn lpcalb_hl(&mut self) -> _LPCALB_HLW {
        _LPCALB_HLW { w: self }
    }
    #[doc = "Bit 4 - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline]
    pub fn mc_hl(&mut self) -> _MC_HLW {
        _MC_HLW { w: self }
    }
    #[doc = "Bit 5 - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline]
    pub fn gpr_hl(&mut self) -> _GPR_HLW {
        _GPR_HLW { w: self }
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline]
    pub fn lpsvcr_hl(&mut self) -> _LPSVCR_HLW {
        _LPSVCR_HLW { w: self }
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Hard Lock When set, prevents any writes to the LPTDCR"]
    #[inline]
    pub fn lptdcr_hl(&mut self) -> _LPTDCR_HLW {
        _LPTDCR_HLW { w: self }
    }
    #[doc = "Bit 9 - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline]
    pub fn mks_hl(&mut self) -> _MKS_HLW {
        _MKS_HLW { w: self }
    }
}
