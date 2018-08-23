#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SWCOUT {
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
#[doc = "Possible values of the field `SM0OUT45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM0OUT45R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SM0OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SM0OUT45_1,
}
impl SM0OUT45R {
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
            SM0OUT45R::SM0OUT45_0 => false,
            SM0OUT45R::SM0OUT45_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM0OUT45R {
        match value {
            false => SM0OUT45R::SM0OUT45_0,
            true => SM0OUT45R::SM0OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM0OUT45_0`"]
    #[inline]
    pub fn is_sm0out45_0(&self) -> bool {
        *self == SM0OUT45R::SM0OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM0OUT45_1`"]
    #[inline]
    pub fn is_sm0out45_1(&self) -> bool {
        *self == SM0OUT45R::SM0OUT45_1
    }
}
#[doc = "Possible values of the field `SM0OUT23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM0OUT23R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SM0OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SM0OUT23_1,
}
impl SM0OUT23R {
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
            SM0OUT23R::SM0OUT23_0 => false,
            SM0OUT23R::SM0OUT23_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM0OUT23R {
        match value {
            false => SM0OUT23R::SM0OUT23_0,
            true => SM0OUT23R::SM0OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM0OUT23_0`"]
    #[inline]
    pub fn is_sm0out23_0(&self) -> bool {
        *self == SM0OUT23R::SM0OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM0OUT23_1`"]
    #[inline]
    pub fn is_sm0out23_1(&self) -> bool {
        *self == SM0OUT23R::SM0OUT23_1
    }
}
#[doc = "Possible values of the field `SM1OUT45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM1OUT45R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    SM1OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    SM1OUT45_1,
}
impl SM1OUT45R {
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
            SM1OUT45R::SM1OUT45_0 => false,
            SM1OUT45R::SM1OUT45_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM1OUT45R {
        match value {
            false => SM1OUT45R::SM1OUT45_0,
            true => SM1OUT45R::SM1OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM1OUT45_0`"]
    #[inline]
    pub fn is_sm1out45_0(&self) -> bool {
        *self == SM1OUT45R::SM1OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM1OUT45_1`"]
    #[inline]
    pub fn is_sm1out45_1(&self) -> bool {
        *self == SM1OUT45R::SM1OUT45_1
    }
}
#[doc = "Possible values of the field `SM1OUT23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM1OUT23R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    SM1OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    SM1OUT23_1,
}
impl SM1OUT23R {
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
            SM1OUT23R::SM1OUT23_0 => false,
            SM1OUT23R::SM1OUT23_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM1OUT23R {
        match value {
            false => SM1OUT23R::SM1OUT23_0,
            true => SM1OUT23R::SM1OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM1OUT23_0`"]
    #[inline]
    pub fn is_sm1out23_0(&self) -> bool {
        *self == SM1OUT23R::SM1OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM1OUT23_1`"]
    #[inline]
    pub fn is_sm1out23_1(&self) -> bool {
        *self == SM1OUT23R::SM1OUT23_1
    }
}
#[doc = "Possible values of the field `SM2OUT45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM2OUT45R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    SM2OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    SM2OUT45_1,
}
impl SM2OUT45R {
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
            SM2OUT45R::SM2OUT45_0 => false,
            SM2OUT45R::SM2OUT45_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM2OUT45R {
        match value {
            false => SM2OUT45R::SM2OUT45_0,
            true => SM2OUT45R::SM2OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM2OUT45_0`"]
    #[inline]
    pub fn is_sm2out45_0(&self) -> bool {
        *self == SM2OUT45R::SM2OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM2OUT45_1`"]
    #[inline]
    pub fn is_sm2out45_1(&self) -> bool {
        *self == SM2OUT45R::SM2OUT45_1
    }
}
#[doc = "Possible values of the field `SM2OUT23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM2OUT23R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    SM2OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    SM2OUT23_1,
}
impl SM2OUT23R {
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
            SM2OUT23R::SM2OUT23_0 => false,
            SM2OUT23R::SM2OUT23_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM2OUT23R {
        match value {
            false => SM2OUT23R::SM2OUT23_0,
            true => SM2OUT23R::SM2OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM2OUT23_0`"]
    #[inline]
    pub fn is_sm2out23_0(&self) -> bool {
        *self == SM2OUT23R::SM2OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM2OUT23_1`"]
    #[inline]
    pub fn is_sm2out23_1(&self) -> bool {
        *self == SM2OUT23R::SM2OUT23_1
    }
}
#[doc = "Possible values of the field `SM3OUT45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM3OUT45R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    SM3OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    SM3OUT45_1,
}
impl SM3OUT45R {
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
            SM3OUT45R::SM3OUT45_0 => false,
            SM3OUT45R::SM3OUT45_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM3OUT45R {
        match value {
            false => SM3OUT45R::SM3OUT45_0,
            true => SM3OUT45R::SM3OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM3OUT45_0`"]
    #[inline]
    pub fn is_sm3out45_0(&self) -> bool {
        *self == SM3OUT45R::SM3OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM3OUT45_1`"]
    #[inline]
    pub fn is_sm3out45_1(&self) -> bool {
        *self == SM3OUT45R::SM3OUT45_1
    }
}
#[doc = "Possible values of the field `SM3OUT23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM3OUT23R {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    SM3OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    SM3OUT23_1,
}
impl SM3OUT23R {
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
            SM3OUT23R::SM3OUT23_0 => false,
            SM3OUT23R::SM3OUT23_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM3OUT23R {
        match value {
            false => SM3OUT23R::SM3OUT23_0,
            true => SM3OUT23R::SM3OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM3OUT23_0`"]
    #[inline]
    pub fn is_sm3out23_0(&self) -> bool {
        *self == SM3OUT23R::SM3OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM3OUT23_1`"]
    #[inline]
    pub fn is_sm3out23_1(&self) -> bool {
        *self == SM3OUT23R::SM3OUT23_1
    }
}
#[doc = "Values that can be written to the field `SM0OUT45`"]
pub enum SM0OUT45W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SM0OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SM0OUT45_1,
}
impl SM0OUT45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM0OUT45W::SM0OUT45_0 => false,
            SM0OUT45W::SM0OUT45_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM0OUT45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM0OUT45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM0OUT45W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline]
    pub fn sm0out45_0(self) -> &'a mut W {
        self.variant(SM0OUT45W::SM0OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline]
    pub fn sm0out45_1(self) -> &'a mut W {
        self.variant(SM0OUT45W::SM0OUT45_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM0OUT23`"]
pub enum SM0OUT23W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SM0OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SM0OUT23_1,
}
impl SM0OUT23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM0OUT23W::SM0OUT23_0 => false,
            SM0OUT23W::SM0OUT23_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM0OUT23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM0OUT23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM0OUT23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline]
    pub fn sm0out23_0(self) -> &'a mut W {
        self.variant(SM0OUT23W::SM0OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline]
    pub fn sm0out23_1(self) -> &'a mut W {
        self.variant(SM0OUT23W::SM0OUT23_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM1OUT45`"]
pub enum SM1OUT45W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    SM1OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    SM1OUT45_1,
}
impl SM1OUT45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM1OUT45W::SM1OUT45_0 => false,
            SM1OUT45W::SM1OUT45_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM1OUT45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM1OUT45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM1OUT45W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    #[inline]
    pub fn sm1out45_0(self) -> &'a mut W {
        self.variant(SM1OUT45W::SM1OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    #[inline]
    pub fn sm1out45_1(self) -> &'a mut W {
        self.variant(SM1OUT45W::SM1OUT45_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM1OUT23`"]
pub enum SM1OUT23W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    SM1OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    SM1OUT23_1,
}
impl SM1OUT23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM1OUT23W::SM1OUT23_0 => false,
            SM1OUT23W::SM1OUT23_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM1OUT23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM1OUT23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM1OUT23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    #[inline]
    pub fn sm1out23_0(self) -> &'a mut W {
        self.variant(SM1OUT23W::SM1OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    #[inline]
    pub fn sm1out23_1(self) -> &'a mut W {
        self.variant(SM1OUT23W::SM1OUT23_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM2OUT45`"]
pub enum SM2OUT45W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    SM2OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    SM2OUT45_1,
}
impl SM2OUT45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM2OUT45W::SM2OUT45_0 => false,
            SM2OUT45W::SM2OUT45_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM2OUT45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM2OUT45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM2OUT45W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    #[inline]
    pub fn sm2out45_0(self) -> &'a mut W {
        self.variant(SM2OUT45W::SM2OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    #[inline]
    pub fn sm2out45_1(self) -> &'a mut W {
        self.variant(SM2OUT45W::SM2OUT45_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM2OUT23`"]
pub enum SM2OUT23W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    SM2OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    SM2OUT23_1,
}
impl SM2OUT23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM2OUT23W::SM2OUT23_0 => false,
            SM2OUT23W::SM2OUT23_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM2OUT23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM2OUT23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM2OUT23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    #[inline]
    pub fn sm2out23_0(self) -> &'a mut W {
        self.variant(SM2OUT23W::SM2OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    #[inline]
    pub fn sm2out23_1(self) -> &'a mut W {
        self.variant(SM2OUT23W::SM2OUT23_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM3OUT45`"]
pub enum SM3OUT45W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    SM3OUT45_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    SM3OUT45_1,
}
impl SM3OUT45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM3OUT45W::SM3OUT45_0 => false,
            SM3OUT45W::SM3OUT45_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM3OUT45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM3OUT45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM3OUT45W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    #[inline]
    pub fn sm3out45_0(self) -> &'a mut W {
        self.variant(SM3OUT45W::SM3OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    #[inline]
    pub fn sm3out45_1(self) -> &'a mut W {
        self.variant(SM3OUT45W::SM3OUT45_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM3OUT23`"]
pub enum SM3OUT23W {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    SM3OUT23_0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    SM3OUT23_1,
}
impl SM3OUT23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM3OUT23W::SM3OUT23_0 => false,
            SM3OUT23W::SM3OUT23_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM3OUT23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM3OUT23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM3OUT23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    #[inline]
    pub fn sm3out23_0(self) -> &'a mut W {
        self.variant(SM3OUT23W::SM3OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    #[inline]
    pub fn sm3out23_1(self) -> &'a mut W {
        self.variant(SM3OUT23W::SM3OUT23_1)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Submodule 0 Software Controlled Output 45"]
    #[inline]
    pub fn sm0out45(&self) -> SM0OUT45R {
        SM0OUT45R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline]
    pub fn sm0out23(&self) -> SM0OUT23R {
        SM0OUT23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Submodule 1 Software Controlled Output 45"]
    #[inline]
    pub fn sm1out45(&self) -> SM1OUT45R {
        SM1OUT45R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Submodule 1 Software Controlled Output 23"]
    #[inline]
    pub fn sm1out23(&self) -> SM1OUT23R {
        SM1OUT23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Submodule 2 Software Controlled Output 45"]
    #[inline]
    pub fn sm2out45(&self) -> SM2OUT45R {
        SM2OUT45R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Submodule 2 Software Controlled Output 23"]
    #[inline]
    pub fn sm2out23(&self) -> SM2OUT23R {
        SM2OUT23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Submodule 3 Software Controlled Output 45"]
    #[inline]
    pub fn sm3out45(&self) -> SM3OUT45R {
        SM3OUT45R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Submodule 3 Software Controlled Output 23"]
    #[inline]
    pub fn sm3out23(&self) -> SM3OUT23R {
        SM3OUT23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Submodule 0 Software Controlled Output 45"]
    #[inline]
    pub fn sm0out45(&mut self) -> _SM0OUT45W {
        _SM0OUT45W { w: self }
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline]
    pub fn sm0out23(&mut self) -> _SM0OUT23W {
        _SM0OUT23W { w: self }
    }
    #[doc = "Bit 2 - Submodule 1 Software Controlled Output 45"]
    #[inline]
    pub fn sm1out45(&mut self) -> _SM1OUT45W {
        _SM1OUT45W { w: self }
    }
    #[doc = "Bit 3 - Submodule 1 Software Controlled Output 23"]
    #[inline]
    pub fn sm1out23(&mut self) -> _SM1OUT23W {
        _SM1OUT23W { w: self }
    }
    #[doc = "Bit 4 - Submodule 2 Software Controlled Output 45"]
    #[inline]
    pub fn sm2out45(&mut self) -> _SM2OUT45W {
        _SM2OUT45W { w: self }
    }
    #[doc = "Bit 5 - Submodule 2 Software Controlled Output 23"]
    #[inline]
    pub fn sm2out23(&mut self) -> _SM2OUT23W {
        _SM2OUT23W { w: self }
    }
    #[doc = "Bit 6 - Submodule 3 Software Controlled Output 45"]
    #[inline]
    pub fn sm3out45(&mut self) -> _SM3OUT45W {
        _SM3OUT45W { w: self }
    }
    #[doc = "Bit 7 - Submodule 3 Software Controlled Output 23"]
    #[inline]
    pub fn sm3out23(&mut self) -> _SM3OUT23W {
        _SM3OUT23W { w: self }
    }
}
