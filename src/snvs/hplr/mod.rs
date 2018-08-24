#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPLR {
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
#[doc = "Possible values of the field `ZMK_WSL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_WSLR {
    #[doc = "Write access is allowed"]
    ZMK_WSL_0,
    #[doc = "Write access is not allowed"]
    ZMK_WSL_1,
}
impl ZMK_WSLR {
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
            ZMK_WSLR::ZMK_WSL_0 => false,
            ZMK_WSLR::ZMK_WSL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_WSLR {
        match value {
            false => ZMK_WSLR::ZMK_WSL_0,
            true => ZMK_WSLR::ZMK_WSL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_WSL_0`"]
    #[inline]
    pub fn is_zmk_wsl_0(&self) -> bool {
        *self == ZMK_WSLR::ZMK_WSL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_WSL_1`"]
    #[inline]
    pub fn is_zmk_wsl_1(&self) -> bool {
        *self == ZMK_WSLR::ZMK_WSL_1
    }
}
#[doc = "Possible values of the field `ZMK_RSL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_RSLR {
    #[doc = "Read access is allowed (only in software Programming mode)"]
    ZMK_RSL_0,
    #[doc = "Read access is not allowed"]
    ZMK_RSL_1,
}
impl ZMK_RSLR {
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
            ZMK_RSLR::ZMK_RSL_0 => false,
            ZMK_RSLR::ZMK_RSL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_RSLR {
        match value {
            false => ZMK_RSLR::ZMK_RSL_0,
            true => ZMK_RSLR::ZMK_RSL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_RSL_0`"]
    #[inline]
    pub fn is_zmk_rsl_0(&self) -> bool {
        *self == ZMK_RSLR::ZMK_RSL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_RSL_1`"]
    #[inline]
    pub fn is_zmk_rsl_1(&self) -> bool {
        *self == ZMK_RSLR::ZMK_RSL_1
    }
}
#[doc = "Possible values of the field `SRTC_SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_SLR {
    #[doc = "Write access is allowed"]
    SRTC_SL_0,
    #[doc = "Write access is not allowed"]
    SRTC_SL_1,
}
impl SRTC_SLR {
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
            SRTC_SLR::SRTC_SL_0 => false,
            SRTC_SLR::SRTC_SL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRTC_SLR {
        match value {
            false => SRTC_SLR::SRTC_SL_0,
            true => SRTC_SLR::SRTC_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_SL_0`"]
    #[inline]
    pub fn is_srtc_sl_0(&self) -> bool {
        *self == SRTC_SLR::SRTC_SL_0
    }
    #[doc = "Checks if the value of the field is `SRTC_SL_1`"]
    #[inline]
    pub fn is_srtc_sl_1(&self) -> bool {
        *self == SRTC_SLR::SRTC_SL_1
    }
}
#[doc = "Possible values of the field `LPCALB_SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCALB_SLR {
    #[doc = "Write access is allowed"]
    LPCALB_SL_0,
    #[doc = "Write access is not allowed"]
    LPCALB_SL_1,
}
impl LPCALB_SLR {
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
            LPCALB_SLR::LPCALB_SL_0 => false,
            LPCALB_SLR::LPCALB_SL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPCALB_SLR {
        match value {
            false => LPCALB_SLR::LPCALB_SL_0,
            true => LPCALB_SLR::LPCALB_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_SL_0`"]
    #[inline]
    pub fn is_lpcalb_sl_0(&self) -> bool {
        *self == LPCALB_SLR::LPCALB_SL_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_SL_1`"]
    #[inline]
    pub fn is_lpcalb_sl_1(&self) -> bool {
        *self == LPCALB_SLR::LPCALB_SL_1
    }
}
#[doc = "Possible values of the field `MC_SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MC_SLR {
    #[doc = "Write access (increment) is allowed"]
    MC_SL_0,
    #[doc = "Write access (increment) is not allowed"]
    MC_SL_1,
}
impl MC_SLR {
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
            MC_SLR::MC_SL_0 => false,
            MC_SLR::MC_SL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MC_SLR {
        match value {
            false => MC_SLR::MC_SL_0,
            true => MC_SLR::MC_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MC_SL_0`"]
    #[inline]
    pub fn is_mc_sl_0(&self) -> bool {
        *self == MC_SLR::MC_SL_0
    }
    #[doc = "Checks if the value of the field is `MC_SL_1`"]
    #[inline]
    pub fn is_mc_sl_1(&self) -> bool {
        *self == MC_SLR::MC_SL_1
    }
}
#[doc = "Possible values of the field `GPR_SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPR_SLR {
    #[doc = "Write access is allowed"]
    GPR_SL_0,
    #[doc = "Write access is not allowed"]
    GPR_SL_1,
}
impl GPR_SLR {
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
            GPR_SLR::GPR_SL_0 => false,
            GPR_SLR::GPR_SL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPR_SLR {
        match value {
            false => GPR_SLR::GPR_SL_0,
            true => GPR_SLR::GPR_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPR_SL_0`"]
    #[inline]
    pub fn is_gpr_sl_0(&self) -> bool {
        *self == GPR_SLR::GPR_SL_0
    }
    #[doc = "Checks if the value of the field is `GPR_SL_1`"]
    #[inline]
    pub fn is_gpr_sl_1(&self) -> bool {
        *self == GPR_SLR::GPR_SL_1
    }
}
#[doc = "Possible values of the field `LPSVCR_SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSVCR_SLR {
    #[doc = "Write access is allowed"]
    LPSVCR_SL_0,
    #[doc = "Write access is not allowed"]
    LPSVCR_SL_1,
}
impl LPSVCR_SLR {
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
            LPSVCR_SLR::LPSVCR_SL_0 => false,
            LPSVCR_SLR::LPSVCR_SL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPSVCR_SLR {
        match value {
            false => LPSVCR_SLR::LPSVCR_SL_0,
            true => LPSVCR_SLR::LPSVCR_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSVCR_SL_0`"]
    #[inline]
    pub fn is_lpsvcr_sl_0(&self) -> bool {
        *self == LPSVCR_SLR::LPSVCR_SL_0
    }
    #[doc = "Checks if the value of the field is `LPSVCR_SL_1`"]
    #[inline]
    pub fn is_lpsvcr_sl_1(&self) -> bool {
        *self == LPSVCR_SLR::LPSVCR_SL_1
    }
}
#[doc = "Possible values of the field `LPTDCR_SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTDCR_SLR {
    #[doc = "Write access is allowed"]
    LPTDCR_SL_0,
    #[doc = "Write access is not allowed"]
    LPTDCR_SL_1,
}
impl LPTDCR_SLR {
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
            LPTDCR_SLR::LPTDCR_SL_0 => false,
            LPTDCR_SLR::LPTDCR_SL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPTDCR_SLR {
        match value {
            false => LPTDCR_SLR::LPTDCR_SL_0,
            true => LPTDCR_SLR::LPTDCR_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTDCR_SL_0`"]
    #[inline]
    pub fn is_lptdcr_sl_0(&self) -> bool {
        *self == LPTDCR_SLR::LPTDCR_SL_0
    }
    #[doc = "Checks if the value of the field is `LPTDCR_SL_1`"]
    #[inline]
    pub fn is_lptdcr_sl_1(&self) -> bool {
        *self == LPTDCR_SLR::LPTDCR_SL_1
    }
}
#[doc = "Possible values of the field `MKS_SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MKS_SLR {
    #[doc = "Write access is allowed"]
    MKS_SL_0,
    #[doc = "Write access is not allowed"]
    MKS_SL_1,
}
impl MKS_SLR {
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
            MKS_SLR::MKS_SL_0 => false,
            MKS_SLR::MKS_SL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MKS_SLR {
        match value {
            false => MKS_SLR::MKS_SL_0,
            true => MKS_SLR::MKS_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MKS_SL_0`"]
    #[inline]
    pub fn is_mks_sl_0(&self) -> bool {
        *self == MKS_SLR::MKS_SL_0
    }
    #[doc = "Checks if the value of the field is `MKS_SL_1`"]
    #[inline]
    pub fn is_mks_sl_1(&self) -> bool {
        *self == MKS_SLR::MKS_SL_1
    }
}
#[doc = "Possible values of the field `HPSVCR_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPSVCR_LR {
    #[doc = "Write access is allowed"]
    HPSVCR_L_0,
    #[doc = "Write access is not allowed"]
    HPSVCR_L_1,
}
impl HPSVCR_LR {
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
            HPSVCR_LR::HPSVCR_L_0 => false,
            HPSVCR_LR::HPSVCR_L_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPSVCR_LR {
        match value {
            false => HPSVCR_LR::HPSVCR_L_0,
            true => HPSVCR_LR::HPSVCR_L_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPSVCR_L_0`"]
    #[inline]
    pub fn is_hpsvcr_l_0(&self) -> bool {
        *self == HPSVCR_LR::HPSVCR_L_0
    }
    #[doc = "Checks if the value of the field is `HPSVCR_L_1`"]
    #[inline]
    pub fn is_hpsvcr_l_1(&self) -> bool {
        *self == HPSVCR_LR::HPSVCR_L_1
    }
}
#[doc = "Possible values of the field `HPSICR_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPSICR_LR {
    #[doc = "Write access is allowed"]
    HPSICR_L_0,
    #[doc = "Write access is not allowed"]
    HPSICR_L_1,
}
impl HPSICR_LR {
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
            HPSICR_LR::HPSICR_L_0 => false,
            HPSICR_LR::HPSICR_L_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPSICR_LR {
        match value {
            false => HPSICR_LR::HPSICR_L_0,
            true => HPSICR_LR::HPSICR_L_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPSICR_L_0`"]
    #[inline]
    pub fn is_hpsicr_l_0(&self) -> bool {
        *self == HPSICR_LR::HPSICR_L_0
    }
    #[doc = "Checks if the value of the field is `HPSICR_L_1`"]
    #[inline]
    pub fn is_hpsicr_l_1(&self) -> bool {
        *self == HPSICR_LR::HPSICR_L_1
    }
}
#[doc = "Possible values of the field `HAC_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAC_LR {
    #[doc = "Write access is allowed"]
    HAC_L_0,
    #[doc = "Write access is not allowed"]
    HAC_L_1,
}
impl HAC_LR {
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
            HAC_LR::HAC_L_0 => false,
            HAC_LR::HAC_L_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HAC_LR {
        match value {
            false => HAC_LR::HAC_L_0,
            true => HAC_LR::HAC_L_1,
        }
    }
    #[doc = "Checks if the value of the field is `HAC_L_0`"]
    #[inline]
    pub fn is_hac_l_0(&self) -> bool {
        *self == HAC_LR::HAC_L_0
    }
    #[doc = "Checks if the value of the field is `HAC_L_1`"]
    #[inline]
    pub fn is_hac_l_1(&self) -> bool {
        *self == HAC_LR::HAC_L_1
    }
}
#[doc = "Values that can be written to the field `ZMK_WSL`"]
pub enum ZMK_WSLW {
    #[doc = "Write access is allowed"]
    ZMK_WSL_0,
    #[doc = "Write access is not allowed"]
    ZMK_WSL_1,
}
impl ZMK_WSLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_WSLW::ZMK_WSL_0 => false,
            ZMK_WSLW::ZMK_WSL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_WSLW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_WSLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_WSLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn zmk_wsl_0(self) -> &'a mut W {
        self.variant(ZMK_WSLW::ZMK_WSL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn zmk_wsl_1(self) -> &'a mut W {
        self.variant(ZMK_WSLW::ZMK_WSL_1)
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
#[doc = "Values that can be written to the field `ZMK_RSL`"]
pub enum ZMK_RSLW {
    #[doc = "Read access is allowed (only in software Programming mode)"]
    ZMK_RSL_0,
    #[doc = "Read access is not allowed"]
    ZMK_RSL_1,
}
impl ZMK_RSLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_RSLW::ZMK_RSL_0 => false,
            ZMK_RSLW::ZMK_RSL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_RSLW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_RSLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_RSLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read access is allowed (only in software Programming mode)"]
    #[inline]
    pub fn zmk_rsl_0(self) -> &'a mut W {
        self.variant(ZMK_RSLW::ZMK_RSL_0)
    }
    #[doc = "Read access is not allowed"]
    #[inline]
    pub fn zmk_rsl_1(self) -> &'a mut W {
        self.variant(ZMK_RSLW::ZMK_RSL_1)
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
#[doc = "Values that can be written to the field `SRTC_SL`"]
pub enum SRTC_SLW {
    #[doc = "Write access is allowed"]
    SRTC_SL_0,
    #[doc = "Write access is not allowed"]
    SRTC_SL_1,
}
impl SRTC_SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRTC_SLW::SRTC_SL_0 => false,
            SRTC_SLW::SRTC_SL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRTC_SLW<'a> {
    w: &'a mut W,
}
impl<'a> _SRTC_SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRTC_SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn srtc_sl_0(self) -> &'a mut W {
        self.variant(SRTC_SLW::SRTC_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn srtc_sl_1(self) -> &'a mut W {
        self.variant(SRTC_SLW::SRTC_SL_1)
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
#[doc = "Values that can be written to the field `LPCALB_SL`"]
pub enum LPCALB_SLW {
    #[doc = "Write access is allowed"]
    LPCALB_SL_0,
    #[doc = "Write access is not allowed"]
    LPCALB_SL_1,
}
impl LPCALB_SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPCALB_SLW::LPCALB_SL_0 => false,
            LPCALB_SLW::LPCALB_SL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCALB_SLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCALB_SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCALB_SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn lpcalb_sl_0(self) -> &'a mut W {
        self.variant(LPCALB_SLW::LPCALB_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn lpcalb_sl_1(self) -> &'a mut W {
        self.variant(LPCALB_SLW::LPCALB_SL_1)
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
#[doc = "Values that can be written to the field `MC_SL`"]
pub enum MC_SLW {
    #[doc = "Write access (increment) is allowed"]
    MC_SL_0,
    #[doc = "Write access (increment) is not allowed"]
    MC_SL_1,
}
impl MC_SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MC_SLW::MC_SL_0 => false,
            MC_SLW::MC_SL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MC_SLW<'a> {
    w: &'a mut W,
}
impl<'a> _MC_SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MC_SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access (increment) is allowed"]
    #[inline]
    pub fn mc_sl_0(self) -> &'a mut W {
        self.variant(MC_SLW::MC_SL_0)
    }
    #[doc = "Write access (increment) is not allowed"]
    #[inline]
    pub fn mc_sl_1(self) -> &'a mut W {
        self.variant(MC_SLW::MC_SL_1)
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
#[doc = "Values that can be written to the field `GPR_SL`"]
pub enum GPR_SLW {
    #[doc = "Write access is allowed"]
    GPR_SL_0,
    #[doc = "Write access is not allowed"]
    GPR_SL_1,
}
impl GPR_SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPR_SLW::GPR_SL_0 => false,
            GPR_SLW::GPR_SL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPR_SLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPR_SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPR_SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn gpr_sl_0(self) -> &'a mut W {
        self.variant(GPR_SLW::GPR_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn gpr_sl_1(self) -> &'a mut W {
        self.variant(GPR_SLW::GPR_SL_1)
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
#[doc = "Values that can be written to the field `LPSVCR_SL`"]
pub enum LPSVCR_SLW {
    #[doc = "Write access is allowed"]
    LPSVCR_SL_0,
    #[doc = "Write access is not allowed"]
    LPSVCR_SL_1,
}
impl LPSVCR_SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPSVCR_SLW::LPSVCR_SL_0 => false,
            LPSVCR_SLW::LPSVCR_SL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSVCR_SLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSVCR_SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSVCR_SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn lpsvcr_sl_0(self) -> &'a mut W {
        self.variant(LPSVCR_SLW::LPSVCR_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn lpsvcr_sl_1(self) -> &'a mut W {
        self.variant(LPSVCR_SLW::LPSVCR_SL_1)
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
#[doc = "Values that can be written to the field `LPTDCR_SL`"]
pub enum LPTDCR_SLW {
    #[doc = "Write access is allowed"]
    LPTDCR_SL_0,
    #[doc = "Write access is not allowed"]
    LPTDCR_SL_1,
}
impl LPTDCR_SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPTDCR_SLW::LPTDCR_SL_0 => false,
            LPTDCR_SLW::LPTDCR_SL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPTDCR_SLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTDCR_SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTDCR_SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn lptdcr_sl_0(self) -> &'a mut W {
        self.variant(LPTDCR_SLW::LPTDCR_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn lptdcr_sl_1(self) -> &'a mut W {
        self.variant(LPTDCR_SLW::LPTDCR_SL_1)
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
#[doc = "Values that can be written to the field `MKS_SL`"]
pub enum MKS_SLW {
    #[doc = "Write access is allowed"]
    MKS_SL_0,
    #[doc = "Write access is not allowed"]
    MKS_SL_1,
}
impl MKS_SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MKS_SLW::MKS_SL_0 => false,
            MKS_SLW::MKS_SL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MKS_SLW<'a> {
    w: &'a mut W,
}
impl<'a> _MKS_SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MKS_SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn mks_sl_0(self) -> &'a mut W {
        self.variant(MKS_SLW::MKS_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn mks_sl_1(self) -> &'a mut W {
        self.variant(MKS_SLW::MKS_SL_1)
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
#[doc = "Values that can be written to the field `HPSVCR_L`"]
pub enum HPSVCR_LW {
    #[doc = "Write access is allowed"]
    HPSVCR_L_0,
    #[doc = "Write access is not allowed"]
    HPSVCR_L_1,
}
impl HPSVCR_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPSVCR_LW::HPSVCR_L_0 => false,
            HPSVCR_LW::HPSVCR_L_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPSVCR_LW<'a> {
    w: &'a mut W,
}
impl<'a> _HPSVCR_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPSVCR_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn hpsvcr_l_0(self) -> &'a mut W {
        self.variant(HPSVCR_LW::HPSVCR_L_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn hpsvcr_l_1(self) -> &'a mut W {
        self.variant(HPSVCR_LW::HPSVCR_L_1)
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
#[doc = "Values that can be written to the field `HPSICR_L`"]
pub enum HPSICR_LW {
    #[doc = "Write access is allowed"]
    HPSICR_L_0,
    #[doc = "Write access is not allowed"]
    HPSICR_L_1,
}
impl HPSICR_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPSICR_LW::HPSICR_L_0 => false,
            HPSICR_LW::HPSICR_L_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPSICR_LW<'a> {
    w: &'a mut W,
}
impl<'a> _HPSICR_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPSICR_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn hpsicr_l_0(self) -> &'a mut W {
        self.variant(HPSICR_LW::HPSICR_L_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn hpsicr_l_1(self) -> &'a mut W {
        self.variant(HPSICR_LW::HPSICR_L_1)
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
#[doc = "Values that can be written to the field `HAC_L`"]
pub enum HAC_LW {
    #[doc = "Write access is allowed"]
    HAC_L_0,
    #[doc = "Write access is not allowed"]
    HAC_L_1,
}
impl HAC_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HAC_LW::HAC_L_0 => false,
            HAC_LW::HAC_L_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HAC_LW<'a> {
    w: &'a mut W,
}
impl<'a> _HAC_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HAC_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline]
    pub fn hac_l_0(self) -> &'a mut W {
        self.variant(HAC_LW::HAC_L_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline]
    pub fn hac_l_1(self) -> &'a mut W {
        self.variant(HAC_LW::HAC_L_1)
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
    #[doc = "Bit 0 - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline]
    pub fn zmk_wsl(&self) -> ZMK_WSLR {
        ZMK_WSLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline]
    pub fn zmk_rsl(&self) -> ZMK_RSLR {
        ZMK_RSLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline]
    pub fn srtc_sl(&self) -> SRTC_SLR {
        SRTC_SLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline]
    pub fn lpcalb_sl(&self) -> LPCALB_SLR {
        LPCALB_SLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline]
    pub fn mc_sl(&self) -> MC_SLR {
        MC_SLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline]
    pub fn gpr_sl(&self) -> GPR_SLR {
        GPR_SLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline]
    pub fn lpsvcr_sl(&self) -> LPSVCR_SLR {
        LPSVCR_SLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Soft Lock When set, prevents any writes to the LPTDCR"]
    #[inline]
    pub fn lptdcr_sl(&self) -> LPTDCR_SLR {
        LPTDCR_SLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline]
    pub fn mks_sl(&self) -> MKS_SLR {
        MKS_SLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline]
    pub fn hpsvcr_l(&self) -> HPSVCR_LR {
        HPSVCR_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline]
    pub fn hpsicr_l(&self) -> HPSICR_LR {
        HPSICR_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline]
    pub fn hac_l(&self) -> HAC_LR {
        HAC_LR::_from({
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
    #[doc = "Bit 0 - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline]
    pub fn zmk_wsl(&mut self) -> _ZMK_WSLW {
        _ZMK_WSLW { w: self }
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline]
    pub fn zmk_rsl(&mut self) -> _ZMK_RSLW {
        _ZMK_RSLW { w: self }
    }
    #[doc = "Bit 2 - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline]
    pub fn srtc_sl(&mut self) -> _SRTC_SLW {
        _SRTC_SLW { w: self }
    }
    #[doc = "Bit 3 - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline]
    pub fn lpcalb_sl(&mut self) -> _LPCALB_SLW {
        _LPCALB_SLW { w: self }
    }
    #[doc = "Bit 4 - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline]
    pub fn mc_sl(&mut self) -> _MC_SLW {
        _MC_SLW { w: self }
    }
    #[doc = "Bit 5 - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline]
    pub fn gpr_sl(&mut self) -> _GPR_SLW {
        _GPR_SLW { w: self }
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline]
    pub fn lpsvcr_sl(&mut self) -> _LPSVCR_SLW {
        _LPSVCR_SLW { w: self }
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Soft Lock When set, prevents any writes to the LPTDCR"]
    #[inline]
    pub fn lptdcr_sl(&mut self) -> _LPTDCR_SLW {
        _LPTDCR_SLW { w: self }
    }
    #[doc = "Bit 9 - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline]
    pub fn mks_sl(&mut self) -> _MKS_SLW {
        _MKS_SLW { w: self }
    }
    #[doc = "Bit 16 - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline]
    pub fn hpsvcr_l(&mut self) -> _HPSVCR_LW {
        _HPSVCR_LW { w: self }
    }
    #[doc = "Bit 17 - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline]
    pub fn hpsicr_l(&mut self) -> _HPSICR_LW {
        _HPSICR_LW { w: self }
    }
    #[doc = "Bit 18 - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline]
    pub fn hac_l(&mut self) -> _HAC_LW {
        _HAC_LW { w: self }
    }
}
