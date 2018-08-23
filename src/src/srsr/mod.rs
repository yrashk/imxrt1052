#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRSR {
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
#[doc = "Possible values of the field `ipp_reset_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPP_RESET_BR {
    #[doc = "Reset is not a result of ipp_reset_b pin."]
    IPP_RESET_B_0,
    #[doc = "Reset is a result of ipp_reset_b pin."]
    IPP_RESET_B_1,
}
impl IPP_RESET_BR {
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
            IPP_RESET_BR::IPP_RESET_B_0 => false,
            IPP_RESET_BR::IPP_RESET_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPP_RESET_BR {
        match value {
            false => IPP_RESET_BR::IPP_RESET_B_0,
            true => IPP_RESET_BR::IPP_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPP_RESET_B_0`"]
    #[inline]
    pub fn is_ipp_reset_b_0(&self) -> bool {
        *self == IPP_RESET_BR::IPP_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `IPP_RESET_B_1`"]
    #[inline]
    pub fn is_ipp_reset_b_1(&self) -> bool {
        *self == IPP_RESET_BR::IPP_RESET_B_1
    }
}
#[doc = "Possible values of the field `lockup_sysresetreq`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_SYSRESETREQR {
    #[doc = "Reset is not a result of the mentioned case."]
    LOCKUP_SYSRESETREQ_0,
    #[doc = "Reset is a result of the mentioned case."]
    LOCKUP_SYSRESETREQ_1,
}
impl LOCKUP_SYSRESETREQR {
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
            LOCKUP_SYSRESETREQR::LOCKUP_SYSRESETREQ_0 => false,
            LOCKUP_SYSRESETREQR::LOCKUP_SYSRESETREQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKUP_SYSRESETREQR {
        match value {
            false => LOCKUP_SYSRESETREQR::LOCKUP_SYSRESETREQ_0,
            true => LOCKUP_SYSRESETREQR::LOCKUP_SYSRESETREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_SYSRESETREQ_0`"]
    #[inline]
    pub fn is_lockup_sysresetreq_0(&self) -> bool {
        *self == LOCKUP_SYSRESETREQR::LOCKUP_SYSRESETREQ_0
    }
    #[doc = "Checks if the value of the field is `LOCKUP_SYSRESETREQ_1`"]
    #[inline]
    pub fn is_lockup_sysresetreq_1(&self) -> bool {
        *self == LOCKUP_SYSRESETREQR::LOCKUP_SYSRESETREQ_1
    }
}
#[doc = "Possible values of the field `csu_reset_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSU_RESET_BR {
    #[doc = "Reset is not a result of the csu_reset_b event."]
    CSU_RESET_B_0,
    #[doc = "Reset is a result of the csu_reset_b event."]
    CSU_RESET_B_1,
}
impl CSU_RESET_BR {
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
            CSU_RESET_BR::CSU_RESET_B_0 => false,
            CSU_RESET_BR::CSU_RESET_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSU_RESET_BR {
        match value {
            false => CSU_RESET_BR::CSU_RESET_B_0,
            true => CSU_RESET_BR::CSU_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSU_RESET_B_0`"]
    #[inline]
    pub fn is_csu_reset_b_0(&self) -> bool {
        *self == CSU_RESET_BR::CSU_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `CSU_RESET_B_1`"]
    #[inline]
    pub fn is_csu_reset_b_1(&self) -> bool {
        *self == CSU_RESET_BR::CSU_RESET_B_1
    }
}
#[doc = "Possible values of the field `ipp_user_reset_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPP_USER_RESET_BR {
    #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_0,
    #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_1,
}
impl IPP_USER_RESET_BR {
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
            IPP_USER_RESET_BR::IPP_USER_RESET_B_0 => false,
            IPP_USER_RESET_BR::IPP_USER_RESET_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPP_USER_RESET_BR {
        match value {
            false => IPP_USER_RESET_BR::IPP_USER_RESET_B_0,
            true => IPP_USER_RESET_BR::IPP_USER_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPP_USER_RESET_B_0`"]
    #[inline]
    pub fn is_ipp_user_reset_b_0(&self) -> bool {
        *self == IPP_USER_RESET_BR::IPP_USER_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `IPP_USER_RESET_B_1`"]
    #[inline]
    pub fn is_ipp_user_reset_b_1(&self) -> bool {
        *self == IPP_USER_RESET_BR::IPP_USER_RESET_B_1
    }
}
#[doc = "Possible values of the field `wdog_rst_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_RST_BR {
    #[doc = "Reset is not a result of the watchdog time-out event."]
    WDOG_RST_B_0,
    #[doc = "Reset is a result of the watchdog time-out event."]
    WDOG_RST_B_1,
}
impl WDOG_RST_BR {
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
            WDOG_RST_BR::WDOG_RST_B_0 => false,
            WDOG_RST_BR::WDOG_RST_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDOG_RST_BR {
        match value {
            false => WDOG_RST_BR::WDOG_RST_B_0,
            true => WDOG_RST_BR::WDOG_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG_RST_B_0`"]
    #[inline]
    pub fn is_wdog_rst_b_0(&self) -> bool {
        *self == WDOG_RST_BR::WDOG_RST_B_0
    }
    #[doc = "Checks if the value of the field is `WDOG_RST_B_1`"]
    #[inline]
    pub fn is_wdog_rst_b_1(&self) -> bool {
        *self == WDOG_RST_BR::WDOG_RST_B_1
    }
}
#[doc = "Possible values of the field `jtag_rst_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_RST_BR {
    #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_0,
    #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_1,
}
impl JTAG_RST_BR {
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
            JTAG_RST_BR::JTAG_RST_B_0 => false,
            JTAG_RST_BR::JTAG_RST_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JTAG_RST_BR {
        match value {
            false => JTAG_RST_BR::JTAG_RST_B_0,
            true => JTAG_RST_BR::JTAG_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_RST_B_0`"]
    #[inline]
    pub fn is_jtag_rst_b_0(&self) -> bool {
        *self == JTAG_RST_BR::JTAG_RST_B_0
    }
    #[doc = "Checks if the value of the field is `JTAG_RST_B_1`"]
    #[inline]
    pub fn is_jtag_rst_b_1(&self) -> bool {
        *self == JTAG_RST_BR::JTAG_RST_B_1
    }
}
#[doc = "Possible values of the field `jtag_sw_rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_SW_RSTR {
    #[doc = "Reset is not a result of software reset from JTAG."]
    JTAG_SW_RST_0,
    #[doc = "Reset is a result of software reset from JTAG."]
    JTAG_SW_RST_1,
}
impl JTAG_SW_RSTR {
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
            JTAG_SW_RSTR::JTAG_SW_RST_0 => false,
            JTAG_SW_RSTR::JTAG_SW_RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JTAG_SW_RSTR {
        match value {
            false => JTAG_SW_RSTR::JTAG_SW_RST_0,
            true => JTAG_SW_RSTR::JTAG_SW_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_SW_RST_0`"]
    #[inline]
    pub fn is_jtag_sw_rst_0(&self) -> bool {
        *self == JTAG_SW_RSTR::JTAG_SW_RST_0
    }
    #[doc = "Checks if the value of the field is `JTAG_SW_RST_1`"]
    #[inline]
    pub fn is_jtag_sw_rst_1(&self) -> bool {
        *self == JTAG_SW_RSTR::JTAG_SW_RST_1
    }
}
#[doc = "Possible values of the field `wdog3_rst_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG3_RST_BR {
    #[doc = "Reset is not a result of the watchdog3 time-out event."]
    WDOG3_RST_B_0,
    #[doc = "Reset is a result of the watchdog3 time-out event."]
    WDOG3_RST_B_1,
}
impl WDOG3_RST_BR {
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
            WDOG3_RST_BR::WDOG3_RST_B_0 => false,
            WDOG3_RST_BR::WDOG3_RST_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDOG3_RST_BR {
        match value {
            false => WDOG3_RST_BR::WDOG3_RST_B_0,
            true => WDOG3_RST_BR::WDOG3_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG3_RST_B_0`"]
    #[inline]
    pub fn is_wdog3_rst_b_0(&self) -> bool {
        *self == WDOG3_RST_BR::WDOG3_RST_B_0
    }
    #[doc = "Checks if the value of the field is `WDOG3_RST_B_1`"]
    #[inline]
    pub fn is_wdog3_rst_b_1(&self) -> bool {
        *self == WDOG3_RST_BR::WDOG3_RST_B_1
    }
}
#[doc = "Possible values of the field `tempsense_rst_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPSENSE_RST_BR {
    #[doc = "Reset is not a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_0,
    #[doc = "Reset is a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_1,
}
impl TEMPSENSE_RST_BR {
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
            TEMPSENSE_RST_BR::TEMPSENSE_RST_B_0 => false,
            TEMPSENSE_RST_BR::TEMPSENSE_RST_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEMPSENSE_RST_BR {
        match value {
            false => TEMPSENSE_RST_BR::TEMPSENSE_RST_B_0,
            true => TEMPSENSE_RST_BR::TEMPSENSE_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEMPSENSE_RST_B_0`"]
    #[inline]
    pub fn is_tempsense_rst_b_0(&self) -> bool {
        *self == TEMPSENSE_RST_BR::TEMPSENSE_RST_B_0
    }
    #[doc = "Checks if the value of the field is `TEMPSENSE_RST_B_1`"]
    #[inline]
    pub fn is_tempsense_rst_b_1(&self) -> bool {
        *self == TEMPSENSE_RST_BR::TEMPSENSE_RST_B_1
    }
}
#[doc = "Values that can be written to the field `ipp_reset_b`"]
pub enum IPP_RESET_BW {
    #[doc = "Reset is not a result of ipp_reset_b pin."]
    IPP_RESET_B_0,
    #[doc = "Reset is a result of ipp_reset_b pin."]
    IPP_RESET_B_1,
}
impl IPP_RESET_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPP_RESET_BW::IPP_RESET_B_0 => false,
            IPP_RESET_BW::IPP_RESET_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPP_RESET_BW<'a> {
    w: &'a mut W,
}
impl<'a> _IPP_RESET_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPP_RESET_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of ipp_reset_b pin."]
    #[inline]
    pub fn ipp_reset_b_0(self) -> &'a mut W {
        self.variant(IPP_RESET_BW::IPP_RESET_B_0)
    }
    #[doc = "Reset is a result of ipp_reset_b pin."]
    #[inline]
    pub fn ipp_reset_b_1(self) -> &'a mut W {
        self.variant(IPP_RESET_BW::IPP_RESET_B_1)
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
#[doc = "Values that can be written to the field `lockup_sysresetreq`"]
pub enum LOCKUP_SYSRESETREQW {
    #[doc = "Reset is not a result of the mentioned case."]
    LOCKUP_SYSRESETREQ_0,
    #[doc = "Reset is a result of the mentioned case."]
    LOCKUP_SYSRESETREQ_1,
}
impl LOCKUP_SYSRESETREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKUP_SYSRESETREQW::LOCKUP_SYSRESETREQ_0 => false,
            LOCKUP_SYSRESETREQW::LOCKUP_SYSRESETREQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKUP_SYSRESETREQW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKUP_SYSRESETREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKUP_SYSRESETREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of the mentioned case."]
    #[inline]
    pub fn lockup_sysresetreq_0(self) -> &'a mut W {
        self.variant(LOCKUP_SYSRESETREQW::LOCKUP_SYSRESETREQ_0)
    }
    #[doc = "Reset is a result of the mentioned case."]
    #[inline]
    pub fn lockup_sysresetreq_1(self) -> &'a mut W {
        self.variant(LOCKUP_SYSRESETREQW::LOCKUP_SYSRESETREQ_1)
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
#[doc = "Values that can be written to the field `csu_reset_b`"]
pub enum CSU_RESET_BW {
    #[doc = "Reset is not a result of the csu_reset_b event."]
    CSU_RESET_B_0,
    #[doc = "Reset is a result of the csu_reset_b event."]
    CSU_RESET_B_1,
}
impl CSU_RESET_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSU_RESET_BW::CSU_RESET_B_0 => false,
            CSU_RESET_BW::CSU_RESET_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSU_RESET_BW<'a> {
    w: &'a mut W,
}
impl<'a> _CSU_RESET_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSU_RESET_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of the csu_reset_b event."]
    #[inline]
    pub fn csu_reset_b_0(self) -> &'a mut W {
        self.variant(CSU_RESET_BW::CSU_RESET_B_0)
    }
    #[doc = "Reset is a result of the csu_reset_b event."]
    #[inline]
    pub fn csu_reset_b_1(self) -> &'a mut W {
        self.variant(CSU_RESET_BW::CSU_RESET_B_1)
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
#[doc = "Values that can be written to the field `ipp_user_reset_b`"]
pub enum IPP_USER_RESET_BW {
    #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_0,
    #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_1,
}
impl IPP_USER_RESET_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPP_USER_RESET_BW::IPP_USER_RESET_B_0 => false,
            IPP_USER_RESET_BW::IPP_USER_RESET_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPP_USER_RESET_BW<'a> {
    w: &'a mut W,
}
impl<'a> _IPP_USER_RESET_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPP_USER_RESET_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    #[inline]
    pub fn ipp_user_reset_b_0(self) -> &'a mut W {
        self.variant(IPP_USER_RESET_BW::IPP_USER_RESET_B_0)
    }
    #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    #[inline]
    pub fn ipp_user_reset_b_1(self) -> &'a mut W {
        self.variant(IPP_USER_RESET_BW::IPP_USER_RESET_B_1)
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
#[doc = "Values that can be written to the field `wdog_rst_b`"]
pub enum WDOG_RST_BW {
    #[doc = "Reset is not a result of the watchdog time-out event."]
    WDOG_RST_B_0,
    #[doc = "Reset is a result of the watchdog time-out event."]
    WDOG_RST_B_1,
}
impl WDOG_RST_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDOG_RST_BW::WDOG_RST_B_0 => false,
            WDOG_RST_BW::WDOG_RST_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOG_RST_BW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG_RST_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOG_RST_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of the watchdog time-out event."]
    #[inline]
    pub fn wdog_rst_b_0(self) -> &'a mut W {
        self.variant(WDOG_RST_BW::WDOG_RST_B_0)
    }
    #[doc = "Reset is a result of the watchdog time-out event."]
    #[inline]
    pub fn wdog_rst_b_1(self) -> &'a mut W {
        self.variant(WDOG_RST_BW::WDOG_RST_B_1)
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
#[doc = "Values that can be written to the field `jtag_rst_b`"]
pub enum JTAG_RST_BW {
    #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_0,
    #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_1,
}
impl JTAG_RST_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JTAG_RST_BW::JTAG_RST_B_0 => false,
            JTAG_RST_BW::JTAG_RST_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JTAG_RST_BW<'a> {
    w: &'a mut W,
}
impl<'a> _JTAG_RST_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JTAG_RST_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
    #[inline]
    pub fn jtag_rst_b_0(self) -> &'a mut W {
        self.variant(JTAG_RST_BW::JTAG_RST_B_0)
    }
    #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
    #[inline]
    pub fn jtag_rst_b_1(self) -> &'a mut W {
        self.variant(JTAG_RST_BW::JTAG_RST_B_1)
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
#[doc = "Values that can be written to the field `jtag_sw_rst`"]
pub enum JTAG_SW_RSTW {
    #[doc = "Reset is not a result of software reset from JTAG."]
    JTAG_SW_RST_0,
    #[doc = "Reset is a result of software reset from JTAG."]
    JTAG_SW_RST_1,
}
impl JTAG_SW_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JTAG_SW_RSTW::JTAG_SW_RST_0 => false,
            JTAG_SW_RSTW::JTAG_SW_RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JTAG_SW_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _JTAG_SW_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JTAG_SW_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of software reset from JTAG."]
    #[inline]
    pub fn jtag_sw_rst_0(self) -> &'a mut W {
        self.variant(JTAG_SW_RSTW::JTAG_SW_RST_0)
    }
    #[doc = "Reset is a result of software reset from JTAG."]
    #[inline]
    pub fn jtag_sw_rst_1(self) -> &'a mut W {
        self.variant(JTAG_SW_RSTW::JTAG_SW_RST_1)
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
#[doc = "Values that can be written to the field `wdog3_rst_b`"]
pub enum WDOG3_RST_BW {
    #[doc = "Reset is not a result of the watchdog3 time-out event."]
    WDOG3_RST_B_0,
    #[doc = "Reset is a result of the watchdog3 time-out event."]
    WDOG3_RST_B_1,
}
impl WDOG3_RST_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDOG3_RST_BW::WDOG3_RST_B_0 => false,
            WDOG3_RST_BW::WDOG3_RST_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOG3_RST_BW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG3_RST_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOG3_RST_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of the watchdog3 time-out event."]
    #[inline]
    pub fn wdog3_rst_b_0(self) -> &'a mut W {
        self.variant(WDOG3_RST_BW::WDOG3_RST_B_0)
    }
    #[doc = "Reset is a result of the watchdog3 time-out event."]
    #[inline]
    pub fn wdog3_rst_b_1(self) -> &'a mut W {
        self.variant(WDOG3_RST_BW::WDOG3_RST_B_1)
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
#[doc = "Values that can be written to the field `tempsense_rst_b`"]
pub enum TEMPSENSE_RST_BW {
    #[doc = "Reset is not a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_0,
    #[doc = "Reset is a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_1,
}
impl TEMPSENSE_RST_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEMPSENSE_RST_BW::TEMPSENSE_RST_B_0 => false,
            TEMPSENSE_RST_BW::TEMPSENSE_RST_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEMPSENSE_RST_BW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPSENSE_RST_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEMPSENSE_RST_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset is not a result of software reset from Temperature Sensor."]
    #[inline]
    pub fn tempsense_rst_b_0(self) -> &'a mut W {
        self.variant(TEMPSENSE_RST_BW::TEMPSENSE_RST_B_0)
    }
    #[doc = "Reset is a result of software reset from Temperature Sensor."]
    #[inline]
    pub fn tempsense_rst_b_1(self) -> &'a mut W {
        self.variant(TEMPSENSE_RST_BW::TEMPSENSE_RST_B_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[inline]
    pub fn ipp_reset_b(&self) -> IPP_RESET_BR {
        IPP_RESET_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates a reset has been caused by CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    #[inline]
    pub fn lockup_sysresetreq(&self) -> LOCKUP_SYSRESETREQR {
        LOCKUP_SYSRESETREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Indicates whether the reset was the result of the csu_reset_b input."]
    #[inline]
    pub fn csu_reset_b(&self) -> CSU_RESET_BR {
        CSU_RESET_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[inline]
    pub fn ipp_user_reset_b(&self) -> IPP_USER_RESET_BR {
        IPP_USER_RESET_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IC Watchdog Time-out reset"]
    #[inline]
    pub fn wdog_rst_b(&self) -> WDOG_RST_BR {
        WDOG_RST_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[inline]
    pub fn jtag_rst_b(&self) -> JTAG_RST_BR {
        JTAG_RST_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[inline]
    pub fn jtag_sw_rst(&self) -> JTAG_SW_RSTR {
        JTAG_SW_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - IC Watchdog3 Time-out reset"]
    #[inline]
    pub fn wdog3_rst_b(&self) -> WDOG3_RST_BR {
        WDOG3_RST_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Temper Sensor software reset"]
    #[inline]
    pub fn tempsense_rst_b(&self) -> TEMPSENSE_RST_BR {
        TEMPSENSE_RST_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[inline]
    pub fn ipp_reset_b(&mut self) -> _IPP_RESET_BW {
        _IPP_RESET_BW { w: self }
    }
    #[doc = "Bit 1 - Indicates a reset has been caused by CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    #[inline]
    pub fn lockup_sysresetreq(&mut self) -> _LOCKUP_SYSRESETREQW {
        _LOCKUP_SYSRESETREQW { w: self }
    }
    #[doc = "Bit 2 - Indicates whether the reset was the result of the csu_reset_b input."]
    #[inline]
    pub fn csu_reset_b(&mut self) -> _CSU_RESET_BW {
        _CSU_RESET_BW { w: self }
    }
    #[doc = "Bit 3 - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[inline]
    pub fn ipp_user_reset_b(&mut self) -> _IPP_USER_RESET_BW {
        _IPP_USER_RESET_BW { w: self }
    }
    #[doc = "Bit 4 - IC Watchdog Time-out reset"]
    #[inline]
    pub fn wdog_rst_b(&mut self) -> _WDOG_RST_BW {
        _WDOG_RST_BW { w: self }
    }
    #[doc = "Bit 5 - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[inline]
    pub fn jtag_rst_b(&mut self) -> _JTAG_RST_BW {
        _JTAG_RST_BW { w: self }
    }
    #[doc = "Bit 6 - JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[inline]
    pub fn jtag_sw_rst(&mut self) -> _JTAG_SW_RSTW {
        _JTAG_SW_RSTW { w: self }
    }
    #[doc = "Bit 7 - IC Watchdog3 Time-out reset"]
    #[inline]
    pub fn wdog3_rst_b(&mut self) -> _WDOG3_RST_BW {
        _WDOG3_RST_BW { w: self }
    }
    #[doc = "Bit 8 - Temper Sensor software reset"]
    #[inline]
    pub fn tempsense_rst_b(&mut self) -> _TEMPSENSE_RST_BW {
        _TEMPSENSE_RST_BW { w: self }
    }
}
