#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRPC {
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
#[doc = "Possible values of the field `GainSel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAINSELR {
    #[doc = "24*(2**10)"]
    GAINSEL_0,
    #[doc = "16*(2**10)"]
    GAINSEL_1,
    #[doc = "12*(2**10)"]
    GAINSEL_2,
    #[doc = "8*(2**10)"]
    GAINSEL_3,
    #[doc = "6*(2**10)"]
    GAINSEL_4,
    #[doc = "4*(2**10)"]
    GAINSEL_5,
    #[doc = "3*(2**10)"]
    GAINSEL_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAINSELR::GAINSEL_0 => 0,
            GAINSELR::GAINSEL_1 => 1,
            GAINSELR::GAINSEL_2 => 2,
            GAINSELR::GAINSEL_3 => 3,
            GAINSELR::GAINSEL_4 => 4,
            GAINSELR::GAINSEL_5 => 5,
            GAINSELR::GAINSEL_6 => 6,
            GAINSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAINSELR {
        match value {
            0 => GAINSELR::GAINSEL_0,
            1 => GAINSELR::GAINSEL_1,
            2 => GAINSELR::GAINSEL_2,
            3 => GAINSELR::GAINSEL_3,
            4 => GAINSELR::GAINSEL_4,
            5 => GAINSELR::GAINSEL_5,
            6 => GAINSELR::GAINSEL_6,
            i => GAINSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GAINSEL_0`"]
    #[inline]
    pub fn is_gain_sel_0(&self) -> bool {
        *self == GAINSELR::GAINSEL_0
    }
    #[doc = "Checks if the value of the field is `GAINSEL_1`"]
    #[inline]
    pub fn is_gain_sel_1(&self) -> bool {
        *self == GAINSELR::GAINSEL_1
    }
    #[doc = "Checks if the value of the field is `GAINSEL_2`"]
    #[inline]
    pub fn is_gain_sel_2(&self) -> bool {
        *self == GAINSELR::GAINSEL_2
    }
    #[doc = "Checks if the value of the field is `GAINSEL_3`"]
    #[inline]
    pub fn is_gain_sel_3(&self) -> bool {
        *self == GAINSELR::GAINSEL_3
    }
    #[doc = "Checks if the value of the field is `GAINSEL_4`"]
    #[inline]
    pub fn is_gain_sel_4(&self) -> bool {
        *self == GAINSELR::GAINSEL_4
    }
    #[doc = "Checks if the value of the field is `GAINSEL_5`"]
    #[inline]
    pub fn is_gain_sel_5(&self) -> bool {
        *self == GAINSELR::GAINSEL_5
    }
    #[doc = "Checks if the value of the field is `GAINSEL_6`"]
    #[inline]
    pub fn is_gain_sel_6(&self) -> bool {
        *self == GAINSELR::GAINSEL_6
    }
}
#[doc = r" Value of the field"]
pub struct LOCKR {
    bits: bool,
}
impl LOCKR {
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
#[doc = "Possible values of the field `ClkSrc_Sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_SELR {
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    CLKSRC_SEL_0,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_SEL_1,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    CLKSRC_SEL_3,
    #[doc = "REF_CLK_32K (XTALOSC)"]
    CLKSRC_SEL_5,
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_SEL_6,
    #[doc = "SPDIF_EXT_CLK"]
    CLKSRC_SEL_8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSRC_SELR::CLKSRC_SEL_0 => 0,
            CLKSRC_SELR::CLKSRC_SEL_1 => 1,
            CLKSRC_SELR::CLKSRC_SEL_3 => 3,
            CLKSRC_SELR::CLKSRC_SEL_5 => 5,
            CLKSRC_SELR::CLKSRC_SEL_6 => 6,
            CLKSRC_SELR::CLKSRC_SEL_8 => 8,
            CLKSRC_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSRC_SELR {
        match value {
            0 => CLKSRC_SELR::CLKSRC_SEL_0,
            1 => CLKSRC_SELR::CLKSRC_SEL_1,
            3 => CLKSRC_SELR::CLKSRC_SEL_3,
            5 => CLKSRC_SELR::CLKSRC_SEL_5,
            6 => CLKSRC_SELR::CLKSRC_SEL_6,
            8 => CLKSRC_SELR::CLKSRC_SEL_8,
            i => CLKSRC_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_0`"]
    #[inline]
    pub fn is_clk_src_sel_0(&self) -> bool {
        *self == CLKSRC_SELR::CLKSRC_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_1`"]
    #[inline]
    pub fn is_clk_src_sel_1(&self) -> bool {
        *self == CLKSRC_SELR::CLKSRC_SEL_1
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_3`"]
    #[inline]
    pub fn is_clk_src_sel_3(&self) -> bool {
        *self == CLKSRC_SELR::CLKSRC_SEL_3
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_5`"]
    #[inline]
    pub fn is_clk_src_sel_5(&self) -> bool {
        *self == CLKSRC_SELR::CLKSRC_SEL_5
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_6`"]
    #[inline]
    pub fn is_clk_src_sel_6(&self) -> bool {
        *self == CLKSRC_SELR::CLKSRC_SEL_6
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_8`"]
    #[inline]
    pub fn is_clk_src_sel_8(&self) -> bool {
        *self == CLKSRC_SELR::CLKSRC_SEL_8
    }
}
#[doc = "Values that can be written to the field `GainSel`"]
pub enum GAINSELW {
    #[doc = "24*(2**10)"]
    GAINSEL_0,
    #[doc = "16*(2**10)"]
    GAINSEL_1,
    #[doc = "12*(2**10)"]
    GAINSEL_2,
    #[doc = "8*(2**10)"]
    GAINSEL_3,
    #[doc = "6*(2**10)"]
    GAINSEL_4,
    #[doc = "4*(2**10)"]
    GAINSEL_5,
    #[doc = "3*(2**10)"]
    GAINSEL_6,
}
impl GAINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAINSELW::GAINSEL_0 => 0,
            GAINSELW::GAINSEL_1 => 1,
            GAINSELW::GAINSEL_2 => 2,
            GAINSELW::GAINSEL_3 => 3,
            GAINSELW::GAINSEL_4 => 4,
            GAINSELW::GAINSEL_5 => 5,
            GAINSELW::GAINSEL_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAINSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "24*(2**10)"]
    #[inline]
    pub fn gain_sel_0(self) -> &'a mut W {
        self.variant(GAINSELW::GAINSEL_0)
    }
    #[doc = "16*(2**10)"]
    #[inline]
    pub fn gain_sel_1(self) -> &'a mut W {
        self.variant(GAINSELW::GAINSEL_1)
    }
    #[doc = "12*(2**10)"]
    #[inline]
    pub fn gain_sel_2(self) -> &'a mut W {
        self.variant(GAINSELW::GAINSEL_2)
    }
    #[doc = "8*(2**10)"]
    #[inline]
    pub fn gain_sel_3(self) -> &'a mut W {
        self.variant(GAINSELW::GAINSEL_3)
    }
    #[doc = "6*(2**10)"]
    #[inline]
    pub fn gain_sel_4(self) -> &'a mut W {
        self.variant(GAINSELW::GAINSEL_4)
    }
    #[doc = "4*(2**10)"]
    #[inline]
    pub fn gain_sel_5(self) -> &'a mut W {
        self.variant(GAINSELW::GAINSEL_5)
    }
    #[doc = "3*(2**10)"]
    #[inline]
    pub fn gain_sel_6(self) -> &'a mut W {
        self.variant(GAINSELW::GAINSEL_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ClkSrc_Sel`"]
pub enum CLKSRC_SELW {
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    CLKSRC_SEL_0,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_SEL_1,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    CLKSRC_SEL_3,
    #[doc = "REF_CLK_32K (XTALOSC)"]
    CLKSRC_SEL_5,
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_SEL_6,
    #[doc = "SPDIF_EXT_CLK"]
    CLKSRC_SEL_8,
}
impl CLKSRC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSRC_SELW::CLKSRC_SEL_0 => 0,
            CLKSRC_SELW::CLKSRC_SEL_1 => 1,
            CLKSRC_SELW::CLKSRC_SEL_3 => 3,
            CLKSRC_SELW::CLKSRC_SEL_5 => 5,
            CLKSRC_SELW::CLKSRC_SEL_6 => 6,
            CLKSRC_SELW::CLKSRC_SEL_8 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSRC_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    #[inline]
    pub fn clk_src_sel_0(self) -> &'a mut W {
        self.variant(CLKSRC_SELW::CLKSRC_SEL_0)
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    #[inline]
    pub fn clk_src_sel_1(self) -> &'a mut W {
        self.variant(CLKSRC_SELW::CLKSRC_SEL_1)
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    #[inline]
    pub fn clk_src_sel_3(self) -> &'a mut W {
        self.variant(CLKSRC_SELW::CLKSRC_SEL_3)
    }
    #[doc = "REF_CLK_32K (XTALOSC)"]
    #[inline]
    pub fn clk_src_sel_5(self) -> &'a mut W {
        self.variant(CLKSRC_SELW::CLKSRC_SEL_5)
    }
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
    #[inline]
    pub fn clk_src_sel_6(self) -> &'a mut W {
        self.variant(CLKSRC_SELW::CLKSRC_SEL_6)
    }
    #[doc = "SPDIF_EXT_CLK"]
    #[inline]
    pub fn clk_src_sel_8(self) -> &'a mut W {
        self.variant(CLKSRC_SELW::CLKSRC_SEL_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 3:5 - Gain selection:"]
    #[inline]
    pub fn gain_sel(&self) -> GAINSELR {
        GAINSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - LOCK bit to show that the internal DPLL is locked, read only"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCKR { bits }
    }
    #[doc = "Bits 7:10 - Clock source selection, all other settings not shown are reserved:"]
    #[inline]
    pub fn clk_src_sel(&self) -> CLKSRC_SELR {
        CLKSRC_SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 3:5 - Gain selection:"]
    #[inline]
    pub fn gain_sel(&mut self) -> _GAINSELW {
        _GAINSELW { w: self }
    }
    #[doc = "Bits 7:10 - Clock source selection, all other settings not shown are reserved:"]
    #[inline]
    pub fn clk_src_sel(&mut self) -> _CLKSRC_SELW {
        _CLKSRC_SELW { w: self }
    }
}
