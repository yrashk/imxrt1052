#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR14 {
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
#[doc = "Possible values of the field `ACMP1_CMP_IGEN_TRIM_DN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_CMP_IGEN_TRIM_DNR {
    #[doc = "no reduce"]
    ACMP1_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP1_CMP_IGEN_TRIM_DN_1,
}
impl ACMP1_CMP_IGEN_TRIM_DNR {
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
            ACMP1_CMP_IGEN_TRIM_DNR::ACMP1_CMP_IGEN_TRIM_DN_0 => false,
            ACMP1_CMP_IGEN_TRIM_DNR::ACMP1_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP1_CMP_IGEN_TRIM_DNR {
        match value {
            false => ACMP1_CMP_IGEN_TRIM_DNR::ACMP1_CMP_IGEN_TRIM_DN_0,
            true => ACMP1_CMP_IGEN_TRIM_DNR::ACMP1_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_DN_0`"]
    #[inline]
    pub fn is_acmp1_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_DNR::ACMP1_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_DN_1`"]
    #[inline]
    pub fn is_acmp1_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_DNR::ACMP1_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Possible values of the field `ACMP2_CMP_IGEN_TRIM_DN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP2_CMP_IGEN_TRIM_DNR {
    #[doc = "no reduce"]
    ACMP2_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP2_CMP_IGEN_TRIM_DN_1,
}
impl ACMP2_CMP_IGEN_TRIM_DNR {
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
            ACMP2_CMP_IGEN_TRIM_DNR::ACMP2_CMP_IGEN_TRIM_DN_0 => false,
            ACMP2_CMP_IGEN_TRIM_DNR::ACMP2_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP2_CMP_IGEN_TRIM_DNR {
        match value {
            false => ACMP2_CMP_IGEN_TRIM_DNR::ACMP2_CMP_IGEN_TRIM_DN_0,
            true => ACMP2_CMP_IGEN_TRIM_DNR::ACMP2_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_DN_0`"]
    #[inline]
    pub fn is_acmp2_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_DNR::ACMP2_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_DN_1`"]
    #[inline]
    pub fn is_acmp2_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_DNR::ACMP2_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Possible values of the field `ACMP3_CMP_IGEN_TRIM_DN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP3_CMP_IGEN_TRIM_DNR {
    #[doc = "no reduce"]
    ACMP3_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP3_CMP_IGEN_TRIM_DN_1,
}
impl ACMP3_CMP_IGEN_TRIM_DNR {
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
            ACMP3_CMP_IGEN_TRIM_DNR::ACMP3_CMP_IGEN_TRIM_DN_0 => false,
            ACMP3_CMP_IGEN_TRIM_DNR::ACMP3_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP3_CMP_IGEN_TRIM_DNR {
        match value {
            false => ACMP3_CMP_IGEN_TRIM_DNR::ACMP3_CMP_IGEN_TRIM_DN_0,
            true => ACMP3_CMP_IGEN_TRIM_DNR::ACMP3_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_DN_0`"]
    #[inline]
    pub fn is_acmp3_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_DNR::ACMP3_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_DN_1`"]
    #[inline]
    pub fn is_acmp3_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_DNR::ACMP3_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Possible values of the field `ACMP4_CMP_IGEN_TRIM_DN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP4_CMP_IGEN_TRIM_DNR {
    #[doc = "no reduce"]
    ACMP4_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP4_CMP_IGEN_TRIM_DN_1,
}
impl ACMP4_CMP_IGEN_TRIM_DNR {
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
            ACMP4_CMP_IGEN_TRIM_DNR::ACMP4_CMP_IGEN_TRIM_DN_0 => false,
            ACMP4_CMP_IGEN_TRIM_DNR::ACMP4_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP4_CMP_IGEN_TRIM_DNR {
        match value {
            false => ACMP4_CMP_IGEN_TRIM_DNR::ACMP4_CMP_IGEN_TRIM_DN_0,
            true => ACMP4_CMP_IGEN_TRIM_DNR::ACMP4_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_DN_0`"]
    #[inline]
    pub fn is_acmp4_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_DNR::ACMP4_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_DN_1`"]
    #[inline]
    pub fn is_acmp4_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_DNR::ACMP4_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Possible values of the field `ACMP1_CMP_IGEN_TRIM_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_CMP_IGEN_TRIM_UPR {
    #[doc = "no increase"]
    ACMP1_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP1_CMP_IGEN_TRIM_UP_1,
}
impl ACMP1_CMP_IGEN_TRIM_UPR {
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
            ACMP1_CMP_IGEN_TRIM_UPR::ACMP1_CMP_IGEN_TRIM_UP_0 => false,
            ACMP1_CMP_IGEN_TRIM_UPR::ACMP1_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP1_CMP_IGEN_TRIM_UPR {
        match value {
            false => ACMP1_CMP_IGEN_TRIM_UPR::ACMP1_CMP_IGEN_TRIM_UP_0,
            true => ACMP1_CMP_IGEN_TRIM_UPR::ACMP1_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_UP_0`"]
    #[inline]
    pub fn is_acmp1_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_UPR::ACMP1_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_UP_1`"]
    #[inline]
    pub fn is_acmp1_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_UPR::ACMP1_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Possible values of the field `ACMP2_CMP_IGEN_TRIM_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP2_CMP_IGEN_TRIM_UPR {
    #[doc = "no increase"]
    ACMP2_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP2_CMP_IGEN_TRIM_UP_1,
}
impl ACMP2_CMP_IGEN_TRIM_UPR {
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
            ACMP2_CMP_IGEN_TRIM_UPR::ACMP2_CMP_IGEN_TRIM_UP_0 => false,
            ACMP2_CMP_IGEN_TRIM_UPR::ACMP2_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP2_CMP_IGEN_TRIM_UPR {
        match value {
            false => ACMP2_CMP_IGEN_TRIM_UPR::ACMP2_CMP_IGEN_TRIM_UP_0,
            true => ACMP2_CMP_IGEN_TRIM_UPR::ACMP2_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_UP_0`"]
    #[inline]
    pub fn is_acmp2_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_UPR::ACMP2_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_UP_1`"]
    #[inline]
    pub fn is_acmp2_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_UPR::ACMP2_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Possible values of the field `ACMP3_CMP_IGEN_TRIM_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP3_CMP_IGEN_TRIM_UPR {
    #[doc = "no increase"]
    ACMP3_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP3_CMP_IGEN_TRIM_UP_1,
}
impl ACMP3_CMP_IGEN_TRIM_UPR {
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
            ACMP3_CMP_IGEN_TRIM_UPR::ACMP3_CMP_IGEN_TRIM_UP_0 => false,
            ACMP3_CMP_IGEN_TRIM_UPR::ACMP3_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP3_CMP_IGEN_TRIM_UPR {
        match value {
            false => ACMP3_CMP_IGEN_TRIM_UPR::ACMP3_CMP_IGEN_TRIM_UP_0,
            true => ACMP3_CMP_IGEN_TRIM_UPR::ACMP3_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_UP_0`"]
    #[inline]
    pub fn is_acmp3_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_UPR::ACMP3_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_UP_1`"]
    #[inline]
    pub fn is_acmp3_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_UPR::ACMP3_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Possible values of the field `ACMP4_CMP_IGEN_TRIM_UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP4_CMP_IGEN_TRIM_UPR {
    #[doc = "no increase"]
    ACMP4_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP4_CMP_IGEN_TRIM_UP_1,
}
impl ACMP4_CMP_IGEN_TRIM_UPR {
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
            ACMP4_CMP_IGEN_TRIM_UPR::ACMP4_CMP_IGEN_TRIM_UP_0 => false,
            ACMP4_CMP_IGEN_TRIM_UPR::ACMP4_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP4_CMP_IGEN_TRIM_UPR {
        match value {
            false => ACMP4_CMP_IGEN_TRIM_UPR::ACMP4_CMP_IGEN_TRIM_UP_0,
            true => ACMP4_CMP_IGEN_TRIM_UPR::ACMP4_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_UP_0`"]
    #[inline]
    pub fn is_acmp4_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_UPR::ACMP4_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_UP_1`"]
    #[inline]
    pub fn is_acmp4_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_UPR::ACMP4_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Possible values of the field `ACMP1_SAMPLE_SYNC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_SAMPLE_SYNC_ENR {
    #[doc = "select XBAR output"]
    ACMP1_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP1_SAMPLE_SYNC_EN_1,
}
impl ACMP1_SAMPLE_SYNC_ENR {
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
            ACMP1_SAMPLE_SYNC_ENR::ACMP1_SAMPLE_SYNC_EN_0 => false,
            ACMP1_SAMPLE_SYNC_ENR::ACMP1_SAMPLE_SYNC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP1_SAMPLE_SYNC_ENR {
        match value {
            false => ACMP1_SAMPLE_SYNC_ENR::ACMP1_SAMPLE_SYNC_EN_0,
            true => ACMP1_SAMPLE_SYNC_ENR::ACMP1_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_SAMPLE_SYNC_EN_0`"]
    #[inline]
    pub fn is_acmp1_sample_sync_en_0(&self) -> bool {
        *self == ACMP1_SAMPLE_SYNC_ENR::ACMP1_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_SAMPLE_SYNC_EN_1`"]
    #[inline]
    pub fn is_acmp1_sample_sync_en_1(&self) -> bool {
        *self == ACMP1_SAMPLE_SYNC_ENR::ACMP1_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Possible values of the field `ACMP2_SAMPLE_SYNC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP2_SAMPLE_SYNC_ENR {
    #[doc = "select XBAR output"]
    ACMP2_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP2_SAMPLE_SYNC_EN_1,
}
impl ACMP2_SAMPLE_SYNC_ENR {
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
            ACMP2_SAMPLE_SYNC_ENR::ACMP2_SAMPLE_SYNC_EN_0 => false,
            ACMP2_SAMPLE_SYNC_ENR::ACMP2_SAMPLE_SYNC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP2_SAMPLE_SYNC_ENR {
        match value {
            false => ACMP2_SAMPLE_SYNC_ENR::ACMP2_SAMPLE_SYNC_EN_0,
            true => ACMP2_SAMPLE_SYNC_ENR::ACMP2_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_SAMPLE_SYNC_EN_0`"]
    #[inline]
    pub fn is_acmp2_sample_sync_en_0(&self) -> bool {
        *self == ACMP2_SAMPLE_SYNC_ENR::ACMP2_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_SAMPLE_SYNC_EN_1`"]
    #[inline]
    pub fn is_acmp2_sample_sync_en_1(&self) -> bool {
        *self == ACMP2_SAMPLE_SYNC_ENR::ACMP2_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Possible values of the field `ACMP3_SAMPLE_SYNC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP3_SAMPLE_SYNC_ENR {
    #[doc = "select XBAR output"]
    ACMP3_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP3_SAMPLE_SYNC_EN_1,
}
impl ACMP3_SAMPLE_SYNC_ENR {
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
            ACMP3_SAMPLE_SYNC_ENR::ACMP3_SAMPLE_SYNC_EN_0 => false,
            ACMP3_SAMPLE_SYNC_ENR::ACMP3_SAMPLE_SYNC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP3_SAMPLE_SYNC_ENR {
        match value {
            false => ACMP3_SAMPLE_SYNC_ENR::ACMP3_SAMPLE_SYNC_EN_0,
            true => ACMP3_SAMPLE_SYNC_ENR::ACMP3_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_SAMPLE_SYNC_EN_0`"]
    #[inline]
    pub fn is_acmp3_sample_sync_en_0(&self) -> bool {
        *self == ACMP3_SAMPLE_SYNC_ENR::ACMP3_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_SAMPLE_SYNC_EN_1`"]
    #[inline]
    pub fn is_acmp3_sample_sync_en_1(&self) -> bool {
        *self == ACMP3_SAMPLE_SYNC_ENR::ACMP3_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Possible values of the field `ACMP4_SAMPLE_SYNC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP4_SAMPLE_SYNC_ENR {
    #[doc = "select XBAR output"]
    ACMP4_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP4_SAMPLE_SYNC_EN_1,
}
impl ACMP4_SAMPLE_SYNC_ENR {
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
            ACMP4_SAMPLE_SYNC_ENR::ACMP4_SAMPLE_SYNC_EN_0 => false,
            ACMP4_SAMPLE_SYNC_ENR::ACMP4_SAMPLE_SYNC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP4_SAMPLE_SYNC_ENR {
        match value {
            false => ACMP4_SAMPLE_SYNC_ENR::ACMP4_SAMPLE_SYNC_EN_0,
            true => ACMP4_SAMPLE_SYNC_ENR::ACMP4_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_SAMPLE_SYNC_EN_0`"]
    #[inline]
    pub fn is_acmp4_sample_sync_en_0(&self) -> bool {
        *self == ACMP4_SAMPLE_SYNC_ENR::ACMP4_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_SAMPLE_SYNC_EN_1`"]
    #[inline]
    pub fn is_acmp4_sample_sync_en_1(&self) -> bool {
        *self == ACMP4_SAMPLE_SYNC_ENR::ACMP4_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Possible values of the field `CM7_MX6RT_CFGITCMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM7_MX6RT_CFGITCMSZR {
    #[doc = "0 KB (No ITCM)"]
    CM7_MX6RT_CFGITCMSZ_0,
    #[doc = "4 KB"]
    CM7_MX6RT_CFGITCMSZ_3,
    #[doc = "8 KB"]
    CM7_MX6RT_CFGITCMSZ_4,
    #[doc = "16 KB"]
    CM7_MX6RT_CFGITCMSZ_5,
    #[doc = "32 KB"]
    CM7_MX6RT_CFGITCMSZ_6,
    #[doc = "64 KB"]
    CM7_MX6RT_CFGITCMSZ_7,
    #[doc = "128 KB"]
    CM7_MX6RT_CFGITCMSZ_8,
    #[doc = "256 KB"]
    CM7_MX6RT_CFGITCMSZ_9,
    #[doc = "512 KB"]
    CM7_MX6RT_CFGITCMSZ_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM7_MX6RT_CFGITCMSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_0 => 0,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_3 => 3,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_4 => 4,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_5 => 5,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_6 => 6,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_7 => 7,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_8 => 8,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_9 => 9,
            CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_10 => 10,
            CM7_MX6RT_CFGITCMSZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM7_MX6RT_CFGITCMSZR {
        match value {
            0 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_0,
            3 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_3,
            4 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_4,
            5 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_5,
            6 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_6,
            7 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_7,
            8 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_8,
            9 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_9,
            10 => CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_10,
            i => CM7_MX6RT_CFGITCMSZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_0`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_0(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_0
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_3`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_3(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_3
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_4`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_4(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_4
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_5`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_5(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_5
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_6`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_6(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_6
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_7`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_7(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_7
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_8`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_8(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_8
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_9`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_9(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_9
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGITCMSZ_10`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgitcmsz_10(&self) -> bool {
        *self == CM7_MX6RT_CFGITCMSZR::CM7_MX6RT_CFGITCMSZ_10
    }
}
#[doc = "Possible values of the field `CM7_MX6RT_CFGDTCMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM7_MX6RT_CFGDTCMSZR {
    #[doc = "0 KB (No DTCM)"]
    CM7_MX6RT_CFGDTCMSZ_0,
    #[doc = "4 KB"]
    CM7_MX6RT_CFGDTCMSZ_3,
    #[doc = "8 KB"]
    CM7_MX6RT_CFGDTCMSZ_4,
    #[doc = "16 KB"]
    CM7_MX6RT_CFGDTCMSZ_5,
    #[doc = "32 KB"]
    CM7_MX6RT_CFGDTCMSZ_6,
    #[doc = "64 KB"]
    CM7_MX6RT_CFGDTCMSZ_7,
    #[doc = "128 KB"]
    CM7_MX6RT_CFGDTCMSZ_8,
    #[doc = "256 KB"]
    CM7_MX6RT_CFGDTCMSZ_9,
    #[doc = "512 KB"]
    CM7_MX6RT_CFGDTCMSZ_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM7_MX6RT_CFGDTCMSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_0 => 0,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_3 => 3,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_4 => 4,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_5 => 5,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_6 => 6,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_7 => 7,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_8 => 8,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_9 => 9,
            CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_10 => 10,
            CM7_MX6RT_CFGDTCMSZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM7_MX6RT_CFGDTCMSZR {
        match value {
            0 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_0,
            3 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_3,
            4 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_4,
            5 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_5,
            6 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_6,
            7 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_7,
            8 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_8,
            9 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_9,
            10 => CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_10,
            i => CM7_MX6RT_CFGDTCMSZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_0`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_0(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_0
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_3`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_3(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_3
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_4`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_4(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_4
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_5`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_5(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_5
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_6`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_6(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_6
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_7`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_7(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_7
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_8`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_8(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_8
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_9`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_9(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_9
    }
    #[doc = "Checks if the value of the field is `CM7_MX6RT_CFGDTCMSZ_10`"]
    #[inline]
    pub fn is_cm7_mx6rt_cfgdtcmsz_10(&self) -> bool {
        *self == CM7_MX6RT_CFGDTCMSZR::CM7_MX6RT_CFGDTCMSZ_10
    }
}
#[doc = "Values that can be written to the field `ACMP1_CMP_IGEN_TRIM_DN`"]
pub enum ACMP1_CMP_IGEN_TRIM_DNW {
    #[doc = "no reduce"]
    ACMP1_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP1_CMP_IGEN_TRIM_DN_1,
}
impl ACMP1_CMP_IGEN_TRIM_DNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP1_CMP_IGEN_TRIM_DNW::ACMP1_CMP_IGEN_TRIM_DN_0 => false,
            ACMP1_CMP_IGEN_TRIM_DNW::ACMP1_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1_CMP_IGEN_TRIM_DNW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1_CMP_IGEN_TRIM_DNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP1_CMP_IGEN_TRIM_DNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no reduce"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_DNW::ACMP1_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_DNW::ACMP1_CMP_IGEN_TRIM_DN_1)
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
#[doc = "Values that can be written to the field `ACMP2_CMP_IGEN_TRIM_DN`"]
pub enum ACMP2_CMP_IGEN_TRIM_DNW {
    #[doc = "no reduce"]
    ACMP2_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP2_CMP_IGEN_TRIM_DN_1,
}
impl ACMP2_CMP_IGEN_TRIM_DNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP2_CMP_IGEN_TRIM_DNW::ACMP2_CMP_IGEN_TRIM_DN_0 => false,
            ACMP2_CMP_IGEN_TRIM_DNW::ACMP2_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP2_CMP_IGEN_TRIM_DNW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP2_CMP_IGEN_TRIM_DNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP2_CMP_IGEN_TRIM_DNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no reduce"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_DNW::ACMP2_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_DNW::ACMP2_CMP_IGEN_TRIM_DN_1)
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
#[doc = "Values that can be written to the field `ACMP3_CMP_IGEN_TRIM_DN`"]
pub enum ACMP3_CMP_IGEN_TRIM_DNW {
    #[doc = "no reduce"]
    ACMP3_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP3_CMP_IGEN_TRIM_DN_1,
}
impl ACMP3_CMP_IGEN_TRIM_DNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP3_CMP_IGEN_TRIM_DNW::ACMP3_CMP_IGEN_TRIM_DN_0 => false,
            ACMP3_CMP_IGEN_TRIM_DNW::ACMP3_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP3_CMP_IGEN_TRIM_DNW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP3_CMP_IGEN_TRIM_DNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP3_CMP_IGEN_TRIM_DNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no reduce"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_DNW::ACMP3_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_DNW::ACMP3_CMP_IGEN_TRIM_DN_1)
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
#[doc = "Values that can be written to the field `ACMP4_CMP_IGEN_TRIM_DN`"]
pub enum ACMP4_CMP_IGEN_TRIM_DNW {
    #[doc = "no reduce"]
    ACMP4_CMP_IGEN_TRIM_DN_0,
    #[doc = "reduces"]
    ACMP4_CMP_IGEN_TRIM_DN_1,
}
impl ACMP4_CMP_IGEN_TRIM_DNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP4_CMP_IGEN_TRIM_DNW::ACMP4_CMP_IGEN_TRIM_DN_0 => false,
            ACMP4_CMP_IGEN_TRIM_DNW::ACMP4_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP4_CMP_IGEN_TRIM_DNW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP4_CMP_IGEN_TRIM_DNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP4_CMP_IGEN_TRIM_DNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no reduce"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_DNW::ACMP4_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_DNW::ACMP4_CMP_IGEN_TRIM_DN_1)
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
#[doc = "Values that can be written to the field `ACMP1_CMP_IGEN_TRIM_UP`"]
pub enum ACMP1_CMP_IGEN_TRIM_UPW {
    #[doc = "no increase"]
    ACMP1_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP1_CMP_IGEN_TRIM_UP_1,
}
impl ACMP1_CMP_IGEN_TRIM_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP1_CMP_IGEN_TRIM_UPW::ACMP1_CMP_IGEN_TRIM_UP_0 => false,
            ACMP1_CMP_IGEN_TRIM_UPW::ACMP1_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1_CMP_IGEN_TRIM_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1_CMP_IGEN_TRIM_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP1_CMP_IGEN_TRIM_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no increase"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_UPW::ACMP1_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_UPW::ACMP1_CMP_IGEN_TRIM_UP_1)
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
#[doc = "Values that can be written to the field `ACMP2_CMP_IGEN_TRIM_UP`"]
pub enum ACMP2_CMP_IGEN_TRIM_UPW {
    #[doc = "no increase"]
    ACMP2_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP2_CMP_IGEN_TRIM_UP_1,
}
impl ACMP2_CMP_IGEN_TRIM_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP2_CMP_IGEN_TRIM_UPW::ACMP2_CMP_IGEN_TRIM_UP_0 => false,
            ACMP2_CMP_IGEN_TRIM_UPW::ACMP2_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP2_CMP_IGEN_TRIM_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP2_CMP_IGEN_TRIM_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP2_CMP_IGEN_TRIM_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no increase"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_UPW::ACMP2_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_UPW::ACMP2_CMP_IGEN_TRIM_UP_1)
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
#[doc = "Values that can be written to the field `ACMP3_CMP_IGEN_TRIM_UP`"]
pub enum ACMP3_CMP_IGEN_TRIM_UPW {
    #[doc = "no increase"]
    ACMP3_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP3_CMP_IGEN_TRIM_UP_1,
}
impl ACMP3_CMP_IGEN_TRIM_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP3_CMP_IGEN_TRIM_UPW::ACMP3_CMP_IGEN_TRIM_UP_0 => false,
            ACMP3_CMP_IGEN_TRIM_UPW::ACMP3_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP3_CMP_IGEN_TRIM_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP3_CMP_IGEN_TRIM_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP3_CMP_IGEN_TRIM_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no increase"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_UPW::ACMP3_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_UPW::ACMP3_CMP_IGEN_TRIM_UP_1)
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
#[doc = "Values that can be written to the field `ACMP4_CMP_IGEN_TRIM_UP`"]
pub enum ACMP4_CMP_IGEN_TRIM_UPW {
    #[doc = "no increase"]
    ACMP4_CMP_IGEN_TRIM_UP_0,
    #[doc = "increases"]
    ACMP4_CMP_IGEN_TRIM_UP_1,
}
impl ACMP4_CMP_IGEN_TRIM_UPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP4_CMP_IGEN_TRIM_UPW::ACMP4_CMP_IGEN_TRIM_UP_0 => false,
            ACMP4_CMP_IGEN_TRIM_UPW::ACMP4_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP4_CMP_IGEN_TRIM_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP4_CMP_IGEN_TRIM_UPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP4_CMP_IGEN_TRIM_UPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no increase"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_UPW::ACMP4_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_UPW::ACMP4_CMP_IGEN_TRIM_UP_1)
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
#[doc = "Values that can be written to the field `ACMP1_SAMPLE_SYNC_EN`"]
pub enum ACMP1_SAMPLE_SYNC_ENW {
    #[doc = "select XBAR output"]
    ACMP1_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP1_SAMPLE_SYNC_EN_1,
}
impl ACMP1_SAMPLE_SYNC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP1_SAMPLE_SYNC_ENW::ACMP1_SAMPLE_SYNC_EN_0 => false,
            ACMP1_SAMPLE_SYNC_ENW::ACMP1_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1_SAMPLE_SYNC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1_SAMPLE_SYNC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP1_SAMPLE_SYNC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "select XBAR output"]
    #[inline]
    pub fn acmp1_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP1_SAMPLE_SYNC_ENW::ACMP1_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline]
    pub fn acmp1_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP1_SAMPLE_SYNC_ENW::ACMP1_SAMPLE_SYNC_EN_1)
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
#[doc = "Values that can be written to the field `ACMP2_SAMPLE_SYNC_EN`"]
pub enum ACMP2_SAMPLE_SYNC_ENW {
    #[doc = "select XBAR output"]
    ACMP2_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP2_SAMPLE_SYNC_EN_1,
}
impl ACMP2_SAMPLE_SYNC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP2_SAMPLE_SYNC_ENW::ACMP2_SAMPLE_SYNC_EN_0 => false,
            ACMP2_SAMPLE_SYNC_ENW::ACMP2_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP2_SAMPLE_SYNC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP2_SAMPLE_SYNC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP2_SAMPLE_SYNC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "select XBAR output"]
    #[inline]
    pub fn acmp2_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP2_SAMPLE_SYNC_ENW::ACMP2_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline]
    pub fn acmp2_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP2_SAMPLE_SYNC_ENW::ACMP2_SAMPLE_SYNC_EN_1)
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
#[doc = "Values that can be written to the field `ACMP3_SAMPLE_SYNC_EN`"]
pub enum ACMP3_SAMPLE_SYNC_ENW {
    #[doc = "select XBAR output"]
    ACMP3_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP3_SAMPLE_SYNC_EN_1,
}
impl ACMP3_SAMPLE_SYNC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP3_SAMPLE_SYNC_ENW::ACMP3_SAMPLE_SYNC_EN_0 => false,
            ACMP3_SAMPLE_SYNC_ENW::ACMP3_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP3_SAMPLE_SYNC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP3_SAMPLE_SYNC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP3_SAMPLE_SYNC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "select XBAR output"]
    #[inline]
    pub fn acmp3_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP3_SAMPLE_SYNC_ENW::ACMP3_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline]
    pub fn acmp3_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP3_SAMPLE_SYNC_ENW::ACMP3_SAMPLE_SYNC_EN_1)
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
#[doc = "Values that can be written to the field `ACMP4_SAMPLE_SYNC_EN`"]
pub enum ACMP4_SAMPLE_SYNC_ENW {
    #[doc = "select XBAR output"]
    ACMP4_SAMPLE_SYNC_EN_0,
    #[doc = "select synced sample_lv"]
    ACMP4_SAMPLE_SYNC_EN_1,
}
impl ACMP4_SAMPLE_SYNC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP4_SAMPLE_SYNC_ENW::ACMP4_SAMPLE_SYNC_EN_0 => false,
            ACMP4_SAMPLE_SYNC_ENW::ACMP4_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP4_SAMPLE_SYNC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP4_SAMPLE_SYNC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP4_SAMPLE_SYNC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "select XBAR output"]
    #[inline]
    pub fn acmp4_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP4_SAMPLE_SYNC_ENW::ACMP4_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline]
    pub fn acmp4_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP4_SAMPLE_SYNC_ENW::ACMP4_SAMPLE_SYNC_EN_1)
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
#[doc = "Values that can be written to the field `CM7_MX6RT_CFGITCMSZ`"]
pub enum CM7_MX6RT_CFGITCMSZW {
    #[doc = "0 KB (No ITCM)"]
    CM7_MX6RT_CFGITCMSZ_0,
    #[doc = "4 KB"]
    CM7_MX6RT_CFGITCMSZ_3,
    #[doc = "8 KB"]
    CM7_MX6RT_CFGITCMSZ_4,
    #[doc = "16 KB"]
    CM7_MX6RT_CFGITCMSZ_5,
    #[doc = "32 KB"]
    CM7_MX6RT_CFGITCMSZ_6,
    #[doc = "64 KB"]
    CM7_MX6RT_CFGITCMSZ_7,
    #[doc = "128 KB"]
    CM7_MX6RT_CFGITCMSZ_8,
    #[doc = "256 KB"]
    CM7_MX6RT_CFGITCMSZ_9,
    #[doc = "512 KB"]
    CM7_MX6RT_CFGITCMSZ_10,
}
impl CM7_MX6RT_CFGITCMSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_0 => 0,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_3 => 3,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_4 => 4,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_5 => 5,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_6 => 6,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_7 => 7,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_8 => 8,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_9 => 9,
            CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_10 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM7_MX6RT_CFGITCMSZW<'a> {
    w: &'a mut W,
}
impl<'a> _CM7_MX6RT_CFGITCMSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM7_MX6RT_CFGITCMSZW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 KB (No ITCM)"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_0(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_0)
    }
    #[doc = "4 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_3(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_3)
    }
    #[doc = "8 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_4(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_4)
    }
    #[doc = "16 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_5(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_5)
    }
    #[doc = "32 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_6(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_6)
    }
    #[doc = "64 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_7(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_7)
    }
    #[doc = "128 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_8(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_8)
    }
    #[doc = "256 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_9(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_9)
    }
    #[doc = "512 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz_10(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGITCMSZW::CM7_MX6RT_CFGITCMSZ_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CM7_MX6RT_CFGDTCMSZ`"]
pub enum CM7_MX6RT_CFGDTCMSZW {
    #[doc = "0 KB (No DTCM)"]
    CM7_MX6RT_CFGDTCMSZ_0,
    #[doc = "4 KB"]
    CM7_MX6RT_CFGDTCMSZ_3,
    #[doc = "8 KB"]
    CM7_MX6RT_CFGDTCMSZ_4,
    #[doc = "16 KB"]
    CM7_MX6RT_CFGDTCMSZ_5,
    #[doc = "32 KB"]
    CM7_MX6RT_CFGDTCMSZ_6,
    #[doc = "64 KB"]
    CM7_MX6RT_CFGDTCMSZ_7,
    #[doc = "128 KB"]
    CM7_MX6RT_CFGDTCMSZ_8,
    #[doc = "256 KB"]
    CM7_MX6RT_CFGDTCMSZ_9,
    #[doc = "512 KB"]
    CM7_MX6RT_CFGDTCMSZ_10,
}
impl CM7_MX6RT_CFGDTCMSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_0 => 0,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_3 => 3,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_4 => 4,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_5 => 5,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_6 => 6,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_7 => 7,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_8 => 8,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_9 => 9,
            CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_10 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM7_MX6RT_CFGDTCMSZW<'a> {
    w: &'a mut W,
}
impl<'a> _CM7_MX6RT_CFGDTCMSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM7_MX6RT_CFGDTCMSZW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 KB (No DTCM)"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_0(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_0)
    }
    #[doc = "4 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_3(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_3)
    }
    #[doc = "8 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_4(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_4)
    }
    #[doc = "16 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_5(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_5)
    }
    #[doc = "32 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_6(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_6)
    }
    #[doc = "64 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_7(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_7)
    }
    #[doc = "128 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_8(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_8)
    }
    #[doc = "256 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_9(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_9)
    }
    #[doc = "512 KB"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz_10(self) -> &'a mut W {
        self.variant(CM7_MX6RT_CFGDTCMSZW::CM7_MX6RT_CFGDTCMSZ_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - reduces ACMP1 internal bias current by 30%"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_dn(&self) -> ACMP1_CMP_IGEN_TRIM_DNR {
        ACMP1_CMP_IGEN_TRIM_DNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - reduces ACMP2 internal bias current by 30%"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_dn(&self) -> ACMP2_CMP_IGEN_TRIM_DNR {
        ACMP2_CMP_IGEN_TRIM_DNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - reduces ACMP3 internal bias current by 30%"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_dn(&self) -> ACMP3_CMP_IGEN_TRIM_DNR {
        ACMP3_CMP_IGEN_TRIM_DNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - reduces ACMP4 internal bias current by 30%"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_dn(&self) -> ACMP4_CMP_IGEN_TRIM_DNR {
        ACMP4_CMP_IGEN_TRIM_DNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - increases ACMP1 internal bias current by 30%"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_up(&self) -> ACMP1_CMP_IGEN_TRIM_UPR {
        ACMP1_CMP_IGEN_TRIM_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - increases ACMP2 internal bias current by 30%"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_up(&self) -> ACMP2_CMP_IGEN_TRIM_UPR {
        ACMP2_CMP_IGEN_TRIM_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - increases ACMP3 internal bias current by 30%"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_up(&self) -> ACMP3_CMP_IGEN_TRIM_UPR {
        ACMP3_CMP_IGEN_TRIM_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - increases ACMP4 internal bias current by 30%"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_up(&self) -> ACMP4_CMP_IGEN_TRIM_UPR {
        ACMP4_CMP_IGEN_TRIM_UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ACMP1 sample_lv source select"]
    #[inline]
    pub fn acmp1_sample_sync_en(&self) -> ACMP1_SAMPLE_SYNC_ENR {
        ACMP1_SAMPLE_SYNC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ACMP2 sample_lv source select"]
    #[inline]
    pub fn acmp2_sample_sync_en(&self) -> ACMP2_SAMPLE_SYNC_ENR {
        ACMP2_SAMPLE_SYNC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - ACMP3 sample_lv source select"]
    #[inline]
    pub fn acmp3_sample_sync_en(&self) -> ACMP3_SAMPLE_SYNC_ENR {
        ACMP3_SAMPLE_SYNC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - ACMP4 sample_lv source select"]
    #[inline]
    pub fn acmp4_sample_sync_en(&self) -> ACMP4_SAMPLE_SYNC_ENR {
        ACMP4_SAMPLE_SYNC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - ITCM total size configuration"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz(&self) -> CM7_MX6RT_CFGITCMSZR {
        CM7_MX6RT_CFGITCMSZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - DTCM total size configuration"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz(&self) -> CM7_MX6RT_CFGDTCMSZR {
        CM7_MX6RT_CFGDTCMSZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - reduces ACMP1 internal bias current by 30%"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_dn(&mut self) -> _ACMP1_CMP_IGEN_TRIM_DNW {
        _ACMP1_CMP_IGEN_TRIM_DNW { w: self }
    }
    #[doc = "Bit 1 - reduces ACMP2 internal bias current by 30%"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_dn(&mut self) -> _ACMP2_CMP_IGEN_TRIM_DNW {
        _ACMP2_CMP_IGEN_TRIM_DNW { w: self }
    }
    #[doc = "Bit 2 - reduces ACMP3 internal bias current by 30%"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_dn(&mut self) -> _ACMP3_CMP_IGEN_TRIM_DNW {
        _ACMP3_CMP_IGEN_TRIM_DNW { w: self }
    }
    #[doc = "Bit 3 - reduces ACMP4 internal bias current by 30%"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_dn(&mut self) -> _ACMP4_CMP_IGEN_TRIM_DNW {
        _ACMP4_CMP_IGEN_TRIM_DNW { w: self }
    }
    #[doc = "Bit 4 - increases ACMP1 internal bias current by 30%"]
    #[inline]
    pub fn acmp1_cmp_igen_trim_up(&mut self) -> _ACMP1_CMP_IGEN_TRIM_UPW {
        _ACMP1_CMP_IGEN_TRIM_UPW { w: self }
    }
    #[doc = "Bit 5 - increases ACMP2 internal bias current by 30%"]
    #[inline]
    pub fn acmp2_cmp_igen_trim_up(&mut self) -> _ACMP2_CMP_IGEN_TRIM_UPW {
        _ACMP2_CMP_IGEN_TRIM_UPW { w: self }
    }
    #[doc = "Bit 6 - increases ACMP3 internal bias current by 30%"]
    #[inline]
    pub fn acmp3_cmp_igen_trim_up(&mut self) -> _ACMP3_CMP_IGEN_TRIM_UPW {
        _ACMP3_CMP_IGEN_TRIM_UPW { w: self }
    }
    #[doc = "Bit 7 - increases ACMP4 internal bias current by 30%"]
    #[inline]
    pub fn acmp4_cmp_igen_trim_up(&mut self) -> _ACMP4_CMP_IGEN_TRIM_UPW {
        _ACMP4_CMP_IGEN_TRIM_UPW { w: self }
    }
    #[doc = "Bit 8 - ACMP1 sample_lv source select"]
    #[inline]
    pub fn acmp1_sample_sync_en(&mut self) -> _ACMP1_SAMPLE_SYNC_ENW {
        _ACMP1_SAMPLE_SYNC_ENW { w: self }
    }
    #[doc = "Bit 9 - ACMP2 sample_lv source select"]
    #[inline]
    pub fn acmp2_sample_sync_en(&mut self) -> _ACMP2_SAMPLE_SYNC_ENW {
        _ACMP2_SAMPLE_SYNC_ENW { w: self }
    }
    #[doc = "Bit 10 - ACMP3 sample_lv source select"]
    #[inline]
    pub fn acmp3_sample_sync_en(&mut self) -> _ACMP3_SAMPLE_SYNC_ENW {
        _ACMP3_SAMPLE_SYNC_ENW { w: self }
    }
    #[doc = "Bit 11 - ACMP4 sample_lv source select"]
    #[inline]
    pub fn acmp4_sample_sync_en(&mut self) -> _ACMP4_SAMPLE_SYNC_ENW {
        _ACMP4_SAMPLE_SYNC_ENW { w: self }
    }
    #[doc = "Bits 16:19 - ITCM total size configuration"]
    #[inline]
    pub fn cm7_mx6rt_cfgitcmsz(&mut self) -> _CM7_MX6RT_CFGITCMSZW {
        _CM7_MX6RT_CFGITCMSZW { w: self }
    }
    #[doc = "Bits 20:23 - DTCM total size configuration"]
    #[inline]
    pub fn cm7_mx6rt_cfgdtcmsz(&mut self) -> _CM7_MX6RT_CFGDTCMSZW {
        _CM7_MX6RT_CFGDTCMSZW { w: self }
    }
}
