#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TGSR {
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
#[doc = "Possible values of the field `TF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF0R {
    #[doc = "Timer Flag for Channel 0 is clear"]
    TF0_0,
    #[doc = "Timer Flag for Channel 0 is set"]
    TF0_1,
}
impl TF0R {
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
            TF0R::TF0_0 => false,
            TF0R::TF0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TF0R {
        match value {
            false => TF0R::TF0_0,
            true => TF0R::TF0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF0_0`"]
    #[inline]
    pub fn is_tf0_0(&self) -> bool {
        *self == TF0R::TF0_0
    }
    #[doc = "Checks if the value of the field is `TF0_1`"]
    #[inline]
    pub fn is_tf0_1(&self) -> bool {
        *self == TF0R::TF0_1
    }
}
#[doc = "Possible values of the field `TF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF1R {
    #[doc = "Timer Flag for Channel 1 is clear"]
    TF1_0,
    #[doc = "Timer Flag for Channel 1 is set"]
    TF1_1,
}
impl TF1R {
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
            TF1R::TF1_0 => false,
            TF1R::TF1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TF1R {
        match value {
            false => TF1R::TF1_0,
            true => TF1R::TF1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF1_0`"]
    #[inline]
    pub fn is_tf1_0(&self) -> bool {
        *self == TF1R::TF1_0
    }
    #[doc = "Checks if the value of the field is `TF1_1`"]
    #[inline]
    pub fn is_tf1_1(&self) -> bool {
        *self == TF1R::TF1_1
    }
}
#[doc = "Possible values of the field `TF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF2R {
    #[doc = "Timer Flag for Channel 2 is clear"]
    TF2_0,
    #[doc = "Timer Flag for Channel 2 is set"]
    TF2_1,
}
impl TF2R {
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
            TF2R::TF2_0 => false,
            TF2R::TF2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TF2R {
        match value {
            false => TF2R::TF2_0,
            true => TF2R::TF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF2_0`"]
    #[inline]
    pub fn is_tf2_0(&self) -> bool {
        *self == TF2R::TF2_0
    }
    #[doc = "Checks if the value of the field is `TF2_1`"]
    #[inline]
    pub fn is_tf2_1(&self) -> bool {
        *self == TF2R::TF2_1
    }
}
#[doc = "Possible values of the field `TF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF3R {
    #[doc = "Timer Flag for Channel 3 is clear"]
    TF3_0,
    #[doc = "Timer Flag for Channel 3 is set"]
    TF3_1,
}
impl TF3R {
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
            TF3R::TF3_0 => false,
            TF3R::TF3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TF3R {
        match value {
            false => TF3R::TF3_0,
            true => TF3R::TF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF3_0`"]
    #[inline]
    pub fn is_tf3_0(&self) -> bool {
        *self == TF3R::TF3_0
    }
    #[doc = "Checks if the value of the field is `TF3_1`"]
    #[inline]
    pub fn is_tf3_1(&self) -> bool {
        *self == TF3R::TF3_1
    }
}
#[doc = "Values that can be written to the field `TF0`"]
pub enum TF0W {
    #[doc = "Timer Flag for Channel 0 is clear"]
    TF0_0,
    #[doc = "Timer Flag for Channel 0 is set"]
    TF0_1,
}
impl TF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TF0W::TF0_0 => false,
            TF0W::TF0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TF0W<'a> {
    w: &'a mut W,
}
impl<'a> _TF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer Flag for Channel 0 is clear"]
    #[inline]
    pub fn tf0_0(self) -> &'a mut W {
        self.variant(TF0W::TF0_0)
    }
    #[doc = "Timer Flag for Channel 0 is set"]
    #[inline]
    pub fn tf0_1(self) -> &'a mut W {
        self.variant(TF0W::TF0_1)
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
#[doc = "Values that can be written to the field `TF1`"]
pub enum TF1W {
    #[doc = "Timer Flag for Channel 1 is clear"]
    TF1_0,
    #[doc = "Timer Flag for Channel 1 is set"]
    TF1_1,
}
impl TF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TF1W::TF1_0 => false,
            TF1W::TF1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TF1W<'a> {
    w: &'a mut W,
}
impl<'a> _TF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer Flag for Channel 1 is clear"]
    #[inline]
    pub fn tf1_0(self) -> &'a mut W {
        self.variant(TF1W::TF1_0)
    }
    #[doc = "Timer Flag for Channel 1 is set"]
    #[inline]
    pub fn tf1_1(self) -> &'a mut W {
        self.variant(TF1W::TF1_1)
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
#[doc = "Values that can be written to the field `TF2`"]
pub enum TF2W {
    #[doc = "Timer Flag for Channel 2 is clear"]
    TF2_0,
    #[doc = "Timer Flag for Channel 2 is set"]
    TF2_1,
}
impl TF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TF2W::TF2_0 => false,
            TF2W::TF2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TF2W<'a> {
    w: &'a mut W,
}
impl<'a> _TF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer Flag for Channel 2 is clear"]
    #[inline]
    pub fn tf2_0(self) -> &'a mut W {
        self.variant(TF2W::TF2_0)
    }
    #[doc = "Timer Flag for Channel 2 is set"]
    #[inline]
    pub fn tf2_1(self) -> &'a mut W {
        self.variant(TF2W::TF2_1)
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
#[doc = "Values that can be written to the field `TF3`"]
pub enum TF3W {
    #[doc = "Timer Flag for Channel 3 is clear"]
    TF3_0,
    #[doc = "Timer Flag for Channel 3 is set"]
    TF3_1,
}
impl TF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TF3W::TF3_0 => false,
            TF3W::TF3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TF3W<'a> {
    w: &'a mut W,
}
impl<'a> _TF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer Flag for Channel 3 is clear"]
    #[inline]
    pub fn tf3_0(self) -> &'a mut W {
        self.variant(TF3W::TF3_0)
    }
    #[doc = "Timer Flag for Channel 3 is set"]
    #[inline]
    pub fn tf3_1(self) -> &'a mut W {
        self.variant(TF3W::TF3_1)
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
    #[doc = "Bit 0 - Copy Of Timer Flag For Channel 0"]
    #[inline]
    pub fn tf0(&self) -> TF0R {
        TF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Copy Of Timer Flag For Channel 1"]
    #[inline]
    pub fn tf1(&self) -> TF1R {
        TF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Copy Of Timer Flag For Channel 2"]
    #[inline]
    pub fn tf2(&self) -> TF2R {
        TF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Copy Of Timer Flag For Channel 3"]
    #[inline]
    pub fn tf3(&self) -> TF3R {
        TF3R::_from({
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
    #[doc = "Bit 0 - Copy Of Timer Flag For Channel 0"]
    #[inline]
    pub fn tf0(&mut self) -> _TF0W {
        _TF0W { w: self }
    }
    #[doc = "Bit 1 - Copy Of Timer Flag For Channel 1"]
    #[inline]
    pub fn tf1(&mut self) -> _TF1W {
        _TF1W { w: self }
    }
    #[doc = "Bit 2 - Copy Of Timer Flag For Channel 2"]
    #[inline]
    pub fn tf2(&mut self) -> _TF2W {
        _TF2W { w: self }
    }
    #[doc = "Bit 3 - Copy Of Timer Flag For Channel 3"]
    #[inline]
    pub fn tf3(&mut self) -> _TF3W {
        _TF3W { w: self }
    }
}
