#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR10 {
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
#[doc = "Possible values of the field `NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDENR {
    #[doc = "Debug turned off."]
    NIDEN_0,
    #[doc = "Debug enabled (default)."]
    NIDEN_1,
}
impl NIDENR {
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
            NIDENR::NIDEN_0 => false,
            NIDENR::NIDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NIDENR {
        match value {
            false => NIDENR::NIDEN_0,
            true => NIDENR::NIDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NIDEN_0`"]
    #[inline]
    pub fn is_niden_0(&self) -> bool {
        *self == NIDENR::NIDEN_0
    }
    #[doc = "Checks if the value of the field is `NIDEN_1`"]
    #[inline]
    pub fn is_niden_1(&self) -> bool {
        *self == NIDENR::NIDEN_1
    }
}
#[doc = "Possible values of the field `DBG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_ENR {
    #[doc = "Debug turned off."]
    DBG_EN_0,
    #[doc = "Debug enabled (default)."]
    DBG_EN_1,
}
impl DBG_ENR {
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
            DBG_ENR::DBG_EN_0 => false,
            DBG_ENR::DBG_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBG_ENR {
        match value {
            false => DBG_ENR::DBG_EN_0,
            true => DBG_ENR::DBG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_EN_0`"]
    #[inline]
    pub fn is_dbg_en_0(&self) -> bool {
        *self == DBG_ENR::DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `DBG_EN_1`"]
    #[inline]
    pub fn is_dbg_en_1(&self) -> bool {
        *self == DBG_ENR::DBG_EN_1
    }
}
#[doc = "Possible values of the field `SEC_ERR_RESP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_ERR_RESPR {
    #[doc = "OKEY response"]
    SEC_ERR_RESP_0,
    #[doc = "SLVError (default)"]
    SEC_ERR_RESP_1,
}
impl SEC_ERR_RESPR {
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
            SEC_ERR_RESPR::SEC_ERR_RESP_0 => false,
            SEC_ERR_RESPR::SEC_ERR_RESP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_ERR_RESPR {
        match value {
            false => SEC_ERR_RESPR::SEC_ERR_RESP_0,
            true => SEC_ERR_RESPR::SEC_ERR_RESP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEC_ERR_RESP_0`"]
    #[inline]
    pub fn is_sec_err_resp_0(&self) -> bool {
        *self == SEC_ERR_RESPR::SEC_ERR_RESP_0
    }
    #[doc = "Checks if the value of the field is `SEC_ERR_RESP_1`"]
    #[inline]
    pub fn is_sec_err_resp_1(&self) -> bool {
        *self == SEC_ERR_RESPR::SEC_ERR_RESP_1
    }
}
#[doc = "Possible values of the field `DCPKEY_OCOTP_OR_KEYMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCPKEY_OCOTP_OR_KEYMUXR {
    #[doc = "Select key from Key MUX (SNVS/OTPMK)."]
    DCPKEY_OCOTP_OR_KEYMUX_0,
    #[doc = "Select key from OCOTP (SW_GP2)."]
    DCPKEY_OCOTP_OR_KEYMUX_1,
}
impl DCPKEY_OCOTP_OR_KEYMUXR {
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
            DCPKEY_OCOTP_OR_KEYMUXR::DCPKEY_OCOTP_OR_KEYMUX_0 => false,
            DCPKEY_OCOTP_OR_KEYMUXR::DCPKEY_OCOTP_OR_KEYMUX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCPKEY_OCOTP_OR_KEYMUXR {
        match value {
            false => DCPKEY_OCOTP_OR_KEYMUXR::DCPKEY_OCOTP_OR_KEYMUX_0,
            true => DCPKEY_OCOTP_OR_KEYMUXR::DCPKEY_OCOTP_OR_KEYMUX_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCPKEY_OCOTP_OR_KEYMUX_0`"]
    #[inline]
    pub fn is_dcpkey_ocotp_or_keymux_0(&self) -> bool {
        *self == DCPKEY_OCOTP_OR_KEYMUXR::DCPKEY_OCOTP_OR_KEYMUX_0
    }
    #[doc = "Checks if the value of the field is `DCPKEY_OCOTP_OR_KEYMUX_1`"]
    #[inline]
    pub fn is_dcpkey_ocotp_or_keymux_1(&self) -> bool {
        *self == DCPKEY_OCOTP_OR_KEYMUXR::DCPKEY_OCOTP_OR_KEYMUX_1
    }
}
#[doc = "Possible values of the field `OCRAM_TZ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_TZ_ENR {
    #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
    OCRAM_TZ_EN_0,
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by [ENDADDR:STARTADDR] follows the execution mode access policy described in CSU chapter."]
    OCRAM_TZ_EN_1,
}
impl OCRAM_TZ_ENR {
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
            OCRAM_TZ_ENR::OCRAM_TZ_EN_0 => false,
            OCRAM_TZ_ENR::OCRAM_TZ_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCRAM_TZ_ENR {
        match value {
            false => OCRAM_TZ_ENR::OCRAM_TZ_EN_0,
            true => OCRAM_TZ_ENR::OCRAM_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_TZ_EN_0`"]
    #[inline]
    pub fn is_ocram_tz_en_0(&self) -> bool {
        *self == OCRAM_TZ_ENR::OCRAM_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_TZ_EN_1`"]
    #[inline]
    pub fn is_ocram_tz_en_1(&self) -> bool {
        *self == OCRAM_TZ_ENR::OCRAM_TZ_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct OCRAM_TZ_ADDRR {
    bits: u8,
}
impl OCRAM_TZ_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LOCK_NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_NIDENR {
    #[doc = "Field is not locked"]
    LOCK_NIDEN_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_NIDEN_1,
}
impl LOCK_NIDENR {
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
            LOCK_NIDENR::LOCK_NIDEN_0 => false,
            LOCK_NIDENR::LOCK_NIDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_NIDENR {
        match value {
            false => LOCK_NIDENR::LOCK_NIDEN_0,
            true => LOCK_NIDENR::LOCK_NIDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_NIDEN_0`"]
    #[inline]
    pub fn is_lock_niden_0(&self) -> bool {
        *self == LOCK_NIDENR::LOCK_NIDEN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_NIDEN_1`"]
    #[inline]
    pub fn is_lock_niden_1(&self) -> bool {
        *self == LOCK_NIDENR::LOCK_NIDEN_1
    }
}
#[doc = "Possible values of the field `LOCK_DBG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_DBG_ENR {
    #[doc = "Field is not locked"]
    LOCK_DBG_EN_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_DBG_EN_1,
}
impl LOCK_DBG_ENR {
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
            LOCK_DBG_ENR::LOCK_DBG_EN_0 => false,
            LOCK_DBG_ENR::LOCK_DBG_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_DBG_ENR {
        match value {
            false => LOCK_DBG_ENR::LOCK_DBG_EN_0,
            true => LOCK_DBG_ENR::LOCK_DBG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_DBG_EN_0`"]
    #[inline]
    pub fn is_lock_dbg_en_0(&self) -> bool {
        *self == LOCK_DBG_ENR::LOCK_DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_DBG_EN_1`"]
    #[inline]
    pub fn is_lock_dbg_en_1(&self) -> bool {
        *self == LOCK_DBG_ENR::LOCK_DBG_EN_1
    }
}
#[doc = "Possible values of the field `LOCK_SEC_ERR_RESP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_SEC_ERR_RESPR {
    #[doc = "Field is not locked"]
    LOCK_SEC_ERR_RESP_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_SEC_ERR_RESP_1,
}
impl LOCK_SEC_ERR_RESPR {
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
            LOCK_SEC_ERR_RESPR::LOCK_SEC_ERR_RESP_0 => false,
            LOCK_SEC_ERR_RESPR::LOCK_SEC_ERR_RESP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_SEC_ERR_RESPR {
        match value {
            false => LOCK_SEC_ERR_RESPR::LOCK_SEC_ERR_RESP_0,
            true => LOCK_SEC_ERR_RESPR::LOCK_SEC_ERR_RESP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_SEC_ERR_RESP_0`"]
    #[inline]
    pub fn is_lock_sec_err_resp_0(&self) -> bool {
        *self == LOCK_SEC_ERR_RESPR::LOCK_SEC_ERR_RESP_0
    }
    #[doc = "Checks if the value of the field is `LOCK_SEC_ERR_RESP_1`"]
    #[inline]
    pub fn is_lock_sec_err_resp_1(&self) -> bool {
        *self == LOCK_SEC_ERR_RESPR::LOCK_SEC_ERR_RESP_1
    }
}
#[doc = "Possible values of the field `LOCK_DCPKEY_OCOTP_OR_KEYMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_DCPKEY_OCOTP_OR_KEYMUXR {
    #[doc = "Field is not locked"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_1,
}
impl LOCK_DCPKEY_OCOTP_OR_KEYMUXR {
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
            LOCK_DCPKEY_OCOTP_OR_KEYMUXR::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0 => false,
            LOCK_DCPKEY_OCOTP_OR_KEYMUXR::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_DCPKEY_OCOTP_OR_KEYMUXR {
        match value {
            false => LOCK_DCPKEY_OCOTP_OR_KEYMUXR::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0,
            true => LOCK_DCPKEY_OCOTP_OR_KEYMUXR::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_DCPKEY_OCOTP_OR_KEYMUX_0`"]
    #[inline]
    pub fn is_lock_dcpkey_ocotp_or_keymux_0(&self) -> bool {
        *self == LOCK_DCPKEY_OCOTP_OR_KEYMUXR::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0
    }
    #[doc = "Checks if the value of the field is `LOCK_DCPKEY_OCOTP_OR_KEYMUX_1`"]
    #[inline]
    pub fn is_lock_dcpkey_ocotp_or_keymux_1(&self) -> bool {
        *self == LOCK_DCPKEY_OCOTP_OR_KEYMUXR::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1
    }
}
#[doc = "Possible values of the field `LOCK_OCRAM_TZ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_OCRAM_TZ_ENR {
    #[doc = "Field is not locked"]
    LOCK_OCRAM_TZ_EN_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_OCRAM_TZ_EN_1,
}
impl LOCK_OCRAM_TZ_ENR {
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
            LOCK_OCRAM_TZ_ENR::LOCK_OCRAM_TZ_EN_0 => false,
            LOCK_OCRAM_TZ_ENR::LOCK_OCRAM_TZ_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_OCRAM_TZ_ENR {
        match value {
            false => LOCK_OCRAM_TZ_ENR::LOCK_OCRAM_TZ_EN_0,
            true => LOCK_OCRAM_TZ_ENR::LOCK_OCRAM_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_EN_0`"]
    #[inline]
    pub fn is_lock_ocram_tz_en_0(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ENR::LOCK_OCRAM_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_EN_1`"]
    #[inline]
    pub fn is_lock_ocram_tz_en_1(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ENR::LOCK_OCRAM_TZ_EN_1
    }
}
#[doc = "Possible values of the field `LOCK_OCRAM_TZ_ADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_OCRAM_TZ_ADDRR {
    #[doc = "Field is not locked"]
    LOCK_OCRAM_TZ_ADDR_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_OCRAM_TZ_ADDR_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_OCRAM_TZ_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_OCRAM_TZ_ADDRR::LOCK_OCRAM_TZ_ADDR_0 => 0,
            LOCK_OCRAM_TZ_ADDRR::LOCK_OCRAM_TZ_ADDR_1 => 1,
            LOCK_OCRAM_TZ_ADDRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_OCRAM_TZ_ADDRR {
        match value {
            0 => LOCK_OCRAM_TZ_ADDRR::LOCK_OCRAM_TZ_ADDR_0,
            1 => LOCK_OCRAM_TZ_ADDRR::LOCK_OCRAM_TZ_ADDR_1,
            i => LOCK_OCRAM_TZ_ADDRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_ADDR_0`"]
    #[inline]
    pub fn is_lock_ocram_tz_addr_0(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ADDRR::LOCK_OCRAM_TZ_ADDR_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_ADDR_1`"]
    #[inline]
    pub fn is_lock_ocram_tz_addr_1(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ADDRR::LOCK_OCRAM_TZ_ADDR_1
    }
}
#[doc = "Values that can be written to the field `NIDEN`"]
pub enum NIDENW {
    #[doc = "Debug turned off."]
    NIDEN_0,
    #[doc = "Debug enabled (default)."]
    NIDEN_1,
}
impl NIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NIDENW::NIDEN_0 => false,
            NIDENW::NIDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _NIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NIDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Debug turned off."]
    #[inline]
    pub fn niden_0(self) -> &'a mut W {
        self.variant(NIDENW::NIDEN_0)
    }
    #[doc = "Debug enabled (default)."]
    #[inline]
    pub fn niden_1(self) -> &'a mut W {
        self.variant(NIDENW::NIDEN_1)
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
#[doc = "Values that can be written to the field `DBG_EN`"]
pub enum DBG_ENW {
    #[doc = "Debug turned off."]
    DBG_EN_0,
    #[doc = "Debug enabled (default)."]
    DBG_EN_1,
}
impl DBG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBG_ENW::DBG_EN_0 => false,
            DBG_ENW::DBG_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Debug turned off."]
    #[inline]
    pub fn dbg_en_0(self) -> &'a mut W {
        self.variant(DBG_ENW::DBG_EN_0)
    }
    #[doc = "Debug enabled (default)."]
    #[inline]
    pub fn dbg_en_1(self) -> &'a mut W {
        self.variant(DBG_ENW::DBG_EN_1)
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
#[doc = "Values that can be written to the field `SEC_ERR_RESP`"]
pub enum SEC_ERR_RESPW {
    #[doc = "OKEY response"]
    SEC_ERR_RESP_0,
    #[doc = "SLVError (default)"]
    SEC_ERR_RESP_1,
}
impl SEC_ERR_RESPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_ERR_RESPW::SEC_ERR_RESP_0 => false,
            SEC_ERR_RESPW::SEC_ERR_RESP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_ERR_RESPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_ERR_RESPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_ERR_RESPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OKEY response"]
    #[inline]
    pub fn sec_err_resp_0(self) -> &'a mut W {
        self.variant(SEC_ERR_RESPW::SEC_ERR_RESP_0)
    }
    #[doc = "SLVError (default)"]
    #[inline]
    pub fn sec_err_resp_1(self) -> &'a mut W {
        self.variant(SEC_ERR_RESPW::SEC_ERR_RESP_1)
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
#[doc = "Values that can be written to the field `DCPKEY_OCOTP_OR_KEYMUX`"]
pub enum DCPKEY_OCOTP_OR_KEYMUXW {
    #[doc = "Select key from Key MUX (SNVS/OTPMK)."]
    DCPKEY_OCOTP_OR_KEYMUX_0,
    #[doc = "Select key from OCOTP (SW_GP2)."]
    DCPKEY_OCOTP_OR_KEYMUX_1,
}
impl DCPKEY_OCOTP_OR_KEYMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCPKEY_OCOTP_OR_KEYMUXW::DCPKEY_OCOTP_OR_KEYMUX_0 => false,
            DCPKEY_OCOTP_OR_KEYMUXW::DCPKEY_OCOTP_OR_KEYMUX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCPKEY_OCOTP_OR_KEYMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCPKEY_OCOTP_OR_KEYMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCPKEY_OCOTP_OR_KEYMUXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select key from Key MUX (SNVS/OTPMK)."]
    #[inline]
    pub fn dcpkey_ocotp_or_keymux_0(self) -> &'a mut W {
        self.variant(DCPKEY_OCOTP_OR_KEYMUXW::DCPKEY_OCOTP_OR_KEYMUX_0)
    }
    #[doc = "Select key from OCOTP (SW_GP2)."]
    #[inline]
    pub fn dcpkey_ocotp_or_keymux_1(self) -> &'a mut W {
        self.variant(DCPKEY_OCOTP_OR_KEYMUXW::DCPKEY_OCOTP_OR_KEYMUX_1)
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
#[doc = "Values that can be written to the field `OCRAM_TZ_EN`"]
pub enum OCRAM_TZ_ENW {
    #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
    OCRAM_TZ_EN_0,
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by [ENDADDR:STARTADDR] follows the execution mode access policy described in CSU chapter."]
    OCRAM_TZ_EN_1,
}
impl OCRAM_TZ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCRAM_TZ_ENW::OCRAM_TZ_EN_0 => false,
            OCRAM_TZ_ENW::OCRAM_TZ_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCRAM_TZ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_TZ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCRAM_TZ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
    #[inline]
    pub fn ocram_tz_en_0(self) -> &'a mut W {
        self.variant(OCRAM_TZ_ENW::OCRAM_TZ_EN_0)
    }
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by [ENDADDR:STARTADDR] follows the execution mode access policy described in CSU chapter."]
    #[inline]
    pub fn ocram_tz_en_1(self) -> &'a mut W {
        self.variant(OCRAM_TZ_ENW::OCRAM_TZ_EN_1)
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
#[doc = r" Proxy"]
pub struct _OCRAM_TZ_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_TZ_ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK_NIDEN`"]
pub enum LOCK_NIDENW {
    #[doc = "Field is not locked"]
    LOCK_NIDEN_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_NIDEN_1,
}
impl LOCK_NIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_NIDENW::LOCK_NIDEN_0 => false,
            LOCK_NIDENW::LOCK_NIDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_NIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_NIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_NIDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Field is not locked"]
    #[inline]
    pub fn lock_niden_0(self) -> &'a mut W {
        self.variant(LOCK_NIDENW::LOCK_NIDEN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline]
    pub fn lock_niden_1(self) -> &'a mut W {
        self.variant(LOCK_NIDENW::LOCK_NIDEN_1)
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
#[doc = "Values that can be written to the field `LOCK_DBG_EN`"]
pub enum LOCK_DBG_ENW {
    #[doc = "Field is not locked"]
    LOCK_DBG_EN_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_DBG_EN_1,
}
impl LOCK_DBG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_DBG_ENW::LOCK_DBG_EN_0 => false,
            LOCK_DBG_ENW::LOCK_DBG_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_DBG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_DBG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_DBG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Field is not locked"]
    #[inline]
    pub fn lock_dbg_en_0(self) -> &'a mut W {
        self.variant(LOCK_DBG_ENW::LOCK_DBG_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline]
    pub fn lock_dbg_en_1(self) -> &'a mut W {
        self.variant(LOCK_DBG_ENW::LOCK_DBG_EN_1)
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
#[doc = "Values that can be written to the field `LOCK_SEC_ERR_RESP`"]
pub enum LOCK_SEC_ERR_RESPW {
    #[doc = "Field is not locked"]
    LOCK_SEC_ERR_RESP_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_SEC_ERR_RESP_1,
}
impl LOCK_SEC_ERR_RESPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_SEC_ERR_RESPW::LOCK_SEC_ERR_RESP_0 => false,
            LOCK_SEC_ERR_RESPW::LOCK_SEC_ERR_RESP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_SEC_ERR_RESPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_SEC_ERR_RESPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_SEC_ERR_RESPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Field is not locked"]
    #[inline]
    pub fn lock_sec_err_resp_0(self) -> &'a mut W {
        self.variant(LOCK_SEC_ERR_RESPW::LOCK_SEC_ERR_RESP_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline]
    pub fn lock_sec_err_resp_1(self) -> &'a mut W {
        self.variant(LOCK_SEC_ERR_RESPW::LOCK_SEC_ERR_RESP_1)
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
#[doc = "Values that can be written to the field `LOCK_DCPKEY_OCOTP_OR_KEYMUX`"]
pub enum LOCK_DCPKEY_OCOTP_OR_KEYMUXW {
    #[doc = "Field is not locked"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_1,
}
impl LOCK_DCPKEY_OCOTP_OR_KEYMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_DCPKEY_OCOTP_OR_KEYMUXW::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0 => false,
            LOCK_DCPKEY_OCOTP_OR_KEYMUXW::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_DCPKEY_OCOTP_OR_KEYMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_DCPKEY_OCOTP_OR_KEYMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_DCPKEY_OCOTP_OR_KEYMUXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Field is not locked"]
    #[inline]
    pub fn lock_dcpkey_ocotp_or_keymux_0(self) -> &'a mut W {
        self.variant(LOCK_DCPKEY_OCOTP_OR_KEYMUXW::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline]
    pub fn lock_dcpkey_ocotp_or_keymux_1(self) -> &'a mut W {
        self.variant(LOCK_DCPKEY_OCOTP_OR_KEYMUXW::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1)
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
#[doc = "Values that can be written to the field `LOCK_OCRAM_TZ_EN`"]
pub enum LOCK_OCRAM_TZ_ENW {
    #[doc = "Field is not locked"]
    LOCK_OCRAM_TZ_EN_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_OCRAM_TZ_EN_1,
}
impl LOCK_OCRAM_TZ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_OCRAM_TZ_ENW::LOCK_OCRAM_TZ_EN_0 => false,
            LOCK_OCRAM_TZ_ENW::LOCK_OCRAM_TZ_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_OCRAM_TZ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_OCRAM_TZ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_OCRAM_TZ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Field is not locked"]
    #[inline]
    pub fn lock_ocram_tz_en_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ENW::LOCK_OCRAM_TZ_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline]
    pub fn lock_ocram_tz_en_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ENW::LOCK_OCRAM_TZ_EN_1)
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
#[doc = "Values that can be written to the field `LOCK_OCRAM_TZ_ADDR`"]
pub enum LOCK_OCRAM_TZ_ADDRW {
    #[doc = "Field is not locked"]
    LOCK_OCRAM_TZ_ADDR_0,
    #[doc = "Field is locked (read access only)"]
    LOCK_OCRAM_TZ_ADDR_1,
}
impl LOCK_OCRAM_TZ_ADDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_OCRAM_TZ_ADDRW::LOCK_OCRAM_TZ_ADDR_0 => 0,
            LOCK_OCRAM_TZ_ADDRW::LOCK_OCRAM_TZ_ADDR_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_OCRAM_TZ_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_OCRAM_TZ_ADDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_OCRAM_TZ_ADDRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Field is not locked"]
    #[inline]
    pub fn lock_ocram_tz_addr_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ADDRW::LOCK_OCRAM_TZ_ADDR_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline]
    pub fn lock_ocram_tz_addr_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ADDRW::LOCK_OCRAM_TZ_ADDR_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - ARM non-secure (non-invasive) debug enable"]
    #[inline]
    pub fn niden(&self) -> NIDENR {
        NIDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ARM invasive debug enable"]
    #[inline]
    pub fn dbg_en(&self) -> DBG_ENR {
        DBG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline]
    pub fn sec_err_resp(&self) -> SEC_ERR_RESPR {
        SEC_ERR_RESPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DCP Key selection bit."]
    #[inline]
    pub fn dcpkey_ocotp_or_keymux(&self) -> DCPKEY_OCOTP_OR_KEYMUXR {
        DCPKEY_OCOTP_OR_KEYMUXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - OCRAM TrustZone (TZ) enable."]
    #[inline]
    pub fn ocram_tz_en(&self) -> OCRAM_TZ_ENR {
        OCRAM_TZ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:15 - OCRAM TrustZone (TZ) start address"]
    #[inline]
    pub fn ocram_tz_addr(&self) -> OCRAM_TZ_ADDRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OCRAM_TZ_ADDRR { bits }
    }
    #[doc = "Bit 16 - Lock NIDEN field for changes"]
    #[inline]
    pub fn lock_niden(&self) -> LOCK_NIDENR {
        LOCK_NIDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Lock DBG_EN field for changes"]
    #[inline]
    pub fn lock_dbg_en(&self) -> LOCK_DBG_ENR {
        LOCK_DBG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Lock SEC_ERR_RESP field for changes"]
    #[inline]
    pub fn lock_sec_err_resp(&self) -> LOCK_SEC_ERR_RESPR {
        LOCK_SEC_ERR_RESPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline]
    pub fn lock_dcpkey_ocotp_or_keymux(&self) -> LOCK_DCPKEY_OCOTP_OR_KEYMUXR {
        LOCK_DCPKEY_OCOTP_OR_KEYMUXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Lock OCRAM_TZ_EN field for changes"]
    #[inline]
    pub fn lock_ocram_tz_en(&self) -> LOCK_OCRAM_TZ_ENR {
        LOCK_OCRAM_TZ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:31 - Lock OCRAM_TZ_ADDR field for changes"]
    #[inline]
    pub fn lock_ocram_tz_addr(&self) -> LOCK_OCRAM_TZ_ADDRR {
        LOCK_OCRAM_TZ_ADDRR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ARM non-secure (non-invasive) debug enable"]
    #[inline]
    pub fn niden(&mut self) -> _NIDENW {
        _NIDENW { w: self }
    }
    #[doc = "Bit 1 - ARM invasive debug enable"]
    #[inline]
    pub fn dbg_en(&mut self) -> _DBG_ENW {
        _DBG_ENW { w: self }
    }
    #[doc = "Bit 2 - Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline]
    pub fn sec_err_resp(&mut self) -> _SEC_ERR_RESPW {
        _SEC_ERR_RESPW { w: self }
    }
    #[doc = "Bit 4 - DCP Key selection bit."]
    #[inline]
    pub fn dcpkey_ocotp_or_keymux(&mut self) -> _DCPKEY_OCOTP_OR_KEYMUXW {
        _DCPKEY_OCOTP_OR_KEYMUXW { w: self }
    }
    #[doc = "Bit 8 - OCRAM TrustZone (TZ) enable."]
    #[inline]
    pub fn ocram_tz_en(&mut self) -> _OCRAM_TZ_ENW {
        _OCRAM_TZ_ENW { w: self }
    }
    #[doc = "Bits 9:15 - OCRAM TrustZone (TZ) start address"]
    #[inline]
    pub fn ocram_tz_addr(&mut self) -> _OCRAM_TZ_ADDRW {
        _OCRAM_TZ_ADDRW { w: self }
    }
    #[doc = "Bit 16 - Lock NIDEN field for changes"]
    #[inline]
    pub fn lock_niden(&mut self) -> _LOCK_NIDENW {
        _LOCK_NIDENW { w: self }
    }
    #[doc = "Bit 17 - Lock DBG_EN field for changes"]
    #[inline]
    pub fn lock_dbg_en(&mut self) -> _LOCK_DBG_ENW {
        _LOCK_DBG_ENW { w: self }
    }
    #[doc = "Bit 18 - Lock SEC_ERR_RESP field for changes"]
    #[inline]
    pub fn lock_sec_err_resp(&mut self) -> _LOCK_SEC_ERR_RESPW {
        _LOCK_SEC_ERR_RESPW { w: self }
    }
    #[doc = "Bit 20 - Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline]
    pub fn lock_dcpkey_ocotp_or_keymux(&mut self) -> _LOCK_DCPKEY_OCOTP_OR_KEYMUXW {
        _LOCK_DCPKEY_OCOTP_OR_KEYMUXW { w: self }
    }
    #[doc = "Bit 24 - Lock OCRAM_TZ_EN field for changes"]
    #[inline]
    pub fn lock_ocram_tz_en(&mut self) -> _LOCK_OCRAM_TZ_ENW {
        _LOCK_OCRAM_TZ_ENW { w: self }
    }
    #[doc = "Bits 25:31 - Lock OCRAM_TZ_ADDR field for changes"]
    #[inline]
    pub fn lock_ocram_tz_addr(&mut self) -> _LOCK_OCRAM_TZ_ADDRW {
        _LOCK_OCRAM_TZ_ADDRW { w: self }
    }
}
