#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPCOMR {
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
#[doc = "Possible values of the field `SSM_ST_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_ST_DISR {
    #[doc = "Secure to Trusted State transition is enabled"]
    SSM_ST_DIS_0,
    #[doc = "Secure to Trusted State transition is disabled"]
    SSM_ST_DIS_1,
}
impl SSM_ST_DISR {
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
            SSM_ST_DISR::SSM_ST_DIS_0 => false,
            SSM_ST_DISR::SSM_ST_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSM_ST_DISR {
        match value {
            false => SSM_ST_DISR::SSM_ST_DIS_0,
            true => SSM_ST_DISR::SSM_ST_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSM_ST_DIS_0`"]
    #[inline]
    pub fn is_ssm_st_dis_0(&self) -> bool {
        *self == SSM_ST_DISR::SSM_ST_DIS_0
    }
    #[doc = "Checks if the value of the field is `SSM_ST_DIS_1`"]
    #[inline]
    pub fn is_ssm_st_dis_1(&self) -> bool {
        *self == SSM_ST_DISR::SSM_ST_DIS_1
    }
}
#[doc = "Possible values of the field `SSM_SFNS_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_SFNS_DISR {
    #[doc = "Soft Fail to Non-Secure State transition is enabled"]
    SSM_SFNS_DIS_0,
    #[doc = "Soft Fail to Non-Secure State transition is disabled"]
    SSM_SFNS_DIS_1,
}
impl SSM_SFNS_DISR {
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
            SSM_SFNS_DISR::SSM_SFNS_DIS_0 => false,
            SSM_SFNS_DISR::SSM_SFNS_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSM_SFNS_DISR {
        match value {
            false => SSM_SFNS_DISR::SSM_SFNS_DIS_0,
            true => SSM_SFNS_DISR::SSM_SFNS_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSM_SFNS_DIS_0`"]
    #[inline]
    pub fn is_ssm_sfns_dis_0(&self) -> bool {
        *self == SSM_SFNS_DISR::SSM_SFNS_DIS_0
    }
    #[doc = "Checks if the value of the field is `SSM_SFNS_DIS_1`"]
    #[inline]
    pub fn is_ssm_sfns_dis_1(&self) -> bool {
        *self == SSM_SFNS_DISR::SSM_SFNS_DIS_1
    }
}
#[doc = "Possible values of the field `LP_SWR_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_SWR_DISR {
    #[doc = "LP software reset is enabled"]
    LP_SWR_DIS_0,
    #[doc = "LP software reset is disabled"]
    LP_SWR_DIS_1,
}
impl LP_SWR_DISR {
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
            LP_SWR_DISR::LP_SWR_DIS_0 => false,
            LP_SWR_DISR::LP_SWR_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LP_SWR_DISR {
        match value {
            false => LP_SWR_DISR::LP_SWR_DIS_0,
            true => LP_SWR_DISR::LP_SWR_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LP_SWR_DIS_0`"]
    #[inline]
    pub fn is_lp_swr_dis_0(&self) -> bool {
        *self == LP_SWR_DISR::LP_SWR_DIS_0
    }
    #[doc = "Checks if the value of the field is `LP_SWR_DIS_1`"]
    #[inline]
    pub fn is_lp_swr_dis_1(&self) -> bool {
        *self == LP_SWR_DISR::LP_SWR_DIS_1
    }
}
#[doc = r" Value of the field"]
pub struct SW_SVR {
    bits: bool,
}
impl SW_SVR {
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
pub struct SW_FSVR {
    bits: bool,
}
impl SW_FSVR {
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
pub struct SW_LPSVR {
    bits: bool,
}
impl SW_LPSVR {
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
#[doc = "Possible values of the field `MKS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MKS_ENR {
    #[doc = "no description available"]
    MKS_EN_0,
    #[doc = "no description available"]
    MKS_EN_1,
}
impl MKS_ENR {
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
            MKS_ENR::MKS_EN_0 => false,
            MKS_ENR::MKS_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MKS_ENR {
        match value {
            false => MKS_ENR::MKS_EN_0,
            true => MKS_ENR::MKS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MKS_EN_0`"]
    #[inline]
    pub fn is_mks_en_0(&self) -> bool {
        *self == MKS_ENR::MKS_EN_0
    }
    #[doc = "Checks if the value of the field is `MKS_EN_1`"]
    #[inline]
    pub fn is_mks_en_1(&self) -> bool {
        *self == MKS_ENR::MKS_EN_1
    }
}
#[doc = "Possible values of the field `HAC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAC_ENR {
    #[doc = "High Assurance Counter is disabled"]
    HAC_EN_0,
    #[doc = "High Assurance Counter is enabled"]
    HAC_EN_1,
}
impl HAC_ENR {
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
            HAC_ENR::HAC_EN_0 => false,
            HAC_ENR::HAC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HAC_ENR {
        match value {
            false => HAC_ENR::HAC_EN_0,
            true => HAC_ENR::HAC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HAC_EN_0`"]
    #[inline]
    pub fn is_hac_en_0(&self) -> bool {
        *self == HAC_ENR::HAC_EN_0
    }
    #[doc = "Checks if the value of the field is `HAC_EN_1`"]
    #[inline]
    pub fn is_hac_en_1(&self) -> bool {
        *self == HAC_ENR::HAC_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct HAC_STOPR {
    bits: bool,
}
impl HAC_STOPR {
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
pub struct NPSWA_ENR {
    bits: bool,
}
impl NPSWA_ENR {
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
#[doc = r" Proxy"]
pub struct _SSM_STW<'a> {
    w: &'a mut W,
}
impl<'a> _SSM_STW<'a> {
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
#[doc = "Values that can be written to the field `SSM_ST_DIS`"]
pub enum SSM_ST_DISW {
    #[doc = "Secure to Trusted State transition is enabled"]
    SSM_ST_DIS_0,
    #[doc = "Secure to Trusted State transition is disabled"]
    SSM_ST_DIS_1,
}
impl SSM_ST_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSM_ST_DISW::SSM_ST_DIS_0 => false,
            SSM_ST_DISW::SSM_ST_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSM_ST_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSM_ST_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSM_ST_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure to Trusted State transition is enabled"]
    #[inline]
    pub fn ssm_st_dis_0(self) -> &'a mut W {
        self.variant(SSM_ST_DISW::SSM_ST_DIS_0)
    }
    #[doc = "Secure to Trusted State transition is disabled"]
    #[inline]
    pub fn ssm_st_dis_1(self) -> &'a mut W {
        self.variant(SSM_ST_DISW::SSM_ST_DIS_1)
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
#[doc = "Values that can be written to the field `SSM_SFNS_DIS`"]
pub enum SSM_SFNS_DISW {
    #[doc = "Soft Fail to Non-Secure State transition is enabled"]
    SSM_SFNS_DIS_0,
    #[doc = "Soft Fail to Non-Secure State transition is disabled"]
    SSM_SFNS_DIS_1,
}
impl SSM_SFNS_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSM_SFNS_DISW::SSM_SFNS_DIS_0 => false,
            SSM_SFNS_DISW::SSM_SFNS_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSM_SFNS_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSM_SFNS_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSM_SFNS_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Soft Fail to Non-Secure State transition is enabled"]
    #[inline]
    pub fn ssm_sfns_dis_0(self) -> &'a mut W {
        self.variant(SSM_SFNS_DISW::SSM_SFNS_DIS_0)
    }
    #[doc = "Soft Fail to Non-Secure State transition is disabled"]
    #[inline]
    pub fn ssm_sfns_dis_1(self) -> &'a mut W {
        self.variant(SSM_SFNS_DISW::SSM_SFNS_DIS_1)
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
#[doc = "Values that can be written to the field `LP_SWR`"]
pub enum LP_SWRW {
    #[doc = "No Action"]
    LP_SWR_0,
    #[doc = "Reset LP section"]
    LP_SWR_1,
}
impl LP_SWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LP_SWRW::LP_SWR_0 => false,
            LP_SWRW::LP_SWR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LP_SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _LP_SWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LP_SWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn lp_swr_0(self) -> &'a mut W {
        self.variant(LP_SWRW::LP_SWR_0)
    }
    #[doc = "Reset LP section"]
    #[inline]
    pub fn lp_swr_1(self) -> &'a mut W {
        self.variant(LP_SWRW::LP_SWR_1)
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
#[doc = "Values that can be written to the field `LP_SWR_DIS`"]
pub enum LP_SWR_DISW {
    #[doc = "LP software reset is enabled"]
    LP_SWR_DIS_0,
    #[doc = "LP software reset is disabled"]
    LP_SWR_DIS_1,
}
impl LP_SWR_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LP_SWR_DISW::LP_SWR_DIS_0 => false,
            LP_SWR_DISW::LP_SWR_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LP_SWR_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _LP_SWR_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LP_SWR_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LP software reset is enabled"]
    #[inline]
    pub fn lp_swr_dis_0(self) -> &'a mut W {
        self.variant(LP_SWR_DISW::LP_SWR_DIS_0)
    }
    #[doc = "LP software reset is disabled"]
    #[inline]
    pub fn lp_swr_dis_1(self) -> &'a mut W {
        self.variant(LP_SWR_DISW::LP_SWR_DIS_1)
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
pub struct _SW_SVW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_SVW<'a> {
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
pub struct _SW_FSVW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_FSVW<'a> {
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
#[doc = r" Proxy"]
pub struct _SW_LPSVW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_LPSVW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROG_ZMK`"]
pub enum PROG_ZMKW {
    #[doc = "No Action"]
    PROG_ZMK_0,
    #[doc = "Activate hardware key programming mechanism"]
    PROG_ZMK_1,
}
impl PROG_ZMKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROG_ZMKW::PROG_ZMK_0 => false,
            PROG_ZMKW::PROG_ZMK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROG_ZMKW<'a> {
    w: &'a mut W,
}
impl<'a> _PROG_ZMKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROG_ZMKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn prog_zmk_0(self) -> &'a mut W {
        self.variant(PROG_ZMKW::PROG_ZMK_0)
    }
    #[doc = "Activate hardware key programming mechanism"]
    #[inline]
    pub fn prog_zmk_1(self) -> &'a mut W {
        self.variant(PROG_ZMKW::PROG_ZMK_1)
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
#[doc = "Values that can be written to the field `MKS_EN`"]
pub enum MKS_ENW {
    #[doc = "no description available"]
    MKS_EN_0,
    #[doc = "no description available"]
    MKS_EN_1,
}
impl MKS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MKS_ENW::MKS_EN_0 => false,
            MKS_ENW::MKS_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MKS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MKS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MKS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn mks_en_0(self) -> &'a mut W {
        self.variant(MKS_ENW::MKS_EN_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn mks_en_1(self) -> &'a mut W {
        self.variant(MKS_ENW::MKS_EN_1)
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
#[doc = "Values that can be written to the field `HAC_EN`"]
pub enum HAC_ENW {
    #[doc = "High Assurance Counter is disabled"]
    HAC_EN_0,
    #[doc = "High Assurance Counter is enabled"]
    HAC_EN_1,
}
impl HAC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HAC_ENW::HAC_EN_0 => false,
            HAC_ENW::HAC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HAC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HAC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HAC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High Assurance Counter is disabled"]
    #[inline]
    pub fn hac_en_0(self) -> &'a mut W {
        self.variant(HAC_ENW::HAC_EN_0)
    }
    #[doc = "High Assurance Counter is enabled"]
    #[inline]
    pub fn hac_en_1(self) -> &'a mut W {
        self.variant(HAC_ENW::HAC_EN_1)
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
#[doc = "Values that can be written to the field `HAC_LOAD`"]
pub enum HAC_LOADW {
    #[doc = "No Action"]
    HAC_LOAD_0,
    #[doc = "Load the HAC"]
    HAC_LOAD_1,
}
impl HAC_LOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HAC_LOADW::HAC_LOAD_0 => false,
            HAC_LOADW::HAC_LOAD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HAC_LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _HAC_LOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HAC_LOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn hac_load_0(self) -> &'a mut W {
        self.variant(HAC_LOADW::HAC_LOAD_0)
    }
    #[doc = "Load the HAC"]
    #[inline]
    pub fn hac_load_1(self) -> &'a mut W {
        self.variant(HAC_LOADW::HAC_LOAD_1)
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
#[doc = "Values that can be written to the field `HAC_CLEAR`"]
pub enum HAC_CLEARW {
    #[doc = "No Action"]
    HAC_CLEAR_0,
    #[doc = "Clear the HAC"]
    HAC_CLEAR_1,
}
impl HAC_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HAC_CLEARW::HAC_CLEAR_0 => false,
            HAC_CLEARW::HAC_CLEAR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HAC_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _HAC_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HAC_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn hac_clear_0(self) -> &'a mut W {
        self.variant(HAC_CLEARW::HAC_CLEAR_0)
    }
    #[doc = "Clear the HAC"]
    #[inline]
    pub fn hac_clear_1(self) -> &'a mut W {
        self.variant(HAC_CLEARW::HAC_CLEAR_1)
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
#[doc = r" Proxy"]
pub struct _HAC_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _HAC_STOPW<'a> {
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
pub struct _NPSWA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _NPSWA_ENW<'a> {
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
    #[doc = "Bit 1 - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline]
    pub fn ssm_st_dis(&self) -> SSM_ST_DISR {
        SSM_ST_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline]
    pub fn ssm_sfns_dis(&self) -> SSM_SFNS_DISR {
        SSM_SFNS_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - LP Software Reset Disable When set, disables the LP software reset"]
    #[inline]
    pub fn lp_swr_dis(&self) -> LP_SWR_DISR {
        LP_SWR_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline]
    pub fn sw_sv(&self) -> SW_SVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_SVR { bits }
    }
    #[doc = "Bit 9 - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline]
    pub fn sw_fsv(&self) -> SW_FSVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_FSVR { bits }
    }
    #[doc = "Bit 10 - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline]
    pub fn sw_lpsv(&self) -> SW_LPSVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_LPSVR { bits }
    }
    #[doc = "Bit 13 - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline]
    pub fn mks_en(&self) -> MKS_ENR {
        MKS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline]
    pub fn hac_en(&self) -> HAC_ENR {
        HAC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline]
    pub fn hac_stop(&self) -> HAC_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HAC_STOPR { bits }
    }
    #[doc = "Bit 31 - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline]
    pub fn npswa_en(&self) -> NPSWA_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NPSWA_ENR { bits }
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
    #[doc = "Bit 0 - SSM State Transition Transition state of the system security monitor"]
    #[inline]
    pub fn ssm_st(&mut self) -> _SSM_STW {
        _SSM_STW { w: self }
    }
    #[doc = "Bit 1 - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline]
    pub fn ssm_st_dis(&mut self) -> _SSM_ST_DISW {
        _SSM_ST_DISW { w: self }
    }
    #[doc = "Bit 2 - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline]
    pub fn ssm_sfns_dis(&mut self) -> _SSM_SFNS_DISW {
        _SSM_SFNS_DISW { w: self }
    }
    #[doc = "Bit 4 - LP Software Reset When set to 1, the registers in the SNVS_LP section are reset"]
    #[inline]
    pub fn lp_swr(&mut self) -> _LP_SWRW {
        _LP_SWRW { w: self }
    }
    #[doc = "Bit 5 - LP Software Reset Disable When set, disables the LP software reset"]
    #[inline]
    pub fn lp_swr_dis(&mut self) -> _LP_SWR_DISW {
        _LP_SWR_DISW { w: self }
    }
    #[doc = "Bit 8 - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline]
    pub fn sw_sv(&mut self) -> _SW_SVW {
        _SW_SVW { w: self }
    }
    #[doc = "Bit 9 - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline]
    pub fn sw_fsv(&mut self) -> _SW_FSVW {
        _SW_FSVW { w: self }
    }
    #[doc = "Bit 10 - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline]
    pub fn sw_lpsv(&mut self) -> _SW_LPSVW {
        _SW_LPSVW { w: self }
    }
    #[doc = "Bit 12 - Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    #[inline]
    pub fn prog_zmk(&mut self) -> _PROG_ZMKW {
        _PROG_ZMKW { w: self }
    }
    #[doc = "Bit 13 - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline]
    pub fn mks_en(&mut self) -> _MKS_ENW {
        _MKS_ENW { w: self }
    }
    #[doc = "Bit 16 - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline]
    pub fn hac_en(&mut self) -> _HAC_ENW {
        _HAC_ENW { w: self }
    }
    #[doc = "Bit 17 - High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    #[inline]
    pub fn hac_load(&mut self) -> _HAC_LOADW {
        _HAC_LOADW { w: self }
    }
    #[doc = "Bit 18 - High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    #[inline]
    pub fn hac_clear(&mut self) -> _HAC_CLEARW {
        _HAC_CLEARW { w: self }
    }
    #[doc = "Bit 19 - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline]
    pub fn hac_stop(&mut self) -> _HAC_STOPW {
        _HAC_STOPW { w: self }
    }
    #[doc = "Bit 31 - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline]
    pub fn npswa_en(&mut self) -> _NPSWA_ENW {
        _NPSWA_ENW { w: self }
    }
}
