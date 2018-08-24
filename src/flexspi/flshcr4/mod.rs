#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLSHCR4 {
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
#[doc = "Possible values of the field `WMOPT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMOPT1R {
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_0,
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_1,
}
impl WMOPT1R {
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
            WMOPT1R::WMOPT1_0 => false,
            WMOPT1R::WMOPT1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WMOPT1R {
        match value {
            false => WMOPT1R::WMOPT1_0,
            true => WMOPT1R::WMOPT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMOPT1_0`"]
    #[inline]
    pub fn is_wmopt1_0(&self) -> bool {
        *self == WMOPT1R::WMOPT1_0
    }
    #[doc = "Checks if the value of the field is `WMOPT1_1`"]
    #[inline]
    pub fn is_wmopt1_1(&self) -> bool {
        *self == WMOPT1R::WMOPT1_1
    }
}
#[doc = "Possible values of the field `WMENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMENAR {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENA_0,
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENA_1,
}
impl WMENAR {
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
            WMENAR::WMENA_0 => false,
            WMENAR::WMENA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WMENAR {
        match value {
            false => WMENAR::WMENA_0,
            true => WMENAR::WMENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMENA_0`"]
    #[inline]
    pub fn is_wmena_0(&self) -> bool {
        *self == WMENAR::WMENA_0
    }
    #[doc = "Checks if the value of the field is `WMENA_1`"]
    #[inline]
    pub fn is_wmena_1(&self) -> bool {
        *self == WMENAR::WMENA_1
    }
}
#[doc = "Possible values of the field `WMENB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMENBR {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENB_0,
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENB_1,
}
impl WMENBR {
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
            WMENBR::WMENB_0 => false,
            WMENBR::WMENB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WMENBR {
        match value {
            false => WMENBR::WMENB_0,
            true => WMENBR::WMENB_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMENB_0`"]
    #[inline]
    pub fn is_wmenb_0(&self) -> bool {
        *self == WMENBR::WMENB_0
    }
    #[doc = "Checks if the value of the field is `WMENB_1`"]
    #[inline]
    pub fn is_wmenb_1(&self) -> bool {
        *self == WMENBR::WMENB_1
    }
}
#[doc = "Values that can be written to the field `WMOPT1`"]
pub enum WMOPT1W {
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_0,
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_1,
}
impl WMOPT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WMOPT1W::WMOPT1_0 => false,
            WMOPT1W::WMOPT1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WMOPT1W<'a> {
    w: &'a mut W,
}
impl<'a> _WMOPT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WMOPT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline]
    pub fn wmopt1_0(self) -> &'a mut W {
        self.variant(WMOPT1W::WMOPT1_0)
    }
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline]
    pub fn wmopt1_1(self) -> &'a mut W {
        self.variant(WMOPT1W::WMOPT1_1)
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
#[doc = "Values that can be written to the field `WMENA`"]
pub enum WMENAW {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENA_0,
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENA_1,
}
impl WMENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WMENAW::WMENA_0 => false,
            WMENAW::WMENA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WMENAW<'a> {
    w: &'a mut W,
}
impl<'a> _WMENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WMENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline]
    pub fn wmena_0(self) -> &'a mut W {
        self.variant(WMENAW::WMENA_0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline]
    pub fn wmena_1(self) -> &'a mut W {
        self.variant(WMENAW::WMENA_1)
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
#[doc = "Values that can be written to the field `WMENB`"]
pub enum WMENBW {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENB_0,
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENB_1,
}
impl WMENBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WMENBW::WMENB_0 => false,
            WMENBW::WMENB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WMENBW<'a> {
    w: &'a mut W,
}
impl<'a> _WMENBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WMENBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline]
    pub fn wmenb_0(self) -> &'a mut W {
        self.variant(WMENBW::WMENB_0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline]
    pub fn wmenb_1(self) -> &'a mut W {
        self.variant(WMENBW::WMENB_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline]
    pub fn wmopt1(&self) -> WMOPT1R {
        WMOPT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline]
    pub fn wmena(&self) -> WMENAR {
        WMENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline]
    pub fn wmenb(&self) -> WMENBR {
        WMENBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline]
    pub fn wmopt1(&mut self) -> _WMOPT1W {
        _WMOPT1W { w: self }
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline]
    pub fn wmena(&mut self) -> _WMENAW {
        _WMENAW { w: self }
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline]
    pub fn wmenb(&mut self) -> _WMENBW {
        _WMENBW { w: self }
    }
}
