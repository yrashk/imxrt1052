#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMEOR {
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
#[doc = "Possible values of the field `MOD_EN_OV_GPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_GPTR {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_GPT_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_GPT_1,
}
impl MOD_EN_OV_GPTR {
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
            MOD_EN_OV_GPTR::MOD_EN_OV_GPT_0 => false,
            MOD_EN_OV_GPTR::MOD_EN_OV_GPT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOD_EN_OV_GPTR {
        match value {
            false => MOD_EN_OV_GPTR::MOD_EN_OV_GPT_0,
            true => MOD_EN_OV_GPTR::MOD_EN_OV_GPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_GPT_0`"]
    #[inline]
    pub fn is_mod_en_ov_gpt_0(&self) -> bool {
        *self == MOD_EN_OV_GPTR::MOD_EN_OV_GPT_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_GPT_1`"]
    #[inline]
    pub fn is_mod_en_ov_gpt_1(&self) -> bool {
        *self == MOD_EN_OV_GPTR::MOD_EN_OV_GPT_1
    }
}
#[doc = "Possible values of the field `MOD_EN_OV_PIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_PITR {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_PIT_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_PIT_1,
}
impl MOD_EN_OV_PITR {
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
            MOD_EN_OV_PITR::MOD_EN_OV_PIT_0 => false,
            MOD_EN_OV_PITR::MOD_EN_OV_PIT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOD_EN_OV_PITR {
        match value {
            false => MOD_EN_OV_PITR::MOD_EN_OV_PIT_0,
            true => MOD_EN_OV_PITR::MOD_EN_OV_PIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_PIT_0`"]
    #[inline]
    pub fn is_mod_en_ov_pit_0(&self) -> bool {
        *self == MOD_EN_OV_PITR::MOD_EN_OV_PIT_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_PIT_1`"]
    #[inline]
    pub fn is_mod_en_ov_pit_1(&self) -> bool {
        *self == MOD_EN_OV_PITR::MOD_EN_OV_PIT_1
    }
}
#[doc = "Possible values of the field `MOD_EN_USDHC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_USDHCR {
    #[doc = "don't override module enable signal"]
    MOD_EN_USDHC_0,
    #[doc = "override module enable signal"]
    MOD_EN_USDHC_1,
}
impl MOD_EN_USDHCR {
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
            MOD_EN_USDHCR::MOD_EN_USDHC_0 => false,
            MOD_EN_USDHCR::MOD_EN_USDHC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOD_EN_USDHCR {
        match value {
            false => MOD_EN_USDHCR::MOD_EN_USDHC_0,
            true => MOD_EN_USDHCR::MOD_EN_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_USDHC_0`"]
    #[inline]
    pub fn is_mod_en_usdhc_0(&self) -> bool {
        *self == MOD_EN_USDHCR::MOD_EN_USDHC_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_USDHC_1`"]
    #[inline]
    pub fn is_mod_en_usdhc_1(&self) -> bool {
        *self == MOD_EN_USDHCR::MOD_EN_USDHC_1
    }
}
#[doc = "Possible values of the field `MOD_EN_OV_TRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_TRNGR {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_TRNG_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_TRNG_1,
}
impl MOD_EN_OV_TRNGR {
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
            MOD_EN_OV_TRNGR::MOD_EN_OV_TRNG_0 => false,
            MOD_EN_OV_TRNGR::MOD_EN_OV_TRNG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOD_EN_OV_TRNGR {
        match value {
            false => MOD_EN_OV_TRNGR::MOD_EN_OV_TRNG_0,
            true => MOD_EN_OV_TRNGR::MOD_EN_OV_TRNG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_TRNG_0`"]
    #[inline]
    pub fn is_mod_en_ov_trng_0(&self) -> bool {
        *self == MOD_EN_OV_TRNGR::MOD_EN_OV_TRNG_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_TRNG_1`"]
    #[inline]
    pub fn is_mod_en_ov_trng_1(&self) -> bool {
        *self == MOD_EN_OV_TRNGR::MOD_EN_OV_TRNG_1
    }
}
#[doc = "Possible values of the field `MOD_EN_OV_CAN2_CPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_CAN2_CPIR {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_CAN2_CPI_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_CAN2_CPI_1,
}
impl MOD_EN_OV_CAN2_CPIR {
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
            MOD_EN_OV_CAN2_CPIR::MOD_EN_OV_CAN2_CPI_0 => false,
            MOD_EN_OV_CAN2_CPIR::MOD_EN_OV_CAN2_CPI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOD_EN_OV_CAN2_CPIR {
        match value {
            false => MOD_EN_OV_CAN2_CPIR::MOD_EN_OV_CAN2_CPI_0,
            true => MOD_EN_OV_CAN2_CPIR::MOD_EN_OV_CAN2_CPI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN2_CPI_0`"]
    #[inline]
    pub fn is_mod_en_ov_can2_cpi_0(&self) -> bool {
        *self == MOD_EN_OV_CAN2_CPIR::MOD_EN_OV_CAN2_CPI_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN2_CPI_1`"]
    #[inline]
    pub fn is_mod_en_ov_can2_cpi_1(&self) -> bool {
        *self == MOD_EN_OV_CAN2_CPIR::MOD_EN_OV_CAN2_CPI_1
    }
}
#[doc = "Possible values of the field `MOD_EN_OV_CAN1_CPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_CAN1_CPIR {
    #[doc = "don't overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_0,
    #[doc = "overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_1,
}
impl MOD_EN_OV_CAN1_CPIR {
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
            MOD_EN_OV_CAN1_CPIR::MOD_EN_OV_CAN1_CPI_0 => false,
            MOD_EN_OV_CAN1_CPIR::MOD_EN_OV_CAN1_CPI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOD_EN_OV_CAN1_CPIR {
        match value {
            false => MOD_EN_OV_CAN1_CPIR::MOD_EN_OV_CAN1_CPI_0,
            true => MOD_EN_OV_CAN1_CPIR::MOD_EN_OV_CAN1_CPI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN1_CPI_0`"]
    #[inline]
    pub fn is_mod_en_ov_can1_cpi_0(&self) -> bool {
        *self == MOD_EN_OV_CAN1_CPIR::MOD_EN_OV_CAN1_CPI_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN1_CPI_1`"]
    #[inline]
    pub fn is_mod_en_ov_can1_cpi_1(&self) -> bool {
        *self == MOD_EN_OV_CAN1_CPIR::MOD_EN_OV_CAN1_CPI_1
    }
}
#[doc = "Values that can be written to the field `MOD_EN_OV_GPT`"]
pub enum MOD_EN_OV_GPTW {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_GPT_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_GPT_1,
}
impl MOD_EN_OV_GPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOD_EN_OV_GPTW::MOD_EN_OV_GPT_0 => false,
            MOD_EN_OV_GPTW::MOD_EN_OV_GPT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOD_EN_OV_GPTW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_EN_OV_GPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOD_EN_OV_GPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline]
    pub fn mod_en_ov_gpt_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_GPTW::MOD_EN_OV_GPT_0)
    }
    #[doc = "override module enable signal"]
    #[inline]
    pub fn mod_en_ov_gpt_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_GPTW::MOD_EN_OV_GPT_1)
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
#[doc = "Values that can be written to the field `MOD_EN_OV_PIT`"]
pub enum MOD_EN_OV_PITW {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_PIT_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_PIT_1,
}
impl MOD_EN_OV_PITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOD_EN_OV_PITW::MOD_EN_OV_PIT_0 => false,
            MOD_EN_OV_PITW::MOD_EN_OV_PIT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOD_EN_OV_PITW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_EN_OV_PITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOD_EN_OV_PITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline]
    pub fn mod_en_ov_pit_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_PITW::MOD_EN_OV_PIT_0)
    }
    #[doc = "override module enable signal"]
    #[inline]
    pub fn mod_en_ov_pit_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_PITW::MOD_EN_OV_PIT_1)
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
#[doc = "Values that can be written to the field `MOD_EN_USDHC`"]
pub enum MOD_EN_USDHCW {
    #[doc = "don't override module enable signal"]
    MOD_EN_USDHC_0,
    #[doc = "override module enable signal"]
    MOD_EN_USDHC_1,
}
impl MOD_EN_USDHCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOD_EN_USDHCW::MOD_EN_USDHC_0 => false,
            MOD_EN_USDHCW::MOD_EN_USDHC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOD_EN_USDHCW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_EN_USDHCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOD_EN_USDHCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline]
    pub fn mod_en_usdhc_0(self) -> &'a mut W {
        self.variant(MOD_EN_USDHCW::MOD_EN_USDHC_0)
    }
    #[doc = "override module enable signal"]
    #[inline]
    pub fn mod_en_usdhc_1(self) -> &'a mut W {
        self.variant(MOD_EN_USDHCW::MOD_EN_USDHC_1)
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
#[doc = "Values that can be written to the field `MOD_EN_OV_TRNG`"]
pub enum MOD_EN_OV_TRNGW {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_TRNG_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_TRNG_1,
}
impl MOD_EN_OV_TRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOD_EN_OV_TRNGW::MOD_EN_OV_TRNG_0 => false,
            MOD_EN_OV_TRNGW::MOD_EN_OV_TRNG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOD_EN_OV_TRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_EN_OV_TRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOD_EN_OV_TRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline]
    pub fn mod_en_ov_trng_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_TRNGW::MOD_EN_OV_TRNG_0)
    }
    #[doc = "override module enable signal"]
    #[inline]
    pub fn mod_en_ov_trng_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_TRNGW::MOD_EN_OV_TRNG_1)
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
#[doc = "Values that can be written to the field `MOD_EN_OV_CAN2_CPI`"]
pub enum MOD_EN_OV_CAN2_CPIW {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_CAN2_CPI_0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_CAN2_CPI_1,
}
impl MOD_EN_OV_CAN2_CPIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOD_EN_OV_CAN2_CPIW::MOD_EN_OV_CAN2_CPI_0 => false,
            MOD_EN_OV_CAN2_CPIW::MOD_EN_OV_CAN2_CPI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOD_EN_OV_CAN2_CPIW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_EN_OV_CAN2_CPIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOD_EN_OV_CAN2_CPIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline]
    pub fn mod_en_ov_can2_cpi_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN2_CPIW::MOD_EN_OV_CAN2_CPI_0)
    }
    #[doc = "override module enable signal"]
    #[inline]
    pub fn mod_en_ov_can2_cpi_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN2_CPIW::MOD_EN_OV_CAN2_CPI_1)
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
#[doc = "Values that can be written to the field `MOD_EN_OV_CAN1_CPI`"]
pub enum MOD_EN_OV_CAN1_CPIW {
    #[doc = "don't overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_0,
    #[doc = "overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_1,
}
impl MOD_EN_OV_CAN1_CPIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOD_EN_OV_CAN1_CPIW::MOD_EN_OV_CAN1_CPI_0 => false,
            MOD_EN_OV_CAN1_CPIW::MOD_EN_OV_CAN1_CPI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOD_EN_OV_CAN1_CPIW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_EN_OV_CAN1_CPIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOD_EN_OV_CAN1_CPIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't overide module enable signal"]
    #[inline]
    pub fn mod_en_ov_can1_cpi_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN1_CPIW::MOD_EN_OV_CAN1_CPI_0)
    }
    #[doc = "overide module enable signal"]
    #[inline]
    pub fn mod_en_ov_can1_cpi_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN1_CPIW::MOD_EN_OV_CAN1_CPI_1)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 5 - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline]
    pub fn mod_en_ov_gpt(&self) -> MOD_EN_OV_GPTR {
        MOD_EN_OV_GPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline]
    pub fn mod_en_ov_pit(&self) -> MOD_EN_OV_PITR {
        MOD_EN_OV_PITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - overide clock enable signal from USDHC."]
    #[inline]
    pub fn mod_en_usdhc(&self) -> MOD_EN_USDHCR {
        MOD_EN_USDHCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Overide clock enable signal from TRNG"]
    #[inline]
    pub fn mod_en_ov_trng(&self) -> MOD_EN_OV_TRNGR {
        MOD_EN_OV_TRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline]
    pub fn mod_en_ov_can2_cpi(&self) -> MOD_EN_OV_CAN2_CPIR {
        MOD_EN_OV_CAN2_CPIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline]
    pub fn mod_en_ov_can1_cpi(&self) -> MOD_EN_OV_CAN1_CPIR {
        MOD_EN_OV_CAN1_CPIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline]
    pub fn mod_en_ov_gpt(&mut self) -> _MOD_EN_OV_GPTW {
        _MOD_EN_OV_GPTW { w: self }
    }
    #[doc = "Bit 6 - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline]
    pub fn mod_en_ov_pit(&mut self) -> _MOD_EN_OV_PITW {
        _MOD_EN_OV_PITW { w: self }
    }
    #[doc = "Bit 7 - overide clock enable signal from USDHC."]
    #[inline]
    pub fn mod_en_usdhc(&mut self) -> _MOD_EN_USDHCW {
        _MOD_EN_USDHCW { w: self }
    }
    #[doc = "Bit 9 - Overide clock enable signal from TRNG"]
    #[inline]
    pub fn mod_en_ov_trng(&mut self) -> _MOD_EN_OV_TRNGW {
        _MOD_EN_OV_TRNGW { w: self }
    }
    #[doc = "Bit 28 - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline]
    pub fn mod_en_ov_can2_cpi(&mut self) -> _MOD_EN_OV_CAN2_CPIW {
        _MOD_EN_OV_CAN2_CPIW { w: self }
    }
    #[doc = "Bit 30 - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline]
    pub fn mod_en_ov_can1_cpi(&mut self) -> _MOD_EN_OV_CAN1_CPIW {
        _MOD_EN_OV_CAN1_CPIW { w: self }
    }
}
