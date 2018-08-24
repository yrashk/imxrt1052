#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `OSCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCNTR {
    #[doc = "count 1 ckil"]
    OSCNT_0,
    #[doc = "count 256 ckil's"]
    OSCNT_255,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCNTR::OSCNT_0 => 0,
            OSCNTR::OSCNT_255 => 255,
            OSCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCNTR {
        match value {
            0 => OSCNTR::OSCNT_0,
            255 => OSCNTR::OSCNT_255,
            i => OSCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OSCNT_0`"]
    #[inline]
    pub fn is_oscnt_0(&self) -> bool {
        *self == OSCNTR::OSCNT_0
    }
    #[doc = "Checks if the value of the field is `OSCNT_255`"]
    #[inline]
    pub fn is_oscnt_255(&self) -> bool {
        *self == OSCNTR::OSCNT_255
    }
}
#[doc = "Possible values of the field `COSC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_ENR {
    #[doc = "disable on chip oscillator"]
    COSC_EN_0,
    #[doc = "enable on chip oscillator"]
    COSC_EN_1,
}
impl COSC_ENR {
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
            COSC_ENR::COSC_EN_0 => false,
            COSC_ENR::COSC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSC_ENR {
        match value {
            false => COSC_ENR::COSC_EN_0,
            true => COSC_ENR::COSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_EN_0`"]
    #[inline]
    pub fn is_cosc_en_0(&self) -> bool {
        *self == COSC_ENR::COSC_EN_0
    }
    #[doc = "Checks if the value of the field is `COSC_EN_1`"]
    #[inline]
    pub fn is_cosc_en_1(&self) -> bool {
        *self == COSC_ENR::COSC_EN_1
    }
}
#[doc = "Possible values of the field `REG_BYPASS_COUNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_BYPASS_COUNTR {
    #[doc = "no delay"]
    REG_BYPASS_COUNT_0,
    #[doc = "1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1,
    #[doc = "63 CKIL clock periods delay"]
    REG_BYPASS_COUNT_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REG_BYPASS_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG_BYPASS_COUNTR::REG_BYPASS_COUNT_0 => 0,
            REG_BYPASS_COUNTR::REG_BYPASS_COUNT_1 => 1,
            REG_BYPASS_COUNTR::REG_BYPASS_COUNT_63 => 63,
            REG_BYPASS_COUNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG_BYPASS_COUNTR {
        match value {
            0 => REG_BYPASS_COUNTR::REG_BYPASS_COUNT_0,
            1 => REG_BYPASS_COUNTR::REG_BYPASS_COUNT_1,
            63 => REG_BYPASS_COUNTR::REG_BYPASS_COUNT_63,
            i => REG_BYPASS_COUNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_0`"]
    #[inline]
    pub fn is_reg_bypass_count_0(&self) -> bool {
        *self == REG_BYPASS_COUNTR::REG_BYPASS_COUNT_0
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_1`"]
    #[inline]
    pub fn is_reg_bypass_count_1(&self) -> bool {
        *self == REG_BYPASS_COUNTR::REG_BYPASS_COUNT_1
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_63`"]
    #[inline]
    pub fn is_reg_bypass_count_63(&self) -> bool {
        *self == REG_BYPASS_COUNTR::REG_BYPASS_COUNT_63
    }
}
#[doc = "Possible values of the field `RBC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBC_ENR {
    #[doc = "REG_BYPASS_COUNTER disabled"]
    RBC_EN_0,
    #[doc = "REG_BYPASS_COUNTER enabled."]
    RBC_EN_1,
}
impl RBC_ENR {
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
            RBC_ENR::RBC_EN_0 => false,
            RBC_ENR::RBC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBC_ENR {
        match value {
            false => RBC_ENR::RBC_EN_0,
            true => RBC_ENR::RBC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBC_EN_0`"]
    #[inline]
    pub fn is_rbc_en_0(&self) -> bool {
        *self == RBC_ENR::RBC_EN_0
    }
    #[doc = "Checks if the value of the field is `RBC_EN_1`"]
    #[inline]
    pub fn is_rbc_en_1(&self) -> bool {
        *self == RBC_ENR::RBC_EN_1
    }
}
#[doc = "Values that can be written to the field `OSCNT`"]
pub enum OSCNTW {
    #[doc = "count 1 ckil"]
    OSCNT_0,
    #[doc = "count 256 ckil's"]
    OSCNT_255,
}
impl OSCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCNTW::OSCNT_0 => 0,
            OSCNTW::OSCNT_255 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "count 1 ckil"]
    #[inline]
    pub fn oscnt_0(self) -> &'a mut W {
        self.variant(OSCNTW::OSCNT_0)
    }
    #[doc = "count 256 ckil's"]
    #[inline]
    pub fn oscnt_255(self) -> &'a mut W {
        self.variant(OSCNTW::OSCNT_255)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COSC_EN`"]
pub enum COSC_ENW {
    #[doc = "disable on chip oscillator"]
    COSC_EN_0,
    #[doc = "enable on chip oscillator"]
    COSC_EN_1,
}
impl COSC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSC_ENW::COSC_EN_0 => false,
            COSC_ENW::COSC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COSC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable on chip oscillator"]
    #[inline]
    pub fn cosc_en_0(self) -> &'a mut W {
        self.variant(COSC_ENW::COSC_EN_0)
    }
    #[doc = "enable on chip oscillator"]
    #[inline]
    pub fn cosc_en_1(self) -> &'a mut W {
        self.variant(COSC_ENW::COSC_EN_1)
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
#[doc = "Values that can be written to the field `REG_BYPASS_COUNT`"]
pub enum REG_BYPASS_COUNTW {
    #[doc = "no delay"]
    REG_BYPASS_COUNT_0,
    #[doc = "1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1,
    #[doc = "63 CKIL clock periods delay"]
    REG_BYPASS_COUNT_63,
}
impl REG_BYPASS_COUNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG_BYPASS_COUNTW::REG_BYPASS_COUNT_0 => 0,
            REG_BYPASS_COUNTW::REG_BYPASS_COUNT_1 => 1,
            REG_BYPASS_COUNTW::REG_BYPASS_COUNT_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG_BYPASS_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _REG_BYPASS_COUNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG_BYPASS_COUNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "no delay"]
    #[inline]
    pub fn reg_bypass_count_0(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNTW::REG_BYPASS_COUNT_0)
    }
    #[doc = "1 CKIL clock period delay"]
    #[inline]
    pub fn reg_bypass_count_1(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNTW::REG_BYPASS_COUNT_1)
    }
    #[doc = "63 CKIL clock periods delay"]
    #[inline]
    pub fn reg_bypass_count_63(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNTW::REG_BYPASS_COUNT_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RBC_EN`"]
pub enum RBC_ENW {
    #[doc = "REG_BYPASS_COUNTER disabled"]
    RBC_EN_0,
    #[doc = "REG_BYPASS_COUNTER enabled."]
    RBC_EN_1,
}
impl RBC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBC_ENW::RBC_EN_0 => false,
            RBC_ENW::RBC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RBC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "REG_BYPASS_COUNTER disabled"]
    #[inline]
    pub fn rbc_en_0(self) -> &'a mut W {
        self.variant(RBC_ENW::RBC_EN_0)
    }
    #[doc = "REG_BYPASS_COUNTER enabled."]
    #[inline]
    pub fn rbc_en_1(self) -> &'a mut W {
        self.variant(RBC_ENW::RBC_EN_1)
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
    #[doc = "Bits 0:7 - Oscillator ready counter value"]
    #[inline]
    pub fn oscnt(&self) -> OSCNTR {
        OSCNTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline]
    pub fn cosc_en(&self) -> COSC_ENR {
        COSC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:26 - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline]
    pub fn reg_bypass_count(&self) -> REG_BYPASS_COUNTR {
        REG_BYPASS_COUNTR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Enable for REG_BYPASS_COUNTER"]
    #[inline]
    pub fn rbc_en(&self) -> RBC_ENR {
        RBC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67180159 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Oscillator ready counter value"]
    #[inline]
    pub fn oscnt(&mut self) -> _OSCNTW {
        _OSCNTW { w: self }
    }
    #[doc = "Bit 12 - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline]
    pub fn cosc_en(&mut self) -> _COSC_ENW {
        _COSC_ENW { w: self }
    }
    #[doc = "Bits 21:26 - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline]
    pub fn reg_bypass_count(&mut self) -> _REG_BYPASS_COUNTW {
        _REG_BYPASS_COUNTW { w: self }
    }
    #[doc = "Bit 27 - Enable for REG_BYPASS_COUNTER"]
    #[inline]
    pub fn rbc_en(&mut self) -> _RBC_ENW {
        _RBC_ENW { w: self }
    }
}
