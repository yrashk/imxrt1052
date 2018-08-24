#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCM_CTRL {
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
#[doc = "Possible values of the field `TCM_WWAIT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCM_WWAIT_ENR {
    #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    TCM_WWAIT_EN_0,
    #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    TCM_WWAIT_EN_1,
}
impl TCM_WWAIT_ENR {
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
            TCM_WWAIT_ENR::TCM_WWAIT_EN_0 => false,
            TCM_WWAIT_ENR::TCM_WWAIT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCM_WWAIT_ENR {
        match value {
            false => TCM_WWAIT_ENR::TCM_WWAIT_EN_0,
            true => TCM_WWAIT_ENR::TCM_WWAIT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCM_WWAIT_EN_0`"]
    #[inline]
    pub fn is_tcm_wwait_en_0(&self) -> bool {
        *self == TCM_WWAIT_ENR::TCM_WWAIT_EN_0
    }
    #[doc = "Checks if the value of the field is `TCM_WWAIT_EN_1`"]
    #[inline]
    pub fn is_tcm_wwait_en_1(&self) -> bool {
        *self == TCM_WWAIT_ENR::TCM_WWAIT_EN_1
    }
}
#[doc = "Possible values of the field `TCM_RWAIT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCM_RWAIT_ENR {
    #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    TCM_RWAIT_EN_0,
    #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    TCM_RWAIT_EN_1,
}
impl TCM_RWAIT_ENR {
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
            TCM_RWAIT_ENR::TCM_RWAIT_EN_0 => false,
            TCM_RWAIT_ENR::TCM_RWAIT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCM_RWAIT_ENR {
        match value {
            false => TCM_RWAIT_ENR::TCM_RWAIT_EN_0,
            true => TCM_RWAIT_ENR::TCM_RWAIT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCM_RWAIT_EN_0`"]
    #[inline]
    pub fn is_tcm_rwait_en_0(&self) -> bool {
        *self == TCM_RWAIT_ENR::TCM_RWAIT_EN_0
    }
    #[doc = "Checks if the value of the field is `TCM_RWAIT_EN_1`"]
    #[inline]
    pub fn is_tcm_rwait_en_1(&self) -> bool {
        *self == TCM_RWAIT_ENR::TCM_RWAIT_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct FORCE_CLK_ONR {
    bits: bool,
}
impl FORCE_CLK_ONR {
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
#[doc = "Values that can be written to the field `TCM_WWAIT_EN`"]
pub enum TCM_WWAIT_ENW {
    #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    TCM_WWAIT_EN_0,
    #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    TCM_WWAIT_EN_1,
}
impl TCM_WWAIT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCM_WWAIT_ENW::TCM_WWAIT_EN_0 => false,
            TCM_WWAIT_ENW::TCM_WWAIT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCM_WWAIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCM_WWAIT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCM_WWAIT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    #[inline]
    pub fn tcm_wwait_en_0(self) -> &'a mut W {
        self.variant(TCM_WWAIT_ENW::TCM_WWAIT_EN_0)
    }
    #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    #[inline]
    pub fn tcm_wwait_en_1(self) -> &'a mut W {
        self.variant(TCM_WWAIT_ENW::TCM_WWAIT_EN_1)
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
#[doc = "Values that can be written to the field `TCM_RWAIT_EN`"]
pub enum TCM_RWAIT_ENW {
    #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    TCM_RWAIT_EN_0,
    #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    TCM_RWAIT_EN_1,
}
impl TCM_RWAIT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCM_RWAIT_ENW::TCM_RWAIT_EN_0 => false,
            TCM_RWAIT_ENW::TCM_RWAIT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCM_RWAIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCM_RWAIT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCM_RWAIT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    #[inline]
    pub fn tcm_rwait_en_0(self) -> &'a mut W {
        self.variant(TCM_RWAIT_ENW::TCM_RWAIT_EN_0)
    }
    #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    #[inline]
    pub fn tcm_rwait_en_1(self) -> &'a mut W {
        self.variant(TCM_RWAIT_ENW::TCM_RWAIT_EN_1)
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
#[doc = r" Proxy"]
pub struct _FORCE_CLK_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_CLK_ONW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TCM Write Wait Mode Enable"]
    #[inline]
    pub fn tcm_wwait_en(&self) -> TCM_WWAIT_ENR {
        TCM_WWAIT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TCM Read Wait Mode Enable"]
    #[inline]
    pub fn tcm_rwait_en(&self) -> TCM_RWAIT_ENR {
        TCM_RWAIT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Force RAM Clock Always On"]
    #[inline]
    pub fn force_clk_on(&self) -> FORCE_CLK_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FORCE_CLK_ONR { bits }
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
    #[doc = "Bit 0 - TCM Write Wait Mode Enable"]
    #[inline]
    pub fn tcm_wwait_en(&mut self) -> _TCM_WWAIT_ENW {
        _TCM_WWAIT_ENW { w: self }
    }
    #[doc = "Bit 1 - TCM Read Wait Mode Enable"]
    #[inline]
    pub fn tcm_rwait_en(&mut self) -> _TCM_RWAIT_ENW {
        _TCM_RWAIT_ENW { w: self }
    }
    #[doc = "Bit 2 - Force RAM Clock Always On"]
    #[inline]
    pub fn force_clk_on(&mut self) -> _FORCE_CLK_ONW {
        _FORCE_CLK_ONW { w: self }
    }
}
