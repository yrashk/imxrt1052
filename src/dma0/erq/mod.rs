#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERQ {
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
#[doc = "Possible values of the field `ERQ0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ0R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ0_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ0_1,
}
impl ERQ0R {
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
            ERQ0R::ERQ0_0 => false,
            ERQ0R::ERQ0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ0R {
        match value {
            false => ERQ0R::ERQ0_0,
            true => ERQ0R::ERQ0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ0_0`"]
    #[inline]
    pub fn is_erq0_0(&self) -> bool {
        *self == ERQ0R::ERQ0_0
    }
    #[doc = "Checks if the value of the field is `ERQ0_1`"]
    #[inline]
    pub fn is_erq0_1(&self) -> bool {
        *self == ERQ0R::ERQ0_1
    }
}
#[doc = "Possible values of the field `ERQ1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ1R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ1_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ1_1,
}
impl ERQ1R {
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
            ERQ1R::ERQ1_0 => false,
            ERQ1R::ERQ1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ1R {
        match value {
            false => ERQ1R::ERQ1_0,
            true => ERQ1R::ERQ1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ1_0`"]
    #[inline]
    pub fn is_erq1_0(&self) -> bool {
        *self == ERQ1R::ERQ1_0
    }
    #[doc = "Checks if the value of the field is `ERQ1_1`"]
    #[inline]
    pub fn is_erq1_1(&self) -> bool {
        *self == ERQ1R::ERQ1_1
    }
}
#[doc = "Possible values of the field `ERQ2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ2R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ2_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ2_1,
}
impl ERQ2R {
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
            ERQ2R::ERQ2_0 => false,
            ERQ2R::ERQ2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ2R {
        match value {
            false => ERQ2R::ERQ2_0,
            true => ERQ2R::ERQ2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ2_0`"]
    #[inline]
    pub fn is_erq2_0(&self) -> bool {
        *self == ERQ2R::ERQ2_0
    }
    #[doc = "Checks if the value of the field is `ERQ2_1`"]
    #[inline]
    pub fn is_erq2_1(&self) -> bool {
        *self == ERQ2R::ERQ2_1
    }
}
#[doc = "Possible values of the field `ERQ3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ3R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ3_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ3_1,
}
impl ERQ3R {
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
            ERQ3R::ERQ3_0 => false,
            ERQ3R::ERQ3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ3R {
        match value {
            false => ERQ3R::ERQ3_0,
            true => ERQ3R::ERQ3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ3_0`"]
    #[inline]
    pub fn is_erq3_0(&self) -> bool {
        *self == ERQ3R::ERQ3_0
    }
    #[doc = "Checks if the value of the field is `ERQ3_1`"]
    #[inline]
    pub fn is_erq3_1(&self) -> bool {
        *self == ERQ3R::ERQ3_1
    }
}
#[doc = "Possible values of the field `ERQ4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ4R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ4_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ4_1,
}
impl ERQ4R {
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
            ERQ4R::ERQ4_0 => false,
            ERQ4R::ERQ4_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ4R {
        match value {
            false => ERQ4R::ERQ4_0,
            true => ERQ4R::ERQ4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ4_0`"]
    #[inline]
    pub fn is_erq4_0(&self) -> bool {
        *self == ERQ4R::ERQ4_0
    }
    #[doc = "Checks if the value of the field is `ERQ4_1`"]
    #[inline]
    pub fn is_erq4_1(&self) -> bool {
        *self == ERQ4R::ERQ4_1
    }
}
#[doc = "Possible values of the field `ERQ5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ5R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ5_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ5_1,
}
impl ERQ5R {
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
            ERQ5R::ERQ5_0 => false,
            ERQ5R::ERQ5_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ5R {
        match value {
            false => ERQ5R::ERQ5_0,
            true => ERQ5R::ERQ5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ5_0`"]
    #[inline]
    pub fn is_erq5_0(&self) -> bool {
        *self == ERQ5R::ERQ5_0
    }
    #[doc = "Checks if the value of the field is `ERQ5_1`"]
    #[inline]
    pub fn is_erq5_1(&self) -> bool {
        *self == ERQ5R::ERQ5_1
    }
}
#[doc = "Possible values of the field `ERQ6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ6R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ6_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ6_1,
}
impl ERQ6R {
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
            ERQ6R::ERQ6_0 => false,
            ERQ6R::ERQ6_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ6R {
        match value {
            false => ERQ6R::ERQ6_0,
            true => ERQ6R::ERQ6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ6_0`"]
    #[inline]
    pub fn is_erq6_0(&self) -> bool {
        *self == ERQ6R::ERQ6_0
    }
    #[doc = "Checks if the value of the field is `ERQ6_1`"]
    #[inline]
    pub fn is_erq6_1(&self) -> bool {
        *self == ERQ6R::ERQ6_1
    }
}
#[doc = "Possible values of the field `ERQ7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ7R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ7_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ7_1,
}
impl ERQ7R {
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
            ERQ7R::ERQ7_0 => false,
            ERQ7R::ERQ7_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ7R {
        match value {
            false => ERQ7R::ERQ7_0,
            true => ERQ7R::ERQ7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ7_0`"]
    #[inline]
    pub fn is_erq7_0(&self) -> bool {
        *self == ERQ7R::ERQ7_0
    }
    #[doc = "Checks if the value of the field is `ERQ7_1`"]
    #[inline]
    pub fn is_erq7_1(&self) -> bool {
        *self == ERQ7R::ERQ7_1
    }
}
#[doc = "Possible values of the field `ERQ8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ8R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ8_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ8_1,
}
impl ERQ8R {
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
            ERQ8R::ERQ8_0 => false,
            ERQ8R::ERQ8_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ8R {
        match value {
            false => ERQ8R::ERQ8_0,
            true => ERQ8R::ERQ8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ8_0`"]
    #[inline]
    pub fn is_erq8_0(&self) -> bool {
        *self == ERQ8R::ERQ8_0
    }
    #[doc = "Checks if the value of the field is `ERQ8_1`"]
    #[inline]
    pub fn is_erq8_1(&self) -> bool {
        *self == ERQ8R::ERQ8_1
    }
}
#[doc = "Possible values of the field `ERQ9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ9R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ9_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ9_1,
}
impl ERQ9R {
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
            ERQ9R::ERQ9_0 => false,
            ERQ9R::ERQ9_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ9R {
        match value {
            false => ERQ9R::ERQ9_0,
            true => ERQ9R::ERQ9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ9_0`"]
    #[inline]
    pub fn is_erq9_0(&self) -> bool {
        *self == ERQ9R::ERQ9_0
    }
    #[doc = "Checks if the value of the field is `ERQ9_1`"]
    #[inline]
    pub fn is_erq9_1(&self) -> bool {
        *self == ERQ9R::ERQ9_1
    }
}
#[doc = "Possible values of the field `ERQ10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ10R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ10_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ10_1,
}
impl ERQ10R {
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
            ERQ10R::ERQ10_0 => false,
            ERQ10R::ERQ10_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ10R {
        match value {
            false => ERQ10R::ERQ10_0,
            true => ERQ10R::ERQ10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ10_0`"]
    #[inline]
    pub fn is_erq10_0(&self) -> bool {
        *self == ERQ10R::ERQ10_0
    }
    #[doc = "Checks if the value of the field is `ERQ10_1`"]
    #[inline]
    pub fn is_erq10_1(&self) -> bool {
        *self == ERQ10R::ERQ10_1
    }
}
#[doc = "Possible values of the field `ERQ11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ11R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ11_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ11_1,
}
impl ERQ11R {
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
            ERQ11R::ERQ11_0 => false,
            ERQ11R::ERQ11_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ11R {
        match value {
            false => ERQ11R::ERQ11_0,
            true => ERQ11R::ERQ11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ11_0`"]
    #[inline]
    pub fn is_erq11_0(&self) -> bool {
        *self == ERQ11R::ERQ11_0
    }
    #[doc = "Checks if the value of the field is `ERQ11_1`"]
    #[inline]
    pub fn is_erq11_1(&self) -> bool {
        *self == ERQ11R::ERQ11_1
    }
}
#[doc = "Possible values of the field `ERQ12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ12R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ12_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ12_1,
}
impl ERQ12R {
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
            ERQ12R::ERQ12_0 => false,
            ERQ12R::ERQ12_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ12R {
        match value {
            false => ERQ12R::ERQ12_0,
            true => ERQ12R::ERQ12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ12_0`"]
    #[inline]
    pub fn is_erq12_0(&self) -> bool {
        *self == ERQ12R::ERQ12_0
    }
    #[doc = "Checks if the value of the field is `ERQ12_1`"]
    #[inline]
    pub fn is_erq12_1(&self) -> bool {
        *self == ERQ12R::ERQ12_1
    }
}
#[doc = "Possible values of the field `ERQ13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ13R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ13_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ13_1,
}
impl ERQ13R {
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
            ERQ13R::ERQ13_0 => false,
            ERQ13R::ERQ13_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ13R {
        match value {
            false => ERQ13R::ERQ13_0,
            true => ERQ13R::ERQ13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ13_0`"]
    #[inline]
    pub fn is_erq13_0(&self) -> bool {
        *self == ERQ13R::ERQ13_0
    }
    #[doc = "Checks if the value of the field is `ERQ13_1`"]
    #[inline]
    pub fn is_erq13_1(&self) -> bool {
        *self == ERQ13R::ERQ13_1
    }
}
#[doc = "Possible values of the field `ERQ14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ14R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ14_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ14_1,
}
impl ERQ14R {
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
            ERQ14R::ERQ14_0 => false,
            ERQ14R::ERQ14_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ14R {
        match value {
            false => ERQ14R::ERQ14_0,
            true => ERQ14R::ERQ14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ14_0`"]
    #[inline]
    pub fn is_erq14_0(&self) -> bool {
        *self == ERQ14R::ERQ14_0
    }
    #[doc = "Checks if the value of the field is `ERQ14_1`"]
    #[inline]
    pub fn is_erq14_1(&self) -> bool {
        *self == ERQ14R::ERQ14_1
    }
}
#[doc = "Possible values of the field `ERQ15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ15R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ15_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ15_1,
}
impl ERQ15R {
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
            ERQ15R::ERQ15_0 => false,
            ERQ15R::ERQ15_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ15R {
        match value {
            false => ERQ15R::ERQ15_0,
            true => ERQ15R::ERQ15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ15_0`"]
    #[inline]
    pub fn is_erq15_0(&self) -> bool {
        *self == ERQ15R::ERQ15_0
    }
    #[doc = "Checks if the value of the field is `ERQ15_1`"]
    #[inline]
    pub fn is_erq15_1(&self) -> bool {
        *self == ERQ15R::ERQ15_1
    }
}
#[doc = "Possible values of the field `ERQ16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ16R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ16_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ16_1,
}
impl ERQ16R {
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
            ERQ16R::ERQ16_0 => false,
            ERQ16R::ERQ16_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ16R {
        match value {
            false => ERQ16R::ERQ16_0,
            true => ERQ16R::ERQ16_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ16_0`"]
    #[inline]
    pub fn is_erq16_0(&self) -> bool {
        *self == ERQ16R::ERQ16_0
    }
    #[doc = "Checks if the value of the field is `ERQ16_1`"]
    #[inline]
    pub fn is_erq16_1(&self) -> bool {
        *self == ERQ16R::ERQ16_1
    }
}
#[doc = "Possible values of the field `ERQ17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ17R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ17_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ17_1,
}
impl ERQ17R {
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
            ERQ17R::ERQ17_0 => false,
            ERQ17R::ERQ17_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ17R {
        match value {
            false => ERQ17R::ERQ17_0,
            true => ERQ17R::ERQ17_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ17_0`"]
    #[inline]
    pub fn is_erq17_0(&self) -> bool {
        *self == ERQ17R::ERQ17_0
    }
    #[doc = "Checks if the value of the field is `ERQ17_1`"]
    #[inline]
    pub fn is_erq17_1(&self) -> bool {
        *self == ERQ17R::ERQ17_1
    }
}
#[doc = "Possible values of the field `ERQ18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ18R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ18_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ18_1,
}
impl ERQ18R {
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
            ERQ18R::ERQ18_0 => false,
            ERQ18R::ERQ18_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ18R {
        match value {
            false => ERQ18R::ERQ18_0,
            true => ERQ18R::ERQ18_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ18_0`"]
    #[inline]
    pub fn is_erq18_0(&self) -> bool {
        *self == ERQ18R::ERQ18_0
    }
    #[doc = "Checks if the value of the field is `ERQ18_1`"]
    #[inline]
    pub fn is_erq18_1(&self) -> bool {
        *self == ERQ18R::ERQ18_1
    }
}
#[doc = "Possible values of the field `ERQ19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ19R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ19_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ19_1,
}
impl ERQ19R {
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
            ERQ19R::ERQ19_0 => false,
            ERQ19R::ERQ19_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ19R {
        match value {
            false => ERQ19R::ERQ19_0,
            true => ERQ19R::ERQ19_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ19_0`"]
    #[inline]
    pub fn is_erq19_0(&self) -> bool {
        *self == ERQ19R::ERQ19_0
    }
    #[doc = "Checks if the value of the field is `ERQ19_1`"]
    #[inline]
    pub fn is_erq19_1(&self) -> bool {
        *self == ERQ19R::ERQ19_1
    }
}
#[doc = "Possible values of the field `ERQ20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ20R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ20_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ20_1,
}
impl ERQ20R {
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
            ERQ20R::ERQ20_0 => false,
            ERQ20R::ERQ20_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ20R {
        match value {
            false => ERQ20R::ERQ20_0,
            true => ERQ20R::ERQ20_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ20_0`"]
    #[inline]
    pub fn is_erq20_0(&self) -> bool {
        *self == ERQ20R::ERQ20_0
    }
    #[doc = "Checks if the value of the field is `ERQ20_1`"]
    #[inline]
    pub fn is_erq20_1(&self) -> bool {
        *self == ERQ20R::ERQ20_1
    }
}
#[doc = "Possible values of the field `ERQ21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ21R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ21_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ21_1,
}
impl ERQ21R {
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
            ERQ21R::ERQ21_0 => false,
            ERQ21R::ERQ21_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ21R {
        match value {
            false => ERQ21R::ERQ21_0,
            true => ERQ21R::ERQ21_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ21_0`"]
    #[inline]
    pub fn is_erq21_0(&self) -> bool {
        *self == ERQ21R::ERQ21_0
    }
    #[doc = "Checks if the value of the field is `ERQ21_1`"]
    #[inline]
    pub fn is_erq21_1(&self) -> bool {
        *self == ERQ21R::ERQ21_1
    }
}
#[doc = "Possible values of the field `ERQ22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ22R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ22_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ22_1,
}
impl ERQ22R {
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
            ERQ22R::ERQ22_0 => false,
            ERQ22R::ERQ22_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ22R {
        match value {
            false => ERQ22R::ERQ22_0,
            true => ERQ22R::ERQ22_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ22_0`"]
    #[inline]
    pub fn is_erq22_0(&self) -> bool {
        *self == ERQ22R::ERQ22_0
    }
    #[doc = "Checks if the value of the field is `ERQ22_1`"]
    #[inline]
    pub fn is_erq22_1(&self) -> bool {
        *self == ERQ22R::ERQ22_1
    }
}
#[doc = "Possible values of the field `ERQ23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ23R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ23_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ23_1,
}
impl ERQ23R {
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
            ERQ23R::ERQ23_0 => false,
            ERQ23R::ERQ23_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ23R {
        match value {
            false => ERQ23R::ERQ23_0,
            true => ERQ23R::ERQ23_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ23_0`"]
    #[inline]
    pub fn is_erq23_0(&self) -> bool {
        *self == ERQ23R::ERQ23_0
    }
    #[doc = "Checks if the value of the field is `ERQ23_1`"]
    #[inline]
    pub fn is_erq23_1(&self) -> bool {
        *self == ERQ23R::ERQ23_1
    }
}
#[doc = "Possible values of the field `ERQ24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ24R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ24_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ24_1,
}
impl ERQ24R {
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
            ERQ24R::ERQ24_0 => false,
            ERQ24R::ERQ24_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ24R {
        match value {
            false => ERQ24R::ERQ24_0,
            true => ERQ24R::ERQ24_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ24_0`"]
    #[inline]
    pub fn is_erq24_0(&self) -> bool {
        *self == ERQ24R::ERQ24_0
    }
    #[doc = "Checks if the value of the field is `ERQ24_1`"]
    #[inline]
    pub fn is_erq24_1(&self) -> bool {
        *self == ERQ24R::ERQ24_1
    }
}
#[doc = "Possible values of the field `ERQ25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ25R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ25_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ25_1,
}
impl ERQ25R {
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
            ERQ25R::ERQ25_0 => false,
            ERQ25R::ERQ25_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ25R {
        match value {
            false => ERQ25R::ERQ25_0,
            true => ERQ25R::ERQ25_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ25_0`"]
    #[inline]
    pub fn is_erq25_0(&self) -> bool {
        *self == ERQ25R::ERQ25_0
    }
    #[doc = "Checks if the value of the field is `ERQ25_1`"]
    #[inline]
    pub fn is_erq25_1(&self) -> bool {
        *self == ERQ25R::ERQ25_1
    }
}
#[doc = "Possible values of the field `ERQ26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ26R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ26_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ26_1,
}
impl ERQ26R {
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
            ERQ26R::ERQ26_0 => false,
            ERQ26R::ERQ26_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ26R {
        match value {
            false => ERQ26R::ERQ26_0,
            true => ERQ26R::ERQ26_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ26_0`"]
    #[inline]
    pub fn is_erq26_0(&self) -> bool {
        *self == ERQ26R::ERQ26_0
    }
    #[doc = "Checks if the value of the field is `ERQ26_1`"]
    #[inline]
    pub fn is_erq26_1(&self) -> bool {
        *self == ERQ26R::ERQ26_1
    }
}
#[doc = "Possible values of the field `ERQ27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ27R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ27_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ27_1,
}
impl ERQ27R {
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
            ERQ27R::ERQ27_0 => false,
            ERQ27R::ERQ27_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ27R {
        match value {
            false => ERQ27R::ERQ27_0,
            true => ERQ27R::ERQ27_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ27_0`"]
    #[inline]
    pub fn is_erq27_0(&self) -> bool {
        *self == ERQ27R::ERQ27_0
    }
    #[doc = "Checks if the value of the field is `ERQ27_1`"]
    #[inline]
    pub fn is_erq27_1(&self) -> bool {
        *self == ERQ27R::ERQ27_1
    }
}
#[doc = "Possible values of the field `ERQ28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ28R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ28_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ28_1,
}
impl ERQ28R {
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
            ERQ28R::ERQ28_0 => false,
            ERQ28R::ERQ28_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ28R {
        match value {
            false => ERQ28R::ERQ28_0,
            true => ERQ28R::ERQ28_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ28_0`"]
    #[inline]
    pub fn is_erq28_0(&self) -> bool {
        *self == ERQ28R::ERQ28_0
    }
    #[doc = "Checks if the value of the field is `ERQ28_1`"]
    #[inline]
    pub fn is_erq28_1(&self) -> bool {
        *self == ERQ28R::ERQ28_1
    }
}
#[doc = "Possible values of the field `ERQ29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ29R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ29_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ29_1,
}
impl ERQ29R {
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
            ERQ29R::ERQ29_0 => false,
            ERQ29R::ERQ29_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ29R {
        match value {
            false => ERQ29R::ERQ29_0,
            true => ERQ29R::ERQ29_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ29_0`"]
    #[inline]
    pub fn is_erq29_0(&self) -> bool {
        *self == ERQ29R::ERQ29_0
    }
    #[doc = "Checks if the value of the field is `ERQ29_1`"]
    #[inline]
    pub fn is_erq29_1(&self) -> bool {
        *self == ERQ29R::ERQ29_1
    }
}
#[doc = "Possible values of the field `ERQ30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ30R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ30_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ30_1,
}
impl ERQ30R {
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
            ERQ30R::ERQ30_0 => false,
            ERQ30R::ERQ30_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ30R {
        match value {
            false => ERQ30R::ERQ30_0,
            true => ERQ30R::ERQ30_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ30_0`"]
    #[inline]
    pub fn is_erq30_0(&self) -> bool {
        *self == ERQ30R::ERQ30_0
    }
    #[doc = "Checks if the value of the field is `ERQ30_1`"]
    #[inline]
    pub fn is_erq30_1(&self) -> bool {
        *self == ERQ30R::ERQ30_1
    }
}
#[doc = "Possible values of the field `ERQ31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ31R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ31_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ31_1,
}
impl ERQ31R {
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
            ERQ31R::ERQ31_0 => false,
            ERQ31R::ERQ31_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ31R {
        match value {
            false => ERQ31R::ERQ31_0,
            true => ERQ31R::ERQ31_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ31_0`"]
    #[inline]
    pub fn is_erq31_0(&self) -> bool {
        *self == ERQ31R::ERQ31_0
    }
    #[doc = "Checks if the value of the field is `ERQ31_1`"]
    #[inline]
    pub fn is_erq31_1(&self) -> bool {
        *self == ERQ31R::ERQ31_1
    }
}
#[doc = "Values that can be written to the field `ERQ0`"]
pub enum ERQ0W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ0_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ0_1,
}
impl ERQ0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ0W::ERQ0_0 => false,
            ERQ0W::ERQ0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq0_0(self) -> &'a mut W {
        self.variant(ERQ0W::ERQ0_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq0_1(self) -> &'a mut W {
        self.variant(ERQ0W::ERQ0_1)
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
#[doc = "Values that can be written to the field `ERQ1`"]
pub enum ERQ1W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ1_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ1_1,
}
impl ERQ1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ1W::ERQ1_0 => false,
            ERQ1W::ERQ1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq1_0(self) -> &'a mut W {
        self.variant(ERQ1W::ERQ1_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq1_1(self) -> &'a mut W {
        self.variant(ERQ1W::ERQ1_1)
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
#[doc = "Values that can be written to the field `ERQ2`"]
pub enum ERQ2W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ2_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ2_1,
}
impl ERQ2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ2W::ERQ2_0 => false,
            ERQ2W::ERQ2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq2_0(self) -> &'a mut W {
        self.variant(ERQ2W::ERQ2_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq2_1(self) -> &'a mut W {
        self.variant(ERQ2W::ERQ2_1)
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
#[doc = "Values that can be written to the field `ERQ3`"]
pub enum ERQ3W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ3_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ3_1,
}
impl ERQ3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ3W::ERQ3_0 => false,
            ERQ3W::ERQ3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq3_0(self) -> &'a mut W {
        self.variant(ERQ3W::ERQ3_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq3_1(self) -> &'a mut W {
        self.variant(ERQ3W::ERQ3_1)
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
#[doc = "Values that can be written to the field `ERQ4`"]
pub enum ERQ4W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ4_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ4_1,
}
impl ERQ4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ4W::ERQ4_0 => false,
            ERQ4W::ERQ4_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq4_0(self) -> &'a mut W {
        self.variant(ERQ4W::ERQ4_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq4_1(self) -> &'a mut W {
        self.variant(ERQ4W::ERQ4_1)
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
#[doc = "Values that can be written to the field `ERQ5`"]
pub enum ERQ5W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ5_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ5_1,
}
impl ERQ5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ5W::ERQ5_0 => false,
            ERQ5W::ERQ5_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq5_0(self) -> &'a mut W {
        self.variant(ERQ5W::ERQ5_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq5_1(self) -> &'a mut W {
        self.variant(ERQ5W::ERQ5_1)
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
#[doc = "Values that can be written to the field `ERQ6`"]
pub enum ERQ6W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ6_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ6_1,
}
impl ERQ6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ6W::ERQ6_0 => false,
            ERQ6W::ERQ6_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ6W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq6_0(self) -> &'a mut W {
        self.variant(ERQ6W::ERQ6_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq6_1(self) -> &'a mut W {
        self.variant(ERQ6W::ERQ6_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ7`"]
pub enum ERQ7W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ7_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ7_1,
}
impl ERQ7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ7W::ERQ7_0 => false,
            ERQ7W::ERQ7_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq7_0(self) -> &'a mut W {
        self.variant(ERQ7W::ERQ7_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq7_1(self) -> &'a mut W {
        self.variant(ERQ7W::ERQ7_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ8`"]
pub enum ERQ8W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ8_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ8_1,
}
impl ERQ8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ8W::ERQ8_0 => false,
            ERQ8W::ERQ8_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ8W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq8_0(self) -> &'a mut W {
        self.variant(ERQ8W::ERQ8_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq8_1(self) -> &'a mut W {
        self.variant(ERQ8W::ERQ8_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ9`"]
pub enum ERQ9W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ9_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ9_1,
}
impl ERQ9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ9W::ERQ9_0 => false,
            ERQ9W::ERQ9_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ9W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq9_0(self) -> &'a mut W {
        self.variant(ERQ9W::ERQ9_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq9_1(self) -> &'a mut W {
        self.variant(ERQ9W::ERQ9_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ10`"]
pub enum ERQ10W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ10_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ10_1,
}
impl ERQ10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ10W::ERQ10_0 => false,
            ERQ10W::ERQ10_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ10W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq10_0(self) -> &'a mut W {
        self.variant(ERQ10W::ERQ10_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq10_1(self) -> &'a mut W {
        self.variant(ERQ10W::ERQ10_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ11`"]
pub enum ERQ11W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ11_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ11_1,
}
impl ERQ11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ11W::ERQ11_0 => false,
            ERQ11W::ERQ11_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ11W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq11_0(self) -> &'a mut W {
        self.variant(ERQ11W::ERQ11_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq11_1(self) -> &'a mut W {
        self.variant(ERQ11W::ERQ11_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ12`"]
pub enum ERQ12W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ12_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ12_1,
}
impl ERQ12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ12W::ERQ12_0 => false,
            ERQ12W::ERQ12_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq12_0(self) -> &'a mut W {
        self.variant(ERQ12W::ERQ12_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq12_1(self) -> &'a mut W {
        self.variant(ERQ12W::ERQ12_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ13`"]
pub enum ERQ13W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ13_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ13_1,
}
impl ERQ13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ13W::ERQ13_0 => false,
            ERQ13W::ERQ13_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq13_0(self) -> &'a mut W {
        self.variant(ERQ13W::ERQ13_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq13_1(self) -> &'a mut W {
        self.variant(ERQ13W::ERQ13_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ14`"]
pub enum ERQ14W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ14_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ14_1,
}
impl ERQ14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ14W::ERQ14_0 => false,
            ERQ14W::ERQ14_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq14_0(self) -> &'a mut W {
        self.variant(ERQ14W::ERQ14_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq14_1(self) -> &'a mut W {
        self.variant(ERQ14W::ERQ14_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ15`"]
pub enum ERQ15W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ15_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ15_1,
}
impl ERQ15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ15W::ERQ15_0 => false,
            ERQ15W::ERQ15_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq15_0(self) -> &'a mut W {
        self.variant(ERQ15W::ERQ15_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq15_1(self) -> &'a mut W {
        self.variant(ERQ15W::ERQ15_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ16`"]
pub enum ERQ16W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ16_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ16_1,
}
impl ERQ16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ16W::ERQ16_0 => false,
            ERQ16W::ERQ16_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ16W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq16_0(self) -> &'a mut W {
        self.variant(ERQ16W::ERQ16_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq16_1(self) -> &'a mut W {
        self.variant(ERQ16W::ERQ16_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ17`"]
pub enum ERQ17W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ17_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ17_1,
}
impl ERQ17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ17W::ERQ17_0 => false,
            ERQ17W::ERQ17_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ17W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq17_0(self) -> &'a mut W {
        self.variant(ERQ17W::ERQ17_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq17_1(self) -> &'a mut W {
        self.variant(ERQ17W::ERQ17_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ18`"]
pub enum ERQ18W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ18_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ18_1,
}
impl ERQ18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ18W::ERQ18_0 => false,
            ERQ18W::ERQ18_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ18W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq18_0(self) -> &'a mut W {
        self.variant(ERQ18W::ERQ18_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq18_1(self) -> &'a mut W {
        self.variant(ERQ18W::ERQ18_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ19`"]
pub enum ERQ19W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ19_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ19_1,
}
impl ERQ19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ19W::ERQ19_0 => false,
            ERQ19W::ERQ19_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ19W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq19_0(self) -> &'a mut W {
        self.variant(ERQ19W::ERQ19_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq19_1(self) -> &'a mut W {
        self.variant(ERQ19W::ERQ19_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ20`"]
pub enum ERQ20W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ20_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ20_1,
}
impl ERQ20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ20W::ERQ20_0 => false,
            ERQ20W::ERQ20_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ20W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq20_0(self) -> &'a mut W {
        self.variant(ERQ20W::ERQ20_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq20_1(self) -> &'a mut W {
        self.variant(ERQ20W::ERQ20_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ21`"]
pub enum ERQ21W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ21_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ21_1,
}
impl ERQ21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ21W::ERQ21_0 => false,
            ERQ21W::ERQ21_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ21W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq21_0(self) -> &'a mut W {
        self.variant(ERQ21W::ERQ21_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq21_1(self) -> &'a mut W {
        self.variant(ERQ21W::ERQ21_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ22`"]
pub enum ERQ22W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ22_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ22_1,
}
impl ERQ22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ22W::ERQ22_0 => false,
            ERQ22W::ERQ22_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ22W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq22_0(self) -> &'a mut W {
        self.variant(ERQ22W::ERQ22_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq22_1(self) -> &'a mut W {
        self.variant(ERQ22W::ERQ22_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ23`"]
pub enum ERQ23W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ23_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ23_1,
}
impl ERQ23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ23W::ERQ23_0 => false,
            ERQ23W::ERQ23_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ23W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq23_0(self) -> &'a mut W {
        self.variant(ERQ23W::ERQ23_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq23_1(self) -> &'a mut W {
        self.variant(ERQ23W::ERQ23_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ24`"]
pub enum ERQ24W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ24_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ24_1,
}
impl ERQ24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ24W::ERQ24_0 => false,
            ERQ24W::ERQ24_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ24W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq24_0(self) -> &'a mut W {
        self.variant(ERQ24W::ERQ24_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq24_1(self) -> &'a mut W {
        self.variant(ERQ24W::ERQ24_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ25`"]
pub enum ERQ25W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ25_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ25_1,
}
impl ERQ25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ25W::ERQ25_0 => false,
            ERQ25W::ERQ25_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ25W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq25_0(self) -> &'a mut W {
        self.variant(ERQ25W::ERQ25_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq25_1(self) -> &'a mut W {
        self.variant(ERQ25W::ERQ25_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ26`"]
pub enum ERQ26W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ26_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ26_1,
}
impl ERQ26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ26W::ERQ26_0 => false,
            ERQ26W::ERQ26_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ26W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq26_0(self) -> &'a mut W {
        self.variant(ERQ26W::ERQ26_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq26_1(self) -> &'a mut W {
        self.variant(ERQ26W::ERQ26_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ27`"]
pub enum ERQ27W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ27_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ27_1,
}
impl ERQ27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ27W::ERQ27_0 => false,
            ERQ27W::ERQ27_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ27W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq27_0(self) -> &'a mut W {
        self.variant(ERQ27W::ERQ27_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq27_1(self) -> &'a mut W {
        self.variant(ERQ27W::ERQ27_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ28`"]
pub enum ERQ28W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ28_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ28_1,
}
impl ERQ28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ28W::ERQ28_0 => false,
            ERQ28W::ERQ28_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ28W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq28_0(self) -> &'a mut W {
        self.variant(ERQ28W::ERQ28_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq28_1(self) -> &'a mut W {
        self.variant(ERQ28W::ERQ28_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ29`"]
pub enum ERQ29W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ29_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ29_1,
}
impl ERQ29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ29W::ERQ29_0 => false,
            ERQ29W::ERQ29_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ29W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq29_0(self) -> &'a mut W {
        self.variant(ERQ29W::ERQ29_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq29_1(self) -> &'a mut W {
        self.variant(ERQ29W::ERQ29_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ30`"]
pub enum ERQ30W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ30_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ30_1,
}
impl ERQ30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ30W::ERQ30_0 => false,
            ERQ30W::ERQ30_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ30W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq30_0(self) -> &'a mut W {
        self.variant(ERQ30W::ERQ30_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq30_1(self) -> &'a mut W {
        self.variant(ERQ30W::ERQ30_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERQ31`"]
pub enum ERQ31W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    ERQ31_0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    ERQ31_1,
}
impl ERQ31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ31W::ERQ31_0 => false,
            ERQ31W::ERQ31_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ31W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn erq31_0(self) -> &'a mut W {
        self.variant(ERQ31W::ERQ31_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn erq31_1(self) -> &'a mut W {
        self.variant(ERQ31W::ERQ31_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline]
    pub fn erq0(&self) -> ERQ0R {
        ERQ0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline]
    pub fn erq1(&self) -> ERQ1R {
        ERQ1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline]
    pub fn erq2(&self) -> ERQ2R {
        ERQ2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline]
    pub fn erq3(&self) -> ERQ3R {
        ERQ3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline]
    pub fn erq4(&self) -> ERQ4R {
        ERQ4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline]
    pub fn erq5(&self) -> ERQ5R {
        ERQ5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline]
    pub fn erq6(&self) -> ERQ6R {
        ERQ6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline]
    pub fn erq7(&self) -> ERQ7R {
        ERQ7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline]
    pub fn erq8(&self) -> ERQ8R {
        ERQ8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline]
    pub fn erq9(&self) -> ERQ9R {
        ERQ9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline]
    pub fn erq10(&self) -> ERQ10R {
        ERQ10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline]
    pub fn erq11(&self) -> ERQ11R {
        ERQ11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline]
    pub fn erq12(&self) -> ERQ12R {
        ERQ12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline]
    pub fn erq13(&self) -> ERQ13R {
        ERQ13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline]
    pub fn erq14(&self) -> ERQ14R {
        ERQ14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline]
    pub fn erq15(&self) -> ERQ15R {
        ERQ15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable DMA Request 16"]
    #[inline]
    pub fn erq16(&self) -> ERQ16R {
        ERQ16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable DMA Request 17"]
    #[inline]
    pub fn erq17(&self) -> ERQ17R {
        ERQ17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable DMA Request 18"]
    #[inline]
    pub fn erq18(&self) -> ERQ18R {
        ERQ18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable DMA Request 19"]
    #[inline]
    pub fn erq19(&self) -> ERQ19R {
        ERQ19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable DMA Request 20"]
    #[inline]
    pub fn erq20(&self) -> ERQ20R {
        ERQ20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable DMA Request 21"]
    #[inline]
    pub fn erq21(&self) -> ERQ21R {
        ERQ21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enable DMA Request 22"]
    #[inline]
    pub fn erq22(&self) -> ERQ22R {
        ERQ22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable DMA Request 23"]
    #[inline]
    pub fn erq23(&self) -> ERQ23R {
        ERQ23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable DMA Request 24"]
    #[inline]
    pub fn erq24(&self) -> ERQ24R {
        ERQ24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable DMA Request 25"]
    #[inline]
    pub fn erq25(&self) -> ERQ25R {
        ERQ25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable DMA Request 26"]
    #[inline]
    pub fn erq26(&self) -> ERQ26R {
        ERQ26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable DMA Request 27"]
    #[inline]
    pub fn erq27(&self) -> ERQ27R {
        ERQ27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable DMA Request 28"]
    #[inline]
    pub fn erq28(&self) -> ERQ28R {
        ERQ28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable DMA Request 29"]
    #[inline]
    pub fn erq29(&self) -> ERQ29R {
        ERQ29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable DMA Request 30"]
    #[inline]
    pub fn erq30(&self) -> ERQ30R {
        ERQ30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable DMA Request 31"]
    #[inline]
    pub fn erq31(&self) -> ERQ31R {
        ERQ31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline]
    pub fn erq0(&mut self) -> _ERQ0W {
        _ERQ0W { w: self }
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline]
    pub fn erq1(&mut self) -> _ERQ1W {
        _ERQ1W { w: self }
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline]
    pub fn erq2(&mut self) -> _ERQ2W {
        _ERQ2W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline]
    pub fn erq3(&mut self) -> _ERQ3W {
        _ERQ3W { w: self }
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline]
    pub fn erq4(&mut self) -> _ERQ4W {
        _ERQ4W { w: self }
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline]
    pub fn erq5(&mut self) -> _ERQ5W {
        _ERQ5W { w: self }
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline]
    pub fn erq6(&mut self) -> _ERQ6W {
        _ERQ6W { w: self }
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline]
    pub fn erq7(&mut self) -> _ERQ7W {
        _ERQ7W { w: self }
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline]
    pub fn erq8(&mut self) -> _ERQ8W {
        _ERQ8W { w: self }
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline]
    pub fn erq9(&mut self) -> _ERQ9W {
        _ERQ9W { w: self }
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline]
    pub fn erq10(&mut self) -> _ERQ10W {
        _ERQ10W { w: self }
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline]
    pub fn erq11(&mut self) -> _ERQ11W {
        _ERQ11W { w: self }
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline]
    pub fn erq12(&mut self) -> _ERQ12W {
        _ERQ12W { w: self }
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline]
    pub fn erq13(&mut self) -> _ERQ13W {
        _ERQ13W { w: self }
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline]
    pub fn erq14(&mut self) -> _ERQ14W {
        _ERQ14W { w: self }
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline]
    pub fn erq15(&mut self) -> _ERQ15W {
        _ERQ15W { w: self }
    }
    #[doc = "Bit 16 - Enable DMA Request 16"]
    #[inline]
    pub fn erq16(&mut self) -> _ERQ16W {
        _ERQ16W { w: self }
    }
    #[doc = "Bit 17 - Enable DMA Request 17"]
    #[inline]
    pub fn erq17(&mut self) -> _ERQ17W {
        _ERQ17W { w: self }
    }
    #[doc = "Bit 18 - Enable DMA Request 18"]
    #[inline]
    pub fn erq18(&mut self) -> _ERQ18W {
        _ERQ18W { w: self }
    }
    #[doc = "Bit 19 - Enable DMA Request 19"]
    #[inline]
    pub fn erq19(&mut self) -> _ERQ19W {
        _ERQ19W { w: self }
    }
    #[doc = "Bit 20 - Enable DMA Request 20"]
    #[inline]
    pub fn erq20(&mut self) -> _ERQ20W {
        _ERQ20W { w: self }
    }
    #[doc = "Bit 21 - Enable DMA Request 21"]
    #[inline]
    pub fn erq21(&mut self) -> _ERQ21W {
        _ERQ21W { w: self }
    }
    #[doc = "Bit 22 - Enable DMA Request 22"]
    #[inline]
    pub fn erq22(&mut self) -> _ERQ22W {
        _ERQ22W { w: self }
    }
    #[doc = "Bit 23 - Enable DMA Request 23"]
    #[inline]
    pub fn erq23(&mut self) -> _ERQ23W {
        _ERQ23W { w: self }
    }
    #[doc = "Bit 24 - Enable DMA Request 24"]
    #[inline]
    pub fn erq24(&mut self) -> _ERQ24W {
        _ERQ24W { w: self }
    }
    #[doc = "Bit 25 - Enable DMA Request 25"]
    #[inline]
    pub fn erq25(&mut self) -> _ERQ25W {
        _ERQ25W { w: self }
    }
    #[doc = "Bit 26 - Enable DMA Request 26"]
    #[inline]
    pub fn erq26(&mut self) -> _ERQ26W {
        _ERQ26W { w: self }
    }
    #[doc = "Bit 27 - Enable DMA Request 27"]
    #[inline]
    pub fn erq27(&mut self) -> _ERQ27W {
        _ERQ27W { w: self }
    }
    #[doc = "Bit 28 - Enable DMA Request 28"]
    #[inline]
    pub fn erq28(&mut self) -> _ERQ28W {
        _ERQ28W { w: self }
    }
    #[doc = "Bit 29 - Enable DMA Request 29"]
    #[inline]
    pub fn erq29(&mut self) -> _ERQ29W {
        _ERQ29W { w: self }
    }
    #[doc = "Bit 30 - Enable DMA Request 30"]
    #[inline]
    pub fn erq30(&mut self) -> _ERQ30W {
        _ERQ30W { w: self }
    }
    #[doc = "Bit 31 - Enable DMA Request 31"]
    #[inline]
    pub fn erq31(&mut self) -> _ERQ31W {
        _ERQ31W { w: self }
    }
}
