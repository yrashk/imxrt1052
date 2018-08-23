#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPMKCR {
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
#[doc = "Possible values of the field `MASTER_KEY_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_KEY_SELR {
    #[doc = "Select one time programmable master key."]
    MASTER_KEY_SEL_0,
    #[doc = "no description available"]
    MASTER_KEY_SEL_2,
    #[doc = "no description available"]
    MASTER_KEY_SEL_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASTER_KEY_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASTER_KEY_SELR::MASTER_KEY_SEL_0 => 0,
            MASTER_KEY_SELR::MASTER_KEY_SEL_2 => 2,
            MASTER_KEY_SELR::MASTER_KEY_SEL_3 => 3,
            MASTER_KEY_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASTER_KEY_SELR {
        match value {
            0 => MASTER_KEY_SELR::MASTER_KEY_SEL_0,
            2 => MASTER_KEY_SELR::MASTER_KEY_SEL_2,
            3 => MASTER_KEY_SELR::MASTER_KEY_SEL_3,
            i => MASTER_KEY_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_KEY_SEL_0`"]
    #[inline]
    pub fn is_master_key_sel_0(&self) -> bool {
        *self == MASTER_KEY_SELR::MASTER_KEY_SEL_0
    }
    #[doc = "Checks if the value of the field is `MASTER_KEY_SEL_2`"]
    #[inline]
    pub fn is_master_key_sel_2(&self) -> bool {
        *self == MASTER_KEY_SELR::MASTER_KEY_SEL_2
    }
    #[doc = "Checks if the value of the field is `MASTER_KEY_SEL_3`"]
    #[inline]
    pub fn is_master_key_sel_3(&self) -> bool {
        *self == MASTER_KEY_SELR::MASTER_KEY_SEL_3
    }
}
#[doc = "Possible values of the field `ZMK_HWP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_HWPR {
    #[doc = "ZMK is in the software programming mode."]
    ZMK_HWP_0,
    #[doc = "ZMK is in the hardware programming mode."]
    ZMK_HWP_1,
}
impl ZMK_HWPR {
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
            ZMK_HWPR::ZMK_HWP_0 => false,
            ZMK_HWPR::ZMK_HWP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_HWPR {
        match value {
            false => ZMK_HWPR::ZMK_HWP_0,
            true => ZMK_HWPR::ZMK_HWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_HWP_0`"]
    #[inline]
    pub fn is_zmk_hwp_0(&self) -> bool {
        *self == ZMK_HWPR::ZMK_HWP_0
    }
    #[doc = "Checks if the value of the field is `ZMK_HWP_1`"]
    #[inline]
    pub fn is_zmk_hwp_1(&self) -> bool {
        *self == ZMK_HWPR::ZMK_HWP_1
    }
}
#[doc = "Possible values of the field `ZMK_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_VALR {
    #[doc = "ZMK is not valid."]
    ZMK_VAL_0,
    #[doc = "ZMK is valid."]
    ZMK_VAL_1,
}
impl ZMK_VALR {
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
            ZMK_VALR::ZMK_VAL_0 => false,
            ZMK_VALR::ZMK_VAL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_VALR {
        match value {
            false => ZMK_VALR::ZMK_VAL_0,
            true => ZMK_VALR::ZMK_VAL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_VAL_0`"]
    #[inline]
    pub fn is_zmk_val_0(&self) -> bool {
        *self == ZMK_VALR::ZMK_VAL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_VAL_1`"]
    #[inline]
    pub fn is_zmk_val_1(&self) -> bool {
        *self == ZMK_VALR::ZMK_VAL_1
    }
}
#[doc = "Possible values of the field `ZMK_ECC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_ECC_ENR {
    #[doc = "ZMK ECC check is disabled."]
    ZMK_ECC_EN_0,
    #[doc = "ZMK ECC check is enabled."]
    ZMK_ECC_EN_1,
}
impl ZMK_ECC_ENR {
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
            ZMK_ECC_ENR::ZMK_ECC_EN_0 => false,
            ZMK_ECC_ENR::ZMK_ECC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_ECC_ENR {
        match value {
            false => ZMK_ECC_ENR::ZMK_ECC_EN_0,
            true => ZMK_ECC_ENR::ZMK_ECC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_EN_0`"]
    #[inline]
    pub fn is_zmk_ecc_en_0(&self) -> bool {
        *self == ZMK_ECC_ENR::ZMK_ECC_EN_0
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_EN_1`"]
    #[inline]
    pub fn is_zmk_ecc_en_1(&self) -> bool {
        *self == ZMK_ECC_ENR::ZMK_ECC_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct ZMK_ECC_VALUER {
    bits: u16,
}
impl ZMK_ECC_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MASTER_KEY_SEL`"]
pub enum MASTER_KEY_SELW {
    #[doc = "Select one time programmable master key."]
    MASTER_KEY_SEL_0,
    #[doc = "no description available"]
    MASTER_KEY_SEL_2,
    #[doc = "no description available"]
    MASTER_KEY_SEL_3,
}
impl MASTER_KEY_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASTER_KEY_SELW::MASTER_KEY_SEL_0 => 0,
            MASTER_KEY_SELW::MASTER_KEY_SEL_2 => 2,
            MASTER_KEY_SELW::MASTER_KEY_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASTER_KEY_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTER_KEY_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASTER_KEY_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select one time programmable master key."]
    #[inline]
    pub fn master_key_sel_0(self) -> &'a mut W {
        self.variant(MASTER_KEY_SELW::MASTER_KEY_SEL_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn master_key_sel_2(self) -> &'a mut W {
        self.variant(MASTER_KEY_SELW::MASTER_KEY_SEL_2)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn master_key_sel_3(self) -> &'a mut W {
        self.variant(MASTER_KEY_SELW::MASTER_KEY_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ZMK_HWP`"]
pub enum ZMK_HWPW {
    #[doc = "ZMK is in the software programming mode."]
    ZMK_HWP_0,
    #[doc = "ZMK is in the hardware programming mode."]
    ZMK_HWP_1,
}
impl ZMK_HWPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_HWPW::ZMK_HWP_0 => false,
            ZMK_HWPW::ZMK_HWP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_HWPW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_HWPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_HWPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ZMK is in the software programming mode."]
    #[inline]
    pub fn zmk_hwp_0(self) -> &'a mut W {
        self.variant(ZMK_HWPW::ZMK_HWP_0)
    }
    #[doc = "ZMK is in the hardware programming mode."]
    #[inline]
    pub fn zmk_hwp_1(self) -> &'a mut W {
        self.variant(ZMK_HWPW::ZMK_HWP_1)
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
#[doc = "Values that can be written to the field `ZMK_VAL`"]
pub enum ZMK_VALW {
    #[doc = "ZMK is not valid."]
    ZMK_VAL_0,
    #[doc = "ZMK is valid."]
    ZMK_VAL_1,
}
impl ZMK_VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_VALW::ZMK_VAL_0 => false,
            ZMK_VALW::ZMK_VAL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_VALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ZMK is not valid."]
    #[inline]
    pub fn zmk_val_0(self) -> &'a mut W {
        self.variant(ZMK_VALW::ZMK_VAL_0)
    }
    #[doc = "ZMK is valid."]
    #[inline]
    pub fn zmk_val_1(self) -> &'a mut W {
        self.variant(ZMK_VALW::ZMK_VAL_1)
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
#[doc = "Values that can be written to the field `ZMK_ECC_EN`"]
pub enum ZMK_ECC_ENW {
    #[doc = "ZMK ECC check is disabled."]
    ZMK_ECC_EN_0,
    #[doc = "ZMK ECC check is enabled."]
    ZMK_ECC_EN_1,
}
impl ZMK_ECC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZMK_ECC_ENW::ZMK_ECC_EN_0 => false,
            ZMK_ECC_ENW::ZMK_ECC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZMK_ECC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ZMK_ECC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZMK_ECC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ZMK ECC check is disabled."]
    #[inline]
    pub fn zmk_ecc_en_0(self) -> &'a mut W {
        self.variant(ZMK_ECC_ENW::ZMK_ECC_EN_0)
    }
    #[doc = "ZMK ECC check is enabled."]
    #[inline]
    pub fn zmk_ecc_en_1(self) -> &'a mut W {
        self.variant(ZMK_ECC_ENW::ZMK_ECC_EN_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline]
    pub fn master_key_sel(&self) -> MASTER_KEY_SELR {
        MASTER_KEY_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline]
    pub fn zmk_hwp(&self) -> ZMK_HWPR {
        ZMK_HWPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline]
    pub fn zmk_val(&self) -> ZMK_VALR {
        ZMK_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline]
    pub fn zmk_ecc_en(&self) -> ZMK_ECC_ENR {
        ZMK_ECC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:15 - Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    #[inline]
    pub fn zmk_ecc_value(&self) -> ZMK_ECC_VALUER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ZMK_ECC_VALUER { bits }
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
    #[doc = "Bits 0:1 - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline]
    pub fn master_key_sel(&mut self) -> _MASTER_KEY_SELW {
        _MASTER_KEY_SELW { w: self }
    }
    #[doc = "Bit 2 - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline]
    pub fn zmk_hwp(&mut self) -> _ZMK_HWPW {
        _ZMK_HWPW { w: self }
    }
    #[doc = "Bit 3 - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline]
    pub fn zmk_val(&mut self) -> _ZMK_VALW {
        _ZMK_VALW { w: self }
    }
    #[doc = "Bit 4 - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline]
    pub fn zmk_ecc_en(&mut self) -> _ZMK_ECC_ENW {
        _ZMK_ECC_ENW { w: self }
    }
}
