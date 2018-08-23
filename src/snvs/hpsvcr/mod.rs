#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPSVCR {
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
#[doc = "Possible values of the field `SV0_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV0_CFGR {
    #[doc = "Security Violation 0 is a non-fatal violation"]
    SV0_CFG_0,
    #[doc = "Security Violation 0 is a fatal violation"]
    SV0_CFG_1,
}
impl SV0_CFGR {
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
            SV0_CFGR::SV0_CFG_0 => false,
            SV0_CFGR::SV0_CFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV0_CFGR {
        match value {
            false => SV0_CFGR::SV0_CFG_0,
            true => SV0_CFGR::SV0_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV0_CFG_0`"]
    #[inline]
    pub fn is_sv0_cfg_0(&self) -> bool {
        *self == SV0_CFGR::SV0_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV0_CFG_1`"]
    #[inline]
    pub fn is_sv0_cfg_1(&self) -> bool {
        *self == SV0_CFGR::SV0_CFG_1
    }
}
#[doc = "Possible values of the field `SV1_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV1_CFGR {
    #[doc = "Security Violation 1 is a non-fatal violation"]
    SV1_CFG_0,
    #[doc = "Security Violation 1 is a fatal violation"]
    SV1_CFG_1,
}
impl SV1_CFGR {
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
            SV1_CFGR::SV1_CFG_0 => false,
            SV1_CFGR::SV1_CFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV1_CFGR {
        match value {
            false => SV1_CFGR::SV1_CFG_0,
            true => SV1_CFGR::SV1_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV1_CFG_0`"]
    #[inline]
    pub fn is_sv1_cfg_0(&self) -> bool {
        *self == SV1_CFGR::SV1_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV1_CFG_1`"]
    #[inline]
    pub fn is_sv1_cfg_1(&self) -> bool {
        *self == SV1_CFGR::SV1_CFG_1
    }
}
#[doc = "Possible values of the field `SV2_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV2_CFGR {
    #[doc = "Security Violation 2 is a non-fatal violation"]
    SV2_CFG_0,
    #[doc = "Security Violation 2 is a fatal violation"]
    SV2_CFG_1,
}
impl SV2_CFGR {
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
            SV2_CFGR::SV2_CFG_0 => false,
            SV2_CFGR::SV2_CFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV2_CFGR {
        match value {
            false => SV2_CFGR::SV2_CFG_0,
            true => SV2_CFGR::SV2_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV2_CFG_0`"]
    #[inline]
    pub fn is_sv2_cfg_0(&self) -> bool {
        *self == SV2_CFGR::SV2_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV2_CFG_1`"]
    #[inline]
    pub fn is_sv2_cfg_1(&self) -> bool {
        *self == SV2_CFGR::SV2_CFG_1
    }
}
#[doc = "Possible values of the field `SV3_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV3_CFGR {
    #[doc = "Security Violation 3 is a non-fatal violation"]
    SV3_CFG_0,
    #[doc = "Security Violation 3 is a fatal violation"]
    SV3_CFG_1,
}
impl SV3_CFGR {
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
            SV3_CFGR::SV3_CFG_0 => false,
            SV3_CFGR::SV3_CFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV3_CFGR {
        match value {
            false => SV3_CFGR::SV3_CFG_0,
            true => SV3_CFGR::SV3_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV3_CFG_0`"]
    #[inline]
    pub fn is_sv3_cfg_0(&self) -> bool {
        *self == SV3_CFGR::SV3_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV3_CFG_1`"]
    #[inline]
    pub fn is_sv3_cfg_1(&self) -> bool {
        *self == SV3_CFGR::SV3_CFG_1
    }
}
#[doc = "Possible values of the field `SV4_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV4_CFGR {
    #[doc = "Security Violation 4 is a non-fatal violation"]
    SV4_CFG_0,
    #[doc = "Security Violation 4 is a fatal violation"]
    SV4_CFG_1,
}
impl SV4_CFGR {
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
            SV4_CFGR::SV4_CFG_0 => false,
            SV4_CFGR::SV4_CFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SV4_CFGR {
        match value {
            false => SV4_CFGR::SV4_CFG_0,
            true => SV4_CFGR::SV4_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV4_CFG_0`"]
    #[inline]
    pub fn is_sv4_cfg_0(&self) -> bool {
        *self == SV4_CFGR::SV4_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV4_CFG_1`"]
    #[inline]
    pub fn is_sv4_cfg_1(&self) -> bool {
        *self == SV4_CFGR::SV4_CFG_1
    }
}
#[doc = "Possible values of the field `SV5_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV5_CFGR {
    #[doc = "Security Violation 5 is disabled"]
    SV5_CFG_0,
    #[doc = "Security Violation 5 is a non-fatal violation"]
    SV5_CFG_1,
    #[doc = "Security Violation 5 is a fatal violation"]
    SV5_CFG_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SV5_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SV5_CFGR::SV5_CFG_0 => 0,
            SV5_CFGR::SV5_CFG_1 => 1,
            SV5_CFGR::SV5_CFG_2 => 2,
            SV5_CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SV5_CFGR {
        match value {
            0 => SV5_CFGR::SV5_CFG_0,
            1 => SV5_CFGR::SV5_CFG_1,
            2 => SV5_CFGR::SV5_CFG_2,
            i => SV5_CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SV5_CFG_0`"]
    #[inline]
    pub fn is_sv5_cfg_0(&self) -> bool {
        *self == SV5_CFGR::SV5_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV5_CFG_1`"]
    #[inline]
    pub fn is_sv5_cfg_1(&self) -> bool {
        *self == SV5_CFGR::SV5_CFG_1
    }
    #[doc = "Checks if the value of the field is `SV5_CFG_2`"]
    #[inline]
    pub fn is_sv5_cfg_2(&self) -> bool {
        *self == SV5_CFGR::SV5_CFG_2
    }
}
#[doc = "Possible values of the field `LPSV_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSV_CFGR {
    #[doc = "LP security violation is disabled"]
    LPSV_CFG_0,
    #[doc = "LP security violation is a non-fatal violation"]
    LPSV_CFG_1,
    #[doc = "LP security violation is a fatal violation"]
    LPSV_CFG_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPSV_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPSV_CFGR::LPSV_CFG_0 => 0,
            LPSV_CFGR::LPSV_CFG_1 => 1,
            LPSV_CFGR::LPSV_CFG_2 => 2,
            LPSV_CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPSV_CFGR {
        match value {
            0 => LPSV_CFGR::LPSV_CFG_0,
            1 => LPSV_CFGR::LPSV_CFG_1,
            2 => LPSV_CFGR::LPSV_CFG_2,
            i => LPSV_CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPSV_CFG_0`"]
    #[inline]
    pub fn is_lpsv_cfg_0(&self) -> bool {
        *self == LPSV_CFGR::LPSV_CFG_0
    }
    #[doc = "Checks if the value of the field is `LPSV_CFG_1`"]
    #[inline]
    pub fn is_lpsv_cfg_1(&self) -> bool {
        *self == LPSV_CFGR::LPSV_CFG_1
    }
    #[doc = "Checks if the value of the field is `LPSV_CFG_2`"]
    #[inline]
    pub fn is_lpsv_cfg_2(&self) -> bool {
        *self == LPSV_CFGR::LPSV_CFG_2
    }
}
#[doc = "Values that can be written to the field `SV0_CFG`"]
pub enum SV0_CFGW {
    #[doc = "Security Violation 0 is a non-fatal violation"]
    SV0_CFG_0,
    #[doc = "Security Violation 0 is a fatal violation"]
    SV0_CFG_1,
}
impl SV0_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV0_CFGW::SV0_CFG_0 => false,
            SV0_CFGW::SV0_CFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV0_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SV0_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV0_CFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 0 is a non-fatal violation"]
    #[inline]
    pub fn sv0_cfg_0(self) -> &'a mut W {
        self.variant(SV0_CFGW::SV0_CFG_0)
    }
    #[doc = "Security Violation 0 is a fatal violation"]
    #[inline]
    pub fn sv0_cfg_1(self) -> &'a mut W {
        self.variant(SV0_CFGW::SV0_CFG_1)
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
#[doc = "Values that can be written to the field `SV1_CFG`"]
pub enum SV1_CFGW {
    #[doc = "Security Violation 1 is a non-fatal violation"]
    SV1_CFG_0,
    #[doc = "Security Violation 1 is a fatal violation"]
    SV1_CFG_1,
}
impl SV1_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV1_CFGW::SV1_CFG_0 => false,
            SV1_CFGW::SV1_CFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV1_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SV1_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV1_CFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 1 is a non-fatal violation"]
    #[inline]
    pub fn sv1_cfg_0(self) -> &'a mut W {
        self.variant(SV1_CFGW::SV1_CFG_0)
    }
    #[doc = "Security Violation 1 is a fatal violation"]
    #[inline]
    pub fn sv1_cfg_1(self) -> &'a mut W {
        self.variant(SV1_CFGW::SV1_CFG_1)
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
#[doc = "Values that can be written to the field `SV2_CFG`"]
pub enum SV2_CFGW {
    #[doc = "Security Violation 2 is a non-fatal violation"]
    SV2_CFG_0,
    #[doc = "Security Violation 2 is a fatal violation"]
    SV2_CFG_1,
}
impl SV2_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV2_CFGW::SV2_CFG_0 => false,
            SV2_CFGW::SV2_CFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV2_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SV2_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV2_CFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 2 is a non-fatal violation"]
    #[inline]
    pub fn sv2_cfg_0(self) -> &'a mut W {
        self.variant(SV2_CFGW::SV2_CFG_0)
    }
    #[doc = "Security Violation 2 is a fatal violation"]
    #[inline]
    pub fn sv2_cfg_1(self) -> &'a mut W {
        self.variant(SV2_CFGW::SV2_CFG_1)
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
#[doc = "Values that can be written to the field `SV3_CFG`"]
pub enum SV3_CFGW {
    #[doc = "Security Violation 3 is a non-fatal violation"]
    SV3_CFG_0,
    #[doc = "Security Violation 3 is a fatal violation"]
    SV3_CFG_1,
}
impl SV3_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV3_CFGW::SV3_CFG_0 => false,
            SV3_CFGW::SV3_CFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV3_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SV3_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV3_CFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 3 is a non-fatal violation"]
    #[inline]
    pub fn sv3_cfg_0(self) -> &'a mut W {
        self.variant(SV3_CFGW::SV3_CFG_0)
    }
    #[doc = "Security Violation 3 is a fatal violation"]
    #[inline]
    pub fn sv3_cfg_1(self) -> &'a mut W {
        self.variant(SV3_CFGW::SV3_CFG_1)
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
#[doc = "Values that can be written to the field `SV4_CFG`"]
pub enum SV4_CFGW {
    #[doc = "Security Violation 4 is a non-fatal violation"]
    SV4_CFG_0,
    #[doc = "Security Violation 4 is a fatal violation"]
    SV4_CFG_1,
}
impl SV4_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SV4_CFGW::SV4_CFG_0 => false,
            SV4_CFGW::SV4_CFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV4_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SV4_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV4_CFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Security Violation 4 is a non-fatal violation"]
    #[inline]
    pub fn sv4_cfg_0(self) -> &'a mut W {
        self.variant(SV4_CFGW::SV4_CFG_0)
    }
    #[doc = "Security Violation 4 is a fatal violation"]
    #[inline]
    pub fn sv4_cfg_1(self) -> &'a mut W {
        self.variant(SV4_CFGW::SV4_CFG_1)
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
#[doc = "Values that can be written to the field `SV5_CFG`"]
pub enum SV5_CFGW {
    #[doc = "Security Violation 5 is disabled"]
    SV5_CFG_0,
    #[doc = "Security Violation 5 is a non-fatal violation"]
    SV5_CFG_1,
    #[doc = "Security Violation 5 is a fatal violation"]
    SV5_CFG_2,
}
impl SV5_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SV5_CFGW::SV5_CFG_0 => 0,
            SV5_CFGW::SV5_CFG_1 => 1,
            SV5_CFGW::SV5_CFG_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SV5_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SV5_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SV5_CFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Security Violation 5 is disabled"]
    #[inline]
    pub fn sv5_cfg_0(self) -> &'a mut W {
        self.variant(SV5_CFGW::SV5_CFG_0)
    }
    #[doc = "Security Violation 5 is a non-fatal violation"]
    #[inline]
    pub fn sv5_cfg_1(self) -> &'a mut W {
        self.variant(SV5_CFGW::SV5_CFG_1)
    }
    #[doc = "Security Violation 5 is a fatal violation"]
    #[inline]
    pub fn sv5_cfg_2(self) -> &'a mut W {
        self.variant(SV5_CFGW::SV5_CFG_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSV_CFG`"]
pub enum LPSV_CFGW {
    #[doc = "LP security violation is disabled"]
    LPSV_CFG_0,
    #[doc = "LP security violation is a non-fatal violation"]
    LPSV_CFG_1,
    #[doc = "LP security violation is a fatal violation"]
    LPSV_CFG_2,
}
impl LPSV_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPSV_CFGW::LPSV_CFG_0 => 0,
            LPSV_CFGW::LPSV_CFG_1 => 1,
            LPSV_CFGW::LPSV_CFG_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSV_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSV_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSV_CFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LP security violation is disabled"]
    #[inline]
    pub fn lpsv_cfg_0(self) -> &'a mut W {
        self.variant(LPSV_CFGW::LPSV_CFG_0)
    }
    #[doc = "LP security violation is a non-fatal violation"]
    #[inline]
    pub fn lpsv_cfg_1(self) -> &'a mut W {
        self.variant(LPSV_CFGW::LPSV_CFG_1)
    }
    #[doc = "LP security violation is a fatal violation"]
    #[inline]
    pub fn lpsv_cfg_2(self) -> &'a mut W {
        self.variant(LPSV_CFGW::LPSV_CFG_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline]
    pub fn sv0_cfg(&self) -> SV0_CFGR {
        SV0_CFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline]
    pub fn sv1_cfg(&self) -> SV1_CFGR {
        SV1_CFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline]
    pub fn sv2_cfg(&self) -> SV2_CFGR {
        SV2_CFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline]
    pub fn sv3_cfg(&self) -> SV3_CFGR {
        SV3_CFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline]
    pub fn sv4_cfg(&self) -> SV4_CFGR {
        SV4_CFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline]
    pub fn sv5_cfg(&self) -> SV5_CFGR {
        SV5_CFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline]
    pub fn lpsv_cfg(&self) -> LPSV_CFGR {
        LPSV_CFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline]
    pub fn sv0_cfg(&mut self) -> _SV0_CFGW {
        _SV0_CFGW { w: self }
    }
    #[doc = "Bit 1 - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline]
    pub fn sv1_cfg(&mut self) -> _SV1_CFGW {
        _SV1_CFGW { w: self }
    }
    #[doc = "Bit 2 - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline]
    pub fn sv2_cfg(&mut self) -> _SV2_CFGW {
        _SV2_CFGW { w: self }
    }
    #[doc = "Bit 3 - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline]
    pub fn sv3_cfg(&mut self) -> _SV3_CFGW {
        _SV3_CFGW { w: self }
    }
    #[doc = "Bit 4 - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline]
    pub fn sv4_cfg(&mut self) -> _SV4_CFGW {
        _SV4_CFGW { w: self }
    }
    #[doc = "Bits 5:6 - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline]
    pub fn sv5_cfg(&mut self) -> _SV5_CFGW {
        _SV5_CFGW { w: self }
    }
    #[doc = "Bits 30:31 - LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline]
    pub fn lpsv_cfg(&mut self) -> _LPSV_CFGW {
        _LPSV_CFGW { w: self }
    }
}
