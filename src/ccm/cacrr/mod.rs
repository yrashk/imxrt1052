#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CACRR {
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
#[doc = "Possible values of the field `ARM_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_PODFR {
    #[doc = "divide by 1"]
    ARM_PODF_0,
    #[doc = "divide by 2"]
    ARM_PODF_1,
    #[doc = "divide by 3"]
    ARM_PODF_2,
    #[doc = "divide by 4"]
    ARM_PODF_3,
    #[doc = "divide by 5"]
    ARM_PODF_4,
    #[doc = "divide by 6"]
    ARM_PODF_5,
    #[doc = "divide by 7"]
    ARM_PODF_6,
    #[doc = "divide by 8"]
    ARM_PODF_7,
}
impl ARM_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARM_PODFR::ARM_PODF_0 => 0,
            ARM_PODFR::ARM_PODF_1 => 1,
            ARM_PODFR::ARM_PODF_2 => 2,
            ARM_PODFR::ARM_PODF_3 => 3,
            ARM_PODFR::ARM_PODF_4 => 4,
            ARM_PODFR::ARM_PODF_5 => 5,
            ARM_PODFR::ARM_PODF_6 => 6,
            ARM_PODFR::ARM_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARM_PODFR {
        match value {
            0 => ARM_PODFR::ARM_PODF_0,
            1 => ARM_PODFR::ARM_PODF_1,
            2 => ARM_PODFR::ARM_PODF_2,
            3 => ARM_PODFR::ARM_PODF_3,
            4 => ARM_PODFR::ARM_PODF_4,
            5 => ARM_PODFR::ARM_PODF_5,
            6 => ARM_PODFR::ARM_PODF_6,
            7 => ARM_PODFR::ARM_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_0`"]
    #[inline]
    pub fn is_arm_podf_0(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_1`"]
    #[inline]
    pub fn is_arm_podf_1(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_1
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_2`"]
    #[inline]
    pub fn is_arm_podf_2(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_2
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_3`"]
    #[inline]
    pub fn is_arm_podf_3(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_3
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_4`"]
    #[inline]
    pub fn is_arm_podf_4(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_4
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_5`"]
    #[inline]
    pub fn is_arm_podf_5(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_5
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_6`"]
    #[inline]
    pub fn is_arm_podf_6(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_6
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_7`"]
    #[inline]
    pub fn is_arm_podf_7(&self) -> bool {
        *self == ARM_PODFR::ARM_PODF_7
    }
}
#[doc = "Values that can be written to the field `ARM_PODF`"]
pub enum ARM_PODFW {
    #[doc = "divide by 1"]
    ARM_PODF_0,
    #[doc = "divide by 2"]
    ARM_PODF_1,
    #[doc = "divide by 3"]
    ARM_PODF_2,
    #[doc = "divide by 4"]
    ARM_PODF_3,
    #[doc = "divide by 5"]
    ARM_PODF_4,
    #[doc = "divide by 6"]
    ARM_PODF_5,
    #[doc = "divide by 7"]
    ARM_PODF_6,
    #[doc = "divide by 8"]
    ARM_PODF_7,
}
impl ARM_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ARM_PODFW::ARM_PODF_0 => 0,
            ARM_PODFW::ARM_PODF_1 => 1,
            ARM_PODFW::ARM_PODF_2 => 2,
            ARM_PODFW::ARM_PODF_3 => 3,
            ARM_PODFW::ARM_PODF_4 => 4,
            ARM_PODFW::ARM_PODF_5 => 5,
            ARM_PODFW::ARM_PODF_6 => 6,
            ARM_PODFW::ARM_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARM_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _ARM_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARM_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn arm_podf_0(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn arm_podf_1(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn arm_podf_2(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn arm_podf_3(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn arm_podf_4(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn arm_podf_5(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn arm_podf_6(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn arm_podf_7(self) -> &'a mut W {
        self.variant(ARM_PODFW::ARM_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Divider for ARM clock root"]
    #[inline]
    pub fn arm_podf(&self) -> ARM_PODFR {
        ARM_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Divider for ARM clock root"]
    #[inline]
    pub fn arm_podf(&mut self) -> _ARM_PODFW {
        _ARM_PODFW { w: self }
    }
}
