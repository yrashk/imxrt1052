#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
pub struct OF1R {
    bits: bool,
}
impl OF1R {
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
pub struct OF2R {
    bits: bool,
}
impl OF2R {
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
#[doc = "Possible values of the field `OF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF3R {
    #[doc = "Compare event has not occurred."]
    OF3_0,
    #[doc = "Compare event has occurred."]
    OF3_1,
}
impl OF3R {
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
            OF3R::OF3_0 => false,
            OF3R::OF3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OF3R {
        match value {
            false => OF3R::OF3_0,
            true => OF3R::OF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `OF3_0`"]
    #[inline]
    pub fn is_of3_0(&self) -> bool {
        *self == OF3R::OF3_0
    }
    #[doc = "Checks if the value of the field is `OF3_1`"]
    #[inline]
    pub fn is_of3_1(&self) -> bool {
        *self == OF3R::OF3_1
    }
}
#[doc = r" Value of the field"]
pub struct IF1R {
    bits: bool,
}
impl IF1R {
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
#[doc = "Possible values of the field `IF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF2R {
    #[doc = "Capture event has not occurred."]
    IF2_0,
    #[doc = "Capture event has occurred."]
    IF2_1,
}
impl IF2R {
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
            IF2R::IF2_0 => false,
            IF2R::IF2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IF2R {
        match value {
            false => IF2R::IF2_0,
            true => IF2R::IF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IF2_0`"]
    #[inline]
    pub fn is_if2_0(&self) -> bool {
        *self == IF2R::IF2_0
    }
    #[doc = "Checks if the value of the field is `IF2_1`"]
    #[inline]
    pub fn is_if2_1(&self) -> bool {
        *self == IF2R::IF2_1
    }
}
#[doc = "Possible values of the field `ROV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVR {
    #[doc = "Rollover has not occurred."]
    ROV_0,
    #[doc = "Rollover has occurred."]
    ROV_1,
}
impl ROVR {
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
            ROVR::ROV_0 => false,
            ROVR::ROV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROVR {
        match value {
            false => ROVR::ROV_0,
            true => ROVR::ROV_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROV_0`"]
    #[inline]
    pub fn is_rov_0(&self) -> bool {
        *self == ROVR::ROV_0
    }
    #[doc = "Checks if the value of the field is `ROV_1`"]
    #[inline]
    pub fn is_rov_1(&self) -> bool {
        *self == ROVR::ROV_1
    }
}
#[doc = r" Proxy"]
pub struct _OF1W<'a> {
    w: &'a mut W,
}
impl<'a> _OF1W<'a> {
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
pub struct _OF2W<'a> {
    w: &'a mut W,
}
impl<'a> _OF2W<'a> {
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
#[doc = "Values that can be written to the field `OF3`"]
pub enum OF3W {
    #[doc = "Compare event has not occurred."]
    OF3_0,
    #[doc = "Compare event has occurred."]
    OF3_1,
}
impl OF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OF3W::OF3_0 => false,
            OF3W::OF3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OF3W<'a> {
    w: &'a mut W,
}
impl<'a> _OF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare event has not occurred."]
    #[inline]
    pub fn of3_0(self) -> &'a mut W {
        self.variant(OF3W::OF3_0)
    }
    #[doc = "Compare event has occurred."]
    #[inline]
    pub fn of3_1(self) -> &'a mut W {
        self.variant(OF3W::OF3_1)
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
pub struct _IF1W<'a> {
    w: &'a mut W,
}
impl<'a> _IF1W<'a> {
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
#[doc = "Values that can be written to the field `IF2`"]
pub enum IF2W {
    #[doc = "Capture event has not occurred."]
    IF2_0,
    #[doc = "Capture event has occurred."]
    IF2_1,
}
impl IF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IF2W::IF2_0 => false,
            IF2W::IF2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IF2W<'a> {
    w: &'a mut W,
}
impl<'a> _IF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture event has not occurred."]
    #[inline]
    pub fn if2_0(self) -> &'a mut W {
        self.variant(IF2W::IF2_0)
    }
    #[doc = "Capture event has occurred."]
    #[inline]
    pub fn if2_1(self) -> &'a mut W {
        self.variant(IF2W::IF2_1)
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
#[doc = "Values that can be written to the field `ROV`"]
pub enum ROVW {
    #[doc = "Rollover has not occurred."]
    ROV_0,
    #[doc = "Rollover has occurred."]
    ROV_1,
}
impl ROVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROVW::ROV_0 => false,
            ROVW::ROV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROVW<'a> {
    w: &'a mut W,
}
impl<'a> _ROVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rollover has not occurred."]
    #[inline]
    pub fn rov_0(self) -> &'a mut W {
        self.variant(ROVW::ROV_0)
    }
    #[doc = "Rollover has occurred."]
    #[inline]
    pub fn rov_1(self) -> &'a mut W {
        self.variant(ROVW::ROV_1)
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
    #[doc = "Bit 0 - See OF3"]
    #[inline]
    pub fn of1(&self) -> OF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OF1R { bits }
    }
    #[doc = "Bit 1 - See OF3"]
    #[inline]
    pub fn of2(&self) -> OF2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OF2R { bits }
    }
    #[doc = "Bit 2 - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline]
    pub fn of3(&self) -> OF3R {
        OF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - See IF2"]
    #[inline]
    pub fn if1(&self) -> IF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF1R { bits }
    }
    #[doc = "Bit 4 - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline]
    pub fn if2(&self) -> IF2R {
        IF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Rollover Flag"]
    #[inline]
    pub fn rov(&self) -> ROVR {
        ROVR::_from({
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
    #[doc = "Bit 0 - See OF3"]
    #[inline]
    pub fn of1(&mut self) -> _OF1W {
        _OF1W { w: self }
    }
    #[doc = "Bit 1 - See OF3"]
    #[inline]
    pub fn of2(&mut self) -> _OF2W {
        _OF2W { w: self }
    }
    #[doc = "Bit 2 - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline]
    pub fn of3(&mut self) -> _OF3W {
        _OF3W { w: self }
    }
    #[doc = "Bit 3 - See IF2"]
    #[inline]
    pub fn if1(&mut self) -> _IF1W {
        _IF1W { w: self }
    }
    #[doc = "Bit 4 - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline]
    pub fn if2(&mut self) -> _IF2W {
        _IF2W { w: self }
    }
    #[doc = "Bit 5 - Rollover Flag"]
    #[inline]
    pub fn rov(&mut self) -> _ROVW {
        _ROVW { w: self }
    }
}
