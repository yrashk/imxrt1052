#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STC {
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
#[doc = "Possible values of the field `TxClk_DF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCLK_DFR {
    #[doc = "divider factor is 1"]
    TXCLK_DF_0,
    #[doc = "divider factor is 2"]
    TXCLK_DF_1,
    #[doc = "divider factor is 128"]
    TXCLK_DF_127,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXCLK_DFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXCLK_DFR::TXCLK_DF_0 => 0,
            TXCLK_DFR::TXCLK_DF_1 => 1,
            TXCLK_DFR::TXCLK_DF_127 => 127,
            TXCLK_DFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXCLK_DFR {
        match value {
            0 => TXCLK_DFR::TXCLK_DF_0,
            1 => TXCLK_DFR::TXCLK_DF_1,
            127 => TXCLK_DFR::TXCLK_DF_127,
            i => TXCLK_DFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXCLK_DF_0`"]
    #[inline]
    pub fn is_tx_clk_df_0(&self) -> bool {
        *self == TXCLK_DFR::TXCLK_DF_0
    }
    #[doc = "Checks if the value of the field is `TXCLK_DF_1`"]
    #[inline]
    pub fn is_tx_clk_df_1(&self) -> bool {
        *self == TXCLK_DFR::TXCLK_DF_1
    }
    #[doc = "Checks if the value of the field is `TXCLK_DF_127`"]
    #[inline]
    pub fn is_tx_clk_df_127(&self) -> bool {
        *self == TXCLK_DFR::TXCLK_DF_127
    }
}
#[doc = "Possible values of the field `tx_all_clk_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ALL_CLK_ENR {
    #[doc = "disable transfer clock."]
    TX_ALL_CLK_EN_0,
    #[doc = "enable transfer clock."]
    TX_ALL_CLK_EN_1,
}
impl TX_ALL_CLK_ENR {
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
            TX_ALL_CLK_ENR::TX_ALL_CLK_EN_0 => false,
            TX_ALL_CLK_ENR::TX_ALL_CLK_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_ALL_CLK_ENR {
        match value {
            false => TX_ALL_CLK_ENR::TX_ALL_CLK_EN_0,
            true => TX_ALL_CLK_ENR::TX_ALL_CLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TX_ALL_CLK_EN_0`"]
    #[inline]
    pub fn is_tx_all_clk_en_0(&self) -> bool {
        *self == TX_ALL_CLK_ENR::TX_ALL_CLK_EN_0
    }
    #[doc = "Checks if the value of the field is `TX_ALL_CLK_EN_1`"]
    #[inline]
    pub fn is_tx_all_clk_en_1(&self) -> bool {
        *self == TX_ALL_CLK_ENR::TX_ALL_CLK_EN_1
    }
}
#[doc = "Possible values of the field `TxClk_Source`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCLK_SOURCER {
    #[doc = "REF_CLK_32K input (XTALOSC 32 kHz clock)"]
    TXCLK_SOURCE_0,
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    TXCLK_SOURCE_1,
    #[doc = "SPDIF_EXT_CLK, from pads"]
    TXCLK_SOURCE_3,
    #[doc = "ipg_clk input (frequency divided)"]
    TXCLK_SOURCE_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXCLK_SOURCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXCLK_SOURCER::TXCLK_SOURCE_0 => 0,
            TXCLK_SOURCER::TXCLK_SOURCE_1 => 1,
            TXCLK_SOURCER::TXCLK_SOURCE_3 => 3,
            TXCLK_SOURCER::TXCLK_SOURCE_5 => 5,
            TXCLK_SOURCER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXCLK_SOURCER {
        match value {
            0 => TXCLK_SOURCER::TXCLK_SOURCE_0,
            1 => TXCLK_SOURCER::TXCLK_SOURCE_1,
            3 => TXCLK_SOURCER::TXCLK_SOURCE_3,
            5 => TXCLK_SOURCER::TXCLK_SOURCE_5,
            i => TXCLK_SOURCER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_0`"]
    #[inline]
    pub fn is_tx_clk_source_0(&self) -> bool {
        *self == TXCLK_SOURCER::TXCLK_SOURCE_0
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_1`"]
    #[inline]
    pub fn is_tx_clk_source_1(&self) -> bool {
        *self == TXCLK_SOURCER::TXCLK_SOURCE_1
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_3`"]
    #[inline]
    pub fn is_tx_clk_source_3(&self) -> bool {
        *self == TXCLK_SOURCER::TXCLK_SOURCE_3
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_5`"]
    #[inline]
    pub fn is_tx_clk_source_5(&self) -> bool {
        *self == TXCLK_SOURCER::TXCLK_SOURCE_5
    }
}
#[doc = "Possible values of the field `SYSCLK_DF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCLK_DFR {
    #[doc = "no clock signal"]
    SYSCLK_DF_0,
    #[doc = "divider factor is 2"]
    SYSCLK_DF_1,
    #[doc = "divider factor is 512"]
    SYSCLK_DF_511,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SYSCLK_DFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SYSCLK_DFR::SYSCLK_DF_0 => 0,
            SYSCLK_DFR::SYSCLK_DF_1 => 1,
            SYSCLK_DFR::SYSCLK_DF_511 => 511,
            SYSCLK_DFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SYSCLK_DFR {
        match value {
            0 => SYSCLK_DFR::SYSCLK_DF_0,
            1 => SYSCLK_DFR::SYSCLK_DF_1,
            511 => SYSCLK_DFR::SYSCLK_DF_511,
            i => SYSCLK_DFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_0`"]
    #[inline]
    pub fn is_sysclk_df_0(&self) -> bool {
        *self == SYSCLK_DFR::SYSCLK_DF_0
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_1`"]
    #[inline]
    pub fn is_sysclk_df_1(&self) -> bool {
        *self == SYSCLK_DFR::SYSCLK_DF_1
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_511`"]
    #[inline]
    pub fn is_sysclk_df_511(&self) -> bool {
        *self == SYSCLK_DFR::SYSCLK_DF_511
    }
}
#[doc = "Values that can be written to the field `TxClk_DF`"]
pub enum TXCLK_DFW {
    #[doc = "divider factor is 1"]
    TXCLK_DF_0,
    #[doc = "divider factor is 2"]
    TXCLK_DF_1,
    #[doc = "divider factor is 128"]
    TXCLK_DF_127,
}
impl TXCLK_DFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXCLK_DFW::TXCLK_DF_0 => 0,
            TXCLK_DFW::TXCLK_DF_1 => 1,
            TXCLK_DFW::TXCLK_DF_127 => 127,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCLK_DFW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCLK_DFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCLK_DFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divider factor is 1"]
    #[inline]
    pub fn tx_clk_df_0(self) -> &'a mut W {
        self.variant(TXCLK_DFW::TXCLK_DF_0)
    }
    #[doc = "divider factor is 2"]
    #[inline]
    pub fn tx_clk_df_1(self) -> &'a mut W {
        self.variant(TXCLK_DFW::TXCLK_DF_1)
    }
    #[doc = "divider factor is 128"]
    #[inline]
    pub fn tx_clk_df_127(self) -> &'a mut W {
        self.variant(TXCLK_DFW::TXCLK_DF_127)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `tx_all_clk_en`"]
pub enum TX_ALL_CLK_ENW {
    #[doc = "disable transfer clock."]
    TX_ALL_CLK_EN_0,
    #[doc = "enable transfer clock."]
    TX_ALL_CLK_EN_1,
}
impl TX_ALL_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_ALL_CLK_ENW::TX_ALL_CLK_EN_0 => false,
            TX_ALL_CLK_ENW::TX_ALL_CLK_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_ALL_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_ALL_CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_ALL_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable transfer clock."]
    #[inline]
    pub fn tx_all_clk_en_0(self) -> &'a mut W {
        self.variant(TX_ALL_CLK_ENW::TX_ALL_CLK_EN_0)
    }
    #[doc = "enable transfer clock."]
    #[inline]
    pub fn tx_all_clk_en_1(self) -> &'a mut W {
        self.variant(TX_ALL_CLK_ENW::TX_ALL_CLK_EN_1)
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
#[doc = "Values that can be written to the field `TxClk_Source`"]
pub enum TXCLK_SOURCEW {
    #[doc = "REF_CLK_32K input (XTALOSC 32 kHz clock)"]
    TXCLK_SOURCE_0,
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    TXCLK_SOURCE_1,
    #[doc = "SPDIF_EXT_CLK, from pads"]
    TXCLK_SOURCE_3,
    #[doc = "ipg_clk input (frequency divided)"]
    TXCLK_SOURCE_5,
}
impl TXCLK_SOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXCLK_SOURCEW::TXCLK_SOURCE_0 => 0,
            TXCLK_SOURCEW::TXCLK_SOURCE_1 => 1,
            TXCLK_SOURCEW::TXCLK_SOURCE_3 => 3,
            TXCLK_SOURCEW::TXCLK_SOURCE_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCLK_SOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCLK_SOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCLK_SOURCEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "REF_CLK_32K input (XTALOSC 32 kHz clock)"]
    #[inline]
    pub fn tx_clk_source_0(self) -> &'a mut W {
        self.variant(TXCLK_SOURCEW::TXCLK_SOURCE_0)
    }
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    #[inline]
    pub fn tx_clk_source_1(self) -> &'a mut W {
        self.variant(TXCLK_SOURCEW::TXCLK_SOURCE_1)
    }
    #[doc = "SPDIF_EXT_CLK, from pads"]
    #[inline]
    pub fn tx_clk_source_3(self) -> &'a mut W {
        self.variant(TXCLK_SOURCEW::TXCLK_SOURCE_3)
    }
    #[doc = "ipg_clk input (frequency divided)"]
    #[inline]
    pub fn tx_clk_source_5(self) -> &'a mut W {
        self.variant(TXCLK_SOURCEW::TXCLK_SOURCE_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSCLK_DF`"]
pub enum SYSCLK_DFW {
    #[doc = "no clock signal"]
    SYSCLK_DF_0,
    #[doc = "divider factor is 2"]
    SYSCLK_DF_1,
    #[doc = "divider factor is 512"]
    SYSCLK_DF_511,
}
impl SYSCLK_DFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SYSCLK_DFW::SYSCLK_DF_0 => 0,
            SYSCLK_DFW::SYSCLK_DF_1 => 1,
            SYSCLK_DFW::SYSCLK_DF_511 => 511,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSCLK_DFW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCLK_DFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSCLK_DFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "no clock signal"]
    #[inline]
    pub fn sysclk_df_0(self) -> &'a mut W {
        self.variant(SYSCLK_DFW::SYSCLK_DF_0)
    }
    #[doc = "divider factor is 2"]
    #[inline]
    pub fn sysclk_df_1(self) -> &'a mut W {
        self.variant(SYSCLK_DFW::SYSCLK_DF_1)
    }
    #[doc = "divider factor is 512"]
    #[inline]
    pub fn sysclk_df_511(self) -> &'a mut W {
        self.variant(SYSCLK_DFW::SYSCLK_DF_511)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:6 - Divider factor (1-128)"]
    #[inline]
    pub fn tx_clk_df(&self) -> TXCLK_DFR {
        TXCLK_DFR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[inline]
    pub fn tx_all_clk_en(&self) -> TX_ALL_CLK_ENR {
        TX_ALL_CLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - no description available"]
    #[inline]
    pub fn tx_clk_source(&self) -> TXCLK_SOURCER {
        TXCLK_SOURCER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:19 - system clock divider factor, 2~512."]
    #[inline]
    pub fn sysclk_df(&self) -> SYSCLK_DFR {
        SYSCLK_DFR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 134912 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Divider factor (1-128)"]
    #[inline]
    pub fn tx_clk_df(&mut self) -> _TXCLK_DFW {
        _TXCLK_DFW { w: self }
    }
    #[doc = "Bit 7 - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[inline]
    pub fn tx_all_clk_en(&mut self) -> _TX_ALL_CLK_ENW {
        _TX_ALL_CLK_ENW { w: self }
    }
    #[doc = "Bits 8:10 - no description available"]
    #[inline]
    pub fn tx_clk_source(&mut self) -> _TXCLK_SOURCEW {
        _TXCLK_SOURCEW { w: self }
    }
    #[doc = "Bits 11:19 - system clock divider factor, 2~512."]
    #[inline]
    pub fn sysclk_df(&mut self) -> _SYSCLK_DFW {
        _SYSCLK_DFW { w: self }
    }
}
