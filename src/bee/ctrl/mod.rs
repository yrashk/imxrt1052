#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `BEE_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEE_ENABLER {
    #[doc = "Disable BEE"]
    BEE_ENABLE_0,
    #[doc = "Enable BEE"]
    BEE_ENABLE_1,
}
impl BEE_ENABLER {
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
            BEE_ENABLER::BEE_ENABLE_0 => false,
            BEE_ENABLER::BEE_ENABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEE_ENABLER {
        match value {
            false => BEE_ENABLER::BEE_ENABLE_0,
            true => BEE_ENABLER::BEE_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEE_ENABLE_0`"]
    #[inline]
    pub fn is_bee_enable_0(&self) -> bool {
        *self == BEE_ENABLER::BEE_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `BEE_ENABLE_1`"]
    #[inline]
    pub fn is_bee_enable_1(&self) -> bool {
        *self == BEE_ENABLER::BEE_ENABLE_1
    }
}
#[doc = r" Value of the field"]
pub struct CTRL_CLK_ENR {
    bits: bool,
}
impl CTRL_CLK_ENR {
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
#[doc = r" Value of the field"]
pub struct CTRL_SFTRST_NR {
    bits: bool,
}
impl CTRL_SFTRST_NR {
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
#[doc = r" Value of the field"]
pub struct KEY_VALIDR {
    bits: bool,
}
impl KEY_VALIDR {
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
#[doc = "Possible values of the field `KEY_REGION_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_REGION_SELR {
    #[doc = "Load AES key for region0"]
    KEY_REGION_SEL_0,
    #[doc = "Load AES key for region1"]
    KEY_REGION_SEL_1,
}
impl KEY_REGION_SELR {
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
            KEY_REGION_SELR::KEY_REGION_SEL_0 => false,
            KEY_REGION_SELR::KEY_REGION_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEY_REGION_SELR {
        match value {
            false => KEY_REGION_SELR::KEY_REGION_SEL_0,
            true => KEY_REGION_SELR::KEY_REGION_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `KEY_REGION_SEL_0`"]
    #[inline]
    pub fn is_key_region_sel_0(&self) -> bool {
        *self == KEY_REGION_SELR::KEY_REGION_SEL_0
    }
    #[doc = "Checks if the value of the field is `KEY_REGION_SEL_1`"]
    #[inline]
    pub fn is_key_region_sel_1(&self) -> bool {
        *self == KEY_REGION_SELR::KEY_REGION_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct AC_PROT_ENR {
    bits: bool,
}
impl AC_PROT_ENR {
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
#[doc = "Possible values of the field `LITTLE_ENDIAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LITTLE_ENDIANR {
    #[doc = "The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    LITTLE_ENDIAN_0,
    #[doc = "The input and output data of AES core is not swapped."]
    LITTLE_ENDIAN_1,
}
impl LITTLE_ENDIANR {
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
            LITTLE_ENDIANR::LITTLE_ENDIAN_0 => false,
            LITTLE_ENDIANR::LITTLE_ENDIAN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LITTLE_ENDIANR {
        match value {
            false => LITTLE_ENDIANR::LITTLE_ENDIAN_0,
            true => LITTLE_ENDIANR::LITTLE_ENDIAN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_0`"]
    #[inline]
    pub fn is_little_endian_0(&self) -> bool {
        *self == LITTLE_ENDIANR::LITTLE_ENDIAN_0
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_1`"]
    #[inline]
    pub fn is_little_endian_1(&self) -> bool {
        *self == LITTLE_ENDIANR::LITTLE_ENDIAN_1
    }
}
#[doc = r" Value of the field"]
pub struct SECURITY_LEVEL_R0R {
    bits: u8,
}
impl SECURITY_LEVEL_R0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CTRL_AES_MODE_R0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_AES_MODE_R0R {
    #[doc = "ECB"]
    CTRL_AES_MODE_R0_0,
    #[doc = "CTR"]
    CTRL_AES_MODE_R0_1,
}
impl CTRL_AES_MODE_R0R {
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
            CTRL_AES_MODE_R0R::CTRL_AES_MODE_R0_0 => false,
            CTRL_AES_MODE_R0R::CTRL_AES_MODE_R0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRL_AES_MODE_R0R {
        match value {
            false => CTRL_AES_MODE_R0R::CTRL_AES_MODE_R0_0,
            true => CTRL_AES_MODE_R0R::CTRL_AES_MODE_R0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R0_0`"]
    #[inline]
    pub fn is_ctrl_aes_mode_r0_0(&self) -> bool {
        *self == CTRL_AES_MODE_R0R::CTRL_AES_MODE_R0_0
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R0_1`"]
    #[inline]
    pub fn is_ctrl_aes_mode_r0_1(&self) -> bool {
        *self == CTRL_AES_MODE_R0R::CTRL_AES_MODE_R0_1
    }
}
#[doc = r" Value of the field"]
pub struct SECURITY_LEVEL_R1R {
    bits: u8,
}
impl SECURITY_LEVEL_R1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CTRL_AES_MODE_R1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_AES_MODE_R1R {
    #[doc = "ECB"]
    CTRL_AES_MODE_R1_0,
    #[doc = "CTR"]
    CTRL_AES_MODE_R1_1,
}
impl CTRL_AES_MODE_R1R {
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
            CTRL_AES_MODE_R1R::CTRL_AES_MODE_R1_0 => false,
            CTRL_AES_MODE_R1R::CTRL_AES_MODE_R1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRL_AES_MODE_R1R {
        match value {
            false => CTRL_AES_MODE_R1R::CTRL_AES_MODE_R1_0,
            true => CTRL_AES_MODE_R1R::CTRL_AES_MODE_R1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R1_0`"]
    #[inline]
    pub fn is_ctrl_aes_mode_r1_0(&self) -> bool {
        *self == CTRL_AES_MODE_R1R::CTRL_AES_MODE_R1_0
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R1_1`"]
    #[inline]
    pub fn is_ctrl_aes_mode_r1_1(&self) -> bool {
        *self == CTRL_AES_MODE_R1R::CTRL_AES_MODE_R1_1
    }
}
#[doc = r" Value of the field"]
pub struct BEE_ENABLE_LOCKR {
    bits: bool,
}
impl BEE_ENABLE_LOCKR {
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
#[doc = r" Value of the field"]
pub struct CTRL_CLK_EN_LOCKR {
    bits: bool,
}
impl CTRL_CLK_EN_LOCKR {
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
#[doc = r" Value of the field"]
pub struct CTRL_SFTRST_N_LOCKR {
    bits: bool,
}
impl CTRL_SFTRST_N_LOCKR {
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
#[doc = r" Value of the field"]
pub struct REGION1_ADDR_LOCKR {
    bits: bool,
}
impl REGION1_ADDR_LOCKR {
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
#[doc = r" Value of the field"]
pub struct KEY_VALID_LOCKR {
    bits: bool,
}
impl KEY_VALID_LOCKR {
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
#[doc = r" Value of the field"]
pub struct KEY_REGION_SEL_LOCKR {
    bits: bool,
}
impl KEY_REGION_SEL_LOCKR {
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
#[doc = r" Value of the field"]
pub struct AC_PROT_EN_LOCKR {
    bits: bool,
}
impl AC_PROT_EN_LOCKR {
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
#[doc = r" Value of the field"]
pub struct LITTLE_ENDIAN_LOCKR {
    bits: bool,
}
impl LITTLE_ENDIAN_LOCKR {
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
#[doc = r" Value of the field"]
pub struct SECURITY_LEVEL_R0_LOCKR {
    bits: u8,
}
impl SECURITY_LEVEL_R0_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRL_AES_MODE_R0_LOCKR {
    bits: bool,
}
impl CTRL_AES_MODE_R0_LOCKR {
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
#[doc = r" Value of the field"]
pub struct REGION0_KEY_LOCKR {
    bits: bool,
}
impl REGION0_KEY_LOCKR {
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
#[doc = r" Value of the field"]
pub struct SECURITY_LEVEL_R1_LOCKR {
    bits: u8,
}
impl SECURITY_LEVEL_R1_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRL_AES_MODE_R1_LOCKR {
    bits: bool,
}
impl CTRL_AES_MODE_R1_LOCKR {
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
#[doc = r" Value of the field"]
pub struct REGION1_KEY_LOCKR {
    bits: bool,
}
impl REGION1_KEY_LOCKR {
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
#[doc = "Values that can be written to the field `BEE_ENABLE`"]
pub enum BEE_ENABLEW {
    #[doc = "Disable BEE"]
    BEE_ENABLE_0,
    #[doc = "Enable BEE"]
    BEE_ENABLE_1,
}
impl BEE_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEE_ENABLEW::BEE_ENABLE_0 => false,
            BEE_ENABLEW::BEE_ENABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEE_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BEE_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEE_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable BEE"]
    #[inline]
    pub fn bee_enable_0(self) -> &'a mut W {
        self.variant(BEE_ENABLEW::BEE_ENABLE_0)
    }
    #[doc = "Enable BEE"]
    #[inline]
    pub fn bee_enable_1(self) -> &'a mut W {
        self.variant(BEE_ENABLEW::BEE_ENABLE_1)
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
#[doc = r" Proxy"]
pub struct _CTRL_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_CLK_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTRL_SFTRST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_SFTRST_NW<'a> {
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
#[doc = r" Proxy"]
pub struct _KEY_VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_VALIDW<'a> {
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
#[doc = "Values that can be written to the field `KEY_REGION_SEL`"]
pub enum KEY_REGION_SELW {
    #[doc = "Load AES key for region0"]
    KEY_REGION_SEL_0,
    #[doc = "Load AES key for region1"]
    KEY_REGION_SEL_1,
}
impl KEY_REGION_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KEY_REGION_SELW::KEY_REGION_SEL_0 => false,
            KEY_REGION_SELW::KEY_REGION_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEY_REGION_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_REGION_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEY_REGION_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Load AES key for region0"]
    #[inline]
    pub fn key_region_sel_0(self) -> &'a mut W {
        self.variant(KEY_REGION_SELW::KEY_REGION_SEL_0)
    }
    #[doc = "Load AES key for region1"]
    #[inline]
    pub fn key_region_sel_1(self) -> &'a mut W {
        self.variant(KEY_REGION_SELW::KEY_REGION_SEL_1)
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
#[doc = r" Proxy"]
pub struct _AC_PROT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC_PROT_ENW<'a> {
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
#[doc = "Values that can be written to the field `LITTLE_ENDIAN`"]
pub enum LITTLE_ENDIANW {
    #[doc = "The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    LITTLE_ENDIAN_0,
    #[doc = "The input and output data of AES core is not swapped."]
    LITTLE_ENDIAN_1,
}
impl LITTLE_ENDIANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LITTLE_ENDIANW::LITTLE_ENDIAN_0 => false,
            LITTLE_ENDIANW::LITTLE_ENDIAN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LITTLE_ENDIANW<'a> {
    w: &'a mut W,
}
impl<'a> _LITTLE_ENDIANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LITTLE_ENDIANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    #[inline]
    pub fn little_endian_0(self) -> &'a mut W {
        self.variant(LITTLE_ENDIANW::LITTLE_ENDIAN_0)
    }
    #[doc = "The input and output data of AES core is not swapped."]
    #[inline]
    pub fn little_endian_1(self) -> &'a mut W {
        self.variant(LITTLE_ENDIANW::LITTLE_ENDIAN_1)
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
#[doc = r" Proxy"]
pub struct _SECURITY_LEVEL_R0W<'a> {
    w: &'a mut W,
}
impl<'a> _SECURITY_LEVEL_R0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTRL_AES_MODE_R0`"]
pub enum CTRL_AES_MODE_R0W {
    #[doc = "ECB"]
    CTRL_AES_MODE_R0_0,
    #[doc = "CTR"]
    CTRL_AES_MODE_R0_1,
}
impl CTRL_AES_MODE_R0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTRL_AES_MODE_R0W::CTRL_AES_MODE_R0_0 => false,
            CTRL_AES_MODE_R0W::CTRL_AES_MODE_R0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTRL_AES_MODE_R0W<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_AES_MODE_R0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRL_AES_MODE_R0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ECB"]
    #[inline]
    pub fn ctrl_aes_mode_r0_0(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R0W::CTRL_AES_MODE_R0_0)
    }
    #[doc = "CTR"]
    #[inline]
    pub fn ctrl_aes_mode_r0_1(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R0W::CTRL_AES_MODE_R0_1)
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
#[doc = r" Proxy"]
pub struct _SECURITY_LEVEL_R1W<'a> {
    w: &'a mut W,
}
impl<'a> _SECURITY_LEVEL_R1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTRL_AES_MODE_R1`"]
pub enum CTRL_AES_MODE_R1W {
    #[doc = "ECB"]
    CTRL_AES_MODE_R1_0,
    #[doc = "CTR"]
    CTRL_AES_MODE_R1_1,
}
impl CTRL_AES_MODE_R1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTRL_AES_MODE_R1W::CTRL_AES_MODE_R1_0 => false,
            CTRL_AES_MODE_R1W::CTRL_AES_MODE_R1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTRL_AES_MODE_R1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_AES_MODE_R1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRL_AES_MODE_R1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ECB"]
    #[inline]
    pub fn ctrl_aes_mode_r1_0(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R1W::CTRL_AES_MODE_R1_0)
    }
    #[doc = "CTR"]
    #[inline]
    pub fn ctrl_aes_mode_r1_1(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R1W::CTRL_AES_MODE_R1_1)
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
#[doc = r" Proxy"]
pub struct _BEE_ENABLE_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _BEE_ENABLE_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTRL_CLK_EN_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_CLK_EN_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTRL_SFTRST_N_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_SFTRST_N_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _REGION1_ADDR_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _REGION1_ADDR_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _KEY_VALID_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_VALID_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _KEY_REGION_SEL_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_REGION_SEL_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _AC_PROT_EN_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _AC_PROT_EN_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _LITTLE_ENDIAN_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LITTLE_ENDIAN_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _SECURITY_LEVEL_R0_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SECURITY_LEVEL_R0_LOCKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRL_AES_MODE_R0_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_AES_MODE_R0_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _REGION0_KEY_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _REGION0_KEY_LOCKW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SECURITY_LEVEL_R1_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SECURITY_LEVEL_R1_LOCKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRL_AES_MODE_R1_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_AES_MODE_R1_LOCKW<'a> {
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
#[doc = r" Proxy"]
pub struct _REGION1_KEY_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _REGION1_KEY_LOCKW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - BEE enable bit"]
    #[inline]
    pub fn bee_enable(&self) -> BEE_ENABLER {
        BEE_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock enable input, low inactive"]
    #[inline]
    pub fn ctrl_clk_en(&self) -> CTRL_CLK_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRL_CLK_ENR { bits }
    }
    #[doc = "Bit 2 - Soft reset input, low active"]
    #[inline]
    pub fn ctrl_sftrst_n(&self) -> CTRL_SFTRST_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRL_SFTRST_NR { bits }
    }
    #[doc = "Bit 4 - AES-128 key is ready"]
    #[inline]
    pub fn key_valid(&self) -> KEY_VALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEY_VALIDR { bits }
    }
    #[doc = "Bit 5 - AES key region select"]
    #[inline]
    pub fn key_region_sel(&self) -> KEY_REGION_SELR {
        KEY_REGION_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable access permission control"]
    #[inline]
    pub fn ac_prot_en(&self) -> AC_PROT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC_PROT_ENR { bits }
    }
    #[doc = "Bit 7 - Endian swap control for the 16 bytes input and output data of AES core."]
    #[inline]
    pub fn little_endian(&self) -> LITTLE_ENDIANR {
        LITTLE_ENDIANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Security level of the allowed access for memory region0"]
    #[inline]
    pub fn security_level_r0(&self) -> SECURITY_LEVEL_R0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECURITY_LEVEL_R0R { bits }
    }
    #[doc = "Bit 10 - AES mode of region0"]
    #[inline]
    pub fn ctrl_aes_mode_r0(&self) -> CTRL_AES_MODE_R0R {
        CTRL_AES_MODE_R0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Security level of the allowed access for memory region1"]
    #[inline]
    pub fn security_level_r1(&self) -> SECURITY_LEVEL_R1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECURITY_LEVEL_R1R { bits }
    }
    #[doc = "Bit 14 - AES mode of region1"]
    #[inline]
    pub fn ctrl_aes_mode_r1(&self) -> CTRL_AES_MODE_R1R {
        CTRL_AES_MODE_R1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Lock bit for bee_enable"]
    #[inline]
    pub fn bee_enable_lock(&self) -> BEE_ENABLE_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BEE_ENABLE_LOCKR { bits }
    }
    #[doc = "Bit 17 - Lock bit for ctrl_clk_en"]
    #[inline]
    pub fn ctrl_clk_en_lock(&self) -> CTRL_CLK_EN_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRL_CLK_EN_LOCKR { bits }
    }
    #[doc = "Bit 18 - Lock bit for ctrl_sftrst"]
    #[inline]
    pub fn ctrl_sftrst_n_lock(&self) -> CTRL_SFTRST_N_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRL_SFTRST_N_LOCKR { bits }
    }
    #[doc = "Bit 19 - Lock bit for region1 address boundary"]
    #[inline]
    pub fn region1_addr_lock(&self) -> REGION1_ADDR_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REGION1_ADDR_LOCKR { bits }
    }
    #[doc = "Bit 20 - Lock bit for key_valid"]
    #[inline]
    pub fn key_valid_lock(&self) -> KEY_VALID_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEY_VALID_LOCKR { bits }
    }
    #[doc = "Bit 21 - Lock bit for key_region_sel"]
    #[inline]
    pub fn key_region_sel_lock(&self) -> KEY_REGION_SEL_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEY_REGION_SEL_LOCKR { bits }
    }
    #[doc = "Bit 22 - Lock bit for ac_prot"]
    #[inline]
    pub fn ac_prot_en_lock(&self) -> AC_PROT_EN_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC_PROT_EN_LOCKR { bits }
    }
    #[doc = "Bit 23 - Lock bit for little_endian"]
    #[inline]
    pub fn little_endian_lock(&self) -> LITTLE_ENDIAN_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LITTLE_ENDIAN_LOCKR { bits }
    }
    #[doc = "Bits 24:25 - Lock bits for security_level_r0"]
    #[inline]
    pub fn security_level_r0_lock(&self) -> SECURITY_LEVEL_R0_LOCKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECURITY_LEVEL_R0_LOCKR { bits }
    }
    #[doc = "Bit 26 - Lock bit for region0 ctrl_aes_mode"]
    #[inline]
    pub fn ctrl_aes_mode_r0_lock(&self) -> CTRL_AES_MODE_R0_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRL_AES_MODE_R0_LOCKR { bits }
    }
    #[doc = "Bit 27 - Lock bit for region0 AES key"]
    #[inline]
    pub fn region0_key_lock(&self) -> REGION0_KEY_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REGION0_KEY_LOCKR { bits }
    }
    #[doc = "Bits 28:29 - Lock bits for security_level_r1"]
    #[inline]
    pub fn security_level_r1_lock(&self) -> SECURITY_LEVEL_R1_LOCKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECURITY_LEVEL_R1_LOCKR { bits }
    }
    #[doc = "Bit 30 - Lock bit for region1 ctrl_aes_mode"]
    #[inline]
    pub fn ctrl_aes_mode_r1_lock(&self) -> CTRL_AES_MODE_R1_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRL_AES_MODE_R1_LOCKR { bits }
    }
    #[doc = "Bit 31 - Lock bit for region1 AES key"]
    #[inline]
    pub fn region1_key_lock(&self) -> REGION1_KEY_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REGION1_KEY_LOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 30464 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - BEE enable bit"]
    #[inline]
    pub fn bee_enable(&mut self) -> _BEE_ENABLEW {
        _BEE_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Clock enable input, low inactive"]
    #[inline]
    pub fn ctrl_clk_en(&mut self) -> _CTRL_CLK_ENW {
        _CTRL_CLK_ENW { w: self }
    }
    #[doc = "Bit 2 - Soft reset input, low active"]
    #[inline]
    pub fn ctrl_sftrst_n(&mut self) -> _CTRL_SFTRST_NW {
        _CTRL_SFTRST_NW { w: self }
    }
    #[doc = "Bit 4 - AES-128 key is ready"]
    #[inline]
    pub fn key_valid(&mut self) -> _KEY_VALIDW {
        _KEY_VALIDW { w: self }
    }
    #[doc = "Bit 5 - AES key region select"]
    #[inline]
    pub fn key_region_sel(&mut self) -> _KEY_REGION_SELW {
        _KEY_REGION_SELW { w: self }
    }
    #[doc = "Bit 6 - Enable access permission control"]
    #[inline]
    pub fn ac_prot_en(&mut self) -> _AC_PROT_ENW {
        _AC_PROT_ENW { w: self }
    }
    #[doc = "Bit 7 - Endian swap control for the 16 bytes input and output data of AES core."]
    #[inline]
    pub fn little_endian(&mut self) -> _LITTLE_ENDIANW {
        _LITTLE_ENDIANW { w: self }
    }
    #[doc = "Bits 8:9 - Security level of the allowed access for memory region0"]
    #[inline]
    pub fn security_level_r0(&mut self) -> _SECURITY_LEVEL_R0W {
        _SECURITY_LEVEL_R0W { w: self }
    }
    #[doc = "Bit 10 - AES mode of region0"]
    #[inline]
    pub fn ctrl_aes_mode_r0(&mut self) -> _CTRL_AES_MODE_R0W {
        _CTRL_AES_MODE_R0W { w: self }
    }
    #[doc = "Bits 12:13 - Security level of the allowed access for memory region1"]
    #[inline]
    pub fn security_level_r1(&mut self) -> _SECURITY_LEVEL_R1W {
        _SECURITY_LEVEL_R1W { w: self }
    }
    #[doc = "Bit 14 - AES mode of region1"]
    #[inline]
    pub fn ctrl_aes_mode_r1(&mut self) -> _CTRL_AES_MODE_R1W {
        _CTRL_AES_MODE_R1W { w: self }
    }
    #[doc = "Bit 16 - Lock bit for bee_enable"]
    #[inline]
    pub fn bee_enable_lock(&mut self) -> _BEE_ENABLE_LOCKW {
        _BEE_ENABLE_LOCKW { w: self }
    }
    #[doc = "Bit 17 - Lock bit for ctrl_clk_en"]
    #[inline]
    pub fn ctrl_clk_en_lock(&mut self) -> _CTRL_CLK_EN_LOCKW {
        _CTRL_CLK_EN_LOCKW { w: self }
    }
    #[doc = "Bit 18 - Lock bit for ctrl_sftrst"]
    #[inline]
    pub fn ctrl_sftrst_n_lock(&mut self) -> _CTRL_SFTRST_N_LOCKW {
        _CTRL_SFTRST_N_LOCKW { w: self }
    }
    #[doc = "Bit 19 - Lock bit for region1 address boundary"]
    #[inline]
    pub fn region1_addr_lock(&mut self) -> _REGION1_ADDR_LOCKW {
        _REGION1_ADDR_LOCKW { w: self }
    }
    #[doc = "Bit 20 - Lock bit for key_valid"]
    #[inline]
    pub fn key_valid_lock(&mut self) -> _KEY_VALID_LOCKW {
        _KEY_VALID_LOCKW { w: self }
    }
    #[doc = "Bit 21 - Lock bit for key_region_sel"]
    #[inline]
    pub fn key_region_sel_lock(&mut self) -> _KEY_REGION_SEL_LOCKW {
        _KEY_REGION_SEL_LOCKW { w: self }
    }
    #[doc = "Bit 22 - Lock bit for ac_prot"]
    #[inline]
    pub fn ac_prot_en_lock(&mut self) -> _AC_PROT_EN_LOCKW {
        _AC_PROT_EN_LOCKW { w: self }
    }
    #[doc = "Bit 23 - Lock bit for little_endian"]
    #[inline]
    pub fn little_endian_lock(&mut self) -> _LITTLE_ENDIAN_LOCKW {
        _LITTLE_ENDIAN_LOCKW { w: self }
    }
    #[doc = "Bits 24:25 - Lock bits for security_level_r0"]
    #[inline]
    pub fn security_level_r0_lock(&mut self) -> _SECURITY_LEVEL_R0_LOCKW {
        _SECURITY_LEVEL_R0_LOCKW { w: self }
    }
    #[doc = "Bit 26 - Lock bit for region0 ctrl_aes_mode"]
    #[inline]
    pub fn ctrl_aes_mode_r0_lock(&mut self) -> _CTRL_AES_MODE_R0_LOCKW {
        _CTRL_AES_MODE_R0_LOCKW { w: self }
    }
    #[doc = "Bit 27 - Lock bit for region0 AES key"]
    #[inline]
    pub fn region0_key_lock(&mut self) -> _REGION0_KEY_LOCKW {
        _REGION0_KEY_LOCKW { w: self }
    }
    #[doc = "Bits 28:29 - Lock bits for security_level_r1"]
    #[inline]
    pub fn security_level_r1_lock(&mut self) -> _SECURITY_LEVEL_R1_LOCKW {
        _SECURITY_LEVEL_R1_LOCKW { w: self }
    }
    #[doc = "Bit 30 - Lock bit for region1 ctrl_aes_mode"]
    #[inline]
    pub fn ctrl_aes_mode_r1_lock(&mut self) -> _CTRL_AES_MODE_R1_LOCKW {
        _CTRL_AES_MODE_R1_LOCKW { w: self }
    }
    #[doc = "Bit 31 - Lock bit for region1 AES key"]
    #[inline]
    pub fn region1_key_lock(&mut self) -> _REGION1_KEY_LOCKW {
        _REGION1_KEY_LOCKW { w: self }
    }
}
