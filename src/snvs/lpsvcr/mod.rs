#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPSVCR {
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
#[doc = "Possible values of the field `SV0_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV0_ENR {
    #[doc = "Security Violation 0 is disabled in the LP domain."]
    SV0_EN_0,
    #[doc = "Security Violation 0 is enabled in the LP domain."]
    SV0_EN_1,
}
impl SV0_ENR {
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
            SV0_ENR::SV0_EN_0 => false,
            SV0_ENR::SV0_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV0_ENR {
        match value {
            false => SV0_ENR::SV0_EN_0,
            true => SV0_ENR::SV0_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV0_EN_0`"]
    #[inline]
    pub fn is_sv0_en_0(&self) -> bool {
        *self == SV0_ENR::SV0_EN_0
    }
    #[doc = "Checks if the value of the field is `SV0_EN_1`"]
    #[inline]
    pub fn is_sv0_en_1(&self) -> bool {
        *self == SV0_ENR::SV0_EN_1
    }
}
#[doc = "Possible values of the field `SV1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV1_ENR {
    #[doc = "Security Violation 1 is disabled in the LP domain."]
    SV1_EN_0,
    #[doc = "Security Violation 1 is enabled in the LP domain."]
    SV1_EN_1,
}
impl SV1_ENR {
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
            SV1_ENR::SV1_EN_0 => false,
            SV1_ENR::SV1_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV1_ENR {
        match value {
            false => SV1_ENR::SV1_EN_0,
            true => SV1_ENR::SV1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV1_EN_0`"]
    #[inline]
    pub fn is_sv1_en_0(&self) -> bool {
        *self == SV1_ENR::SV1_EN_0
    }
    #[doc = "Checks if the value of the field is `SV1_EN_1`"]
    #[inline]
    pub fn is_sv1_en_1(&self) -> bool {
        *self == SV1_ENR::SV1_EN_1
    }
}
#[doc = "Possible values of the field `SV2_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV2_ENR {
    #[doc = "Security Violation 2 is disabled in the LP domain."]
    SV2_EN_0,
    #[doc = "Security Violation 2 is enabled in the LP domain."]
    SV2_EN_1,
}
impl SV2_ENR {
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
            SV2_ENR::SV2_EN_0 => false,
            SV2_ENR::SV2_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV2_ENR {
        match value {
            false => SV2_ENR::SV2_EN_0,
            true => SV2_ENR::SV2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV2_EN_0`"]
    #[inline]
    pub fn is_sv2_en_0(&self) -> bool {
        *self == SV2_ENR::SV2_EN_0
    }
    #[doc = "Checks if the value of the field is `SV2_EN_1`"]
    #[inline]
    pub fn is_sv2_en_1(&self) -> bool {
        *self == SV2_ENR::SV2_EN_1
    }
}
#[doc = "Possible values of the field `SV3_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV3_ENR {
    #[doc = "Security Violation 3 is disabled in the LP domain."]
    SV3_EN_0,
    #[doc = "Security Violation 3 is enabled in the LP domain."]
    SV3_EN_1,
}
impl SV3_ENR {
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
            SV3_ENR::SV3_EN_0 => false,
            SV3_ENR::SV3_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV3_ENR {
        match value {
            false => SV3_ENR::SV3_EN_0,
            true => SV3_ENR::SV3_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV3_EN_0`"]
    #[inline]
    pub fn is_sv3_en_0(&self) -> bool {
        *self == SV3_ENR::SV3_EN_0
    }
    #[doc = "Checks if the value of the field is `SV3_EN_1`"]
    #[inline]
    pub fn is_sv3_en_1(&self) -> bool {
        *self == SV3_ENR::SV3_EN_1
    }
}
#[doc = "Possible values of the field `SV4_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV4_ENR {
    #[doc = "Security Violation 4 is disabled in the LP domain."]
    SV4_EN_0,
    #[doc = "Security Violation 4 is enabled in the LP domain."]
    SV4_EN_1,
}
impl SV4_ENR {
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
            SV4_ENR::SV4_EN_0 => false,
            SV4_ENR::SV4_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV4_ENR {
        match value {
            false => SV4_ENR::SV4_EN_0,
            true => SV4_ENR::SV4_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV4_EN_0`"]
    #[inline]
    pub fn is_sv4_en_0(&self) -> bool {
        *self == SV4_ENR::SV4_EN_0
    }
    #[doc = "Checks if the value of the field is `SV4_EN_1`"]
    #[inline]
    pub fn is_sv4_en_1(&self) -> bool {
        *self == SV4_ENR::SV4_EN_1
    }
}
#[doc = "Possible values of the field `SV5_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV5_ENR {
    #[doc = "Security Violation 5 is disabled in the LP domain."]
    SV5_EN_0,
    #[doc = "Security Violation 5 is enabled in the LP domain."]
    SV5_EN_1,
}
impl SV5_ENR {
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
            SV5_ENR::SV5_EN_0 => false,
            SV5_ENR::SV5_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV5_ENR {
        match value {
            false => SV5_ENR::SV5_EN_0,
            true => SV5_ENR::SV5_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV5_EN_0`"]
    #[inline]
    pub fn is_sv5_en_0(&self) -> bool {
        *self == SV5_ENR::SV5_EN_0
    }
    #[doc = "Checks if the value of the field is `SV5_EN_1`"]
    #[inline]
    pub fn is_sv5_en_1(&self) -> bool {
        *self == SV5_ENR::SV5_EN_1
    }
}
#[doc = "Values that can be written to the field `SV0_EN`"]
pub enum SV0_ENW {
    #[doc = "Security Violation 0 is disabled in the LP domain."]
    SV0_EN_0,
    #[doc = "Security Violation 0 is enabled in the LP domain."]
    SV0_EN_1,
}
impl SV0_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV0_ENW::SV0_EN_0 => false,
            SV0_ENW::SV0_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SV0_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV0_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 0 is disabled in the LP domain."]
    #[inline]
    pub fn sv0_en_0(self) -> &'a mut W {
        self.variant(SV0_ENW::SV0_EN_0)
    }
    #[doc = "Security Violation 0 is enabled in the LP domain."]
    #[inline]
    pub fn sv0_en_1(self) -> &'a mut W {
        self.variant(SV0_ENW::SV0_EN_1)
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
#[doc = "Values that can be written to the field `SV1_EN`"]
pub enum SV1_ENW {
    #[doc = "Security Violation 1 is disabled in the LP domain."]
    SV1_EN_0,
    #[doc = "Security Violation 1 is enabled in the LP domain."]
    SV1_EN_1,
}
impl SV1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV1_ENW::SV1_EN_0 => false,
            SV1_ENW::SV1_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SV1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 1 is disabled in the LP domain."]
    #[inline]
    pub fn sv1_en_0(self) -> &'a mut W {
        self.variant(SV1_ENW::SV1_EN_0)
    }
    #[doc = "Security Violation 1 is enabled in the LP domain."]
    #[inline]
    pub fn sv1_en_1(self) -> &'a mut W {
        self.variant(SV1_ENW::SV1_EN_1)
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
#[doc = "Values that can be written to the field `SV2_EN`"]
pub enum SV2_ENW {
    #[doc = "Security Violation 2 is disabled in the LP domain."]
    SV2_EN_0,
    #[doc = "Security Violation 2 is enabled in the LP domain."]
    SV2_EN_1,
}
impl SV2_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV2_ENW::SV2_EN_0 => false,
            SV2_ENW::SV2_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SV2_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV2_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 2 is disabled in the LP domain."]
    #[inline]
    pub fn sv2_en_0(self) -> &'a mut W {
        self.variant(SV2_ENW::SV2_EN_0)
    }
    #[doc = "Security Violation 2 is enabled in the LP domain."]
    #[inline]
    pub fn sv2_en_1(self) -> &'a mut W {
        self.variant(SV2_ENW::SV2_EN_1)
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
#[doc = "Values that can be written to the field `SV3_EN`"]
pub enum SV3_ENW {
    #[doc = "Security Violation 3 is disabled in the LP domain."]
    SV3_EN_0,
    #[doc = "Security Violation 3 is enabled in the LP domain."]
    SV3_EN_1,
}
impl SV3_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV3_ENW::SV3_EN_0 => false,
            SV3_ENW::SV3_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV3_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SV3_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV3_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 3 is disabled in the LP domain."]
    #[inline]
    pub fn sv3_en_0(self) -> &'a mut W {
        self.variant(SV3_ENW::SV3_EN_0)
    }
    #[doc = "Security Violation 3 is enabled in the LP domain."]
    #[inline]
    pub fn sv3_en_1(self) -> &'a mut W {
        self.variant(SV3_ENW::SV3_EN_1)
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
#[doc = "Values that can be written to the field `SV4_EN`"]
pub enum SV4_ENW {
    #[doc = "Security Violation 4 is disabled in the LP domain."]
    SV4_EN_0,
    #[doc = "Security Violation 4 is enabled in the LP domain."]
    SV4_EN_1,
}
impl SV4_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV4_ENW::SV4_EN_0 => false,
            SV4_ENW::SV4_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV4_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SV4_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV4_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 4 is disabled in the LP domain."]
    #[inline]
    pub fn sv4_en_0(self) -> &'a mut W {
        self.variant(SV4_ENW::SV4_EN_0)
    }
    #[doc = "Security Violation 4 is enabled in the LP domain."]
    #[inline]
    pub fn sv4_en_1(self) -> &'a mut W {
        self.variant(SV4_ENW::SV4_EN_1)
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
#[doc = "Values that can be written to the field `SV5_EN`"]
pub enum SV5_ENW {
    #[doc = "Security Violation 5 is disabled in the LP domain."]
    SV5_EN_0,
    #[doc = "Security Violation 5 is enabled in the LP domain."]
    SV5_EN_1,
}
impl SV5_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV5_ENW::SV5_EN_0 => false,
            SV5_ENW::SV5_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV5_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SV5_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV5_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 5 is disabled in the LP domain."]
    #[inline]
    pub fn sv5_en_0(self) -> &'a mut W {
        self.variant(SV5_ENW::SV5_EN_0)
    }
    #[doc = "Security Violation 5 is enabled in the LP domain."]
    #[inline]
    pub fn sv5_en_1(self) -> &'a mut W {
        self.variant(SV5_ENW::SV5_EN_1)
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
    #[doc = "Bit 0 - Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[inline]
    pub fn sv0_en(&self) -> SV0_ENR {
        SV0_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[inline]
    pub fn sv1_en(&self) -> SV1_ENR {
        SV1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[inline]
    pub fn sv2_en(&self) -> SV2_ENR {
        SV2_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[inline]
    pub fn sv3_en(&self) -> SV3_ENR {
        SV3_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[inline]
    pub fn sv4_en(&self) -> SV4_ENR {
        SV4_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[inline]
    pub fn sv5_en(&self) -> SV5_ENR {
        SV5_ENR::_from({
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
    #[doc = "Bit 0 - Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[inline]
    pub fn sv0_en(&mut self) -> _SV0_ENW {
        _SV0_ENW { w: self }
    }
    #[doc = "Bit 1 - Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[inline]
    pub fn sv1_en(&mut self) -> _SV1_ENW {
        _SV1_ENW { w: self }
    }
    #[doc = "Bit 2 - Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[inline]
    pub fn sv2_en(&mut self) -> _SV2_ENW {
        _SV2_ENW { w: self }
    }
    #[doc = "Bit 3 - Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[inline]
    pub fn sv3_en(&mut self) -> _SV3_ENW {
        _SV3_ENW { w: self }
    }
    #[doc = "Bit 4 - Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[inline]
    pub fn sv4_en(&mut self) -> _SV4_ENW {
        _SV4_ENW { w: self }
    }
    #[doc = "Bit 5 - Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[inline]
    pub fn sv5_en(&mut self) -> _SV5_ENW {
        _SV5_ENW { w: self }
    }
}
