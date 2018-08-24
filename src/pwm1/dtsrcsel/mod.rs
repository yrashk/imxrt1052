#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::DTSRCSEL {
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
#[doc = "Possible values of the field `SM0SEL45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM0SEL45R {
    #[doc = "Generated SM0PWM45 signal is used by the deadtime logic."]
    SM0SEL45_0,
    #[doc = "Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    SM0SEL45_1,
    #[doc = "SWCOUT[SM0OUT45] is used by the deadtime logic."]
    SM0SEL45_2,
    #[doc = "PWM0_EXTB signal is used by the deadtime logic."]
    SM0SEL45_3,
}
impl SM0SEL45R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM0SEL45R::SM0SEL45_0 => 0,
            SM0SEL45R::SM0SEL45_1 => 1,
            SM0SEL45R::SM0SEL45_2 => 2,
            SM0SEL45R::SM0SEL45_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM0SEL45R {
        match value {
            0 => SM0SEL45R::SM0SEL45_0,
            1 => SM0SEL45R::SM0SEL45_1,
            2 => SM0SEL45R::SM0SEL45_2,
            3 => SM0SEL45R::SM0SEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM0SEL45_0`"]
    #[inline]
    pub fn is_sm0sel45_0(&self) -> bool {
        *self == SM0SEL45R::SM0SEL45_0
    }
    #[doc = "Checks if the value of the field is `SM0SEL45_1`"]
    #[inline]
    pub fn is_sm0sel45_1(&self) -> bool {
        *self == SM0SEL45R::SM0SEL45_1
    }
    #[doc = "Checks if the value of the field is `SM0SEL45_2`"]
    #[inline]
    pub fn is_sm0sel45_2(&self) -> bool {
        *self == SM0SEL45R::SM0SEL45_2
    }
    #[doc = "Checks if the value of the field is `SM0SEL45_3`"]
    #[inline]
    pub fn is_sm0sel45_3(&self) -> bool {
        *self == SM0SEL45R::SM0SEL45_3
    }
}
#[doc = "Possible values of the field `SM0SEL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM0SEL23R {
    #[doc = "Generated SM0PWM23 signal is used by the deadtime logic."]
    SM0SEL23_0,
    #[doc = "Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    SM0SEL23_1,
    #[doc = "SWCOUT[SM0OUT23] is used by the deadtime logic."]
    SM0SEL23_2,
    #[doc = "PWM0_EXTA signal is used by the deadtime logic."]
    SM0SEL23_3,
}
impl SM0SEL23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM0SEL23R::SM0SEL23_0 => 0,
            SM0SEL23R::SM0SEL23_1 => 1,
            SM0SEL23R::SM0SEL23_2 => 2,
            SM0SEL23R::SM0SEL23_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM0SEL23R {
        match value {
            0 => SM0SEL23R::SM0SEL23_0,
            1 => SM0SEL23R::SM0SEL23_1,
            2 => SM0SEL23R::SM0SEL23_2,
            3 => SM0SEL23R::SM0SEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM0SEL23_0`"]
    #[inline]
    pub fn is_sm0sel23_0(&self) -> bool {
        *self == SM0SEL23R::SM0SEL23_0
    }
    #[doc = "Checks if the value of the field is `SM0SEL23_1`"]
    #[inline]
    pub fn is_sm0sel23_1(&self) -> bool {
        *self == SM0SEL23R::SM0SEL23_1
    }
    #[doc = "Checks if the value of the field is `SM0SEL23_2`"]
    #[inline]
    pub fn is_sm0sel23_2(&self) -> bool {
        *self == SM0SEL23R::SM0SEL23_2
    }
    #[doc = "Checks if the value of the field is `SM0SEL23_3`"]
    #[inline]
    pub fn is_sm0sel23_3(&self) -> bool {
        *self == SM0SEL23R::SM0SEL23_3
    }
}
#[doc = "Possible values of the field `SM1SEL45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM1SEL45R {
    #[doc = "Generated SM1PWM45 signal is used by the deadtime logic."]
    SM1SEL45_0,
    #[doc = "Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    SM1SEL45_1,
    #[doc = "SWCOUT[SM1OUT45] is used by the deadtime logic."]
    SM1SEL45_2,
    #[doc = "PWM1_EXTB signal is used by the deadtime logic."]
    SM1SEL45_3,
}
impl SM1SEL45R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM1SEL45R::SM1SEL45_0 => 0,
            SM1SEL45R::SM1SEL45_1 => 1,
            SM1SEL45R::SM1SEL45_2 => 2,
            SM1SEL45R::SM1SEL45_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM1SEL45R {
        match value {
            0 => SM1SEL45R::SM1SEL45_0,
            1 => SM1SEL45R::SM1SEL45_1,
            2 => SM1SEL45R::SM1SEL45_2,
            3 => SM1SEL45R::SM1SEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_0`"]
    #[inline]
    pub fn is_sm1sel45_0(&self) -> bool {
        *self == SM1SEL45R::SM1SEL45_0
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_1`"]
    #[inline]
    pub fn is_sm1sel45_1(&self) -> bool {
        *self == SM1SEL45R::SM1SEL45_1
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_2`"]
    #[inline]
    pub fn is_sm1sel45_2(&self) -> bool {
        *self == SM1SEL45R::SM1SEL45_2
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_3`"]
    #[inline]
    pub fn is_sm1sel45_3(&self) -> bool {
        *self == SM1SEL45R::SM1SEL45_3
    }
}
#[doc = "Possible values of the field `SM1SEL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM1SEL23R {
    #[doc = "Generated SM1PWM23 signal is used by the deadtime logic."]
    SM1SEL23_0,
    #[doc = "Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    SM1SEL23_1,
    #[doc = "SWCOUT[SM1OUT23] is used by the deadtime logic."]
    SM1SEL23_2,
    #[doc = "PWM1_EXTA signal is used by the deadtime logic."]
    SM1SEL23_3,
}
impl SM1SEL23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM1SEL23R::SM1SEL23_0 => 0,
            SM1SEL23R::SM1SEL23_1 => 1,
            SM1SEL23R::SM1SEL23_2 => 2,
            SM1SEL23R::SM1SEL23_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM1SEL23R {
        match value {
            0 => SM1SEL23R::SM1SEL23_0,
            1 => SM1SEL23R::SM1SEL23_1,
            2 => SM1SEL23R::SM1SEL23_2,
            3 => SM1SEL23R::SM1SEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_0`"]
    #[inline]
    pub fn is_sm1sel23_0(&self) -> bool {
        *self == SM1SEL23R::SM1SEL23_0
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_1`"]
    #[inline]
    pub fn is_sm1sel23_1(&self) -> bool {
        *self == SM1SEL23R::SM1SEL23_1
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_2`"]
    #[inline]
    pub fn is_sm1sel23_2(&self) -> bool {
        *self == SM1SEL23R::SM1SEL23_2
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_3`"]
    #[inline]
    pub fn is_sm1sel23_3(&self) -> bool {
        *self == SM1SEL23R::SM1SEL23_3
    }
}
#[doc = "Possible values of the field `SM2SEL45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM2SEL45R {
    #[doc = "Generated SM2PWM45 signal is used by the deadtime logic."]
    SM2SEL45_0,
    #[doc = "Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    SM2SEL45_1,
    #[doc = "SWCOUT[SM2OUT45] is used by the deadtime logic."]
    SM2SEL45_2,
    #[doc = "PWM2_EXTB signal is used by the deadtime logic."]
    SM2SEL45_3,
}
impl SM2SEL45R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM2SEL45R::SM2SEL45_0 => 0,
            SM2SEL45R::SM2SEL45_1 => 1,
            SM2SEL45R::SM2SEL45_2 => 2,
            SM2SEL45R::SM2SEL45_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM2SEL45R {
        match value {
            0 => SM2SEL45R::SM2SEL45_0,
            1 => SM2SEL45R::SM2SEL45_1,
            2 => SM2SEL45R::SM2SEL45_2,
            3 => SM2SEL45R::SM2SEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_0`"]
    #[inline]
    pub fn is_sm2sel45_0(&self) -> bool {
        *self == SM2SEL45R::SM2SEL45_0
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_1`"]
    #[inline]
    pub fn is_sm2sel45_1(&self) -> bool {
        *self == SM2SEL45R::SM2SEL45_1
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_2`"]
    #[inline]
    pub fn is_sm2sel45_2(&self) -> bool {
        *self == SM2SEL45R::SM2SEL45_2
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_3`"]
    #[inline]
    pub fn is_sm2sel45_3(&self) -> bool {
        *self == SM2SEL45R::SM2SEL45_3
    }
}
#[doc = "Possible values of the field `SM2SEL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM2SEL23R {
    #[doc = "Generated SM2PWM23 signal is used by the deadtime logic."]
    SM2SEL23_0,
    #[doc = "Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    SM2SEL23_1,
    #[doc = "SWCOUT[SM2OUT23] is used by the deadtime logic."]
    SM2SEL23_2,
    #[doc = "PWM2_EXTA signal is used by the deadtime logic."]
    SM2SEL23_3,
}
impl SM2SEL23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM2SEL23R::SM2SEL23_0 => 0,
            SM2SEL23R::SM2SEL23_1 => 1,
            SM2SEL23R::SM2SEL23_2 => 2,
            SM2SEL23R::SM2SEL23_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM2SEL23R {
        match value {
            0 => SM2SEL23R::SM2SEL23_0,
            1 => SM2SEL23R::SM2SEL23_1,
            2 => SM2SEL23R::SM2SEL23_2,
            3 => SM2SEL23R::SM2SEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_0`"]
    #[inline]
    pub fn is_sm2sel23_0(&self) -> bool {
        *self == SM2SEL23R::SM2SEL23_0
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_1`"]
    #[inline]
    pub fn is_sm2sel23_1(&self) -> bool {
        *self == SM2SEL23R::SM2SEL23_1
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_2`"]
    #[inline]
    pub fn is_sm2sel23_2(&self) -> bool {
        *self == SM2SEL23R::SM2SEL23_2
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_3`"]
    #[inline]
    pub fn is_sm2sel23_3(&self) -> bool {
        *self == SM2SEL23R::SM2SEL23_3
    }
}
#[doc = "Possible values of the field `SM3SEL45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM3SEL45R {
    #[doc = "Generated SM3PWM45 signal is used by the deadtime logic."]
    SM3SEL45_0,
    #[doc = "Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    SM3SEL45_1,
    #[doc = "SWCOUT[SM3OUT45] is used by the deadtime logic."]
    SM3SEL45_2,
    #[doc = "PWM3_EXTB signal is used by the deadtime logic."]
    SM3SEL45_3,
}
impl SM3SEL45R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM3SEL45R::SM3SEL45_0 => 0,
            SM3SEL45R::SM3SEL45_1 => 1,
            SM3SEL45R::SM3SEL45_2 => 2,
            SM3SEL45R::SM3SEL45_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM3SEL45R {
        match value {
            0 => SM3SEL45R::SM3SEL45_0,
            1 => SM3SEL45R::SM3SEL45_1,
            2 => SM3SEL45R::SM3SEL45_2,
            3 => SM3SEL45R::SM3SEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_0`"]
    #[inline]
    pub fn is_sm3sel45_0(&self) -> bool {
        *self == SM3SEL45R::SM3SEL45_0
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_1`"]
    #[inline]
    pub fn is_sm3sel45_1(&self) -> bool {
        *self == SM3SEL45R::SM3SEL45_1
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_2`"]
    #[inline]
    pub fn is_sm3sel45_2(&self) -> bool {
        *self == SM3SEL45R::SM3SEL45_2
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_3`"]
    #[inline]
    pub fn is_sm3sel45_3(&self) -> bool {
        *self == SM3SEL45R::SM3SEL45_3
    }
}
#[doc = "Possible values of the field `SM3SEL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM3SEL23R {
    #[doc = "Generated SM3PWM23 signal is used by the deadtime logic."]
    SM3SEL23_0,
    #[doc = "Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    SM3SEL23_1,
    #[doc = "SWCOUT[SM3OUT23] is used by the deadtime logic."]
    SM3SEL23_2,
    #[doc = "PWM3_EXTA signal is used by the deadtime logic."]
    SM3SEL23_3,
}
impl SM3SEL23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SM3SEL23R::SM3SEL23_0 => 0,
            SM3SEL23R::SM3SEL23_1 => 1,
            SM3SEL23R::SM3SEL23_2 => 2,
            SM3SEL23R::SM3SEL23_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SM3SEL23R {
        match value {
            0 => SM3SEL23R::SM3SEL23_0,
            1 => SM3SEL23R::SM3SEL23_1,
            2 => SM3SEL23R::SM3SEL23_2,
            3 => SM3SEL23R::SM3SEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_0`"]
    #[inline]
    pub fn is_sm3sel23_0(&self) -> bool {
        *self == SM3SEL23R::SM3SEL23_0
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_1`"]
    #[inline]
    pub fn is_sm3sel23_1(&self) -> bool {
        *self == SM3SEL23R::SM3SEL23_1
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_2`"]
    #[inline]
    pub fn is_sm3sel23_2(&self) -> bool {
        *self == SM3SEL23R::SM3SEL23_2
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_3`"]
    #[inline]
    pub fn is_sm3sel23_3(&self) -> bool {
        *self == SM3SEL23R::SM3SEL23_3
    }
}
#[doc = "Values that can be written to the field `SM0SEL45`"]
pub enum SM0SEL45W {
    #[doc = "Generated SM0PWM45 signal is used by the deadtime logic."]
    SM0SEL45_0,
    #[doc = "Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    SM0SEL45_1,
    #[doc = "SWCOUT[SM0OUT45] is used by the deadtime logic."]
    SM0SEL45_2,
    #[doc = "PWM0_EXTB signal is used by the deadtime logic."]
    SM0SEL45_3,
}
impl SM0SEL45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM0SEL45W::SM0SEL45_0 => 0,
            SM0SEL45W::SM0SEL45_1 => 1,
            SM0SEL45W::SM0SEL45_2 => 2,
            SM0SEL45W::SM0SEL45_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM0SEL45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM0SEL45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM0SEL45W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM0PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel45_0(self) -> &'a mut W {
        self.variant(SM0SEL45W::SM0SEL45_0)
    }
    #[doc = "Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel45_1(self) -> &'a mut W {
        self.variant(SM0SEL45W::SM0SEL45_1)
    }
    #[doc = "SWCOUT[SM0OUT45] is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel45_2(self) -> &'a mut W {
        self.variant(SM0SEL45W::SM0SEL45_2)
    }
    #[doc = "PWM0_EXTB signal is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel45_3(self) -> &'a mut W {
        self.variant(SM0SEL45W::SM0SEL45_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM0SEL23`"]
pub enum SM0SEL23W {
    #[doc = "Generated SM0PWM23 signal is used by the deadtime logic."]
    SM0SEL23_0,
    #[doc = "Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    SM0SEL23_1,
    #[doc = "SWCOUT[SM0OUT23] is used by the deadtime logic."]
    SM0SEL23_2,
    #[doc = "PWM0_EXTA signal is used by the deadtime logic."]
    SM0SEL23_3,
}
impl SM0SEL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM0SEL23W::SM0SEL23_0 => 0,
            SM0SEL23W::SM0SEL23_1 => 1,
            SM0SEL23W::SM0SEL23_2 => 2,
            SM0SEL23W::SM0SEL23_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM0SEL23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM0SEL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM0SEL23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM0PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel23_0(self) -> &'a mut W {
        self.variant(SM0SEL23W::SM0SEL23_0)
    }
    #[doc = "Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel23_1(self) -> &'a mut W {
        self.variant(SM0SEL23W::SM0SEL23_1)
    }
    #[doc = "SWCOUT[SM0OUT23] is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel23_2(self) -> &'a mut W {
        self.variant(SM0SEL23W::SM0SEL23_2)
    }
    #[doc = "PWM0_EXTA signal is used by the deadtime logic."]
    #[inline]
    pub fn sm0sel23_3(self) -> &'a mut W {
        self.variant(SM0SEL23W::SM0SEL23_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM1SEL45`"]
pub enum SM1SEL45W {
    #[doc = "Generated SM1PWM45 signal is used by the deadtime logic."]
    SM1SEL45_0,
    #[doc = "Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    SM1SEL45_1,
    #[doc = "SWCOUT[SM1OUT45] is used by the deadtime logic."]
    SM1SEL45_2,
    #[doc = "PWM1_EXTB signal is used by the deadtime logic."]
    SM1SEL45_3,
}
impl SM1SEL45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM1SEL45W::SM1SEL45_0 => 0,
            SM1SEL45W::SM1SEL45_1 => 1,
            SM1SEL45W::SM1SEL45_2 => 2,
            SM1SEL45W::SM1SEL45_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM1SEL45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM1SEL45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM1SEL45W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM1PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel45_0(self) -> &'a mut W {
        self.variant(SM1SEL45W::SM1SEL45_0)
    }
    #[doc = "Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel45_1(self) -> &'a mut W {
        self.variant(SM1SEL45W::SM1SEL45_1)
    }
    #[doc = "SWCOUT[SM1OUT45] is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel45_2(self) -> &'a mut W {
        self.variant(SM1SEL45W::SM1SEL45_2)
    }
    #[doc = "PWM1_EXTB signal is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel45_3(self) -> &'a mut W {
        self.variant(SM1SEL45W::SM1SEL45_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM1SEL23`"]
pub enum SM1SEL23W {
    #[doc = "Generated SM1PWM23 signal is used by the deadtime logic."]
    SM1SEL23_0,
    #[doc = "Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    SM1SEL23_1,
    #[doc = "SWCOUT[SM1OUT23] is used by the deadtime logic."]
    SM1SEL23_2,
    #[doc = "PWM1_EXTA signal is used by the deadtime logic."]
    SM1SEL23_3,
}
impl SM1SEL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM1SEL23W::SM1SEL23_0 => 0,
            SM1SEL23W::SM1SEL23_1 => 1,
            SM1SEL23W::SM1SEL23_2 => 2,
            SM1SEL23W::SM1SEL23_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM1SEL23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM1SEL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM1SEL23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM1PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel23_0(self) -> &'a mut W {
        self.variant(SM1SEL23W::SM1SEL23_0)
    }
    #[doc = "Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel23_1(self) -> &'a mut W {
        self.variant(SM1SEL23W::SM1SEL23_1)
    }
    #[doc = "SWCOUT[SM1OUT23] is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel23_2(self) -> &'a mut W {
        self.variant(SM1SEL23W::SM1SEL23_2)
    }
    #[doc = "PWM1_EXTA signal is used by the deadtime logic."]
    #[inline]
    pub fn sm1sel23_3(self) -> &'a mut W {
        self.variant(SM1SEL23W::SM1SEL23_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM2SEL45`"]
pub enum SM2SEL45W {
    #[doc = "Generated SM2PWM45 signal is used by the deadtime logic."]
    SM2SEL45_0,
    #[doc = "Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    SM2SEL45_1,
    #[doc = "SWCOUT[SM2OUT45] is used by the deadtime logic."]
    SM2SEL45_2,
    #[doc = "PWM2_EXTB signal is used by the deadtime logic."]
    SM2SEL45_3,
}
impl SM2SEL45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM2SEL45W::SM2SEL45_0 => 0,
            SM2SEL45W::SM2SEL45_1 => 1,
            SM2SEL45W::SM2SEL45_2 => 2,
            SM2SEL45W::SM2SEL45_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM2SEL45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM2SEL45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM2SEL45W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM2PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel45_0(self) -> &'a mut W {
        self.variant(SM2SEL45W::SM2SEL45_0)
    }
    #[doc = "Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel45_1(self) -> &'a mut W {
        self.variant(SM2SEL45W::SM2SEL45_1)
    }
    #[doc = "SWCOUT[SM2OUT45] is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel45_2(self) -> &'a mut W {
        self.variant(SM2SEL45W::SM2SEL45_2)
    }
    #[doc = "PWM2_EXTB signal is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel45_3(self) -> &'a mut W {
        self.variant(SM2SEL45W::SM2SEL45_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM2SEL23`"]
pub enum SM2SEL23W {
    #[doc = "Generated SM2PWM23 signal is used by the deadtime logic."]
    SM2SEL23_0,
    #[doc = "Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    SM2SEL23_1,
    #[doc = "SWCOUT[SM2OUT23] is used by the deadtime logic."]
    SM2SEL23_2,
    #[doc = "PWM2_EXTA signal is used by the deadtime logic."]
    SM2SEL23_3,
}
impl SM2SEL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM2SEL23W::SM2SEL23_0 => 0,
            SM2SEL23W::SM2SEL23_1 => 1,
            SM2SEL23W::SM2SEL23_2 => 2,
            SM2SEL23W::SM2SEL23_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM2SEL23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM2SEL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM2SEL23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM2PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel23_0(self) -> &'a mut W {
        self.variant(SM2SEL23W::SM2SEL23_0)
    }
    #[doc = "Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel23_1(self) -> &'a mut W {
        self.variant(SM2SEL23W::SM2SEL23_1)
    }
    #[doc = "SWCOUT[SM2OUT23] is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel23_2(self) -> &'a mut W {
        self.variant(SM2SEL23W::SM2SEL23_2)
    }
    #[doc = "PWM2_EXTA signal is used by the deadtime logic."]
    #[inline]
    pub fn sm2sel23_3(self) -> &'a mut W {
        self.variant(SM2SEL23W::SM2SEL23_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM3SEL45`"]
pub enum SM3SEL45W {
    #[doc = "Generated SM3PWM45 signal is used by the deadtime logic."]
    SM3SEL45_0,
    #[doc = "Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    SM3SEL45_1,
    #[doc = "SWCOUT[SM3OUT45] is used by the deadtime logic."]
    SM3SEL45_2,
    #[doc = "PWM3_EXTB signal is used by the deadtime logic."]
    SM3SEL45_3,
}
impl SM3SEL45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM3SEL45W::SM3SEL45_0 => 0,
            SM3SEL45W::SM3SEL45_1 => 1,
            SM3SEL45W::SM3SEL45_2 => 2,
            SM3SEL45W::SM3SEL45_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM3SEL45W<'a> {
    w: &'a mut W,
}
impl<'a> _SM3SEL45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM3SEL45W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM3PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel45_0(self) -> &'a mut W {
        self.variant(SM3SEL45W::SM3SEL45_0)
    }
    #[doc = "Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel45_1(self) -> &'a mut W {
        self.variant(SM3SEL45W::SM3SEL45_1)
    }
    #[doc = "SWCOUT[SM3OUT45] is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel45_2(self) -> &'a mut W {
        self.variant(SM3SEL45W::SM3SEL45_2)
    }
    #[doc = "PWM3_EXTB signal is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel45_3(self) -> &'a mut W {
        self.variant(SM3SEL45W::SM3SEL45_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM3SEL23`"]
pub enum SM3SEL23W {
    #[doc = "Generated SM3PWM23 signal is used by the deadtime logic."]
    SM3SEL23_0,
    #[doc = "Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    SM3SEL23_1,
    #[doc = "SWCOUT[SM3OUT23] is used by the deadtime logic."]
    SM3SEL23_2,
    #[doc = "PWM3_EXTA signal is used by the deadtime logic."]
    SM3SEL23_3,
}
impl SM3SEL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SM3SEL23W::SM3SEL23_0 => 0,
            SM3SEL23W::SM3SEL23_1 => 1,
            SM3SEL23W::SM3SEL23_2 => 2,
            SM3SEL23W::SM3SEL23_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM3SEL23W<'a> {
    w: &'a mut W,
}
impl<'a> _SM3SEL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM3SEL23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generated SM3PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel23_0(self) -> &'a mut W {
        self.variant(SM3SEL23W::SM3SEL23_0)
    }
    #[doc = "Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel23_1(self) -> &'a mut W {
        self.variant(SM3SEL23W::SM3SEL23_1)
    }
    #[doc = "SWCOUT[SM3OUT23] is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel23_2(self) -> &'a mut W {
        self.variant(SM3SEL23W::SM3SEL23_2)
    }
    #[doc = "PWM3_EXTA signal is used by the deadtime logic."]
    #[inline]
    pub fn sm3sel23_3(self) -> &'a mut W {
        self.variant(SM3SEL23W::SM3SEL23_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Submodule 0 PWM45 Control Select"]
    #[inline]
    pub fn sm0sel45(&self) -> SM0SEL45R {
        SM0SEL45R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - Submodule 0 PWM23 Control Select"]
    #[inline]
    pub fn sm0sel23(&self) -> SM0SEL23R {
        SM0SEL23R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - Submodule 1 PWM45 Control Select"]
    #[inline]
    pub fn sm1sel45(&self) -> SM1SEL45R {
        SM1SEL45R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 6:7 - Submodule 1 PWM23 Control Select"]
    #[inline]
    pub fn sm1sel23(&self) -> SM1SEL23R {
        SM1SEL23R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:9 - Submodule 2 PWM45 Control Select"]
    #[inline]
    pub fn sm2sel45(&self) -> SM2SEL45R {
        SM2SEL45R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 10:11 - Submodule 2 PWM23 Control Select"]
    #[inline]
    pub fn sm2sel23(&self) -> SM2SEL23R {
        SM2SEL23R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 12:13 - Submodule 3 PWM45 Control Select"]
    #[inline]
    pub fn sm3sel45(&self) -> SM3SEL45R {
        SM3SEL45R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 14:15 - Submodule 3 PWM23 Control Select"]
    #[inline]
    pub fn sm3sel23(&self) -> SM3SEL23R {
        SM3SEL23R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    #[doc = "Bits 0:1 - Submodule 0 PWM45 Control Select"]
    #[inline]
    pub fn sm0sel45(&mut self) -> _SM0SEL45W {
        _SM0SEL45W { w: self }
    }
    #[doc = "Bits 2:3 - Submodule 0 PWM23 Control Select"]
    #[inline]
    pub fn sm0sel23(&mut self) -> _SM0SEL23W {
        _SM0SEL23W { w: self }
    }
    #[doc = "Bits 4:5 - Submodule 1 PWM45 Control Select"]
    #[inline]
    pub fn sm1sel45(&mut self) -> _SM1SEL45W {
        _SM1SEL45W { w: self }
    }
    #[doc = "Bits 6:7 - Submodule 1 PWM23 Control Select"]
    #[inline]
    pub fn sm1sel23(&mut self) -> _SM1SEL23W {
        _SM1SEL23W { w: self }
    }
    #[doc = "Bits 8:9 - Submodule 2 PWM45 Control Select"]
    #[inline]
    pub fn sm2sel45(&mut self) -> _SM2SEL45W {
        _SM2SEL45W { w: self }
    }
    #[doc = "Bits 10:11 - Submodule 2 PWM23 Control Select"]
    #[inline]
    pub fn sm2sel23(&mut self) -> _SM2SEL23W {
        _SM2SEL23W { w: self }
    }
    #[doc = "Bits 12:13 - Submodule 3 PWM45 Control Select"]
    #[inline]
    pub fn sm3sel45(&mut self) -> _SM3SEL45W {
        _SM3SEL45W { w: self }
    }
    #[doc = "Bits 14:15 - Submodule 3 PWM23 Control Select"]
    #[inline]
    pub fn sm3sel23(&mut self) -> _SM3SEL23W {
        _SM3SEL23W { w: self }
    }
}
