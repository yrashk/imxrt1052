#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR1 {
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
#[doc = "Possible values of the field `ICR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR0R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR0R::LOW_LEVEL => 0,
            ICR0R::HIGH_LEVEL => 1,
            ICR0R::RISING_EDGE => 2,
            ICR0R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR0R {
        match value {
            0 => ICR0R::LOW_LEVEL,
            1 => ICR0R::HIGH_LEVEL,
            2 => ICR0R::RISING_EDGE,
            3 => ICR0R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR0R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR0R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR0R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR0R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR1R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR1R::LOW_LEVEL => 0,
            ICR1R::HIGH_LEVEL => 1,
            ICR1R::RISING_EDGE => 2,
            ICR1R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR1R {
        match value {
            0 => ICR1R::LOW_LEVEL,
            1 => ICR1R::HIGH_LEVEL,
            2 => ICR1R::RISING_EDGE,
            3 => ICR1R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR1R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR1R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR1R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR1R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR2R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR2R::LOW_LEVEL => 0,
            ICR2R::HIGH_LEVEL => 1,
            ICR2R::RISING_EDGE => 2,
            ICR2R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR2R {
        match value {
            0 => ICR2R::LOW_LEVEL,
            1 => ICR2R::HIGH_LEVEL,
            2 => ICR2R::RISING_EDGE,
            3 => ICR2R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR2R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR2R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR2R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR2R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR3R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR3R::LOW_LEVEL => 0,
            ICR3R::HIGH_LEVEL => 1,
            ICR3R::RISING_EDGE => 2,
            ICR3R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR3R {
        match value {
            0 => ICR3R::LOW_LEVEL,
            1 => ICR3R::HIGH_LEVEL,
            2 => ICR3R::RISING_EDGE,
            3 => ICR3R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR3R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR3R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR3R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR3R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR4R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR4R::LOW_LEVEL => 0,
            ICR4R::HIGH_LEVEL => 1,
            ICR4R::RISING_EDGE => 2,
            ICR4R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR4R {
        match value {
            0 => ICR4R::LOW_LEVEL,
            1 => ICR4R::HIGH_LEVEL,
            2 => ICR4R::RISING_EDGE,
            3 => ICR4R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR4R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR4R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR4R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR4R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR5R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR5R::LOW_LEVEL => 0,
            ICR5R::HIGH_LEVEL => 1,
            ICR5R::RISING_EDGE => 2,
            ICR5R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR5R {
        match value {
            0 => ICR5R::LOW_LEVEL,
            1 => ICR5R::HIGH_LEVEL,
            2 => ICR5R::RISING_EDGE,
            3 => ICR5R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR5R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR5R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR5R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR5R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR6R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR6R::LOW_LEVEL => 0,
            ICR6R::HIGH_LEVEL => 1,
            ICR6R::RISING_EDGE => 2,
            ICR6R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR6R {
        match value {
            0 => ICR6R::LOW_LEVEL,
            1 => ICR6R::HIGH_LEVEL,
            2 => ICR6R::RISING_EDGE,
            3 => ICR6R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR6R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR6R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR6R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR6R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR7R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR7R::LOW_LEVEL => 0,
            ICR7R::HIGH_LEVEL => 1,
            ICR7R::RISING_EDGE => 2,
            ICR7R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR7R {
        match value {
            0 => ICR7R::LOW_LEVEL,
            1 => ICR7R::HIGH_LEVEL,
            2 => ICR7R::RISING_EDGE,
            3 => ICR7R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR7R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR7R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR7R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR7R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR8R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR8R::LOW_LEVEL => 0,
            ICR8R::HIGH_LEVEL => 1,
            ICR8R::RISING_EDGE => 2,
            ICR8R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR8R {
        match value {
            0 => ICR8R::LOW_LEVEL,
            1 => ICR8R::HIGH_LEVEL,
            2 => ICR8R::RISING_EDGE,
            3 => ICR8R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR8R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR8R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR8R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR8R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR9R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR9R::LOW_LEVEL => 0,
            ICR9R::HIGH_LEVEL => 1,
            ICR9R::RISING_EDGE => 2,
            ICR9R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR9R {
        match value {
            0 => ICR9R::LOW_LEVEL,
            1 => ICR9R::HIGH_LEVEL,
            2 => ICR9R::RISING_EDGE,
            3 => ICR9R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR9R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR9R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR9R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR9R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR10R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR10R::LOW_LEVEL => 0,
            ICR10R::HIGH_LEVEL => 1,
            ICR10R::RISING_EDGE => 2,
            ICR10R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR10R {
        match value {
            0 => ICR10R::LOW_LEVEL,
            1 => ICR10R::HIGH_LEVEL,
            2 => ICR10R::RISING_EDGE,
            3 => ICR10R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR10R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR10R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR10R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR10R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR11R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR11R::LOW_LEVEL => 0,
            ICR11R::HIGH_LEVEL => 1,
            ICR11R::RISING_EDGE => 2,
            ICR11R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR11R {
        match value {
            0 => ICR11R::LOW_LEVEL,
            1 => ICR11R::HIGH_LEVEL,
            2 => ICR11R::RISING_EDGE,
            3 => ICR11R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR11R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR11R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR11R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR11R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR12R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR12R::LOW_LEVEL => 0,
            ICR12R::HIGH_LEVEL => 1,
            ICR12R::RISING_EDGE => 2,
            ICR12R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR12R {
        match value {
            0 => ICR12R::LOW_LEVEL,
            1 => ICR12R::HIGH_LEVEL,
            2 => ICR12R::RISING_EDGE,
            3 => ICR12R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR12R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR12R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR12R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR12R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR13R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR13R::LOW_LEVEL => 0,
            ICR13R::HIGH_LEVEL => 1,
            ICR13R::RISING_EDGE => 2,
            ICR13R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR13R {
        match value {
            0 => ICR13R::LOW_LEVEL,
            1 => ICR13R::HIGH_LEVEL,
            2 => ICR13R::RISING_EDGE,
            3 => ICR13R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR13R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR13R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR13R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR13R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR14R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR14R::LOW_LEVEL => 0,
            ICR14R::HIGH_LEVEL => 1,
            ICR14R::RISING_EDGE => 2,
            ICR14R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR14R {
        match value {
            0 => ICR14R::LOW_LEVEL,
            1 => ICR14R::HIGH_LEVEL,
            2 => ICR14R::RISING_EDGE,
            3 => ICR14R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR14R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR14R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR14R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR14R::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `ICR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR15R {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICR15R::LOW_LEVEL => 0,
            ICR15R::HIGH_LEVEL => 1,
            ICR15R::RISING_EDGE => 2,
            ICR15R::FALLING_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICR15R {
        match value {
            0 => ICR15R::LOW_LEVEL,
            1 => ICR15R::HIGH_LEVEL,
            2 => ICR15R::RISING_EDGE,
            3 => ICR15R::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline]
    pub fn is_low_level(&self) -> bool {
        *self == ICR15R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline]
    pub fn is_high_level(&self) -> bool {
        *self == ICR15R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR15R::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR15R::FALLING_EDGE
    }
}
#[doc = "Values that can be written to the field `ICR0`"]
pub enum ICR0W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR0W::LOW_LEVEL => 0,
            ICR0W::HIGH_LEVEL => 1,
            ICR0W::RISING_EDGE => 2,
            ICR0W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR0W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR0W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR0W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR0W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR1`"]
pub enum ICR1W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR1W::LOW_LEVEL => 0,
            ICR1W::HIGH_LEVEL => 1,
            ICR1W::RISING_EDGE => 2,
            ICR1W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR1W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR1W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR1W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR1W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR2`"]
pub enum ICR2W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR2W::LOW_LEVEL => 0,
            ICR2W::HIGH_LEVEL => 1,
            ICR2W::RISING_EDGE => 2,
            ICR2W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR2W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR2W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR2W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR2W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR3`"]
pub enum ICR3W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR3W::LOW_LEVEL => 0,
            ICR3W::HIGH_LEVEL => 1,
            ICR3W::RISING_EDGE => 2,
            ICR3W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR3W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR3W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR3W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR3W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR4`"]
pub enum ICR4W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR4W::LOW_LEVEL => 0,
            ICR4W::HIGH_LEVEL => 1,
            ICR4W::RISING_EDGE => 2,
            ICR4W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR4W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR4W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR4W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR4W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR4W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR5`"]
pub enum ICR5W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR5W::LOW_LEVEL => 0,
            ICR5W::HIGH_LEVEL => 1,
            ICR5W::RISING_EDGE => 2,
            ICR5W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR5W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR5W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR5W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR5W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR5W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR6`"]
pub enum ICR6W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR6W::LOW_LEVEL => 0,
            ICR6W::HIGH_LEVEL => 1,
            ICR6W::RISING_EDGE => 2,
            ICR6W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR6W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR6W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR6W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR6W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR6W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR7`"]
pub enum ICR7W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR7W::LOW_LEVEL => 0,
            ICR7W::HIGH_LEVEL => 1,
            ICR7W::RISING_EDGE => 2,
            ICR7W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR7W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR7W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR7W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR7W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR7W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR8`"]
pub enum ICR8W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR8W::LOW_LEVEL => 0,
            ICR8W::HIGH_LEVEL => 1,
            ICR8W::RISING_EDGE => 2,
            ICR8W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR8W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR8W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR8W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR8W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR8W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR9`"]
pub enum ICR9W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR9W::LOW_LEVEL => 0,
            ICR9W::HIGH_LEVEL => 1,
            ICR9W::RISING_EDGE => 2,
            ICR9W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR9W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR9W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR9W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR9W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR9W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR10`"]
pub enum ICR10W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR10W::LOW_LEVEL => 0,
            ICR10W::HIGH_LEVEL => 1,
            ICR10W::RISING_EDGE => 2,
            ICR10W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR10W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR10W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR10W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR10W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR10W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR11`"]
pub enum ICR11W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR11W::LOW_LEVEL => 0,
            ICR11W::HIGH_LEVEL => 1,
            ICR11W::RISING_EDGE => 2,
            ICR11W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR11W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR11W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR11W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR11W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR11W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR12`"]
pub enum ICR12W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR12W::LOW_LEVEL => 0,
            ICR12W::HIGH_LEVEL => 1,
            ICR12W::RISING_EDGE => 2,
            ICR12W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR12W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR12W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR12W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR12W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR12W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR13`"]
pub enum ICR13W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR13W::LOW_LEVEL => 0,
            ICR13W::HIGH_LEVEL => 1,
            ICR13W::RISING_EDGE => 2,
            ICR13W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR13W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR13W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR13W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR13W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR13W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR14`"]
pub enum ICR14W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR14W::LOW_LEVEL => 0,
            ICR14W::HIGH_LEVEL => 1,
            ICR14W::RISING_EDGE => 2,
            ICR14W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR14W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR14W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR14W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR14W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR14W::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `ICR15`"]
pub enum ICR15W {
    #[doc = "Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl ICR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICR15W::LOW_LEVEL => 0,
            ICR15W::HIGH_LEVEL => 1,
            ICR15W::RISING_EDGE => 2,
            ICR15W::FALLING_EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICR15W<'a> {
    w: &'a mut W,
}
impl<'a> _ICR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICR15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR15W::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR15W::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR15W::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR15W::FALLING_EDGE)
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
    #[doc = "Bits 0:1 - ICR0"]
    #[inline]
    pub fn icr0(&self) -> ICR0R {
        ICR0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - ICR1"]
    #[inline]
    pub fn icr1(&self) -> ICR1R {
        ICR1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - ICR2"]
    #[inline]
    pub fn icr2(&self) -> ICR2R {
        ICR2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - ICR3"]
    #[inline]
    pub fn icr3(&self) -> ICR3R {
        ICR3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - ICR4"]
    #[inline]
    pub fn icr4(&self) -> ICR4R {
        ICR4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - ICR5"]
    #[inline]
    pub fn icr5(&self) -> ICR5R {
        ICR5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - ICR6"]
    #[inline]
    pub fn icr6(&self) -> ICR6R {
        ICR6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - ICR7"]
    #[inline]
    pub fn icr7(&self) -> ICR7R {
        ICR7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - ICR8"]
    #[inline]
    pub fn icr8(&self) -> ICR8R {
        ICR8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - ICR9"]
    #[inline]
    pub fn icr9(&self) -> ICR9R {
        ICR9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - ICR10"]
    #[inline]
    pub fn icr10(&self) -> ICR10R {
        ICR10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - ICR11"]
    #[inline]
    pub fn icr11(&self) -> ICR11R {
        ICR11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - ICR12"]
    #[inline]
    pub fn icr12(&self) -> ICR12R {
        ICR12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - ICR13"]
    #[inline]
    pub fn icr13(&self) -> ICR13R {
        ICR13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - ICR14"]
    #[inline]
    pub fn icr14(&self) -> ICR14R {
        ICR14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - ICR15"]
    #[inline]
    pub fn icr15(&self) -> ICR15R {
        ICR15R::_from({
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
    #[doc = "Bits 0:1 - ICR0"]
    #[inline]
    pub fn icr0(&mut self) -> _ICR0W {
        _ICR0W { w: self }
    }
    #[doc = "Bits 2:3 - ICR1"]
    #[inline]
    pub fn icr1(&mut self) -> _ICR1W {
        _ICR1W { w: self }
    }
    #[doc = "Bits 4:5 - ICR2"]
    #[inline]
    pub fn icr2(&mut self) -> _ICR2W {
        _ICR2W { w: self }
    }
    #[doc = "Bits 6:7 - ICR3"]
    #[inline]
    pub fn icr3(&mut self) -> _ICR3W {
        _ICR3W { w: self }
    }
    #[doc = "Bits 8:9 - ICR4"]
    #[inline]
    pub fn icr4(&mut self) -> _ICR4W {
        _ICR4W { w: self }
    }
    #[doc = "Bits 10:11 - ICR5"]
    #[inline]
    pub fn icr5(&mut self) -> _ICR5W {
        _ICR5W { w: self }
    }
    #[doc = "Bits 12:13 - ICR6"]
    #[inline]
    pub fn icr6(&mut self) -> _ICR6W {
        _ICR6W { w: self }
    }
    #[doc = "Bits 14:15 - ICR7"]
    #[inline]
    pub fn icr7(&mut self) -> _ICR7W {
        _ICR7W { w: self }
    }
    #[doc = "Bits 16:17 - ICR8"]
    #[inline]
    pub fn icr8(&mut self) -> _ICR8W {
        _ICR8W { w: self }
    }
    #[doc = "Bits 18:19 - ICR9"]
    #[inline]
    pub fn icr9(&mut self) -> _ICR9W {
        _ICR9W { w: self }
    }
    #[doc = "Bits 20:21 - ICR10"]
    #[inline]
    pub fn icr10(&mut self) -> _ICR10W {
        _ICR10W { w: self }
    }
    #[doc = "Bits 22:23 - ICR11"]
    #[inline]
    pub fn icr11(&mut self) -> _ICR11W {
        _ICR11W { w: self }
    }
    #[doc = "Bits 24:25 - ICR12"]
    #[inline]
    pub fn icr12(&mut self) -> _ICR12W {
        _ICR12W { w: self }
    }
    #[doc = "Bits 26:27 - ICR13"]
    #[inline]
    pub fn icr13(&mut self) -> _ICR13W {
        _ICR13W { w: self }
    }
    #[doc = "Bits 28:29 - ICR14"]
    #[inline]
    pub fn icr14(&mut self) -> _ICR14W {
        _ICR14W { w: self }
    }
    #[doc = "Bits 30:31 - ICR15"]
    #[inline]
    pub fn icr15(&mut self) -> _ICR15W {
        _ICR15W { w: self }
    }
}
