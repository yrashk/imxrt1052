#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INT_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HW_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_ERRR {
    #[doc = "no error"]
    HW_ERR_0,
    #[doc = "error detected."]
    HW_ERR_1,
}
impl HW_ERRR {
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
            HW_ERRR::HW_ERR_0 => false,
            HW_ERRR::HW_ERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HW_ERRR {
        match value {
            false => HW_ERRR::HW_ERR_0,
            true => HW_ERRR::HW_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `HW_ERR_0`"]
    #[inline]
    pub fn is_hw_err_0(&self) -> bool {
        *self == HW_ERRR::HW_ERR_0
    }
    #[doc = "Checks if the value of the field is `HW_ERR_1`"]
    #[inline]
    pub fn is_hw_err_1(&self) -> bool {
        *self == HW_ERRR::HW_ERR_1
    }
}
#[doc = "Possible values of the field `ENT_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENT_VALR {
    #[doc = "Busy generation entropy. Any value read is invalid."]
    ENT_VAL_0,
    #[doc = "TRNG can be stopped and entropy is valid if read."]
    ENT_VAL_1,
}
impl ENT_VALR {
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
            ENT_VALR::ENT_VAL_0 => false,
            ENT_VALR::ENT_VAL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENT_VALR {
        match value {
            false => ENT_VALR::ENT_VAL_0,
            true => ENT_VALR::ENT_VAL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_0`"]
    #[inline]
    pub fn is_ent_val_0(&self) -> bool {
        *self == ENT_VALR::ENT_VAL_0
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_1`"]
    #[inline]
    pub fn is_ent_val_1(&self) -> bool {
        *self == ENT_VALR::ENT_VAL_1
    }
}
#[doc = "Possible values of the field `FRQ_CT_FAIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRQ_CT_FAILR {
    #[doc = "No hardware nor self test frequency errors."]
    FRQ_CT_FAIL_0,
    #[doc = "The frequency counter has detected a failure."]
    FRQ_CT_FAIL_1,
}
impl FRQ_CT_FAILR {
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
            FRQ_CT_FAILR::FRQ_CT_FAIL_0 => false,
            FRQ_CT_FAILR::FRQ_CT_FAIL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRQ_CT_FAILR {
        match value {
            false => FRQ_CT_FAILR::FRQ_CT_FAIL_0,
            true => FRQ_CT_FAILR::FRQ_CT_FAIL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_0`"]
    #[inline]
    pub fn is_frq_ct_fail_0(&self) -> bool {
        *self == FRQ_CT_FAILR::FRQ_CT_FAIL_0
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_1`"]
    #[inline]
    pub fn is_frq_ct_fail_1(&self) -> bool {
        *self == FRQ_CT_FAILR::FRQ_CT_FAIL_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Read: Error status"]
    #[inline]
    pub fn hw_err(&self) -> HW_ERRR {
        HW_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Read only: Entropy Valid"]
    #[inline]
    pub fn ent_val(&self) -> ENT_VALR {
        ENT_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAILR {
        FRQ_CT_FAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
