#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_STATUS {
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
#[doc = "Possible values of the field `ITCM_MAM_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITCM_MAM_STATUSR {
    #[doc = "ITCM did not access magic address."]
    ITCM_MAM_STATUS_0,
    #[doc = "ITCM accessed magic address."]
    ITCM_MAM_STATUS_1,
}
impl ITCM_MAM_STATUSR {
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
            ITCM_MAM_STATUSR::ITCM_MAM_STATUS_0 => false,
            ITCM_MAM_STATUSR::ITCM_MAM_STATUS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITCM_MAM_STATUSR {
        match value {
            false => ITCM_MAM_STATUSR::ITCM_MAM_STATUS_0,
            true => ITCM_MAM_STATUSR::ITCM_MAM_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITCM_MAM_STATUS_0`"]
    #[inline]
    pub fn is_itcm_mam_status_0(&self) -> bool {
        *self == ITCM_MAM_STATUSR::ITCM_MAM_STATUS_0
    }
    #[doc = "Checks if the value of the field is `ITCM_MAM_STATUS_1`"]
    #[inline]
    pub fn is_itcm_mam_status_1(&self) -> bool {
        *self == ITCM_MAM_STATUSR::ITCM_MAM_STATUS_1
    }
}
#[doc = "Possible values of the field `DTCM_MAM_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCM_MAM_STATUSR {
    #[doc = "DTCM did not access magic address."]
    DTCM_MAM_STATUS_0,
    #[doc = "DTCM accessed magic address."]
    DTCM_MAM_STATUS_1,
}
impl DTCM_MAM_STATUSR {
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
            DTCM_MAM_STATUSR::DTCM_MAM_STATUS_0 => false,
            DTCM_MAM_STATUSR::DTCM_MAM_STATUS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTCM_MAM_STATUSR {
        match value {
            false => DTCM_MAM_STATUSR::DTCM_MAM_STATUS_0,
            true => DTCM_MAM_STATUSR::DTCM_MAM_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTCM_MAM_STATUS_0`"]
    #[inline]
    pub fn is_dtcm_mam_status_0(&self) -> bool {
        *self == DTCM_MAM_STATUSR::DTCM_MAM_STATUS_0
    }
    #[doc = "Checks if the value of the field is `DTCM_MAM_STATUS_1`"]
    #[inline]
    pub fn is_dtcm_mam_status_1(&self) -> bool {
        *self == DTCM_MAM_STATUSR::DTCM_MAM_STATUS_1
    }
}
#[doc = "Possible values of the field `OCRAM_MAM_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_MAM_STATUSR {
    #[doc = "OCRAM did not access magic address."]
    OCRAM_MAM_STATUS_0,
    #[doc = "OCRAM accessed magic address."]
    OCRAM_MAM_STATUS_1,
}
impl OCRAM_MAM_STATUSR {
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
            OCRAM_MAM_STATUSR::OCRAM_MAM_STATUS_0 => false,
            OCRAM_MAM_STATUSR::OCRAM_MAM_STATUS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCRAM_MAM_STATUSR {
        match value {
            false => OCRAM_MAM_STATUSR::OCRAM_MAM_STATUS_0,
            true => OCRAM_MAM_STATUSR::OCRAM_MAM_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_MAM_STATUS_0`"]
    #[inline]
    pub fn is_ocram_mam_status_0(&self) -> bool {
        *self == OCRAM_MAM_STATUSR::OCRAM_MAM_STATUS_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_MAM_STATUS_1`"]
    #[inline]
    pub fn is_ocram_mam_status_1(&self) -> bool {
        *self == OCRAM_MAM_STATUSR::OCRAM_MAM_STATUS_1
    }
}
#[doc = "Possible values of the field `ITCM_ERR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITCM_ERR_STATUSR {
    #[doc = "ITCM access error does not happen"]
    ITCM_ERR_STATUS_0,
    #[doc = "ITCM access error happens."]
    ITCM_ERR_STATUS_1,
}
impl ITCM_ERR_STATUSR {
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
            ITCM_ERR_STATUSR::ITCM_ERR_STATUS_0 => false,
            ITCM_ERR_STATUSR::ITCM_ERR_STATUS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITCM_ERR_STATUSR {
        match value {
            false => ITCM_ERR_STATUSR::ITCM_ERR_STATUS_0,
            true => ITCM_ERR_STATUSR::ITCM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STATUS_0`"]
    #[inline]
    pub fn is_itcm_err_status_0(&self) -> bool {
        *self == ITCM_ERR_STATUSR::ITCM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STATUS_1`"]
    #[inline]
    pub fn is_itcm_err_status_1(&self) -> bool {
        *self == ITCM_ERR_STATUSR::ITCM_ERR_STATUS_1
    }
}
#[doc = "Possible values of the field `DTCM_ERR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCM_ERR_STATUSR {
    #[doc = "DTCM access error does not happen"]
    DTCM_ERR_STATUS_0,
    #[doc = "DTCM access error happens."]
    DTCM_ERR_STATUS_1,
}
impl DTCM_ERR_STATUSR {
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
            DTCM_ERR_STATUSR::DTCM_ERR_STATUS_0 => false,
            DTCM_ERR_STATUSR::DTCM_ERR_STATUS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTCM_ERR_STATUSR {
        match value {
            false => DTCM_ERR_STATUSR::DTCM_ERR_STATUS_0,
            true => DTCM_ERR_STATUSR::DTCM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STATUS_0`"]
    #[inline]
    pub fn is_dtcm_err_status_0(&self) -> bool {
        *self == DTCM_ERR_STATUSR::DTCM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STATUS_1`"]
    #[inline]
    pub fn is_dtcm_err_status_1(&self) -> bool {
        *self == DTCM_ERR_STATUSR::DTCM_ERR_STATUS_1
    }
}
#[doc = "Possible values of the field `OCRAM_ERR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_ERR_STATUSR {
    #[doc = "OCRAM access error does not happen"]
    OCRAM_ERR_STATUS_0,
    #[doc = "OCRAM access error happens."]
    OCRAM_ERR_STATUS_1,
}
impl OCRAM_ERR_STATUSR {
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
            OCRAM_ERR_STATUSR::OCRAM_ERR_STATUS_0 => false,
            OCRAM_ERR_STATUSR::OCRAM_ERR_STATUS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCRAM_ERR_STATUSR {
        match value {
            false => OCRAM_ERR_STATUSR::OCRAM_ERR_STATUS_0,
            true => OCRAM_ERR_STATUSR::OCRAM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STATUS_0`"]
    #[inline]
    pub fn is_ocram_err_status_0(&self) -> bool {
        *self == OCRAM_ERR_STATUSR::OCRAM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STATUS_1`"]
    #[inline]
    pub fn is_ocram_err_status_1(&self) -> bool {
        *self == OCRAM_ERR_STATUSR::OCRAM_ERR_STATUS_1
    }
}
#[doc = "Values that can be written to the field `ITCM_MAM_STATUS`"]
pub enum ITCM_MAM_STATUSW {
    #[doc = "ITCM did not access magic address."]
    ITCM_MAM_STATUS_0,
    #[doc = "ITCM accessed magic address."]
    ITCM_MAM_STATUS_1,
}
impl ITCM_MAM_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITCM_MAM_STATUSW::ITCM_MAM_STATUS_0 => false,
            ITCM_MAM_STATUSW::ITCM_MAM_STATUS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITCM_MAM_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCM_MAM_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITCM_MAM_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ITCM did not access magic address."]
    #[inline]
    pub fn itcm_mam_status_0(self) -> &'a mut W {
        self.variant(ITCM_MAM_STATUSW::ITCM_MAM_STATUS_0)
    }
    #[doc = "ITCM accessed magic address."]
    #[inline]
    pub fn itcm_mam_status_1(self) -> &'a mut W {
        self.variant(ITCM_MAM_STATUSW::ITCM_MAM_STATUS_1)
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
#[doc = "Values that can be written to the field `DTCM_MAM_STATUS`"]
pub enum DTCM_MAM_STATUSW {
    #[doc = "DTCM did not access magic address."]
    DTCM_MAM_STATUS_0,
    #[doc = "DTCM accessed magic address."]
    DTCM_MAM_STATUS_1,
}
impl DTCM_MAM_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTCM_MAM_STATUSW::DTCM_MAM_STATUS_0 => false,
            DTCM_MAM_STATUSW::DTCM_MAM_STATUS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCM_MAM_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCM_MAM_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCM_MAM_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DTCM did not access magic address."]
    #[inline]
    pub fn dtcm_mam_status_0(self) -> &'a mut W {
        self.variant(DTCM_MAM_STATUSW::DTCM_MAM_STATUS_0)
    }
    #[doc = "DTCM accessed magic address."]
    #[inline]
    pub fn dtcm_mam_status_1(self) -> &'a mut W {
        self.variant(DTCM_MAM_STATUSW::DTCM_MAM_STATUS_1)
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
#[doc = "Values that can be written to the field `OCRAM_MAM_STATUS`"]
pub enum OCRAM_MAM_STATUSW {
    #[doc = "OCRAM did not access magic address."]
    OCRAM_MAM_STATUS_0,
    #[doc = "OCRAM accessed magic address."]
    OCRAM_MAM_STATUS_1,
}
impl OCRAM_MAM_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCRAM_MAM_STATUSW::OCRAM_MAM_STATUS_0 => false,
            OCRAM_MAM_STATUSW::OCRAM_MAM_STATUS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCRAM_MAM_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_MAM_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCRAM_MAM_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OCRAM did not access magic address."]
    #[inline]
    pub fn ocram_mam_status_0(self) -> &'a mut W {
        self.variant(OCRAM_MAM_STATUSW::OCRAM_MAM_STATUS_0)
    }
    #[doc = "OCRAM accessed magic address."]
    #[inline]
    pub fn ocram_mam_status_1(self) -> &'a mut W {
        self.variant(OCRAM_MAM_STATUSW::OCRAM_MAM_STATUS_1)
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
#[doc = "Values that can be written to the field `ITCM_ERR_STATUS`"]
pub enum ITCM_ERR_STATUSW {
    #[doc = "ITCM access error does not happen"]
    ITCM_ERR_STATUS_0,
    #[doc = "ITCM access error happens."]
    ITCM_ERR_STATUS_1,
}
impl ITCM_ERR_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITCM_ERR_STATUSW::ITCM_ERR_STATUS_0 => false,
            ITCM_ERR_STATUSW::ITCM_ERR_STATUS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITCM_ERR_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCM_ERR_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITCM_ERR_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ITCM access error does not happen"]
    #[inline]
    pub fn itcm_err_status_0(self) -> &'a mut W {
        self.variant(ITCM_ERR_STATUSW::ITCM_ERR_STATUS_0)
    }
    #[doc = "ITCM access error happens."]
    #[inline]
    pub fn itcm_err_status_1(self) -> &'a mut W {
        self.variant(ITCM_ERR_STATUSW::ITCM_ERR_STATUS_1)
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
#[doc = "Values that can be written to the field `DTCM_ERR_STATUS`"]
pub enum DTCM_ERR_STATUSW {
    #[doc = "DTCM access error does not happen"]
    DTCM_ERR_STATUS_0,
    #[doc = "DTCM access error happens."]
    DTCM_ERR_STATUS_1,
}
impl DTCM_ERR_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTCM_ERR_STATUSW::DTCM_ERR_STATUS_0 => false,
            DTCM_ERR_STATUSW::DTCM_ERR_STATUS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCM_ERR_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCM_ERR_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCM_ERR_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DTCM access error does not happen"]
    #[inline]
    pub fn dtcm_err_status_0(self) -> &'a mut W {
        self.variant(DTCM_ERR_STATUSW::DTCM_ERR_STATUS_0)
    }
    #[doc = "DTCM access error happens."]
    #[inline]
    pub fn dtcm_err_status_1(self) -> &'a mut W {
        self.variant(DTCM_ERR_STATUSW::DTCM_ERR_STATUS_1)
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
#[doc = "Values that can be written to the field `OCRAM_ERR_STATUS`"]
pub enum OCRAM_ERR_STATUSW {
    #[doc = "OCRAM access error does not happen"]
    OCRAM_ERR_STATUS_0,
    #[doc = "OCRAM access error happens."]
    OCRAM_ERR_STATUS_1,
}
impl OCRAM_ERR_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCRAM_ERR_STATUSW::OCRAM_ERR_STATUS_0 => false,
            OCRAM_ERR_STATUSW::OCRAM_ERR_STATUS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCRAM_ERR_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_ERR_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCRAM_ERR_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OCRAM access error does not happen"]
    #[inline]
    pub fn ocram_err_status_0(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STATUSW::OCRAM_ERR_STATUS_0)
    }
    #[doc = "OCRAM access error happens."]
    #[inline]
    pub fn ocram_err_status_1(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STATUSW::OCRAM_ERR_STATUS_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - ITCM Magic Address Match Status"]
    #[inline]
    pub fn itcm_mam_status(&self) -> ITCM_MAM_STATUSR {
        ITCM_MAM_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DTCM Magic Address Match Status"]
    #[inline]
    pub fn dtcm_mam_status(&self) -> DTCM_MAM_STATUSR {
        DTCM_MAM_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - OCRAM Magic Address Match Status"]
    #[inline]
    pub fn ocram_mam_status(&self) -> OCRAM_MAM_STATUSR {
        OCRAM_MAM_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ITCM Access Error Status"]
    #[inline]
    pub fn itcm_err_status(&self) -> ITCM_ERR_STATUSR {
        ITCM_ERR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DTCM Access Error Status"]
    #[inline]
    pub fn dtcm_err_status(&self) -> DTCM_ERR_STATUSR {
        DTCM_ERR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - OCRAM Access Error Status"]
    #[inline]
    pub fn ocram_err_status(&self) -> OCRAM_ERR_STATUSR {
        OCRAM_ERR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - ITCM Magic Address Match Status"]
    #[inline]
    pub fn itcm_mam_status(&mut self) -> _ITCM_MAM_STATUSW {
        _ITCM_MAM_STATUSW { w: self }
    }
    #[doc = "Bit 1 - DTCM Magic Address Match Status"]
    #[inline]
    pub fn dtcm_mam_status(&mut self) -> _DTCM_MAM_STATUSW {
        _DTCM_MAM_STATUSW { w: self }
    }
    #[doc = "Bit 2 - OCRAM Magic Address Match Status"]
    #[inline]
    pub fn ocram_mam_status(&mut self) -> _OCRAM_MAM_STATUSW {
        _OCRAM_MAM_STATUSW { w: self }
    }
    #[doc = "Bit 3 - ITCM Access Error Status"]
    #[inline]
    pub fn itcm_err_status(&mut self) -> _ITCM_ERR_STATUSW {
        _ITCM_ERR_STATUSW { w: self }
    }
    #[doc = "Bit 4 - DTCM Access Error Status"]
    #[inline]
    pub fn dtcm_err_status(&mut self) -> _DTCM_ERR_STATUSW {
        _DTCM_ERR_STATUSW { w: self }
    }
    #[doc = "Bit 5 - OCRAM Access Error Status"]
    #[inline]
    pub fn ocram_err_status(&mut self) -> _OCRAM_ERR_STATUSW {
        _OCRAM_ERR_STATUSW { w: self }
    }
}
