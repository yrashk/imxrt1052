#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_STAT_EN {
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
#[doc = "Possible values of the field `ITCM_MAM_STAT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITCM_MAM_STAT_ENR {
    #[doc = "Masked"]
    ITCM_MAM_STAT_EN_0,
    #[doc = "Enabled"]
    ITCM_MAM_STAT_EN_1,
}
impl ITCM_MAM_STAT_ENR {
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
            ITCM_MAM_STAT_ENR::ITCM_MAM_STAT_EN_0 => false,
            ITCM_MAM_STAT_ENR::ITCM_MAM_STAT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITCM_MAM_STAT_ENR {
        match value {
            false => ITCM_MAM_STAT_ENR::ITCM_MAM_STAT_EN_0,
            true => ITCM_MAM_STAT_ENR::ITCM_MAM_STAT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITCM_MAM_STAT_EN_0`"]
    #[inline]
    pub fn is_itcm_mam_stat_en_0(&self) -> bool {
        *self == ITCM_MAM_STAT_ENR::ITCM_MAM_STAT_EN_0
    }
    #[doc = "Checks if the value of the field is `ITCM_MAM_STAT_EN_1`"]
    #[inline]
    pub fn is_itcm_mam_stat_en_1(&self) -> bool {
        *self == ITCM_MAM_STAT_ENR::ITCM_MAM_STAT_EN_1
    }
}
#[doc = "Possible values of the field `DTCM_MAM_STAT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCM_MAM_STAT_ENR {
    #[doc = "Masked"]
    DTCM_MAM_STAT_EN_0,
    #[doc = "Enabled"]
    DTCM_MAM_STAT_EN_1,
}
impl DTCM_MAM_STAT_ENR {
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
            DTCM_MAM_STAT_ENR::DTCM_MAM_STAT_EN_0 => false,
            DTCM_MAM_STAT_ENR::DTCM_MAM_STAT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTCM_MAM_STAT_ENR {
        match value {
            false => DTCM_MAM_STAT_ENR::DTCM_MAM_STAT_EN_0,
            true => DTCM_MAM_STAT_ENR::DTCM_MAM_STAT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTCM_MAM_STAT_EN_0`"]
    #[inline]
    pub fn is_dtcm_mam_stat_en_0(&self) -> bool {
        *self == DTCM_MAM_STAT_ENR::DTCM_MAM_STAT_EN_0
    }
    #[doc = "Checks if the value of the field is `DTCM_MAM_STAT_EN_1`"]
    #[inline]
    pub fn is_dtcm_mam_stat_en_1(&self) -> bool {
        *self == DTCM_MAM_STAT_ENR::DTCM_MAM_STAT_EN_1
    }
}
#[doc = "Possible values of the field `OCRAM_MAM_STAT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_MAM_STAT_ENR {
    #[doc = "Masked"]
    OCRAM_MAM_STAT_EN_0,
    #[doc = "Enabled"]
    OCRAM_MAM_STAT_EN_1,
}
impl OCRAM_MAM_STAT_ENR {
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
            OCRAM_MAM_STAT_ENR::OCRAM_MAM_STAT_EN_0 => false,
            OCRAM_MAM_STAT_ENR::OCRAM_MAM_STAT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCRAM_MAM_STAT_ENR {
        match value {
            false => OCRAM_MAM_STAT_ENR::OCRAM_MAM_STAT_EN_0,
            true => OCRAM_MAM_STAT_ENR::OCRAM_MAM_STAT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_MAM_STAT_EN_0`"]
    #[inline]
    pub fn is_ocram_mam_stat_en_0(&self) -> bool {
        *self == OCRAM_MAM_STAT_ENR::OCRAM_MAM_STAT_EN_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_MAM_STAT_EN_1`"]
    #[inline]
    pub fn is_ocram_mam_stat_en_1(&self) -> bool {
        *self == OCRAM_MAM_STAT_ENR::OCRAM_MAM_STAT_EN_1
    }
}
#[doc = "Possible values of the field `ITCM_ERR_STAT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITCM_ERR_STAT_ENR {
    #[doc = "Masked"]
    ITCM_ERR_STAT_EN_0,
    #[doc = "Enabled"]
    ITCM_ERR_STAT_EN_1,
}
impl ITCM_ERR_STAT_ENR {
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
            ITCM_ERR_STAT_ENR::ITCM_ERR_STAT_EN_0 => false,
            ITCM_ERR_STAT_ENR::ITCM_ERR_STAT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITCM_ERR_STAT_ENR {
        match value {
            false => ITCM_ERR_STAT_ENR::ITCM_ERR_STAT_EN_0,
            true => ITCM_ERR_STAT_ENR::ITCM_ERR_STAT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STAT_EN_0`"]
    #[inline]
    pub fn is_itcm_err_stat_en_0(&self) -> bool {
        *self == ITCM_ERR_STAT_ENR::ITCM_ERR_STAT_EN_0
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STAT_EN_1`"]
    #[inline]
    pub fn is_itcm_err_stat_en_1(&self) -> bool {
        *self == ITCM_ERR_STAT_ENR::ITCM_ERR_STAT_EN_1
    }
}
#[doc = "Possible values of the field `DTCM_ERR_STAT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCM_ERR_STAT_ENR {
    #[doc = "Masked"]
    DTCM_ERR_STAT_EN_0,
    #[doc = "Enabled"]
    DTCM_ERR_STAT_EN_1,
}
impl DTCM_ERR_STAT_ENR {
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
            DTCM_ERR_STAT_ENR::DTCM_ERR_STAT_EN_0 => false,
            DTCM_ERR_STAT_ENR::DTCM_ERR_STAT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTCM_ERR_STAT_ENR {
        match value {
            false => DTCM_ERR_STAT_ENR::DTCM_ERR_STAT_EN_0,
            true => DTCM_ERR_STAT_ENR::DTCM_ERR_STAT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STAT_EN_0`"]
    #[inline]
    pub fn is_dtcm_err_stat_en_0(&self) -> bool {
        *self == DTCM_ERR_STAT_ENR::DTCM_ERR_STAT_EN_0
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STAT_EN_1`"]
    #[inline]
    pub fn is_dtcm_err_stat_en_1(&self) -> bool {
        *self == DTCM_ERR_STAT_ENR::DTCM_ERR_STAT_EN_1
    }
}
#[doc = "Possible values of the field `OCRAM_ERR_STAT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_ERR_STAT_ENR {
    #[doc = "Masked"]
    OCRAM_ERR_STAT_EN_0,
    #[doc = "Enabled"]
    OCRAM_ERR_STAT_EN_1,
}
impl OCRAM_ERR_STAT_ENR {
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
            OCRAM_ERR_STAT_ENR::OCRAM_ERR_STAT_EN_0 => false,
            OCRAM_ERR_STAT_ENR::OCRAM_ERR_STAT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCRAM_ERR_STAT_ENR {
        match value {
            false => OCRAM_ERR_STAT_ENR::OCRAM_ERR_STAT_EN_0,
            true => OCRAM_ERR_STAT_ENR::OCRAM_ERR_STAT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STAT_EN_0`"]
    #[inline]
    pub fn is_ocram_err_stat_en_0(&self) -> bool {
        *self == OCRAM_ERR_STAT_ENR::OCRAM_ERR_STAT_EN_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STAT_EN_1`"]
    #[inline]
    pub fn is_ocram_err_stat_en_1(&self) -> bool {
        *self == OCRAM_ERR_STAT_ENR::OCRAM_ERR_STAT_EN_1
    }
}
#[doc = "Values that can be written to the field `ITCM_MAM_STAT_EN`"]
pub enum ITCM_MAM_STAT_ENW {
    #[doc = "Masked"]
    ITCM_MAM_STAT_EN_0,
    #[doc = "Enabled"]
    ITCM_MAM_STAT_EN_1,
}
impl ITCM_MAM_STAT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITCM_MAM_STAT_ENW::ITCM_MAM_STAT_EN_0 => false,
            ITCM_MAM_STAT_ENW::ITCM_MAM_STAT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITCM_MAM_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCM_MAM_STAT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITCM_MAM_STAT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn itcm_mam_stat_en_0(self) -> &'a mut W {
        self.variant(ITCM_MAM_STAT_ENW::ITCM_MAM_STAT_EN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn itcm_mam_stat_en_1(self) -> &'a mut W {
        self.variant(ITCM_MAM_STAT_ENW::ITCM_MAM_STAT_EN_1)
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
#[doc = "Values that can be written to the field `DTCM_MAM_STAT_EN`"]
pub enum DTCM_MAM_STAT_ENW {
    #[doc = "Masked"]
    DTCM_MAM_STAT_EN_0,
    #[doc = "Enabled"]
    DTCM_MAM_STAT_EN_1,
}
impl DTCM_MAM_STAT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTCM_MAM_STAT_ENW::DTCM_MAM_STAT_EN_0 => false,
            DTCM_MAM_STAT_ENW::DTCM_MAM_STAT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCM_MAM_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCM_MAM_STAT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCM_MAM_STAT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dtcm_mam_stat_en_0(self) -> &'a mut W {
        self.variant(DTCM_MAM_STAT_ENW::DTCM_MAM_STAT_EN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dtcm_mam_stat_en_1(self) -> &'a mut W {
        self.variant(DTCM_MAM_STAT_ENW::DTCM_MAM_STAT_EN_1)
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
#[doc = "Values that can be written to the field `OCRAM_MAM_STAT_EN`"]
pub enum OCRAM_MAM_STAT_ENW {
    #[doc = "Masked"]
    OCRAM_MAM_STAT_EN_0,
    #[doc = "Enabled"]
    OCRAM_MAM_STAT_EN_1,
}
impl OCRAM_MAM_STAT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCRAM_MAM_STAT_ENW::OCRAM_MAM_STAT_EN_0 => false,
            OCRAM_MAM_STAT_ENW::OCRAM_MAM_STAT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCRAM_MAM_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_MAM_STAT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCRAM_MAM_STAT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ocram_mam_stat_en_0(self) -> &'a mut W {
        self.variant(OCRAM_MAM_STAT_ENW::OCRAM_MAM_STAT_EN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ocram_mam_stat_en_1(self) -> &'a mut W {
        self.variant(OCRAM_MAM_STAT_ENW::OCRAM_MAM_STAT_EN_1)
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
#[doc = "Values that can be written to the field `ITCM_ERR_STAT_EN`"]
pub enum ITCM_ERR_STAT_ENW {
    #[doc = "Masked"]
    ITCM_ERR_STAT_EN_0,
    #[doc = "Enabled"]
    ITCM_ERR_STAT_EN_1,
}
impl ITCM_ERR_STAT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITCM_ERR_STAT_ENW::ITCM_ERR_STAT_EN_0 => false,
            ITCM_ERR_STAT_ENW::ITCM_ERR_STAT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITCM_ERR_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCM_ERR_STAT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITCM_ERR_STAT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn itcm_err_stat_en_0(self) -> &'a mut W {
        self.variant(ITCM_ERR_STAT_ENW::ITCM_ERR_STAT_EN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn itcm_err_stat_en_1(self) -> &'a mut W {
        self.variant(ITCM_ERR_STAT_ENW::ITCM_ERR_STAT_EN_1)
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
#[doc = "Values that can be written to the field `DTCM_ERR_STAT_EN`"]
pub enum DTCM_ERR_STAT_ENW {
    #[doc = "Masked"]
    DTCM_ERR_STAT_EN_0,
    #[doc = "Enabled"]
    DTCM_ERR_STAT_EN_1,
}
impl DTCM_ERR_STAT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTCM_ERR_STAT_ENW::DTCM_ERR_STAT_EN_0 => false,
            DTCM_ERR_STAT_ENW::DTCM_ERR_STAT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCM_ERR_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCM_ERR_STAT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCM_ERR_STAT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn dtcm_err_stat_en_0(self) -> &'a mut W {
        self.variant(DTCM_ERR_STAT_ENW::DTCM_ERR_STAT_EN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dtcm_err_stat_en_1(self) -> &'a mut W {
        self.variant(DTCM_ERR_STAT_ENW::DTCM_ERR_STAT_EN_1)
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
#[doc = "Values that can be written to the field `OCRAM_ERR_STAT_EN`"]
pub enum OCRAM_ERR_STAT_ENW {
    #[doc = "Masked"]
    OCRAM_ERR_STAT_EN_0,
    #[doc = "Enabled"]
    OCRAM_ERR_STAT_EN_1,
}
impl OCRAM_ERR_STAT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCRAM_ERR_STAT_ENW::OCRAM_ERR_STAT_EN_0 => false,
            OCRAM_ERR_STAT_ENW::OCRAM_ERR_STAT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCRAM_ERR_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_ERR_STAT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCRAM_ERR_STAT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn ocram_err_stat_en_0(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STAT_ENW::OCRAM_ERR_STAT_EN_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ocram_err_stat_en_1(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STAT_ENW::OCRAM_ERR_STAT_EN_1)
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
    #[doc = "Bit 0 - ITCM Magic Address Match Status Enable"]
    #[inline]
    pub fn itcm_mam_stat_en(&self) -> ITCM_MAM_STAT_ENR {
        ITCM_MAM_STAT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DTCM Magic Address Match Status Enable"]
    #[inline]
    pub fn dtcm_mam_stat_en(&self) -> DTCM_MAM_STAT_ENR {
        DTCM_MAM_STAT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - OCRAM Magic Address Match Status Enable"]
    #[inline]
    pub fn ocram_mam_stat_en(&self) -> OCRAM_MAM_STAT_ENR {
        OCRAM_MAM_STAT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ITCM Access Error Status Enable"]
    #[inline]
    pub fn itcm_err_stat_en(&self) -> ITCM_ERR_STAT_ENR {
        ITCM_ERR_STAT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DTCM Access Error Status Enable"]
    #[inline]
    pub fn dtcm_err_stat_en(&self) -> DTCM_ERR_STAT_ENR {
        DTCM_ERR_STAT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - OCRAM Access Error Status Enable"]
    #[inline]
    pub fn ocram_err_stat_en(&self) -> OCRAM_ERR_STAT_ENR {
        OCRAM_ERR_STAT_ENR::_from({
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
    #[doc = "Bit 0 - ITCM Magic Address Match Status Enable"]
    #[inline]
    pub fn itcm_mam_stat_en(&mut self) -> _ITCM_MAM_STAT_ENW {
        _ITCM_MAM_STAT_ENW { w: self }
    }
    #[doc = "Bit 1 - DTCM Magic Address Match Status Enable"]
    #[inline]
    pub fn dtcm_mam_stat_en(&mut self) -> _DTCM_MAM_STAT_ENW {
        _DTCM_MAM_STAT_ENW { w: self }
    }
    #[doc = "Bit 2 - OCRAM Magic Address Match Status Enable"]
    #[inline]
    pub fn ocram_mam_stat_en(&mut self) -> _OCRAM_MAM_STAT_ENW {
        _OCRAM_MAM_STAT_ENW { w: self }
    }
    #[doc = "Bit 3 - ITCM Access Error Status Enable"]
    #[inline]
    pub fn itcm_err_stat_en(&mut self) -> _ITCM_ERR_STAT_ENW {
        _ITCM_ERR_STAT_ENW { w: self }
    }
    #[doc = "Bit 4 - DTCM Access Error Status Enable"]
    #[inline]
    pub fn dtcm_err_stat_en(&mut self) -> _DTCM_ERR_STAT_ENW {
        _DTCM_ERR_STAT_ENW { w: self }
    }
    #[doc = "Bit 5 - OCRAM Access Error Status Enable"]
    #[inline]
    pub fn ocram_err_stat_en(&mut self) -> _OCRAM_ERR_STAT_ENW {
        _OCRAM_ERR_STAT_ENW { w: self }
    }
}
