#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIER {
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
#[doc = "Possible values of the field `AVIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVIER {
    #[doc = "Disabled"]
    AVIE_0,
    #[doc = "Enabled"]
    AVIE_1,
}
impl AVIER {
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
            AVIER::AVIE_0 => false,
            AVIER::AVIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVIER {
        match value {
            false => AVIER::AVIE_0,
            true => AVIER::AVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVIE_0`"]
    #[inline]
    pub fn is_avie_0(&self) -> bool {
        *self == AVIER::AVIE_0
    }
    #[doc = "Checks if the value of the field is `AVIE_1`"]
    #[inline]
    pub fn is_avie_1(&self) -> bool {
        *self == AVIER::AVIE_1
    }
}
#[doc = "Possible values of the field `TAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIER {
    #[doc = "Disabled"]
    TAIE_0,
    #[doc = "Enabled"]
    TAIE_1,
}
impl TAIER {
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
            TAIER::TAIE_0 => false,
            TAIER::TAIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAIER {
        match value {
            false => TAIER::TAIE_0,
            true => TAIER::TAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIE_0`"]
    #[inline]
    pub fn is_taie_0(&self) -> bool {
        *self == TAIER::TAIE_0
    }
    #[doc = "Checks if the value of the field is `TAIE_1`"]
    #[inline]
    pub fn is_taie_1(&self) -> bool {
        *self == TAIER::TAIE_1
    }
}
#[doc = "Possible values of the field `RSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIER {
    #[doc = "Disabled"]
    RSIE_0,
    #[doc = "Enabled"]
    RSIE_1,
}
impl RSIER {
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
            RSIER::RSIE_0 => false,
            RSIER::RSIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSIER {
        match value {
            false => RSIER::RSIE_0,
            true => RSIER::RSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSIE_0`"]
    #[inline]
    pub fn is_rsie_0(&self) -> bool {
        *self == RSIER::RSIE_0
    }
    #[doc = "Checks if the value of the field is `RSIE_1`"]
    #[inline]
    pub fn is_rsie_1(&self) -> bool {
        *self == RSIER::RSIE_1
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
#[doc = "Possible values of the field `BEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIER {
    #[doc = "Disabled"]
    BEIE_0,
    #[doc = "Enabled"]
    BEIE_1,
}
impl BEIER {
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
            BEIER::BEIE_0 => false,
            BEIER::BEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEIER {
        match value {
            false => BEIER::BEIE_0,
            true => BEIER::BEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEIE_0`"]
    #[inline]
    pub fn is_beie_0(&self) -> bool {
        *self == BEIER::BEIE_0
    }
    #[doc = "Checks if the value of the field is `BEIE_1`"]
    #[inline]
    pub fn is_beie_1(&self) -> bool {
        *self == BEIER::BEIE_1
    }
}
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "Disabled"]
    FEIE_0,
    #[doc = "Enabled"]
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
#[doc = "Possible values of the field `AM0IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM0IER {
    #[doc = "Enabled"]
    AM0IE_0,
    #[doc = "Disabled"]
    AM0IE_1,
}
impl AM0IER {
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
            AM0IER::AM0IE_0 => false,
            AM0IER::AM0IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM0IER {
        match value {
            false => AM0IER::AM0IE_0,
            true => AM0IER::AM0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM0IE_0`"]
    #[inline]
    pub fn is_am0ie_0(&self) -> bool {
        *self == AM0IER::AM0IE_0
    }
    #[doc = "Checks if the value of the field is `AM0IE_1`"]
    #[inline]
    pub fn is_am0ie_1(&self) -> bool {
        *self == AM0IER::AM0IE_1
    }
}
#[doc = "Possible values of the field `AM1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM1FR {
    #[doc = "Disabled"]
    AM1F_0,
    #[doc = "Enabled"]
    AM1F_1,
}
impl AM1FR {
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
            AM1FR::AM1F_0 => false,
            AM1FR::AM1F_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM1FR {
        match value {
            false => AM1FR::AM1F_0,
            true => AM1FR::AM1F_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM1F_0`"]
    #[inline]
    pub fn is_am1f_0(&self) -> bool {
        *self == AM1FR::AM1F_0
    }
    #[doc = "Checks if the value of the field is `AM1F_1`"]
    #[inline]
    pub fn is_am1f_1(&self) -> bool {
        *self == AM1FR::AM1F_1
    }
}
#[doc = "Possible values of the field `GCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCIER {
    #[doc = "Disabled"]
    GCIE_0,
    #[doc = "Enabled"]
    GCIE_1,
}
impl GCIER {
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
            GCIER::GCIE_0 => false,
            GCIER::GCIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCIER {
        match value {
            false => GCIER::GCIE_0,
            true => GCIER::GCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCIE_0`"]
    #[inline]
    pub fn is_gcie_0(&self) -> bool {
        *self == GCIER::GCIE_0
    }
    #[doc = "Checks if the value of the field is `GCIE_1`"]
    #[inline]
    pub fn is_gcie_1(&self) -> bool {
        *self == GCIER::GCIE_1
    }
}
#[doc = "Possible values of the field `SARIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SARIER {
    #[doc = "Disabled"]
    SARIE_0,
    #[doc = "Enabled"]
    SARIE_1,
}
impl SARIER {
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
            SARIER::SARIE_0 => false,
            SARIER::SARIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SARIER {
        match value {
            false => SARIER::SARIE_0,
            true => SARIER::SARIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SARIE_0`"]
    #[inline]
    pub fn is_sarie_0(&self) -> bool {
        *self == SARIER::SARIE_0
    }
    #[doc = "Checks if the value of the field is `SARIE_1`"]
    #[inline]
    pub fn is_sarie_1(&self) -> bool {
        *self == SARIER::SARIE_1
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
#[doc = "Values that can be written to the field `AVIE`"]
pub enum AVIEW {
    #[doc = "Disabled"]
    AVIE_0,
    #[doc = "Enabled"]
    AVIE_1,
}
impl AVIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVIEW::AVIE_0 => false,
            AVIEW::AVIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn avie_0(self) -> &'a mut W {
        self.variant(AVIEW::AVIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn avie_1(self) -> &'a mut W {
        self.variant(AVIEW::AVIE_1)
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
#[doc = "Values that can be written to the field `TAIE`"]
pub enum TAIEW {
    #[doc = "Disabled"]
    TAIE_0,
    #[doc = "Enabled"]
    TAIE_1,
}
impl TAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAIEW::TAIE_0 => false,
            TAIEW::TAIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn taie_0(self) -> &'a mut W {
        self.variant(TAIEW::TAIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn taie_1(self) -> &'a mut W {
        self.variant(TAIEW::TAIE_1)
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
#[doc = "Values that can be written to the field `RSIE`"]
pub enum RSIEW {
    #[doc = "Disabled"]
    RSIE_0,
    #[doc = "Enabled"]
    RSIE_1,
}
impl RSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSIEW::RSIE_0 => false,
            RSIEW::RSIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn rsie_0(self) -> &'a mut W {
        self.variant(RSIEW::RSIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn rsie_1(self) -> &'a mut W {
        self.variant(RSIEW::RSIE_1)
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
#[doc = "Values that can be written to the field `BEIE`"]
pub enum BEIEW {
    #[doc = "Disabled"]
    BEIE_0,
    #[doc = "Enabled"]
    BEIE_1,
}
impl BEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEIEW::BEIE_0 => false,
            BEIEW::BEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _BEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn beie_0(self) -> &'a mut W {
        self.variant(BEIEW::BEIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn beie_1(self) -> &'a mut W {
        self.variant(BEIEW::BEIE_1)
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
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "Disabled"]
    FEIE_0,
    #[doc = "Enabled"]
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
    #[doc = "Disabled"]
    #[inline]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIEW::FEIE_0)
    }
    #[doc = "Enabled"]
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AM0IE`"]
pub enum AM0IEW {
    #[doc = "Enabled"]
    AM0IE_0,
    #[doc = "Disabled"]
    AM0IE_1,
}
impl AM0IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AM0IEW::AM0IE_0 => false,
            AM0IEW::AM0IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AM0IEW<'a> {
    w: &'a mut W,
}
impl<'a> _AM0IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AM0IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn am0ie_0(self) -> &'a mut W {
        self.variant(AM0IEW::AM0IE_0)
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn am0ie_1(self) -> &'a mut W {
        self.variant(AM0IEW::AM0IE_1)
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
#[doc = "Values that can be written to the field `AM1F`"]
pub enum AM1FW {
    #[doc = "Disabled"]
    AM1F_0,
    #[doc = "Enabled"]
    AM1F_1,
}
impl AM1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AM1FW::AM1F_0 => false,
            AM1FW::AM1F_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AM1FW<'a> {
    w: &'a mut W,
}
impl<'a> _AM1FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AM1FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn am1f_0(self) -> &'a mut W {
        self.variant(AM1FW::AM1F_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn am1f_1(self) -> &'a mut W {
        self.variant(AM1FW::AM1F_1)
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
#[doc = "Values that can be written to the field `GCIE`"]
pub enum GCIEW {
    #[doc = "Disabled"]
    GCIE_0,
    #[doc = "Enabled"]
    GCIE_1,
}
impl GCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GCIEW::GCIE_0 => false,
            GCIEW::GCIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _GCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn gcie_0(self) -> &'a mut W {
        self.variant(GCIEW::GCIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn gcie_1(self) -> &'a mut W {
        self.variant(GCIEW::GCIE_1)
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
#[doc = "Values that can be written to the field `SARIE`"]
pub enum SARIEW {
    #[doc = "Disabled"]
    SARIE_0,
    #[doc = "Enabled"]
    SARIE_1,
}
impl SARIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SARIEW::SARIE_0 => false,
            SARIEW::SARIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SARIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SARIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SARIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn sarie_0(self) -> &'a mut W {
        self.variant(SARIEW::SARIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn sarie_1(self) -> &'a mut W {
        self.variant(SARIEW::SARIE_1)
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
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline]
    pub fn avie(&self) -> AVIER {
        AVIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline]
    pub fn taie(&self) -> TAIER {
        TAIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline]
    pub fn rsie(&self) -> RSIER {
        RSIER::_from({
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
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline]
    pub fn beie(&self) -> BEIER {
        BEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline]
    pub fn am0ie(&self) -> AM0IER {
        AM0IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline]
    pub fn am1f(&self) -> AM1FR {
        AM1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline]
    pub fn gcie(&self) -> GCIER {
        GCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline]
    pub fn sarie(&self) -> SARIER {
        SARIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline]
    pub fn avie(&mut self) -> _AVIEW {
        _AVIEW { w: self }
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline]
    pub fn taie(&mut self) -> _TAIEW {
        _TAIEW { w: self }
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline]
    pub fn rsie(&mut self) -> _RSIEW {
        _RSIEW { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline]
    pub fn sdie(&mut self) -> _SDIEW {
        _SDIEW { w: self }
    }
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline]
    pub fn beie(&mut self) -> _BEIEW {
        _BEIEW { w: self }
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline]
    pub fn am0ie(&mut self) -> _AM0IEW {
        _AM0IEW { w: self }
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline]
    pub fn am1f(&mut self) -> _AM1FW {
        _AM1FW { w: self }
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline]
    pub fn gcie(&mut self) -> _GCIEW {
        _GCIEW { w: self }
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline]
    pub fn sarie(&mut self) -> _SARIEW {
        _SARIEW { w: self }
    }
}
