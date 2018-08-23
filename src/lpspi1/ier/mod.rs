#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Possible values of the field `WCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCIER {
    #[doc = "Disabled"]
    WCIE_0,
    #[doc = "Enabled"]
    WCIE_1,
}
impl WCIER {
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
            WCIER::WCIE_0 => false,
            WCIER::WCIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCIER {
        match value {
            false => WCIER::WCIE_0,
            true => WCIER::WCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WCIE_0`"]
    #[inline]
    pub fn is_wcie_0(&self) -> bool {
        *self == WCIER::WCIE_0
    }
    #[doc = "Checks if the value of the field is `WCIE_1`"]
    #[inline]
    pub fn is_wcie_1(&self) -> bool {
        *self == WCIER::WCIE_1
    }
}
#[doc = "Possible values of the field `FCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCIER {
    #[doc = "Disabled"]
    FCIE_0,
    #[doc = "Enabled"]
    FCIE_1,
}
impl FCIER {
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
            FCIER::FCIE_0 => false,
            FCIER::FCIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCIER {
        match value {
            false => FCIER::FCIE_0,
            true => FCIER::FCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCIE_0`"]
    #[inline]
    pub fn is_fcie_0(&self) -> bool {
        *self == FCIER::FCIE_0
    }
    #[doc = "Checks if the value of the field is `FCIE_1`"]
    #[inline]
    pub fn is_fcie_1(&self) -> bool {
        *self == FCIER::FCIE_1
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Disabled"]
    TCIE_0,
    #[doc = "Enabled"]
    TCIE_1,
}
impl TCIER {
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
            TCIER::TCIE_0 => false,
            TCIER::TCIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::TCIE_0,
            true => TCIER::TCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCIE_0`"]
    #[inline]
    pub fn is_tcie_0(&self) -> bool {
        *self == TCIER::TCIE_0
    }
    #[doc = "Checks if the value of the field is `TCIE_1`"]
    #[inline]
    pub fn is_tcie_1(&self) -> bool {
        *self == TCIER::TCIE_1
    }
}
#[doc = "Possible values of the field `TEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIER {
    #[doc = "Disabled"]
    TEIE_0,
    #[doc = "Enabled"]
    TEIE_1,
}
impl TEIER {
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
            TEIER::TEIE_0 => false,
            TEIER::TEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIER {
        match value {
            false => TEIER::TEIE_0,
            true => TEIER::TEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEIE_0`"]
    #[inline]
    pub fn is_teie_0(&self) -> bool {
        *self == TEIER::TEIE_0
    }
    #[doc = "Checks if the value of the field is `TEIE_1`"]
    #[inline]
    pub fn is_teie_1(&self) -> bool {
        *self == TEIER::TEIE_1
    }
}
#[doc = "Possible values of the field `REIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIER {
    #[doc = "Disabled"]
    REIE_0,
    #[doc = "Enabled"]
    REIE_1,
}
impl REIER {
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
            REIER::REIE_0 => false,
            REIER::REIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REIER {
        match value {
            false => REIER::REIE_0,
            true => REIER::REIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `REIE_0`"]
    #[inline]
    pub fn is_reie_0(&self) -> bool {
        *self == REIER::REIE_0
    }
    #[doc = "Checks if the value of the field is `REIE_1`"]
    #[inline]
    pub fn is_reie_1(&self) -> bool {
        *self == REIER::REIE_1
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
#[doc = "Values that can be written to the field `WCIE`"]
pub enum WCIEW {
    #[doc = "Disabled"]
    WCIE_0,
    #[doc = "Enabled"]
    WCIE_1,
}
impl WCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WCIEW::WCIE_0 => false,
            WCIEW::WCIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn wcie_0(self) -> &'a mut W {
        self.variant(WCIEW::WCIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn wcie_1(self) -> &'a mut W {
        self.variant(WCIEW::WCIE_1)
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
#[doc = "Values that can be written to the field `FCIE`"]
pub enum FCIEW {
    #[doc = "Disabled"]
    FCIE_0,
    #[doc = "Enabled"]
    FCIE_1,
}
impl FCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCIEW::FCIE_0 => false,
            FCIEW::FCIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn fcie_0(self) -> &'a mut W {
        self.variant(FCIEW::FCIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn fcie_1(self) -> &'a mut W {
        self.variant(FCIEW::FCIE_1)
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
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Disabled"]
    TCIE_0,
    #[doc = "Enabled"]
    TCIE_1,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::TCIE_0 => false,
            TCIEW::TCIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn tcie_0(self) -> &'a mut W {
        self.variant(TCIEW::TCIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn tcie_1(self) -> &'a mut W {
        self.variant(TCIEW::TCIE_1)
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
#[doc = "Values that can be written to the field `TEIE`"]
pub enum TEIEW {
    #[doc = "Disabled"]
    TEIE_0,
    #[doc = "Enabled"]
    TEIE_1,
}
impl TEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEIEW::TEIE_0 => false,
            TEIEW::TEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn teie_0(self) -> &'a mut W {
        self.variant(TEIEW::TEIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn teie_1(self) -> &'a mut W {
        self.variant(TEIEW::TEIE_1)
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
#[doc = "Values that can be written to the field `REIE`"]
pub enum REIEW {
    #[doc = "Disabled"]
    REIE_0,
    #[doc = "Enabled"]
    REIE_1,
}
impl REIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REIEW::REIE_0 => false,
            REIEW::REIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REIEW<'a> {
    w: &'a mut W,
}
impl<'a> _REIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn reie_0(self) -> &'a mut W {
        self.variant(REIEW::REIE_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn reie_1(self) -> &'a mut W {
        self.variant(REIEW::REIE_1)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline]
    pub fn wcie(&self) -> WCIER {
        WCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline]
    pub fn fcie(&self) -> FCIER {
        FCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline]
    pub fn teie(&self) -> TEIER {
        TEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline]
    pub fn reie(&self) -> REIER {
        REIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline]
    pub fn dmie(&self) -> DMIER {
        DMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline]
    pub fn wcie(&mut self) -> _WCIEW {
        _WCIEW { w: self }
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline]
    pub fn fcie(&mut self) -> _FCIEW {
        _FCIEW { w: self }
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline]
    pub fn teie(&mut self) -> _TEIEW {
        _TEIEW { w: self }
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline]
    pub fn reie(&mut self) -> _REIEW {
        _REIEW { w: self }
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline]
    pub fn dmie(&mut self) -> _DMIEW {
        _DMIEW { w: self }
    }
}
