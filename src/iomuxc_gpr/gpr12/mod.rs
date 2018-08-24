#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR12 {
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
#[doc = "Possible values of the field `FLEXIO1_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_IPG_STOP_MODER {
    #[doc = "FlexIO1 is functional in Stop mode."]
    FLEXIO1_IPG_STOP_MODE_0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
    FLEXIO1_IPG_STOP_MODE_1,
}
impl FLEXIO1_IPG_STOP_MODER {
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
            FLEXIO1_IPG_STOP_MODER::FLEXIO1_IPG_STOP_MODE_0 => false,
            FLEXIO1_IPG_STOP_MODER::FLEXIO1_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO1_IPG_STOP_MODER {
        match value {
            false => FLEXIO1_IPG_STOP_MODER::FLEXIO1_IPG_STOP_MODE_0,
            true => FLEXIO1_IPG_STOP_MODER::FLEXIO1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_flexio1_ipg_stop_mode_0(&self) -> bool {
        *self == FLEXIO1_IPG_STOP_MODER::FLEXIO1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_flexio1_ipg_stop_mode_1(&self) -> bool {
        *self == FLEXIO1_IPG_STOP_MODER::FLEXIO1_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `FLEXIO1_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_IPG_DOZER {
    #[doc = "FLEXIO1 is not in doze mode"]
    FLEXIO1_IPG_DOZE_0,
    #[doc = "FLEXIO1 is in doze mode"]
    FLEXIO1_IPG_DOZE_1,
}
impl FLEXIO1_IPG_DOZER {
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
            FLEXIO1_IPG_DOZER::FLEXIO1_IPG_DOZE_0 => false,
            FLEXIO1_IPG_DOZER::FLEXIO1_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO1_IPG_DOZER {
        match value {
            false => FLEXIO1_IPG_DOZER::FLEXIO1_IPG_DOZE_0,
            true => FLEXIO1_IPG_DOZER::FLEXIO1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_DOZE_0`"]
    #[inline]
    pub fn is_flexio1_ipg_doze_0(&self) -> bool {
        *self == FLEXIO1_IPG_DOZER::FLEXIO1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_DOZE_1`"]
    #[inline]
    pub fn is_flexio1_ipg_doze_1(&self) -> bool {
        *self == FLEXIO1_IPG_DOZER::FLEXIO1_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `FLEXIO2_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_IPG_STOP_MODER {
    #[doc = "FlexIO2 is functional in Stop mode."]
    FLEXIO2_IPG_STOP_MODE_0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO2 is not functional in Stop mode."]
    FLEXIO2_IPG_STOP_MODE_1,
}
impl FLEXIO2_IPG_STOP_MODER {
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
            FLEXIO2_IPG_STOP_MODER::FLEXIO2_IPG_STOP_MODE_0 => false,
            FLEXIO2_IPG_STOP_MODER::FLEXIO2_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO2_IPG_STOP_MODER {
        match value {
            false => FLEXIO2_IPG_STOP_MODER::FLEXIO2_IPG_STOP_MODE_0,
            true => FLEXIO2_IPG_STOP_MODER::FLEXIO2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_flexio2_ipg_stop_mode_0(&self) -> bool {
        *self == FLEXIO2_IPG_STOP_MODER::FLEXIO2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_flexio2_ipg_stop_mode_1(&self) -> bool {
        *self == FLEXIO2_IPG_STOP_MODER::FLEXIO2_IPG_STOP_MODE_1
    }
}
#[doc = "Possible values of the field `FLEXIO2_IPG_DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_IPG_DOZER {
    #[doc = "FLEXIO2 is not in doze mode"]
    FLEXIO2_IPG_DOZE_0,
    #[doc = "FLEXIO2 is in doze mode"]
    FLEXIO2_IPG_DOZE_1,
}
impl FLEXIO2_IPG_DOZER {
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
            FLEXIO2_IPG_DOZER::FLEXIO2_IPG_DOZE_0 => false,
            FLEXIO2_IPG_DOZER::FLEXIO2_IPG_DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO2_IPG_DOZER {
        match value {
            false => FLEXIO2_IPG_DOZER::FLEXIO2_IPG_DOZE_0,
            true => FLEXIO2_IPG_DOZER::FLEXIO2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_DOZE_0`"]
    #[inline]
    pub fn is_flexio2_ipg_doze_0(&self) -> bool {
        *self == FLEXIO2_IPG_DOZER::FLEXIO2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_DOZE_1`"]
    #[inline]
    pub fn is_flexio2_ipg_doze_1(&self) -> bool {
        *self == FLEXIO2_IPG_DOZER::FLEXIO2_IPG_DOZE_1
    }
}
#[doc = "Possible values of the field `ACMP_IPG_STOP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_IPG_STOP_MODER {
    #[doc = "ACMP is functional in Stop mode."]
    ACMP_IPG_STOP_MODE_0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode."]
    ACMP_IPG_STOP_MODE_1,
}
impl ACMP_IPG_STOP_MODER {
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
            ACMP_IPG_STOP_MODER::ACMP_IPG_STOP_MODE_0 => false,
            ACMP_IPG_STOP_MODER::ACMP_IPG_STOP_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_IPG_STOP_MODER {
        match value {
            false => ACMP_IPG_STOP_MODER::ACMP_IPG_STOP_MODE_0,
            true => ACMP_IPG_STOP_MODER::ACMP_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_IPG_STOP_MODE_0`"]
    #[inline]
    pub fn is_acmp_ipg_stop_mode_0(&self) -> bool {
        *self == ACMP_IPG_STOP_MODER::ACMP_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `ACMP_IPG_STOP_MODE_1`"]
    #[inline]
    pub fn is_acmp_ipg_stop_mode_1(&self) -> bool {
        *self == ACMP_IPG_STOP_MODER::ACMP_IPG_STOP_MODE_1
    }
}
#[doc = "Values that can be written to the field `FLEXIO1_IPG_STOP_MODE`"]
pub enum FLEXIO1_IPG_STOP_MODEW {
    #[doc = "FlexIO1 is functional in Stop mode."]
    FLEXIO1_IPG_STOP_MODE_0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
    FLEXIO1_IPG_STOP_MODE_1,
}
impl FLEXIO1_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXIO1_IPG_STOP_MODEW::FLEXIO1_IPG_STOP_MODE_0 => false,
            FLEXIO1_IPG_STOP_MODEW::FLEXIO1_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO1_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO1_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO1_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO1 is functional in Stop mode."]
    #[inline]
    pub fn flexio1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_STOP_MODEW::FLEXIO1_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
    #[inline]
    pub fn flexio1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_STOP_MODEW::FLEXIO1_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `FLEXIO1_IPG_DOZE`"]
pub enum FLEXIO1_IPG_DOZEW {
    #[doc = "FLEXIO1 is not in doze mode"]
    FLEXIO1_IPG_DOZE_0,
    #[doc = "FLEXIO1 is in doze mode"]
    FLEXIO1_IPG_DOZE_1,
}
impl FLEXIO1_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXIO1_IPG_DOZEW::FLEXIO1_IPG_DOZE_0 => false,
            FLEXIO1_IPG_DOZEW::FLEXIO1_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO1_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO1_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO1_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLEXIO1 is not in doze mode"]
    #[inline]
    pub fn flexio1_ipg_doze_0(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_DOZEW::FLEXIO1_IPG_DOZE_0)
    }
    #[doc = "FLEXIO1 is in doze mode"]
    #[inline]
    pub fn flexio1_ipg_doze_1(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_DOZEW::FLEXIO1_IPG_DOZE_1)
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
#[doc = "Values that can be written to the field `FLEXIO2_IPG_STOP_MODE`"]
pub enum FLEXIO2_IPG_STOP_MODEW {
    #[doc = "FlexIO2 is functional in Stop mode."]
    FLEXIO2_IPG_STOP_MODE_0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO2 is not functional in Stop mode."]
    FLEXIO2_IPG_STOP_MODE_1,
}
impl FLEXIO2_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXIO2_IPG_STOP_MODEW::FLEXIO2_IPG_STOP_MODE_0 => false,
            FLEXIO2_IPG_STOP_MODEW::FLEXIO2_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO2_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO2_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO2_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO2 is functional in Stop mode."]
    #[inline]
    pub fn flexio2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_STOP_MODEW::FLEXIO2_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO2 is not functional in Stop mode."]
    #[inline]
    pub fn flexio2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_STOP_MODEW::FLEXIO2_IPG_STOP_MODE_1)
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
#[doc = "Values that can be written to the field `FLEXIO2_IPG_DOZE`"]
pub enum FLEXIO2_IPG_DOZEW {
    #[doc = "FLEXIO2 is not in doze mode"]
    FLEXIO2_IPG_DOZE_0,
    #[doc = "FLEXIO2 is in doze mode"]
    FLEXIO2_IPG_DOZE_1,
}
impl FLEXIO2_IPG_DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXIO2_IPG_DOZEW::FLEXIO2_IPG_DOZE_0 => false,
            FLEXIO2_IPG_DOZEW::FLEXIO2_IPG_DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO2_IPG_DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO2_IPG_DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO2_IPG_DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLEXIO2 is not in doze mode"]
    #[inline]
    pub fn flexio2_ipg_doze_0(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_DOZEW::FLEXIO2_IPG_DOZE_0)
    }
    #[doc = "FLEXIO2 is in doze mode"]
    #[inline]
    pub fn flexio2_ipg_doze_1(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_DOZEW::FLEXIO2_IPG_DOZE_1)
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
#[doc = "Values that can be written to the field `ACMP_IPG_STOP_MODE`"]
pub enum ACMP_IPG_STOP_MODEW {
    #[doc = "ACMP is functional in Stop mode."]
    ACMP_IPG_STOP_MODE_0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode."]
    ACMP_IPG_STOP_MODE_1,
}
impl ACMP_IPG_STOP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_IPG_STOP_MODEW::ACMP_IPG_STOP_MODE_0 => false,
            ACMP_IPG_STOP_MODEW::ACMP_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_IPG_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_IPG_STOP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_IPG_STOP_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACMP is functional in Stop mode."]
    #[inline]
    pub fn acmp_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(ACMP_IPG_STOP_MODEW::ACMP_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode."]
    #[inline]
    pub fn acmp_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(ACMP_IPG_STOP_MODEW::ACMP_IPG_STOP_MODE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn flexio1_ipg_stop_mode(&self) -> FLEXIO1_IPG_STOP_MODER {
        FLEXIO1_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - FLEXIO1 ipg_doze mode"]
    #[inline]
    pub fn flexio1_ipg_doze(&self) -> FLEXIO1_IPG_DOZER {
        FLEXIO1_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn flexio2_ipg_stop_mode(&self) -> FLEXIO2_IPG_STOP_MODER {
        FLEXIO2_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FLEXIO2 ipg_doze mode"]
    #[inline]
    pub fn flexio2_ipg_doze(&self) -> FLEXIO2_IPG_DOZER {
        FLEXIO2_IPG_DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn acmp_ipg_stop_mode(&self) -> ACMP_IPG_STOP_MODER {
        ACMP_IPG_STOP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn flexio1_ipg_stop_mode(&mut self) -> _FLEXIO1_IPG_STOP_MODEW {
        _FLEXIO1_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 1 - FLEXIO1 ipg_doze mode"]
    #[inline]
    pub fn flexio1_ipg_doze(&mut self) -> _FLEXIO1_IPG_DOZEW {
        _FLEXIO1_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 2 - FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn flexio2_ipg_stop_mode(&mut self) -> _FLEXIO2_IPG_STOP_MODEW {
        _FLEXIO2_IPG_STOP_MODEW { w: self }
    }
    #[doc = "Bit 3 - FLEXIO2 ipg_doze mode"]
    #[inline]
    pub fn flexio2_ipg_doze(&mut self) -> _FLEXIO2_IPG_DOZEW {
        _FLEXIO2_IPG_DOZEW { w: self }
    }
    #[doc = "Bit 4 - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline]
    pub fn acmp_ipg_stop_mode(&mut self) -> _ACMP_IPG_STOP_MODEW {
        _ACMP_IPG_STOP_MODEW { w: self }
    }
}
