#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUTOCMD12_ERR_STATUS {
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
#[doc = "Possible values of the field `AC12NE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12NER {
    #[doc = "Executed"]
    AC12NE_0,
    #[doc = "Not executed"]
    AC12NE_1,
}
impl AC12NER {
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
            AC12NER::AC12NE_0 => false,
            AC12NER::AC12NE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12NER {
        match value {
            false => AC12NER::AC12NE_0,
            true => AC12NER::AC12NE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12NE_0`"]
    #[inline]
    pub fn is_ac12ne_0(&self) -> bool {
        *self == AC12NER::AC12NE_0
    }
    #[doc = "Checks if the value of the field is `AC12NE_1`"]
    #[inline]
    pub fn is_ac12ne_1(&self) -> bool {
        *self == AC12NER::AC12NE_1
    }
}
#[doc = "Possible values of the field `AC12TOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12TOER {
    #[doc = "No error"]
    AC12TOE_0,
    #[doc = "Time out"]
    AC12TOE_1,
}
impl AC12TOER {
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
            AC12TOER::AC12TOE_0 => false,
            AC12TOER::AC12TOE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12TOER {
        match value {
            false => AC12TOER::AC12TOE_0,
            true => AC12TOER::AC12TOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12TOE_0`"]
    #[inline]
    pub fn is_ac12toe_0(&self) -> bool {
        *self == AC12TOER::AC12TOE_0
    }
    #[doc = "Checks if the value of the field is `AC12TOE_1`"]
    #[inline]
    pub fn is_ac12toe_1(&self) -> bool {
        *self == AC12TOER::AC12TOE_1
    }
}
#[doc = "Possible values of the field `AC12EBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EBER {
    #[doc = "No error"]
    AC12EBE_0,
    #[doc = "End Bit Error Generated"]
    AC12EBE_1,
}
impl AC12EBER {
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
            AC12EBER::AC12EBE_0 => false,
            AC12EBER::AC12EBE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12EBER {
        match value {
            false => AC12EBER::AC12EBE_0,
            true => AC12EBER::AC12EBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12EBE_0`"]
    #[inline]
    pub fn is_ac12ebe_0(&self) -> bool {
        *self == AC12EBER::AC12EBE_0
    }
    #[doc = "Checks if the value of the field is `AC12EBE_1`"]
    #[inline]
    pub fn is_ac12ebe_1(&self) -> bool {
        *self == AC12EBER::AC12EBE_1
    }
}
#[doc = "Possible values of the field `AC12CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12CER {
    #[doc = "No CRC error"]
    AC12CE_0,
    #[doc = "CRC Error Met in Auto CMD12/23 Response"]
    AC12CE_1,
}
impl AC12CER {
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
            AC12CER::AC12CE_0 => false,
            AC12CER::AC12CE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12CER {
        match value {
            false => AC12CER::AC12CE_0,
            true => AC12CER::AC12CE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12CE_0`"]
    #[inline]
    pub fn is_ac12ce_0(&self) -> bool {
        *self == AC12CER::AC12CE_0
    }
    #[doc = "Checks if the value of the field is `AC12CE_1`"]
    #[inline]
    pub fn is_ac12ce_1(&self) -> bool {
        *self == AC12CER::AC12CE_1
    }
}
#[doc = "Possible values of the field `AC12IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12IER {
    #[doc = "No error"]
    AC12IE_0,
    #[doc = "Error, the CMD index in response is not CMD12/23"]
    AC12IE_1,
}
impl AC12IER {
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
            AC12IER::AC12IE_0 => false,
            AC12IER::AC12IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12IER {
        match value {
            false => AC12IER::AC12IE_0,
            true => AC12IER::AC12IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12IE_0`"]
    #[inline]
    pub fn is_ac12ie_0(&self) -> bool {
        *self == AC12IER::AC12IE_0
    }
    #[doc = "Checks if the value of the field is `AC12IE_1`"]
    #[inline]
    pub fn is_ac12ie_1(&self) -> bool {
        *self == AC12IER::AC12IE_1
    }
}
#[doc = "Possible values of the field `CNIBAC12E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNIBAC12ER {
    #[doc = "No error"]
    CNIBAC12E_0,
    #[doc = "Not Issued"]
    CNIBAC12E_1,
}
impl CNIBAC12ER {
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
            CNIBAC12ER::CNIBAC12E_0 => false,
            CNIBAC12ER::CNIBAC12E_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNIBAC12ER {
        match value {
            false => CNIBAC12ER::CNIBAC12E_0,
            true => CNIBAC12ER::CNIBAC12E_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNIBAC12E_0`"]
    #[inline]
    pub fn is_cnibac12e_0(&self) -> bool {
        *self == CNIBAC12ER::CNIBAC12E_0
    }
    #[doc = "Checks if the value of the field is `CNIBAC12E_1`"]
    #[inline]
    pub fn is_cnibac12e_1(&self) -> bool {
        *self == CNIBAC12ER::CNIBAC12E_1
    }
}
#[doc = r" Value of the field"]
pub struct EXECUTE_TUNINGR {
    bits: bool,
}
impl EXECUTE_TUNINGR {
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
#[doc = "Possible values of the field `SMP_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP_CLK_SELR {
    #[doc = "Fixed clock is used to sample data"]
    SMP_CLK_SEL_0,
    #[doc = "Tuned clock is used to sample data"]
    SMP_CLK_SEL_1,
}
impl SMP_CLK_SELR {
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
            SMP_CLK_SELR::SMP_CLK_SEL_0 => false,
            SMP_CLK_SELR::SMP_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMP_CLK_SELR {
        match value {
            false => SMP_CLK_SELR::SMP_CLK_SEL_0,
            true => SMP_CLK_SELR::SMP_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_0`"]
    #[inline]
    pub fn is_smp_clk_sel_0(&self) -> bool {
        *self == SMP_CLK_SELR::SMP_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_1`"]
    #[inline]
    pub fn is_smp_clk_sel_1(&self) -> bool {
        *self == SMP_CLK_SELR::SMP_CLK_SEL_1
    }
}
#[doc = r" Proxy"]
pub struct _EXECUTE_TUNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _EXECUTE_TUNINGW<'a> {
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
#[doc = "Values that can be written to the field `SMP_CLK_SEL`"]
pub enum SMP_CLK_SELW {
    #[doc = "Fixed clock is used to sample data"]
    SMP_CLK_SEL_0,
    #[doc = "Tuned clock is used to sample data"]
    SMP_CLK_SEL_1,
}
impl SMP_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMP_CLK_SELW::SMP_CLK_SEL_0 => false,
            SMP_CLK_SELW::SMP_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMP_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SMP_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMP_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed clock is used to sample data"]
    #[inline]
    pub fn smp_clk_sel_0(self) -> &'a mut W {
        self.variant(SMP_CLK_SELW::SMP_CLK_SEL_0)
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline]
    pub fn smp_clk_sel_1(self) -> &'a mut W {
        self.variant(SMP_CLK_SELW::SMP_CLK_SEL_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline]
    pub fn ac12ne(&self) -> AC12NER {
        AC12NER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Auto CMD12 / 23 Timeout Error"]
    #[inline]
    pub fn ac12toe(&self) -> AC12TOER {
        AC12TOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Auto CMD12 / 23 End Bit Error"]
    #[inline]
    pub fn ac12ebe(&self) -> AC12EBER {
        AC12EBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Auto CMD12 / 23 CRC Error"]
    #[inline]
    pub fn ac12ce(&self) -> AC12CER {
        AC12CER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Auto CMD12 / 23 Index Error"]
    #[inline]
    pub fn ac12ie(&self) -> AC12IER {
        AC12IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline]
    pub fn cnibac12e(&self) -> CNIBAC12ER {
        CNIBAC12ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline]
    pub fn execute_tuning(&self) -> EXECUTE_TUNINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXECUTE_TUNINGR { bits }
    }
    #[doc = "Bit 23 - Sample Clock Select"]
    #[inline]
    pub fn smp_clk_sel(&self) -> SMP_CLK_SELR {
        SMP_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline]
    pub fn execute_tuning(&mut self) -> _EXECUTE_TUNINGW {
        _EXECUTE_TUNINGW { w: self }
    }
    #[doc = "Bit 23 - Sample Clock Select"]
    #[inline]
    pub fn smp_clk_sel(&mut self) -> _SMP_CLK_SELW {
        _SMP_CLK_SELW { w: self }
    }
}
