#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IR {
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
pub struct OF1IER {
    bits: bool,
}
impl OF1IER {
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
pub struct OF2IER {
    bits: bool,
}
impl OF2IER {
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
#[doc = "Possible values of the field `OF3IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF3IER {
    #[doc = "Output Compare Channel n interrupt is disabled."]
    OF3IE_0,
    #[doc = "Output Compare Channel n interrupt is enabled."]
    OF3IE_1,
}
impl OF3IER {
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
            OF3IER::OF3IE_0 => false,
            OF3IER::OF3IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OF3IER {
        match value {
            false => OF3IER::OF3IE_0,
            true => OF3IER::OF3IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `OF3IE_0`"]
    #[inline]
    pub fn is_of3ie_0(&self) -> bool {
        *self == OF3IER::OF3IE_0
    }
    #[doc = "Checks if the value of the field is `OF3IE_1`"]
    #[inline]
    pub fn is_of3ie_1(&self) -> bool {
        *self == OF3IER::OF3IE_1
    }
}
#[doc = r" Value of the field"]
pub struct IF1IER {
    bits: bool,
}
impl IF1IER {
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
#[doc = "Possible values of the field `IF2IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF2IER {
    #[doc = "IF2IE Input Capture n Interrupt Enable is disabled."]
    IF2IE_0,
    #[doc = "IF2IE Input Capture n Interrupt Enable is enabled."]
    IF2IE_1,
}
impl IF2IER {
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
            IF2IER::IF2IE_0 => false,
            IF2IER::IF2IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IF2IER {
        match value {
            false => IF2IER::IF2IE_0,
            true => IF2IER::IF2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IF2IE_0`"]
    #[inline]
    pub fn is_if2ie_0(&self) -> bool {
        *self == IF2IER::IF2IE_0
    }
    #[doc = "Checks if the value of the field is `IF2IE_1`"]
    #[inline]
    pub fn is_if2ie_1(&self) -> bool {
        *self == IF2IER::IF2IE_1
    }
}
#[doc = "Possible values of the field `ROVIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVIER {
    #[doc = "Rollover interrupt is disabled."]
    ROVIE_0,
    #[doc = "Rollover interrupt enabled."]
    ROVIE_1,
}
impl ROVIER {
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
            ROVIER::ROVIE_0 => false,
            ROVIER::ROVIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROVIER {
        match value {
            false => ROVIER::ROVIE_0,
            true => ROVIER::ROVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROVIE_0`"]
    #[inline]
    pub fn is_rovie_0(&self) -> bool {
        *self == ROVIER::ROVIE_0
    }
    #[doc = "Checks if the value of the field is `ROVIE_1`"]
    #[inline]
    pub fn is_rovie_1(&self) -> bool {
        *self == ROVIER::ROVIE_1
    }
}
#[doc = r" Proxy"]
pub struct _OF1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _OF1IEW<'a> {
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
#[doc = r" Proxy"]
pub struct _OF2IEW<'a> {
    w: &'a mut W,
}
impl<'a> _OF2IEW<'a> {
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
#[doc = "Values that can be written to the field `OF3IE`"]
pub enum OF3IEW {
    #[doc = "Output Compare Channel n interrupt is disabled."]
    OF3IE_0,
    #[doc = "Output Compare Channel n interrupt is enabled."]
    OF3IE_1,
}
impl OF3IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OF3IEW::OF3IE_0 => false,
            OF3IEW::OF3IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OF3IEW<'a> {
    w: &'a mut W,
}
impl<'a> _OF3IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OF3IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output Compare Channel n interrupt is disabled."]
    #[inline]
    pub fn of3ie_0(self) -> &'a mut W {
        self.variant(OF3IEW::OF3IE_0)
    }
    #[doc = "Output Compare Channel n interrupt is enabled."]
    #[inline]
    pub fn of3ie_1(self) -> &'a mut W {
        self.variant(OF3IEW::OF3IE_1)
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
#[doc = r" Proxy"]
pub struct _IF1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _IF1IEW<'a> {
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
#[doc = "Values that can be written to the field `IF2IE`"]
pub enum IF2IEW {
    #[doc = "IF2IE Input Capture n Interrupt Enable is disabled."]
    IF2IE_0,
    #[doc = "IF2IE Input Capture n Interrupt Enable is enabled."]
    IF2IE_1,
}
impl IF2IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IF2IEW::IF2IE_0 => false,
            IF2IEW::IF2IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IF2IEW<'a> {
    w: &'a mut W,
}
impl<'a> _IF2IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IF2IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IF2IE Input Capture n Interrupt Enable is disabled."]
    #[inline]
    pub fn if2ie_0(self) -> &'a mut W {
        self.variant(IF2IEW::IF2IE_0)
    }
    #[doc = "IF2IE Input Capture n Interrupt Enable is enabled."]
    #[inline]
    pub fn if2ie_1(self) -> &'a mut W {
        self.variant(IF2IEW::IF2IE_1)
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
#[doc = "Values that can be written to the field `ROVIE`"]
pub enum ROVIEW {
    #[doc = "Rollover interrupt is disabled."]
    ROVIE_0,
    #[doc = "Rollover interrupt enabled."]
    ROVIE_1,
}
impl ROVIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROVIEW::ROVIE_0 => false,
            ROVIEW::ROVIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ROVIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROVIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rollover interrupt is disabled."]
    #[inline]
    pub fn rovie_0(self) -> &'a mut W {
        self.variant(ROVIEW::ROVIE_0)
    }
    #[doc = "Rollover interrupt enabled."]
    #[inline]
    pub fn rovie_1(self) -> &'a mut W {
        self.variant(ROVIEW::ROVIE_1)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - See OF3IE"]
    #[inline]
    pub fn of1ie(&self) -> OF1IER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OF1IER { bits }
    }
    #[doc = "Bit 1 - See OF3IE"]
    #[inline]
    pub fn of2ie(&self) -> OF2IER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OF2IER { bits }
    }
    #[doc = "Bit 2 - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline]
    pub fn of3ie(&self) -> OF3IER {
        OF3IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - See IF2IE"]
    #[inline]
    pub fn if1ie(&self) -> IF1IER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF1IER { bits }
    }
    #[doc = "Bit 4 - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline]
    pub fn if2ie(&self) -> IF2IER {
        IF2IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline]
    pub fn rovie(&self) -> ROVIER {
        ROVIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - See OF3IE"]
    #[inline]
    pub fn of1ie(&mut self) -> _OF1IEW {
        _OF1IEW { w: self }
    }
    #[doc = "Bit 1 - See OF3IE"]
    #[inline]
    pub fn of2ie(&mut self) -> _OF2IEW {
        _OF2IEW { w: self }
    }
    #[doc = "Bit 2 - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline]
    pub fn of3ie(&mut self) -> _OF3IEW {
        _OF3IEW { w: self }
    }
    #[doc = "Bit 3 - See IF2IE"]
    #[inline]
    pub fn if1ie(&mut self) -> _IF1IEW {
        _IF1IEW { w: self }
    }
    #[doc = "Bit 4 - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline]
    pub fn if2ie(&mut self) -> _IF2IEW {
        _IF2IEW { w: self }
    }
    #[doc = "Bit 5 - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline]
    pub fn rovie(&mut self) -> _ROVIEW {
        _ROVIEW { w: self }
    }
}
