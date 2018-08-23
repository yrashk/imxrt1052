#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TST {
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
#[doc = r" Value of the field"]
pub struct TEST_COUNTR {
    bits: u8,
}
impl TEST_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEST_PERIODR {
    bits: u8,
}
impl TEST_PERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `QDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QDNR {
    #[doc = "Leaves quadrature decoder signal in a positive direction"]
    QDN_0,
    #[doc = "Generates a negative quadrature decoder signal"]
    QDN_1,
}
impl QDNR {
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
            QDNR::QDN_0 => false,
            QDNR::QDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QDNR {
        match value {
            false => QDNR::QDN_0,
            true => QDNR::QDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `QDN_0`"]
    #[inline]
    pub fn is_qdn_0(&self) -> bool {
        *self == QDNR::QDN_0
    }
    #[doc = "Checks if the value of the field is `QDN_1`"]
    #[inline]
    pub fn is_qdn_1(&self) -> bool {
        *self == QDNR::QDN_1
    }
}
#[doc = "Possible values of the field `TCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCER {
    #[doc = "Test count is not enabled"]
    TCE_0,
    #[doc = "Test count is enabled"]
    TCE_1,
}
impl TCER {
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
            TCER::TCE_0 => false,
            TCER::TCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCER {
        match value {
            false => TCER::TCE_0,
            true => TCER::TCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCE_0`"]
    #[inline]
    pub fn is_tce_0(&self) -> bool {
        *self == TCER::TCE_0
    }
    #[doc = "Checks if the value of the field is `TCE_1`"]
    #[inline]
    pub fn is_tce_1(&self) -> bool {
        *self == TCER::TCE_1
    }
}
#[doc = "Possible values of the field `TEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TENR {
    #[doc = "Test module is not enabled"]
    TEN_0,
    #[doc = "Test module is enabled"]
    TEN_1,
}
impl TENR {
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
            TENR::TEN_0 => false,
            TENR::TEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TENR {
        match value {
            false => TENR::TEN_0,
            true => TENR::TEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEN_0`"]
    #[inline]
    pub fn is_ten_0(&self) -> bool {
        *self == TENR::TEN_0
    }
    #[doc = "Checks if the value of the field is `TEN_1`"]
    #[inline]
    pub fn is_ten_1(&self) -> bool {
        *self == TENR::TEN_1
    }
}
#[doc = r" Proxy"]
pub struct _TEST_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TEST_COUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEST_PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _TEST_PERIODW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QDN`"]
pub enum QDNW {
    #[doc = "Leaves quadrature decoder signal in a positive direction"]
    QDN_0,
    #[doc = "Generates a negative quadrature decoder signal"]
    QDN_1,
}
impl QDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QDNW::QDN_0 => false,
            QDNW::QDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QDNW<'a> {
    w: &'a mut W,
}
impl<'a> _QDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Leaves quadrature decoder signal in a positive direction"]
    #[inline]
    pub fn qdn_0(self) -> &'a mut W {
        self.variant(QDNW::QDN_0)
    }
    #[doc = "Generates a negative quadrature decoder signal"]
    #[inline]
    pub fn qdn_1(self) -> &'a mut W {
        self.variant(QDNW::QDN_1)
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
#[doc = "Values that can be written to the field `TCE`"]
pub enum TCEW {
    #[doc = "Test count is not enabled"]
    TCE_0,
    #[doc = "Test count is enabled"]
    TCE_1,
}
impl TCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCEW::TCE_0 => false,
            TCEW::TCE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Test count is not enabled"]
    #[inline]
    pub fn tce_0(self) -> &'a mut W {
        self.variant(TCEW::TCE_0)
    }
    #[doc = "Test count is enabled"]
    #[inline]
    pub fn tce_1(self) -> &'a mut W {
        self.variant(TCEW::TCE_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEN`"]
pub enum TENW {
    #[doc = "Test module is not enabled"]
    TEN_0,
    #[doc = "Test module is enabled"]
    TEN_1,
}
impl TENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TENW::TEN_0 => false,
            TENW::TEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TENW<'a> {
    w: &'a mut W,
}
impl<'a> _TENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Test module is not enabled"]
    #[inline]
    pub fn ten_0(self) -> &'a mut W {
        self.variant(TENW::TEN_0)
    }
    #[doc = "Test module is enabled"]
    #[inline]
    pub fn ten_1(self) -> &'a mut W {
        self.variant(TENW::TEN_1)
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
    #[doc = "Bits 0:7 - These bits hold the number of quadrature advances to generate."]
    #[inline]
    pub fn test_count(&self) -> TEST_COUNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        TEST_COUNTR { bits }
    }
    #[doc = "Bits 8:12 - These bits hold the period of quadrature phase in IPBus clock cycles."]
    #[inline]
    pub fn test_period(&self) -> TEST_PERIODR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        TEST_PERIODR { bits }
    }
    #[doc = "Bit 13 - Quadrature Decoder Negative Signal"]
    #[inline]
    pub fn qdn(&self) -> QDNR {
        QDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Test Counter Enable"]
    #[inline]
    pub fn tce(&self) -> TCER {
        TCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Test Mode Enable"]
    #[inline]
    pub fn ten(&self) -> TENR {
        TENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:7 - These bits hold the number of quadrature advances to generate."]
    #[inline]
    pub fn test_count(&mut self) -> _TEST_COUNTW {
        _TEST_COUNTW { w: self }
    }
    #[doc = "Bits 8:12 - These bits hold the period of quadrature phase in IPBus clock cycles."]
    #[inline]
    pub fn test_period(&mut self) -> _TEST_PERIODW {
        _TEST_PERIODW { w: self }
    }
    #[doc = "Bit 13 - Quadrature Decoder Negative Signal"]
    #[inline]
    pub fn qdn(&mut self) -> _QDNW {
        _QDNW { w: self }
    }
    #[doc = "Bit 14 - Test Counter Enable"]
    #[inline]
    pub fn tce(&mut self) -> _TCEW {
        _TCEW { w: self }
    }
    #[doc = "Bit 15 - Test Mode Enable"]
    #[inline]
    pub fn ten(&mut self) -> _TENW {
        _TENW { w: self }
    }
}
