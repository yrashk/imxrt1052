#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_STATUS {
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
#[doc = "Possible values of the field `CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCR {
    #[doc = "Command not complete"]
    CC_0,
    #[doc = "Command complete"]
    CC_1,
}
impl CCR {
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
            CCR::CC_0 => false,
            CCR::CC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCR {
        match value {
            false => CCR::CC_0,
            true => CCR::CC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CC_0`"]
    #[inline]
    pub fn is_cc_0(&self) -> bool {
        *self == CCR::CC_0
    }
    #[doc = "Checks if the value of the field is `CC_1`"]
    #[inline]
    pub fn is_cc_1(&self) -> bool {
        *self == CCR::CC_1
    }
}
#[doc = "Possible values of the field `TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR {
    #[doc = "Transfer not complete"]
    TC_0,
    #[doc = "Transfer complete"]
    TC_1,
}
impl TCR {
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
            TCR::TC_0 => false,
            TCR::TC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCR {
        match value {
            false => TCR::TC_0,
            true => TCR::TC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TC_0`"]
    #[inline]
    pub fn is_tc_0(&self) -> bool {
        *self == TCR::TC_0
    }
    #[doc = "Checks if the value of the field is `TC_1`"]
    #[inline]
    pub fn is_tc_1(&self) -> bool {
        *self == TCR::TC_1
    }
}
#[doc = "Possible values of the field `BGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGER {
    #[doc = "No block gap event"]
    BGE_0,
    #[doc = "Transaction stopped at block gap"]
    BGE_1,
}
impl BGER {
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
            BGER::BGE_0 => false,
            BGER::BGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGER {
        match value {
            false => BGER::BGE_0,
            true => BGER::BGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGE_0`"]
    #[inline]
    pub fn is_bge_0(&self) -> bool {
        *self == BGER::BGE_0
    }
    #[doc = "Checks if the value of the field is `BGE_1`"]
    #[inline]
    pub fn is_bge_1(&self) -> bool {
        *self == BGER::BGE_1
    }
}
#[doc = "Possible values of the field `DINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTR {
    #[doc = "No DMA Interrupt"]
    DINT_0,
    #[doc = "DMA Interrupt is generated"]
    DINT_1,
}
impl DINTR {
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
            DINTR::DINT_0 => false,
            DINTR::DINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DINTR {
        match value {
            false => DINTR::DINT_0,
            true => DINTR::DINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DINT_0`"]
    #[inline]
    pub fn is_dint_0(&self) -> bool {
        *self == DINTR::DINT_0
    }
    #[doc = "Checks if the value of the field is `DINT_1`"]
    #[inline]
    pub fn is_dint_1(&self) -> bool {
        *self == DINTR::DINT_1
    }
}
#[doc = "Possible values of the field `BWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRR {
    #[doc = "Not ready to write buffer"]
    BWR_0,
    #[doc = "Ready to write buffer:"]
    BWR_1,
}
impl BWRR {
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
            BWRR::BWR_0 => false,
            BWRR::BWR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRR {
        match value {
            false => BWRR::BWR_0,
            true => BWRR::BWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWR_0`"]
    #[inline]
    pub fn is_bwr_0(&self) -> bool {
        *self == BWRR::BWR_0
    }
    #[doc = "Checks if the value of the field is `BWR_1`"]
    #[inline]
    pub fn is_bwr_1(&self) -> bool {
        *self == BWRR::BWR_1
    }
}
#[doc = "Possible values of the field `BRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRR {
    #[doc = "Not ready to read buffer"]
    BRR_0,
    #[doc = "Ready to read buffer"]
    BRR_1,
}
impl BRRR {
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
            BRRR::BRR_0 => false,
            BRRR::BRR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRRR {
        match value {
            false => BRRR::BRR_0,
            true => BRRR::BRR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRR_0`"]
    #[inline]
    pub fn is_brr_0(&self) -> bool {
        *self == BRRR::BRR_0
    }
    #[doc = "Checks if the value of the field is `BRR_1`"]
    #[inline]
    pub fn is_brr_1(&self) -> bool {
        *self == BRRR::BRR_1
    }
}
#[doc = "Possible values of the field `CINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSR {
    #[doc = "Card state unstable or removed"]
    CINS_0,
    #[doc = "Card inserted"]
    CINS_1,
}
impl CINSR {
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
            CINSR::CINS_0 => false,
            CINSR::CINS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSR {
        match value {
            false => CINSR::CINS_0,
            true => CINSR::CINS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINS_0`"]
    #[inline]
    pub fn is_cins_0(&self) -> bool {
        *self == CINSR::CINS_0
    }
    #[doc = "Checks if the value of the field is `CINS_1`"]
    #[inline]
    pub fn is_cins_1(&self) -> bool {
        *self == CINSR::CINS_1
    }
}
#[doc = "Possible values of the field `CRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMR {
    #[doc = "Card state unstable or inserted"]
    CRM_0,
    #[doc = "Card removed"]
    CRM_1,
}
impl CRMR {
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
            CRMR::CRM_0 => false,
            CRMR::CRM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRMR {
        match value {
            false => CRMR::CRM_0,
            true => CRMR::CRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRM_0`"]
    #[inline]
    pub fn is_crm_0(&self) -> bool {
        *self == CRMR::CRM_0
    }
    #[doc = "Checks if the value of the field is `CRM_1`"]
    #[inline]
    pub fn is_crm_1(&self) -> bool {
        *self == CRMR::CRM_1
    }
}
#[doc = "Possible values of the field `CINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTR {
    #[doc = "No Card Interrupt"]
    CINT_0,
    #[doc = "Generate Card Interrupt"]
    CINT_1,
}
impl CINTR {
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
            CINTR::CINT_0 => false,
            CINTR::CINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTR {
        match value {
            false => CINTR::CINT_0,
            true => CINTR::CINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINT_0`"]
    #[inline]
    pub fn is_cint_0(&self) -> bool {
        *self == CINTR::CINT_0
    }
    #[doc = "Checks if the value of the field is `CINT_1`"]
    #[inline]
    pub fn is_cint_1(&self) -> bool {
        *self == CINTR::CINT_1
    }
}
#[doc = "Possible values of the field `RTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTER {
    #[doc = "Re-Tuning is not required"]
    RTE_0,
    #[doc = "Re-Tuning should be performed"]
    RTE_1,
}
impl RTER {
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
            RTER::RTE_0 => false,
            RTER::RTE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTER {
        match value {
            false => RTER::RTE_0,
            true => RTER::RTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTE_0`"]
    #[inline]
    pub fn is_rte_0(&self) -> bool {
        *self == RTER::RTE_0
    }
    #[doc = "Checks if the value of the field is `RTE_1`"]
    #[inline]
    pub fn is_rte_1(&self) -> bool {
        *self == RTER::RTE_1
    }
}
#[doc = r" Value of the field"]
pub struct TPR {
    bits: bool,
}
impl TPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `CTOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOER {
    #[doc = "No Error"]
    CTOE_0,
    #[doc = "Time out"]
    CTOE_1,
}
impl CTOER {
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
            CTOER::CTOE_0 => false,
            CTOER::CTOE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTOER {
        match value {
            false => CTOER::CTOE_0,
            true => CTOER::CTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTOE_0`"]
    #[inline]
    pub fn is_ctoe_0(&self) -> bool {
        *self == CTOER::CTOE_0
    }
    #[doc = "Checks if the value of the field is `CTOE_1`"]
    #[inline]
    pub fn is_ctoe_1(&self) -> bool {
        *self == CTOER::CTOE_1
    }
}
#[doc = "Possible values of the field `CCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCER {
    #[doc = "No Error"]
    CCE_0,
    #[doc = "CRC Error Generated."]
    CCE_1,
}
impl CCER {
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
            CCER::CCE_0 => false,
            CCER::CCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCER {
        match value {
            false => CCER::CCE_0,
            true => CCER::CCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCE_0`"]
    #[inline]
    pub fn is_cce_0(&self) -> bool {
        *self == CCER::CCE_0
    }
    #[doc = "Checks if the value of the field is `CCE_1`"]
    #[inline]
    pub fn is_cce_1(&self) -> bool {
        *self == CCER::CCE_1
    }
}
#[doc = "Possible values of the field `CEBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBER {
    #[doc = "No Error"]
    CEBE_0,
    #[doc = "End Bit Error Generated"]
    CEBE_1,
}
impl CEBER {
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
            CEBER::CEBE_0 => false,
            CEBER::CEBE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEBER {
        match value {
            false => CEBER::CEBE_0,
            true => CEBER::CEBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEBE_0`"]
    #[inline]
    pub fn is_cebe_0(&self) -> bool {
        *self == CEBER::CEBE_0
    }
    #[doc = "Checks if the value of the field is `CEBE_1`"]
    #[inline]
    pub fn is_cebe_1(&self) -> bool {
        *self == CEBER::CEBE_1
    }
}
#[doc = "Possible values of the field `CIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIER {
    #[doc = "No Error"]
    CIE_0,
    #[doc = "Error"]
    CIE_1,
}
impl CIER {
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
            CIER::CIE_0 => false,
            CIER::CIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIER {
        match value {
            false => CIER::CIE_0,
            true => CIER::CIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIE_0`"]
    #[inline]
    pub fn is_cie_0(&self) -> bool {
        *self == CIER::CIE_0
    }
    #[doc = "Checks if the value of the field is `CIE_1`"]
    #[inline]
    pub fn is_cie_1(&self) -> bool {
        *self == CIER::CIE_1
    }
}
#[doc = "Possible values of the field `DTOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOER {
    #[doc = "No Error"]
    DTOE_0,
    #[doc = "Time out"]
    DTOE_1,
}
impl DTOER {
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
            DTOER::DTOE_0 => false,
            DTOER::DTOE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTOER {
        match value {
            false => DTOER::DTOE_0,
            true => DTOER::DTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTOE_0`"]
    #[inline]
    pub fn is_dtoe_0(&self) -> bool {
        *self == DTOER::DTOE_0
    }
    #[doc = "Checks if the value of the field is `DTOE_1`"]
    #[inline]
    pub fn is_dtoe_1(&self) -> bool {
        *self == DTOER::DTOE_1
    }
}
#[doc = "Possible values of the field `DCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCER {
    #[doc = "No Error"]
    DCE_0,
    #[doc = "Error"]
    DCE_1,
}
impl DCER {
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
            DCER::DCE_0 => false,
            DCER::DCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCER {
        match value {
            false => DCER::DCE_0,
            true => DCER::DCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCE_0`"]
    #[inline]
    pub fn is_dce_0(&self) -> bool {
        *self == DCER::DCE_0
    }
    #[doc = "Checks if the value of the field is `DCE_1`"]
    #[inline]
    pub fn is_dce_1(&self) -> bool {
        *self == DCER::DCE_1
    }
}
#[doc = "Possible values of the field `DEBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBER {
    #[doc = "No Error"]
    DEBE_0,
    #[doc = "Error"]
    DEBE_1,
}
impl DEBER {
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
            DEBER::DEBE_0 => false,
            DEBER::DEBE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBER {
        match value {
            false => DEBER::DEBE_0,
            true => DEBER::DEBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBE_0`"]
    #[inline]
    pub fn is_debe_0(&self) -> bool {
        *self == DEBER::DEBE_0
    }
    #[doc = "Checks if the value of the field is `DEBE_1`"]
    #[inline]
    pub fn is_debe_1(&self) -> bool {
        *self == DEBER::DEBE_1
    }
}
#[doc = "Possible values of the field `AC12E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12ER {
    #[doc = "No Error"]
    AC12E_0,
    #[doc = "Error"]
    AC12E_1,
}
impl AC12ER {
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
            AC12ER::AC12E_0 => false,
            AC12ER::AC12E_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12ER {
        match value {
            false => AC12ER::AC12E_0,
            true => AC12ER::AC12E_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12E_0`"]
    #[inline]
    pub fn is_ac12e_0(&self) -> bool {
        *self == AC12ER::AC12E_0
    }
    #[doc = "Checks if the value of the field is `AC12E_1`"]
    #[inline]
    pub fn is_ac12e_1(&self) -> bool {
        *self == AC12ER::AC12E_1
    }
}
#[doc = r" Value of the field"]
pub struct TNER {
    bits: bool,
}
impl TNER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `DMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAER {
    #[doc = "No Error"]
    DMAE_0,
    #[doc = "Error"]
    DMAE_1,
}
impl DMAER {
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
            DMAER::DMAE_0 => false,
            DMAER::DMAE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAER {
        match value {
            false => DMAER::DMAE_0,
            true => DMAER::DMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAE_0`"]
    #[inline]
    pub fn is_dmae_0(&self) -> bool {
        *self == DMAER::DMAE_0
    }
    #[doc = "Checks if the value of the field is `DMAE_1`"]
    #[inline]
    pub fn is_dmae_1(&self) -> bool {
        *self == DMAER::DMAE_1
    }
}
#[doc = "Values that can be written to the field `CC`"]
pub enum CCW {
    #[doc = "Command not complete"]
    CC_0,
    #[doc = "Command complete"]
    CC_1,
}
impl CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCW::CC_0 => false,
            CCW::CC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Command not complete"]
    #[inline]
    pub fn cc_0(self) -> &'a mut W {
        self.variant(CCW::CC_0)
    }
    #[doc = "Command complete"]
    #[inline]
    pub fn cc_1(self) -> &'a mut W {
        self.variant(CCW::CC_1)
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
#[doc = "Values that can be written to the field `TC`"]
pub enum TCW {
    #[doc = "Transfer not complete"]
    TC_0,
    #[doc = "Transfer complete"]
    TC_1,
}
impl TCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCW::TC_0 => false,
            TCW::TC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCW<'a> {
    w: &'a mut W,
}
impl<'a> _TCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer not complete"]
    #[inline]
    pub fn tc_0(self) -> &'a mut W {
        self.variant(TCW::TC_0)
    }
    #[doc = "Transfer complete"]
    #[inline]
    pub fn tc_1(self) -> &'a mut W {
        self.variant(TCW::TC_1)
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
#[doc = "Values that can be written to the field `BGE`"]
pub enum BGEW {
    #[doc = "No block gap event"]
    BGE_0,
    #[doc = "Transaction stopped at block gap"]
    BGE_1,
}
impl BGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGEW::BGE_0 => false,
            BGEW::BGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No block gap event"]
    #[inline]
    pub fn bge_0(self) -> &'a mut W {
        self.variant(BGEW::BGE_0)
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline]
    pub fn bge_1(self) -> &'a mut W {
        self.variant(BGEW::BGE_1)
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
#[doc = "Values that can be written to the field `DINT`"]
pub enum DINTW {
    #[doc = "No DMA Interrupt"]
    DINT_0,
    #[doc = "DMA Interrupt is generated"]
    DINT_1,
}
impl DINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DINTW::DINT_0 => false,
            DINTW::DINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINTW<'a> {
    w: &'a mut W,
}
impl<'a> _DINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA Interrupt"]
    #[inline]
    pub fn dint_0(self) -> &'a mut W {
        self.variant(DINTW::DINT_0)
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline]
    pub fn dint_1(self) -> &'a mut W {
        self.variant(DINTW::DINT_1)
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
#[doc = "Values that can be written to the field `BWR`"]
pub enum BWRW {
    #[doc = "Not ready to write buffer"]
    BWR_0,
    #[doc = "Ready to write buffer:"]
    BWR_1,
}
impl BWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRW::BWR_0 => false,
            BWRW::BWR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWRW<'a> {
    w: &'a mut W,
}
impl<'a> _BWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not ready to write buffer"]
    #[inline]
    pub fn bwr_0(self) -> &'a mut W {
        self.variant(BWRW::BWR_0)
    }
    #[doc = "Ready to write buffer:"]
    #[inline]
    pub fn bwr_1(self) -> &'a mut W {
        self.variant(BWRW::BWR_1)
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
#[doc = "Values that can be written to the field `BRR`"]
pub enum BRRW {
    #[doc = "Not ready to read buffer"]
    BRR_0,
    #[doc = "Ready to read buffer"]
    BRR_1,
}
impl BRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRRW::BRR_0 => false,
            BRRW::BRR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRRW<'a> {
    w: &'a mut W,
}
impl<'a> _BRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not ready to read buffer"]
    #[inline]
    pub fn brr_0(self) -> &'a mut W {
        self.variant(BRRW::BRR_0)
    }
    #[doc = "Ready to read buffer"]
    #[inline]
    pub fn brr_1(self) -> &'a mut W {
        self.variant(BRRW::BRR_1)
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
#[doc = "Values that can be written to the field `CINS`"]
pub enum CINSW {
    #[doc = "Card state unstable or removed"]
    CINS_0,
    #[doc = "Card inserted"]
    CINS_1,
}
impl CINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSW::CINS_0 => false,
            CINSW::CINS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card state unstable or removed"]
    #[inline]
    pub fn cins_0(self) -> &'a mut W {
        self.variant(CINSW::CINS_0)
    }
    #[doc = "Card inserted"]
    #[inline]
    pub fn cins_1(self) -> &'a mut W {
        self.variant(CINSW::CINS_1)
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
#[doc = "Values that can be written to the field `CRM`"]
pub enum CRMW {
    #[doc = "Card state unstable or inserted"]
    CRM_0,
    #[doc = "Card removed"]
    CRM_1,
}
impl CRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMW::CRM_0 => false,
            CRMW::CRM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card state unstable or inserted"]
    #[inline]
    pub fn crm_0(self) -> &'a mut W {
        self.variant(CRMW::CRM_0)
    }
    #[doc = "Card removed"]
    #[inline]
    pub fn crm_1(self) -> &'a mut W {
        self.variant(CRMW::CRM_1)
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
#[doc = "Values that can be written to the field `CINT`"]
pub enum CINTW {
    #[doc = "No Card Interrupt"]
    CINT_0,
    #[doc = "Generate Card Interrupt"]
    CINT_1,
}
impl CINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINTW::CINT_0 => false,
            CINTW::CINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINTW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline]
    pub fn cint_0(self) -> &'a mut W {
        self.variant(CINTW::CINT_0)
    }
    #[doc = "Generate Card Interrupt"]
    #[inline]
    pub fn cint_1(self) -> &'a mut W {
        self.variant(CINTW::CINT_1)
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
#[doc = "Values that can be written to the field `RTE`"]
pub enum RTEW {
    #[doc = "Re-Tuning is not required"]
    RTE_0,
    #[doc = "Re-Tuning should be performed"]
    RTE_1,
}
impl RTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTEW::RTE_0 => false,
            RTEW::RTE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Re-Tuning is not required"]
    #[inline]
    pub fn rte_0(self) -> &'a mut W {
        self.variant(RTEW::RTE_0)
    }
    #[doc = "Re-Tuning should be performed"]
    #[inline]
    pub fn rte_1(self) -> &'a mut W {
        self.variant(RTEW::RTE_1)
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
#[doc = r" Proxy"]
pub struct _TPW<'a> {
    w: &'a mut W,
}
impl<'a> _TPW<'a> {
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
#[doc = "Values that can be written to the field `CTOE`"]
pub enum CTOEW {
    #[doc = "No Error"]
    CTOE_0,
    #[doc = "Time out"]
    CTOE_1,
}
impl CTOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTOEW::CTOE_0 => false,
            CTOEW::CTOE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTOEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn ctoe_0(self) -> &'a mut W {
        self.variant(CTOEW::CTOE_0)
    }
    #[doc = "Time out"]
    #[inline]
    pub fn ctoe_1(self) -> &'a mut W {
        self.variant(CTOEW::CTOE_1)
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
#[doc = "Values that can be written to the field `CCE`"]
pub enum CCEW {
    #[doc = "No Error"]
    CCE_0,
    #[doc = "CRC Error Generated."]
    CCE_1,
}
impl CCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCEW::CCE_0 => false,
            CCEW::CCE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn cce_0(self) -> &'a mut W {
        self.variant(CCEW::CCE_0)
    }
    #[doc = "CRC Error Generated."]
    #[inline]
    pub fn cce_1(self) -> &'a mut W {
        self.variant(CCEW::CCE_1)
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
#[doc = "Values that can be written to the field `CEBE`"]
pub enum CEBEW {
    #[doc = "No Error"]
    CEBE_0,
    #[doc = "End Bit Error Generated"]
    CEBE_1,
}
impl CEBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEBEW::CEBE_0 => false,
            CEBEW::CEBE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEBEW<'a> {
    w: &'a mut W,
}
impl<'a> _CEBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEBEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn cebe_0(self) -> &'a mut W {
        self.variant(CEBEW::CEBE_0)
    }
    #[doc = "End Bit Error Generated"]
    #[inline]
    pub fn cebe_1(self) -> &'a mut W {
        self.variant(CEBEW::CEBE_1)
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
#[doc = "Values that can be written to the field `CIE`"]
pub enum CIEW {
    #[doc = "No Error"]
    CIE_0,
    #[doc = "Error"]
    CIE_1,
}
impl CIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIEW::CIE_0 => false,
            CIEW::CIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn cie_0(self) -> &'a mut W {
        self.variant(CIEW::CIE_0)
    }
    #[doc = "Error"]
    #[inline]
    pub fn cie_1(self) -> &'a mut W {
        self.variant(CIEW::CIE_1)
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
#[doc = "Values that can be written to the field `DTOE`"]
pub enum DTOEW {
    #[doc = "No Error"]
    DTOE_0,
    #[doc = "Time out"]
    DTOE_1,
}
impl DTOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTOEW::DTOE_0 => false,
            DTOEW::DTOE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOEW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn dtoe_0(self) -> &'a mut W {
        self.variant(DTOEW::DTOE_0)
    }
    #[doc = "Time out"]
    #[inline]
    pub fn dtoe_1(self) -> &'a mut W {
        self.variant(DTOEW::DTOE_1)
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
#[doc = "Values that can be written to the field `DCE`"]
pub enum DCEW {
    #[doc = "No Error"]
    DCE_0,
    #[doc = "Error"]
    DCE_1,
}
impl DCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEW::DCE_0 => false,
            DCEW::DCE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn dce_0(self) -> &'a mut W {
        self.variant(DCEW::DCE_0)
    }
    #[doc = "Error"]
    #[inline]
    pub fn dce_1(self) -> &'a mut W {
        self.variant(DCEW::DCE_1)
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
#[doc = "Values that can be written to the field `DEBE`"]
pub enum DEBEW {
    #[doc = "No Error"]
    DEBE_0,
    #[doc = "Error"]
    DEBE_1,
}
impl DEBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBEW::DEBE_0 => false,
            DEBEW::DEBE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn debe_0(self) -> &'a mut W {
        self.variant(DEBEW::DEBE_0)
    }
    #[doc = "Error"]
    #[inline]
    pub fn debe_1(self) -> &'a mut W {
        self.variant(DEBEW::DEBE_1)
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
#[doc = "Values that can be written to the field `AC12E`"]
pub enum AC12EW {
    #[doc = "No Error"]
    AC12E_0,
    #[doc = "Error"]
    AC12E_1,
}
impl AC12EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12EW::AC12E_0 => false,
            AC12EW::AC12E_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12EW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn ac12e_0(self) -> &'a mut W {
        self.variant(AC12EW::AC12E_0)
    }
    #[doc = "Error"]
    #[inline]
    pub fn ac12e_1(self) -> &'a mut W {
        self.variant(AC12EW::AC12E_1)
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
#[doc = r" Proxy"]
pub struct _TNEW<'a> {
    w: &'a mut W,
}
impl<'a> _TNEW<'a> {
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
#[doc = "Values that can be written to the field `DMAE`"]
pub enum DMAEW {
    #[doc = "No Error"]
    DMAE_0,
    #[doc = "Error"]
    DMAE_1,
}
impl DMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAEW::DMAE_0 => false,
            DMAEW::DMAE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline]
    pub fn dmae_0(self) -> &'a mut W {
        self.variant(DMAEW::DMAE_0)
    }
    #[doc = "Error"]
    #[inline]
    pub fn dmae_1(self) -> &'a mut W {
        self.variant(DMAEW::DMAE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cc(&self) -> CCR {
        CCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn tc(&self) -> TCR {
        TCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn bge(&self) -> BGER {
        BGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline]
    pub fn dint(&self) -> DINTR {
        DINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn bwr(&self) -> BWRR {
        BWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn brr(&self) -> BRRR {
        BRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn cins(&self) -> CINSR {
        CINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn crm(&self) -> CRMR {
        CRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline]
    pub fn cint(&self) -> CINTR {
        CINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn rte(&self) -> RTER {
        RTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn tp(&self) -> TPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TPR { bits }
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline]
    pub fn ctoe(&self) -> CTOER {
        CTOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline]
    pub fn cce(&self) -> CCER {
        CCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline]
    pub fn cebe(&self) -> CEBER {
        CEBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline]
    pub fn cie(&self) -> CIER {
        CIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline]
    pub fn dtoe(&self) -> DTOER {
        DTOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline]
    pub fn dce(&self) -> DCER {
        DCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline]
    pub fn debe(&self) -> DEBER {
        DEBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline]
    pub fn ac12e(&self) -> AC12ER {
        AC12ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn tne(&self) -> TNER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TNER { bits }
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline]
    pub fn dmae(&self) -> DMAER {
        DMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cc(&mut self) -> _CCW {
        _CCW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn tc(&mut self) -> _TCW {
        _TCW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn bge(&mut self) -> _BGEW {
        _BGEW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline]
    pub fn dint(&mut self) -> _DINTW {
        _DINTW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn bwr(&mut self) -> _BWRW {
        _BWRW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn brr(&mut self) -> _BRRW {
        _BRRW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn cins(&mut self) -> _CINSW {
        _CINSW { w: self }
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn crm(&mut self) -> _CRMW {
        _CRMW { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline]
    pub fn cint(&mut self) -> _CINTW {
        _CINTW { w: self }
    }
    #[doc = "Bit 12 - Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn rte(&mut self) -> _RTEW {
        _RTEW { w: self }
    }
    #[doc = "Bit 14 - Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn tp(&mut self) -> _TPW {
        _TPW { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline]
    pub fn ctoe(&mut self) -> _CTOEW {
        _CTOEW { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline]
    pub fn cce(&mut self) -> _CCEW {
        _CCEW { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline]
    pub fn cebe(&mut self) -> _CEBEW {
        _CEBEW { w: self }
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline]
    pub fn cie(&mut self) -> _CIEW {
        _CIEW { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline]
    pub fn dtoe(&mut self) -> _DTOEW {
        _DTOEW { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline]
    pub fn dce(&mut self) -> _DCEW {
        _DCEW { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline]
    pub fn debe(&mut self) -> _DEBEW {
        _DEBEW { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline]
    pub fn ac12e(&mut self) -> _AC12EW {
        _AC12EW { w: self }
    }
    #[doc = "Bit 26 - Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn tne(&mut self) -> _TNEW {
        _TNEW { w: self }
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline]
    pub fn dmae(&mut self) -> _DMAEW {
        _DMAEW { w: self }
    }
}
