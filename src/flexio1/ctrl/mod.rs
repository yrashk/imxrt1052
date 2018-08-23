#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `FLEXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXENR {
    #[doc = "FlexIO module is disabled."]
    FLEXEN_0,
    #[doc = "FlexIO module is enabled."]
    FLEXEN_1,
}
impl FLEXENR {
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
            FLEXENR::FLEXEN_0 => false,
            FLEXENR::FLEXEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXENR {
        match value {
            false => FLEXENR::FLEXEN_0,
            true => FLEXENR::FLEXEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXEN_0`"]
    #[inline]
    pub fn is_flexen_0(&self) -> bool {
        *self == FLEXENR::FLEXEN_0
    }
    #[doc = "Checks if the value of the field is `FLEXEN_1`"]
    #[inline]
    pub fn is_flexen_1(&self) -> bool {
        *self == FLEXENR::FLEXEN_1
    }
}
#[doc = "Possible values of the field `SWRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTR {
    #[doc = "Software reset is disabled"]
    SWRST_0,
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    SWRST_1,
}
impl SWRSTR {
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
            SWRSTR::SWRST_0 => false,
            SWRSTR::SWRST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTR {
        match value {
            false => SWRSTR::SWRST_0,
            true => SWRSTR::SWRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWRST_0`"]
    #[inline]
    pub fn is_swrst_0(&self) -> bool {
        *self == SWRSTR::SWRST_0
    }
    #[doc = "Checks if the value of the field is `SWRST_1`"]
    #[inline]
    pub fn is_swrst_1(&self) -> bool {
        *self == SWRSTR::SWRST_1
    }
}
#[doc = "Possible values of the field `FASTACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTACCR {
    #[doc = "Configures for normal register accesses to FlexIO"]
    FASTACC_0,
    #[doc = "Configures for fast register accesses to FlexIO"]
    FASTACC_1,
}
impl FASTACCR {
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
            FASTACCR::FASTACC_0 => false,
            FASTACCR::FASTACC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FASTACCR {
        match value {
            false => FASTACCR::FASTACC_0,
            true => FASTACCR::FASTACC_1,
        }
    }
    #[doc = "Checks if the value of the field is `FASTACC_0`"]
    #[inline]
    pub fn is_fastacc_0(&self) -> bool {
        *self == FASTACCR::FASTACC_0
    }
    #[doc = "Checks if the value of the field is `FASTACC_1`"]
    #[inline]
    pub fn is_fastacc_1(&self) -> bool {
        *self == FASTACCR::FASTACC_1
    }
}
#[doc = "Possible values of the field `DBGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGER {
    #[doc = "FlexIO is disabled in debug modes."]
    DBGE_0,
    #[doc = "FlexIO is enabled in debug modes"]
    DBGE_1,
}
impl DBGER {
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
            DBGER::DBGE_0 => false,
            DBGER::DBGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGER {
        match value {
            false => DBGER::DBGE_0,
            true => DBGER::DBGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGE_0`"]
    #[inline]
    pub fn is_dbge_0(&self) -> bool {
        *self == DBGER::DBGE_0
    }
    #[doc = "Checks if the value of the field is `DBGE_1`"]
    #[inline]
    pub fn is_dbge_1(&self) -> bool {
        *self == DBGER::DBGE_1
    }
}
#[doc = "Possible values of the field `DOZEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZENR {
    #[doc = "FlexIO enabled in Doze modes."]
    DOZEN_0,
    #[doc = "FlexIO disabled in Doze modes."]
    DOZEN_1,
}
impl DOZENR {
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
            DOZENR::DOZEN_0 => false,
            DOZENR::DOZEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZENR {
        match value {
            false => DOZENR::DOZEN_0,
            true => DOZENR::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline]
    pub fn is_dozen_0(&self) -> bool {
        *self == DOZENR::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline]
    pub fn is_dozen_1(&self) -> bool {
        *self == DOZENR::DOZEN_1
    }
}
#[doc = "Values that can be written to the field `FLEXEN`"]
pub enum FLEXENW {
    #[doc = "FlexIO module is disabled."]
    FLEXEN_0,
    #[doc = "FlexIO module is enabled."]
    FLEXEN_1,
}
impl FLEXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXENW::FLEXEN_0 => false,
            FLEXENW::FLEXEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO module is disabled."]
    #[inline]
    pub fn flexen_0(self) -> &'a mut W {
        self.variant(FLEXENW::FLEXEN_0)
    }
    #[doc = "FlexIO module is enabled."]
    #[inline]
    pub fn flexen_1(self) -> &'a mut W {
        self.variant(FLEXENW::FLEXEN_1)
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
#[doc = "Values that can be written to the field `SWRST`"]
pub enum SWRSTW {
    #[doc = "Software reset is disabled"]
    SWRST_0,
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    SWRST_1,
}
impl SWRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTW::SWRST_0 => false,
            SWRSTW::SWRST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software reset is disabled"]
    #[inline]
    pub fn swrst_0(self) -> &'a mut W {
        self.variant(SWRSTW::SWRST_0)
    }
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    #[inline]
    pub fn swrst_1(self) -> &'a mut W {
        self.variant(SWRSTW::SWRST_1)
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
#[doc = "Values that can be written to the field `FASTACC`"]
pub enum FASTACCW {
    #[doc = "Configures for normal register accesses to FlexIO"]
    FASTACC_0,
    #[doc = "Configures for fast register accesses to FlexIO"]
    FASTACC_1,
}
impl FASTACCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FASTACCW::FASTACC_0 => false,
            FASTACCW::FASTACC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FASTACCW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTACCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FASTACCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configures for normal register accesses to FlexIO"]
    #[inline]
    pub fn fastacc_0(self) -> &'a mut W {
        self.variant(FASTACCW::FASTACC_0)
    }
    #[doc = "Configures for fast register accesses to FlexIO"]
    #[inline]
    pub fn fastacc_1(self) -> &'a mut W {
        self.variant(FASTACCW::FASTACC_1)
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
#[doc = "Values that can be written to the field `DBGE`"]
pub enum DBGEW {
    #[doc = "FlexIO is disabled in debug modes."]
    DBGE_0,
    #[doc = "FlexIO is enabled in debug modes"]
    DBGE_1,
}
impl DBGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGEW::DBGE_0 => false,
            DBGEW::DBGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO is disabled in debug modes."]
    #[inline]
    pub fn dbge_0(self) -> &'a mut W {
        self.variant(DBGEW::DBGE_0)
    }
    #[doc = "FlexIO is enabled in debug modes"]
    #[inline]
    pub fn dbge_1(self) -> &'a mut W {
        self.variant(DBGEW::DBGE_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOZEN`"]
pub enum DOZENW {
    #[doc = "FlexIO enabled in Doze modes."]
    DOZEN_0,
    #[doc = "FlexIO disabled in Doze modes."]
    DOZEN_1,
}
impl DOZENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZENW::DOZEN_0 => false,
            DOZENW::DOZEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO enabled in Doze modes."]
    #[inline]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_0)
    }
    #[doc = "FlexIO disabled in Doze modes."]
    #[inline]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline]
    pub fn flexen(&self) -> FLEXENR {
        FLEXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        SWRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline]
    pub fn fastacc(&self) -> FASTACCR {
        FASTACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline]
    pub fn dbge(&self) -> DBGER {
        DBGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline]
    pub fn dozen(&self) -> DOZENR {
        DOZENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline]
    pub fn flexen(&mut self) -> _FLEXENW {
        _FLEXENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline]
    pub fn fastacc(&mut self) -> _FASTACCW {
        _FASTACCW { w: self }
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline]
    pub fn dbge(&mut self) -> _DBGEW {
        _DBGEW { w: self }
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline]
    pub fn dozen(&mut self) -> _DOZENW {
        _DOZENW { w: self }
    }
}
