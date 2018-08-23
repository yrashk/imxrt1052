#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CDHIPR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SEMC_PODF_BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_PODF_BUSYR {
    #[doc = "divider is not busy and its value represents the actual division."]
    SEMC_PODF_BUSY_0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the semc_podf will be applied."]
    SEMC_PODF_BUSY_1,
}
impl SEMC_PODF_BUSYR {
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
            SEMC_PODF_BUSYR::SEMC_PODF_BUSY_0 => false,
            SEMC_PODF_BUSYR::SEMC_PODF_BUSY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEMC_PODF_BUSYR {
        match value {
            false => SEMC_PODF_BUSYR::SEMC_PODF_BUSY_0,
            true => SEMC_PODF_BUSYR::SEMC_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_BUSY_0`"]
    #[inline]
    pub fn is_semc_podf_busy_0(&self) -> bool {
        *self == SEMC_PODF_BUSYR::SEMC_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_BUSY_1`"]
    #[inline]
    pub fn is_semc_podf_busy_1(&self) -> bool {
        *self == SEMC_PODF_BUSYR::SEMC_PODF_BUSY_1
    }
}
#[doc = "Possible values of the field `AHB_PODF_BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_PODF_BUSYR {
    #[doc = "divider is not busy and its value represents the actual division."]
    AHB_PODF_BUSY_0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the ahb_podf will be applied."]
    AHB_PODF_BUSY_1,
}
impl AHB_PODF_BUSYR {
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
            AHB_PODF_BUSYR::AHB_PODF_BUSY_0 => false,
            AHB_PODF_BUSYR::AHB_PODF_BUSY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_PODF_BUSYR {
        match value {
            false => AHB_PODF_BUSYR::AHB_PODF_BUSY_0,
            true => AHB_PODF_BUSYR::AHB_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_BUSY_0`"]
    #[inline]
    pub fn is_ahb_podf_busy_0(&self) -> bool {
        *self == AHB_PODF_BUSYR::AHB_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_BUSY_1`"]
    #[inline]
    pub fn is_ahb_podf_busy_1(&self) -> bool {
        *self == AHB_PODF_BUSYR::AHB_PODF_BUSY_1
    }
}
#[doc = "Possible values of the field `PERIPH2_CLK_SEL_BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH2_CLK_SEL_BUSYR {
    #[doc = "mux is not busy and its value represents the actual division."]
    PERIPH2_CLK_SEL_BUSY_0,
    #[doc = "mux is busy with handshake process with module. The value read in the periph2_clk_sel represents the previous value of select, and after the handshake periph2_clk_sel value will be applied."]
    PERIPH2_CLK_SEL_BUSY_1,
}
impl PERIPH2_CLK_SEL_BUSYR {
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
            PERIPH2_CLK_SEL_BUSYR::PERIPH2_CLK_SEL_BUSY_0 => false,
            PERIPH2_CLK_SEL_BUSYR::PERIPH2_CLK_SEL_BUSY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERIPH2_CLK_SEL_BUSYR {
        match value {
            false => PERIPH2_CLK_SEL_BUSYR::PERIPH2_CLK_SEL_BUSY_0,
            true => PERIPH2_CLK_SEL_BUSYR::PERIPH2_CLK_SEL_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_BUSY_0`"]
    #[inline]
    pub fn is_periph2_clk_sel_busy_0(&self) -> bool {
        *self == PERIPH2_CLK_SEL_BUSYR::PERIPH2_CLK_SEL_BUSY_0
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_BUSY_1`"]
    #[inline]
    pub fn is_periph2_clk_sel_busy_1(&self) -> bool {
        *self == PERIPH2_CLK_SEL_BUSYR::PERIPH2_CLK_SEL_BUSY_1
    }
}
#[doc = "Possible values of the field `PERIPH_CLK_SEL_BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK_SEL_BUSYR {
    #[doc = "mux is not busy and its value represents the actual division."]
    PERIPH_CLK_SEL_BUSY_0,
    #[doc = "mux is busy with handshake process with module. The value read in the periph_clk_sel represents the previous value of select, and after the handshake periph_clk_sel value will be applied."]
    PERIPH_CLK_SEL_BUSY_1,
}
impl PERIPH_CLK_SEL_BUSYR {
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
            PERIPH_CLK_SEL_BUSYR::PERIPH_CLK_SEL_BUSY_0 => false,
            PERIPH_CLK_SEL_BUSYR::PERIPH_CLK_SEL_BUSY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERIPH_CLK_SEL_BUSYR {
        match value {
            false => PERIPH_CLK_SEL_BUSYR::PERIPH_CLK_SEL_BUSY_0,
            true => PERIPH_CLK_SEL_BUSYR::PERIPH_CLK_SEL_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_BUSY_0`"]
    #[inline]
    pub fn is_periph_clk_sel_busy_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_BUSYR::PERIPH_CLK_SEL_BUSY_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_BUSY_1`"]
    #[inline]
    pub fn is_periph_clk_sel_busy_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_BUSYR::PERIPH_CLK_SEL_BUSY_1
    }
}
#[doc = "Possible values of the field `ARM_PODF_BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_PODF_BUSYR {
    #[doc = "divider is not busy and its value represents the actual division."]
    ARM_PODF_BUSY_0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the arm_podf will be applied."]
    ARM_PODF_BUSY_1,
}
impl ARM_PODF_BUSYR {
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
            ARM_PODF_BUSYR::ARM_PODF_BUSY_0 => false,
            ARM_PODF_BUSYR::ARM_PODF_BUSY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARM_PODF_BUSYR {
        match value {
            false => ARM_PODF_BUSYR::ARM_PODF_BUSY_0,
            true => ARM_PODF_BUSYR::ARM_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_BUSY_0`"]
    #[inline]
    pub fn is_arm_podf_busy_0(&self) -> bool {
        *self == ARM_PODF_BUSYR::ARM_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_BUSY_1`"]
    #[inline]
    pub fn is_arm_podf_busy_1(&self) -> bool {
        *self == ARM_PODF_BUSYR::ARM_PODF_BUSY_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Busy indicator for semc_podf."]
    #[inline]
    pub fn semc_podf_busy(&self) -> SEMC_PODF_BUSYR {
        SEMC_PODF_BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Busy indicator for ahb_podf."]
    #[inline]
    pub fn ahb_podf_busy(&self) -> AHB_PODF_BUSYR {
        AHB_PODF_BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Busy indicator for periph2_clk_sel mux control."]
    #[inline]
    pub fn periph2_clk_sel_busy(&self) -> PERIPH2_CLK_SEL_BUSYR {
        PERIPH2_CLK_SEL_BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Busy indicator for periph_clk_sel mux control."]
    #[inline]
    pub fn periph_clk_sel_busy(&self) -> PERIPH_CLK_SEL_BUSYR {
        PERIPH_CLK_SEL_BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Busy indicator for arm_podf."]
    #[inline]
    pub fn arm_podf_busy(&self) -> ARM_PODF_BUSYR {
        ARM_PODF_BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
