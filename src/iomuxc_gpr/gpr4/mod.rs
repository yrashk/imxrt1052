#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR4 {
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
#[doc = "Possible values of the field `EDMA_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDMA_STOP_REQR {
    #[doc = "stop request off"]
    EDMA_STOP_REQ_0,
    #[doc = "stop request on"]
    EDMA_STOP_REQ_1,
}
impl EDMA_STOP_REQR {
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
            EDMA_STOP_REQR::EDMA_STOP_REQ_0 => false,
            EDMA_STOP_REQR::EDMA_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDMA_STOP_REQR {
        match value {
            false => EDMA_STOP_REQR::EDMA_STOP_REQ_0,
            true => EDMA_STOP_REQR::EDMA_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_REQ_0`"]
    #[inline]
    pub fn is_edma_stop_req_0(&self) -> bool {
        *self == EDMA_STOP_REQR::EDMA_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_REQ_1`"]
    #[inline]
    pub fn is_edma_stop_req_1(&self) -> bool {
        *self == EDMA_STOP_REQR::EDMA_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `CAN1_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN1_STOP_REQR {
    #[doc = "stop request off"]
    CAN1_STOP_REQ_0,
    #[doc = "stop request on"]
    CAN1_STOP_REQ_1,
}
impl CAN1_STOP_REQR {
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
            CAN1_STOP_REQR::CAN1_STOP_REQ_0 => false,
            CAN1_STOP_REQR::CAN1_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAN1_STOP_REQR {
        match value {
            false => CAN1_STOP_REQR::CAN1_STOP_REQ_0,
            true => CAN1_STOP_REQR::CAN1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_REQ_0`"]
    #[inline]
    pub fn is_can1_stop_req_0(&self) -> bool {
        *self == CAN1_STOP_REQR::CAN1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_REQ_1`"]
    #[inline]
    pub fn is_can1_stop_req_1(&self) -> bool {
        *self == CAN1_STOP_REQR::CAN1_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `CAN2_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN2_STOP_REQR {
    #[doc = "stop request off"]
    CAN2_STOP_REQ_0,
    #[doc = "stop request on"]
    CAN2_STOP_REQ_1,
}
impl CAN2_STOP_REQR {
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
            CAN2_STOP_REQR::CAN2_STOP_REQ_0 => false,
            CAN2_STOP_REQR::CAN2_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAN2_STOP_REQR {
        match value {
            false => CAN2_STOP_REQR::CAN2_STOP_REQ_0,
            true => CAN2_STOP_REQR::CAN2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_REQ_0`"]
    #[inline]
    pub fn is_can2_stop_req_0(&self) -> bool {
        *self == CAN2_STOP_REQR::CAN2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_REQ_1`"]
    #[inline]
    pub fn is_can2_stop_req_1(&self) -> bool {
        *self == CAN2_STOP_REQR::CAN2_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `TRNG_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_STOP_REQR {
    #[doc = "stop request off"]
    TRNG_STOP_REQ_0,
    #[doc = "stop request on"]
    TRNG_STOP_REQ_1,
}
impl TRNG_STOP_REQR {
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
            TRNG_STOP_REQR::TRNG_STOP_REQ_0 => false,
            TRNG_STOP_REQR::TRNG_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRNG_STOP_REQR {
        match value {
            false => TRNG_STOP_REQR::TRNG_STOP_REQ_0,
            true => TRNG_STOP_REQR::TRNG_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_REQ_0`"]
    #[inline]
    pub fn is_trng_stop_req_0(&self) -> bool {
        *self == TRNG_STOP_REQR::TRNG_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_REQ_1`"]
    #[inline]
    pub fn is_trng_stop_req_1(&self) -> bool {
        *self == TRNG_STOP_REQR::TRNG_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `ENET_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_STOP_REQR {
    #[doc = "stop request off"]
    ENET_STOP_REQ_0,
    #[doc = "stop request on"]
    ENET_STOP_REQ_1,
}
impl ENET_STOP_REQR {
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
            ENET_STOP_REQR::ENET_STOP_REQ_0 => false,
            ENET_STOP_REQR::ENET_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENET_STOP_REQR {
        match value {
            false => ENET_STOP_REQR::ENET_STOP_REQ_0,
            true => ENET_STOP_REQR::ENET_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_REQ_0`"]
    #[inline]
    pub fn is_enet_stop_req_0(&self) -> bool {
        *self == ENET_STOP_REQR::ENET_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_REQ_1`"]
    #[inline]
    pub fn is_enet_stop_req_1(&self) -> bool {
        *self == ENET_STOP_REQR::ENET_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `SAI1_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_STOP_REQR {
    #[doc = "stop request off"]
    SAI1_STOP_REQ_0,
    #[doc = "stop request on"]
    SAI1_STOP_REQ_1,
}
impl SAI1_STOP_REQR {
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
            SAI1_STOP_REQR::SAI1_STOP_REQ_0 => false,
            SAI1_STOP_REQR::SAI1_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI1_STOP_REQR {
        match value {
            false => SAI1_STOP_REQR::SAI1_STOP_REQ_0,
            true => SAI1_STOP_REQR::SAI1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_REQ_0`"]
    #[inline]
    pub fn is_sai1_stop_req_0(&self) -> bool {
        *self == SAI1_STOP_REQR::SAI1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_REQ_1`"]
    #[inline]
    pub fn is_sai1_stop_req_1(&self) -> bool {
        *self == SAI1_STOP_REQR::SAI1_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `SAI2_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_STOP_REQR {
    #[doc = "stop request off"]
    SAI2_STOP_REQ_0,
    #[doc = "stop request on"]
    SAI2_STOP_REQ_1,
}
impl SAI2_STOP_REQR {
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
            SAI2_STOP_REQR::SAI2_STOP_REQ_0 => false,
            SAI2_STOP_REQR::SAI2_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI2_STOP_REQR {
        match value {
            false => SAI2_STOP_REQR::SAI2_STOP_REQ_0,
            true => SAI2_STOP_REQR::SAI2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_REQ_0`"]
    #[inline]
    pub fn is_sai2_stop_req_0(&self) -> bool {
        *self == SAI2_STOP_REQR::SAI2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_REQ_1`"]
    #[inline]
    pub fn is_sai2_stop_req_1(&self) -> bool {
        *self == SAI2_STOP_REQR::SAI2_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `SAI3_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_STOP_REQR {
    #[doc = "stop request off"]
    SAI3_STOP_REQ_0,
    #[doc = "stop request on"]
    SAI3_STOP_REQ_1,
}
impl SAI3_STOP_REQR {
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
            SAI3_STOP_REQR::SAI3_STOP_REQ_0 => false,
            SAI3_STOP_REQR::SAI3_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI3_STOP_REQR {
        match value {
            false => SAI3_STOP_REQR::SAI3_STOP_REQ_0,
            true => SAI3_STOP_REQR::SAI3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_REQ_0`"]
    #[inline]
    pub fn is_sai3_stop_req_0(&self) -> bool {
        *self == SAI3_STOP_REQR::SAI3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_REQ_1`"]
    #[inline]
    pub fn is_sai3_stop_req_1(&self) -> bool {
        *self == SAI3_STOP_REQR::SAI3_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `SEMC_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_STOP_REQR {
    #[doc = "stop request off"]
    SEMC_STOP_REQ_0,
    #[doc = "stop request on"]
    SEMC_STOP_REQ_1,
}
impl SEMC_STOP_REQR {
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
            SEMC_STOP_REQR::SEMC_STOP_REQ_0 => false,
            SEMC_STOP_REQR::SEMC_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEMC_STOP_REQR {
        match value {
            false => SEMC_STOP_REQR::SEMC_STOP_REQ_0,
            true => SEMC_STOP_REQR::SEMC_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_REQ_0`"]
    #[inline]
    pub fn is_semc_stop_req_0(&self) -> bool {
        *self == SEMC_STOP_REQR::SEMC_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_REQ_1`"]
    #[inline]
    pub fn is_semc_stop_req_1(&self) -> bool {
        *self == SEMC_STOP_REQR::SEMC_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `PIT_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_STOP_REQR {
    #[doc = "stop request off"]
    PIT_STOP_REQ_0,
    #[doc = "stop request on"]
    PIT_STOP_REQ_1,
}
impl PIT_STOP_REQR {
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
            PIT_STOP_REQR::PIT_STOP_REQ_0 => false,
            PIT_STOP_REQR::PIT_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIT_STOP_REQR {
        match value {
            false => PIT_STOP_REQR::PIT_STOP_REQ_0,
            true => PIT_STOP_REQR::PIT_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_REQ_0`"]
    #[inline]
    pub fn is_pit_stop_req_0(&self) -> bool {
        *self == PIT_STOP_REQR::PIT_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_REQ_1`"]
    #[inline]
    pub fn is_pit_stop_req_1(&self) -> bool {
        *self == PIT_STOP_REQR::PIT_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `FLEXSPI_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_STOP_REQR {
    #[doc = "stop request off"]
    FLEXSPI_STOP_REQ_0,
    #[doc = "stop request on"]
    FLEXSPI_STOP_REQ_1,
}
impl FLEXSPI_STOP_REQR {
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
            FLEXSPI_STOP_REQR::FLEXSPI_STOP_REQ_0 => false,
            FLEXSPI_STOP_REQR::FLEXSPI_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXSPI_STOP_REQR {
        match value {
            false => FLEXSPI_STOP_REQR::FLEXSPI_STOP_REQ_0,
            true => FLEXSPI_STOP_REQR::FLEXSPI_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_REQ_0`"]
    #[inline]
    pub fn is_flexspi_stop_req_0(&self) -> bool {
        *self == FLEXSPI_STOP_REQR::FLEXSPI_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_REQ_1`"]
    #[inline]
    pub fn is_flexspi_stop_req_1(&self) -> bool {
        *self == FLEXSPI_STOP_REQR::FLEXSPI_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `FLEXIO1_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_STOP_REQR {
    #[doc = "stop request off"]
    FLEXIO1_STOP_REQ_0,
    #[doc = "stop request on"]
    FLEXIO1_STOP_REQ_1,
}
impl FLEXIO1_STOP_REQR {
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
            FLEXIO1_STOP_REQR::FLEXIO1_STOP_REQ_0 => false,
            FLEXIO1_STOP_REQR::FLEXIO1_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO1_STOP_REQR {
        match value {
            false => FLEXIO1_STOP_REQR::FLEXIO1_STOP_REQ_0,
            true => FLEXIO1_STOP_REQR::FLEXIO1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_REQ_0`"]
    #[inline]
    pub fn is_flexio1_stop_req_0(&self) -> bool {
        *self == FLEXIO1_STOP_REQR::FLEXIO1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_REQ_1`"]
    #[inline]
    pub fn is_flexio1_stop_req_1(&self) -> bool {
        *self == FLEXIO1_STOP_REQR::FLEXIO1_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `FLEXIO2_STOP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_STOP_REQR {
    #[doc = "stop request off"]
    FLEXIO2_STOP_REQ_0,
    #[doc = "stop request on"]
    FLEXIO2_STOP_REQ_1,
}
impl FLEXIO2_STOP_REQR {
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
            FLEXIO2_STOP_REQR::FLEXIO2_STOP_REQ_0 => false,
            FLEXIO2_STOP_REQR::FLEXIO2_STOP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO2_STOP_REQR {
        match value {
            false => FLEXIO2_STOP_REQR::FLEXIO2_STOP_REQ_0,
            true => FLEXIO2_STOP_REQR::FLEXIO2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_REQ_0`"]
    #[inline]
    pub fn is_flexio2_stop_req_0(&self) -> bool {
        *self == FLEXIO2_STOP_REQR::FLEXIO2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_REQ_1`"]
    #[inline]
    pub fn is_flexio2_stop_req_1(&self) -> bool {
        *self == FLEXIO2_STOP_REQR::FLEXIO2_STOP_REQ_1
    }
}
#[doc = "Possible values of the field `EDMA_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDMA_STOP_ACKR {
    #[doc = "EDMA stop acknowledge is not asserted"]
    EDMA_STOP_ACK_0,
    #[doc = "EDMA stop acknowledge is asserted (EDMA is in STOP mode)."]
    EDMA_STOP_ACK_1,
}
impl EDMA_STOP_ACKR {
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
            EDMA_STOP_ACKR::EDMA_STOP_ACK_0 => false,
            EDMA_STOP_ACKR::EDMA_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDMA_STOP_ACKR {
        match value {
            false => EDMA_STOP_ACKR::EDMA_STOP_ACK_0,
            true => EDMA_STOP_ACKR::EDMA_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_ACK_0`"]
    #[inline]
    pub fn is_edma_stop_ack_0(&self) -> bool {
        *self == EDMA_STOP_ACKR::EDMA_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_ACK_1`"]
    #[inline]
    pub fn is_edma_stop_ack_1(&self) -> bool {
        *self == EDMA_STOP_ACKR::EDMA_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `CAN1_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN1_STOP_ACKR {
    #[doc = "CAN1 stop acknowledge is not asserted"]
    CAN1_STOP_ACK_0,
    #[doc = "CAN1 stop acknowledge is asserted"]
    CAN1_STOP_ACK_1,
}
impl CAN1_STOP_ACKR {
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
            CAN1_STOP_ACKR::CAN1_STOP_ACK_0 => false,
            CAN1_STOP_ACKR::CAN1_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAN1_STOP_ACKR {
        match value {
            false => CAN1_STOP_ACKR::CAN1_STOP_ACK_0,
            true => CAN1_STOP_ACKR::CAN1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_ACK_0`"]
    #[inline]
    pub fn is_can1_stop_ack_0(&self) -> bool {
        *self == CAN1_STOP_ACKR::CAN1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_ACK_1`"]
    #[inline]
    pub fn is_can1_stop_ack_1(&self) -> bool {
        *self == CAN1_STOP_ACKR::CAN1_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `CAN2_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN2_STOP_ACKR {
    #[doc = "CAN2 stop acknowledge is not asserted"]
    CAN2_STOP_ACK_0,
    #[doc = "CAN2 stop acknowledge is asserted"]
    CAN2_STOP_ACK_1,
}
impl CAN2_STOP_ACKR {
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
            CAN2_STOP_ACKR::CAN2_STOP_ACK_0 => false,
            CAN2_STOP_ACKR::CAN2_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAN2_STOP_ACKR {
        match value {
            false => CAN2_STOP_ACKR::CAN2_STOP_ACK_0,
            true => CAN2_STOP_ACKR::CAN2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_ACK_0`"]
    #[inline]
    pub fn is_can2_stop_ack_0(&self) -> bool {
        *self == CAN2_STOP_ACKR::CAN2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_ACK_1`"]
    #[inline]
    pub fn is_can2_stop_ack_1(&self) -> bool {
        *self == CAN2_STOP_ACKR::CAN2_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `TRNG_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_STOP_ACKR {
    #[doc = "ENET1 stop acknowledge is not asserted"]
    TRNG_STOP_ACK_0,
    #[doc = "ENET1 stop acknowledge is asserted"]
    TRNG_STOP_ACK_1,
}
impl TRNG_STOP_ACKR {
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
            TRNG_STOP_ACKR::TRNG_STOP_ACK_0 => false,
            TRNG_STOP_ACKR::TRNG_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRNG_STOP_ACKR {
        match value {
            false => TRNG_STOP_ACKR::TRNG_STOP_ACK_0,
            true => TRNG_STOP_ACKR::TRNG_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_ACK_0`"]
    #[inline]
    pub fn is_trng_stop_ack_0(&self) -> bool {
        *self == TRNG_STOP_ACKR::TRNG_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_ACK_1`"]
    #[inline]
    pub fn is_trng_stop_ack_1(&self) -> bool {
        *self == TRNG_STOP_ACKR::TRNG_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `ENET_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_STOP_ACKR {
    #[doc = "ENET2 stop acknowledge is not asserted"]
    ENET_STOP_ACK_0,
    #[doc = "ENET2 stop acknowledge is asserted"]
    ENET_STOP_ACK_1,
}
impl ENET_STOP_ACKR {
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
            ENET_STOP_ACKR::ENET_STOP_ACK_0 => false,
            ENET_STOP_ACKR::ENET_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENET_STOP_ACKR {
        match value {
            false => ENET_STOP_ACKR::ENET_STOP_ACK_0,
            true => ENET_STOP_ACKR::ENET_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_ACK_0`"]
    #[inline]
    pub fn is_enet_stop_ack_0(&self) -> bool {
        *self == ENET_STOP_ACKR::ENET_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_ACK_1`"]
    #[inline]
    pub fn is_enet_stop_ack_1(&self) -> bool {
        *self == ENET_STOP_ACKR::ENET_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `SAI1_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_STOP_ACKR {
    #[doc = "SAI1 stop acknowledge is not asserted"]
    SAI1_STOP_ACK_0,
    #[doc = "SAI1 stop acknowledge is asserted"]
    SAI1_STOP_ACK_1,
}
impl SAI1_STOP_ACKR {
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
            SAI1_STOP_ACKR::SAI1_STOP_ACK_0 => false,
            SAI1_STOP_ACKR::SAI1_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI1_STOP_ACKR {
        match value {
            false => SAI1_STOP_ACKR::SAI1_STOP_ACK_0,
            true => SAI1_STOP_ACKR::SAI1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_ACK_0`"]
    #[inline]
    pub fn is_sai1_stop_ack_0(&self) -> bool {
        *self == SAI1_STOP_ACKR::SAI1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_ACK_1`"]
    #[inline]
    pub fn is_sai1_stop_ack_1(&self) -> bool {
        *self == SAI1_STOP_ACKR::SAI1_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `SAI2_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_STOP_ACKR {
    #[doc = "SAI2 stop acknowledge is not asserted"]
    SAI2_STOP_ACK_0,
    #[doc = "SAI2 stop acknowledge is asserted"]
    SAI2_STOP_ACK_1,
}
impl SAI2_STOP_ACKR {
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
            SAI2_STOP_ACKR::SAI2_STOP_ACK_0 => false,
            SAI2_STOP_ACKR::SAI2_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI2_STOP_ACKR {
        match value {
            false => SAI2_STOP_ACKR::SAI2_STOP_ACK_0,
            true => SAI2_STOP_ACKR::SAI2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_ACK_0`"]
    #[inline]
    pub fn is_sai2_stop_ack_0(&self) -> bool {
        *self == SAI2_STOP_ACKR::SAI2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_ACK_1`"]
    #[inline]
    pub fn is_sai2_stop_ack_1(&self) -> bool {
        *self == SAI2_STOP_ACKR::SAI2_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `SAI3_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_STOP_ACKR {
    #[doc = "SAI3 stop acknowledge is not asserted"]
    SAI3_STOP_ACK_0,
    #[doc = "SAI3 stop acknowledge is asserted"]
    SAI3_STOP_ACK_1,
}
impl SAI3_STOP_ACKR {
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
            SAI3_STOP_ACKR::SAI3_STOP_ACK_0 => false,
            SAI3_STOP_ACKR::SAI3_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAI3_STOP_ACKR {
        match value {
            false => SAI3_STOP_ACKR::SAI3_STOP_ACK_0,
            true => SAI3_STOP_ACKR::SAI3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_ACK_0`"]
    #[inline]
    pub fn is_sai3_stop_ack_0(&self) -> bool {
        *self == SAI3_STOP_ACKR::SAI3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_ACK_1`"]
    #[inline]
    pub fn is_sai3_stop_ack_1(&self) -> bool {
        *self == SAI3_STOP_ACKR::SAI3_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `SEMC_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_STOP_ACKR {
    #[doc = "SEMC stop acknowledge is not asserted"]
    SEMC_STOP_ACK_0,
    #[doc = "SEMC stop acknowledge is asserted"]
    SEMC_STOP_ACK_1,
}
impl SEMC_STOP_ACKR {
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
            SEMC_STOP_ACKR::SEMC_STOP_ACK_0 => false,
            SEMC_STOP_ACKR::SEMC_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEMC_STOP_ACKR {
        match value {
            false => SEMC_STOP_ACKR::SEMC_STOP_ACK_0,
            true => SEMC_STOP_ACKR::SEMC_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_ACK_0`"]
    #[inline]
    pub fn is_semc_stop_ack_0(&self) -> bool {
        *self == SEMC_STOP_ACKR::SEMC_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_ACK_1`"]
    #[inline]
    pub fn is_semc_stop_ack_1(&self) -> bool {
        *self == SEMC_STOP_ACKR::SEMC_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `PIT_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_STOP_ACKR {
    #[doc = "PIT stop acknowledge is not asserted"]
    PIT_STOP_ACK_0,
    #[doc = "PIT stop acknowledge is asserted"]
    PIT_STOP_ACK_1,
}
impl PIT_STOP_ACKR {
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
            PIT_STOP_ACKR::PIT_STOP_ACK_0 => false,
            PIT_STOP_ACKR::PIT_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIT_STOP_ACKR {
        match value {
            false => PIT_STOP_ACKR::PIT_STOP_ACK_0,
            true => PIT_STOP_ACKR::PIT_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_ACK_0`"]
    #[inline]
    pub fn is_pit_stop_ack_0(&self) -> bool {
        *self == PIT_STOP_ACKR::PIT_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_ACK_1`"]
    #[inline]
    pub fn is_pit_stop_ack_1(&self) -> bool {
        *self == PIT_STOP_ACKR::PIT_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `FLEXSPI_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_STOP_ACKR {
    #[doc = "FLEXSPI stop acknowledge is not asserted"]
    FLEXSPI_STOP_ACK_0,
    #[doc = "FLEXSPI stop acknowledge is asserted"]
    FLEXSPI_STOP_ACK_1,
}
impl FLEXSPI_STOP_ACKR {
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
            FLEXSPI_STOP_ACKR::FLEXSPI_STOP_ACK_0 => false,
            FLEXSPI_STOP_ACKR::FLEXSPI_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXSPI_STOP_ACKR {
        match value {
            false => FLEXSPI_STOP_ACKR::FLEXSPI_STOP_ACK_0,
            true => FLEXSPI_STOP_ACKR::FLEXSPI_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_ACK_0`"]
    #[inline]
    pub fn is_flexspi_stop_ack_0(&self) -> bool {
        *self == FLEXSPI_STOP_ACKR::FLEXSPI_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_ACK_1`"]
    #[inline]
    pub fn is_flexspi_stop_ack_1(&self) -> bool {
        *self == FLEXSPI_STOP_ACKR::FLEXSPI_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `FLEXIO1_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_STOP_ACKR {
    #[doc = "FLEXIO1 stop acknowledge is not asserted"]
    FLEXIO1_STOP_ACK_0,
    #[doc = "FLEXIO1 stop acknowledge is asserted"]
    FLEXIO1_STOP_ACK_1,
}
impl FLEXIO1_STOP_ACKR {
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
            FLEXIO1_STOP_ACKR::FLEXIO1_STOP_ACK_0 => false,
            FLEXIO1_STOP_ACKR::FLEXIO1_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO1_STOP_ACKR {
        match value {
            false => FLEXIO1_STOP_ACKR::FLEXIO1_STOP_ACK_0,
            true => FLEXIO1_STOP_ACKR::FLEXIO1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_ACK_0`"]
    #[inline]
    pub fn is_flexio1_stop_ack_0(&self) -> bool {
        *self == FLEXIO1_STOP_ACKR::FLEXIO1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_ACK_1`"]
    #[inline]
    pub fn is_flexio1_stop_ack_1(&self) -> bool {
        *self == FLEXIO1_STOP_ACKR::FLEXIO1_STOP_ACK_1
    }
}
#[doc = "Possible values of the field `FLEXIO2_STOP_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_STOP_ACKR {
    #[doc = "FLEXIO2 stop acknowledge is not asserted"]
    FLEXIO2_STOP_ACK_0,
    #[doc = "FLEXIO2 stop acknowledge is asserted (FLEXIO2 is in STOP mode)"]
    FLEXIO2_STOP_ACK_1,
}
impl FLEXIO2_STOP_ACKR {
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
            FLEXIO2_STOP_ACKR::FLEXIO2_STOP_ACK_0 => false,
            FLEXIO2_STOP_ACKR::FLEXIO2_STOP_ACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO2_STOP_ACKR {
        match value {
            false => FLEXIO2_STOP_ACKR::FLEXIO2_STOP_ACK_0,
            true => FLEXIO2_STOP_ACKR::FLEXIO2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_ACK_0`"]
    #[inline]
    pub fn is_flexio2_stop_ack_0(&self) -> bool {
        *self == FLEXIO2_STOP_ACKR::FLEXIO2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_ACK_1`"]
    #[inline]
    pub fn is_flexio2_stop_ack_1(&self) -> bool {
        *self == FLEXIO2_STOP_ACKR::FLEXIO2_STOP_ACK_1
    }
}
#[doc = "Values that can be written to the field `EDMA_STOP_REQ`"]
pub enum EDMA_STOP_REQW {
    #[doc = "stop request off"]
    EDMA_STOP_REQ_0,
    #[doc = "stop request on"]
    EDMA_STOP_REQ_1,
}
impl EDMA_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDMA_STOP_REQW::EDMA_STOP_REQ_0 => false,
            EDMA_STOP_REQW::EDMA_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDMA_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _EDMA_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDMA_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn edma_stop_req_0(self) -> &'a mut W {
        self.variant(EDMA_STOP_REQW::EDMA_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn edma_stop_req_1(self) -> &'a mut W {
        self.variant(EDMA_STOP_REQW::EDMA_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `CAN1_STOP_REQ`"]
pub enum CAN1_STOP_REQW {
    #[doc = "stop request off"]
    CAN1_STOP_REQ_0,
    #[doc = "stop request on"]
    CAN1_STOP_REQ_1,
}
impl CAN1_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAN1_STOP_REQW::CAN1_STOP_REQ_0 => false,
            CAN1_STOP_REQW::CAN1_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAN1_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN1_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN1_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn can1_stop_req_0(self) -> &'a mut W {
        self.variant(CAN1_STOP_REQW::CAN1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn can1_stop_req_1(self) -> &'a mut W {
        self.variant(CAN1_STOP_REQW::CAN1_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `CAN2_STOP_REQ`"]
pub enum CAN2_STOP_REQW {
    #[doc = "stop request off"]
    CAN2_STOP_REQ_0,
    #[doc = "stop request on"]
    CAN2_STOP_REQ_1,
}
impl CAN2_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAN2_STOP_REQW::CAN2_STOP_REQ_0 => false,
            CAN2_STOP_REQW::CAN2_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAN2_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN2_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN2_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn can2_stop_req_0(self) -> &'a mut W {
        self.variant(CAN2_STOP_REQW::CAN2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn can2_stop_req_1(self) -> &'a mut W {
        self.variant(CAN2_STOP_REQW::CAN2_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `TRNG_STOP_REQ`"]
pub enum TRNG_STOP_REQW {
    #[doc = "stop request off"]
    TRNG_STOP_REQ_0,
    #[doc = "stop request on"]
    TRNG_STOP_REQ_1,
}
impl TRNG_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRNG_STOP_REQW::TRNG_STOP_REQ_0 => false,
            TRNG_STOP_REQW::TRNG_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRNG_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _TRNG_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRNG_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn trng_stop_req_0(self) -> &'a mut W {
        self.variant(TRNG_STOP_REQW::TRNG_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn trng_stop_req_1(self) -> &'a mut W {
        self.variant(TRNG_STOP_REQW::TRNG_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `ENET_STOP_REQ`"]
pub enum ENET_STOP_REQW {
    #[doc = "stop request off"]
    ENET_STOP_REQ_0,
    #[doc = "stop request on"]
    ENET_STOP_REQ_1,
}
impl ENET_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENET_STOP_REQW::ENET_STOP_REQ_0 => false,
            ENET_STOP_REQW::ENET_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENET_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _ENET_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENET_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn enet_stop_req_0(self) -> &'a mut W {
        self.variant(ENET_STOP_REQW::ENET_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn enet_stop_req_1(self) -> &'a mut W {
        self.variant(ENET_STOP_REQW::ENET_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `SAI1_STOP_REQ`"]
pub enum SAI1_STOP_REQW {
    #[doc = "stop request off"]
    SAI1_STOP_REQ_0,
    #[doc = "stop request on"]
    SAI1_STOP_REQ_1,
}
impl SAI1_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAI1_STOP_REQW::SAI1_STOP_REQ_0 => false,
            SAI1_STOP_REQW::SAI1_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI1_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn sai1_stop_req_0(self) -> &'a mut W {
        self.variant(SAI1_STOP_REQW::SAI1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn sai1_stop_req_1(self) -> &'a mut W {
        self.variant(SAI1_STOP_REQW::SAI1_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `SAI2_STOP_REQ`"]
pub enum SAI2_STOP_REQW {
    #[doc = "stop request off"]
    SAI2_STOP_REQ_0,
    #[doc = "stop request on"]
    SAI2_STOP_REQ_1,
}
impl SAI2_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAI2_STOP_REQW::SAI2_STOP_REQ_0 => false,
            SAI2_STOP_REQW::SAI2_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI2_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn sai2_stop_req_0(self) -> &'a mut W {
        self.variant(SAI2_STOP_REQW::SAI2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn sai2_stop_req_1(self) -> &'a mut W {
        self.variant(SAI2_STOP_REQW::SAI2_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `SAI3_STOP_REQ`"]
pub enum SAI3_STOP_REQW {
    #[doc = "stop request off"]
    SAI3_STOP_REQ_0,
    #[doc = "stop request on"]
    SAI3_STOP_REQ_1,
}
impl SAI3_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAI3_STOP_REQW::SAI3_STOP_REQ_0 => false,
            SAI3_STOP_REQW::SAI3_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI3_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI3_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI3_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn sai3_stop_req_0(self) -> &'a mut W {
        self.variant(SAI3_STOP_REQW::SAI3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn sai3_stop_req_1(self) -> &'a mut W {
        self.variant(SAI3_STOP_REQW::SAI3_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `SEMC_STOP_REQ`"]
pub enum SEMC_STOP_REQW {
    #[doc = "stop request off"]
    SEMC_STOP_REQ_0,
    #[doc = "stop request on"]
    SEMC_STOP_REQ_1,
}
impl SEMC_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEMC_STOP_REQW::SEMC_STOP_REQ_0 => false,
            SEMC_STOP_REQW::SEMC_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEMC_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SEMC_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEMC_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn semc_stop_req_0(self) -> &'a mut W {
        self.variant(SEMC_STOP_REQW::SEMC_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn semc_stop_req_1(self) -> &'a mut W {
        self.variant(SEMC_STOP_REQW::SEMC_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `PIT_STOP_REQ`"]
pub enum PIT_STOP_REQW {
    #[doc = "stop request off"]
    PIT_STOP_REQ_0,
    #[doc = "stop request on"]
    PIT_STOP_REQ_1,
}
impl PIT_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIT_STOP_REQW::PIT_STOP_REQ_0 => false,
            PIT_STOP_REQW::PIT_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIT_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _PIT_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIT_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn pit_stop_req_0(self) -> &'a mut W {
        self.variant(PIT_STOP_REQW::PIT_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn pit_stop_req_1(self) -> &'a mut W {
        self.variant(PIT_STOP_REQW::PIT_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `FLEXSPI_STOP_REQ`"]
pub enum FLEXSPI_STOP_REQW {
    #[doc = "stop request off"]
    FLEXSPI_STOP_REQ_0,
    #[doc = "stop request on"]
    FLEXSPI_STOP_REQ_1,
}
impl FLEXSPI_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXSPI_STOP_REQW::FLEXSPI_STOP_REQ_0 => false,
            FLEXSPI_STOP_REQW::FLEXSPI_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXSPI_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXSPI_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXSPI_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn flexspi_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXSPI_STOP_REQW::FLEXSPI_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn flexspi_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXSPI_STOP_REQW::FLEXSPI_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `FLEXIO1_STOP_REQ`"]
pub enum FLEXIO1_STOP_REQW {
    #[doc = "stop request off"]
    FLEXIO1_STOP_REQ_0,
    #[doc = "stop request on"]
    FLEXIO1_STOP_REQ_1,
}
impl FLEXIO1_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXIO1_STOP_REQW::FLEXIO1_STOP_REQ_0 => false,
            FLEXIO1_STOP_REQW::FLEXIO1_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO1_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO1_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO1_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn flexio1_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXIO1_STOP_REQW::FLEXIO1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn flexio1_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXIO1_STOP_REQW::FLEXIO1_STOP_REQ_1)
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
#[doc = "Values that can be written to the field `FLEXIO2_STOP_REQ`"]
pub enum FLEXIO2_STOP_REQW {
    #[doc = "stop request off"]
    FLEXIO2_STOP_REQ_0,
    #[doc = "stop request on"]
    FLEXIO2_STOP_REQ_1,
}
impl FLEXIO2_STOP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXIO2_STOP_REQW::FLEXIO2_STOP_REQ_0 => false,
            FLEXIO2_STOP_REQW::FLEXIO2_STOP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO2_STOP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO2_STOP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO2_STOP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stop request off"]
    #[inline]
    pub fn flexio2_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXIO2_STOP_REQW::FLEXIO2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline]
    pub fn flexio2_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXIO2_STOP_REQW::FLEXIO2_STOP_REQ_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - EDMA stop request."]
    #[inline]
    pub fn edma_stop_req(&self) -> EDMA_STOP_REQR {
        EDMA_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CAN1 stop request."]
    #[inline]
    pub fn can1_stop_req(&self) -> CAN1_STOP_REQR {
        CAN1_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CAN2 stop request."]
    #[inline]
    pub fn can2_stop_req(&self) -> CAN2_STOP_REQR {
        CAN2_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - TRNG stop request."]
    #[inline]
    pub fn trng_stop_req(&self) -> TRNG_STOP_REQR {
        TRNG_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ENET stop request."]
    #[inline]
    pub fn enet_stop_req(&self) -> ENET_STOP_REQR {
        ENET_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - SAI1 stop request."]
    #[inline]
    pub fn sai1_stop_req(&self) -> SAI1_STOP_REQR {
        SAI1_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - SAI2 stop request."]
    #[inline]
    pub fn sai2_stop_req(&self) -> SAI2_STOP_REQR {
        SAI2_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SAI3 stop request."]
    #[inline]
    pub fn sai3_stop_req(&self) -> SAI3_STOP_REQR {
        SAI3_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SEMC stop request."]
    #[inline]
    pub fn semc_stop_req(&self) -> SEMC_STOP_REQR {
        SEMC_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PIT stop request."]
    #[inline]
    pub fn pit_stop_req(&self) -> PIT_STOP_REQR {
        PIT_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - FlexSPI stop request."]
    #[inline]
    pub fn flexspi_stop_req(&self) -> FLEXSPI_STOP_REQR {
        FLEXSPI_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FlexIO1 stop request."]
    #[inline]
    pub fn flexio1_stop_req(&self) -> FLEXIO1_STOP_REQR {
        FLEXIO1_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - FlexIO2 stop request."]
    #[inline]
    pub fn flexio2_stop_req(&self) -> FLEXIO2_STOP_REQR {
        FLEXIO2_STOP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - EDMA stop acknowledge. This is a status (read-only) bit"]
    #[inline]
    pub fn edma_stop_ack(&self) -> EDMA_STOP_ACKR {
        EDMA_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - CAN1 stop acknowledge."]
    #[inline]
    pub fn can1_stop_ack(&self) -> CAN1_STOP_ACKR {
        CAN1_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - CAN2 stop acknowledge."]
    #[inline]
    pub fn can2_stop_ack(&self) -> CAN2_STOP_ACKR {
        CAN2_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - TRNG stop acknowledge"]
    #[inline]
    pub fn trng_stop_ack(&self) -> TRNG_STOP_ACKR {
        TRNG_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - ENET stop acknowledge."]
    #[inline]
    pub fn enet_stop_ack(&self) -> ENET_STOP_ACKR {
        ENET_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SAI1 stop acknowledge"]
    #[inline]
    pub fn sai1_stop_ack(&self) -> SAI1_STOP_ACKR {
        SAI1_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SAI2 stop acknowledge"]
    #[inline]
    pub fn sai2_stop_ack(&self) -> SAI2_STOP_ACKR {
        SAI2_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - SAI3 stop acknowledge"]
    #[inline]
    pub fn sai3_stop_ack(&self) -> SAI3_STOP_ACKR {
        SAI3_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - SEMC stop acknowledge"]
    #[inline]
    pub fn semc_stop_ack(&self) -> SEMC_STOP_ACKR {
        SEMC_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - PIT stop acknowledge"]
    #[inline]
    pub fn pit_stop_ack(&self) -> PIT_STOP_ACKR {
        PIT_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - FLEXSPI stop acknowledge"]
    #[inline]
    pub fn flexspi_stop_ack(&self) -> FLEXSPI_STOP_ACKR {
        FLEXSPI_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - FLEXIO1 stop acknowledge"]
    #[inline]
    pub fn flexio1_stop_ack(&self) -> FLEXIO1_STOP_ACKR {
        FLEXIO1_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - FLEXIO2 stop acknowledge"]
    #[inline]
    pub fn flexio2_stop_ack(&self) -> FLEXIO2_STOP_ACKR {
        FLEXIO2_STOP_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - EDMA stop request."]
    #[inline]
    pub fn edma_stop_req(&mut self) -> _EDMA_STOP_REQW {
        _EDMA_STOP_REQW { w: self }
    }
    #[doc = "Bit 1 - CAN1 stop request."]
    #[inline]
    pub fn can1_stop_req(&mut self) -> _CAN1_STOP_REQW {
        _CAN1_STOP_REQW { w: self }
    }
    #[doc = "Bit 2 - CAN2 stop request."]
    #[inline]
    pub fn can2_stop_req(&mut self) -> _CAN2_STOP_REQW {
        _CAN2_STOP_REQW { w: self }
    }
    #[doc = "Bit 3 - TRNG stop request."]
    #[inline]
    pub fn trng_stop_req(&mut self) -> _TRNG_STOP_REQW {
        _TRNG_STOP_REQW { w: self }
    }
    #[doc = "Bit 4 - ENET stop request."]
    #[inline]
    pub fn enet_stop_req(&mut self) -> _ENET_STOP_REQW {
        _ENET_STOP_REQW { w: self }
    }
    #[doc = "Bit 5 - SAI1 stop request."]
    #[inline]
    pub fn sai1_stop_req(&mut self) -> _SAI1_STOP_REQW {
        _SAI1_STOP_REQW { w: self }
    }
    #[doc = "Bit 6 - SAI2 stop request."]
    #[inline]
    pub fn sai2_stop_req(&mut self) -> _SAI2_STOP_REQW {
        _SAI2_STOP_REQW { w: self }
    }
    #[doc = "Bit 7 - SAI3 stop request."]
    #[inline]
    pub fn sai3_stop_req(&mut self) -> _SAI3_STOP_REQW {
        _SAI3_STOP_REQW { w: self }
    }
    #[doc = "Bit 9 - SEMC stop request."]
    #[inline]
    pub fn semc_stop_req(&mut self) -> _SEMC_STOP_REQW {
        _SEMC_STOP_REQW { w: self }
    }
    #[doc = "Bit 10 - PIT stop request."]
    #[inline]
    pub fn pit_stop_req(&mut self) -> _PIT_STOP_REQW {
        _PIT_STOP_REQW { w: self }
    }
    #[doc = "Bit 11 - FlexSPI stop request."]
    #[inline]
    pub fn flexspi_stop_req(&mut self) -> _FLEXSPI_STOP_REQW {
        _FLEXSPI_STOP_REQW { w: self }
    }
    #[doc = "Bit 12 - FlexIO1 stop request."]
    #[inline]
    pub fn flexio1_stop_req(&mut self) -> _FLEXIO1_STOP_REQW {
        _FLEXIO1_STOP_REQW { w: self }
    }
    #[doc = "Bit 13 - FlexIO2 stop request."]
    #[inline]
    pub fn flexio2_stop_req(&mut self) -> _FLEXIO2_STOP_REQW {
        _FLEXIO2_STOP_REQW { w: self }
    }
}
