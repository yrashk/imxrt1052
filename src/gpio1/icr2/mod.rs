#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR2 {
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
#[doc = "Possible values of the field `ICR16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR16R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR16R::LOW_LEVEL => 0,
            ICR16R::HIGH_LEVEL => 1,
            ICR16R::RISING_EDGE => 2,
            ICR16R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR16R {
        match value {
            0 => ICR16R::LOW_LEVEL,
            1 => ICR16R::HIGH_LEVEL,
            2 => ICR16R::RISING_EDGE,
            3 => ICR16R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR16R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR16R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR16R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR16R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR17R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR17R::LOW_LEVEL => 0,
            ICR17R::HIGH_LEVEL => 1,
            ICR17R::RISING_EDGE => 2,
            ICR17R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR17R {
        match value {
            0 => ICR17R::LOW_LEVEL,
            1 => ICR17R::HIGH_LEVEL,
            2 => ICR17R::RISING_EDGE,
            3 => ICR17R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR17R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR17R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR17R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR17R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR18R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR18R::LOW_LEVEL => 0,
            ICR18R::HIGH_LEVEL => 1,
            ICR18R::RISING_EDGE => 2,
            ICR18R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR18R {
        match value {
            0 => ICR18R::LOW_LEVEL,
            1 => ICR18R::HIGH_LEVEL,
            2 => ICR18R::RISING_EDGE,
            3 => ICR18R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR18R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR18R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR18R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR18R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR19R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR19R::LOW_LEVEL => 0,
            ICR19R::HIGH_LEVEL => 1,
            ICR19R::RISING_EDGE => 2,
            ICR19R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR19R {
        match value {
            0 => ICR19R::LOW_LEVEL,
            1 => ICR19R::HIGH_LEVEL,
            2 => ICR19R::RISING_EDGE,
            3 => ICR19R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR19R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR19R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR19R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR19R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR20R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR20R::LOW_LEVEL => 0,
            ICR20R::HIGH_LEVEL => 1,
            ICR20R::RISING_EDGE => 2,
            ICR20R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR20R {
        match value {
            0 => ICR20R::LOW_LEVEL,
            1 => ICR20R::HIGH_LEVEL,
            2 => ICR20R::RISING_EDGE,
            3 => ICR20R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR20R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR20R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR20R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR20R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR21R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR21R::LOW_LEVEL => 0,
            ICR21R::HIGH_LEVEL => 1,
            ICR21R::RISING_EDGE => 2,
            ICR21R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR21R {
        match value {
            0 => ICR21R::LOW_LEVEL,
            1 => ICR21R::HIGH_LEVEL,
            2 => ICR21R::RISING_EDGE,
            3 => ICR21R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR21R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR21R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR21R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR21R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR22R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR22R::LOW_LEVEL => 0,
            ICR22R::HIGH_LEVEL => 1,
            ICR22R::RISING_EDGE => 2,
            ICR22R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR22R {
        match value {
            0 => ICR22R::LOW_LEVEL,
            1 => ICR22R::HIGH_LEVEL,
            2 => ICR22R::RISING_EDGE,
            3 => ICR22R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR22R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR22R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR22R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR22R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR23R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR23R::LOW_LEVEL => 0,
            ICR23R::HIGH_LEVEL => 1,
            ICR23R::RISING_EDGE => 2,
            ICR23R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR23R {
        match value {
            0 => ICR23R::LOW_LEVEL,
            1 => ICR23R::HIGH_LEVEL,
            2 => ICR23R::RISING_EDGE,
            3 => ICR23R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR23R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR23R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR23R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR23R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR24R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR24R::LOW_LEVEL => 0,
            ICR24R::HIGH_LEVEL => 1,
            ICR24R::RISING_EDGE => 2,
            ICR24R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR24R {
        match value {
            0 => ICR24R::LOW_LEVEL,
            1 => ICR24R::HIGH_LEVEL,
            2 => ICR24R::RISING_EDGE,
            3 => ICR24R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR24R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR24R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR24R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR24R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR25R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR25R::LOW_LEVEL => 0,
            ICR25R::HIGH_LEVEL => 1,
            ICR25R::RISING_EDGE => 2,
            ICR25R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR25R {
        match value {
            0 => ICR25R::LOW_LEVEL,
            1 => ICR25R::HIGH_LEVEL,
            2 => ICR25R::RISING_EDGE,
            3 => ICR25R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR25R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR25R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR25R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR25R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR26R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR26R::LOW_LEVEL => 0,
            ICR26R::HIGH_LEVEL => 1,
            ICR26R::RISING_EDGE => 2,
            ICR26R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR26R {
        match value {
            0 => ICR26R::LOW_LEVEL,
            1 => ICR26R::HIGH_LEVEL,
            2 => ICR26R::RISING_EDGE,
            3 => ICR26R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR26R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR26R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR26R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR26R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR27R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR27R::LOW_LEVEL => 0,
            ICR27R::HIGH_LEVEL => 1,
            ICR27R::RISING_EDGE => 2,
            ICR27R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR27R {
        match value {
            0 => ICR27R::LOW_LEVEL,
            1 => ICR27R::HIGH_LEVEL,
            2 => ICR27R::RISING_EDGE,
            3 => ICR27R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR27R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR27R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR27R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR27R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR28R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR28R::LOW_LEVEL => 0,
            ICR28R::HIGH_LEVEL => 1,
            ICR28R::RISING_EDGE => 2,
            ICR28R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR28R {
        match value {
            0 => ICR28R::LOW_LEVEL,
            1 => ICR28R::HIGH_LEVEL,
            2 => ICR28R::RISING_EDGE,
            3 => ICR28R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR28R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR28R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR28R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR28R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR29R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR29R::LOW_LEVEL => 0,
            ICR29R::HIGH_LEVEL => 1,
            ICR29R::RISING_EDGE => 2,
            ICR29R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR29R {
        match value {
            0 => ICR29R::LOW_LEVEL,
            1 => ICR29R::HIGH_LEVEL,
            2 => ICR29R::RISING_EDGE,
            3 => ICR29R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR29R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR29R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR29R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR29R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR30R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR30R::LOW_LEVEL => 0,
            ICR30R::HIGH_LEVEL => 1,
            ICR30R::RISING_EDGE => 2,
            ICR30R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR30R {
        match value {
            0 => ICR30R::LOW_LEVEL,
            1 => ICR30R::HIGH_LEVEL,
            2 => ICR30R::RISING_EDGE,
            3 => ICR30R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR30R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR30R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR30R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR30R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR31R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR31R::LOW_LEVEL => 0,
            ICR31R::HIGH_LEVEL => 1,
            ICR31R::RISING_EDGE => 2,
            ICR31R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR31R {
        match value {
            0 => ICR31R::LOW_LEVEL,
            1 => ICR31R::HIGH_LEVEL,
            2 => ICR31R::RISING_EDGE,
            3 => ICR31R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR31R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR31R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR31R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR31R::FALLING_EDGE
    }
}
#[doc = "Values that can be written to the field `ICR16`"]
pub enum ICR16W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR16W::LOW_LEVEL => 0,
            ICR16W::HIGH_LEVEL => 1,
            ICR16W::RISING_EDGE => 2,
            ICR16W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR16W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR16W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR16W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR16W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR16W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR16W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR17`"]
pub enum ICR17W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR17W::LOW_LEVEL => 0,
            ICR17W::HIGH_LEVEL => 1,
            ICR17W::RISING_EDGE => 2,
            ICR17W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR17W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR17W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR17W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR17W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR17W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR17W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR18`"]
pub enum ICR18W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR18W::LOW_LEVEL => 0,
            ICR18W::HIGH_LEVEL => 1,
            ICR18W::RISING_EDGE => 2,
            ICR18W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR18W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR18W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR18W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR18W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR18W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR18W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR19`"]
pub enum ICR19W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR19W::LOW_LEVEL => 0,
            ICR19W::HIGH_LEVEL => 1,
            ICR19W::RISING_EDGE => 2,
            ICR19W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR19W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR19W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR19W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR19W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR19W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR19W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR20`"]
pub enum ICR20W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR20W::LOW_LEVEL => 0,
            ICR20W::HIGH_LEVEL => 1,
            ICR20W::RISING_EDGE => 2,
            ICR20W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR20W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR20W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR20W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR20W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR20W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR20W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR21`"]
pub enum ICR21W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR21W::LOW_LEVEL => 0,
            ICR21W::HIGH_LEVEL => 1,
            ICR21W::RISING_EDGE => 2,
            ICR21W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR21W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR21W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR21W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR21W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR21W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR21W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR22`"]
pub enum ICR22W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR22W::LOW_LEVEL => 0,
            ICR22W::HIGH_LEVEL => 1,
            ICR22W::RISING_EDGE => 2,
            ICR22W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR22W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR22W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR22W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR22W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR22W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR22W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR23`"]
pub enum ICR23W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR23W::LOW_LEVEL => 0,
            ICR23W::HIGH_LEVEL => 1,
            ICR23W::RISING_EDGE => 2,
            ICR23W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR23W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR23W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR23W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR23W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR23W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR24`"]
pub enum ICR24W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR24W::LOW_LEVEL => 0,
            ICR24W::HIGH_LEVEL => 1,
            ICR24W::RISING_EDGE => 2,
            ICR24W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR24W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR24W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR24W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR24W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR24W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR24W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR25`"]
pub enum ICR25W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR25W::LOW_LEVEL => 0,
            ICR25W::HIGH_LEVEL => 1,
            ICR25W::RISING_EDGE => 2,
            ICR25W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR25W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR25W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR25W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR25W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR25W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR25W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR26`"]
pub enum ICR26W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR26W::LOW_LEVEL => 0,
            ICR26W::HIGH_LEVEL => 1,
            ICR26W::RISING_EDGE => 2,
            ICR26W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR26W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR26W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR26W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR26W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR26W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR27`"]
pub enum ICR27W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR27W::LOW_LEVEL => 0,
            ICR27W::HIGH_LEVEL => 1,
            ICR27W::RISING_EDGE => 2,
            ICR27W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR27W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR27W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR27W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR27W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR27W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR27W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR28`"]
pub enum ICR28W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR28W::LOW_LEVEL => 0,
            ICR28W::HIGH_LEVEL => 1,
            ICR28W::RISING_EDGE => 2,
            ICR28W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR28W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR28W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR28W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR28W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR28W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR28W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR29`"]
pub enum ICR29W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR29W::LOW_LEVEL => 0,
            ICR29W::HIGH_LEVEL => 1,
            ICR29W::RISING_EDGE => 2,
            ICR29W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR29W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR29W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR29W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR29W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR29W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR29W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR30`"]
pub enum ICR30W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR30W::LOW_LEVEL => 0,
            ICR30W::HIGH_LEVEL => 1,
            ICR30W::RISING_EDGE => 2,
            ICR30W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR30W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR30W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR30W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR30W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR30W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR30W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICR31`"]
pub enum ICR31W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR31W::LOW_LEVEL => 0,
            ICR31W::HIGH_LEVEL => 1,
            ICR31W::RISING_EDGE => 2,
            ICR31W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR31W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR31W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR31W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR31W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR31W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR31W::FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - ICR16"]
    #[inline]
    pub fn icr16(&self) -> ICR16R {
        ICR16R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - ICR17"]
    #[inline]
    pub fn icr17(&self) -> ICR17R {
        ICR17R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - ICR18"]
    #[inline]
    pub fn icr18(&self) -> ICR18R {
        ICR18R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - ICR19"]
    #[inline]
    pub fn icr19(&self) -> ICR19R {
        ICR19R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - ICR20"]
    #[inline]
    pub fn icr20(&self) -> ICR20R {
        ICR20R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - ICR21"]
    #[inline]
    pub fn icr21(&self) -> ICR21R {
        ICR21R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - ICR22"]
    #[inline]
    pub fn icr22(&self) -> ICR22R {
        ICR22R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - ICR23"]
    #[inline]
    pub fn icr23(&self) -> ICR23R {
        ICR23R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - ICR24"]
    #[inline]
    pub fn icr24(&self) -> ICR24R {
        ICR24R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - ICR25"]
    #[inline]
    pub fn icr25(&self) -> ICR25R {
        ICR25R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - ICR26"]
    #[inline]
    pub fn icr26(&self) -> ICR26R {
        ICR26R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - ICR27"]
    #[inline]
    pub fn icr27(&self) -> ICR27R {
        ICR27R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - ICR28"]
    #[inline]
    pub fn icr28(&self) -> ICR28R {
        ICR28R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - ICR29"]
    #[inline]
    pub fn icr29(&self) -> ICR29R {
        ICR29R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - ICR30"]
    #[inline]
    pub fn icr30(&self) -> ICR30R {
        ICR30R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - ICR31"]
    #[inline]
    pub fn icr31(&self) -> ICR31R {
        ICR31R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - ICR16"]
    #[inline]
    pub fn icr16(&mut self) -> _ICR16W {
        _ICR16W { w: self }
    }
    #[doc = "Bits 2:3 - ICR17"]
    #[inline]
    pub fn icr17(&mut self) -> _ICR17W {
        _ICR17W { w: self }
    }
    #[doc = "Bits 4:5 - ICR18"]
    #[inline]
    pub fn icr18(&mut self) -> _ICR18W {
        _ICR18W { w: self }
    }
    #[doc = "Bits 6:7 - ICR19"]
    #[inline]
    pub fn icr19(&mut self) -> _ICR19W {
        _ICR19W { w: self }
    }
    #[doc = "Bits 8:9 - ICR20"]
    #[inline]
    pub fn icr20(&mut self) -> _ICR20W {
        _ICR20W { w: self }
    }
    #[doc = "Bits 10:11 - ICR21"]
    #[inline]
    pub fn icr21(&mut self) -> _ICR21W {
        _ICR21W { w: self }
    }
    #[doc = "Bits 12:13 - ICR22"]
    #[inline]
    pub fn icr22(&mut self) -> _ICR22W {
        _ICR22W { w: self }
    }
    #[doc = "Bits 14:15 - ICR23"]
    #[inline]
    pub fn icr23(&mut self) -> _ICR23W {
        _ICR23W { w: self }
    }
    #[doc = "Bits 16:17 - ICR24"]
    #[inline]
    pub fn icr24(&mut self) -> _ICR24W {
        _ICR24W { w: self }
    }
    #[doc = "Bits 18:19 - ICR25"]
    #[inline]
    pub fn icr25(&mut self) -> _ICR25W {
        _ICR25W { w: self }
    }
    #[doc = "Bits 20:21 - ICR26"]
    #[inline]
    pub fn icr26(&mut self) -> _ICR26W {
        _ICR26W { w: self }
    }
    #[doc = "Bits 22:23 - ICR27"]
    #[inline]
    pub fn icr27(&mut self) -> _ICR27W {
        _ICR27W { w: self }
    }
    #[doc = "Bits 24:25 - ICR28"]
    #[inline]
    pub fn icr28(&mut self) -> _ICR28W {
        _ICR28W { w: self }
    }
    #[doc = "Bits 26:27 - ICR29"]
    #[inline]
    pub fn icr29(&mut self) -> _ICR29W {
        _ICR29W { w: self }
    }
    #[doc = "Bits 28:29 - ICR30"]
    #[inline]
    pub fn icr30(&mut self) -> _ICR30W {
        _ICR30W { w: self }
    }
    #[doc = "Bits 30:31 - ICR31"]
    #[inline]
    pub fn icr31(&mut self) -> _ICR31W {
        _ICR31W { w: self }
    }
}
