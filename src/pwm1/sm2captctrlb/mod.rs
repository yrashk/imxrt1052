#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM2CAPTCTRLB {
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
#[doc = "Possible values of the field `ARMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARMBR {
    #[doc = "Input capture operation is disabled."]
    ARMB_0,
    #[doc = "Input capture operation as specified by CAPTCTRLB[EDGBx] is enabled."]
    ARMB_1,
}
impl ARMBR {
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
            ARMBR::ARMB_0 => false,
            ARMBR::ARMB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARMBR {
        match value {
            false => ARMBR::ARMB_0,
            true => ARMBR::ARMB_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARMB_0`"]
    #[inline]
    pub fn is_armb_0(&self) -> bool {
        *self == ARMBR::ARMB_0
    }
    #[doc = "Checks if the value of the field is `ARMB_1`"]
    #[inline]
    pub fn is_armb_1(&self) -> bool {
        *self == ARMBR::ARMB_1
    }
}
#[doc = "Possible values of the field `ONESHOTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTBR {
    #[doc = "no description available"]
    ONESHOTB_0,
    #[doc = "no description available"]
    ONESHOTB_1,
}
impl ONESHOTBR {
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
            ONESHOTBR::ONESHOTB_0 => false,
            ONESHOTBR::ONESHOTB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONESHOTBR {
        match value {
            false => ONESHOTBR::ONESHOTB_0,
            true => ONESHOTBR::ONESHOTB_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOTB_0`"]
    #[inline]
    pub fn is_oneshotb_0(&self) -> bool {
        *self == ONESHOTBR::ONESHOTB_0
    }
    #[doc = "Checks if the value of the field is `ONESHOTB_1`"]
    #[inline]
    pub fn is_oneshotb_1(&self) -> bool {
        *self == ONESHOTBR::ONESHOTB_1
    }
}
#[doc = "Possible values of the field `EDGB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGB0R {
    #[doc = "Disabled"]
    EDGB0_0,
    #[doc = "Capture falling edges"]
    EDGB0_1,
    #[doc = "Capture rising edges"]
    EDGB0_2,
    #[doc = "Capture any edge"]
    EDGB0_3,
}
impl EDGB0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGB0R::EDGB0_0 => 0,
            EDGB0R::EDGB0_1 => 1,
            EDGB0R::EDGB0_2 => 2,
            EDGB0R::EDGB0_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGB0R {
        match value {
            0 => EDGB0R::EDGB0_0,
            1 => EDGB0R::EDGB0_1,
            2 => EDGB0R::EDGB0_2,
            3 => EDGB0R::EDGB0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGB0_0`"]
    #[inline]
    pub fn is_edgb0_0(&self) -> bool {
        *self == EDGB0R::EDGB0_0
    }
    #[doc = "Checks if the value of the field is `EDGB0_1`"]
    #[inline]
    pub fn is_edgb0_1(&self) -> bool {
        *self == EDGB0R::EDGB0_1
    }
    #[doc = "Checks if the value of the field is `EDGB0_2`"]
    #[inline]
    pub fn is_edgb0_2(&self) -> bool {
        *self == EDGB0R::EDGB0_2
    }
    #[doc = "Checks if the value of the field is `EDGB0_3`"]
    #[inline]
    pub fn is_edgb0_3(&self) -> bool {
        *self == EDGB0R::EDGB0_3
    }
}
#[doc = "Possible values of the field `EDGB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGB1R {
    #[doc = "Disabled"]
    EDGB1_0,
    #[doc = "Capture falling edges"]
    EDGB1_1,
    #[doc = "Capture rising edges"]
    EDGB1_2,
    #[doc = "Capture any edge"]
    EDGB1_3,
}
impl EDGB1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGB1R::EDGB1_0 => 0,
            EDGB1R::EDGB1_1 => 1,
            EDGB1R::EDGB1_2 => 2,
            EDGB1R::EDGB1_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGB1R {
        match value {
            0 => EDGB1R::EDGB1_0,
            1 => EDGB1R::EDGB1_1,
            2 => EDGB1R::EDGB1_2,
            3 => EDGB1R::EDGB1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGB1_0`"]
    #[inline]
    pub fn is_edgb1_0(&self) -> bool {
        *self == EDGB1R::EDGB1_0
    }
    #[doc = "Checks if the value of the field is `EDGB1_1`"]
    #[inline]
    pub fn is_edgb1_1(&self) -> bool {
        *self == EDGB1R::EDGB1_1
    }
    #[doc = "Checks if the value of the field is `EDGB1_2`"]
    #[inline]
    pub fn is_edgb1_2(&self) -> bool {
        *self == EDGB1R::EDGB1_2
    }
    #[doc = "Checks if the value of the field is `EDGB1_3`"]
    #[inline]
    pub fn is_edgb1_3(&self) -> bool {
        *self == EDGB1R::EDGB1_3
    }
}
#[doc = "Possible values of the field `INP_SELB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_SELBR {
    #[doc = "Raw PWM_B input signal selected as source."]
    INP_SELB_0,
    #[doc = "no description available"]
    INP_SELB_1,
}
impl INP_SELBR {
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
            INP_SELBR::INP_SELB_0 => false,
            INP_SELBR::INP_SELB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INP_SELBR {
        match value {
            false => INP_SELBR::INP_SELB_0,
            true => INP_SELBR::INP_SELB_1,
        }
    }
    #[doc = "Checks if the value of the field is `INP_SELB_0`"]
    #[inline]
    pub fn is_inp_selb_0(&self) -> bool {
        *self == INP_SELBR::INP_SELB_0
    }
    #[doc = "Checks if the value of the field is `INP_SELB_1`"]
    #[inline]
    pub fn is_inp_selb_1(&self) -> bool {
        *self == INP_SELBR::INP_SELB_1
    }
}
#[doc = "Possible values of the field `EDGCNTB_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGCNTB_ENR {
    #[doc = "Edge counter disabled and held in reset"]
    EDGCNTB_EN_0,
    #[doc = "Edge counter enabled"]
    EDGCNTB_EN_1,
}
impl EDGCNTB_ENR {
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
            EDGCNTB_ENR::EDGCNTB_EN_0 => false,
            EDGCNTB_ENR::EDGCNTB_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGCNTB_ENR {
        match value {
            false => EDGCNTB_ENR::EDGCNTB_EN_0,
            true => EDGCNTB_ENR::EDGCNTB_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDGCNTB_EN_0`"]
    #[inline]
    pub fn is_edgcntb_en_0(&self) -> bool {
        *self == EDGCNTB_ENR::EDGCNTB_EN_0
    }
    #[doc = "Checks if the value of the field is `EDGCNTB_EN_1`"]
    #[inline]
    pub fn is_edgcntb_en_1(&self) -> bool {
        *self == EDGCNTB_ENR::EDGCNTB_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct CFBWMR {
    bits: u8,
}
impl CFBWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CB0CNTR {
    bits: u8,
}
impl CB0CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CB1CNTR {
    bits: u8,
}
impl CB1CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ARMB`"]
pub enum ARMBW {
    #[doc = "Input capture operation is disabled."]
    ARMB_0,
    #[doc = "Input capture operation as specified by CAPTCTRLB[EDGBx] is enabled."]
    ARMB_1,
}
impl ARMBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARMBW::ARMB_0 => false,
            ARMBW::ARMB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARMBW<'a> {
    w: &'a mut W,
}
impl<'a> _ARMBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARMBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input capture operation is disabled."]
    #[inline]
    pub fn armb_0(self) -> &'a mut W {
        self.variant(ARMBW::ARMB_0)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLB[EDGBx] is enabled."]
    #[inline]
    pub fn armb_1(self) -> &'a mut W {
        self.variant(ARMBW::ARMB_1)
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
#[doc = "Values that can be written to the field `ONESHOTB`"]
pub enum ONESHOTBW {
    #[doc = "no description available"]
    ONESHOTB_0,
    #[doc = "no description available"]
    ONESHOTB_1,
}
impl ONESHOTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONESHOTBW::ONESHOTB_0 => false,
            ONESHOTBW::ONESHOTB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONESHOTBW<'a> {
    w: &'a mut W,
}
impl<'a> _ONESHOTBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONESHOTBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn oneshotb_0(self) -> &'a mut W {
        self.variant(ONESHOTBW::ONESHOTB_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn oneshotb_1(self) -> &'a mut W {
        self.variant(ONESHOTBW::ONESHOTB_1)
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
#[doc = "Values that can be written to the field `EDGB0`"]
pub enum EDGB0W {
    #[doc = "Disabled"]
    EDGB0_0,
    #[doc = "Capture falling edges"]
    EDGB0_1,
    #[doc = "Capture rising edges"]
    EDGB0_2,
    #[doc = "Capture any edge"]
    EDGB0_3,
}
impl EDGB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGB0W::EDGB0_0 => 0,
            EDGB0W::EDGB0_1 => 1,
            EDGB0W::EDGB0_2 => 2,
            EDGB0W::EDGB0_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGB0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGB0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn edgb0_0(self) -> &'a mut W {
        self.variant(EDGB0W::EDGB0_0)
    }
    #[doc = "Capture falling edges"]
    #[inline]
    pub fn edgb0_1(self) -> &'a mut W {
        self.variant(EDGB0W::EDGB0_1)
    }
    #[doc = "Capture rising edges"]
    #[inline]
    pub fn edgb0_2(self) -> &'a mut W {
        self.variant(EDGB0W::EDGB0_2)
    }
    #[doc = "Capture any edge"]
    #[inline]
    pub fn edgb0_3(self) -> &'a mut W {
        self.variant(EDGB0W::EDGB0_3)
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
#[doc = "Values that can be written to the field `EDGB1`"]
pub enum EDGB1W {
    #[doc = "Disabled"]
    EDGB1_0,
    #[doc = "Capture falling edges"]
    EDGB1_1,
    #[doc = "Capture rising edges"]
    EDGB1_2,
    #[doc = "Capture any edge"]
    EDGB1_3,
}
impl EDGB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGB1W::EDGB1_0 => 0,
            EDGB1W::EDGB1_1 => 1,
            EDGB1W::EDGB1_2 => 2,
            EDGB1W::EDGB1_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGB1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGB1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn edgb1_0(self) -> &'a mut W {
        self.variant(EDGB1W::EDGB1_0)
    }
    #[doc = "Capture falling edges"]
    #[inline]
    pub fn edgb1_1(self) -> &'a mut W {
        self.variant(EDGB1W::EDGB1_1)
    }
    #[doc = "Capture rising edges"]
    #[inline]
    pub fn edgb1_2(self) -> &'a mut W {
        self.variant(EDGB1W::EDGB1_2)
    }
    #[doc = "Capture any edge"]
    #[inline]
    pub fn edgb1_3(self) -> &'a mut W {
        self.variant(EDGB1W::EDGB1_3)
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
#[doc = "Values that can be written to the field `INP_SELB`"]
pub enum INP_SELBW {
    #[doc = "Raw PWM_B input signal selected as source."]
    INP_SELB_0,
    #[doc = "no description available"]
    INP_SELB_1,
}
impl INP_SELBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INP_SELBW::INP_SELB_0 => false,
            INP_SELBW::INP_SELB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INP_SELBW<'a> {
    w: &'a mut W,
}
impl<'a> _INP_SELBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INP_SELBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Raw PWM_B input signal selected as source."]
    #[inline]
    pub fn inp_selb_0(self) -> &'a mut W {
        self.variant(INP_SELBW::INP_SELB_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn inp_selb_1(self) -> &'a mut W {
        self.variant(INP_SELBW::INP_SELB_1)
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
#[doc = "Values that can be written to the field `EDGCNTB_EN`"]
pub enum EDGCNTB_ENW {
    #[doc = "Edge counter disabled and held in reset"]
    EDGCNTB_EN_0,
    #[doc = "Edge counter enabled"]
    EDGCNTB_EN_1,
}
impl EDGCNTB_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGCNTB_ENW::EDGCNTB_EN_0 => false,
            EDGCNTB_ENW::EDGCNTB_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGCNTB_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGCNTB_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGCNTB_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge counter disabled and held in reset"]
    #[inline]
    pub fn edgcntb_en_0(self) -> &'a mut W {
        self.variant(EDGCNTB_ENW::EDGCNTB_EN_0)
    }
    #[doc = "Edge counter enabled"]
    #[inline]
    pub fn edgcntb_en_1(self) -> &'a mut W {
        self.variant(EDGCNTB_ENW::EDGCNTB_EN_1)
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
pub struct _CFBWMW<'a> {
    w: &'a mut W,
}
impl<'a> _CFBWMW<'a> {
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
    #[doc = "Bit 0 - Arm B"]
    #[inline]
    pub fn armb(&self) -> ARMBR {
        ARMBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - One Shot Mode B"]
    #[inline]
    pub fn oneshotb(&self) -> ONESHOTBR {
        ONESHOTBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 2:3 - Edge B 0"]
    #[inline]
    pub fn edgb0(&self) -> EDGB0R {
        EDGB0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - Edge B 1"]
    #[inline]
    pub fn edgb1(&self) -> EDGB1R {
        EDGB1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Input Select B"]
    #[inline]
    pub fn inp_selb(&self) -> INP_SELBR {
        INP_SELBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Edge Counter B Enable"]
    #[inline]
    pub fn edgcntb_en(&self) -> EDGCNTB_ENR {
        EDGCNTB_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:9 - Capture B FIFOs Water Mark"]
    #[inline]
    pub fn cfbwm(&self) -> CFBWMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CFBWMR { bits }
    }
    #[doc = "Bits 10:12 - Capture B0 FIFO Word Count"]
    #[inline]
    pub fn cb0cnt(&self) -> CB0CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CB0CNTR { bits }
    }
    #[doc = "Bits 13:15 - Capture B1 FIFO Word Count"]
    #[inline]
    pub fn cb1cnt(&self) -> CB1CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CB1CNTR { bits }
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
    #[doc = "Bit 0 - Arm B"]
    #[inline]
    pub fn armb(&mut self) -> _ARMBW {
        _ARMBW { w: self }
    }
    #[doc = "Bit 1 - One Shot Mode B"]
    #[inline]
    pub fn oneshotb(&mut self) -> _ONESHOTBW {
        _ONESHOTBW { w: self }
    }
    #[doc = "Bits 2:3 - Edge B 0"]
    #[inline]
    pub fn edgb0(&mut self) -> _EDGB0W {
        _EDGB0W { w: self }
    }
    #[doc = "Bits 4:5 - Edge B 1"]
    #[inline]
    pub fn edgb1(&mut self) -> _EDGB1W {
        _EDGB1W { w: self }
    }
    #[doc = "Bit 6 - Input Select B"]
    #[inline]
    pub fn inp_selb(&mut self) -> _INP_SELBW {
        _INP_SELBW { w: self }
    }
    #[doc = "Bit 7 - Edge Counter B Enable"]
    #[inline]
    pub fn edgcntb_en(&mut self) -> _EDGCNTB_ENW {
        _EDGCNTB_ENW { w: self }
    }
    #[doc = "Bits 8:9 - Capture B FIFOs Water Mark"]
    #[inline]
    pub fn cfbwm(&mut self) -> _CFBWMW {
        _CFBWMW { w: self }
    }
}
