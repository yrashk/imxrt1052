#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM2CAPTCTRLA {
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
#[doc = "Possible values of the field `ARMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARMAR {
    #[doc = "Input capture operation is disabled."]
    ARMA_0,
    #[doc = "Input capture operation as specified by CAPTCTRLA[EDGAx] is enabled."]
    ARMA_1,
}
impl ARMAR {
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
            ARMAR::ARMA_0 => false,
            ARMAR::ARMA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARMAR {
        match value {
            false => ARMAR::ARMA_0,
            true => ARMAR::ARMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARMA_0`"]
    #[inline]
    pub fn is_arma_0(&self) -> bool {
        *self == ARMAR::ARMA_0
    }
    #[doc = "Checks if the value of the field is `ARMA_1`"]
    #[inline]
    pub fn is_arma_1(&self) -> bool {
        *self == ARMAR::ARMA_1
    }
}
#[doc = "Possible values of the field `ONESHOTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTAR {
    #[doc = "no description available"]
    ONESHOTA_0,
    #[doc = "no description available"]
    ONESHOTA_1,
}
impl ONESHOTAR {
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
            ONESHOTAR::ONESHOTA_0 => false,
            ONESHOTAR::ONESHOTA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONESHOTAR {
        match value {
            false => ONESHOTAR::ONESHOTA_0,
            true => ONESHOTAR::ONESHOTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOTA_0`"]
    #[inline]
    pub fn is_oneshota_0(&self) -> bool {
        *self == ONESHOTAR::ONESHOTA_0
    }
    #[doc = "Checks if the value of the field is `ONESHOTA_1`"]
    #[inline]
    pub fn is_oneshota_1(&self) -> bool {
        *self == ONESHOTAR::ONESHOTA_1
    }
}
#[doc = "Possible values of the field `EDGA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGA0R {
    #[doc = "Disabled"]
    EDGA0_0,
    #[doc = "Capture falling edges"]
    EDGA0_1,
    #[doc = "Capture rising edges"]
    EDGA0_2,
    #[doc = "Capture any edge"]
    EDGA0_3,
}
impl EDGA0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGA0R::EDGA0_0 => 0,
            EDGA0R::EDGA0_1 => 1,
            EDGA0R::EDGA0_2 => 2,
            EDGA0R::EDGA0_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGA0R {
        match value {
            0 => EDGA0R::EDGA0_0,
            1 => EDGA0R::EDGA0_1,
            2 => EDGA0R::EDGA0_2,
            3 => EDGA0R::EDGA0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGA0_0`"]
    #[inline]
    pub fn is_edga0_0(&self) -> bool {
        *self == EDGA0R::EDGA0_0
    }
    #[doc = "Checks if the value of the field is `EDGA0_1`"]
    #[inline]
    pub fn is_edga0_1(&self) -> bool {
        *self == EDGA0R::EDGA0_1
    }
    #[doc = "Checks if the value of the field is `EDGA0_2`"]
    #[inline]
    pub fn is_edga0_2(&self) -> bool {
        *self == EDGA0R::EDGA0_2
    }
    #[doc = "Checks if the value of the field is `EDGA0_3`"]
    #[inline]
    pub fn is_edga0_3(&self) -> bool {
        *self == EDGA0R::EDGA0_3
    }
}
#[doc = "Possible values of the field `EDGA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGA1R {
    #[doc = "Disabled"]
    EDGA1_0,
    #[doc = "Capture falling edges"]
    EDGA1_1,
    #[doc = "Capture rising edges"]
    EDGA1_2,
    #[doc = "Capture any edge"]
    EDGA1_3,
}
impl EDGA1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGA1R::EDGA1_0 => 0,
            EDGA1R::EDGA1_1 => 1,
            EDGA1R::EDGA1_2 => 2,
            EDGA1R::EDGA1_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGA1R {
        match value {
            0 => EDGA1R::EDGA1_0,
            1 => EDGA1R::EDGA1_1,
            2 => EDGA1R::EDGA1_2,
            3 => EDGA1R::EDGA1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGA1_0`"]
    #[inline]
    pub fn is_edga1_0(&self) -> bool {
        *self == EDGA1R::EDGA1_0
    }
    #[doc = "Checks if the value of the field is `EDGA1_1`"]
    #[inline]
    pub fn is_edga1_1(&self) -> bool {
        *self == EDGA1R::EDGA1_1
    }
    #[doc = "Checks if the value of the field is `EDGA1_2`"]
    #[inline]
    pub fn is_edga1_2(&self) -> bool {
        *self == EDGA1R::EDGA1_2
    }
    #[doc = "Checks if the value of the field is `EDGA1_3`"]
    #[inline]
    pub fn is_edga1_3(&self) -> bool {
        *self == EDGA1R::EDGA1_3
    }
}
#[doc = "Possible values of the field `INP_SELA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_SELAR {
    #[doc = "Raw PWM_A input signal selected as source."]
    INP_SELA_0,
    #[doc = "no description available"]
    INP_SELA_1,
}
impl INP_SELAR {
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
            INP_SELAR::INP_SELA_0 => false,
            INP_SELAR::INP_SELA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INP_SELAR {
        match value {
            false => INP_SELAR::INP_SELA_0,
            true => INP_SELAR::INP_SELA_1,
        }
    }
    #[doc = "Checks if the value of the field is `INP_SELA_0`"]
    #[inline]
    pub fn is_inp_sela_0(&self) -> bool {
        *self == INP_SELAR::INP_SELA_0
    }
    #[doc = "Checks if the value of the field is `INP_SELA_1`"]
    #[inline]
    pub fn is_inp_sela_1(&self) -> bool {
        *self == INP_SELAR::INP_SELA_1
    }
}
#[doc = "Possible values of the field `EDGCNTA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGCNTA_ENR {
    #[doc = "Edge counter disabled and held in reset"]
    EDGCNTA_EN_0,
    #[doc = "Edge counter enabled"]
    EDGCNTA_EN_1,
}
impl EDGCNTA_ENR {
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
            EDGCNTA_ENR::EDGCNTA_EN_0 => false,
            EDGCNTA_ENR::EDGCNTA_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGCNTA_ENR {
        match value {
            false => EDGCNTA_ENR::EDGCNTA_EN_0,
            true => EDGCNTA_ENR::EDGCNTA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDGCNTA_EN_0`"]
    #[inline]
    pub fn is_edgcnta_en_0(&self) -> bool {
        *self == EDGCNTA_ENR::EDGCNTA_EN_0
    }
    #[doc = "Checks if the value of the field is `EDGCNTA_EN_1`"]
    #[inline]
    pub fn is_edgcnta_en_1(&self) -> bool {
        *self == EDGCNTA_ENR::EDGCNTA_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct CFAWMR {
    bits: u8,
}
impl CFAWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CA0CNTR {
    bits: u8,
}
impl CA0CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CA1CNTR {
    bits: u8,
}
impl CA1CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ARMA`"]
pub enum ARMAW {
    #[doc = "Input capture operation is disabled."]
    ARMA_0,
    #[doc = "Input capture operation as specified by CAPTCTRLA[EDGAx] is enabled."]
    ARMA_1,
}
impl ARMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARMAW::ARMA_0 => false,
            ARMAW::ARMA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARMAW<'a> {
    w: &'a mut W,
}
impl<'a> _ARMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input capture operation is disabled."]
    #[inline]
    pub fn arma_0(self) -> &'a mut W {
        self.variant(ARMAW::ARMA_0)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLA[EDGAx] is enabled."]
    #[inline]
    pub fn arma_1(self) -> &'a mut W {
        self.variant(ARMAW::ARMA_1)
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
#[doc = "Values that can be written to the field `ONESHOTA`"]
pub enum ONESHOTAW {
    #[doc = "no description available"]
    ONESHOTA_0,
    #[doc = "no description available"]
    ONESHOTA_1,
}
impl ONESHOTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONESHOTAW::ONESHOTA_0 => false,
            ONESHOTAW::ONESHOTA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONESHOTAW<'a> {
    w: &'a mut W,
}
impl<'a> _ONESHOTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONESHOTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn oneshota_0(self) -> &'a mut W {
        self.variant(ONESHOTAW::ONESHOTA_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn oneshota_1(self) -> &'a mut W {
        self.variant(ONESHOTAW::ONESHOTA_1)
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
#[doc = "Values that can be written to the field `EDGA0`"]
pub enum EDGA0W {
    #[doc = "Disabled"]
    EDGA0_0,
    #[doc = "Capture falling edges"]
    EDGA0_1,
    #[doc = "Capture rising edges"]
    EDGA0_2,
    #[doc = "Capture any edge"]
    EDGA0_3,
}
impl EDGA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGA0W::EDGA0_0 => 0,
            EDGA0W::EDGA0_1 => 1,
            EDGA0W::EDGA0_2 => 2,
            EDGA0W::EDGA0_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGA0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGA0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn edga0_0(self) -> &'a mut W {
        self.variant(EDGA0W::EDGA0_0)
    }
    #[doc = "Capture falling edges"]
    #[inline]
    pub fn edga0_1(self) -> &'a mut W {
        self.variant(EDGA0W::EDGA0_1)
    }
    #[doc = "Capture rising edges"]
    #[inline]
    pub fn edga0_2(self) -> &'a mut W {
        self.variant(EDGA0W::EDGA0_2)
    }
    #[doc = "Capture any edge"]
    #[inline]
    pub fn edga0_3(self) -> &'a mut W {
        self.variant(EDGA0W::EDGA0_3)
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
#[doc = "Values that can be written to the field `EDGA1`"]
pub enum EDGA1W {
    #[doc = "Disabled"]
    EDGA1_0,
    #[doc = "Capture falling edges"]
    EDGA1_1,
    #[doc = "Capture rising edges"]
    EDGA1_2,
    #[doc = "Capture any edge"]
    EDGA1_3,
}
impl EDGA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGA1W::EDGA1_0 => 0,
            EDGA1W::EDGA1_1 => 1,
            EDGA1W::EDGA1_2 => 2,
            EDGA1W::EDGA1_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGA1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGA1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn edga1_0(self) -> &'a mut W {
        self.variant(EDGA1W::EDGA1_0)
    }
    #[doc = "Capture falling edges"]
    #[inline]
    pub fn edga1_1(self) -> &'a mut W {
        self.variant(EDGA1W::EDGA1_1)
    }
    #[doc = "Capture rising edges"]
    #[inline]
    pub fn edga1_2(self) -> &'a mut W {
        self.variant(EDGA1W::EDGA1_2)
    }
    #[doc = "Capture any edge"]
    #[inline]
    pub fn edga1_3(self) -> &'a mut W {
        self.variant(EDGA1W::EDGA1_3)
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
#[doc = "Values that can be written to the field `INP_SELA`"]
pub enum INP_SELAW {
    #[doc = "Raw PWM_A input signal selected as source."]
    INP_SELA_0,
    #[doc = "no description available"]
    INP_SELA_1,
}
impl INP_SELAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INP_SELAW::INP_SELA_0 => false,
            INP_SELAW::INP_SELA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INP_SELAW<'a> {
    w: &'a mut W,
}
impl<'a> _INP_SELAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INP_SELAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Raw PWM_A input signal selected as source."]
    #[inline]
    pub fn inp_sela_0(self) -> &'a mut W {
        self.variant(INP_SELAW::INP_SELA_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn inp_sela_1(self) -> &'a mut W {
        self.variant(INP_SELAW::INP_SELA_1)
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
#[doc = "Values that can be written to the field `EDGCNTA_EN`"]
pub enum EDGCNTA_ENW {
    #[doc = "Edge counter disabled and held in reset"]
    EDGCNTA_EN_0,
    #[doc = "Edge counter enabled"]
    EDGCNTA_EN_1,
}
impl EDGCNTA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGCNTA_ENW::EDGCNTA_EN_0 => false,
            EDGCNTA_ENW::EDGCNTA_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGCNTA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGCNTA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGCNTA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge counter disabled and held in reset"]
    #[inline]
    pub fn edgcnta_en_0(self) -> &'a mut W {
        self.variant(EDGCNTA_ENW::EDGCNTA_EN_0)
    }
    #[doc = "Edge counter enabled"]
    #[inline]
    pub fn edgcnta_en_1(self) -> &'a mut W {
        self.variant(EDGCNTA_ENW::EDGCNTA_EN_1)
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
pub struct _CFAWMW<'a> {
    w: &'a mut W,
}
impl<'a> _CFAWMW<'a> {
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
    #[doc = "Bit 0 - Arm A"]
    #[inline]
    pub fn arma(&self) -> ARMAR {
        ARMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - One Shot Mode A"]
    #[inline]
    pub fn oneshota(&self) -> ONESHOTAR {
        ONESHOTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 2:3 - Edge A 0"]
    #[inline]
    pub fn edga0(&self) -> EDGA0R {
        EDGA0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - Edge A 1"]
    #[inline]
    pub fn edga1(&self) -> EDGA1R {
        EDGA1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Input Select A"]
    #[inline]
    pub fn inp_sela(&self) -> INP_SELAR {
        INP_SELAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Edge Counter A Enable"]
    #[inline]
    pub fn edgcnta_en(&self) -> EDGCNTA_ENR {
        EDGCNTA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:9 - Capture A FIFOs Water Mark"]
    #[inline]
    pub fn cfawm(&self) -> CFAWMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CFAWMR { bits }
    }
    #[doc = "Bits 10:12 - Capture A0 FIFO Word Count"]
    #[inline]
    pub fn ca0cnt(&self) -> CA0CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CA0CNTR { bits }
    }
    #[doc = "Bits 13:15 - Capture A1 FIFO Word Count"]
    #[inline]
    pub fn ca1cnt(&self) -> CA1CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CA1CNTR { bits }
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
    #[doc = "Bit 0 - Arm A"]
    #[inline]
    pub fn arma(&mut self) -> _ARMAW {
        _ARMAW { w: self }
    }
    #[doc = "Bit 1 - One Shot Mode A"]
    #[inline]
    pub fn oneshota(&mut self) -> _ONESHOTAW {
        _ONESHOTAW { w: self }
    }
    #[doc = "Bits 2:3 - Edge A 0"]
    #[inline]
    pub fn edga0(&mut self) -> _EDGA0W {
        _EDGA0W { w: self }
    }
    #[doc = "Bits 4:5 - Edge A 1"]
    #[inline]
    pub fn edga1(&mut self) -> _EDGA1W {
        _EDGA1W { w: self }
    }
    #[doc = "Bit 6 - Input Select A"]
    #[inline]
    pub fn inp_sela(&mut self) -> _INP_SELAW {
        _INP_SELAW { w: self }
    }
    #[doc = "Bit 7 - Edge Counter A Enable"]
    #[inline]
    pub fn edgcnta_en(&mut self) -> _EDGCNTA_ENW {
        _EDGCNTA_ENW { w: self }
    }
    #[doc = "Bits 8:9 - Capture A FIFOs Water Mark"]
    #[inline]
    pub fn cfawm(&mut self) -> _CFAWMW {
        _CFAWMW { w: self }
    }
}
