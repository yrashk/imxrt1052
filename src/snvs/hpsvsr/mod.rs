#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPSVSR {
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
#[doc = "Possible values of the field `SV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV0R {
    #[doc = "No Security Violation 0 security violation was detected."]
    SV0_0,
    #[doc = "Security Violation 0 security violation was detected."]
    SV0_1,
}
impl SV0R {
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
            SV0R::SV0_0 => false,
            SV0R::SV0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV0R {
        match value {
            false => SV0R::SV0_0,
            true => SV0R::SV0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV0_0`"]
    #[inline]
    pub fn is_sv0_0(&self) -> bool {
        *self == SV0R::SV0_0
    }
    #[doc = "Checks if the value of the field is `SV0_1`"]
    #[inline]
    pub fn is_sv0_1(&self) -> bool {
        *self == SV0R::SV0_1
    }
}
#[doc = "Possible values of the field `SV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV1R {
    #[doc = "No Security Violation 1 security violation was detected."]
    SV1_0,
    #[doc = "Security Violation 1 security violation was detected."]
    SV1_1,
}
impl SV1R {
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
            SV1R::SV1_0 => false,
            SV1R::SV1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV1R {
        match value {
            false => SV1R::SV1_0,
            true => SV1R::SV1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV1_0`"]
    #[inline]
    pub fn is_sv1_0(&self) -> bool {
        *self == SV1R::SV1_0
    }
    #[doc = "Checks if the value of the field is `SV1_1`"]
    #[inline]
    pub fn is_sv1_1(&self) -> bool {
        *self == SV1R::SV1_1
    }
}
#[doc = "Possible values of the field `SV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV2R {
    #[doc = "No Security Violation 2 security violation was detected."]
    SV2_0,
    #[doc = "Security Violation 2 security violation was detected."]
    SV2_1,
}
impl SV2R {
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
            SV2R::SV2_0 => false,
            SV2R::SV2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV2R {
        match value {
            false => SV2R::SV2_0,
            true => SV2R::SV2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV2_0`"]
    #[inline]
    pub fn is_sv2_0(&self) -> bool {
        *self == SV2R::SV2_0
    }
    #[doc = "Checks if the value of the field is `SV2_1`"]
    #[inline]
    pub fn is_sv2_1(&self) -> bool {
        *self == SV2R::SV2_1
    }
}
#[doc = "Possible values of the field `SV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV3R {
    #[doc = "No Security Violation 3 security violation was detected."]
    SV3_0,
    #[doc = "Security Violation 3 security violation was detected."]
    SV3_1,
}
impl SV3R {
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
            SV3R::SV3_0 => false,
            SV3R::SV3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV3R {
        match value {
            false => SV3R::SV3_0,
            true => SV3R::SV3_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV3_0`"]
    #[inline]
    pub fn is_sv3_0(&self) -> bool {
        *self == SV3R::SV3_0
    }
    #[doc = "Checks if the value of the field is `SV3_1`"]
    #[inline]
    pub fn is_sv3_1(&self) -> bool {
        *self == SV3R::SV3_1
    }
}
#[doc = "Possible values of the field `SV4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV4R {
    #[doc = "No Security Violation 4 security violation was detected."]
    SV4_0,
    #[doc = "Security Violation 4 security violation was detected."]
    SV4_1,
}
impl SV4R {
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
            SV4R::SV4_0 => false,
            SV4R::SV4_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV4R {
        match value {
            false => SV4R::SV4_0,
            true => SV4R::SV4_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV4_0`"]
    #[inline]
    pub fn is_sv4_0(&self) -> bool {
        *self == SV4R::SV4_0
    }
    #[doc = "Checks if the value of the field is `SV4_1`"]
    #[inline]
    pub fn is_sv4_1(&self) -> bool {
        *self == SV4R::SV4_1
    }
}
#[doc = "Possible values of the field `SV5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV5R {
    #[doc = "No Security Violation 5 security violation was detected."]
    SV5_0,
    #[doc = "Security Violation 5 security violation was detected."]
    SV5_1,
}
impl SV5R {
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
            SV5R::SV5_0 => false,
            SV5R::SV5_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV5R {
        match value {
            false => SV5R::SV5_0,
            true => SV5R::SV5_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV5_0`"]
    #[inline]
    pub fn is_sv5_0(&self) -> bool {
        *self == SV5R::SV5_0
    }
    #[doc = "Checks if the value of the field is `SV5_1`"]
    #[inline]
    pub fn is_sv5_1(&self) -> bool {
        *self == SV5R::SV5_1
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
#[doc = r" Value of the field"]
pub struct ZMK_SYNDROMER {
    bits: u16,
}
impl ZMK_SYNDROMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ZMK_ECC_FAIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_ECC_FAILR {
    #[doc = "ZMK ECC Failure was not detected."]
    ZMK_ECC_FAIL_0,
    #[doc = "ZMK ECC Failure was detected."]
    ZMK_ECC_FAIL_1,
}
impl ZMK_ECC_FAILR {
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
            ZMK_ECC_FAILR::ZMK_ECC_FAIL_0 => false,
            ZMK_ECC_FAILR::ZMK_ECC_FAIL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_ECC_FAILR {
        match value {
            false => ZMK_ECC_FAILR::ZMK_ECC_FAIL_0,
            true => ZMK_ECC_FAILR::ZMK_ECC_FAIL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_FAIL_0`"]
    #[inline]
    pub fn is_zmk_ecc_fail_0(&self) -> bool {
        *self == ZMK_ECC_FAILR::ZMK_ECC_FAIL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_FAIL_1`"]
    #[inline]
    pub fn is_zmk_ecc_fail_1(&self) -> bool {
        *self == ZMK_ECC_FAILR::ZMK_ECC_FAIL_1
    }
}
#[doc = r" Value of the field"]
pub struct LP_SEC_VIOR {
    bits: bool,
}
impl LP_SEC_VIOR {
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
#[doc = "Values that can be written to the field `SV0`"]
pub enum SV0W {
    #[doc = "No Security Violation 0 security violation was detected."]
    SV0_0,
    #[doc = "Security Violation 0 security violation was detected."]
    SV0_1,
}
impl SV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV0W::SV0_0 => false,
            SV0W::SV0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV0W<'a> {
    w: &'a mut W,
}
impl<'a> _SV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Security Violation 0 security violation was detected."]
    #[inline]
    pub fn sv0_0(self) -> &'a mut W {
        self.variant(SV0W::SV0_0)
    }
    #[doc = "Security Violation 0 security violation was detected."]
    #[inline]
    pub fn sv0_1(self) -> &'a mut W {
        self.variant(SV0W::SV0_1)
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
#[doc = "Values that can be written to the field `SV1`"]
pub enum SV1W {
    #[doc = "No Security Violation 1 security violation was detected."]
    SV1_0,
    #[doc = "Security Violation 1 security violation was detected."]
    SV1_1,
}
impl SV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV1W::SV1_0 => false,
            SV1W::SV1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV1W<'a> {
    w: &'a mut W,
}
impl<'a> _SV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Security Violation 1 security violation was detected."]
    #[inline]
    pub fn sv1_0(self) -> &'a mut W {
        self.variant(SV1W::SV1_0)
    }
    #[doc = "Security Violation 1 security violation was detected."]
    #[inline]
    pub fn sv1_1(self) -> &'a mut W {
        self.variant(SV1W::SV1_1)
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
#[doc = "Values that can be written to the field `SV2`"]
pub enum SV2W {
    #[doc = "No Security Violation 2 security violation was detected."]
    SV2_0,
    #[doc = "Security Violation 2 security violation was detected."]
    SV2_1,
}
impl SV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV2W::SV2_0 => false,
            SV2W::SV2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV2W<'a> {
    w: &'a mut W,
}
impl<'a> _SV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Security Violation 2 security violation was detected."]
    #[inline]
    pub fn sv2_0(self) -> &'a mut W {
        self.variant(SV2W::SV2_0)
    }
    #[doc = "Security Violation 2 security violation was detected."]
    #[inline]
    pub fn sv2_1(self) -> &'a mut W {
        self.variant(SV2W::SV2_1)
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
#[doc = "Values that can be written to the field `SV3`"]
pub enum SV3W {
    #[doc = "No Security Violation 3 security violation was detected."]
    SV3_0,
    #[doc = "Security Violation 3 security violation was detected."]
    SV3_1,
}
impl SV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV3W::SV3_0 => false,
            SV3W::SV3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV3W<'a> {
    w: &'a mut W,
}
impl<'a> _SV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Security Violation 3 security violation was detected."]
    #[inline]
    pub fn sv3_0(self) -> &'a mut W {
        self.variant(SV3W::SV3_0)
    }
    #[doc = "Security Violation 3 security violation was detected."]
    #[inline]
    pub fn sv3_1(self) -> &'a mut W {
        self.variant(SV3W::SV3_1)
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
#[doc = "Values that can be written to the field `SV4`"]
pub enum SV4W {
    #[doc = "No Security Violation 4 security violation was detected."]
    SV4_0,
    #[doc = "Security Violation 4 security violation was detected."]
    SV4_1,
}
impl SV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV4W::SV4_0 => false,
            SV4W::SV4_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV4W<'a> {
    w: &'a mut W,
}
impl<'a> _SV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Security Violation 4 security violation was detected."]
    #[inline]
    pub fn sv4_0(self) -> &'a mut W {
        self.variant(SV4W::SV4_0)
    }
    #[doc = "Security Violation 4 security violation was detected."]
    #[inline]
    pub fn sv4_1(self) -> &'a mut W {
        self.variant(SV4W::SV4_1)
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
#[doc = "Values that can be written to the field `SV5`"]
pub enum SV5W {
    #[doc = "No Security Violation 5 security violation was detected."]
    SV5_0,
    #[doc = "Security Violation 5 security violation was detected."]
    SV5_1,
}
impl SV5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV5W::SV5_0 => false,
            SV5W::SV5_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV5W<'a> {
    w: &'a mut W,
}
impl<'a> _SV5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Security Violation 5 security violation was detected."]
    #[inline]
    pub fn sv5_0(self) -> &'a mut W {
        self.variant(SV5W::SV5_0)
    }
    #[doc = "Security Violation 5 security violation was detected."]
    #[inline]
    pub fn sv5_1(self) -> &'a mut W {
        self.variant(SV5W::SV5_1)
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
#[doc = "Values that can be written to the field `ZMK_ECC_FAIL`"]
pub enum ZMK_ECC_FAILW {
    #[doc = "ZMK ECC Failure was not detected."]
    ZMK_ECC_FAIL_0,
    #[doc = "ZMK ECC Failure was detected."]
    ZMK_ECC_FAIL_1,
}
impl ZMK_ECC_FAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_ECC_FAILW::ZMK_ECC_FAIL_0 => false,
            ZMK_ECC_FAILW::ZMK_ECC_FAIL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_ECC_FAILW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_ECC_FAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_ECC_FAILW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ZMK ECC Failure was not detected."]
    #[inline]
    pub fn zmk_ecc_fail_0(self) -> &'a mut W {
        self.variant(ZMK_ECC_FAILW::ZMK_ECC_FAIL_0)
    }
    #[doc = "ZMK ECC Failure was detected."]
    #[inline]
    pub fn zmk_ecc_fail_1(self) -> &'a mut W {
        self.variant(ZMK_ECC_FAILW::ZMK_ECC_FAIL_1)
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Security Violation 0 security violation was detected."]
    #[inline]
    pub fn sv0(&self) -> SV0R {
        SV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Security Violation 1 security violation was detected."]
    #[inline]
    pub fn sv1(&self) -> SV1R {
        SV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Security Violation 2 security violation was detected."]
    #[inline]
    pub fn sv2(&self) -> SV2R {
        SV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Security Violation 3 security violation was detected."]
    #[inline]
    pub fn sv3(&self) -> SV3R {
        SV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Security Violation 4 security violation was detected."]
    #[inline]
    pub fn sv4(&self) -> SV4R {
        SV4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Security Violation 5 security violation was detected."]
    #[inline]
    pub fn sv5(&self) -> SV5R {
        SV5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    #[inline]
    pub fn sw_sv(&self) -> SW_SVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_SVR { bits }
    }
    #[doc = "Bit 14 - Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    #[inline]
    pub fn sw_fsv(&self) -> SW_FSVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_FSVR { bits }
    }
    #[doc = "Bit 15 - LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    #[inline]
    pub fn sw_lpsv(&self) -> SW_LPSVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_LPSVR { bits }
    }
    #[doc = "Bits 16:24 - Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    #[inline]
    pub fn zmk_syndrome(&self) -> ZMK_SYNDROMER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ZMK_SYNDROMER { bits }
    }
    #[doc = "Bit 27 - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline]
    pub fn zmk_ecc_fail(&self) -> ZMK_ECC_FAILR {
        ZMK_ECC_FAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - LP Security Violation A security volation was detected in the SNVS low power section."]
    #[inline]
    pub fn lp_sec_vio(&self) -> LP_SEC_VIOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LP_SEC_VIOR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Security Violation 0 security violation was detected."]
    #[inline]
    pub fn sv0(&mut self) -> _SV0W {
        _SV0W { w: self }
    }
    #[doc = "Bit 1 - Security Violation 1 security violation was detected."]
    #[inline]
    pub fn sv1(&mut self) -> _SV1W {
        _SV1W { w: self }
    }
    #[doc = "Bit 2 - Security Violation 2 security violation was detected."]
    #[inline]
    pub fn sv2(&mut self) -> _SV2W {
        _SV2W { w: self }
    }
    #[doc = "Bit 3 - Security Violation 3 security violation was detected."]
    #[inline]
    pub fn sv3(&mut self) -> _SV3W {
        _SV3W { w: self }
    }
    #[doc = "Bit 4 - Security Violation 4 security violation was detected."]
    #[inline]
    pub fn sv4(&mut self) -> _SV4W {
        _SV4W { w: self }
    }
    #[doc = "Bit 5 - Security Violation 5 security violation was detected."]
    #[inline]
    pub fn sv5(&mut self) -> _SV5W {
        _SV5W { w: self }
    }
    #[doc = "Bit 27 - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline]
    pub fn zmk_ecc_fail(&mut self) -> _ZMK_ECC_FAILW {
        _ZMK_ECC_FAILW { w: self }
    }
}
