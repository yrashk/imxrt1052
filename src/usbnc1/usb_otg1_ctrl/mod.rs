#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB_OTG1_CTRL {
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
#[doc = "Possible values of the field `OVER_CUR_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVER_CUR_DISR {
    #[doc = "Enables overcurrent detection"]
    OVER_CUR_DIS_0,
    #[doc = "Disables overcurrent detection"]
    OVER_CUR_DIS_1,
}
impl OVER_CUR_DISR {
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
            OVER_CUR_DISR::OVER_CUR_DIS_0 => false,
            OVER_CUR_DISR::OVER_CUR_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVER_CUR_DISR {
        match value {
            false => OVER_CUR_DISR::OVER_CUR_DIS_0,
            true => OVER_CUR_DISR::OVER_CUR_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_DIS_0`"]
    #[inline]
    pub fn is_over_cur_dis_0(&self) -> bool {
        *self == OVER_CUR_DISR::OVER_CUR_DIS_0
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_DIS_1`"]
    #[inline]
    pub fn is_over_cur_dis_1(&self) -> bool {
        *self == OVER_CUR_DISR::OVER_CUR_DIS_1
    }
}
#[doc = "Possible values of the field `OVER_CUR_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVER_CUR_POLR {
    #[doc = "High active (high on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_0,
    #[doc = "Low active (low on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_1,
}
impl OVER_CUR_POLR {
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
            OVER_CUR_POLR::OVER_CUR_POL_0 => false,
            OVER_CUR_POLR::OVER_CUR_POL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVER_CUR_POLR {
        match value {
            false => OVER_CUR_POLR::OVER_CUR_POL_0,
            true => OVER_CUR_POLR::OVER_CUR_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_POL_0`"]
    #[inline]
    pub fn is_over_cur_pol_0(&self) -> bool {
        *self == OVER_CUR_POLR::OVER_CUR_POL_0
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_POL_1`"]
    #[inline]
    pub fn is_over_cur_pol_1(&self) -> bool {
        *self == OVER_CUR_POLR::OVER_CUR_POL_1
    }
}
#[doc = "Possible values of the field `PWR_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_POLR {
    #[doc = "PMIC Power Pin is Low active."]
    PWR_POL_0,
    #[doc = "PMIC Power Pin is High active."]
    PWR_POL_1,
}
impl PWR_POLR {
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
            PWR_POLR::PWR_POL_0 => false,
            PWR_POLR::PWR_POL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_POLR {
        match value {
            false => PWR_POLR::PWR_POL_0,
            true => PWR_POLR::PWR_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWR_POL_0`"]
    #[inline]
    pub fn is_pwr_pol_0(&self) -> bool {
        *self == PWR_POLR::PWR_POL_0
    }
    #[doc = "Checks if the value of the field is `PWR_POL_1`"]
    #[inline]
    pub fn is_pwr_pol_1(&self) -> bool {
        *self == PWR_POLR::PWR_POL_1
    }
}
#[doc = "Possible values of the field `WIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIER {
    #[doc = "Interrupt Disabled"]
    WIE_0,
    #[doc = "Interrupt Enabled"]
    WIE_1,
}
impl WIER {
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
            WIER::WIE_0 => false,
            WIER::WIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIER {
        match value {
            false => WIER::WIE_0,
            true => WIER::WIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIE_0`"]
    #[inline]
    pub fn is_wie_0(&self) -> bool {
        *self == WIER::WIE_0
    }
    #[doc = "Checks if the value of the field is `WIE_1`"]
    #[inline]
    pub fn is_wie_1(&self) -> bool {
        *self == WIER::WIE_1
    }
}
#[doc = "Possible values of the field `WKUP_SW_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_SW_ENR {
    #[doc = "Disable"]
    WKUP_SW_EN_0,
    #[doc = "Enable"]
    WKUP_SW_EN_1,
}
impl WKUP_SW_ENR {
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
            WKUP_SW_ENR::WKUP_SW_EN_0 => false,
            WKUP_SW_ENR::WKUP_SW_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUP_SW_ENR {
        match value {
            false => WKUP_SW_ENR::WKUP_SW_EN_0,
            true => WKUP_SW_ENR::WKUP_SW_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_EN_0`"]
    #[inline]
    pub fn is_wkup_sw_en_0(&self) -> bool {
        *self == WKUP_SW_ENR::WKUP_SW_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_EN_1`"]
    #[inline]
    pub fn is_wkup_sw_en_1(&self) -> bool {
        *self == WKUP_SW_ENR::WKUP_SW_EN_1
    }
}
#[doc = "Possible values of the field `WKUP_SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_SWR {
    #[doc = "Inactive"]
    WKUP_SW_0,
    #[doc = "Force wake-up"]
    WKUP_SW_1,
}
impl WKUP_SWR {
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
            WKUP_SWR::WKUP_SW_0 => false,
            WKUP_SWR::WKUP_SW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUP_SWR {
        match value {
            false => WKUP_SWR::WKUP_SW_0,
            true => WKUP_SWR::WKUP_SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_0`"]
    #[inline]
    pub fn is_wkup_sw_0(&self) -> bool {
        *self == WKUP_SWR::WKUP_SW_0
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_1`"]
    #[inline]
    pub fn is_wkup_sw_1(&self) -> bool {
        *self == WKUP_SWR::WKUP_SW_1
    }
}
#[doc = "Possible values of the field `WKUP_ID_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_ID_ENR {
    #[doc = "Disable"]
    WKUP_ID_EN_0,
    #[doc = "Enable"]
    WKUP_ID_EN_1,
}
impl WKUP_ID_ENR {
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
            WKUP_ID_ENR::WKUP_ID_EN_0 => false,
            WKUP_ID_ENR::WKUP_ID_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUP_ID_ENR {
        match value {
            false => WKUP_ID_ENR::WKUP_ID_EN_0,
            true => WKUP_ID_ENR::WKUP_ID_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_ID_EN_0`"]
    #[inline]
    pub fn is_wkup_id_en_0(&self) -> bool {
        *self == WKUP_ID_ENR::WKUP_ID_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_ID_EN_1`"]
    #[inline]
    pub fn is_wkup_id_en_1(&self) -> bool {
        *self == WKUP_ID_ENR::WKUP_ID_EN_1
    }
}
#[doc = "Possible values of the field `WKUP_VBUS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_VBUS_ENR {
    #[doc = "Disable"]
    WKUP_VBUS_EN_0,
    #[doc = "Enable"]
    WKUP_VBUS_EN_1,
}
impl WKUP_VBUS_ENR {
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
            WKUP_VBUS_ENR::WKUP_VBUS_EN_0 => false,
            WKUP_VBUS_ENR::WKUP_VBUS_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUP_VBUS_ENR {
        match value {
            false => WKUP_VBUS_ENR::WKUP_VBUS_EN_0,
            true => WKUP_VBUS_ENR::WKUP_VBUS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_VBUS_EN_0`"]
    #[inline]
    pub fn is_wkup_vbus_en_0(&self) -> bool {
        *self == WKUP_VBUS_ENR::WKUP_VBUS_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_VBUS_EN_1`"]
    #[inline]
    pub fn is_wkup_vbus_en_1(&self) -> bool {
        *self == WKUP_VBUS_ENR::WKUP_VBUS_EN_1
    }
}
#[doc = "Possible values of the field `WKUP_DPDM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_DPDM_ENR {
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
    WKUP_DPDM_EN_0,
    #[doc = "(Default) DPDM changes wake-up to be enabled, it is for device only."]
    WKUP_DPDM_EN_1,
}
impl WKUP_DPDM_ENR {
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
            WKUP_DPDM_ENR::WKUP_DPDM_EN_0 => false,
            WKUP_DPDM_ENR::WKUP_DPDM_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUP_DPDM_ENR {
        match value {
            false => WKUP_DPDM_ENR::WKUP_DPDM_EN_0,
            true => WKUP_DPDM_ENR::WKUP_DPDM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_DPDM_EN_0`"]
    #[inline]
    pub fn is_wkup_dpdm_en_0(&self) -> bool {
        *self == WKUP_DPDM_ENR::WKUP_DPDM_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_DPDM_EN_1`"]
    #[inline]
    pub fn is_wkup_dpdm_en_1(&self) -> bool {
        *self == WKUP_DPDM_ENR::WKUP_DPDM_EN_1
    }
}
#[doc = "Possible values of the field `WIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIRR {
    #[doc = "No wake-up interrupt request received"]
    WIR_0,
    #[doc = "Wake-up Interrupt Request received"]
    WIR_1,
}
impl WIRR {
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
            WIRR::WIR_0 => false,
            WIRR::WIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIRR {
        match value {
            false => WIRR::WIR_0,
            true => WIRR::WIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIR_0`"]
    #[inline]
    pub fn is_wir_0(&self) -> bool {
        *self == WIRR::WIR_0
    }
    #[doc = "Checks if the value of the field is `WIR_1`"]
    #[inline]
    pub fn is_wir_1(&self) -> bool {
        *self == WIRR::WIR_1
    }
}
#[doc = "Values that can be written to the field `OVER_CUR_DIS`"]
pub enum OVER_CUR_DISW {
    #[doc = "Enables overcurrent detection"]
    OVER_CUR_DIS_0,
    #[doc = "Disables overcurrent detection"]
    OVER_CUR_DIS_1,
}
impl OVER_CUR_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVER_CUR_DISW::OVER_CUR_DIS_0 => false,
            OVER_CUR_DISW::OVER_CUR_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVER_CUR_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _OVER_CUR_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVER_CUR_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables overcurrent detection"]
    #[inline]
    pub fn over_cur_dis_0(self) -> &'a mut W {
        self.variant(OVER_CUR_DISW::OVER_CUR_DIS_0)
    }
    #[doc = "Disables overcurrent detection"]
    #[inline]
    pub fn over_cur_dis_1(self) -> &'a mut W {
        self.variant(OVER_CUR_DISW::OVER_CUR_DIS_1)
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
#[doc = "Values that can be written to the field `OVER_CUR_POL`"]
pub enum OVER_CUR_POLW {
    #[doc = "High active (high on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_0,
    #[doc = "Low active (low on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_1,
}
impl OVER_CUR_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVER_CUR_POLW::OVER_CUR_POL_0 => false,
            OVER_CUR_POLW::OVER_CUR_POL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVER_CUR_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _OVER_CUR_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVER_CUR_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High active (high on this signal represents an overcurrent condition)"]
    #[inline]
    pub fn over_cur_pol_0(self) -> &'a mut W {
        self.variant(OVER_CUR_POLW::OVER_CUR_POL_0)
    }
    #[doc = "Low active (low on this signal represents an overcurrent condition)"]
    #[inline]
    pub fn over_cur_pol_1(self) -> &'a mut W {
        self.variant(OVER_CUR_POLW::OVER_CUR_POL_1)
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
#[doc = "Values that can be written to the field `PWR_POL`"]
pub enum PWR_POLW {
    #[doc = "PMIC Power Pin is Low active."]
    PWR_POL_0,
    #[doc = "PMIC Power Pin is High active."]
    PWR_POL_1,
}
impl PWR_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWR_POLW::PWR_POL_0 => false,
            PWR_POLW::PWR_POL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWR_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PMIC Power Pin is Low active."]
    #[inline]
    pub fn pwr_pol_0(self) -> &'a mut W {
        self.variant(PWR_POLW::PWR_POL_0)
    }
    #[doc = "PMIC Power Pin is High active."]
    #[inline]
    pub fn pwr_pol_1(self) -> &'a mut W {
        self.variant(PWR_POLW::PWR_POL_1)
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
#[doc = "Values that can be written to the field `WIE`"]
pub enum WIEW {
    #[doc = "Interrupt Disabled"]
    WIE_0,
    #[doc = "Interrupt Enabled"]
    WIE_1,
}
impl WIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WIEW::WIE_0 => false,
            WIEW::WIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt Disabled"]
    #[inline]
    pub fn wie_0(self) -> &'a mut W {
        self.variant(WIEW::WIE_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline]
    pub fn wie_1(self) -> &'a mut W {
        self.variant(WIEW::WIE_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUP_SW_EN`"]
pub enum WKUP_SW_ENW {
    #[doc = "Disable"]
    WKUP_SW_EN_0,
    #[doc = "Enable"]
    WKUP_SW_EN_1,
}
impl WKUP_SW_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUP_SW_ENW::WKUP_SW_EN_0 => false,
            WKUP_SW_ENW::WKUP_SW_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUP_SW_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUP_SW_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUP_SW_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn wkup_sw_en_0(self) -> &'a mut W {
        self.variant(WKUP_SW_ENW::WKUP_SW_EN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn wkup_sw_en_1(self) -> &'a mut W {
        self.variant(WKUP_SW_ENW::WKUP_SW_EN_1)
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
#[doc = "Values that can be written to the field `WKUP_SW`"]
pub enum WKUP_SWW {
    #[doc = "Inactive"]
    WKUP_SW_0,
    #[doc = "Force wake-up"]
    WKUP_SW_1,
}
impl WKUP_SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUP_SWW::WKUP_SW_0 => false,
            WKUP_SWW::WKUP_SW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUP_SWW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUP_SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUP_SWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive"]
    #[inline]
    pub fn wkup_sw_0(self) -> &'a mut W {
        self.variant(WKUP_SWW::WKUP_SW_0)
    }
    #[doc = "Force wake-up"]
    #[inline]
    pub fn wkup_sw_1(self) -> &'a mut W {
        self.variant(WKUP_SWW::WKUP_SW_1)
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
#[doc = "Values that can be written to the field `WKUP_ID_EN`"]
pub enum WKUP_ID_ENW {
    #[doc = "Disable"]
    WKUP_ID_EN_0,
    #[doc = "Enable"]
    WKUP_ID_EN_1,
}
impl WKUP_ID_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUP_ID_ENW::WKUP_ID_EN_0 => false,
            WKUP_ID_ENW::WKUP_ID_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUP_ID_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUP_ID_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUP_ID_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn wkup_id_en_0(self) -> &'a mut W {
        self.variant(WKUP_ID_ENW::WKUP_ID_EN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn wkup_id_en_1(self) -> &'a mut W {
        self.variant(WKUP_ID_ENW::WKUP_ID_EN_1)
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
#[doc = "Values that can be written to the field `WKUP_VBUS_EN`"]
pub enum WKUP_VBUS_ENW {
    #[doc = "Disable"]
    WKUP_VBUS_EN_0,
    #[doc = "Enable"]
    WKUP_VBUS_EN_1,
}
impl WKUP_VBUS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUP_VBUS_ENW::WKUP_VBUS_EN_0 => false,
            WKUP_VBUS_ENW::WKUP_VBUS_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUP_VBUS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUP_VBUS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUP_VBUS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn wkup_vbus_en_0(self) -> &'a mut W {
        self.variant(WKUP_VBUS_ENW::WKUP_VBUS_EN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn wkup_vbus_en_1(self) -> &'a mut W {
        self.variant(WKUP_VBUS_ENW::WKUP_VBUS_EN_1)
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
#[doc = "Values that can be written to the field `WKUP_DPDM_EN`"]
pub enum WKUP_DPDM_ENW {
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
    WKUP_DPDM_EN_0,
    #[doc = "(Default) DPDM changes wake-up to be enabled, it is for device only."]
    WKUP_DPDM_EN_1,
}
impl WKUP_DPDM_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUP_DPDM_ENW::WKUP_DPDM_EN_0 => false,
            WKUP_DPDM_ENW::WKUP_DPDM_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUP_DPDM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUP_DPDM_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUP_DPDM_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
    #[inline]
    pub fn wkup_dpdm_en_0(self) -> &'a mut W {
        self.variant(WKUP_DPDM_ENW::WKUP_DPDM_EN_0)
    }
    #[doc = "(Default) DPDM changes wake-up to be enabled, it is for device only."]
    #[inline]
    pub fn wkup_dpdm_en_1(self) -> &'a mut W {
        self.variant(WKUP_DPDM_ENW::WKUP_DPDM_EN_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - Disable OTG1 Overcurrent Detection"]
    #[inline]
    pub fn over_cur_dis(&self) -> OVER_CUR_DISR {
        OVER_CUR_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[inline]
    pub fn over_cur_pol(&self) -> OVER_CUR_POLR {
        OVER_CUR_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline]
    pub fn pwr_pol(&self) -> PWR_POLR {
        PWR_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[inline]
    pub fn wie(&self) -> WIER {
        WIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - OTG1 Software Wake-up Enable"]
    #[inline]
    pub fn wkup_sw_en(&self) -> WKUP_SW_ENR {
        WKUP_SW_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - OTG1 Software Wake-up"]
    #[inline]
    pub fn wkup_sw(&self) -> WKUP_SWR {
        WKUP_SWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - OTG1 Wake-up on ID change enable"]
    #[inline]
    pub fn wkup_id_en(&self) -> WKUP_ID_ENR {
        WKUP_ID_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - OTG1 wake-up on VBUS change enable"]
    #[inline]
    pub fn wkup_vbus_en(&self) -> WKUP_VBUS_ENR {
        WKUP_VBUS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Wake-up on DPDM change enable"]
    #[inline]
    pub fn wkup_dpdm_en(&self) -> WKUP_DPDM_ENR {
        WKUP_DPDM_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port"]
    #[inline]
    pub fn wir(&self) -> WIRR {
        WIRR::_from({
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
        W { bits: 805310464 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Disable OTG1 Overcurrent Detection"]
    #[inline]
    pub fn over_cur_dis(&mut self) -> _OVER_CUR_DISW {
        _OVER_CUR_DISW { w: self }
    }
    #[doc = "Bit 8 - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[inline]
    pub fn over_cur_pol(&mut self) -> _OVER_CUR_POLW {
        _OVER_CUR_POLW { w: self }
    }
    #[doc = "Bit 9 - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline]
    pub fn pwr_pol(&mut self) -> _PWR_POLW {
        _PWR_POLW { w: self }
    }
    #[doc = "Bit 10 - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[inline]
    pub fn wie(&mut self) -> _WIEW {
        _WIEW { w: self }
    }
    #[doc = "Bit 14 - OTG1 Software Wake-up Enable"]
    #[inline]
    pub fn wkup_sw_en(&mut self) -> _WKUP_SW_ENW {
        _WKUP_SW_ENW { w: self }
    }
    #[doc = "Bit 15 - OTG1 Software Wake-up"]
    #[inline]
    pub fn wkup_sw(&mut self) -> _WKUP_SWW {
        _WKUP_SWW { w: self }
    }
    #[doc = "Bit 16 - OTG1 Wake-up on ID change enable"]
    #[inline]
    pub fn wkup_id_en(&mut self) -> _WKUP_ID_ENW {
        _WKUP_ID_ENW { w: self }
    }
    #[doc = "Bit 17 - OTG1 wake-up on VBUS change enable"]
    #[inline]
    pub fn wkup_vbus_en(&mut self) -> _WKUP_VBUS_ENW {
        _WKUP_VBUS_ENW { w: self }
    }
    #[doc = "Bit 29 - Wake-up on DPDM change enable"]
    #[inline]
    pub fn wkup_dpdm_en(&mut self) -> _WKUP_DPDM_ENW {
        _WKUP_DPDM_ENW { w: self }
    }
}
