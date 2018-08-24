#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR16 {
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
#[doc = "Possible values of the field `INIT_ITCM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_ITCM_ENR {
    #[doc = "ITCM is disabled"]
    INIT_ITCM_EN_0,
    #[doc = "ITCM is enabled"]
    INIT_ITCM_EN_1,
}
impl INIT_ITCM_ENR {
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
            INIT_ITCM_ENR::INIT_ITCM_EN_0 => false,
            INIT_ITCM_ENR::INIT_ITCM_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INIT_ITCM_ENR {
        match value {
            false => INIT_ITCM_ENR::INIT_ITCM_EN_0,
            true => INIT_ITCM_ENR::INIT_ITCM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INIT_ITCM_EN_0`"]
    #[inline]
    pub fn is_init_itcm_en_0(&self) -> bool {
        *self == INIT_ITCM_ENR::INIT_ITCM_EN_0
    }
    #[doc = "Checks if the value of the field is `INIT_ITCM_EN_1`"]
    #[inline]
    pub fn is_init_itcm_en_1(&self) -> bool {
        *self == INIT_ITCM_ENR::INIT_ITCM_EN_1
    }
}
#[doc = "Possible values of the field `INIT_DTCM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_DTCM_ENR {
    #[doc = "DTCM is disabled"]
    INIT_DTCM_EN_0,
    #[doc = "DTCM is enabled"]
    INIT_DTCM_EN_1,
}
impl INIT_DTCM_ENR {
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
            INIT_DTCM_ENR::INIT_DTCM_EN_0 => false,
            INIT_DTCM_ENR::INIT_DTCM_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INIT_DTCM_ENR {
        match value {
            false => INIT_DTCM_ENR::INIT_DTCM_EN_0,
            true => INIT_DTCM_ENR::INIT_DTCM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INIT_DTCM_EN_0`"]
    #[inline]
    pub fn is_init_dtcm_en_0(&self) -> bool {
        *self == INIT_DTCM_ENR::INIT_DTCM_EN_0
    }
    #[doc = "Checks if the value of the field is `INIT_DTCM_EN_1`"]
    #[inline]
    pub fn is_init_dtcm_en_1(&self) -> bool {
        *self == INIT_DTCM_ENR::INIT_DTCM_EN_1
    }
}
#[doc = "Possible values of the field `FLEXRAM_BANK_CFG_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXRAM_BANK_CFG_SELR {
    #[doc = "use fuse value to config"]
    FLEXRAM_BANK_CFG_SEL_0,
    #[doc = "use FLEXRAM_BANK_CFG to config"]
    FLEXRAM_BANK_CFG_SEL_1,
}
impl FLEXRAM_BANK_CFG_SELR {
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
            FLEXRAM_BANK_CFG_SELR::FLEXRAM_BANK_CFG_SEL_0 => false,
            FLEXRAM_BANK_CFG_SELR::FLEXRAM_BANK_CFG_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXRAM_BANK_CFG_SELR {
        match value {
            false => FLEXRAM_BANK_CFG_SELR::FLEXRAM_BANK_CFG_SEL_0,
            true => FLEXRAM_BANK_CFG_SELR::FLEXRAM_BANK_CFG_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXRAM_BANK_CFG_SEL_0`"]
    #[inline]
    pub fn is_flexram_bank_cfg_sel_0(&self) -> bool {
        *self == FLEXRAM_BANK_CFG_SELR::FLEXRAM_BANK_CFG_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXRAM_BANK_CFG_SEL_1`"]
    #[inline]
    pub fn is_flexram_bank_cfg_sel_1(&self) -> bool {
        *self == FLEXRAM_BANK_CFG_SELR::FLEXRAM_BANK_CFG_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct CM7_INIT_VTORR {
    bits: u32,
}
impl CM7_INIT_VTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `INIT_ITCM_EN`"]
pub enum INIT_ITCM_ENW {
    #[doc = "ITCM is disabled"]
    INIT_ITCM_EN_0,
    #[doc = "ITCM is enabled"]
    INIT_ITCM_EN_1,
}
impl INIT_ITCM_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INIT_ITCM_ENW::INIT_ITCM_EN_0 => false,
            INIT_ITCM_ENW::INIT_ITCM_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INIT_ITCM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INIT_ITCM_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INIT_ITCM_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ITCM is disabled"]
    #[inline]
    pub fn init_itcm_en_0(self) -> &'a mut W {
        self.variant(INIT_ITCM_ENW::INIT_ITCM_EN_0)
    }
    #[doc = "ITCM is enabled"]
    #[inline]
    pub fn init_itcm_en_1(self) -> &'a mut W {
        self.variant(INIT_ITCM_ENW::INIT_ITCM_EN_1)
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
#[doc = "Values that can be written to the field `INIT_DTCM_EN`"]
pub enum INIT_DTCM_ENW {
    #[doc = "DTCM is disabled"]
    INIT_DTCM_EN_0,
    #[doc = "DTCM is enabled"]
    INIT_DTCM_EN_1,
}
impl INIT_DTCM_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INIT_DTCM_ENW::INIT_DTCM_EN_0 => false,
            INIT_DTCM_ENW::INIT_DTCM_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INIT_DTCM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INIT_DTCM_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INIT_DTCM_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DTCM is disabled"]
    #[inline]
    pub fn init_dtcm_en_0(self) -> &'a mut W {
        self.variant(INIT_DTCM_ENW::INIT_DTCM_EN_0)
    }
    #[doc = "DTCM is enabled"]
    #[inline]
    pub fn init_dtcm_en_1(self) -> &'a mut W {
        self.variant(INIT_DTCM_ENW::INIT_DTCM_EN_1)
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
#[doc = "Values that can be written to the field `FLEXRAM_BANK_CFG_SEL`"]
pub enum FLEXRAM_BANK_CFG_SELW {
    #[doc = "use fuse value to config"]
    FLEXRAM_BANK_CFG_SEL_0,
    #[doc = "use FLEXRAM_BANK_CFG to config"]
    FLEXRAM_BANK_CFG_SEL_1,
}
impl FLEXRAM_BANK_CFG_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXRAM_BANK_CFG_SELW::FLEXRAM_BANK_CFG_SEL_0 => false,
            FLEXRAM_BANK_CFG_SELW::FLEXRAM_BANK_CFG_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXRAM_BANK_CFG_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXRAM_BANK_CFG_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXRAM_BANK_CFG_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "use fuse value to config"]
    #[inline]
    pub fn flexram_bank_cfg_sel_0(self) -> &'a mut W {
        self.variant(FLEXRAM_BANK_CFG_SELW::FLEXRAM_BANK_CFG_SEL_0)
    }
    #[doc = "use FLEXRAM_BANK_CFG to config"]
    #[inline]
    pub fn flexram_bank_cfg_sel_1(self) -> &'a mut W {
        self.variant(FLEXRAM_BANK_CFG_SELW::FLEXRAM_BANK_CFG_SEL_1)
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
#[doc = r" Proxy"]
pub struct _CM7_INIT_VTORW<'a> {
    w: &'a mut W,
}
impl<'a> _CM7_INIT_VTORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 33554431;
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - ITCM enable initialization out of reset"]
    #[inline]
    pub fn init_itcm_en(&self) -> INIT_ITCM_ENR {
        INIT_ITCM_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DTCM enable initialization out of reset"]
    #[inline]
    pub fn init_dtcm_en(&self) -> INIT_DTCM_ENR {
        INIT_DTCM_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FlexRAM bank config source select"]
    #[inline]
    pub fn flexram_bank_cfg_sel(&self) -> FLEXRAM_BANK_CFG_SELR {
        FLEXRAM_BANK_CFG_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:31 - Vector table offset register out of reset"]
    #[inline]
    pub fn cm7_init_vtor(&self) -> CM7_INIT_VTORR {
        let bits = {
            const MASK: u32 = 33554431;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CM7_INIT_VTORR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2097155 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ITCM enable initialization out of reset"]
    #[inline]
    pub fn init_itcm_en(&mut self) -> _INIT_ITCM_ENW {
        _INIT_ITCM_ENW { w: self }
    }
    #[doc = "Bit 1 - DTCM enable initialization out of reset"]
    #[inline]
    pub fn init_dtcm_en(&mut self) -> _INIT_DTCM_ENW {
        _INIT_DTCM_ENW { w: self }
    }
    #[doc = "Bit 2 - FlexRAM bank config source select"]
    #[inline]
    pub fn flexram_bank_cfg_sel(&mut self) -> _FLEXRAM_BANK_CFG_SELW {
        _FLEXRAM_BANK_CFG_SELW { w: self }
    }
    #[doc = "Bits 7:31 - Vector table offset register out of reset"]
    #[inline]
    pub fn cm7_init_vtor(&mut self) -> _CM7_INIT_VTORW {
        _CM7_INIT_VTORW { w: self }
    }
}
