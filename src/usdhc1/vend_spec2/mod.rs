#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VEND_SPEC2 {
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
#[doc = "Possible values of the field `CARD_INT_D3_TEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_D3_TESTR {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_TEST_0,
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_TEST_1,
}
impl CARD_INT_D3_TESTR {
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
            CARD_INT_D3_TESTR::CARD_INT_D3_TEST_0 => false,
            CARD_INT_D3_TESTR::CARD_INT_D3_TEST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INT_D3_TESTR {
        match value {
            false => CARD_INT_D3_TESTR::CARD_INT_D3_TEST_0,
            true => CARD_INT_D3_TESTR::CARD_INT_D3_TEST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_0`"]
    #[inline]
    pub fn is_card_int_d3_test_0(&self) -> bool {
        *self == CARD_INT_D3_TESTR::CARD_INT_D3_TEST_0
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_1`"]
    #[inline]
    pub fn is_card_int_d3_test_1(&self) -> bool {
        *self == CARD_INT_D3_TESTR::CARD_INT_D3_TEST_1
    }
}
#[doc = r" Value of the field"]
pub struct TUNING_8BIT_ENR {
    bits: bool,
}
impl TUNING_8BIT_ENR {
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
pub struct TUNING_1BIT_ENR {
    bits: bool,
}
impl TUNING_1BIT_ENR {
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
#[doc = "Possible values of the field `TUNING_CMD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TUNING_CMD_ENR {
    #[doc = "Auto tuning circuit does not check the CMD line."]
    TUNING_CMD_EN_0,
    #[doc = "Auto tuning circuit checks the CMD line."]
    TUNING_CMD_EN_1,
}
impl TUNING_CMD_ENR {
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
            TUNING_CMD_ENR::TUNING_CMD_EN_0 => false,
            TUNING_CMD_ENR::TUNING_CMD_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TUNING_CMD_ENR {
        match value {
            false => TUNING_CMD_ENR::TUNING_CMD_EN_0,
            true => TUNING_CMD_ENR::TUNING_CMD_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_0`"]
    #[inline]
    pub fn is_tuning_cmd_en_0(&self) -> bool {
        *self == TUNING_CMD_ENR::TUNING_CMD_EN_0
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_1`"]
    #[inline]
    pub fn is_tuning_cmd_en_1(&self) -> bool {
        *self == TUNING_CMD_ENR::TUNING_CMD_EN_1
    }
}
#[doc = "Possible values of the field `ACMD23_ARGU2_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD23_ARGU2_ENR {
    #[doc = "Disable"]
    ACMD23_ARGU2_EN_0,
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    ACMD23_ARGU2_EN_1,
}
impl ACMD23_ARGU2_ENR {
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
            ACMD23_ARGU2_ENR::ACMD23_ARGU2_EN_0 => false,
            ACMD23_ARGU2_ENR::ACMD23_ARGU2_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMD23_ARGU2_ENR {
        match value {
            false => ACMD23_ARGU2_ENR::ACMD23_ARGU2_EN_0,
            true => ACMD23_ARGU2_ENR::ACMD23_ARGU2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_0`"]
    #[inline]
    pub fn is_acmd23_argu2_en_0(&self) -> bool {
        *self == ACMD23_ARGU2_ENR::ACMD23_ARGU2_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_1`"]
    #[inline]
    pub fn is_acmd23_argu2_en_1(&self) -> bool {
        *self == ACMD23_ARGU2_ENR::ACMD23_ARGU2_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct AHB_RSTR {
    bits: bool,
}
impl AHB_RSTR {
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
#[doc = "Values that can be written to the field `CARD_INT_D3_TEST`"]
pub enum CARD_INT_D3_TESTW {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_TEST_0,
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_TEST_1,
}
impl CARD_INT_D3_TESTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_INT_D3_TESTW::CARD_INT_D3_TEST_0 => false,
            CARD_INT_D3_TESTW::CARD_INT_D3_TEST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_INT_D3_TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_INT_D3_TESTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_INT_D3_TESTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Check the card interrupt only when DATA3 is high."]
    #[inline]
    pub fn card_int_d3_test_0(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TESTW::CARD_INT_D3_TEST_0)
    }
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    #[inline]
    pub fn card_int_d3_test_1(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TESTW::CARD_INT_D3_TEST_1)
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
#[doc = r" Proxy"]
pub struct _TUNING_8BIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_8BIT_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TUNING_1BIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_1BIT_ENW<'a> {
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
#[doc = "Values that can be written to the field `TUNING_CMD_EN`"]
pub enum TUNING_CMD_ENW {
    #[doc = "Auto tuning circuit does not check the CMD line."]
    TUNING_CMD_EN_0,
    #[doc = "Auto tuning circuit checks the CMD line."]
    TUNING_CMD_EN_1,
}
impl TUNING_CMD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TUNING_CMD_ENW::TUNING_CMD_EN_0 => false,
            TUNING_CMD_ENW::TUNING_CMD_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TUNING_CMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_CMD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TUNING_CMD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto tuning circuit does not check the CMD line."]
    #[inline]
    pub fn tuning_cmd_en_0(self) -> &'a mut W {
        self.variant(TUNING_CMD_ENW::TUNING_CMD_EN_0)
    }
    #[doc = "Auto tuning circuit checks the CMD line."]
    #[inline]
    pub fn tuning_cmd_en_1(self) -> &'a mut W {
        self.variant(TUNING_CMD_ENW::TUNING_CMD_EN_1)
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
#[doc = "Values that can be written to the field `ACMD23_ARGU2_EN`"]
pub enum ACMD23_ARGU2_ENW {
    #[doc = "Disable"]
    ACMD23_ARGU2_EN_0,
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    ACMD23_ARGU2_EN_1,
}
impl ACMD23_ARGU2_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMD23_ARGU2_ENW::ACMD23_ARGU2_EN_0 => false,
            ACMD23_ARGU2_ENW::ACMD23_ARGU2_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMD23_ARGU2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMD23_ARGU2_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMD23_ARGU2_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn acmd23_argu2_en_0(self) -> &'a mut W {
        self.variant(ACMD23_ARGU2_ENW::ACMD23_ARGU2_EN_0)
    }
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    #[inline]
    pub fn acmd23_argu2_en_1(self) -> &'a mut W {
        self.variant(ACMD23_ARGU2_ENW::ACMD23_ARGU2_EN_1)
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
pub struct _AHB_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_RSTW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline]
    pub fn card_int_d3_test(&self) -> CARD_INT_D3_TESTR {
        CARD_INT_D3_TESTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - TUNING_8bit_EN"]
    #[inline]
    pub fn tuning_8bit_en(&self) -> TUNING_8BIT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TUNING_8BIT_ENR { bits }
    }
    #[doc = "Bit 5 - TUNING_1bit_EN"]
    #[inline]
    pub fn tuning_1bit_en(&self) -> TUNING_1BIT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TUNING_1BIT_ENR { bits }
    }
    #[doc = "Bit 6 - TUNING_CMD_EN"]
    #[inline]
    pub fn tuning_cmd_en(&self) -> TUNING_CMD_ENR {
        TUNING_CMD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline]
    pub fn acmd23_argu2_en(&self) -> ACMD23_ARGU2_ENR {
        ACMD23_ARGU2_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - AHB BUS reset"]
    #[inline]
    pub fn ahb_rst(&self) -> AHB_RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AHB_RSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4102 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline]
    pub fn card_int_d3_test(&mut self) -> _CARD_INT_D3_TESTW {
        _CARD_INT_D3_TESTW { w: self }
    }
    #[doc = "Bit 4 - TUNING_8bit_EN"]
    #[inline]
    pub fn tuning_8bit_en(&mut self) -> _TUNING_8BIT_ENW {
        _TUNING_8BIT_ENW { w: self }
    }
    #[doc = "Bit 5 - TUNING_1bit_EN"]
    #[inline]
    pub fn tuning_1bit_en(&mut self) -> _TUNING_1BIT_ENW {
        _TUNING_1BIT_ENW { w: self }
    }
    #[doc = "Bit 6 - TUNING_CMD_EN"]
    #[inline]
    pub fn tuning_cmd_en(&mut self) -> _TUNING_CMD_ENW {
        _TUNING_CMD_ENW { w: self }
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline]
    pub fn acmd23_argu2_en(&mut self) -> _ACMD23_ARGU2_ENW {
        _ACMD23_ARGU2_ENW { w: self }
    }
    #[doc = "Bit 14 - AHB BUS reset"]
    #[inline]
    pub fn ahb_rst(&mut self) -> _AHB_RSTW {
        _AHB_RSTW { w: self }
    }
}
