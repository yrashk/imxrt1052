#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSICR3 {
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
#[doc = "Possible values of the field `ECC_AUTO_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_AUTO_ENR {
    #[doc = "Auto Error correction is disabled."]
    ECC_AUTO_EN_0,
    #[doc = "Auto Error correction is enabled."]
    ECC_AUTO_EN_1,
}
impl ECC_AUTO_ENR {
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
            ECC_AUTO_ENR::ECC_AUTO_EN_0 => false,
            ECC_AUTO_ENR::ECC_AUTO_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECC_AUTO_ENR {
        match value {
            false => ECC_AUTO_ENR::ECC_AUTO_EN_0,
            true => ECC_AUTO_ENR::ECC_AUTO_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECC_AUTO_EN_0`"]
    #[inline]
    pub fn is_ecc_auto_en_0(&self) -> bool {
        *self == ECC_AUTO_ENR::ECC_AUTO_EN_0
    }
    #[doc = "Checks if the value of the field is `ECC_AUTO_EN_1`"]
    #[inline]
    pub fn is_ecc_auto_en_1(&self) -> bool {
        *self == ECC_AUTO_ENR::ECC_AUTO_EN_1
    }
}
#[doc = "Possible values of the field `ECC_INT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_INT_ENR {
    #[doc = "No interrupt is generated when error is detected. Only the status bit ECC_INT is set."]
    ECC_INT_EN_0,
    #[doc = "Interrupt is generated when error is detected."]
    ECC_INT_EN_1,
}
impl ECC_INT_ENR {
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
            ECC_INT_ENR::ECC_INT_EN_0 => false,
            ECC_INT_ENR::ECC_INT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECC_INT_ENR {
        match value {
            false => ECC_INT_ENR::ECC_INT_EN_0,
            true => ECC_INT_ENR::ECC_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECC_INT_EN_0`"]
    #[inline]
    pub fn is_ecc_int_en_0(&self) -> bool {
        *self == ECC_INT_ENR::ECC_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `ECC_INT_EN_1`"]
    #[inline]
    pub fn is_ecc_int_en_1(&self) -> bool {
        *self == ECC_INT_ENR::ECC_INT_EN_1
    }
}
#[doc = "Possible values of the field `ZERO_PACK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZERO_PACK_ENR {
    #[doc = "Zero packing disabled"]
    ZERO_PACK_EN_0,
    #[doc = "Zero packing enabled"]
    ZERO_PACK_EN_1,
}
impl ZERO_PACK_ENR {
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
            ZERO_PACK_ENR::ZERO_PACK_EN_0 => false,
            ZERO_PACK_ENR::ZERO_PACK_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZERO_PACK_ENR {
        match value {
            false => ZERO_PACK_ENR::ZERO_PACK_EN_0,
            true => ZERO_PACK_ENR::ZERO_PACK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO_PACK_EN_0`"]
    #[inline]
    pub fn is_zero_pack_en_0(&self) -> bool {
        *self == ZERO_PACK_ENR::ZERO_PACK_EN_0
    }
    #[doc = "Checks if the value of the field is `ZERO_PACK_EN_1`"]
    #[inline]
    pub fn is_zero_pack_en_1(&self) -> bool {
        *self == ZERO_PACK_ENR::ZERO_PACK_EN_1
    }
}
#[doc = "Possible values of the field `TWO_8BIT_SENSOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWO_8BIT_SENSORR {
    #[doc = "Only one sensor is connected."]
    TWO_8BIT_SENSOR_0,
    #[doc = "Two 8-bit sensors are connected or one 16-bit sensor is connected."]
    TWO_8BIT_SENSOR_1,
}
impl TWO_8BIT_SENSORR {
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
            TWO_8BIT_SENSORR::TWO_8BIT_SENSOR_0 => false,
            TWO_8BIT_SENSORR::TWO_8BIT_SENSOR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWO_8BIT_SENSORR {
        match value {
            false => TWO_8BIT_SENSORR::TWO_8BIT_SENSOR_0,
            true => TWO_8BIT_SENSORR::TWO_8BIT_SENSOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_8BIT_SENSOR_0`"]
    #[inline]
    pub fn is_two_8bit_sensor_0(&self) -> bool {
        *self == TWO_8BIT_SENSORR::TWO_8BIT_SENSOR_0
    }
    #[doc = "Checks if the value of the field is `TWO_8BIT_SENSOR_1`"]
    #[inline]
    pub fn is_two_8bit_sensor_1(&self) -> bool {
        *self == TWO_8BIT_SENSORR::TWO_8BIT_SENSOR_1
    }
}
#[doc = "Possible values of the field `RxFF_LEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_LEVELR {
    #[doc = "4 Words"]
    RXFF_LEVEL_0,
    #[doc = "8 Words"]
    RXFF_LEVEL_1,
    #[doc = "16 Words"]
    RXFF_LEVEL_2,
    #[doc = "24 Words"]
    RXFF_LEVEL_3,
    #[doc = "32 Words"]
    RXFF_LEVEL_4,
    #[doc = "48 Words"]
    RXFF_LEVEL_5,
    #[doc = "64 Words"]
    RXFF_LEVEL_6,
    #[doc = "96 Words"]
    RXFF_LEVEL_7,
}
impl RXFF_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXFF_LEVELR::RXFF_LEVEL_0 => 0,
            RXFF_LEVELR::RXFF_LEVEL_1 => 1,
            RXFF_LEVELR::RXFF_LEVEL_2 => 2,
            RXFF_LEVELR::RXFF_LEVEL_3 => 3,
            RXFF_LEVELR::RXFF_LEVEL_4 => 4,
            RXFF_LEVELR::RXFF_LEVEL_5 => 5,
            RXFF_LEVELR::RXFF_LEVEL_6 => 6,
            RXFF_LEVELR::RXFF_LEVEL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXFF_LEVELR {
        match value {
            0 => RXFF_LEVELR::RXFF_LEVEL_0,
            1 => RXFF_LEVELR::RXFF_LEVEL_1,
            2 => RXFF_LEVELR::RXFF_LEVEL_2,
            3 => RXFF_LEVELR::RXFF_LEVEL_3,
            4 => RXFF_LEVELR::RXFF_LEVEL_4,
            5 => RXFF_LEVELR::RXFF_LEVEL_5,
            6 => RXFF_LEVELR::RXFF_LEVEL_6,
            7 => RXFF_LEVELR::RXFF_LEVEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_0`"]
    #[inline]
    pub fn is_rx_ff_level_0(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_0
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_1`"]
    #[inline]
    pub fn is_rx_ff_level_1(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_1
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_2`"]
    #[inline]
    pub fn is_rx_ff_level_2(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_2
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_3`"]
    #[inline]
    pub fn is_rx_ff_level_3(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_3
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_4`"]
    #[inline]
    pub fn is_rx_ff_level_4(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_4
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_5`"]
    #[inline]
    pub fn is_rx_ff_level_5(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_5
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_6`"]
    #[inline]
    pub fn is_rx_ff_level_6(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_6
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_7`"]
    #[inline]
    pub fn is_rx_ff_level_7(&self) -> bool {
        *self == RXFF_LEVELR::RXFF_LEVEL_7
    }
}
#[doc = "Possible values of the field `HRESP_ERR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRESP_ERR_ENR {
    #[doc = "Disable hresponse error interrupt"]
    HRESP_ERR_EN_0,
    #[doc = "Enable hresponse error interrupt"]
    HRESP_ERR_EN_1,
}
impl HRESP_ERR_ENR {
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
            HRESP_ERR_ENR::HRESP_ERR_EN_0 => false,
            HRESP_ERR_ENR::HRESP_ERR_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRESP_ERR_ENR {
        match value {
            false => HRESP_ERR_ENR::HRESP_ERR_EN_0,
            true => HRESP_ERR_ENR::HRESP_ERR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_EN_0`"]
    #[inline]
    pub fn is_hresp_err_en_0(&self) -> bool {
        *self == HRESP_ERR_ENR::HRESP_ERR_EN_0
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_EN_1`"]
    #[inline]
    pub fn is_hresp_err_en_1(&self) -> bool {
        *self == HRESP_ERR_ENR::HRESP_ERR_EN_1
    }
}
#[doc = "Possible values of the field `STATFF_LEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATFF_LEVELR {
    #[doc = "4 Words"]
    STATFF_LEVEL_0,
    #[doc = "8 Words"]
    STATFF_LEVEL_1,
    #[doc = "12 Words"]
    STATFF_LEVEL_2,
    #[doc = "16 Words"]
    STATFF_LEVEL_3,
    #[doc = "24 Words"]
    STATFF_LEVEL_4,
    #[doc = "32 Words"]
    STATFF_LEVEL_5,
    #[doc = "48 Words"]
    STATFF_LEVEL_6,
    #[doc = "64 Words"]
    STATFF_LEVEL_7,
}
impl STATFF_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATFF_LEVELR::STATFF_LEVEL_0 => 0,
            STATFF_LEVELR::STATFF_LEVEL_1 => 1,
            STATFF_LEVELR::STATFF_LEVEL_2 => 2,
            STATFF_LEVELR::STATFF_LEVEL_3 => 3,
            STATFF_LEVELR::STATFF_LEVEL_4 => 4,
            STATFF_LEVELR::STATFF_LEVEL_5 => 5,
            STATFF_LEVELR::STATFF_LEVEL_6 => 6,
            STATFF_LEVELR::STATFF_LEVEL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATFF_LEVELR {
        match value {
            0 => STATFF_LEVELR::STATFF_LEVEL_0,
            1 => STATFF_LEVELR::STATFF_LEVEL_1,
            2 => STATFF_LEVELR::STATFF_LEVEL_2,
            3 => STATFF_LEVELR::STATFF_LEVEL_3,
            4 => STATFF_LEVELR::STATFF_LEVEL_4,
            5 => STATFF_LEVELR::STATFF_LEVEL_5,
            6 => STATFF_LEVELR::STATFF_LEVEL_6,
            7 => STATFF_LEVELR::STATFF_LEVEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_0`"]
    #[inline]
    pub fn is_statff_level_0(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_0
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_1`"]
    #[inline]
    pub fn is_statff_level_1(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_1
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_2`"]
    #[inline]
    pub fn is_statff_level_2(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_2
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_3`"]
    #[inline]
    pub fn is_statff_level_3(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_3
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_4`"]
    #[inline]
    pub fn is_statff_level_4(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_4
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_5`"]
    #[inline]
    pub fn is_statff_level_5(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_5
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_6`"]
    #[inline]
    pub fn is_statff_level_6(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_6
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_7`"]
    #[inline]
    pub fn is_statff_level_7(&self) -> bool {
        *self == STATFF_LEVELR::STATFF_LEVEL_7
    }
}
#[doc = "Possible values of the field `DMA_REQ_EN_SFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REQ_EN_SFFR {
    #[doc = "Disable the dma request"]
    DMA_REQ_EN_SFF_0,
    #[doc = "Enable the dma request"]
    DMA_REQ_EN_SFF_1,
}
impl DMA_REQ_EN_SFFR {
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
            DMA_REQ_EN_SFFR::DMA_REQ_EN_SFF_0 => false,
            DMA_REQ_EN_SFFR::DMA_REQ_EN_SFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_REQ_EN_SFFR {
        match value {
            false => DMA_REQ_EN_SFFR::DMA_REQ_EN_SFF_0,
            true => DMA_REQ_EN_SFFR::DMA_REQ_EN_SFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_SFF_0`"]
    #[inline]
    pub fn is_dma_req_en_sff_0(&self) -> bool {
        *self == DMA_REQ_EN_SFFR::DMA_REQ_EN_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_SFF_1`"]
    #[inline]
    pub fn is_dma_req_en_sff_1(&self) -> bool {
        *self == DMA_REQ_EN_SFFR::DMA_REQ_EN_SFF_1
    }
}
#[doc = "Possible values of the field `DMA_REQ_EN_RFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REQ_EN_RFFR {
    #[doc = "Disable the dma request"]
    DMA_REQ_EN_RFF_0,
    #[doc = "Enable the dma request"]
    DMA_REQ_EN_RFF_1,
}
impl DMA_REQ_EN_RFFR {
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
            DMA_REQ_EN_RFFR::DMA_REQ_EN_RFF_0 => false,
            DMA_REQ_EN_RFFR::DMA_REQ_EN_RFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_REQ_EN_RFFR {
        match value {
            false => DMA_REQ_EN_RFFR::DMA_REQ_EN_RFF_0,
            true => DMA_REQ_EN_RFFR::DMA_REQ_EN_RFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_RFF_0`"]
    #[inline]
    pub fn is_dma_req_en_rff_0(&self) -> bool {
        *self == DMA_REQ_EN_RFFR::DMA_REQ_EN_RFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_RFF_1`"]
    #[inline]
    pub fn is_dma_req_en_rff_1(&self) -> bool {
        *self == DMA_REQ_EN_RFFR::DMA_REQ_EN_RFF_1
    }
}
#[doc = "Possible values of the field `DMA_REFLASH_SFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REFLASH_SFFR {
    #[doc = "No reflashing"]
    DMA_REFLASH_SFF_0,
    #[doc = "Reflash the embedded DMA controller"]
    DMA_REFLASH_SFF_1,
}
impl DMA_REFLASH_SFFR {
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
            DMA_REFLASH_SFFR::DMA_REFLASH_SFF_0 => false,
            DMA_REFLASH_SFFR::DMA_REFLASH_SFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_REFLASH_SFFR {
        match value {
            false => DMA_REFLASH_SFFR::DMA_REFLASH_SFF_0,
            true => DMA_REFLASH_SFFR::DMA_REFLASH_SFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_SFF_0`"]
    #[inline]
    pub fn is_dma_reflash_sff_0(&self) -> bool {
        *self == DMA_REFLASH_SFFR::DMA_REFLASH_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_SFF_1`"]
    #[inline]
    pub fn is_dma_reflash_sff_1(&self) -> bool {
        *self == DMA_REFLASH_SFFR::DMA_REFLASH_SFF_1
    }
}
#[doc = "Possible values of the field `DMA_REFLASH_RFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REFLASH_RFFR {
    #[doc = "No reflashing"]
    DMA_REFLASH_RFF_0,
    #[doc = "Reflash the embedded DMA controller"]
    DMA_REFLASH_RFF_1,
}
impl DMA_REFLASH_RFFR {
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
            DMA_REFLASH_RFFR::DMA_REFLASH_RFF_0 => false,
            DMA_REFLASH_RFFR::DMA_REFLASH_RFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_REFLASH_RFFR {
        match value {
            false => DMA_REFLASH_RFFR::DMA_REFLASH_RFF_0,
            true => DMA_REFLASH_RFFR::DMA_REFLASH_RFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_RFF_0`"]
    #[inline]
    pub fn is_dma_reflash_rff_0(&self) -> bool {
        *self == DMA_REFLASH_RFFR::DMA_REFLASH_RFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_RFF_1`"]
    #[inline]
    pub fn is_dma_reflash_rff_1(&self) -> bool {
        *self == DMA_REFLASH_RFFR::DMA_REFLASH_RFF_1
    }
}
#[doc = "Possible values of the field `FRMCNT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMCNT_RSTR {
    #[doc = "Do not reset"]
    FRMCNT_RST_0,
    #[doc = "Reset frame counter immediately"]
    FRMCNT_RST_1,
}
impl FRMCNT_RSTR {
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
            FRMCNT_RSTR::FRMCNT_RST_0 => false,
            FRMCNT_RSTR::FRMCNT_RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRMCNT_RSTR {
        match value {
            false => FRMCNT_RSTR::FRMCNT_RST_0,
            true => FRMCNT_RSTR::FRMCNT_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRMCNT_RST_0`"]
    #[inline]
    pub fn is_frmcnt_rst_0(&self) -> bool {
        *self == FRMCNT_RSTR::FRMCNT_RST_0
    }
    #[doc = "Checks if the value of the field is `FRMCNT_RST_1`"]
    #[inline]
    pub fn is_frmcnt_rst_1(&self) -> bool {
        *self == FRMCNT_RSTR::FRMCNT_RST_1
    }
}
#[doc = r" Value of the field"]
pub struct FRMCNTR {
    bits: u16,
}
impl FRMCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ECC_AUTO_EN`"]
pub enum ECC_AUTO_ENW {
    #[doc = "Auto Error correction is disabled."]
    ECC_AUTO_EN_0,
    #[doc = "Auto Error correction is enabled."]
    ECC_AUTO_EN_1,
}
impl ECC_AUTO_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECC_AUTO_ENW::ECC_AUTO_EN_0 => false,
            ECC_AUTO_ENW::ECC_AUTO_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECC_AUTO_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ECC_AUTO_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECC_AUTO_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto Error correction is disabled."]
    #[inline]
    pub fn ecc_auto_en_0(self) -> &'a mut W {
        self.variant(ECC_AUTO_ENW::ECC_AUTO_EN_0)
    }
    #[doc = "Auto Error correction is enabled."]
    #[inline]
    pub fn ecc_auto_en_1(self) -> &'a mut W {
        self.variant(ECC_AUTO_ENW::ECC_AUTO_EN_1)
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
#[doc = "Values that can be written to the field `ECC_INT_EN`"]
pub enum ECC_INT_ENW {
    #[doc = "No interrupt is generated when error is detected. Only the status bit ECC_INT is set."]
    ECC_INT_EN_0,
    #[doc = "Interrupt is generated when error is detected."]
    ECC_INT_EN_1,
}
impl ECC_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECC_INT_ENW::ECC_INT_EN_0 => false,
            ECC_INT_ENW::ECC_INT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECC_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ECC_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECC_INT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt is generated when error is detected. Only the status bit ECC_INT is set."]
    #[inline]
    pub fn ecc_int_en_0(self) -> &'a mut W {
        self.variant(ECC_INT_ENW::ECC_INT_EN_0)
    }
    #[doc = "Interrupt is generated when error is detected."]
    #[inline]
    pub fn ecc_int_en_1(self) -> &'a mut W {
        self.variant(ECC_INT_ENW::ECC_INT_EN_1)
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
#[doc = "Values that can be written to the field `ZERO_PACK_EN`"]
pub enum ZERO_PACK_ENW {
    #[doc = "Zero packing disabled"]
    ZERO_PACK_EN_0,
    #[doc = "Zero packing enabled"]
    ZERO_PACK_EN_1,
}
impl ZERO_PACK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZERO_PACK_ENW::ZERO_PACK_EN_0 => false,
            ZERO_PACK_ENW::ZERO_PACK_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZERO_PACK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ZERO_PACK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZERO_PACK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Zero packing disabled"]
    #[inline]
    pub fn zero_pack_en_0(self) -> &'a mut W {
        self.variant(ZERO_PACK_ENW::ZERO_PACK_EN_0)
    }
    #[doc = "Zero packing enabled"]
    #[inline]
    pub fn zero_pack_en_1(self) -> &'a mut W {
        self.variant(ZERO_PACK_ENW::ZERO_PACK_EN_1)
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
#[doc = "Values that can be written to the field `TWO_8BIT_SENSOR`"]
pub enum TWO_8BIT_SENSORW {
    #[doc = "Only one sensor is connected."]
    TWO_8BIT_SENSOR_0,
    #[doc = "Two 8-bit sensors are connected or one 16-bit sensor is connected."]
    TWO_8BIT_SENSOR_1,
}
impl TWO_8BIT_SENSORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWO_8BIT_SENSORW::TWO_8BIT_SENSOR_0 => false,
            TWO_8BIT_SENSORW::TWO_8BIT_SENSOR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWO_8BIT_SENSORW<'a> {
    w: &'a mut W,
}
impl<'a> _TWO_8BIT_SENSORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWO_8BIT_SENSORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only one sensor is connected."]
    #[inline]
    pub fn two_8bit_sensor_0(self) -> &'a mut W {
        self.variant(TWO_8BIT_SENSORW::TWO_8BIT_SENSOR_0)
    }
    #[doc = "Two 8-bit sensors are connected or one 16-bit sensor is connected."]
    #[inline]
    pub fn two_8bit_sensor_1(self) -> &'a mut W {
        self.variant(TWO_8BIT_SENSORW::TWO_8BIT_SENSOR_1)
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
#[doc = "Values that can be written to the field `RxFF_LEVEL`"]
pub enum RXFF_LEVELW {
    #[doc = "4 Words"]
    RXFF_LEVEL_0,
    #[doc = "8 Words"]
    RXFF_LEVEL_1,
    #[doc = "16 Words"]
    RXFF_LEVEL_2,
    #[doc = "24 Words"]
    RXFF_LEVEL_3,
    #[doc = "32 Words"]
    RXFF_LEVEL_4,
    #[doc = "48 Words"]
    RXFF_LEVEL_5,
    #[doc = "64 Words"]
    RXFF_LEVEL_6,
    #[doc = "96 Words"]
    RXFF_LEVEL_7,
}
impl RXFF_LEVELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXFF_LEVELW::RXFF_LEVEL_0 => 0,
            RXFF_LEVELW::RXFF_LEVEL_1 => 1,
            RXFF_LEVELW::RXFF_LEVEL_2 => 2,
            RXFF_LEVELW::RXFF_LEVEL_3 => 3,
            RXFF_LEVELW::RXFF_LEVEL_4 => 4,
            RXFF_LEVELW::RXFF_LEVEL_5 => 5,
            RXFF_LEVELW::RXFF_LEVEL_6 => 6,
            RXFF_LEVELW::RXFF_LEVEL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFF_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFF_LEVELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFF_LEVELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 Words"]
    #[inline]
    pub fn rx_ff_level_0(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_0)
    }
    #[doc = "8 Words"]
    #[inline]
    pub fn rx_ff_level_1(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_1)
    }
    #[doc = "16 Words"]
    #[inline]
    pub fn rx_ff_level_2(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_2)
    }
    #[doc = "24 Words"]
    #[inline]
    pub fn rx_ff_level_3(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_3)
    }
    #[doc = "32 Words"]
    #[inline]
    pub fn rx_ff_level_4(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_4)
    }
    #[doc = "48 Words"]
    #[inline]
    pub fn rx_ff_level_5(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_5)
    }
    #[doc = "64 Words"]
    #[inline]
    pub fn rx_ff_level_6(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_6)
    }
    #[doc = "96 Words"]
    #[inline]
    pub fn rx_ff_level_7(self) -> &'a mut W {
        self.variant(RXFF_LEVELW::RXFF_LEVEL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRESP_ERR_EN`"]
pub enum HRESP_ERR_ENW {
    #[doc = "Disable hresponse error interrupt"]
    HRESP_ERR_EN_0,
    #[doc = "Enable hresponse error interrupt"]
    HRESP_ERR_EN_1,
}
impl HRESP_ERR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRESP_ERR_ENW::HRESP_ERR_EN_0 => false,
            HRESP_ERR_ENW::HRESP_ERR_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRESP_ERR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HRESP_ERR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRESP_ERR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable hresponse error interrupt"]
    #[inline]
    pub fn hresp_err_en_0(self) -> &'a mut W {
        self.variant(HRESP_ERR_ENW::HRESP_ERR_EN_0)
    }
    #[doc = "Enable hresponse error interrupt"]
    #[inline]
    pub fn hresp_err_en_1(self) -> &'a mut W {
        self.variant(HRESP_ERR_ENW::HRESP_ERR_EN_1)
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
#[doc = "Values that can be written to the field `STATFF_LEVEL`"]
pub enum STATFF_LEVELW {
    #[doc = "4 Words"]
    STATFF_LEVEL_0,
    #[doc = "8 Words"]
    STATFF_LEVEL_1,
    #[doc = "12 Words"]
    STATFF_LEVEL_2,
    #[doc = "16 Words"]
    STATFF_LEVEL_3,
    #[doc = "24 Words"]
    STATFF_LEVEL_4,
    #[doc = "32 Words"]
    STATFF_LEVEL_5,
    #[doc = "48 Words"]
    STATFF_LEVEL_6,
    #[doc = "64 Words"]
    STATFF_LEVEL_7,
}
impl STATFF_LEVELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STATFF_LEVELW::STATFF_LEVEL_0 => 0,
            STATFF_LEVELW::STATFF_LEVEL_1 => 1,
            STATFF_LEVELW::STATFF_LEVEL_2 => 2,
            STATFF_LEVELW::STATFF_LEVEL_3 => 3,
            STATFF_LEVELW::STATFF_LEVEL_4 => 4,
            STATFF_LEVELW::STATFF_LEVEL_5 => 5,
            STATFF_LEVELW::STATFF_LEVEL_6 => 6,
            STATFF_LEVELW::STATFF_LEVEL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATFF_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _STATFF_LEVELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATFF_LEVELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 Words"]
    #[inline]
    pub fn statff_level_0(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_0)
    }
    #[doc = "8 Words"]
    #[inline]
    pub fn statff_level_1(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_1)
    }
    #[doc = "12 Words"]
    #[inline]
    pub fn statff_level_2(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_2)
    }
    #[doc = "16 Words"]
    #[inline]
    pub fn statff_level_3(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_3)
    }
    #[doc = "24 Words"]
    #[inline]
    pub fn statff_level_4(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_4)
    }
    #[doc = "32 Words"]
    #[inline]
    pub fn statff_level_5(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_5)
    }
    #[doc = "48 Words"]
    #[inline]
    pub fn statff_level_6(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_6)
    }
    #[doc = "64 Words"]
    #[inline]
    pub fn statff_level_7(self) -> &'a mut W {
        self.variant(STATFF_LEVELW::STATFF_LEVEL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_REQ_EN_SFF`"]
pub enum DMA_REQ_EN_SFFW {
    #[doc = "Disable the dma request"]
    DMA_REQ_EN_SFF_0,
    #[doc = "Enable the dma request"]
    DMA_REQ_EN_SFF_1,
}
impl DMA_REQ_EN_SFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_REQ_EN_SFFW::DMA_REQ_EN_SFF_0 => false,
            DMA_REQ_EN_SFFW::DMA_REQ_EN_SFF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_REQ_EN_SFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_REQ_EN_SFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_REQ_EN_SFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the dma request"]
    #[inline]
    pub fn dma_req_en_sff_0(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_SFFW::DMA_REQ_EN_SFF_0)
    }
    #[doc = "Enable the dma request"]
    #[inline]
    pub fn dma_req_en_sff_1(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_SFFW::DMA_REQ_EN_SFF_1)
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
#[doc = "Values that can be written to the field `DMA_REQ_EN_RFF`"]
pub enum DMA_REQ_EN_RFFW {
    #[doc = "Disable the dma request"]
    DMA_REQ_EN_RFF_0,
    #[doc = "Enable the dma request"]
    DMA_REQ_EN_RFF_1,
}
impl DMA_REQ_EN_RFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_REQ_EN_RFFW::DMA_REQ_EN_RFF_0 => false,
            DMA_REQ_EN_RFFW::DMA_REQ_EN_RFF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_REQ_EN_RFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_REQ_EN_RFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_REQ_EN_RFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the dma request"]
    #[inline]
    pub fn dma_req_en_rff_0(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_RFFW::DMA_REQ_EN_RFF_0)
    }
    #[doc = "Enable the dma request"]
    #[inline]
    pub fn dma_req_en_rff_1(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_RFFW::DMA_REQ_EN_RFF_1)
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
#[doc = "Values that can be written to the field `DMA_REFLASH_SFF`"]
pub enum DMA_REFLASH_SFFW {
    #[doc = "No reflashing"]
    DMA_REFLASH_SFF_0,
    #[doc = "Reflash the embedded DMA controller"]
    DMA_REFLASH_SFF_1,
}
impl DMA_REFLASH_SFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_REFLASH_SFFW::DMA_REFLASH_SFF_0 => false,
            DMA_REFLASH_SFFW::DMA_REFLASH_SFF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_REFLASH_SFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_REFLASH_SFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_REFLASH_SFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reflashing"]
    #[inline]
    pub fn dma_reflash_sff_0(self) -> &'a mut W {
        self.variant(DMA_REFLASH_SFFW::DMA_REFLASH_SFF_0)
    }
    #[doc = "Reflash the embedded DMA controller"]
    #[inline]
    pub fn dma_reflash_sff_1(self) -> &'a mut W {
        self.variant(DMA_REFLASH_SFFW::DMA_REFLASH_SFF_1)
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
#[doc = "Values that can be written to the field `DMA_REFLASH_RFF`"]
pub enum DMA_REFLASH_RFFW {
    #[doc = "No reflashing"]
    DMA_REFLASH_RFF_0,
    #[doc = "Reflash the embedded DMA controller"]
    DMA_REFLASH_RFF_1,
}
impl DMA_REFLASH_RFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_REFLASH_RFFW::DMA_REFLASH_RFF_0 => false,
            DMA_REFLASH_RFFW::DMA_REFLASH_RFF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_REFLASH_RFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_REFLASH_RFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_REFLASH_RFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reflashing"]
    #[inline]
    pub fn dma_reflash_rff_0(self) -> &'a mut W {
        self.variant(DMA_REFLASH_RFFW::DMA_REFLASH_RFF_0)
    }
    #[doc = "Reflash the embedded DMA controller"]
    #[inline]
    pub fn dma_reflash_rff_1(self) -> &'a mut W {
        self.variant(DMA_REFLASH_RFFW::DMA_REFLASH_RFF_1)
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
#[doc = "Values that can be written to the field `FRMCNT_RST`"]
pub enum FRMCNT_RSTW {
    #[doc = "Do not reset"]
    FRMCNT_RST_0,
    #[doc = "Reset frame counter immediately"]
    FRMCNT_RST_1,
}
impl FRMCNT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRMCNT_RSTW::FRMCNT_RST_0 => false,
            FRMCNT_RSTW::FRMCNT_RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRMCNT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRMCNT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRMCNT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not reset"]
    #[inline]
    pub fn frmcnt_rst_0(self) -> &'a mut W {
        self.variant(FRMCNT_RSTW::FRMCNT_RST_0)
    }
    #[doc = "Reset frame counter immediately"]
    #[inline]
    pub fn frmcnt_rst_1(self) -> &'a mut W {
        self.variant(FRMCNT_RSTW::FRMCNT_RST_1)
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
#[doc = r" Proxy"]
pub struct _FRMCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRMCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Automatic Error Correction Enable"]
    #[inline]
    pub fn ecc_auto_en(&self) -> ECC_AUTO_ENR {
        ECC_AUTO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Error Detection Interrupt Enable"]
    #[inline]
    pub fn ecc_int_en(&self) -> ECC_INT_ENR {
        ECC_INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Dummy Zero Packing Enable"]
    #[inline]
    pub fn zero_pack_en(&self) -> ZERO_PACK_ENR {
        ZERO_PACK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Two 8-bit Sensor Mode"]
    #[inline]
    pub fn two_8bit_sensor(&self) -> TWO_8BIT_SENSORR {
        TWO_8BIT_SENSORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - RxFIFO Full Level"]
    #[inline]
    pub fn rx_ff_level(&self) -> RXFF_LEVELR {
        RXFF_LEVELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Hresponse Error Enable. This bit enables the hresponse error interrupt."]
    #[inline]
    pub fn hresp_err_en(&self) -> HRESP_ERR_ENR {
        HRESP_ERR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - STATFIFO Full Level"]
    #[inline]
    pub fn statff_level(&self) -> STATFF_LEVELR {
        STATFF_LEVELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - DMA Request Enable for STATFIFO"]
    #[inline]
    pub fn dma_req_en_sff(&self) -> DMA_REQ_EN_SFFR {
        DMA_REQ_EN_SFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - DMA Request Enable for RxFIFO"]
    #[inline]
    pub fn dma_req_en_rff(&self) -> DMA_REQ_EN_RFFR {
        DMA_REQ_EN_RFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Reflash DMA Controller for STATFIFO"]
    #[inline]
    pub fn dma_reflash_sff(&self) -> DMA_REFLASH_SFFR {
        DMA_REFLASH_SFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Reflash DMA Controller for RxFIFO"]
    #[inline]
    pub fn dma_reflash_rff(&self) -> DMA_REFLASH_RFFR {
        DMA_REFLASH_RFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)"]
    #[inline]
    pub fn frmcnt_rst(&self) -> FRMCNT_RSTR {
        FRMCNT_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - Frame Counter"]
    #[inline]
    pub fn frmcnt(&self) -> FRMCNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRMCNTR { bits }
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
    #[doc = "Bit 0 - Automatic Error Correction Enable"]
    #[inline]
    pub fn ecc_auto_en(&mut self) -> _ECC_AUTO_ENW {
        _ECC_AUTO_ENW { w: self }
    }
    #[doc = "Bit 1 - Error Detection Interrupt Enable"]
    #[inline]
    pub fn ecc_int_en(&mut self) -> _ECC_INT_ENW {
        _ECC_INT_ENW { w: self }
    }
    #[doc = "Bit 2 - Dummy Zero Packing Enable"]
    #[inline]
    pub fn zero_pack_en(&mut self) -> _ZERO_PACK_ENW {
        _ZERO_PACK_ENW { w: self }
    }
    #[doc = "Bit 3 - Two 8-bit Sensor Mode"]
    #[inline]
    pub fn two_8bit_sensor(&mut self) -> _TWO_8BIT_SENSORW {
        _TWO_8BIT_SENSORW { w: self }
    }
    #[doc = "Bits 4:6 - RxFIFO Full Level"]
    #[inline]
    pub fn rx_ff_level(&mut self) -> _RXFF_LEVELW {
        _RXFF_LEVELW { w: self }
    }
    #[doc = "Bit 7 - Hresponse Error Enable. This bit enables the hresponse error interrupt."]
    #[inline]
    pub fn hresp_err_en(&mut self) -> _HRESP_ERR_ENW {
        _HRESP_ERR_ENW { w: self }
    }
    #[doc = "Bits 8:10 - STATFIFO Full Level"]
    #[inline]
    pub fn statff_level(&mut self) -> _STATFF_LEVELW {
        _STATFF_LEVELW { w: self }
    }
    #[doc = "Bit 11 - DMA Request Enable for STATFIFO"]
    #[inline]
    pub fn dma_req_en_sff(&mut self) -> _DMA_REQ_EN_SFFW {
        _DMA_REQ_EN_SFFW { w: self }
    }
    #[doc = "Bit 12 - DMA Request Enable for RxFIFO"]
    #[inline]
    pub fn dma_req_en_rff(&mut self) -> _DMA_REQ_EN_RFFW {
        _DMA_REQ_EN_RFFW { w: self }
    }
    #[doc = "Bit 13 - Reflash DMA Controller for STATFIFO"]
    #[inline]
    pub fn dma_reflash_sff(&mut self) -> _DMA_REFLASH_SFFW {
        _DMA_REFLASH_SFFW { w: self }
    }
    #[doc = "Bit 14 - Reflash DMA Controller for RxFIFO"]
    #[inline]
    pub fn dma_reflash_rff(&mut self) -> _DMA_REFLASH_RFFW {
        _DMA_REFLASH_RFFW { w: self }
    }
    #[doc = "Bit 15 - Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)"]
    #[inline]
    pub fn frmcnt_rst(&mut self) -> _FRMCNT_RSTW {
        _FRMCNT_RSTW { w: self }
    }
    #[doc = "Bits 16:31 - Frame Counter"]
    #[inline]
    pub fn frmcnt(&mut self) -> _FRMCNTW {
        _FRMCNTW { w: self }
    }
}
