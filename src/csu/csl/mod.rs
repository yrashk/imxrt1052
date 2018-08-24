#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSL {
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
#[doc = "Possible values of the field `SUR_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUR_S2R {
    #[doc = "The secure user read access is disabled for the second slave."]
    SUR_S2_0,
    #[doc = "The secure user read access is enabled for the second slave."]
    SUR_S2_1,
}
impl SUR_S2R {
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
            SUR_S2R::SUR_S2_0 => false,
            SUR_S2R::SUR_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUR_S2R {
        match value {
            false => SUR_S2R::SUR_S2_0,
            true => SUR_S2R::SUR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUR_S2_0`"]
    #[inline]
    pub fn is_sur_s2_0(&self) -> bool {
        *self == SUR_S2R::SUR_S2_0
    }
    #[doc = "Checks if the value of the field is `SUR_S2_1`"]
    #[inline]
    pub fn is_sur_s2_1(&self) -> bool {
        *self == SUR_S2R::SUR_S2_1
    }
}
#[doc = "Possible values of the field `SSR_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSR_S2R {
    #[doc = "The secure supervisor read access is disabled for the second slave."]
    SSR_S2_0,
    #[doc = "The secure supervisor read access is enabled for the second slave."]
    SSR_S2_1,
}
impl SSR_S2R {
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
            SSR_S2R::SSR_S2_0 => false,
            SSR_S2R::SSR_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSR_S2R {
        match value {
            false => SSR_S2R::SSR_S2_0,
            true => SSR_S2R::SSR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSR_S2_0`"]
    #[inline]
    pub fn is_ssr_s2_0(&self) -> bool {
        *self == SSR_S2R::SSR_S2_0
    }
    #[doc = "Checks if the value of the field is `SSR_S2_1`"]
    #[inline]
    pub fn is_ssr_s2_1(&self) -> bool {
        *self == SSR_S2R::SSR_S2_1
    }
}
#[doc = "Possible values of the field `NUR_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUR_S2R {
    #[doc = "The non-secure user read access is disabled for the second slave."]
    NUR_S2_0,
    #[doc = "The non-secure user read access is enabled for the second slave."]
    NUR_S2_1,
}
impl NUR_S2R {
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
            NUR_S2R::NUR_S2_0 => false,
            NUR_S2R::NUR_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NUR_S2R {
        match value {
            false => NUR_S2R::NUR_S2_0,
            true => NUR_S2R::NUR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUR_S2_0`"]
    #[inline]
    pub fn is_nur_s2_0(&self) -> bool {
        *self == NUR_S2R::NUR_S2_0
    }
    #[doc = "Checks if the value of the field is `NUR_S2_1`"]
    #[inline]
    pub fn is_nur_s2_1(&self) -> bool {
        *self == NUR_S2R::NUR_S2_1
    }
}
#[doc = "Possible values of the field `NSR_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSR_S2R {
    #[doc = "The non-secure supervisor read access is disabled for the second slave."]
    NSR_S2_0,
    #[doc = "The non-secure supervisor read access is enabled for the second slave."]
    NSR_S2_1,
}
impl NSR_S2R {
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
            NSR_S2R::NSR_S2_0 => false,
            NSR_S2R::NSR_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSR_S2R {
        match value {
            false => NSR_S2R::NSR_S2_0,
            true => NSR_S2R::NSR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSR_S2_0`"]
    #[inline]
    pub fn is_nsr_s2_0(&self) -> bool {
        *self == NSR_S2R::NSR_S2_0
    }
    #[doc = "Checks if the value of the field is `NSR_S2_1`"]
    #[inline]
    pub fn is_nsr_s2_1(&self) -> bool {
        *self == NSR_S2R::NSR_S2_1
    }
}
#[doc = "Possible values of the field `SUW_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUW_S2R {
    #[doc = "The secure user write access is disabled for the second slave."]
    SUW_S2_0,
    #[doc = "The secure user write access is enabled for the second slave."]
    SUW_S2_1,
}
impl SUW_S2R {
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
            SUW_S2R::SUW_S2_0 => false,
            SUW_S2R::SUW_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUW_S2R {
        match value {
            false => SUW_S2R::SUW_S2_0,
            true => SUW_S2R::SUW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUW_S2_0`"]
    #[inline]
    pub fn is_suw_s2_0(&self) -> bool {
        *self == SUW_S2R::SUW_S2_0
    }
    #[doc = "Checks if the value of the field is `SUW_S2_1`"]
    #[inline]
    pub fn is_suw_s2_1(&self) -> bool {
        *self == SUW_S2R::SUW_S2_1
    }
}
#[doc = "Possible values of the field `SSW_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_S2R {
    #[doc = "The secure supervisor write access is disabled for the second slave."]
    SSW_S2_0,
    #[doc = "The secure supervisor write access is enabled for the second slave."]
    SSW_S2_1,
}
impl SSW_S2R {
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
            SSW_S2R::SSW_S2_0 => false,
            SSW_S2R::SSW_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSW_S2R {
        match value {
            false => SSW_S2R::SSW_S2_0,
            true => SSW_S2R::SSW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSW_S2_0`"]
    #[inline]
    pub fn is_ssw_s2_0(&self) -> bool {
        *self == SSW_S2R::SSW_S2_0
    }
    #[doc = "Checks if the value of the field is `SSW_S2_1`"]
    #[inline]
    pub fn is_ssw_s2_1(&self) -> bool {
        *self == SSW_S2R::SSW_S2_1
    }
}
#[doc = "Possible values of the field `NUW_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUW_S2R {
    #[doc = "The non-secure user write access is disabled for the second slave."]
    NUW_S2_0,
    #[doc = "The non-secure user write access is enabled for the second slave."]
    NUW_S2_1,
}
impl NUW_S2R {
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
            NUW_S2R::NUW_S2_0 => false,
            NUW_S2R::NUW_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NUW_S2R {
        match value {
            false => NUW_S2R::NUW_S2_0,
            true => NUW_S2R::NUW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUW_S2_0`"]
    #[inline]
    pub fn is_nuw_s2_0(&self) -> bool {
        *self == NUW_S2R::NUW_S2_0
    }
    #[doc = "Checks if the value of the field is `NUW_S2_1`"]
    #[inline]
    pub fn is_nuw_s2_1(&self) -> bool {
        *self == NUW_S2R::NUW_S2_1
    }
}
#[doc = "Possible values of the field `NSW_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSW_S2R {
    #[doc = "The non-secure supervisor write access is disabled for the second slave."]
    NSW_S2_0,
    #[doc = "The non-secure supervisor write access is enabled for the second slave."]
    NSW_S2_1,
}
impl NSW_S2R {
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
            NSW_S2R::NSW_S2_0 => false,
            NSW_S2R::NSW_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSW_S2R {
        match value {
            false => NSW_S2R::NSW_S2_0,
            true => NSW_S2R::NSW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSW_S2_0`"]
    #[inline]
    pub fn is_nsw_s2_0(&self) -> bool {
        *self == NSW_S2R::NSW_S2_0
    }
    #[doc = "Checks if the value of the field is `NSW_S2_1`"]
    #[inline]
    pub fn is_nsw_s2_1(&self) -> bool {
        *self == NSW_S2R::NSW_S2_1
    }
}
#[doc = "Possible values of the field `LOCK_S2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_S2R {
    #[doc = "Not locked. Bits 7-0 can be written by the software."]
    LOCK_S2_0,
    #[doc = "Bits 7-0 are locked and cannot be written by the software"]
    LOCK_S2_1,
}
impl LOCK_S2R {
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
            LOCK_S2R::LOCK_S2_0 => false,
            LOCK_S2R::LOCK_S2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_S2R {
        match value {
            false => LOCK_S2R::LOCK_S2_0,
            true => LOCK_S2R::LOCK_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_S2_0`"]
    #[inline]
    pub fn is_lock_s2_0(&self) -> bool {
        *self == LOCK_S2R::LOCK_S2_0
    }
    #[doc = "Checks if the value of the field is `LOCK_S2_1`"]
    #[inline]
    pub fn is_lock_s2_1(&self) -> bool {
        *self == LOCK_S2R::LOCK_S2_1
    }
}
#[doc = "Possible values of the field `SUR_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUR_S1R {
    #[doc = "The secure user read access is disabled for the first slave."]
    SUR_S1_0,
    #[doc = "The secure user read access is enabled for the first slave."]
    SUR_S1_1,
}
impl SUR_S1R {
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
            SUR_S1R::SUR_S1_0 => false,
            SUR_S1R::SUR_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUR_S1R {
        match value {
            false => SUR_S1R::SUR_S1_0,
            true => SUR_S1R::SUR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUR_S1_0`"]
    #[inline]
    pub fn is_sur_s1_0(&self) -> bool {
        *self == SUR_S1R::SUR_S1_0
    }
    #[doc = "Checks if the value of the field is `SUR_S1_1`"]
    #[inline]
    pub fn is_sur_s1_1(&self) -> bool {
        *self == SUR_S1R::SUR_S1_1
    }
}
#[doc = "Possible values of the field `SSR_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSR_S1R {
    #[doc = "The secure supervisor read access is disabled for the first slave."]
    SSR_S1_0,
    #[doc = "The secure supervisor read access is enabled for the first slave."]
    SSR_S1_1,
}
impl SSR_S1R {
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
            SSR_S1R::SSR_S1_0 => false,
            SSR_S1R::SSR_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSR_S1R {
        match value {
            false => SSR_S1R::SSR_S1_0,
            true => SSR_S1R::SSR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSR_S1_0`"]
    #[inline]
    pub fn is_ssr_s1_0(&self) -> bool {
        *self == SSR_S1R::SSR_S1_0
    }
    #[doc = "Checks if the value of the field is `SSR_S1_1`"]
    #[inline]
    pub fn is_ssr_s1_1(&self) -> bool {
        *self == SSR_S1R::SSR_S1_1
    }
}
#[doc = "Possible values of the field `NUR_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUR_S1R {
    #[doc = "The non-secure user read access is disabled for the first slave."]
    NUR_S1_0,
    #[doc = "The non-secure user read access is enabled for the first slave."]
    NUR_S1_1,
}
impl NUR_S1R {
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
            NUR_S1R::NUR_S1_0 => false,
            NUR_S1R::NUR_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NUR_S1R {
        match value {
            false => NUR_S1R::NUR_S1_0,
            true => NUR_S1R::NUR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUR_S1_0`"]
    #[inline]
    pub fn is_nur_s1_0(&self) -> bool {
        *self == NUR_S1R::NUR_S1_0
    }
    #[doc = "Checks if the value of the field is `NUR_S1_1`"]
    #[inline]
    pub fn is_nur_s1_1(&self) -> bool {
        *self == NUR_S1R::NUR_S1_1
    }
}
#[doc = "Possible values of the field `NSR_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSR_S1R {
    #[doc = "The non-secure supervisor read access is disabled for the first slave."]
    NSR_S1_0,
    #[doc = "The non-secure supervisor read access is enabled for the first slave."]
    NSR_S1_1,
}
impl NSR_S1R {
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
            NSR_S1R::NSR_S1_0 => false,
            NSR_S1R::NSR_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSR_S1R {
        match value {
            false => NSR_S1R::NSR_S1_0,
            true => NSR_S1R::NSR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSR_S1_0`"]
    #[inline]
    pub fn is_nsr_s1_0(&self) -> bool {
        *self == NSR_S1R::NSR_S1_0
    }
    #[doc = "Checks if the value of the field is `NSR_S1_1`"]
    #[inline]
    pub fn is_nsr_s1_1(&self) -> bool {
        *self == NSR_S1R::NSR_S1_1
    }
}
#[doc = "Possible values of the field `SUW_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUW_S1R {
    #[doc = "The secure user write access is disabled for the first slave."]
    SUW_S1_0,
    #[doc = "The secure user write access is enabled for the first slave."]
    SUW_S1_1,
}
impl SUW_S1R {
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
            SUW_S1R::SUW_S1_0 => false,
            SUW_S1R::SUW_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUW_S1R {
        match value {
            false => SUW_S1R::SUW_S1_0,
            true => SUW_S1R::SUW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUW_S1_0`"]
    #[inline]
    pub fn is_suw_s1_0(&self) -> bool {
        *self == SUW_S1R::SUW_S1_0
    }
    #[doc = "Checks if the value of the field is `SUW_S1_1`"]
    #[inline]
    pub fn is_suw_s1_1(&self) -> bool {
        *self == SUW_S1R::SUW_S1_1
    }
}
#[doc = "Possible values of the field `SSW_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_S1R {
    #[doc = "The secure supervisor write access is disabled for the first slave."]
    SSW_S1_0,
    #[doc = "The secure supervisor write access is enabled for the first slave."]
    SSW_S1_1,
}
impl SSW_S1R {
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
            SSW_S1R::SSW_S1_0 => false,
            SSW_S1R::SSW_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSW_S1R {
        match value {
            false => SSW_S1R::SSW_S1_0,
            true => SSW_S1R::SSW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSW_S1_0`"]
    #[inline]
    pub fn is_ssw_s1_0(&self) -> bool {
        *self == SSW_S1R::SSW_S1_0
    }
    #[doc = "Checks if the value of the field is `SSW_S1_1`"]
    #[inline]
    pub fn is_ssw_s1_1(&self) -> bool {
        *self == SSW_S1R::SSW_S1_1
    }
}
#[doc = "Possible values of the field `NUW_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUW_S1R {
    #[doc = "The non-secure user write access is disabled for the first slave."]
    NUW_S1_0,
    #[doc = "The non-secure user write access is enabled for the first slave."]
    NUW_S1_1,
}
impl NUW_S1R {
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
            NUW_S1R::NUW_S1_0 => false,
            NUW_S1R::NUW_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NUW_S1R {
        match value {
            false => NUW_S1R::NUW_S1_0,
            true => NUW_S1R::NUW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUW_S1_0`"]
    #[inline]
    pub fn is_nuw_s1_0(&self) -> bool {
        *self == NUW_S1R::NUW_S1_0
    }
    #[doc = "Checks if the value of the field is `NUW_S1_1`"]
    #[inline]
    pub fn is_nuw_s1_1(&self) -> bool {
        *self == NUW_S1R::NUW_S1_1
    }
}
#[doc = "Possible values of the field `NSW_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSW_S1R {
    #[doc = "The non-secure supervisor write access is disabled for the first slave."]
    NSW_S1_0,
    #[doc = "The non-secure supervisor write access is enabled for the first slave"]
    NSW_S1_1,
}
impl NSW_S1R {
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
            NSW_S1R::NSW_S1_0 => false,
            NSW_S1R::NSW_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSW_S1R {
        match value {
            false => NSW_S1R::NSW_S1_0,
            true => NSW_S1R::NSW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSW_S1_0`"]
    #[inline]
    pub fn is_nsw_s1_0(&self) -> bool {
        *self == NSW_S1R::NSW_S1_0
    }
    #[doc = "Checks if the value of the field is `NSW_S1_1`"]
    #[inline]
    pub fn is_nsw_s1_1(&self) -> bool {
        *self == NSW_S1R::NSW_S1_1
    }
}
#[doc = "Possible values of the field `LOCK_S1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_S1R {
    #[doc = "Not locked. The bits 16-23 can be written by the software."]
    LOCK_S1_0,
    #[doc = "The bits 16-23 are locked and can't be written by the software."]
    LOCK_S1_1,
}
impl LOCK_S1R {
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
            LOCK_S1R::LOCK_S1_0 => false,
            LOCK_S1R::LOCK_S1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_S1R {
        match value {
            false => LOCK_S1R::LOCK_S1_0,
            true => LOCK_S1R::LOCK_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_S1_0`"]
    #[inline]
    pub fn is_lock_s1_0(&self) -> bool {
        *self == LOCK_S1R::LOCK_S1_0
    }
    #[doc = "Checks if the value of the field is `LOCK_S1_1`"]
    #[inline]
    pub fn is_lock_s1_1(&self) -> bool {
        *self == LOCK_S1R::LOCK_S1_1
    }
}
#[doc = "Values that can be written to the field `SUR_S2`"]
pub enum SUR_S2W {
    #[doc = "The secure user read access is disabled for the second slave."]
    SUR_S2_0,
    #[doc = "The secure user read access is enabled for the second slave."]
    SUR_S2_1,
}
impl SUR_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUR_S2W::SUR_S2_0 => false,
            SUR_S2W::SUR_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUR_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _SUR_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUR_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure user read access is disabled for the second slave."]
    #[inline]
    pub fn sur_s2_0(self) -> &'a mut W {
        self.variant(SUR_S2W::SUR_S2_0)
    }
    #[doc = "The secure user read access is enabled for the second slave."]
    #[inline]
    pub fn sur_s2_1(self) -> &'a mut W {
        self.variant(SUR_S2W::SUR_S2_1)
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
#[doc = "Values that can be written to the field `SSR_S2`"]
pub enum SSR_S2W {
    #[doc = "The secure supervisor read access is disabled for the second slave."]
    SSR_S2_0,
    #[doc = "The secure supervisor read access is enabled for the second slave."]
    SSR_S2_1,
}
impl SSR_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSR_S2W::SSR_S2_0 => false,
            SSR_S2W::SSR_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSR_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _SSR_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSR_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure supervisor read access is disabled for the second slave."]
    #[inline]
    pub fn ssr_s2_0(self) -> &'a mut W {
        self.variant(SSR_S2W::SSR_S2_0)
    }
    #[doc = "The secure supervisor read access is enabled for the second slave."]
    #[inline]
    pub fn ssr_s2_1(self) -> &'a mut W {
        self.variant(SSR_S2W::SSR_S2_1)
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
#[doc = "Values that can be written to the field `NUR_S2`"]
pub enum NUR_S2W {
    #[doc = "The non-secure user read access is disabled for the second slave."]
    NUR_S2_0,
    #[doc = "The non-secure user read access is enabled for the second slave."]
    NUR_S2_1,
}
impl NUR_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NUR_S2W::NUR_S2_0 => false,
            NUR_S2W::NUR_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NUR_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _NUR_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NUR_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure user read access is disabled for the second slave."]
    #[inline]
    pub fn nur_s2_0(self) -> &'a mut W {
        self.variant(NUR_S2W::NUR_S2_0)
    }
    #[doc = "The non-secure user read access is enabled for the second slave."]
    #[inline]
    pub fn nur_s2_1(self) -> &'a mut W {
        self.variant(NUR_S2W::NUR_S2_1)
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
#[doc = "Values that can be written to the field `NSR_S2`"]
pub enum NSR_S2W {
    #[doc = "The non-secure supervisor read access is disabled for the second slave."]
    NSR_S2_0,
    #[doc = "The non-secure supervisor read access is enabled for the second slave."]
    NSR_S2_1,
}
impl NSR_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSR_S2W::NSR_S2_0 => false,
            NSR_S2W::NSR_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSR_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _NSR_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSR_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure supervisor read access is disabled for the second slave."]
    #[inline]
    pub fn nsr_s2_0(self) -> &'a mut W {
        self.variant(NSR_S2W::NSR_S2_0)
    }
    #[doc = "The non-secure supervisor read access is enabled for the second slave."]
    #[inline]
    pub fn nsr_s2_1(self) -> &'a mut W {
        self.variant(NSR_S2W::NSR_S2_1)
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
#[doc = "Values that can be written to the field `SUW_S2`"]
pub enum SUW_S2W {
    #[doc = "The secure user write access is disabled for the second slave."]
    SUW_S2_0,
    #[doc = "The secure user write access is enabled for the second slave."]
    SUW_S2_1,
}
impl SUW_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUW_S2W::SUW_S2_0 => false,
            SUW_S2W::SUW_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUW_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _SUW_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUW_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure user write access is disabled for the second slave."]
    #[inline]
    pub fn suw_s2_0(self) -> &'a mut W {
        self.variant(SUW_S2W::SUW_S2_0)
    }
    #[doc = "The secure user write access is enabled for the second slave."]
    #[inline]
    pub fn suw_s2_1(self) -> &'a mut W {
        self.variant(SUW_S2W::SUW_S2_1)
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
#[doc = "Values that can be written to the field `SSW_S2`"]
pub enum SSW_S2W {
    #[doc = "The secure supervisor write access is disabled for the second slave."]
    SSW_S2_0,
    #[doc = "The secure supervisor write access is enabled for the second slave."]
    SSW_S2_1,
}
impl SSW_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSW_S2W::SSW_S2_0 => false,
            SSW_S2W::SSW_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSW_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _SSW_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSW_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure supervisor write access is disabled for the second slave."]
    #[inline]
    pub fn ssw_s2_0(self) -> &'a mut W {
        self.variant(SSW_S2W::SSW_S2_0)
    }
    #[doc = "The secure supervisor write access is enabled for the second slave."]
    #[inline]
    pub fn ssw_s2_1(self) -> &'a mut W {
        self.variant(SSW_S2W::SSW_S2_1)
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
#[doc = "Values that can be written to the field `NUW_S2`"]
pub enum NUW_S2W {
    #[doc = "The non-secure user write access is disabled for the second slave."]
    NUW_S2_0,
    #[doc = "The non-secure user write access is enabled for the second slave."]
    NUW_S2_1,
}
impl NUW_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NUW_S2W::NUW_S2_0 => false,
            NUW_S2W::NUW_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NUW_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _NUW_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NUW_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure user write access is disabled for the second slave."]
    #[inline]
    pub fn nuw_s2_0(self) -> &'a mut W {
        self.variant(NUW_S2W::NUW_S2_0)
    }
    #[doc = "The non-secure user write access is enabled for the second slave."]
    #[inline]
    pub fn nuw_s2_1(self) -> &'a mut W {
        self.variant(NUW_S2W::NUW_S2_1)
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
#[doc = "Values that can be written to the field `NSW_S2`"]
pub enum NSW_S2W {
    #[doc = "The non-secure supervisor write access is disabled for the second slave."]
    NSW_S2_0,
    #[doc = "The non-secure supervisor write access is enabled for the second slave."]
    NSW_S2_1,
}
impl NSW_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSW_S2W::NSW_S2_0 => false,
            NSW_S2W::NSW_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSW_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _NSW_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSW_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure supervisor write access is disabled for the second slave."]
    #[inline]
    pub fn nsw_s2_0(self) -> &'a mut W {
        self.variant(NSW_S2W::NSW_S2_0)
    }
    #[doc = "The non-secure supervisor write access is enabled for the second slave."]
    #[inline]
    pub fn nsw_s2_1(self) -> &'a mut W {
        self.variant(NSW_S2W::NSW_S2_1)
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
#[doc = "Values that can be written to the field `LOCK_S2`"]
pub enum LOCK_S2W {
    #[doc = "Not locked. Bits 7-0 can be written by the software."]
    LOCK_S2_0,
    #[doc = "Bits 7-0 are locked and cannot be written by the software"]
    LOCK_S2_1,
}
impl LOCK_S2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_S2W::LOCK_S2_0 => false,
            LOCK_S2W::LOCK_S2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_S2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_S2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not locked. Bits 7-0 can be written by the software."]
    #[inline]
    pub fn lock_s2_0(self) -> &'a mut W {
        self.variant(LOCK_S2W::LOCK_S2_0)
    }
    #[doc = "Bits 7-0 are locked and cannot be written by the software"]
    #[inline]
    pub fn lock_s2_1(self) -> &'a mut W {
        self.variant(LOCK_S2W::LOCK_S2_1)
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
#[doc = "Values that can be written to the field `SUR_S1`"]
pub enum SUR_S1W {
    #[doc = "The secure user read access is disabled for the first slave."]
    SUR_S1_0,
    #[doc = "The secure user read access is enabled for the first slave."]
    SUR_S1_1,
}
impl SUR_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUR_S1W::SUR_S1_0 => false,
            SUR_S1W::SUR_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUR_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _SUR_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUR_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure user read access is disabled for the first slave."]
    #[inline]
    pub fn sur_s1_0(self) -> &'a mut W {
        self.variant(SUR_S1W::SUR_S1_0)
    }
    #[doc = "The secure user read access is enabled for the first slave."]
    #[inline]
    pub fn sur_s1_1(self) -> &'a mut W {
        self.variant(SUR_S1W::SUR_S1_1)
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
#[doc = "Values that can be written to the field `SSR_S1`"]
pub enum SSR_S1W {
    #[doc = "The secure supervisor read access is disabled for the first slave."]
    SSR_S1_0,
    #[doc = "The secure supervisor read access is enabled for the first slave."]
    SSR_S1_1,
}
impl SSR_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSR_S1W::SSR_S1_0 => false,
            SSR_S1W::SSR_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSR_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _SSR_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSR_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure supervisor read access is disabled for the first slave."]
    #[inline]
    pub fn ssr_s1_0(self) -> &'a mut W {
        self.variant(SSR_S1W::SSR_S1_0)
    }
    #[doc = "The secure supervisor read access is enabled for the first slave."]
    #[inline]
    pub fn ssr_s1_1(self) -> &'a mut W {
        self.variant(SSR_S1W::SSR_S1_1)
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
#[doc = "Values that can be written to the field `NUR_S1`"]
pub enum NUR_S1W {
    #[doc = "The non-secure user read access is disabled for the first slave."]
    NUR_S1_0,
    #[doc = "The non-secure user read access is enabled for the first slave."]
    NUR_S1_1,
}
impl NUR_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NUR_S1W::NUR_S1_0 => false,
            NUR_S1W::NUR_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NUR_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _NUR_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NUR_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure user read access is disabled for the first slave."]
    #[inline]
    pub fn nur_s1_0(self) -> &'a mut W {
        self.variant(NUR_S1W::NUR_S1_0)
    }
    #[doc = "The non-secure user read access is enabled for the first slave."]
    #[inline]
    pub fn nur_s1_1(self) -> &'a mut W {
        self.variant(NUR_S1W::NUR_S1_1)
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
#[doc = "Values that can be written to the field `NSR_S1`"]
pub enum NSR_S1W {
    #[doc = "The non-secure supervisor read access is disabled for the first slave."]
    NSR_S1_0,
    #[doc = "The non-secure supervisor read access is enabled for the first slave."]
    NSR_S1_1,
}
impl NSR_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSR_S1W::NSR_S1_0 => false,
            NSR_S1W::NSR_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSR_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _NSR_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSR_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure supervisor read access is disabled for the first slave."]
    #[inline]
    pub fn nsr_s1_0(self) -> &'a mut W {
        self.variant(NSR_S1W::NSR_S1_0)
    }
    #[doc = "The non-secure supervisor read access is enabled for the first slave."]
    #[inline]
    pub fn nsr_s1_1(self) -> &'a mut W {
        self.variant(NSR_S1W::NSR_S1_1)
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
#[doc = "Values that can be written to the field `SUW_S1`"]
pub enum SUW_S1W {
    #[doc = "The secure user write access is disabled for the first slave."]
    SUW_S1_0,
    #[doc = "The secure user write access is enabled for the first slave."]
    SUW_S1_1,
}
impl SUW_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUW_S1W::SUW_S1_0 => false,
            SUW_S1W::SUW_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUW_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _SUW_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUW_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure user write access is disabled for the first slave."]
    #[inline]
    pub fn suw_s1_0(self) -> &'a mut W {
        self.variant(SUW_S1W::SUW_S1_0)
    }
    #[doc = "The secure user write access is enabled for the first slave."]
    #[inline]
    pub fn suw_s1_1(self) -> &'a mut W {
        self.variant(SUW_S1W::SUW_S1_1)
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
#[doc = "Values that can be written to the field `SSW_S1`"]
pub enum SSW_S1W {
    #[doc = "The secure supervisor write access is disabled for the first slave."]
    SSW_S1_0,
    #[doc = "The secure supervisor write access is enabled for the first slave."]
    SSW_S1_1,
}
impl SSW_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSW_S1W::SSW_S1_0 => false,
            SSW_S1W::SSW_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSW_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _SSW_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSW_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The secure supervisor write access is disabled for the first slave."]
    #[inline]
    pub fn ssw_s1_0(self) -> &'a mut W {
        self.variant(SSW_S1W::SSW_S1_0)
    }
    #[doc = "The secure supervisor write access is enabled for the first slave."]
    #[inline]
    pub fn ssw_s1_1(self) -> &'a mut W {
        self.variant(SSW_S1W::SSW_S1_1)
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
#[doc = "Values that can be written to the field `NUW_S1`"]
pub enum NUW_S1W {
    #[doc = "The non-secure user write access is disabled for the first slave."]
    NUW_S1_0,
    #[doc = "The non-secure user write access is enabled for the first slave."]
    NUW_S1_1,
}
impl NUW_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NUW_S1W::NUW_S1_0 => false,
            NUW_S1W::NUW_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NUW_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _NUW_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NUW_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure user write access is disabled for the first slave."]
    #[inline]
    pub fn nuw_s1_0(self) -> &'a mut W {
        self.variant(NUW_S1W::NUW_S1_0)
    }
    #[doc = "The non-secure user write access is enabled for the first slave."]
    #[inline]
    pub fn nuw_s1_1(self) -> &'a mut W {
        self.variant(NUW_S1W::NUW_S1_1)
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
#[doc = "Values that can be written to the field `NSW_S1`"]
pub enum NSW_S1W {
    #[doc = "The non-secure supervisor write access is disabled for the first slave."]
    NSW_S1_0,
    #[doc = "The non-secure supervisor write access is enabled for the first slave"]
    NSW_S1_1,
}
impl NSW_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSW_S1W::NSW_S1_0 => false,
            NSW_S1W::NSW_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSW_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _NSW_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSW_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The non-secure supervisor write access is disabled for the first slave."]
    #[inline]
    pub fn nsw_s1_0(self) -> &'a mut W {
        self.variant(NSW_S1W::NSW_S1_0)
    }
    #[doc = "The non-secure supervisor write access is enabled for the first slave"]
    #[inline]
    pub fn nsw_s1_1(self) -> &'a mut W {
        self.variant(NSW_S1W::NSW_S1_1)
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
#[doc = "Values that can be written to the field `LOCK_S1`"]
pub enum LOCK_S1W {
    #[doc = "Not locked. The bits 16-23 can be written by the software."]
    LOCK_S1_0,
    #[doc = "The bits 16-23 are locked and can't be written by the software."]
    LOCK_S1_1,
}
impl LOCK_S1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_S1W::LOCK_S1_0 => false,
            LOCK_S1W::LOCK_S1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_S1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_S1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not locked. The bits 16-23 can be written by the software."]
    #[inline]
    pub fn lock_s1_0(self) -> &'a mut W {
        self.variant(LOCK_S1W::LOCK_S1_0)
    }
    #[doc = "The bits 16-23 are locked and can't be written by the software."]
    #[inline]
    pub fn lock_s1_1(self) -> &'a mut W {
        self.variant(LOCK_S1W::LOCK_S1_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Secure user read access control for the second slave"]
    #[inline]
    pub fn sur_s2(&self) -> SUR_S2R {
        SUR_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Secure supervisor read access control for the second slave"]
    #[inline]
    pub fn ssr_s2(&self) -> SSR_S2R {
        SSR_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Non-secure user read access control for the second slave"]
    #[inline]
    pub fn nur_s2(&self) -> NUR_S2R {
        NUR_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Non-secure supervisor read access control for the second slave"]
    #[inline]
    pub fn nsr_s2(&self) -> NSR_S2R {
        NSR_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Secure user write access control for the second slave"]
    #[inline]
    pub fn suw_s2(&self) -> SUW_S2R {
        SUW_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Secure supervisor write access control for the second slave"]
    #[inline]
    pub fn ssw_s2(&self) -> SSW_S2R {
        SSW_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Non-secure user write access control for the second slave"]
    #[inline]
    pub fn nuw_s2(&self) -> NUW_S2R {
        NUW_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Non-secure supervisor write access control for the second slave"]
    #[inline]
    pub fn nsw_s2(&self) -> NSW_S2R {
        NSW_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline]
    pub fn lock_s2(&self) -> LOCK_S2R {
        LOCK_S2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Secure user read access control for the first slave"]
    #[inline]
    pub fn sur_s1(&self) -> SUR_S1R {
        SUR_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Secure supervisor read access control for the first slave"]
    #[inline]
    pub fn ssr_s1(&self) -> SSR_S1R {
        SSR_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Non-secure user read access control for the first slave"]
    #[inline]
    pub fn nur_s1(&self) -> NUR_S1R {
        NUR_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Non-secure supervisor read access control for the first slave"]
    #[inline]
    pub fn nsr_s1(&self) -> NSR_S1R {
        NSR_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Secure user write access control for the first slave"]
    #[inline]
    pub fn suw_s1(&self) -> SUW_S1R {
        SUW_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Secure supervisor write access control for the first slave"]
    #[inline]
    pub fn ssw_s1(&self) -> SSW_S1R {
        SSW_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Non-secure user write access control for the first slave"]
    #[inline]
    pub fn nuw_s1(&self) -> NUW_S1R {
        NUW_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Non-secure supervisor write access control for the first slave"]
    #[inline]
    pub fn nsw_s1(&self) -> NSW_S1R {
        NSW_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline]
    pub fn lock_s1(&self) -> LOCK_S1R {
        LOCK_S1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3342387 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Secure user read access control for the second slave"]
    #[inline]
    pub fn sur_s2(&mut self) -> _SUR_S2W {
        _SUR_S2W { w: self }
    }
    #[doc = "Bit 1 - Secure supervisor read access control for the second slave"]
    #[inline]
    pub fn ssr_s2(&mut self) -> _SSR_S2W {
        _SSR_S2W { w: self }
    }
    #[doc = "Bit 2 - Non-secure user read access control for the second slave"]
    #[inline]
    pub fn nur_s2(&mut self) -> _NUR_S2W {
        _NUR_S2W { w: self }
    }
    #[doc = "Bit 3 - Non-secure supervisor read access control for the second slave"]
    #[inline]
    pub fn nsr_s2(&mut self) -> _NSR_S2W {
        _NSR_S2W { w: self }
    }
    #[doc = "Bit 4 - Secure user write access control for the second slave"]
    #[inline]
    pub fn suw_s2(&mut self) -> _SUW_S2W {
        _SUW_S2W { w: self }
    }
    #[doc = "Bit 5 - Secure supervisor write access control for the second slave"]
    #[inline]
    pub fn ssw_s2(&mut self) -> _SSW_S2W {
        _SSW_S2W { w: self }
    }
    #[doc = "Bit 6 - Non-secure user write access control for the second slave"]
    #[inline]
    pub fn nuw_s2(&mut self) -> _NUW_S2W {
        _NUW_S2W { w: self }
    }
    #[doc = "Bit 7 - Non-secure supervisor write access control for the second slave"]
    #[inline]
    pub fn nsw_s2(&mut self) -> _NSW_S2W {
        _NSW_S2W { w: self }
    }
    #[doc = "Bit 8 - The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline]
    pub fn lock_s2(&mut self) -> _LOCK_S2W {
        _LOCK_S2W { w: self }
    }
    #[doc = "Bit 16 - Secure user read access control for the first slave"]
    #[inline]
    pub fn sur_s1(&mut self) -> _SUR_S1W {
        _SUR_S1W { w: self }
    }
    #[doc = "Bit 17 - Secure supervisor read access control for the first slave"]
    #[inline]
    pub fn ssr_s1(&mut self) -> _SSR_S1W {
        _SSR_S1W { w: self }
    }
    #[doc = "Bit 18 - Non-secure user read access control for the first slave"]
    #[inline]
    pub fn nur_s1(&mut self) -> _NUR_S1W {
        _NUR_S1W { w: self }
    }
    #[doc = "Bit 19 - Non-secure supervisor read access control for the first slave"]
    #[inline]
    pub fn nsr_s1(&mut self) -> _NSR_S1W {
        _NSR_S1W { w: self }
    }
    #[doc = "Bit 20 - Secure user write access control for the first slave"]
    #[inline]
    pub fn suw_s1(&mut self) -> _SUW_S1W {
        _SUW_S1W { w: self }
    }
    #[doc = "Bit 21 - Secure supervisor write access control for the first slave"]
    #[inline]
    pub fn ssw_s1(&mut self) -> _SSW_S1W {
        _SSW_S1W { w: self }
    }
    #[doc = "Bit 22 - Non-secure user write access control for the first slave"]
    #[inline]
    pub fn nuw_s1(&mut self) -> _NUW_S1W {
        _NUW_S1W { w: self }
    }
    #[doc = "Bit 23 - Non-secure supervisor write access control for the first slave"]
    #[inline]
    pub fn nsw_s1(&mut self) -> _NSW_S1W {
        _NSW_S1W { w: self }
    }
    #[doc = "Bit 24 - The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline]
    pub fn lock_s1(&mut self) -> _LOCK_S1W {
        _LOCK_S1W { w: self }
    }
}
