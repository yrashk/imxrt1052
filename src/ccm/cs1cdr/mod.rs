#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CS1CDR {
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
#[doc = "Possible values of the field `SAI1_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_CLK_PODFR {
    #[doc = "divide by 1"]
    SAI1_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    SAI1_CLK_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI1_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI1_CLK_PODFR::SAI1_CLK_PODF_0 => 0,
            SAI1_CLK_PODFR::SAI1_CLK_PODF_63 => 63,
            SAI1_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI1_CLK_PODFR {
        match value {
            0 => SAI1_CLK_PODFR::SAI1_CLK_PODF_0,
            63 => SAI1_CLK_PODFR::SAI1_CLK_PODF_63,
            i => SAI1_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PODF_0`"]
    #[inline]
    pub fn is_sai1_clk_podf_0(&self) -> bool {
        *self == SAI1_CLK_PODFR::SAI1_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PODF_63`"]
    #[inline]
    pub fn is_sai1_clk_podf_63(&self) -> bool {
        *self == SAI1_CLK_PODFR::SAI1_CLK_PODF_63
    }
}
#[doc = "Possible values of the field `SAI1_CLK_PRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_CLK_PREDR {
    #[doc = "divide by 1"]
    SAI1_CLK_PRED_0,
    #[doc = "divide by 2"]
    SAI1_CLK_PRED_1,
    #[doc = "divide by 3"]
    SAI1_CLK_PRED_2,
    #[doc = "divide by 4"]
    SAI1_CLK_PRED_3,
    #[doc = "divide by 5"]
    SAI1_CLK_PRED_4,
    #[doc = "divide by 6"]
    SAI1_CLK_PRED_5,
    #[doc = "divide by 7"]
    SAI1_CLK_PRED_6,
    #[doc = "divide by 8"]
    SAI1_CLK_PRED_7,
}
impl SAI1_CLK_PREDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI1_CLK_PREDR::SAI1_CLK_PRED_0 => 0,
            SAI1_CLK_PREDR::SAI1_CLK_PRED_1 => 1,
            SAI1_CLK_PREDR::SAI1_CLK_PRED_2 => 2,
            SAI1_CLK_PREDR::SAI1_CLK_PRED_3 => 3,
            SAI1_CLK_PREDR::SAI1_CLK_PRED_4 => 4,
            SAI1_CLK_PREDR::SAI1_CLK_PRED_5 => 5,
            SAI1_CLK_PREDR::SAI1_CLK_PRED_6 => 6,
            SAI1_CLK_PREDR::SAI1_CLK_PRED_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI1_CLK_PREDR {
        match value {
            0 => SAI1_CLK_PREDR::SAI1_CLK_PRED_0,
            1 => SAI1_CLK_PREDR::SAI1_CLK_PRED_1,
            2 => SAI1_CLK_PREDR::SAI1_CLK_PRED_2,
            3 => SAI1_CLK_PREDR::SAI1_CLK_PRED_3,
            4 => SAI1_CLK_PREDR::SAI1_CLK_PRED_4,
            5 => SAI1_CLK_PREDR::SAI1_CLK_PRED_5,
            6 => SAI1_CLK_PREDR::SAI1_CLK_PRED_6,
            7 => SAI1_CLK_PREDR::SAI1_CLK_PRED_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_0`"]
    #[inline]
    pub fn is_sai1_clk_pred_0(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_0
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_1`"]
    #[inline]
    pub fn is_sai1_clk_pred_1(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_1
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_2`"]
    #[inline]
    pub fn is_sai1_clk_pred_2(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_2
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_3`"]
    #[inline]
    pub fn is_sai1_clk_pred_3(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_3
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_4`"]
    #[inline]
    pub fn is_sai1_clk_pred_4(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_4
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_5`"]
    #[inline]
    pub fn is_sai1_clk_pred_5(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_5
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_6`"]
    #[inline]
    pub fn is_sai1_clk_pred_6(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_6
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_PRED_7`"]
    #[inline]
    pub fn is_sai1_clk_pred_7(&self) -> bool {
        *self == SAI1_CLK_PREDR::SAI1_CLK_PRED_7
    }
}
#[doc = "Possible values of the field `FLEXIO2_CLK_PRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_CLK_PREDR {
    #[doc = "divide by 1"]
    FLEXIO2_CLK_PRED_0,
    #[doc = "divide by 2"]
    FLEXIO2_CLK_PRED_1,
    #[doc = "divide by 3"]
    FLEXIO2_CLK_PRED_2,
    #[doc = "divide by 4"]
    FLEXIO2_CLK_PRED_3,
    #[doc = "divide by 5"]
    FLEXIO2_CLK_PRED_4,
    #[doc = "divide by 6"]
    FLEXIO2_CLK_PRED_5,
    #[doc = "divide by 7"]
    FLEXIO2_CLK_PRED_6,
    #[doc = "divide by 8"]
    FLEXIO2_CLK_PRED_7,
}
impl FLEXIO2_CLK_PREDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_0 => 0,
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_1 => 1,
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_2 => 2,
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_3 => 3,
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_4 => 4,
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_5 => 5,
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_6 => 6,
            FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXIO2_CLK_PREDR {
        match value {
            0 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_0,
            1 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_1,
            2 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_2,
            3 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_3,
            4 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_4,
            5 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_5,
            6 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_6,
            7 => FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_0`"]
    #[inline]
    pub fn is_flexio2_clk_pred_0(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_1`"]
    #[inline]
    pub fn is_flexio2_clk_pred_1(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_1
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_2`"]
    #[inline]
    pub fn is_flexio2_clk_pred_2(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_2
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_3`"]
    #[inline]
    pub fn is_flexio2_clk_pred_3(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_3
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_4`"]
    #[inline]
    pub fn is_flexio2_clk_pred_4(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_4
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_5`"]
    #[inline]
    pub fn is_flexio2_clk_pred_5(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_5
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_6`"]
    #[inline]
    pub fn is_flexio2_clk_pred_6(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_6
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PRED_7`"]
    #[inline]
    pub fn is_flexio2_clk_pred_7(&self) -> bool {
        *self == FLEXIO2_CLK_PREDR::FLEXIO2_CLK_PRED_7
    }
}
#[doc = "Possible values of the field `SAI3_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_CLK_PODFR {
    #[doc = "divide by 1"]
    SAI3_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    SAI3_CLK_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI3_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI3_CLK_PODFR::SAI3_CLK_PODF_0 => 0,
            SAI3_CLK_PODFR::SAI3_CLK_PODF_63 => 63,
            SAI3_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI3_CLK_PODFR {
        match value {
            0 => SAI3_CLK_PODFR::SAI3_CLK_PODF_0,
            63 => SAI3_CLK_PODFR::SAI3_CLK_PODF_63,
            i => SAI3_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PODF_0`"]
    #[inline]
    pub fn is_sai3_clk_podf_0(&self) -> bool {
        *self == SAI3_CLK_PODFR::SAI3_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PODF_63`"]
    #[inline]
    pub fn is_sai3_clk_podf_63(&self) -> bool {
        *self == SAI3_CLK_PODFR::SAI3_CLK_PODF_63
    }
}
#[doc = "Possible values of the field `SAI3_CLK_PRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_CLK_PREDR {
    #[doc = "divide by 1"]
    SAI3_CLK_PRED_0,
    #[doc = "divide by 2"]
    SAI3_CLK_PRED_1,
    #[doc = "divide by 3"]
    SAI3_CLK_PRED_2,
    #[doc = "divide by 4"]
    SAI3_CLK_PRED_3,
    #[doc = "divide by 5"]
    SAI3_CLK_PRED_4,
    #[doc = "divide by 6"]
    SAI3_CLK_PRED_5,
    #[doc = "divide by 7"]
    SAI3_CLK_PRED_6,
    #[doc = "divide by 8"]
    SAI3_CLK_PRED_7,
}
impl SAI3_CLK_PREDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI3_CLK_PREDR::SAI3_CLK_PRED_0 => 0,
            SAI3_CLK_PREDR::SAI3_CLK_PRED_1 => 1,
            SAI3_CLK_PREDR::SAI3_CLK_PRED_2 => 2,
            SAI3_CLK_PREDR::SAI3_CLK_PRED_3 => 3,
            SAI3_CLK_PREDR::SAI3_CLK_PRED_4 => 4,
            SAI3_CLK_PREDR::SAI3_CLK_PRED_5 => 5,
            SAI3_CLK_PREDR::SAI3_CLK_PRED_6 => 6,
            SAI3_CLK_PREDR::SAI3_CLK_PRED_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI3_CLK_PREDR {
        match value {
            0 => SAI3_CLK_PREDR::SAI3_CLK_PRED_0,
            1 => SAI3_CLK_PREDR::SAI3_CLK_PRED_1,
            2 => SAI3_CLK_PREDR::SAI3_CLK_PRED_2,
            3 => SAI3_CLK_PREDR::SAI3_CLK_PRED_3,
            4 => SAI3_CLK_PREDR::SAI3_CLK_PRED_4,
            5 => SAI3_CLK_PREDR::SAI3_CLK_PRED_5,
            6 => SAI3_CLK_PREDR::SAI3_CLK_PRED_6,
            7 => SAI3_CLK_PREDR::SAI3_CLK_PRED_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_0`"]
    #[inline]
    pub fn is_sai3_clk_pred_0(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_0
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_1`"]
    #[inline]
    pub fn is_sai3_clk_pred_1(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_1
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_2`"]
    #[inline]
    pub fn is_sai3_clk_pred_2(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_2
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_3`"]
    #[inline]
    pub fn is_sai3_clk_pred_3(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_3
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_4`"]
    #[inline]
    pub fn is_sai3_clk_pred_4(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_4
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_5`"]
    #[inline]
    pub fn is_sai3_clk_pred_5(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_5
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_6`"]
    #[inline]
    pub fn is_sai3_clk_pred_6(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_6
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_PRED_7`"]
    #[inline]
    pub fn is_sai3_clk_pred_7(&self) -> bool {
        *self == SAI3_CLK_PREDR::SAI3_CLK_PRED_7
    }
}
#[doc = "Possible values of the field `FLEXIO2_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_CLK_PODFR {
    #[doc = "divide by 1"]
    FLEXIO2_CLK_PODF_0,
    #[doc = "divide by 2"]
    FLEXIO2_CLK_PODF_1,
    #[doc = "divide by 3"]
    FLEXIO2_CLK_PODF_2,
    #[doc = "divide by 4"]
    FLEXIO2_CLK_PODF_3,
    #[doc = "divide by 5"]
    FLEXIO2_CLK_PODF_4,
    #[doc = "divide by 6"]
    FLEXIO2_CLK_PODF_5,
    #[doc = "divide by 7"]
    FLEXIO2_CLK_PODF_6,
    #[doc = "divide by 8"]
    FLEXIO2_CLK_PODF_7,
}
impl FLEXIO2_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_0 => 0,
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_1 => 1,
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_2 => 2,
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_3 => 3,
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_4 => 4,
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_5 => 5,
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_6 => 6,
            FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXIO2_CLK_PODFR {
        match value {
            0 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_0,
            1 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_1,
            2 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_2,
            3 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_3,
            4 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_4,
            5 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_5,
            6 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_6,
            7 => FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_0`"]
    #[inline]
    pub fn is_flexio2_clk_podf_0(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_1`"]
    #[inline]
    pub fn is_flexio2_clk_podf_1(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_1
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_2`"]
    #[inline]
    pub fn is_flexio2_clk_podf_2(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_2
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_3`"]
    #[inline]
    pub fn is_flexio2_clk_podf_3(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_3
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_4`"]
    #[inline]
    pub fn is_flexio2_clk_podf_4(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_4
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_5`"]
    #[inline]
    pub fn is_flexio2_clk_podf_5(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_5
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_6`"]
    #[inline]
    pub fn is_flexio2_clk_podf_6(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_6
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_PODF_7`"]
    #[inline]
    pub fn is_flexio2_clk_podf_7(&self) -> bool {
        *self == FLEXIO2_CLK_PODFR::FLEXIO2_CLK_PODF_7
    }
}
#[doc = "Values that can be written to the field `SAI1_CLK_PODF`"]
pub enum SAI1_CLK_PODFW {
    #[doc = "divide by 1"]
    SAI1_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    SAI1_CLK_PODF_63,
}
impl SAI1_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI1_CLK_PODFW::SAI1_CLK_PODF_0 => 0,
            SAI1_CLK_PODFW::SAI1_CLK_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn sai1_clk_podf_0(self) -> &'a mut W {
        self.variant(SAI1_CLK_PODFW::SAI1_CLK_PODF_0)
    }
    #[doc = "divide by 2^6"]
    #[inline]
    pub fn sai1_clk_podf_63(self) -> &'a mut W {
        self.variant(SAI1_CLK_PODFW::SAI1_CLK_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI1_CLK_PRED`"]
pub enum SAI1_CLK_PREDW {
    #[doc = "divide by 1"]
    SAI1_CLK_PRED_0,
    #[doc = "divide by 2"]
    SAI1_CLK_PRED_1,
    #[doc = "divide by 3"]
    SAI1_CLK_PRED_2,
    #[doc = "divide by 4"]
    SAI1_CLK_PRED_3,
    #[doc = "divide by 5"]
    SAI1_CLK_PRED_4,
    #[doc = "divide by 6"]
    SAI1_CLK_PRED_5,
    #[doc = "divide by 7"]
    SAI1_CLK_PRED_6,
    #[doc = "divide by 8"]
    SAI1_CLK_PRED_7,
}
impl SAI1_CLK_PREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI1_CLK_PREDW::SAI1_CLK_PRED_0 => 0,
            SAI1_CLK_PREDW::SAI1_CLK_PRED_1 => 1,
            SAI1_CLK_PREDW::SAI1_CLK_PRED_2 => 2,
            SAI1_CLK_PREDW::SAI1_CLK_PRED_3 => 3,
            SAI1_CLK_PREDW::SAI1_CLK_PRED_4 => 4,
            SAI1_CLK_PREDW::SAI1_CLK_PRED_5 => 5,
            SAI1_CLK_PREDW::SAI1_CLK_PRED_6 => 6,
            SAI1_CLK_PREDW::SAI1_CLK_PRED_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_CLK_PREDW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_CLK_PREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_CLK_PREDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn sai1_clk_pred_0(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn sai1_clk_pred_1(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn sai1_clk_pred_2(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn sai1_clk_pred_3(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn sai1_clk_pred_4(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn sai1_clk_pred_5(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn sai1_clk_pred_6(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn sai1_clk_pred_7(self) -> &'a mut W {
        self.variant(SAI1_CLK_PREDW::SAI1_CLK_PRED_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLEXIO2_CLK_PRED`"]
pub enum FLEXIO2_CLK_PREDW {
    #[doc = "divide by 1"]
    FLEXIO2_CLK_PRED_0,
    #[doc = "divide by 2"]
    FLEXIO2_CLK_PRED_1,
    #[doc = "divide by 3"]
    FLEXIO2_CLK_PRED_2,
    #[doc = "divide by 4"]
    FLEXIO2_CLK_PRED_3,
    #[doc = "divide by 5"]
    FLEXIO2_CLK_PRED_4,
    #[doc = "divide by 6"]
    FLEXIO2_CLK_PRED_5,
    #[doc = "divide by 7"]
    FLEXIO2_CLK_PRED_6,
    #[doc = "divide by 8"]
    FLEXIO2_CLK_PRED_7,
}
impl FLEXIO2_CLK_PREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_0 => 0,
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_1 => 1,
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_2 => 2,
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_3 => 3,
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_4 => 4,
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_5 => 5,
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_6 => 6,
            FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO2_CLK_PREDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO2_CLK_PREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO2_CLK_PREDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn flexio2_clk_pred_0(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn flexio2_clk_pred_1(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn flexio2_clk_pred_2(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn flexio2_clk_pred_3(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn flexio2_clk_pred_4(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn flexio2_clk_pred_5(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn flexio2_clk_pred_6(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn flexio2_clk_pred_7(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PREDW::FLEXIO2_CLK_PRED_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI3_CLK_PODF`"]
pub enum SAI3_CLK_PODFW {
    #[doc = "divide by 1"]
    SAI3_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    SAI3_CLK_PODF_63,
}
impl SAI3_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI3_CLK_PODFW::SAI3_CLK_PODF_0 => 0,
            SAI3_CLK_PODFW::SAI3_CLK_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI3_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI3_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI3_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn sai3_clk_podf_0(self) -> &'a mut W {
        self.variant(SAI3_CLK_PODFW::SAI3_CLK_PODF_0)
    }
    #[doc = "divide by 2^6"]
    #[inline]
    pub fn sai3_clk_podf_63(self) -> &'a mut W {
        self.variant(SAI3_CLK_PODFW::SAI3_CLK_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI3_CLK_PRED`"]
pub enum SAI3_CLK_PREDW {
    #[doc = "divide by 1"]
    SAI3_CLK_PRED_0,
    #[doc = "divide by 2"]
    SAI3_CLK_PRED_1,
    #[doc = "divide by 3"]
    SAI3_CLK_PRED_2,
    #[doc = "divide by 4"]
    SAI3_CLK_PRED_3,
    #[doc = "divide by 5"]
    SAI3_CLK_PRED_4,
    #[doc = "divide by 6"]
    SAI3_CLK_PRED_5,
    #[doc = "divide by 7"]
    SAI3_CLK_PRED_6,
    #[doc = "divide by 8"]
    SAI3_CLK_PRED_7,
}
impl SAI3_CLK_PREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI3_CLK_PREDW::SAI3_CLK_PRED_0 => 0,
            SAI3_CLK_PREDW::SAI3_CLK_PRED_1 => 1,
            SAI3_CLK_PREDW::SAI3_CLK_PRED_2 => 2,
            SAI3_CLK_PREDW::SAI3_CLK_PRED_3 => 3,
            SAI3_CLK_PREDW::SAI3_CLK_PRED_4 => 4,
            SAI3_CLK_PREDW::SAI3_CLK_PRED_5 => 5,
            SAI3_CLK_PREDW::SAI3_CLK_PRED_6 => 6,
            SAI3_CLK_PREDW::SAI3_CLK_PRED_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI3_CLK_PREDW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI3_CLK_PREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI3_CLK_PREDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn sai3_clk_pred_0(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn sai3_clk_pred_1(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn sai3_clk_pred_2(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn sai3_clk_pred_3(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn sai3_clk_pred_4(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn sai3_clk_pred_5(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn sai3_clk_pred_6(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn sai3_clk_pred_7(self) -> &'a mut W {
        self.variant(SAI3_CLK_PREDW::SAI3_CLK_PRED_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLEXIO2_CLK_PODF`"]
pub enum FLEXIO2_CLK_PODFW {
    #[doc = "divide by 1"]
    FLEXIO2_CLK_PODF_0,
    #[doc = "divide by 2"]
    FLEXIO2_CLK_PODF_1,
    #[doc = "divide by 3"]
    FLEXIO2_CLK_PODF_2,
    #[doc = "divide by 4"]
    FLEXIO2_CLK_PODF_3,
    #[doc = "divide by 5"]
    FLEXIO2_CLK_PODF_4,
    #[doc = "divide by 6"]
    FLEXIO2_CLK_PODF_5,
    #[doc = "divide by 7"]
    FLEXIO2_CLK_PODF_6,
    #[doc = "divide by 8"]
    FLEXIO2_CLK_PODF_7,
}
impl FLEXIO2_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_0 => 0,
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_1 => 1,
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_2 => 2,
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_3 => 3,
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_4 => 4,
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_5 => 5,
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_6 => 6,
            FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO2_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO2_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO2_CLK_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn flexio2_clk_podf_0(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn flexio2_clk_podf_1(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn flexio2_clk_podf_2(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn flexio2_clk_podf_3(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn flexio2_clk_podf_4(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn flexio2_clk_podf_5(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn flexio2_clk_podf_6(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn flexio2_clk_podf_7(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_PODFW::FLEXIO2_CLK_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
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
    #[doc = "Bits 0:5 - Divider for sai1 clock podf"]
    #[inline]
    pub fn sai1_clk_podf(&self) -> SAI1_CLK_PODFR {
        SAI1_CLK_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - Divider for sai1 clock pred."]
    #[inline]
    pub fn sai1_clk_pred(&self) -> SAI1_CLK_PREDR {
        SAI1_CLK_PREDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - Divider for flexio2 clock."]
    #[inline]
    pub fn flexio2_clk_pred(&self) -> FLEXIO2_CLK_PREDR {
        FLEXIO2_CLK_PREDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Divider for sai3 clock podf"]
    #[inline]
    pub fn sai3_clk_podf(&self) -> SAI3_CLK_PODFR {
        SAI3_CLK_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:24 - Divider for sai3 clock pred."]
    #[inline]
    pub fn sai3_clk_pred(&self) -> SAI3_CLK_PREDR {
        SAI3_CLK_PREDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:27 - Divider for flexio2 clock."]
    #[inline]
    pub fn flexio2_clk_podf(&self) -> FLEXIO2_CLK_PODFR {
        FLEXIO2_CLK_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 247530177 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Divider for sai1 clock podf"]
    #[inline]
    pub fn sai1_clk_podf(&mut self) -> _SAI1_CLK_PODFW {
        _SAI1_CLK_PODFW { w: self }
    }
    #[doc = "Bits 6:8 - Divider for sai1 clock pred."]
    #[inline]
    pub fn sai1_clk_pred(&mut self) -> _SAI1_CLK_PREDW {
        _SAI1_CLK_PREDW { w: self }
    }
    #[doc = "Bits 9:11 - Divider for flexio2 clock."]
    #[inline]
    pub fn flexio2_clk_pred(&mut self) -> _FLEXIO2_CLK_PREDW {
        _FLEXIO2_CLK_PREDW { w: self }
    }
    #[doc = "Bits 16:21 - Divider for sai3 clock podf"]
    #[inline]
    pub fn sai3_clk_podf(&mut self) -> _SAI3_CLK_PODFW {
        _SAI3_CLK_PODFW { w: self }
    }
    #[doc = "Bits 22:24 - Divider for sai3 clock pred."]
    #[inline]
    pub fn sai3_clk_pred(&mut self) -> _SAI3_CLK_PREDW {
        _SAI3_CLK_PREDW { w: self }
    }
    #[doc = "Bits 25:27 - Divider for flexio2 clock."]
    #[inline]
    pub fn flexio2_clk_podf(&mut self) -> _FLEXIO2_CLK_PODFW {
        _FLEXIO2_CLK_PODFW { w: self }
    }
}
