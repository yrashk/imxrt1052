#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSCMR2 {
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
#[doc = "Possible values of the field `CAN_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_CLK_PODFR {
    #[doc = "divide by 1"]
    CAN_CLK_PODF_0,
    #[doc = "divide by 8"]
    CAN_CLK_PODF_7,
    #[doc = "divide by 2^6"]
    CAN_CLK_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CAN_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAN_CLK_PODFR::CAN_CLK_PODF_0 => 0,
            CAN_CLK_PODFR::CAN_CLK_PODF_7 => 7,
            CAN_CLK_PODFR::CAN_CLK_PODF_63 => 63,
            CAN_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAN_CLK_PODFR {
        match value {
            0 => CAN_CLK_PODFR::CAN_CLK_PODF_0,
            7 => CAN_CLK_PODFR::CAN_CLK_PODF_7,
            63 => CAN_CLK_PODFR::CAN_CLK_PODF_63,
            i => CAN_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAN_CLK_PODF_0`"]
    #[inline]
    pub fn is_can_clk_podf_0(&self) -> bool {
        *self == CAN_CLK_PODFR::CAN_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `CAN_CLK_PODF_7`"]
    #[inline]
    pub fn is_can_clk_podf_7(&self) -> bool {
        *self == CAN_CLK_PODFR::CAN_CLK_PODF_7
    }
    #[doc = "Checks if the value of the field is `CAN_CLK_PODF_63`"]
    #[inline]
    pub fn is_can_clk_podf_63(&self) -> bool {
        *self == CAN_CLK_PODFR::CAN_CLK_PODF_63
    }
}
#[doc = "Possible values of the field `CAN_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_CLK_SELR {
    #[doc = "derive clock from pll3_sw_clk divided clock (60M)"]
    CAN_CLK_SEL_0,
    #[doc = "derive clock from osc_clk (24M)"]
    CAN_CLK_SEL_1,
    #[doc = "derive clock from pll3_sw_clk divided clock (80M)"]
    CAN_CLK_SEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CAN_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAN_CLK_SELR::CAN_CLK_SEL_0 => 0,
            CAN_CLK_SELR::CAN_CLK_SEL_1 => 1,
            CAN_CLK_SELR::CAN_CLK_SEL_2 => 2,
            CAN_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAN_CLK_SELR {
        match value {
            0 => CAN_CLK_SELR::CAN_CLK_SEL_0,
            1 => CAN_CLK_SELR::CAN_CLK_SEL_1,
            2 => CAN_CLK_SELR::CAN_CLK_SEL_2,
            i => CAN_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAN_CLK_SEL_0`"]
    #[inline]
    pub fn is_can_clk_sel_0(&self) -> bool {
        *self == CAN_CLK_SELR::CAN_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `CAN_CLK_SEL_1`"]
    #[inline]
    pub fn is_can_clk_sel_1(&self) -> bool {
        *self == CAN_CLK_SELR::CAN_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `CAN_CLK_SEL_2`"]
    #[inline]
    pub fn is_can_clk_sel_2(&self) -> bool {
        *self == CAN_CLK_SELR::CAN_CLK_SEL_2
    }
}
#[doc = "Possible values of the field `FLEXIO2_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_CLK_SELR {
    #[doc = "derive clock from PLL4 divided clock"]
    FLEXIO2_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD2 clock"]
    FLEXIO2_CLK_SEL_1,
    #[doc = "derive clock from PLL5 clock"]
    FLEXIO2_CLK_SEL_2,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO2_CLK_SEL_3,
}
impl FLEXIO2_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_0 => 0,
            FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_1 => 1,
            FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_2 => 2,
            FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXIO2_CLK_SELR {
        match value {
            0 => FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_0,
            1 => FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_1,
            2 => FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_2,
            3 => FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_SEL_0`"]
    #[inline]
    pub fn is_flexio2_clk_sel_0(&self) -> bool {
        *self == FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_SEL_1`"]
    #[inline]
    pub fn is_flexio2_clk_sel_1(&self) -> bool {
        *self == FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_SEL_2`"]
    #[inline]
    pub fn is_flexio2_clk_sel_2(&self) -> bool {
        *self == FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_CLK_SEL_3`"]
    #[inline]
    pub fn is_flexio2_clk_sel_3(&self) -> bool {
        *self == FLEXIO2_CLK_SELR::FLEXIO2_CLK_SEL_3
    }
}
#[doc = "Values that can be written to the field `CAN_CLK_PODF`"]
pub enum CAN_CLK_PODFW {
    #[doc = "divide by 1"]
    CAN_CLK_PODF_0,
    #[doc = "divide by 8"]
    CAN_CLK_PODF_7,
    #[doc = "divide by 2^6"]
    CAN_CLK_PODF_63,
}
impl CAN_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAN_CLK_PODFW::CAN_CLK_PODF_0 => 0,
            CAN_CLK_PODFW::CAN_CLK_PODF_7 => 7,
            CAN_CLK_PODFW::CAN_CLK_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAN_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn can_clk_podf_0(self) -> &'a mut W {
        self.variant(CAN_CLK_PODFW::CAN_CLK_PODF_0)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn can_clk_podf_7(self) -> &'a mut W {
        self.variant(CAN_CLK_PODFW::CAN_CLK_PODF_7)
    }
    #[doc = "divide by 2^6"]
    #[inline]
    pub fn can_clk_podf_63(self) -> &'a mut W {
        self.variant(CAN_CLK_PODFW::CAN_CLK_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAN_CLK_SEL`"]
pub enum CAN_CLK_SELW {
    #[doc = "derive clock from pll3_sw_clk divided clock (60M)"]
    CAN_CLK_SEL_0,
    #[doc = "derive clock from osc_clk (24M)"]
    CAN_CLK_SEL_1,
    #[doc = "derive clock from pll3_sw_clk divided clock (80M)"]
    CAN_CLK_SEL_2,
}
impl CAN_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAN_CLK_SELW::CAN_CLK_SEL_0 => 0,
            CAN_CLK_SELW::CAN_CLK_SEL_1 => 1,
            CAN_CLK_SELW::CAN_CLK_SEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAN_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "derive clock from pll3_sw_clk divided clock (60M)"]
    #[inline]
    pub fn can_clk_sel_0(self) -> &'a mut W {
        self.variant(CAN_CLK_SELW::CAN_CLK_SEL_0)
    }
    #[doc = "derive clock from osc_clk (24M)"]
    #[inline]
    pub fn can_clk_sel_1(self) -> &'a mut W {
        self.variant(CAN_CLK_SELW::CAN_CLK_SEL_1)
    }
    #[doc = "derive clock from pll3_sw_clk divided clock (80M)"]
    #[inline]
    pub fn can_clk_sel_2(self) -> &'a mut W {
        self.variant(CAN_CLK_SELW::CAN_CLK_SEL_2)
    }
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
#[doc = "Values that can be written to the field `FLEXIO2_CLK_SEL`"]
pub enum FLEXIO2_CLK_SELW {
    #[doc = "derive clock from PLL4 divided clock"]
    FLEXIO2_CLK_SEL_0,
    #[doc = "derive clock from PLL3 PFD2 clock"]
    FLEXIO2_CLK_SEL_1,
    #[doc = "derive clock from PLL5 clock"]
    FLEXIO2_CLK_SEL_2,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO2_CLK_SEL_3,
}
impl FLEXIO2_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_0 => 0,
            FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_1 => 1,
            FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_2 => 2,
            FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO2_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO2_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO2_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "derive clock from PLL4 divided clock"]
    #[inline]
    pub fn flexio2_clk_sel_0(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD2 clock"]
    #[inline]
    pub fn flexio2_clk_sel_1(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL5 clock"]
    #[inline]
    pub fn flexio2_clk_sel_2(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_2)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline]
    pub fn flexio2_clk_sel_3(self) -> &'a mut W {
        self.variant(FLEXIO2_CLK_SELW::FLEXIO2_CLK_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
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
    #[doc = "Bits 2:7 - Divider for can clock podf."]
    #[inline]
    pub fn can_clk_podf(&self) -> CAN_CLK_PODFR {
        CAN_CLK_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Selector for FlexCAN clock multiplexer"]
    #[inline]
    pub fn can_clk_sel(&self) -> CAN_CLK_SELR {
        CAN_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:20 - Selector for flexio2 clock multiplexer"]
    #[inline]
    pub fn flexio2_clk_sel(&self) -> FLEXIO2_CLK_SELR {
        FLEXIO2_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 51981318 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:7 - Divider for can clock podf."]
    #[inline]
    pub fn can_clk_podf(&mut self) -> _CAN_CLK_PODFW {
        _CAN_CLK_PODFW { w: self }
    }
    #[doc = "Bits 8:9 - Selector for FlexCAN clock multiplexer"]
    #[inline]
    pub fn can_clk_sel(&mut self) -> _CAN_CLK_SELW {
        _CAN_CLK_SELW { w: self }
    }
    #[doc = "Bits 19:20 - Selector for flexio2 clock multiplexer"]
    #[inline]
    pub fn flexio2_clk_sel(&mut self) -> _FLEXIO2_CLK_SELW {
        _FLEXIO2_CLK_SELW { w: self }
    }
}
