#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIER {
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
#[doc = "Possible values of the field `TDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIER {
    #[doc = "Disabled"]
    TDIE_0,
    #[doc = "Enabled"]
    TDIE_1,
}
impl TDIER {
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
            TDIER::TDIE_0 => false,
            TDIER::TDIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDIER {
        match value {
            false => TDIER::TDIE_0,
            true => TDIER::TDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDIE_0`"]
    #[inline]
    pub fn is_tdie_0(&self) -> bool {
        *self == TDIER::TDIE_0
    }
    #[doc = "Checks if the value of the field is `TDIE_1`"]
    #[inline]
    pub fn is_tdie_1(&self) -> bool {
        *self == TDIER::TDIE_1
    }
}
#[doc = "Possible values of the field `RDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIER {
    #[doc = "Disabled"]
    RDIE_0,
    #[doc = "Enabled"]
    RDIE_1,
}
impl RDIER {
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
            RDIER::RDIE_0 => false,
            RDIER::RDIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDIER {
        match value {
            false => RDIER::RDIE_0,
            true => RDIER::RDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDIE_0`"]
    #[inline]
    pub fn is_rdie_0(&self) -> bool {
        *self == RDIER::RDIE_0
    }
    #[doc = "Checks if the value of the field is `RDIE_1`"]
    #[inline]
    pub fn is_rdie_1(&self) -> bool {
        *self == RDIER::RDIE_1
    }
}
#[doc = "Possible values of the field `EPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIER {
    #[doc = "Disabled"]
    EPIE_0,
    #[doc = "Enabled"]
    EPIE_1,
}
impl EPIER {
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
            EPIER::EPIE_0 => false,
            EPIER::EPIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIER {
        match value {
            false => EPIER::EPIE_0,
            true => EPIER::EPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EPIE_0`"]
    #[inline]
    pub fn is_epie_0(&self) -> bool {
        *self == EPIER::EPIE_0
    }
    #[doc = "Checks if the value of the field is `EPIE_1`"]
    #[inline]
    pub fn is_epie_1(&self) -> bool {
        *self == EPIER::EPIE_1
    }
}
#[doc = "Possible values of the field `SDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIER {
    #[doc = "Disabled"]
    SDIE_0,
    #[doc = "Enabled"]
    SDIE_1,
}
impl SDIER {
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
            SDIER::SDIE_0 => false,
            SDIER::SDIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDIER {
        match value {
            false => SDIER::SDIE_0,
            true => SDIER::SDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDIE_0`"]
    #[inline]
    pub fn is_sdie_0(&self) -> bool {
        *self == SDIER::SDIE_0
    }
    #[doc = "Checks if the value of the field is `SDIE_1`"]
    #[inline]
    pub fn is_sdie_1(&self) -> bool {
        *self == SDIER::SDIE_1
    }
}
#[doc = "Possible values of the field `NDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDIER {
    #[doc = "Disabled"]
    NDIE_0,
    #[doc = "Enabled"]
    NDIE_1,
}
impl NDIER {
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
            NDIER::NDIE_0 => false,
            NDIER::NDIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDIER {
        match value {
            false => NDIER::NDIE_0,
            true => NDIER::NDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDIE_0`"]
    #[inline]
    pub fn is_ndie_0(&self) -> bool {
        *self == NDIER::NDIE_0
    }
    #[doc = "Checks if the value of the field is `NDIE_1`"]
    #[inline]
    pub fn is_ndie_1(&self) -> bool {
        *self == NDIER::NDIE_1
    }
}
#[doc = "Possible values of the field `ALIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIER {
    #[doc = "Disabled"]
    ALIE_0,
    #[doc = "Enabled"]
    ALIE_1,
}
impl ALIER {
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
            ALIER::ALIE_0 => false,
            ALIER::ALIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALIER {
        match value {
            false => ALIER::ALIE_0,
            true => ALIER::ALIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ALIE_0`"]
    #[inline]
    pub fn is_alie_0(&self) -> bool {
        *self == ALIER::ALIE_0
    }
    #[doc = "Checks if the value of the field is `ALIE_1`"]
    #[inline]
    pub fn is_alie_1(&self) -> bool {
        *self == ALIER::ALIE_1
    }
}
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "Enabled"]
    FEIE_0,
    #[doc = "Disabled"]
    FEIE_1,
}
impl FEIER {
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
            FEIER::FEIE_0 => false,
            FEIER::FEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIER {
        match value {
            false => FEIER::FEIE_0,
            true => FEIER::FEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEIE_0`"]
    #[inline]
    pub fn is_feie_0(&self) -> bool {
        *self == FEIER::FEIE_0
    }
    #[doc = "Checks if the value of the field is `FEIE_1`"]
    #[inline]
    pub fn is_feie_1(&self) -> bool {
        *self == FEIER::FEIE_1
    }
}
#[doc = "Possible values of the field `PLTIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTIER {
    #[doc = "Disabled"]
    PLTIE_0,
    #[doc = "Enabled"]
    PLTIE_1,
}
impl PLTIER {
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
            PLTIER::PLTIE_0 => false,
            PLTIER::PLTIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLTIER {
        match value {
            false => PLTIER::PLTIE_0,
            true => PLTIER::PLTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLTIE_0`"]
    #[inline]
    pub fn is_pltie_0(&self) -> bool {
        *self == PLTIER::PLTIE_0
    }
    #[doc = "Checks if the value of the field is `PLTIE_1`"]
    #[inline]
    pub fn is_pltie_1(&self) -> bool {
        *self == PLTIER::PLTIE_1
    }
}
#[doc = "Possible values of the field `DMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIER {
    #[doc = "Disabled"]
    DMIE_0,
    #[doc = "Enabled"]
    DMIE_1,
}
impl DMIER {
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
            DMIER::DMIE_0 => false,
            DMIER::DMIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMIER {
        match value {
            false => DMIER::DMIE_0,
            true => DMIER::DMIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIE_0`"]
    #[inline]
    pub fn is_dmie_0(&self) -> bool {
        *self == DMIER::DMIE_0
    }
    #[doc = "Checks if the value of the field is `DMIE_1`"]
    #[inline]
    pub fn is_dmie_1(&self) -> bool {
        *self == DMIER::DMIE_1
    }
}
#[doc = "Values that can be written to the field `TDIE`"]
pub enum TDIEW {
    #[doc = "Disabled"]
    TDIE_0,
    #[doc = "Enabled"]
    TDIE_1,
}
impl TDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDIEW::TDIE_0 => false,
            TDIEW::TDIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn tdie_0(self) -> &'a mut W {
        self.variant(TDIEW::TDIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tdie_1(self) -> &'a mut W {
        self.variant(TDIEW::TDIE_1)
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
#[doc = "Values that can be written to the field `RDIE`"]
pub enum RDIEW {
    #[doc = "Disabled"]
    RDIE_0,
    #[doc = "Enabled"]
    RDIE_1,
}
impl RDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDIEW::RDIE_0 => false,
            RDIEW::RDIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn rdie_0(self) -> &'a mut W {
        self.variant(RDIEW::RDIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn rdie_1(self) -> &'a mut W {
        self.variant(RDIEW::RDIE_1)
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
#[doc = "Values that can be written to the field `EPIE`"]
pub enum EPIEW {
    #[doc = "Disabled"]
    EPIE_0,
    #[doc = "Enabled"]
    EPIE_1,
}
impl EPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIEW::EPIE_0 => false,
            EPIEW::EPIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn epie_0(self) -> &'a mut W {
        self.variant(EPIEW::EPIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn epie_1(self) -> &'a mut W {
        self.variant(EPIEW::EPIE_1)
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
#[doc = "Values that can be written to the field `SDIE`"]
pub enum SDIEW {
    #[doc = "Disabled"]
    SDIE_0,
    #[doc = "Enabled"]
    SDIE_1,
}
impl SDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDIEW::SDIE_0 => false,
            SDIEW::SDIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn sdie_0(self) -> &'a mut W {
        self.variant(SDIEW::SDIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn sdie_1(self) -> &'a mut W {
        self.variant(SDIEW::SDIE_1)
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
#[doc = "Values that can be written to the field `NDIE`"]
pub enum NDIEW {
    #[doc = "Disabled"]
    NDIE_0,
    #[doc = "Enabled"]
    NDIE_1,
}
impl NDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDIEW::NDIE_0 => false,
            NDIEW::NDIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn ndie_0(self) -> &'a mut W {
        self.variant(NDIEW::NDIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn ndie_1(self) -> &'a mut W {
        self.variant(NDIEW::NDIE_1)
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
#[doc = "Values that can be written to the field `ALIE`"]
pub enum ALIEW {
    #[doc = "Disabled"]
    ALIE_0,
    #[doc = "Enabled"]
    ALIE_1,
}
impl ALIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALIEW::ALIE_0 => false,
            ALIEW::ALIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ALIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn alie_0(self) -> &'a mut W {
        self.variant(ALIEW::ALIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn alie_1(self) -> &'a mut W {
        self.variant(ALIEW::ALIE_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "Enabled"]
    FEIE_0,
    #[doc = "Disabled"]
    FEIE_1,
}
impl FEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEIEW::FEIE_0 => false,
            FEIEW::FEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIEW::FEIE_0)
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn feie_1(self) -> &'a mut W {
        self.variant(FEIEW::FEIE_1)
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
#[doc = "Values that can be written to the field `PLTIE`"]
pub enum PLTIEW {
    #[doc = "Disabled"]
    PLTIE_0,
    #[doc = "Enabled"]
    PLTIE_1,
}
impl PLTIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLTIEW::PLTIE_0 => false,
            PLTIEW::PLTIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLTIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLTIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLTIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn pltie_0(self) -> &'a mut W {
        self.variant(PLTIEW::PLTIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn pltie_1(self) -> &'a mut W {
        self.variant(PLTIEW::PLTIE_1)
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
#[doc = "Values that can be written to the field `DMIE`"]
pub enum DMIEW {
    #[doc = "Disabled"]
    DMIE_0,
    #[doc = "Enabled"]
    DMIE_1,
}
impl DMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMIEW::DMIE_0 => false,
            DMIEW::DMIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn dmie_0(self) -> &'a mut W {
        self.variant(DMIEW::DMIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn dmie_1(self) -> &'a mut W {
        self.variant(DMIEW::DMIE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline]
    pub fn tdie(&self) -> TDIER {
        TDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline]
    pub fn rdie(&self) -> RDIER {
        RDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline]
    pub fn epie(&self) -> EPIER {
        EPIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline]
    pub fn sdie(&self) -> SDIER {
        SDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline]
    pub fn ndie(&self) -> NDIER {
        NDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline]
    pub fn alie(&self) -> ALIER {
        ALIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline]
    pub fn pltie(&self) -> PLTIER {
        PLTIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline]
    pub fn dmie(&self) -> DMIER {
        DMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline]
    pub fn tdie(&mut self) -> _TDIEW {
        _TDIEW { w: self }
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline]
    pub fn rdie(&mut self) -> _RDIEW {
        _RDIEW { w: self }
    }
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline]
    pub fn epie(&mut self) -> _EPIEW {
        _EPIEW { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline]
    pub fn sdie(&mut self) -> _SDIEW {
        _SDIEW { w: self }
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline]
    pub fn ndie(&mut self) -> _NDIEW {
        _NDIEW { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline]
    pub fn alie(&mut self) -> _ALIEW {
        _ALIEW { w: self }
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline]
    pub fn pltie(&mut self) -> _PLTIEW {
        _PLTIEW { w: self }
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline]
    pub fn dmie(&mut self) -> _DMIEW {
        _DMIEW { w: self }
    }
}
