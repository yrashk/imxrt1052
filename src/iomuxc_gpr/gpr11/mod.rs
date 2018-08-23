#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR11 {
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
#[doc = "Possible values of the field `M7_APC_AC_R0_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7_APC_AC_R0_CTRLR {
    #[doc = "No access protection"]
    M7_APC_AC_R0_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R0_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R0_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R0_CTRL_3,
}
impl M7_APC_AC_R0_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_0 => 0,
            M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_1 => 1,
            M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_2 => 2,
            M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M7_APC_AC_R0_CTRLR {
        match value {
            0 => M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_0,
            1 => M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_1,
            2 => M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_2,
            3 => M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_0`"]
    #[inline]
    pub fn is_m7_apc_ac_r0_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_1`"]
    #[inline]
    pub fn is_m7_apc_ac_r0_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_2`"]
    #[inline]
    pub fn is_m7_apc_ac_r0_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_3`"]
    #[inline]
    pub fn is_m7_apc_ac_r0_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R0_CTRLR::M7_APC_AC_R0_CTRL_3
    }
}
#[doc = "Possible values of the field `M7_APC_AC_R1_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7_APC_AC_R1_CTRLR {
    #[doc = "No access protection"]
    M7_APC_AC_R1_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R1_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R1_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R1_CTRL_3,
}
impl M7_APC_AC_R1_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_0 => 0,
            M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_1 => 1,
            M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_2 => 2,
            M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M7_APC_AC_R1_CTRLR {
        match value {
            0 => M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_0,
            1 => M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_1,
            2 => M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_2,
            3 => M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_0`"]
    #[inline]
    pub fn is_m7_apc_ac_r1_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_1`"]
    #[inline]
    pub fn is_m7_apc_ac_r1_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_2`"]
    #[inline]
    pub fn is_m7_apc_ac_r1_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_3`"]
    #[inline]
    pub fn is_m7_apc_ac_r1_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R1_CTRLR::M7_APC_AC_R1_CTRL_3
    }
}
#[doc = "Possible values of the field `M7_APC_AC_R2_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7_APC_AC_R2_CTRLR {
    #[doc = "No access protection"]
    M7_APC_AC_R2_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R2_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R2_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R2_CTRL_3,
}
impl M7_APC_AC_R2_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_0 => 0,
            M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_1 => 1,
            M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_2 => 2,
            M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M7_APC_AC_R2_CTRLR {
        match value {
            0 => M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_0,
            1 => M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_1,
            2 => M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_2,
            3 => M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_0`"]
    #[inline]
    pub fn is_m7_apc_ac_r2_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_1`"]
    #[inline]
    pub fn is_m7_apc_ac_r2_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_2`"]
    #[inline]
    pub fn is_m7_apc_ac_r2_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_3`"]
    #[inline]
    pub fn is_m7_apc_ac_r2_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R2_CTRLR::M7_APC_AC_R2_CTRL_3
    }
}
#[doc = "Possible values of the field `M7_APC_AC_R3_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7_APC_AC_R3_CTRLR {
    #[doc = "No access protection"]
    M7_APC_AC_R3_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R3_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R3_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R3_CTRL_3,
}
impl M7_APC_AC_R3_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_0 => 0,
            M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_1 => 1,
            M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_2 => 2,
            M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M7_APC_AC_R3_CTRLR {
        match value {
            0 => M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_0,
            1 => M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_1,
            2 => M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_2,
            3 => M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_0`"]
    #[inline]
    pub fn is_m7_apc_ac_r3_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_1`"]
    #[inline]
    pub fn is_m7_apc_ac_r3_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_2`"]
    #[inline]
    pub fn is_m7_apc_ac_r3_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_3`"]
    #[inline]
    pub fn is_m7_apc_ac_r3_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R3_CTRLR::M7_APC_AC_R3_CTRL_3
    }
}
#[doc = r" Value of the field"]
pub struct BEE_DE_RX_ENR {
    bits: u8,
}
impl BEE_DE_RX_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_M7_APC_AC_R0_CTRLR {
    bits: u8,
}
impl LOCK_M7_APC_AC_R0_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_M7_APC_AC_R1_CTRLR {
    bits: u8,
}
impl LOCK_M7_APC_AC_R1_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_M7_APC_AC_R2_CTRLR {
    bits: u8,
}
impl LOCK_M7_APC_AC_R2_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_M7_APC_AC_R3_CTRLR {
    bits: u8,
}
impl LOCK_M7_APC_AC_R3_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_BEE_DE_RX_ENR {
    bits: u8,
}
impl LOCK_BEE_DE_RX_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `M7_APC_AC_R0_CTRL`"]
pub enum M7_APC_AC_R0_CTRLW {
    #[doc = "No access protection"]
    M7_APC_AC_R0_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R0_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R0_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R0_CTRL_3,
}
impl M7_APC_AC_R0_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_0 => 0,
            M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_1 => 1,
            M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_2 => 2,
            M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7_APC_AC_R0_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _M7_APC_AC_R0_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7_APC_AC_R0_CTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access protection"]
    #[inline]
    pub fn m7_apc_ac_r0_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline]
    pub fn m7_apc_ac_r0_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline]
    pub fn m7_apc_ac_r0_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline]
    pub fn m7_apc_ac_r0_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRLW::M7_APC_AC_R0_CTRL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M7_APC_AC_R1_CTRL`"]
pub enum M7_APC_AC_R1_CTRLW {
    #[doc = "No access protection"]
    M7_APC_AC_R1_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R1_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R1_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R1_CTRL_3,
}
impl M7_APC_AC_R1_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_0 => 0,
            M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_1 => 1,
            M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_2 => 2,
            M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7_APC_AC_R1_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _M7_APC_AC_R1_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7_APC_AC_R1_CTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access protection"]
    #[inline]
    pub fn m7_apc_ac_r1_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline]
    pub fn m7_apc_ac_r1_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline]
    pub fn m7_apc_ac_r1_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline]
    pub fn m7_apc_ac_r1_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRLW::M7_APC_AC_R1_CTRL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M7_APC_AC_R2_CTRL`"]
pub enum M7_APC_AC_R2_CTRLW {
    #[doc = "No access protection"]
    M7_APC_AC_R2_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R2_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R2_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R2_CTRL_3,
}
impl M7_APC_AC_R2_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_0 => 0,
            M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_1 => 1,
            M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_2 => 2,
            M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7_APC_AC_R2_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _M7_APC_AC_R2_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7_APC_AC_R2_CTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access protection"]
    #[inline]
    pub fn m7_apc_ac_r2_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline]
    pub fn m7_apc_ac_r2_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline]
    pub fn m7_apc_ac_r2_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline]
    pub fn m7_apc_ac_r2_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRLW::M7_APC_AC_R2_CTRL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M7_APC_AC_R3_CTRL`"]
pub enum M7_APC_AC_R3_CTRLW {
    #[doc = "No access protection"]
    M7_APC_AC_R3_CTRL_0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R3_CTRL_1,
    #[doc = "FlexSPI access protection"]
    M7_APC_AC_R3_CTRL_2,
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R3_CTRL_3,
}
impl M7_APC_AC_R3_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_0 => 0,
            M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_1 => 1,
            M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_2 => 2,
            M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7_APC_AC_R3_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _M7_APC_AC_R3_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7_APC_AC_R3_CTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access protection"]
    #[inline]
    pub fn m7_apc_ac_r3_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline]
    pub fn m7_apc_ac_r3_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline]
    pub fn m7_apc_ac_r3_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline]
    pub fn m7_apc_ac_r3_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRLW::M7_APC_AC_R3_CTRL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BEE_DE_RX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BEE_DE_RX_ENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_M7_APC_AC_R0_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_M7_APC_AC_R0_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_M7_APC_AC_R1_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_M7_APC_AC_R1_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_M7_APC_AC_R2_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_M7_APC_AC_R2_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_M7_APC_AC_R3_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_M7_APC_AC_R3_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_BEE_DE_RX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_BEE_DE_RX_ENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:1 - Access control of memory region-0"]
    #[inline]
    pub fn m7_apc_ac_r0_ctrl(&self) -> M7_APC_AC_R0_CTRLR {
        M7_APC_AC_R0_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Access control of memory region-1"]
    #[inline]
    pub fn m7_apc_ac_r1_ctrl(&self) -> M7_APC_AC_R1_CTRLR {
        M7_APC_AC_R1_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Access control of memory region-2"]
    #[inline]
    pub fn m7_apc_ac_r2_ctrl(&self) -> M7_APC_AC_R2_CTRLR {
        M7_APC_AC_R2_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Access control of memory region-3"]
    #[inline]
    pub fn m7_apc_ac_r3_ctrl(&self) -> M7_APC_AC_R3_CTRLR {
        M7_APC_AC_R3_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - BEE data decryption of memory region-n (n = 3 to 0)"]
    #[inline]
    pub fn bee_de_rx_en(&self) -> BEE_DE_RX_ENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BEE_DE_RX_ENR { bits }
    }
    #[doc = "Bits 16:17 - Lock M7_APC_AC_R0_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r0_ctrl(&self) -> LOCK_M7_APC_AC_R0_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_M7_APC_AC_R0_CTRLR { bits }
    }
    #[doc = "Bits 18:19 - Lock M7_APC_AC_R1_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r1_ctrl(&self) -> LOCK_M7_APC_AC_R1_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_M7_APC_AC_R1_CTRLR { bits }
    }
    #[doc = "Bits 20:21 - Lock M7_APC_AC_R2_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r2_ctrl(&self) -> LOCK_M7_APC_AC_R2_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_M7_APC_AC_R2_CTRLR { bits }
    }
    #[doc = "Bits 22:23 - Lock M7_APC_AC_R3_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r3_ctrl(&self) -> LOCK_M7_APC_AC_R3_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_M7_APC_AC_R3_CTRLR { bits }
    }
    #[doc = "Bits 24:27 - Lock BEE_DE_RX_EN[n] (n = 3 to 0) field for changes"]
    #[inline]
    pub fn lock_bee_de_rx_en(&self) -> LOCK_BEE_DE_RX_ENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_BEE_DE_RX_ENR { bits }
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
    #[doc = "Bits 0:1 - Access control of memory region-0"]
    #[inline]
    pub fn m7_apc_ac_r0_ctrl(&mut self) -> _M7_APC_AC_R0_CTRLW {
        _M7_APC_AC_R0_CTRLW { w: self }
    }
    #[doc = "Bits 2:3 - Access control of memory region-1"]
    #[inline]
    pub fn m7_apc_ac_r1_ctrl(&mut self) -> _M7_APC_AC_R1_CTRLW {
        _M7_APC_AC_R1_CTRLW { w: self }
    }
    #[doc = "Bits 4:5 - Access control of memory region-2"]
    #[inline]
    pub fn m7_apc_ac_r2_ctrl(&mut self) -> _M7_APC_AC_R2_CTRLW {
        _M7_APC_AC_R2_CTRLW { w: self }
    }
    #[doc = "Bits 6:7 - Access control of memory region-3"]
    #[inline]
    pub fn m7_apc_ac_r3_ctrl(&mut self) -> _M7_APC_AC_R3_CTRLW {
        _M7_APC_AC_R3_CTRLW { w: self }
    }
    #[doc = "Bits 8:11 - BEE data decryption of memory region-n (n = 3 to 0)"]
    #[inline]
    pub fn bee_de_rx_en(&mut self) -> _BEE_DE_RX_ENW {
        _BEE_DE_RX_ENW { w: self }
    }
    #[doc = "Bits 16:17 - Lock M7_APC_AC_R0_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r0_ctrl(&mut self) -> _LOCK_M7_APC_AC_R0_CTRLW {
        _LOCK_M7_APC_AC_R0_CTRLW { w: self }
    }
    #[doc = "Bits 18:19 - Lock M7_APC_AC_R1_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r1_ctrl(&mut self) -> _LOCK_M7_APC_AC_R1_CTRLW {
        _LOCK_M7_APC_AC_R1_CTRLW { w: self }
    }
    #[doc = "Bits 20:21 - Lock M7_APC_AC_R2_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r2_ctrl(&mut self) -> _LOCK_M7_APC_AC_R2_CTRLW {
        _LOCK_M7_APC_AC_R2_CTRLW { w: self }
    }
    #[doc = "Bits 22:23 - Lock M7_APC_AC_R3_CTRL field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r3_ctrl(&mut self) -> _LOCK_M7_APC_AC_R3_CTRLW {
        _LOCK_M7_APC_AC_R3_CTRLW { w: self }
    }
    #[doc = "Bits 24:27 - Lock BEE_DE_RX_EN[n] (n = 3 to 0) field for changes"]
    #[inline]
    pub fn lock_bee_de_rx_en(&mut self) -> _LOCK_BEE_DE_RX_ENW {
        _LOCK_BEE_DE_RX_ENW { w: self }
    }
}
