#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCR {
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
#[doc = "Possible values of the field `MUX_A8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_A8R {
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_0,
    #[doc = "NAND CE#"]
    MUX_A8_1,
    #[doc = "NOR CE#"]
    MUX_A8_2,
    #[doc = "PSRAM CE#"]
    MUX_A8_3,
    #[doc = "DBI CSX"]
    MUX_A8_4,
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_5,
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_6,
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_7,
}
impl MUX_A8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUX_A8R::MUX_A8_0 => 0,
            MUX_A8R::MUX_A8_1 => 1,
            MUX_A8R::MUX_A8_2 => 2,
            MUX_A8R::MUX_A8_3 => 3,
            MUX_A8R::MUX_A8_4 => 4,
            MUX_A8R::MUX_A8_5 => 5,
            MUX_A8R::MUX_A8_6 => 6,
            MUX_A8R::MUX_A8_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUX_A8R {
        match value {
            0 => MUX_A8R::MUX_A8_0,
            1 => MUX_A8R::MUX_A8_1,
            2 => MUX_A8R::MUX_A8_2,
            3 => MUX_A8R::MUX_A8_3,
            4 => MUX_A8R::MUX_A8_4,
            5 => MUX_A8R::MUX_A8_5,
            6 => MUX_A8R::MUX_A8_6,
            7 => MUX_A8R::MUX_A8_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_A8_0`"]
    #[inline]
    pub fn is_mux_a8_0(&self) -> bool {
        *self == MUX_A8R::MUX_A8_0
    }
    #[doc = "Checks if the value of the field is `MUX_A8_1`"]
    #[inline]
    pub fn is_mux_a8_1(&self) -> bool {
        *self == MUX_A8R::MUX_A8_1
    }
    #[doc = "Checks if the value of the field is `MUX_A8_2`"]
    #[inline]
    pub fn is_mux_a8_2(&self) -> bool {
        *self == MUX_A8R::MUX_A8_2
    }
    #[doc = "Checks if the value of the field is `MUX_A8_3`"]
    #[inline]
    pub fn is_mux_a8_3(&self) -> bool {
        *self == MUX_A8R::MUX_A8_3
    }
    #[doc = "Checks if the value of the field is `MUX_A8_4`"]
    #[inline]
    pub fn is_mux_a8_4(&self) -> bool {
        *self == MUX_A8R::MUX_A8_4
    }
    #[doc = "Checks if the value of the field is `MUX_A8_5`"]
    #[inline]
    pub fn is_mux_a8_5(&self) -> bool {
        *self == MUX_A8R::MUX_A8_5
    }
    #[doc = "Checks if the value of the field is `MUX_A8_6`"]
    #[inline]
    pub fn is_mux_a8_6(&self) -> bool {
        *self == MUX_A8R::MUX_A8_6
    }
    #[doc = "Checks if the value of the field is `MUX_A8_7`"]
    #[inline]
    pub fn is_mux_a8_7(&self) -> bool {
        *self == MUX_A8R::MUX_A8_7
    }
}
#[doc = "Possible values of the field `MUX_CSX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_CSX0R {
    #[doc = "NOR/PSRAM Address bit 24 (A24)"]
    MUX_CSX0_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX0_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX0_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX0_3,
    #[doc = "NAND CE#"]
    MUX_CSX0_4,
    #[doc = "NOR CE#"]
    MUX_CSX0_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX0_6,
    #[doc = "DBI CSX"]
    MUX_CSX0_7,
}
impl MUX_CSX0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUX_CSX0R::MUX_CSX0_0 => 0,
            MUX_CSX0R::MUX_CSX0_1 => 1,
            MUX_CSX0R::MUX_CSX0_2 => 2,
            MUX_CSX0R::MUX_CSX0_3 => 3,
            MUX_CSX0R::MUX_CSX0_4 => 4,
            MUX_CSX0R::MUX_CSX0_5 => 5,
            MUX_CSX0R::MUX_CSX0_6 => 6,
            MUX_CSX0R::MUX_CSX0_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUX_CSX0R {
        match value {
            0 => MUX_CSX0R::MUX_CSX0_0,
            1 => MUX_CSX0R::MUX_CSX0_1,
            2 => MUX_CSX0R::MUX_CSX0_2,
            3 => MUX_CSX0R::MUX_CSX0_3,
            4 => MUX_CSX0R::MUX_CSX0_4,
            5 => MUX_CSX0R::MUX_CSX0_5,
            6 => MUX_CSX0R::MUX_CSX0_6,
            7 => MUX_CSX0R::MUX_CSX0_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_0`"]
    #[inline]
    pub fn is_mux_csx0_0(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_1`"]
    #[inline]
    pub fn is_mux_csx0_1(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_2`"]
    #[inline]
    pub fn is_mux_csx0_2(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_3`"]
    #[inline]
    pub fn is_mux_csx0_3(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_4`"]
    #[inline]
    pub fn is_mux_csx0_4(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_5`"]
    #[inline]
    pub fn is_mux_csx0_5(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_6`"]
    #[inline]
    pub fn is_mux_csx0_6(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_7`"]
    #[inline]
    pub fn is_mux_csx0_7(&self) -> bool {
        *self == MUX_CSX0R::MUX_CSX0_7
    }
}
#[doc = "Possible values of the field `MUX_CSX1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_CSX1R {
    #[doc = "NOR/PSRAM Address bit 25 (A25)"]
    MUX_CSX1_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX1_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX1_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX1_3,
    #[doc = "NAND CE#"]
    MUX_CSX1_4,
    #[doc = "NOR CE#"]
    MUX_CSX1_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX1_6,
    #[doc = "DBI CSX"]
    MUX_CSX1_7,
}
impl MUX_CSX1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUX_CSX1R::MUX_CSX1_0 => 0,
            MUX_CSX1R::MUX_CSX1_1 => 1,
            MUX_CSX1R::MUX_CSX1_2 => 2,
            MUX_CSX1R::MUX_CSX1_3 => 3,
            MUX_CSX1R::MUX_CSX1_4 => 4,
            MUX_CSX1R::MUX_CSX1_5 => 5,
            MUX_CSX1R::MUX_CSX1_6 => 6,
            MUX_CSX1R::MUX_CSX1_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUX_CSX1R {
        match value {
            0 => MUX_CSX1R::MUX_CSX1_0,
            1 => MUX_CSX1R::MUX_CSX1_1,
            2 => MUX_CSX1R::MUX_CSX1_2,
            3 => MUX_CSX1R::MUX_CSX1_3,
            4 => MUX_CSX1R::MUX_CSX1_4,
            5 => MUX_CSX1R::MUX_CSX1_5,
            6 => MUX_CSX1R::MUX_CSX1_6,
            7 => MUX_CSX1R::MUX_CSX1_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_0`"]
    #[inline]
    pub fn is_mux_csx1_0(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_1`"]
    #[inline]
    pub fn is_mux_csx1_1(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_2`"]
    #[inline]
    pub fn is_mux_csx1_2(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_3`"]
    #[inline]
    pub fn is_mux_csx1_3(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_4`"]
    #[inline]
    pub fn is_mux_csx1_4(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_5`"]
    #[inline]
    pub fn is_mux_csx1_5(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_6`"]
    #[inline]
    pub fn is_mux_csx1_6(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_7`"]
    #[inline]
    pub fn is_mux_csx1_7(&self) -> bool {
        *self == MUX_CSX1R::MUX_CSX1_7
    }
}
#[doc = "Possible values of the field `MUX_CSX2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_CSX2R {
    #[doc = "NOR/PSRAM Address bit 26 (A26)"]
    MUX_CSX2_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX2_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX2_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX2_3,
    #[doc = "NAND CE#"]
    MUX_CSX2_4,
    #[doc = "NOR CE#"]
    MUX_CSX2_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX2_6,
    #[doc = "DBI CSX"]
    MUX_CSX2_7,
}
impl MUX_CSX2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUX_CSX2R::MUX_CSX2_0 => 0,
            MUX_CSX2R::MUX_CSX2_1 => 1,
            MUX_CSX2R::MUX_CSX2_2 => 2,
            MUX_CSX2R::MUX_CSX2_3 => 3,
            MUX_CSX2R::MUX_CSX2_4 => 4,
            MUX_CSX2R::MUX_CSX2_5 => 5,
            MUX_CSX2R::MUX_CSX2_6 => 6,
            MUX_CSX2R::MUX_CSX2_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUX_CSX2R {
        match value {
            0 => MUX_CSX2R::MUX_CSX2_0,
            1 => MUX_CSX2R::MUX_CSX2_1,
            2 => MUX_CSX2R::MUX_CSX2_2,
            3 => MUX_CSX2R::MUX_CSX2_3,
            4 => MUX_CSX2R::MUX_CSX2_4,
            5 => MUX_CSX2R::MUX_CSX2_5,
            6 => MUX_CSX2R::MUX_CSX2_6,
            7 => MUX_CSX2R::MUX_CSX2_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_0`"]
    #[inline]
    pub fn is_mux_csx2_0(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_1`"]
    #[inline]
    pub fn is_mux_csx2_1(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_2`"]
    #[inline]
    pub fn is_mux_csx2_2(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_3`"]
    #[inline]
    pub fn is_mux_csx2_3(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_4`"]
    #[inline]
    pub fn is_mux_csx2_4(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_5`"]
    #[inline]
    pub fn is_mux_csx2_5(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_6`"]
    #[inline]
    pub fn is_mux_csx2_6(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_7`"]
    #[inline]
    pub fn is_mux_csx2_7(&self) -> bool {
        *self == MUX_CSX2R::MUX_CSX2_7
    }
}
#[doc = "Possible values of the field `MUX_CSX3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_CSX3R {
    #[doc = "NOR/PSRAM Address bit 27 (A27)"]
    MUX_CSX3_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX3_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX3_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX3_3,
    #[doc = "NAND CE#"]
    MUX_CSX3_4,
    #[doc = "NOR CE#"]
    MUX_CSX3_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX3_6,
    #[doc = "DBI CSX"]
    MUX_CSX3_7,
}
impl MUX_CSX3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUX_CSX3R::MUX_CSX3_0 => 0,
            MUX_CSX3R::MUX_CSX3_1 => 1,
            MUX_CSX3R::MUX_CSX3_2 => 2,
            MUX_CSX3R::MUX_CSX3_3 => 3,
            MUX_CSX3R::MUX_CSX3_4 => 4,
            MUX_CSX3R::MUX_CSX3_5 => 5,
            MUX_CSX3R::MUX_CSX3_6 => 6,
            MUX_CSX3R::MUX_CSX3_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUX_CSX3R {
        match value {
            0 => MUX_CSX3R::MUX_CSX3_0,
            1 => MUX_CSX3R::MUX_CSX3_1,
            2 => MUX_CSX3R::MUX_CSX3_2,
            3 => MUX_CSX3R::MUX_CSX3_3,
            4 => MUX_CSX3R::MUX_CSX3_4,
            5 => MUX_CSX3R::MUX_CSX3_5,
            6 => MUX_CSX3R::MUX_CSX3_6,
            7 => MUX_CSX3R::MUX_CSX3_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_0`"]
    #[inline]
    pub fn is_mux_csx3_0(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_1`"]
    #[inline]
    pub fn is_mux_csx3_1(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_2`"]
    #[inline]
    pub fn is_mux_csx3_2(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_3`"]
    #[inline]
    pub fn is_mux_csx3_3(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_4`"]
    #[inline]
    pub fn is_mux_csx3_4(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_5`"]
    #[inline]
    pub fn is_mux_csx3_5(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_6`"]
    #[inline]
    pub fn is_mux_csx3_6(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_7`"]
    #[inline]
    pub fn is_mux_csx3_7(&self) -> bool {
        *self == MUX_CSX3R::MUX_CSX3_7
    }
}
#[doc = "Possible values of the field `MUX_RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_RDYR {
    #[doc = "NAND Ready/Wait# input"]
    MUX_RDY_0,
    #[doc = "SDRAM CS1"]
    MUX_RDY_1,
    #[doc = "SDRAM CS2"]
    MUX_RDY_2,
    #[doc = "SDRAM CS3"]
    MUX_RDY_3,
    #[doc = "NOR CE#"]
    MUX_RDY_4,
    #[doc = "PSRAM CE#"]
    MUX_RDY_5,
    #[doc = "DBI CSX"]
    MUX_RDY_6,
    #[doc = "NOR/PSRAM Address bit 27"]
    MUX_RDY_7,
}
impl MUX_RDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUX_RDYR::MUX_RDY_0 => 0,
            MUX_RDYR::MUX_RDY_1 => 1,
            MUX_RDYR::MUX_RDY_2 => 2,
            MUX_RDYR::MUX_RDY_3 => 3,
            MUX_RDYR::MUX_RDY_4 => 4,
            MUX_RDYR::MUX_RDY_5 => 5,
            MUX_RDYR::MUX_RDY_6 => 6,
            MUX_RDYR::MUX_RDY_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUX_RDYR {
        match value {
            0 => MUX_RDYR::MUX_RDY_0,
            1 => MUX_RDYR::MUX_RDY_1,
            2 => MUX_RDYR::MUX_RDY_2,
            3 => MUX_RDYR::MUX_RDY_3,
            4 => MUX_RDYR::MUX_RDY_4,
            5 => MUX_RDYR::MUX_RDY_5,
            6 => MUX_RDYR::MUX_RDY_6,
            7 => MUX_RDYR::MUX_RDY_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_0`"]
    #[inline]
    pub fn is_mux_rdy_0(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_0
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_1`"]
    #[inline]
    pub fn is_mux_rdy_1(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_1
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_2`"]
    #[inline]
    pub fn is_mux_rdy_2(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_2
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_3`"]
    #[inline]
    pub fn is_mux_rdy_3(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_3
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_4`"]
    #[inline]
    pub fn is_mux_rdy_4(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_4
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_5`"]
    #[inline]
    pub fn is_mux_rdy_5(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_5
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_6`"]
    #[inline]
    pub fn is_mux_rdy_6(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_6
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_7`"]
    #[inline]
    pub fn is_mux_rdy_7(&self) -> bool {
        *self == MUX_RDYR::MUX_RDY_7
    }
}
#[doc = "Values that can be written to the field `MUX_A8`"]
pub enum MUX_A8W {
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_0,
    #[doc = "NAND CE#"]
    MUX_A8_1,
    #[doc = "NOR CE#"]
    MUX_A8_2,
    #[doc = "PSRAM CE#"]
    MUX_A8_3,
    #[doc = "DBI CSX"]
    MUX_A8_4,
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_5,
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_6,
    #[doc = "SDRAM Address bit (A8)"]
    MUX_A8_7,
}
impl MUX_A8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUX_A8W::MUX_A8_0 => 0,
            MUX_A8W::MUX_A8_1 => 1,
            MUX_A8W::MUX_A8_2 => 2,
            MUX_A8W::MUX_A8_3 => 3,
            MUX_A8W::MUX_A8_4 => 4,
            MUX_A8W::MUX_A8_5 => 5,
            MUX_A8W::MUX_A8_6 => 6,
            MUX_A8W::MUX_A8_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX_A8W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_A8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX_A8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline]
    pub fn mux_a8_0(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_0)
    }
    #[doc = "NAND CE#"]
    #[inline]
    pub fn mux_a8_1(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_1)
    }
    #[doc = "NOR CE#"]
    #[inline]
    pub fn mux_a8_2(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_2)
    }
    #[doc = "PSRAM CE#"]
    #[inline]
    pub fn mux_a8_3(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_3)
    }
    #[doc = "DBI CSX"]
    #[inline]
    pub fn mux_a8_4(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_4)
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline]
    pub fn mux_a8_5(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_5)
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline]
    pub fn mux_a8_6(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_6)
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline]
    pub fn mux_a8_7(self) -> &'a mut W {
        self.variant(MUX_A8W::MUX_A8_7)
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
#[doc = "Values that can be written to the field `MUX_CSX0`"]
pub enum MUX_CSX0W {
    #[doc = "NOR/PSRAM Address bit 24 (A24)"]
    MUX_CSX0_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX0_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX0_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX0_3,
    #[doc = "NAND CE#"]
    MUX_CSX0_4,
    #[doc = "NOR CE#"]
    MUX_CSX0_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX0_6,
    #[doc = "DBI CSX"]
    MUX_CSX0_7,
}
impl MUX_CSX0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUX_CSX0W::MUX_CSX0_0 => 0,
            MUX_CSX0W::MUX_CSX0_1 => 1,
            MUX_CSX0W::MUX_CSX0_2 => 2,
            MUX_CSX0W::MUX_CSX0_3 => 3,
            MUX_CSX0W::MUX_CSX0_4 => 4,
            MUX_CSX0W::MUX_CSX0_5 => 5,
            MUX_CSX0W::MUX_CSX0_6 => 6,
            MUX_CSX0W::MUX_CSX0_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX_CSX0W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_CSX0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX_CSX0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NOR/PSRAM Address bit 24 (A24)"]
    #[inline]
    pub fn mux_csx0_0(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline]
    pub fn mux_csx0_1(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline]
    pub fn mux_csx0_2(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline]
    pub fn mux_csx0_3(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_3)
    }
    #[doc = "NAND CE#"]
    #[inline]
    pub fn mux_csx0_4(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_4)
    }
    #[doc = "NOR CE#"]
    #[inline]
    pub fn mux_csx0_5(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline]
    pub fn mux_csx0_6(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_6)
    }
    #[doc = "DBI CSX"]
    #[inline]
    pub fn mux_csx0_7(self) -> &'a mut W {
        self.variant(MUX_CSX0W::MUX_CSX0_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MUX_CSX1`"]
pub enum MUX_CSX1W {
    #[doc = "NOR/PSRAM Address bit 25 (A25)"]
    MUX_CSX1_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX1_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX1_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX1_3,
    #[doc = "NAND CE#"]
    MUX_CSX1_4,
    #[doc = "NOR CE#"]
    MUX_CSX1_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX1_6,
    #[doc = "DBI CSX"]
    MUX_CSX1_7,
}
impl MUX_CSX1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUX_CSX1W::MUX_CSX1_0 => 0,
            MUX_CSX1W::MUX_CSX1_1 => 1,
            MUX_CSX1W::MUX_CSX1_2 => 2,
            MUX_CSX1W::MUX_CSX1_3 => 3,
            MUX_CSX1W::MUX_CSX1_4 => 4,
            MUX_CSX1W::MUX_CSX1_5 => 5,
            MUX_CSX1W::MUX_CSX1_6 => 6,
            MUX_CSX1W::MUX_CSX1_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX_CSX1W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_CSX1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX_CSX1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NOR/PSRAM Address bit 25 (A25)"]
    #[inline]
    pub fn mux_csx1_0(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline]
    pub fn mux_csx1_1(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline]
    pub fn mux_csx1_2(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline]
    pub fn mux_csx1_3(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_3)
    }
    #[doc = "NAND CE#"]
    #[inline]
    pub fn mux_csx1_4(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_4)
    }
    #[doc = "NOR CE#"]
    #[inline]
    pub fn mux_csx1_5(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline]
    pub fn mux_csx1_6(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_6)
    }
    #[doc = "DBI CSX"]
    #[inline]
    pub fn mux_csx1_7(self) -> &'a mut W {
        self.variant(MUX_CSX1W::MUX_CSX1_7)
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
#[doc = "Values that can be written to the field `MUX_CSX2`"]
pub enum MUX_CSX2W {
    #[doc = "NOR/PSRAM Address bit 26 (A26)"]
    MUX_CSX2_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX2_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX2_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX2_3,
    #[doc = "NAND CE#"]
    MUX_CSX2_4,
    #[doc = "NOR CE#"]
    MUX_CSX2_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX2_6,
    #[doc = "DBI CSX"]
    MUX_CSX2_7,
}
impl MUX_CSX2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUX_CSX2W::MUX_CSX2_0 => 0,
            MUX_CSX2W::MUX_CSX2_1 => 1,
            MUX_CSX2W::MUX_CSX2_2 => 2,
            MUX_CSX2W::MUX_CSX2_3 => 3,
            MUX_CSX2W::MUX_CSX2_4 => 4,
            MUX_CSX2W::MUX_CSX2_5 => 5,
            MUX_CSX2W::MUX_CSX2_6 => 6,
            MUX_CSX2W::MUX_CSX2_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX_CSX2W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_CSX2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX_CSX2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NOR/PSRAM Address bit 26 (A26)"]
    #[inline]
    pub fn mux_csx2_0(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline]
    pub fn mux_csx2_1(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline]
    pub fn mux_csx2_2(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline]
    pub fn mux_csx2_3(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_3)
    }
    #[doc = "NAND CE#"]
    #[inline]
    pub fn mux_csx2_4(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_4)
    }
    #[doc = "NOR CE#"]
    #[inline]
    pub fn mux_csx2_5(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline]
    pub fn mux_csx2_6(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_6)
    }
    #[doc = "DBI CSX"]
    #[inline]
    pub fn mux_csx2_7(self) -> &'a mut W {
        self.variant(MUX_CSX2W::MUX_CSX2_7)
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
#[doc = "Values that can be written to the field `MUX_CSX3`"]
pub enum MUX_CSX3W {
    #[doc = "NOR/PSRAM Address bit 27 (A27)"]
    MUX_CSX3_0,
    #[doc = "SDRAM CS1"]
    MUX_CSX3_1,
    #[doc = "SDRAM CS2"]
    MUX_CSX3_2,
    #[doc = "SDRAM CS3"]
    MUX_CSX3_3,
    #[doc = "NAND CE#"]
    MUX_CSX3_4,
    #[doc = "NOR CE#"]
    MUX_CSX3_5,
    #[doc = "PSRAM CE#"]
    MUX_CSX3_6,
    #[doc = "DBI CSX"]
    MUX_CSX3_7,
}
impl MUX_CSX3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUX_CSX3W::MUX_CSX3_0 => 0,
            MUX_CSX3W::MUX_CSX3_1 => 1,
            MUX_CSX3W::MUX_CSX3_2 => 2,
            MUX_CSX3W::MUX_CSX3_3 => 3,
            MUX_CSX3W::MUX_CSX3_4 => 4,
            MUX_CSX3W::MUX_CSX3_5 => 5,
            MUX_CSX3W::MUX_CSX3_6 => 6,
            MUX_CSX3W::MUX_CSX3_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX_CSX3W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_CSX3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX_CSX3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NOR/PSRAM Address bit 27 (A27)"]
    #[inline]
    pub fn mux_csx3_0(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline]
    pub fn mux_csx3_1(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline]
    pub fn mux_csx3_2(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline]
    pub fn mux_csx3_3(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_3)
    }
    #[doc = "NAND CE#"]
    #[inline]
    pub fn mux_csx3_4(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_4)
    }
    #[doc = "NOR CE#"]
    #[inline]
    pub fn mux_csx3_5(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline]
    pub fn mux_csx3_6(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_6)
    }
    #[doc = "DBI CSX"]
    #[inline]
    pub fn mux_csx3_7(self) -> &'a mut W {
        self.variant(MUX_CSX3W::MUX_CSX3_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MUX_RDY`"]
pub enum MUX_RDYW {
    #[doc = "NAND Ready/Wait# input"]
    MUX_RDY_0,
    #[doc = "SDRAM CS1"]
    MUX_RDY_1,
    #[doc = "SDRAM CS2"]
    MUX_RDY_2,
    #[doc = "SDRAM CS3"]
    MUX_RDY_3,
    #[doc = "NOR CE#"]
    MUX_RDY_4,
    #[doc = "PSRAM CE#"]
    MUX_RDY_5,
    #[doc = "DBI CSX"]
    MUX_RDY_6,
    #[doc = "NOR/PSRAM Address bit 27"]
    MUX_RDY_7,
}
impl MUX_RDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUX_RDYW::MUX_RDY_0 => 0,
            MUX_RDYW::MUX_RDY_1 => 1,
            MUX_RDYW::MUX_RDY_2 => 2,
            MUX_RDYW::MUX_RDY_3 => 3,
            MUX_RDYW::MUX_RDY_4 => 4,
            MUX_RDYW::MUX_RDY_5 => 5,
            MUX_RDYW::MUX_RDY_6 => 6,
            MUX_RDYW::MUX_RDY_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX_RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_RDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX_RDYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NAND Ready/Wait# input"]
    #[inline]
    pub fn mux_rdy_0(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline]
    pub fn mux_rdy_1(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline]
    pub fn mux_rdy_2(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline]
    pub fn mux_rdy_3(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_3)
    }
    #[doc = "NOR CE#"]
    #[inline]
    pub fn mux_rdy_4(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_4)
    }
    #[doc = "PSRAM CE#"]
    #[inline]
    pub fn mux_rdy_5(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_5)
    }
    #[doc = "DBI CSX"]
    #[inline]
    pub fn mux_rdy_6(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_6)
    }
    #[doc = "NOR/PSRAM Address bit 27"]
    #[inline]
    pub fn mux_rdy_7(self) -> &'a mut W {
        self.variant(MUX_RDYW::MUX_RDY_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:2 - SEMC_A8 output selection"]
    #[inline]
    pub fn mux_a8(&self) -> MUX_A8R {
        MUX_A8R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - SEMC_CSX0 output selection"]
    #[inline]
    pub fn mux_csx0(&self) -> MUX_CSX0R {
        MUX_CSX0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - SEMC_CSX1 output selection"]
    #[inline]
    pub fn mux_csx1(&self) -> MUX_CSX1R {
        MUX_CSX1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - SEMC_CSX2 output selection"]
    #[inline]
    pub fn mux_csx2(&self) -> MUX_CSX2R {
        MUX_CSX2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - SEMC_CSX3 output selection"]
    #[inline]
    pub fn mux_csx3(&self) -> MUX_CSX3R {
        MUX_CSX3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:17 - SEMC_RDY function selection"]
    #[inline]
    pub fn mux_rdy(&self) -> MUX_RDYR {
        MUX_RDYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:2 - SEMC_A8 output selection"]
    #[inline]
    pub fn mux_a8(&mut self) -> _MUX_A8W {
        _MUX_A8W { w: self }
    }
    #[doc = "Bits 3:5 - SEMC_CSX0 output selection"]
    #[inline]
    pub fn mux_csx0(&mut self) -> _MUX_CSX0W {
        _MUX_CSX0W { w: self }
    }
    #[doc = "Bits 6:8 - SEMC_CSX1 output selection"]
    #[inline]
    pub fn mux_csx1(&mut self) -> _MUX_CSX1W {
        _MUX_CSX1W { w: self }
    }
    #[doc = "Bits 9:11 - SEMC_CSX2 output selection"]
    #[inline]
    pub fn mux_csx2(&mut self) -> _MUX_CSX2W {
        _MUX_CSX2W { w: self }
    }
    #[doc = "Bits 12:14 - SEMC_CSX3 output selection"]
    #[inline]
    pub fn mux_csx3(&mut self) -> _MUX_CSX3W {
        _MUX_CSX3W { w: self }
    }
    #[doc = "Bits 15:17 - SEMC_RDY function selection"]
    #[inline]
    pub fn mux_rdy(&mut self) -> _MUX_RDYW {
        _MUX_RDYW { w: self }
    }
}
