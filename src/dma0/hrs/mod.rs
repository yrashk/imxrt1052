#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HRS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HRS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS0R {
    #[doc = "A hardware service request for channel 0 is not present"]
    HRS0_0,
    #[doc = "A hardware service request for channel 0 is present"]
    HRS0_1,
}
impl HRS0R {
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
            HRS0R::HRS0_0 => false,
            HRS0R::HRS0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS0R {
        match value {
            false => HRS0R::HRS0_0,
            true => HRS0R::HRS0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS0_0`"]
    #[inline]
    pub fn is_hrs0_0(&self) -> bool {
        *self == HRS0R::HRS0_0
    }
    #[doc = "Checks if the value of the field is `HRS0_1`"]
    #[inline]
    pub fn is_hrs0_1(&self) -> bool {
        *self == HRS0R::HRS0_1
    }
}
#[doc = "Possible values of the field `HRS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS1R {
    #[doc = "A hardware service request for channel 1 is not present"]
    HRS1_0,
    #[doc = "A hardware service request for channel 1 is present"]
    HRS1_1,
}
impl HRS1R {
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
            HRS1R::HRS1_0 => false,
            HRS1R::HRS1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS1R {
        match value {
            false => HRS1R::HRS1_0,
            true => HRS1R::HRS1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS1_0`"]
    #[inline]
    pub fn is_hrs1_0(&self) -> bool {
        *self == HRS1R::HRS1_0
    }
    #[doc = "Checks if the value of the field is `HRS1_1`"]
    #[inline]
    pub fn is_hrs1_1(&self) -> bool {
        *self == HRS1R::HRS1_1
    }
}
#[doc = "Possible values of the field `HRS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS2R {
    #[doc = "A hardware service request for channel 2 is not present"]
    HRS2_0,
    #[doc = "A hardware service request for channel 2 is present"]
    HRS2_1,
}
impl HRS2R {
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
            HRS2R::HRS2_0 => false,
            HRS2R::HRS2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS2R {
        match value {
            false => HRS2R::HRS2_0,
            true => HRS2R::HRS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS2_0`"]
    #[inline]
    pub fn is_hrs2_0(&self) -> bool {
        *self == HRS2R::HRS2_0
    }
    #[doc = "Checks if the value of the field is `HRS2_1`"]
    #[inline]
    pub fn is_hrs2_1(&self) -> bool {
        *self == HRS2R::HRS2_1
    }
}
#[doc = "Possible values of the field `HRS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS3R {
    #[doc = "A hardware service request for channel 3 is not present"]
    HRS3_0,
    #[doc = "A hardware service request for channel 3 is present"]
    HRS3_1,
}
impl HRS3R {
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
            HRS3R::HRS3_0 => false,
            HRS3R::HRS3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS3R {
        match value {
            false => HRS3R::HRS3_0,
            true => HRS3R::HRS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS3_0`"]
    #[inline]
    pub fn is_hrs3_0(&self) -> bool {
        *self == HRS3R::HRS3_0
    }
    #[doc = "Checks if the value of the field is `HRS3_1`"]
    #[inline]
    pub fn is_hrs3_1(&self) -> bool {
        *self == HRS3R::HRS3_1
    }
}
#[doc = "Possible values of the field `HRS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS4R {
    #[doc = "A hardware service request for channel 4 is not present"]
    HRS4_0,
    #[doc = "A hardware service request for channel 4 is present"]
    HRS4_1,
}
impl HRS4R {
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
            HRS4R::HRS4_0 => false,
            HRS4R::HRS4_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS4R {
        match value {
            false => HRS4R::HRS4_0,
            true => HRS4R::HRS4_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS4_0`"]
    #[inline]
    pub fn is_hrs4_0(&self) -> bool {
        *self == HRS4R::HRS4_0
    }
    #[doc = "Checks if the value of the field is `HRS4_1`"]
    #[inline]
    pub fn is_hrs4_1(&self) -> bool {
        *self == HRS4R::HRS4_1
    }
}
#[doc = "Possible values of the field `HRS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS5R {
    #[doc = "A hardware service request for channel 5 is not present"]
    HRS5_0,
    #[doc = "A hardware service request for channel 5 is present"]
    HRS5_1,
}
impl HRS5R {
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
            HRS5R::HRS5_0 => false,
            HRS5R::HRS5_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS5R {
        match value {
            false => HRS5R::HRS5_0,
            true => HRS5R::HRS5_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS5_0`"]
    #[inline]
    pub fn is_hrs5_0(&self) -> bool {
        *self == HRS5R::HRS5_0
    }
    #[doc = "Checks if the value of the field is `HRS5_1`"]
    #[inline]
    pub fn is_hrs5_1(&self) -> bool {
        *self == HRS5R::HRS5_1
    }
}
#[doc = "Possible values of the field `HRS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS6R {
    #[doc = "A hardware service request for channel 6 is not present"]
    HRS6_0,
    #[doc = "A hardware service request for channel 6 is present"]
    HRS6_1,
}
impl HRS6R {
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
            HRS6R::HRS6_0 => false,
            HRS6R::HRS6_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS6R {
        match value {
            false => HRS6R::HRS6_0,
            true => HRS6R::HRS6_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS6_0`"]
    #[inline]
    pub fn is_hrs6_0(&self) -> bool {
        *self == HRS6R::HRS6_0
    }
    #[doc = "Checks if the value of the field is `HRS6_1`"]
    #[inline]
    pub fn is_hrs6_1(&self) -> bool {
        *self == HRS6R::HRS6_1
    }
}
#[doc = "Possible values of the field `HRS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS7R {
    #[doc = "A hardware service request for channel 7 is not present"]
    HRS7_0,
    #[doc = "A hardware service request for channel 7 is present"]
    HRS7_1,
}
impl HRS7R {
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
            HRS7R::HRS7_0 => false,
            HRS7R::HRS7_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS7R {
        match value {
            false => HRS7R::HRS7_0,
            true => HRS7R::HRS7_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS7_0`"]
    #[inline]
    pub fn is_hrs7_0(&self) -> bool {
        *self == HRS7R::HRS7_0
    }
    #[doc = "Checks if the value of the field is `HRS7_1`"]
    #[inline]
    pub fn is_hrs7_1(&self) -> bool {
        *self == HRS7R::HRS7_1
    }
}
#[doc = "Possible values of the field `HRS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS8R {
    #[doc = "A hardware service request for channel 8 is not present"]
    HRS8_0,
    #[doc = "A hardware service request for channel 8 is present"]
    HRS8_1,
}
impl HRS8R {
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
            HRS8R::HRS8_0 => false,
            HRS8R::HRS8_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS8R {
        match value {
            false => HRS8R::HRS8_0,
            true => HRS8R::HRS8_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS8_0`"]
    #[inline]
    pub fn is_hrs8_0(&self) -> bool {
        *self == HRS8R::HRS8_0
    }
    #[doc = "Checks if the value of the field is `HRS8_1`"]
    #[inline]
    pub fn is_hrs8_1(&self) -> bool {
        *self == HRS8R::HRS8_1
    }
}
#[doc = "Possible values of the field `HRS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS9R {
    #[doc = "A hardware service request for channel 9 is not present"]
    HRS9_0,
    #[doc = "A hardware service request for channel 9 is present"]
    HRS9_1,
}
impl HRS9R {
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
            HRS9R::HRS9_0 => false,
            HRS9R::HRS9_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS9R {
        match value {
            false => HRS9R::HRS9_0,
            true => HRS9R::HRS9_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS9_0`"]
    #[inline]
    pub fn is_hrs9_0(&self) -> bool {
        *self == HRS9R::HRS9_0
    }
    #[doc = "Checks if the value of the field is `HRS9_1`"]
    #[inline]
    pub fn is_hrs9_1(&self) -> bool {
        *self == HRS9R::HRS9_1
    }
}
#[doc = "Possible values of the field `HRS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS10R {
    #[doc = "A hardware service request for channel 10 is not present"]
    HRS10_0,
    #[doc = "A hardware service request for channel 10 is present"]
    HRS10_1,
}
impl HRS10R {
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
            HRS10R::HRS10_0 => false,
            HRS10R::HRS10_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS10R {
        match value {
            false => HRS10R::HRS10_0,
            true => HRS10R::HRS10_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS10_0`"]
    #[inline]
    pub fn is_hrs10_0(&self) -> bool {
        *self == HRS10R::HRS10_0
    }
    #[doc = "Checks if the value of the field is `HRS10_1`"]
    #[inline]
    pub fn is_hrs10_1(&self) -> bool {
        *self == HRS10R::HRS10_1
    }
}
#[doc = "Possible values of the field `HRS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS11R {
    #[doc = "A hardware service request for channel 11 is not present"]
    HRS11_0,
    #[doc = "A hardware service request for channel 11 is present"]
    HRS11_1,
}
impl HRS11R {
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
            HRS11R::HRS11_0 => false,
            HRS11R::HRS11_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS11R {
        match value {
            false => HRS11R::HRS11_0,
            true => HRS11R::HRS11_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS11_0`"]
    #[inline]
    pub fn is_hrs11_0(&self) -> bool {
        *self == HRS11R::HRS11_0
    }
    #[doc = "Checks if the value of the field is `HRS11_1`"]
    #[inline]
    pub fn is_hrs11_1(&self) -> bool {
        *self == HRS11R::HRS11_1
    }
}
#[doc = "Possible values of the field `HRS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS12R {
    #[doc = "A hardware service request for channel 12 is not present"]
    HRS12_0,
    #[doc = "A hardware service request for channel 12 is present"]
    HRS12_1,
}
impl HRS12R {
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
            HRS12R::HRS12_0 => false,
            HRS12R::HRS12_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS12R {
        match value {
            false => HRS12R::HRS12_0,
            true => HRS12R::HRS12_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS12_0`"]
    #[inline]
    pub fn is_hrs12_0(&self) -> bool {
        *self == HRS12R::HRS12_0
    }
    #[doc = "Checks if the value of the field is `HRS12_1`"]
    #[inline]
    pub fn is_hrs12_1(&self) -> bool {
        *self == HRS12R::HRS12_1
    }
}
#[doc = "Possible values of the field `HRS13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS13R {
    #[doc = "A hardware service request for channel 13 is not present"]
    HRS13_0,
    #[doc = "A hardware service request for channel 13 is present"]
    HRS13_1,
}
impl HRS13R {
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
            HRS13R::HRS13_0 => false,
            HRS13R::HRS13_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS13R {
        match value {
            false => HRS13R::HRS13_0,
            true => HRS13R::HRS13_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS13_0`"]
    #[inline]
    pub fn is_hrs13_0(&self) -> bool {
        *self == HRS13R::HRS13_0
    }
    #[doc = "Checks if the value of the field is `HRS13_1`"]
    #[inline]
    pub fn is_hrs13_1(&self) -> bool {
        *self == HRS13R::HRS13_1
    }
}
#[doc = "Possible values of the field `HRS14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS14R {
    #[doc = "A hardware service request for channel 14 is not present"]
    HRS14_0,
    #[doc = "A hardware service request for channel 14 is present"]
    HRS14_1,
}
impl HRS14R {
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
            HRS14R::HRS14_0 => false,
            HRS14R::HRS14_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS14R {
        match value {
            false => HRS14R::HRS14_0,
            true => HRS14R::HRS14_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS14_0`"]
    #[inline]
    pub fn is_hrs14_0(&self) -> bool {
        *self == HRS14R::HRS14_0
    }
    #[doc = "Checks if the value of the field is `HRS14_1`"]
    #[inline]
    pub fn is_hrs14_1(&self) -> bool {
        *self == HRS14R::HRS14_1
    }
}
#[doc = "Possible values of the field `HRS15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS15R {
    #[doc = "A hardware service request for channel 15 is not present"]
    HRS15_0,
    #[doc = "A hardware service request for channel 15 is present"]
    HRS15_1,
}
impl HRS15R {
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
            HRS15R::HRS15_0 => false,
            HRS15R::HRS15_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS15R {
        match value {
            false => HRS15R::HRS15_0,
            true => HRS15R::HRS15_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS15_0`"]
    #[inline]
    pub fn is_hrs15_0(&self) -> bool {
        *self == HRS15R::HRS15_0
    }
    #[doc = "Checks if the value of the field is `HRS15_1`"]
    #[inline]
    pub fn is_hrs15_1(&self) -> bool {
        *self == HRS15R::HRS15_1
    }
}
#[doc = "Possible values of the field `HRS16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS16R {
    #[doc = "A hardware service request for channel 16 is not present"]
    HRS16_0,
    #[doc = "A hardware service request for channel 16 is present"]
    HRS16_1,
}
impl HRS16R {
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
            HRS16R::HRS16_0 => false,
            HRS16R::HRS16_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS16R {
        match value {
            false => HRS16R::HRS16_0,
            true => HRS16R::HRS16_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS16_0`"]
    #[inline]
    pub fn is_hrs16_0(&self) -> bool {
        *self == HRS16R::HRS16_0
    }
    #[doc = "Checks if the value of the field is `HRS16_1`"]
    #[inline]
    pub fn is_hrs16_1(&self) -> bool {
        *self == HRS16R::HRS16_1
    }
}
#[doc = "Possible values of the field `HRS17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS17R {
    #[doc = "A hardware service request for channel 17 is not present"]
    HRS17_0,
    #[doc = "A hardware service request for channel 17 is present"]
    HRS17_1,
}
impl HRS17R {
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
            HRS17R::HRS17_0 => false,
            HRS17R::HRS17_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS17R {
        match value {
            false => HRS17R::HRS17_0,
            true => HRS17R::HRS17_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS17_0`"]
    #[inline]
    pub fn is_hrs17_0(&self) -> bool {
        *self == HRS17R::HRS17_0
    }
    #[doc = "Checks if the value of the field is `HRS17_1`"]
    #[inline]
    pub fn is_hrs17_1(&self) -> bool {
        *self == HRS17R::HRS17_1
    }
}
#[doc = "Possible values of the field `HRS18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS18R {
    #[doc = "A hardware service request for channel 18 is not present"]
    HRS18_0,
    #[doc = "A hardware service request for channel 18 is present"]
    HRS18_1,
}
impl HRS18R {
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
            HRS18R::HRS18_0 => false,
            HRS18R::HRS18_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS18R {
        match value {
            false => HRS18R::HRS18_0,
            true => HRS18R::HRS18_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS18_0`"]
    #[inline]
    pub fn is_hrs18_0(&self) -> bool {
        *self == HRS18R::HRS18_0
    }
    #[doc = "Checks if the value of the field is `HRS18_1`"]
    #[inline]
    pub fn is_hrs18_1(&self) -> bool {
        *self == HRS18R::HRS18_1
    }
}
#[doc = "Possible values of the field `HRS19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS19R {
    #[doc = "A hardware service request for channel 19 is not present"]
    HRS19_0,
    #[doc = "A hardware service request for channel 19 is present"]
    HRS19_1,
}
impl HRS19R {
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
            HRS19R::HRS19_0 => false,
            HRS19R::HRS19_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS19R {
        match value {
            false => HRS19R::HRS19_0,
            true => HRS19R::HRS19_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS19_0`"]
    #[inline]
    pub fn is_hrs19_0(&self) -> bool {
        *self == HRS19R::HRS19_0
    }
    #[doc = "Checks if the value of the field is `HRS19_1`"]
    #[inline]
    pub fn is_hrs19_1(&self) -> bool {
        *self == HRS19R::HRS19_1
    }
}
#[doc = "Possible values of the field `HRS20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS20R {
    #[doc = "A hardware service request for channel 20 is not present"]
    HRS20_0,
    #[doc = "A hardware service request for channel 20 is present"]
    HRS20_1,
}
impl HRS20R {
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
            HRS20R::HRS20_0 => false,
            HRS20R::HRS20_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS20R {
        match value {
            false => HRS20R::HRS20_0,
            true => HRS20R::HRS20_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS20_0`"]
    #[inline]
    pub fn is_hrs20_0(&self) -> bool {
        *self == HRS20R::HRS20_0
    }
    #[doc = "Checks if the value of the field is `HRS20_1`"]
    #[inline]
    pub fn is_hrs20_1(&self) -> bool {
        *self == HRS20R::HRS20_1
    }
}
#[doc = "Possible values of the field `HRS21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS21R {
    #[doc = "A hardware service request for channel 21 is not present"]
    HRS21_0,
    #[doc = "A hardware service request for channel 21 is present"]
    HRS21_1,
}
impl HRS21R {
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
            HRS21R::HRS21_0 => false,
            HRS21R::HRS21_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS21R {
        match value {
            false => HRS21R::HRS21_0,
            true => HRS21R::HRS21_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS21_0`"]
    #[inline]
    pub fn is_hrs21_0(&self) -> bool {
        *self == HRS21R::HRS21_0
    }
    #[doc = "Checks if the value of the field is `HRS21_1`"]
    #[inline]
    pub fn is_hrs21_1(&self) -> bool {
        *self == HRS21R::HRS21_1
    }
}
#[doc = "Possible values of the field `HRS22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS22R {
    #[doc = "A hardware service request for channel 22 is not present"]
    HRS22_0,
    #[doc = "A hardware service request for channel 22 is present"]
    HRS22_1,
}
impl HRS22R {
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
            HRS22R::HRS22_0 => false,
            HRS22R::HRS22_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS22R {
        match value {
            false => HRS22R::HRS22_0,
            true => HRS22R::HRS22_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS22_0`"]
    #[inline]
    pub fn is_hrs22_0(&self) -> bool {
        *self == HRS22R::HRS22_0
    }
    #[doc = "Checks if the value of the field is `HRS22_1`"]
    #[inline]
    pub fn is_hrs22_1(&self) -> bool {
        *self == HRS22R::HRS22_1
    }
}
#[doc = "Possible values of the field `HRS23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS23R {
    #[doc = "A hardware service request for channel 23 is not present"]
    HRS23_0,
    #[doc = "A hardware service request for channel 23 is present"]
    HRS23_1,
}
impl HRS23R {
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
            HRS23R::HRS23_0 => false,
            HRS23R::HRS23_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS23R {
        match value {
            false => HRS23R::HRS23_0,
            true => HRS23R::HRS23_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS23_0`"]
    #[inline]
    pub fn is_hrs23_0(&self) -> bool {
        *self == HRS23R::HRS23_0
    }
    #[doc = "Checks if the value of the field is `HRS23_1`"]
    #[inline]
    pub fn is_hrs23_1(&self) -> bool {
        *self == HRS23R::HRS23_1
    }
}
#[doc = "Possible values of the field `HRS24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS24R {
    #[doc = "A hardware service request for channel 24 is not present"]
    HRS24_0,
    #[doc = "A hardware service request for channel 24 is present"]
    HRS24_1,
}
impl HRS24R {
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
            HRS24R::HRS24_0 => false,
            HRS24R::HRS24_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS24R {
        match value {
            false => HRS24R::HRS24_0,
            true => HRS24R::HRS24_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS24_0`"]
    #[inline]
    pub fn is_hrs24_0(&self) -> bool {
        *self == HRS24R::HRS24_0
    }
    #[doc = "Checks if the value of the field is `HRS24_1`"]
    #[inline]
    pub fn is_hrs24_1(&self) -> bool {
        *self == HRS24R::HRS24_1
    }
}
#[doc = "Possible values of the field `HRS25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS25R {
    #[doc = "A hardware service request for channel 25 is not present"]
    HRS25_0,
    #[doc = "A hardware service request for channel 25 is present"]
    HRS25_1,
}
impl HRS25R {
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
            HRS25R::HRS25_0 => false,
            HRS25R::HRS25_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS25R {
        match value {
            false => HRS25R::HRS25_0,
            true => HRS25R::HRS25_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS25_0`"]
    #[inline]
    pub fn is_hrs25_0(&self) -> bool {
        *self == HRS25R::HRS25_0
    }
    #[doc = "Checks if the value of the field is `HRS25_1`"]
    #[inline]
    pub fn is_hrs25_1(&self) -> bool {
        *self == HRS25R::HRS25_1
    }
}
#[doc = "Possible values of the field `HRS26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS26R {
    #[doc = "A hardware service request for channel 26 is not present"]
    HRS26_0,
    #[doc = "A hardware service request for channel 26 is present"]
    HRS26_1,
}
impl HRS26R {
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
            HRS26R::HRS26_0 => false,
            HRS26R::HRS26_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS26R {
        match value {
            false => HRS26R::HRS26_0,
            true => HRS26R::HRS26_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS26_0`"]
    #[inline]
    pub fn is_hrs26_0(&self) -> bool {
        *self == HRS26R::HRS26_0
    }
    #[doc = "Checks if the value of the field is `HRS26_1`"]
    #[inline]
    pub fn is_hrs26_1(&self) -> bool {
        *self == HRS26R::HRS26_1
    }
}
#[doc = "Possible values of the field `HRS27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS27R {
    #[doc = "A hardware service request for channel 27 is not present"]
    HRS27_0,
    #[doc = "A hardware service request for channel 27 is present"]
    HRS27_1,
}
impl HRS27R {
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
            HRS27R::HRS27_0 => false,
            HRS27R::HRS27_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS27R {
        match value {
            false => HRS27R::HRS27_0,
            true => HRS27R::HRS27_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS27_0`"]
    #[inline]
    pub fn is_hrs27_0(&self) -> bool {
        *self == HRS27R::HRS27_0
    }
    #[doc = "Checks if the value of the field is `HRS27_1`"]
    #[inline]
    pub fn is_hrs27_1(&self) -> bool {
        *self == HRS27R::HRS27_1
    }
}
#[doc = "Possible values of the field `HRS28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS28R {
    #[doc = "A hardware service request for channel 28 is not present"]
    HRS28_0,
    #[doc = "A hardware service request for channel 28 is present"]
    HRS28_1,
}
impl HRS28R {
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
            HRS28R::HRS28_0 => false,
            HRS28R::HRS28_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS28R {
        match value {
            false => HRS28R::HRS28_0,
            true => HRS28R::HRS28_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS28_0`"]
    #[inline]
    pub fn is_hrs28_0(&self) -> bool {
        *self == HRS28R::HRS28_0
    }
    #[doc = "Checks if the value of the field is `HRS28_1`"]
    #[inline]
    pub fn is_hrs28_1(&self) -> bool {
        *self == HRS28R::HRS28_1
    }
}
#[doc = "Possible values of the field `HRS29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS29R {
    #[doc = "A hardware service request for channel 29 is not preset"]
    HRS29_0,
    #[doc = "A hardware service request for channel 29 is present"]
    HRS29_1,
}
impl HRS29R {
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
            HRS29R::HRS29_0 => false,
            HRS29R::HRS29_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS29R {
        match value {
            false => HRS29R::HRS29_0,
            true => HRS29R::HRS29_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS29_0`"]
    #[inline]
    pub fn is_hrs29_0(&self) -> bool {
        *self == HRS29R::HRS29_0
    }
    #[doc = "Checks if the value of the field is `HRS29_1`"]
    #[inline]
    pub fn is_hrs29_1(&self) -> bool {
        *self == HRS29R::HRS29_1
    }
}
#[doc = "Possible values of the field `HRS30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS30R {
    #[doc = "A hardware service request for channel 30 is not present"]
    HRS30_0,
    #[doc = "A hardware service request for channel 30 is present"]
    HRS30_1,
}
impl HRS30R {
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
            HRS30R::HRS30_0 => false,
            HRS30R::HRS30_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS30R {
        match value {
            false => HRS30R::HRS30_0,
            true => HRS30R::HRS30_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS30_0`"]
    #[inline]
    pub fn is_hrs30_0(&self) -> bool {
        *self == HRS30R::HRS30_0
    }
    #[doc = "Checks if the value of the field is `HRS30_1`"]
    #[inline]
    pub fn is_hrs30_1(&self) -> bool {
        *self == HRS30R::HRS30_1
    }
}
#[doc = "Possible values of the field `HRS31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS31R {
    #[doc = "A hardware service request for channel 31 is not present"]
    HRS31_0,
    #[doc = "A hardware service request for channel 31 is present"]
    HRS31_1,
}
impl HRS31R {
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
            HRS31R::HRS31_0 => false,
            HRS31R::HRS31_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS31R {
        match value {
            false => HRS31R::HRS31_0,
            true => HRS31R::HRS31_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS31_0`"]
    #[inline]
    pub fn is_hrs31_0(&self) -> bool {
        *self == HRS31R::HRS31_0
    }
    #[doc = "Checks if the value of the field is `HRS31_1`"]
    #[inline]
    pub fn is_hrs31_1(&self) -> bool {
        *self == HRS31R::HRS31_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline]
    pub fn hrs0(&self) -> HRS0R {
        HRS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline]
    pub fn hrs1(&self) -> HRS1R {
        HRS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline]
    pub fn hrs2(&self) -> HRS2R {
        HRS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline]
    pub fn hrs3(&self) -> HRS3R {
        HRS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline]
    pub fn hrs4(&self) -> HRS4R {
        HRS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline]
    pub fn hrs5(&self) -> HRS5R {
        HRS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline]
    pub fn hrs6(&self) -> HRS6R {
        HRS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline]
    pub fn hrs7(&self) -> HRS7R {
        HRS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline]
    pub fn hrs8(&self) -> HRS8R {
        HRS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline]
    pub fn hrs9(&self) -> HRS9R {
        HRS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline]
    pub fn hrs10(&self) -> HRS10R {
        HRS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline]
    pub fn hrs11(&self) -> HRS11R {
        HRS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline]
    pub fn hrs12(&self) -> HRS12R {
        HRS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline]
    pub fn hrs13(&self) -> HRS13R {
        HRS13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline]
    pub fn hrs14(&self) -> HRS14R {
        HRS14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline]
    pub fn hrs15(&self) -> HRS15R {
        HRS15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Hardware Request Status Channel 16"]
    #[inline]
    pub fn hrs16(&self) -> HRS16R {
        HRS16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Hardware Request Status Channel 17"]
    #[inline]
    pub fn hrs17(&self) -> HRS17R {
        HRS17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Hardware Request Status Channel 18"]
    #[inline]
    pub fn hrs18(&self) -> HRS18R {
        HRS18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Hardware Request Status Channel 19"]
    #[inline]
    pub fn hrs19(&self) -> HRS19R {
        HRS19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Hardware Request Status Channel 20"]
    #[inline]
    pub fn hrs20(&self) -> HRS20R {
        HRS20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Hardware Request Status Channel 21"]
    #[inline]
    pub fn hrs21(&self) -> HRS21R {
        HRS21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Hardware Request Status Channel 22"]
    #[inline]
    pub fn hrs22(&self) -> HRS22R {
        HRS22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Hardware Request Status Channel 23"]
    #[inline]
    pub fn hrs23(&self) -> HRS23R {
        HRS23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Hardware Request Status Channel 24"]
    #[inline]
    pub fn hrs24(&self) -> HRS24R {
        HRS24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Hardware Request Status Channel 25"]
    #[inline]
    pub fn hrs25(&self) -> HRS25R {
        HRS25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Hardware Request Status Channel 26"]
    #[inline]
    pub fn hrs26(&self) -> HRS26R {
        HRS26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Hardware Request Status Channel 27"]
    #[inline]
    pub fn hrs27(&self) -> HRS27R {
        HRS27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Hardware Request Status Channel 28"]
    #[inline]
    pub fn hrs28(&self) -> HRS28R {
        HRS28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Hardware Request Status Channel 29"]
    #[inline]
    pub fn hrs29(&self) -> HRS29R {
        HRS29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Hardware Request Status Channel 30"]
    #[inline]
    pub fn hrs30(&self) -> HRS30R {
        HRS30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Hardware Request Status Channel 31"]
    #[inline]
    pub fn hrs31(&self) -> HRS31R {
        HRS31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
