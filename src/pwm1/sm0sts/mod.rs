#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM0STS {
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
#[doc = "Possible values of the field `CMPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPFR {
    #[doc = "No compare event has occurred for a particular VALx value."]
    CMPF_0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    CMPF_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMPFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMPFR::CMPF_0 => 0,
            CMPFR::CMPF_1 => 1,
            CMPFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMPFR {
        match value {
            0 => CMPFR::CMPF_0,
            1 => CMPFR::CMPF_1,
            i => CMPFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMPF_0`"]
    #[inline]
    pub fn is_cmpf_0(&self) -> bool {
        *self == CMPFR::CMPF_0
    }
    #[doc = "Checks if the value of the field is `CMPF_1`"]
    #[inline]
    pub fn is_cmpf_1(&self) -> bool {
        *self == CMPFR::CMPF_1
    }
}
#[doc = r" Value of the field"]
pub struct CFX0R {
    bits: bool,
}
impl CFX0R {
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
pub struct CFX1R {
    bits: bool,
}
impl CFX1R {
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
pub struct CFB0R {
    bits: bool,
}
impl CFB0R {
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
pub struct CFB1R {
    bits: bool,
}
impl CFB1R {
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
pub struct CFA0R {
    bits: bool,
}
impl CFA0R {
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
pub struct CFA1R {
    bits: bool,
}
impl CFA1R {
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
#[doc = "Possible values of the field `RF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFR {
    #[doc = "No new reload cycle since last STS[RF] clearing"]
    RF_0,
    #[doc = "New reload cycle since last STS[RF] clearing"]
    RF_1,
}
impl RFR {
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
            RFR::RF_0 => false,
            RFR::RF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFR {
        match value {
            false => RFR::RF_0,
            true => RFR::RF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_0`"]
    #[inline]
    pub fn is_rf_0(&self) -> bool {
        *self == RFR::RF_0
    }
    #[doc = "Checks if the value of the field is `RF_1`"]
    #[inline]
    pub fn is_rf_1(&self) -> bool {
        *self == RFR::RF_1
    }
}
#[doc = "Possible values of the field `REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFR {
    #[doc = "No reload error occurred."]
    REF_0,
    #[doc = "Reload signal occurred with non-coherent data and MCTRL[LDOK] = 0."]
    REF_1,
}
impl REFR {
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
            REFR::REF_0 => false,
            REFR::REF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFR {
        match value {
            false => REFR::REF_0,
            true => REFR::REF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REF_0`"]
    #[inline]
    pub fn is_ref_0(&self) -> bool {
        *self == REFR::REF_0
    }
    #[doc = "Checks if the value of the field is `REF_1`"]
    #[inline]
    pub fn is_ref_1(&self) -> bool {
        *self == REFR::REF_1
    }
}
#[doc = "Possible values of the field `RUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUFR {
    #[doc = "No register update has occurred since last reload."]
    RUF_0,
    #[doc = "At least one of the double buffered registers has been updated since the last reload."]
    RUF_1,
}
impl RUFR {
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
            RUFR::RUF_0 => false,
            RUFR::RUF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUFR {
        match value {
            false => RUFR::RUF_0,
            true => RUFR::RUF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUF_0`"]
    #[inline]
    pub fn is_ruf_0(&self) -> bool {
        *self == RUFR::RUF_0
    }
    #[doc = "Checks if the value of the field is `RUF_1`"]
    #[inline]
    pub fn is_ruf_1(&self) -> bool {
        *self == RUFR::RUF_1
    }
}
#[doc = "Values that can be written to the field `CMPF`"]
pub enum CMPFW {
    #[doc = "No compare event has occurred for a particular VALx value."]
    CMPF_0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    CMPF_1,
}
impl CMPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMPFW::CMPF_0 => 0,
            CMPFW::CMPF_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPFW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No compare event has occurred for a particular VALx value."]
    #[inline]
    pub fn cmpf_0(self) -> &'a mut W {
        self.variant(CMPFW::CMPF_0)
    }
    #[doc = "A compare event has occurred for a particular VALx value."]
    #[inline]
    pub fn cmpf_1(self) -> &'a mut W {
        self.variant(CMPFW::CMPF_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFX0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFX0W<'a> {
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
#[doc = r" Proxy"]
pub struct _CFX1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFX1W<'a> {
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
pub struct _CFB0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFB0W<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFB1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFB1W<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFA0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFA0W<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFA1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFA1W<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RF`"]
pub enum RFW {
    #[doc = "No new reload cycle since last STS[RF] clearing"]
    RF_0,
    #[doc = "New reload cycle since last STS[RF] clearing"]
    RF_1,
}
impl RFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFW::RF_0 => false,
            RFW::RF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFW<'a> {
    w: &'a mut W,
}
impl<'a> _RFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No new reload cycle since last STS[RF] clearing"]
    #[inline]
    pub fn rf_0(self) -> &'a mut W {
        self.variant(RFW::RF_0)
    }
    #[doc = "New reload cycle since last STS[RF] clearing"]
    #[inline]
    pub fn rf_1(self) -> &'a mut W {
        self.variant(RFW::RF_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REF`"]
pub enum REFW {
    #[doc = "No reload error occurred."]
    REF_0,
    #[doc = "Reload signal occurred with non-coherent data and MCTRL[LDOK] = 0."]
    REF_1,
}
impl REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFW::REF_0 => false,
            REFW::REF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFW<'a> {
    w: &'a mut W,
}
impl<'a> _REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reload error occurred."]
    #[inline]
    pub fn ref_0(self) -> &'a mut W {
        self.variant(REFW::REF_0)
    }
    #[doc = "Reload signal occurred with non-coherent data and MCTRL[LDOK] = 0."]
    #[inline]
    pub fn ref_1(self) -> &'a mut W {
        self.variant(REFW::REF_1)
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
    #[doc = "Bits 0:5 - Compare Flags"]
    #[inline]
    pub fn cmpf(&self) -> CMPFR {
        CMPFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Capture Flag X0"]
    #[inline]
    pub fn cfx0(&self) -> CFX0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CFX0R { bits }
    }
    #[doc = "Bit 7 - Capture Flag X1"]
    #[inline]
    pub fn cfx1(&self) -> CFX1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CFX1R { bits }
    }
    #[doc = "Bit 8 - Capture Flag B0"]
    #[inline]
    pub fn cfb0(&self) -> CFB0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CFB0R { bits }
    }
    #[doc = "Bit 9 - Capture Flag B1"]
    #[inline]
    pub fn cfb1(&self) -> CFB1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CFB1R { bits }
    }
    #[doc = "Bit 10 - Capture Flag A0"]
    #[inline]
    pub fn cfa0(&self) -> CFA0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CFA0R { bits }
    }
    #[doc = "Bit 11 - Capture Flag A1"]
    #[inline]
    pub fn cfa1(&self) -> CFA1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CFA1R { bits }
    }
    #[doc = "Bit 12 - Reload Flag"]
    #[inline]
    pub fn rf(&self) -> RFR {
        RFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Reload Error Flag"]
    #[inline]
    pub fn ref_(&self) -> REFR {
        REFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Registers Updated Flag"]
    #[inline]
    pub fn ruf(&self) -> RUFR {
        RUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Compare Flags"]
    #[inline]
    pub fn cmpf(&mut self) -> _CMPFW {
        _CMPFW { w: self }
    }
    #[doc = "Bit 6 - Capture Flag X0"]
    #[inline]
    pub fn cfx0(&mut self) -> _CFX0W {
        _CFX0W { w: self }
    }
    #[doc = "Bit 7 - Capture Flag X1"]
    #[inline]
    pub fn cfx1(&mut self) -> _CFX1W {
        _CFX1W { w: self }
    }
    #[doc = "Bit 8 - Capture Flag B0"]
    #[inline]
    pub fn cfb0(&mut self) -> _CFB0W {
        _CFB0W { w: self }
    }
    #[doc = "Bit 9 - Capture Flag B1"]
    #[inline]
    pub fn cfb1(&mut self) -> _CFB1W {
        _CFB1W { w: self }
    }
    #[doc = "Bit 10 - Capture Flag A0"]
    #[inline]
    pub fn cfa0(&mut self) -> _CFA0W {
        _CFA0W { w: self }
    }
    #[doc = "Bit 11 - Capture Flag A1"]
    #[inline]
    pub fn cfa1(&mut self) -> _CFA1W {
        _CFA1W { w: self }
    }
    #[doc = "Bit 12 - Reload Flag"]
    #[inline]
    pub fn rf(&mut self) -> _RFW {
        _RFW { w: self }
    }
    #[doc = "Bit 13 - Reload Error Flag"]
    #[inline]
    pub fn ref_(&mut self) -> _REFW {
        _REFW { w: self }
    }
}
