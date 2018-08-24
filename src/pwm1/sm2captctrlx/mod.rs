#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM2CAPTCTRLX {
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
#[doc = "Possible values of the field `ARMX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARMXR {
    #[doc = "Input capture operation is disabled."]
    ARMX_0,
    #[doc = "Input capture operation as specified by CAPTCTRLX[EDGXx] is enabled."]
    ARMX_1,
}
impl ARMXR {
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
            ARMXR::ARMX_0 => false,
            ARMXR::ARMX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARMXR {
        match value {
            false => ARMXR::ARMX_0,
            true => ARMXR::ARMX_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARMX_0`"]
    #[inline]
    pub fn is_armx_0(&self) -> bool {
        *self == ARMXR::ARMX_0
    }
    #[doc = "Checks if the value of the field is `ARMX_1`"]
    #[inline]
    pub fn is_armx_1(&self) -> bool {
        *self == ARMXR::ARMX_1
    }
}
#[doc = "Possible values of the field `ONESHOTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTXR {
    #[doc = "no description available"]
    ONESHOTX_0,
    #[doc = "no description available"]
    ONESHOTX_1,
}
impl ONESHOTXR {
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
            ONESHOTXR::ONESHOTX_0 => false,
            ONESHOTXR::ONESHOTX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONESHOTXR {
        match value {
            false => ONESHOTXR::ONESHOTX_0,
            true => ONESHOTXR::ONESHOTX_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOTX_0`"]
    #[inline]
    pub fn is_oneshotx_0(&self) -> bool {
        *self == ONESHOTXR::ONESHOTX_0
    }
    #[doc = "Checks if the value of the field is `ONESHOTX_1`"]
    #[inline]
    pub fn is_oneshotx_1(&self) -> bool {
        *self == ONESHOTXR::ONESHOTX_1
    }
}
#[doc = "Possible values of the field `EDGX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGX0R {
    #[doc = "Disabled"]
    EDGX0_0,
    #[doc = "Capture falling edges"]
    EDGX0_1,
    #[doc = "Capture rising edges"]
    EDGX0_2,
    #[doc = "Capture any edge"]
    EDGX0_3,
}
impl EDGX0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGX0R::EDGX0_0 => 0,
            EDGX0R::EDGX0_1 => 1,
            EDGX0R::EDGX0_2 => 2,
            EDGX0R::EDGX0_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGX0R {
        match value {
            0 => EDGX0R::EDGX0_0,
            1 => EDGX0R::EDGX0_1,
            2 => EDGX0R::EDGX0_2,
            3 => EDGX0R::EDGX0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGX0_0`"]
    #[inline]
    pub fn is_edgx0_0(&self) -> bool {
        *self == EDGX0R::EDGX0_0
    }
    #[doc = "Checks if the value of the field is `EDGX0_1`"]
    #[inline]
    pub fn is_edgx0_1(&self) -> bool {
        *self == EDGX0R::EDGX0_1
    }
    #[doc = "Checks if the value of the field is `EDGX0_2`"]
    #[inline]
    pub fn is_edgx0_2(&self) -> bool {
        *self == EDGX0R::EDGX0_2
    }
    #[doc = "Checks if the value of the field is `EDGX0_3`"]
    #[inline]
    pub fn is_edgx0_3(&self) -> bool {
        *self == EDGX0R::EDGX0_3
    }
}
#[doc = "Possible values of the field `EDGX1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGX1R {
    #[doc = "Disabled"]
    EDGX1_0,
    #[doc = "Capture falling edges"]
    EDGX1_1,
    #[doc = "Capture rising edges"]
    EDGX1_2,
    #[doc = "Capture any edge"]
    EDGX1_3,
}
impl EDGX1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGX1R::EDGX1_0 => 0,
            EDGX1R::EDGX1_1 => 1,
            EDGX1R::EDGX1_2 => 2,
            EDGX1R::EDGX1_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGX1R {
        match value {
            0 => EDGX1R::EDGX1_0,
            1 => EDGX1R::EDGX1_1,
            2 => EDGX1R::EDGX1_2,
            3 => EDGX1R::EDGX1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGX1_0`"]
    #[inline]
    pub fn is_edgx1_0(&self) -> bool {
        *self == EDGX1R::EDGX1_0
    }
    #[doc = "Checks if the value of the field is `EDGX1_1`"]
    #[inline]
    pub fn is_edgx1_1(&self) -> bool {
        *self == EDGX1R::EDGX1_1
    }
    #[doc = "Checks if the value of the field is `EDGX1_2`"]
    #[inline]
    pub fn is_edgx1_2(&self) -> bool {
        *self == EDGX1R::EDGX1_2
    }
    #[doc = "Checks if the value of the field is `EDGX1_3`"]
    #[inline]
    pub fn is_edgx1_3(&self) -> bool {
        *self == EDGX1R::EDGX1_3
    }
}
#[doc = "Possible values of the field `INP_SELX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_SELXR {
    #[doc = "Raw PWM_X input signal selected as source."]
    INP_SELX_0,
    #[doc = "no description available"]
    INP_SELX_1,
}
impl INP_SELXR {
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
            INP_SELXR::INP_SELX_0 => false,
            INP_SELXR::INP_SELX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INP_SELXR {
        match value {
            false => INP_SELXR::INP_SELX_0,
            true => INP_SELXR::INP_SELX_1,
        }
    }
    #[doc = "Checks if the value of the field is `INP_SELX_0`"]
    #[inline]
    pub fn is_inp_selx_0(&self) -> bool {
        *self == INP_SELXR::INP_SELX_0
    }
    #[doc = "Checks if the value of the field is `INP_SELX_1`"]
    #[inline]
    pub fn is_inp_selx_1(&self) -> bool {
        *self == INP_SELXR::INP_SELX_1
    }
}
#[doc = "Possible values of the field `EDGCNTX_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGCNTX_ENR {
    #[doc = "Edge counter disabled and held in reset"]
    EDGCNTX_EN_0,
    #[doc = "Edge counter enabled"]
    EDGCNTX_EN_1,
}
impl EDGCNTX_ENR {
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
            EDGCNTX_ENR::EDGCNTX_EN_0 => false,
            EDGCNTX_ENR::EDGCNTX_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGCNTX_ENR {
        match value {
            false => EDGCNTX_ENR::EDGCNTX_EN_0,
            true => EDGCNTX_ENR::EDGCNTX_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDGCNTX_EN_0`"]
    #[inline]
    pub fn is_edgcntx_en_0(&self) -> bool {
        *self == EDGCNTX_ENR::EDGCNTX_EN_0
    }
    #[doc = "Checks if the value of the field is `EDGCNTX_EN_1`"]
    #[inline]
    pub fn is_edgcntx_en_1(&self) -> bool {
        *self == EDGCNTX_ENR::EDGCNTX_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct CFXWMR {
    bits: u8,
}
impl CFXWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CX0CNTR {
    bits: u8,
}
impl CX0CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CX1CNTR {
    bits: u8,
}
impl CX1CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ARMX`"]
pub enum ARMXW {
    #[doc = "Input capture operation is disabled."]
    ARMX_0,
    #[doc = "Input capture operation as specified by CAPTCTRLX[EDGXx] is enabled."]
    ARMX_1,
}
impl ARMXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARMXW::ARMX_0 => false,
            ARMXW::ARMX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARMXW<'a> {
    w: &'a mut W,
}
impl<'a> _ARMXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARMXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input capture operation is disabled."]
    #[inline]
    pub fn armx_0(self) -> &'a mut W {
        self.variant(ARMXW::ARMX_0)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLX[EDGXx] is enabled."]
    #[inline]
    pub fn armx_1(self) -> &'a mut W {
        self.variant(ARMXW::ARMX_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ONESHOTX`"]
pub enum ONESHOTXW {
    #[doc = "no description available"]
    ONESHOTX_0,
    #[doc = "no description available"]
    ONESHOTX_1,
}
impl ONESHOTXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONESHOTXW::ONESHOTX_0 => false,
            ONESHOTXW::ONESHOTX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONESHOTXW<'a> {
    w: &'a mut W,
}
impl<'a> _ONESHOTXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONESHOTXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn oneshotx_0(self) -> &'a mut W {
        self.variant(ONESHOTXW::ONESHOTX_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn oneshotx_1(self) -> &'a mut W {
        self.variant(ONESHOTXW::ONESHOTX_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGX0`"]
pub enum EDGX0W {
    #[doc = "Disabled"]
    EDGX0_0,
    #[doc = "Capture falling edges"]
    EDGX0_1,
    #[doc = "Capture rising edges"]
    EDGX0_2,
    #[doc = "Capture any edge"]
    EDGX0_3,
}
impl EDGX0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGX0W::EDGX0_0 => 0,
            EDGX0W::EDGX0_1 => 1,
            EDGX0W::EDGX0_2 => 2,
            EDGX0W::EDGX0_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGX0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGX0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGX0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn edgx0_0(self) -> &'a mut W {
        self.variant(EDGX0W::EDGX0_0)
    }
    #[doc = "Capture falling edges"]
    #[inline]
    pub fn edgx0_1(self) -> &'a mut W {
        self.variant(EDGX0W::EDGX0_1)
    }
    #[doc = "Capture rising edges"]
    #[inline]
    pub fn edgx0_2(self) -> &'a mut W {
        self.variant(EDGX0W::EDGX0_2)
    }
    #[doc = "Capture any edge"]
    #[inline]
    pub fn edgx0_3(self) -> &'a mut W {
        self.variant(EDGX0W::EDGX0_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGX1`"]
pub enum EDGX1W {
    #[doc = "Disabled"]
    EDGX1_0,
    #[doc = "Capture falling edges"]
    EDGX1_1,
    #[doc = "Capture rising edges"]
    EDGX1_2,
    #[doc = "Capture any edge"]
    EDGX1_3,
}
impl EDGX1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGX1W::EDGX1_0 => 0,
            EDGX1W::EDGX1_1 => 1,
            EDGX1W::EDGX1_2 => 2,
            EDGX1W::EDGX1_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGX1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGX1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGX1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn edgx1_0(self) -> &'a mut W {
        self.variant(EDGX1W::EDGX1_0)
    }
    #[doc = "Capture falling edges"]
    #[inline]
    pub fn edgx1_1(self) -> &'a mut W {
        self.variant(EDGX1W::EDGX1_1)
    }
    #[doc = "Capture rising edges"]
    #[inline]
    pub fn edgx1_2(self) -> &'a mut W {
        self.variant(EDGX1W::EDGX1_2)
    }
    #[doc = "Capture any edge"]
    #[inline]
    pub fn edgx1_3(self) -> &'a mut W {
        self.variant(EDGX1W::EDGX1_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INP_SELX`"]
pub enum INP_SELXW {
    #[doc = "Raw PWM_X input signal selected as source."]
    INP_SELX_0,
    #[doc = "no description available"]
    INP_SELX_1,
}
impl INP_SELXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INP_SELXW::INP_SELX_0 => false,
            INP_SELXW::INP_SELX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INP_SELXW<'a> {
    w: &'a mut W,
}
impl<'a> _INP_SELXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INP_SELXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Raw PWM_X input signal selected as source."]
    #[inline]
    pub fn inp_selx_0(self) -> &'a mut W {
        self.variant(INP_SELXW::INP_SELX_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn inp_selx_1(self) -> &'a mut W {
        self.variant(INP_SELXW::INP_SELX_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGCNTX_EN`"]
pub enum EDGCNTX_ENW {
    #[doc = "Edge counter disabled and held in reset"]
    EDGCNTX_EN_0,
    #[doc = "Edge counter enabled"]
    EDGCNTX_EN_1,
}
impl EDGCNTX_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGCNTX_ENW::EDGCNTX_EN_0 => false,
            EDGCNTX_ENW::EDGCNTX_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGCNTX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGCNTX_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGCNTX_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge counter disabled and held in reset"]
    #[inline]
    pub fn edgcntx_en_0(self) -> &'a mut W {
        self.variant(EDGCNTX_ENW::EDGCNTX_EN_0)
    }
    #[doc = "Edge counter enabled"]
    #[inline]
    pub fn edgcntx_en_1(self) -> &'a mut W {
        self.variant(EDGCNTX_ENW::EDGCNTX_EN_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFXWMW<'a> {
    w: &'a mut W,
}
impl<'a> _CFXWMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Arm X"]
    #[inline]
    pub fn armx(&self) -> ARMXR {
        ARMXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - One Shot Mode Aux"]
    #[inline]
    pub fn oneshotx(&self) -> ONESHOTXR {
        ONESHOTXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 2:3 - Edge X 0"]
    #[inline]
    pub fn edgx0(&self) -> EDGX0R {
        EDGX0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - Edge X 1"]
    #[inline]
    pub fn edgx1(&self) -> EDGX1R {
        EDGX1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Input Select X"]
    #[inline]
    pub fn inp_selx(&self) -> INP_SELXR {
        INP_SELXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Edge Counter X Enable"]
    #[inline]
    pub fn edgcntx_en(&self) -> EDGCNTX_ENR {
        EDGCNTX_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:9 - Capture X FIFOs Water Mark"]
    #[inline]
    pub fn cfxwm(&self) -> CFXWMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CFXWMR { bits }
    }
    #[doc = "Bits 10:12 - Capture X0 FIFO Word Count"]
    #[inline]
    pub fn cx0cnt(&self) -> CX0CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CX0CNTR { bits }
    }
    #[doc = "Bits 13:15 - Capture X1 FIFO Word Count"]
    #[inline]
    pub fn cx1cnt(&self) -> CX1CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CX1CNTR { bits }
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Arm X"]
    #[inline]
    pub fn armx(&mut self) -> _ARMXW {
        _ARMXW { w: self }
    }
    #[doc = "Bit 1 - One Shot Mode Aux"]
    #[inline]
    pub fn oneshotx(&mut self) -> _ONESHOTXW {
        _ONESHOTXW { w: self }
    }
    #[doc = "Bits 2:3 - Edge X 0"]
    #[inline]
    pub fn edgx0(&mut self) -> _EDGX0W {
        _EDGX0W { w: self }
    }
    #[doc = "Bits 4:5 - Edge X 1"]
    #[inline]
    pub fn edgx1(&mut self) -> _EDGX1W {
        _EDGX1W { w: self }
    }
    #[doc = "Bit 6 - Input Select X"]
    #[inline]
    pub fn inp_selx(&mut self) -> _INP_SELXW {
        _INP_SELXW { w: self }
    }
    #[doc = "Bit 7 - Edge Counter X Enable"]
    #[inline]
    pub fn edgcntx_en(&mut self) -> _EDGCNTX_ENW {
        _EDGCNTX_ENW { w: self }
    }
    #[doc = "Bits 8:9 - Capture X FIFOs Water Mark"]
    #[inline]
    pub fn cfxwm(&mut self) -> _CFXWMW {
        _CFXWMW { w: self }
    }
}
