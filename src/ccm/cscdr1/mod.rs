#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSCDR1 {
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
#[doc = "Possible values of the field `UART_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_CLK_PODFR {
    #[doc = "divide by 1"]
    UART_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    UART_CLK_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UART_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART_CLK_PODFR::UART_CLK_PODF_0 => 0,
            UART_CLK_PODFR::UART_CLK_PODF_63 => 63,
            UART_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART_CLK_PODFR {
        match value {
            0 => UART_CLK_PODFR::UART_CLK_PODF_0,
            63 => UART_CLK_PODFR::UART_CLK_PODF_63,
            i => UART_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART_CLK_PODF_0`"]
    #[inline]
    pub fn is_uart_clk_podf_0(&self) -> bool {
        *self == UART_CLK_PODFR::UART_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `UART_CLK_PODF_63`"]
    #[inline]
    pub fn is_uart_clk_podf_63(&self) -> bool {
        *self == UART_CLK_PODFR::UART_CLK_PODF_63
    }
}
#[doc = "Possible values of the field `UART_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_CLK_SELR {
    #[doc = "derive clock from pll3_80m"]
    UART_CLK_SEL_0,
    #[doc = "derive clock from osc_clk"]
    UART_CLK_SEL_1,
}
impl UART_CLK_SELR {
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
            UART_CLK_SELR::UART_CLK_SEL_0 => false,
            UART_CLK_SELR::UART_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART_CLK_SELR {
        match value {
            false => UART_CLK_SELR::UART_CLK_SEL_0,
            true => UART_CLK_SELR::UART_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART_CLK_SEL_0`"]
    #[inline]
    pub fn is_uart_clk_sel_0(&self) -> bool {
        *self == UART_CLK_SELR::UART_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `UART_CLK_SEL_1`"]
    #[inline]
    pub fn is_uart_clk_sel_1(&self) -> bool {
        *self == UART_CLK_SELR::UART_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `USDHC1_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC1_PODFR {
    #[doc = "divide by 1"]
    USDHC1_PODF_0,
    #[doc = "divide by 2"]
    USDHC1_PODF_1,
    #[doc = "divide by 3"]
    USDHC1_PODF_2,
    #[doc = "divide by 4"]
    USDHC1_PODF_3,
    #[doc = "divide by 5"]
    USDHC1_PODF_4,
    #[doc = "divide by 6"]
    USDHC1_PODF_5,
    #[doc = "divide by 7"]
    USDHC1_PODF_6,
    #[doc = "divide by 8"]
    USDHC1_PODF_7,
}
impl USDHC1_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USDHC1_PODFR::USDHC1_PODF_0 => 0,
            USDHC1_PODFR::USDHC1_PODF_1 => 1,
            USDHC1_PODFR::USDHC1_PODF_2 => 2,
            USDHC1_PODFR::USDHC1_PODF_3 => 3,
            USDHC1_PODFR::USDHC1_PODF_4 => 4,
            USDHC1_PODFR::USDHC1_PODF_5 => 5,
            USDHC1_PODFR::USDHC1_PODF_6 => 6,
            USDHC1_PODFR::USDHC1_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USDHC1_PODFR {
        match value {
            0 => USDHC1_PODFR::USDHC1_PODF_0,
            1 => USDHC1_PODFR::USDHC1_PODF_1,
            2 => USDHC1_PODFR::USDHC1_PODF_2,
            3 => USDHC1_PODFR::USDHC1_PODF_3,
            4 => USDHC1_PODFR::USDHC1_PODF_4,
            5 => USDHC1_PODFR::USDHC1_PODF_5,
            6 => USDHC1_PODFR::USDHC1_PODF_6,
            7 => USDHC1_PODFR::USDHC1_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_0`"]
    #[inline]
    pub fn is_usdhc1_podf_0(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_0
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_1`"]
    #[inline]
    pub fn is_usdhc1_podf_1(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_1
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_2`"]
    #[inline]
    pub fn is_usdhc1_podf_2(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_2
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_3`"]
    #[inline]
    pub fn is_usdhc1_podf_3(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_3
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_4`"]
    #[inline]
    pub fn is_usdhc1_podf_4(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_4
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_5`"]
    #[inline]
    pub fn is_usdhc1_podf_5(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_5
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_6`"]
    #[inline]
    pub fn is_usdhc1_podf_6(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_6
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_7`"]
    #[inline]
    pub fn is_usdhc1_podf_7(&self) -> bool {
        *self == USDHC1_PODFR::USDHC1_PODF_7
    }
}
#[doc = "Possible values of the field `USDHC2_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC2_PODFR {
    #[doc = "divide by 1"]
    USDHC2_PODF_0,
    #[doc = "divide by 2"]
    USDHC2_PODF_1,
    #[doc = "divide by 3"]
    USDHC2_PODF_2,
    #[doc = "divide by 4"]
    USDHC2_PODF_3,
    #[doc = "divide by 5"]
    USDHC2_PODF_4,
    #[doc = "divide by 6"]
    USDHC2_PODF_5,
    #[doc = "divide by 7"]
    USDHC2_PODF_6,
    #[doc = "divide by 8"]
    USDHC2_PODF_7,
}
impl USDHC2_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USDHC2_PODFR::USDHC2_PODF_0 => 0,
            USDHC2_PODFR::USDHC2_PODF_1 => 1,
            USDHC2_PODFR::USDHC2_PODF_2 => 2,
            USDHC2_PODFR::USDHC2_PODF_3 => 3,
            USDHC2_PODFR::USDHC2_PODF_4 => 4,
            USDHC2_PODFR::USDHC2_PODF_5 => 5,
            USDHC2_PODFR::USDHC2_PODF_6 => 6,
            USDHC2_PODFR::USDHC2_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USDHC2_PODFR {
        match value {
            0 => USDHC2_PODFR::USDHC2_PODF_0,
            1 => USDHC2_PODFR::USDHC2_PODF_1,
            2 => USDHC2_PODFR::USDHC2_PODF_2,
            3 => USDHC2_PODFR::USDHC2_PODF_3,
            4 => USDHC2_PODFR::USDHC2_PODF_4,
            5 => USDHC2_PODFR::USDHC2_PODF_5,
            6 => USDHC2_PODFR::USDHC2_PODF_6,
            7 => USDHC2_PODFR::USDHC2_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_0`"]
    #[inline]
    pub fn is_usdhc2_podf_0(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_0
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_1`"]
    #[inline]
    pub fn is_usdhc2_podf_1(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_1
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_2`"]
    #[inline]
    pub fn is_usdhc2_podf_2(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_2
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_3`"]
    #[inline]
    pub fn is_usdhc2_podf_3(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_3
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_4`"]
    #[inline]
    pub fn is_usdhc2_podf_4(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_4
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_5`"]
    #[inline]
    pub fn is_usdhc2_podf_5(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_5
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_6`"]
    #[inline]
    pub fn is_usdhc2_podf_6(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_6
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_7`"]
    #[inline]
    pub fn is_usdhc2_podf_7(&self) -> bool {
        *self == USDHC2_PODFR::USDHC2_PODF_7
    }
}
#[doc = "Possible values of the field `TRACE_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACE_PODFR {
    #[doc = "divide by 1"]
    TRACE_PODF_0,
    #[doc = "divide by 2"]
    TRACE_PODF_1,
    #[doc = "divide by 3"]
    TRACE_PODF_2,
    #[doc = "divide by 4"]
    TRACE_PODF_3,
    #[doc = "divide by 5"]
    TRACE_PODF_4,
    #[doc = "divide by 6"]
    TRACE_PODF_5,
    #[doc = "divide by 7"]
    TRACE_PODF_6,
    #[doc = "divide by 8"]
    TRACE_PODF_7,
}
impl TRACE_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRACE_PODFR::TRACE_PODF_0 => 0,
            TRACE_PODFR::TRACE_PODF_1 => 1,
            TRACE_PODFR::TRACE_PODF_2 => 2,
            TRACE_PODFR::TRACE_PODF_3 => 3,
            TRACE_PODFR::TRACE_PODF_4 => 4,
            TRACE_PODFR::TRACE_PODF_5 => 5,
            TRACE_PODFR::TRACE_PODF_6 => 6,
            TRACE_PODFR::TRACE_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRACE_PODFR {
        match value {
            0 => TRACE_PODFR::TRACE_PODF_0,
            1 => TRACE_PODFR::TRACE_PODF_1,
            2 => TRACE_PODFR::TRACE_PODF_2,
            3 => TRACE_PODFR::TRACE_PODF_3,
            4 => TRACE_PODFR::TRACE_PODF_4,
            5 => TRACE_PODFR::TRACE_PODF_5,
            6 => TRACE_PODFR::TRACE_PODF_6,
            7 => TRACE_PODFR::TRACE_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_0`"]
    #[inline]
    pub fn is_trace_podf_0(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_0
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_1`"]
    #[inline]
    pub fn is_trace_podf_1(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_1
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_2`"]
    #[inline]
    pub fn is_trace_podf_2(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_2
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_3`"]
    #[inline]
    pub fn is_trace_podf_3(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_3
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_4`"]
    #[inline]
    pub fn is_trace_podf_4(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_4
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_5`"]
    #[inline]
    pub fn is_trace_podf_5(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_5
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_6`"]
    #[inline]
    pub fn is_trace_podf_6(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_6
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_7`"]
    #[inline]
    pub fn is_trace_podf_7(&self) -> bool {
        *self == TRACE_PODFR::TRACE_PODF_7
    }
}
#[doc = "Values that can be written to the field `UART_CLK_PODF`"]
pub enum UART_CLK_PODFW {
    #[doc = "divide by 1"]
    UART_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    UART_CLK_PODF_63,
}
impl UART_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART_CLK_PODFW::UART_CLK_PODF_0 => 0,
            UART_CLK_PODFW::UART_CLK_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn uart_clk_podf_0(self) -> &'a mut W {
        self.variant(UART_CLK_PODFW::UART_CLK_PODF_0)
    }
    #[doc = "divide by 2^6"]
    #[inline]
    pub fn uart_clk_podf_63(self) -> &'a mut W {
        self.variant(UART_CLK_PODFW::UART_CLK_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART_CLK_SEL`"]
pub enum UART_CLK_SELW {
    #[doc = "derive clock from pll3_80m"]
    UART_CLK_SEL_0,
    #[doc = "derive clock from osc_clk"]
    UART_CLK_SEL_1,
}
impl UART_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART_CLK_SELW::UART_CLK_SEL_0 => false,
            UART_CLK_SELW::UART_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "derive clock from pll3_80m"]
    #[inline]
    pub fn uart_clk_sel_0(self) -> &'a mut W {
        self.variant(UART_CLK_SELW::UART_CLK_SEL_0)
    }
    #[doc = "derive clock from osc_clk"]
    #[inline]
    pub fn uart_clk_sel_1(self) -> &'a mut W {
        self.variant(UART_CLK_SELW::UART_CLK_SEL_1)
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
#[doc = "Values that can be written to the field `USDHC1_PODF`"]
pub enum USDHC1_PODFW {
    #[doc = "divide by 1"]
    USDHC1_PODF_0,
    #[doc = "divide by 2"]
    USDHC1_PODF_1,
    #[doc = "divide by 3"]
    USDHC1_PODF_2,
    #[doc = "divide by 4"]
    USDHC1_PODF_3,
    #[doc = "divide by 5"]
    USDHC1_PODF_4,
    #[doc = "divide by 6"]
    USDHC1_PODF_5,
    #[doc = "divide by 7"]
    USDHC1_PODF_6,
    #[doc = "divide by 8"]
    USDHC1_PODF_7,
}
impl USDHC1_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USDHC1_PODFW::USDHC1_PODF_0 => 0,
            USDHC1_PODFW::USDHC1_PODF_1 => 1,
            USDHC1_PODFW::USDHC1_PODF_2 => 2,
            USDHC1_PODFW::USDHC1_PODF_3 => 3,
            USDHC1_PODFW::USDHC1_PODF_4 => 4,
            USDHC1_PODFW::USDHC1_PODF_5 => 5,
            USDHC1_PODFW::USDHC1_PODF_6 => 6,
            USDHC1_PODFW::USDHC1_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USDHC1_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _USDHC1_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USDHC1_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn usdhc1_podf_0(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn usdhc1_podf_1(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn usdhc1_podf_2(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn usdhc1_podf_3(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn usdhc1_podf_4(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn usdhc1_podf_5(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn usdhc1_podf_6(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn usdhc1_podf_7(self) -> &'a mut W {
        self.variant(USDHC1_PODFW::USDHC1_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USDHC2_PODF`"]
pub enum USDHC2_PODFW {
    #[doc = "divide by 1"]
    USDHC2_PODF_0,
    #[doc = "divide by 2"]
    USDHC2_PODF_1,
    #[doc = "divide by 3"]
    USDHC2_PODF_2,
    #[doc = "divide by 4"]
    USDHC2_PODF_3,
    #[doc = "divide by 5"]
    USDHC2_PODF_4,
    #[doc = "divide by 6"]
    USDHC2_PODF_5,
    #[doc = "divide by 7"]
    USDHC2_PODF_6,
    #[doc = "divide by 8"]
    USDHC2_PODF_7,
}
impl USDHC2_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USDHC2_PODFW::USDHC2_PODF_0 => 0,
            USDHC2_PODFW::USDHC2_PODF_1 => 1,
            USDHC2_PODFW::USDHC2_PODF_2 => 2,
            USDHC2_PODFW::USDHC2_PODF_3 => 3,
            USDHC2_PODFW::USDHC2_PODF_4 => 4,
            USDHC2_PODFW::USDHC2_PODF_5 => 5,
            USDHC2_PODFW::USDHC2_PODF_6 => 6,
            USDHC2_PODFW::USDHC2_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USDHC2_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _USDHC2_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USDHC2_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn usdhc2_podf_0(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn usdhc2_podf_1(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn usdhc2_podf_2(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn usdhc2_podf_3(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn usdhc2_podf_4(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn usdhc2_podf_5(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn usdhc2_podf_6(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn usdhc2_podf_7(self) -> &'a mut W {
        self.variant(USDHC2_PODFW::USDHC2_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRACE_PODF`"]
pub enum TRACE_PODFW {
    #[doc = "divide by 1"]
    TRACE_PODF_0,
    #[doc = "divide by 2"]
    TRACE_PODF_1,
    #[doc = "divide by 3"]
    TRACE_PODF_2,
    #[doc = "divide by 4"]
    TRACE_PODF_3,
    #[doc = "divide by 5"]
    TRACE_PODF_4,
    #[doc = "divide by 6"]
    TRACE_PODF_5,
    #[doc = "divide by 7"]
    TRACE_PODF_6,
    #[doc = "divide by 8"]
    TRACE_PODF_7,
}
impl TRACE_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRACE_PODFW::TRACE_PODF_0 => 0,
            TRACE_PODFW::TRACE_PODF_1 => 1,
            TRACE_PODFW::TRACE_PODF_2 => 2,
            TRACE_PODFW::TRACE_PODF_3 => 3,
            TRACE_PODFW::TRACE_PODF_4 => 4,
            TRACE_PODFW::TRACE_PODF_5 => 5,
            TRACE_PODFW::TRACE_PODF_6 => 6,
            TRACE_PODFW::TRACE_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACE_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACE_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACE_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn trace_podf_0(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn trace_podf_1(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn trace_podf_2(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn trace_podf_3(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn trace_podf_4(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn trace_podf_5(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn trace_podf_6(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn trace_podf_7(self) -> &'a mut W {
        self.variant(TRACE_PODFW::TRACE_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
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
    #[doc = "Bits 0:5 - Divider for uart clock podf."]
    #[inline]
    pub fn uart_clk_podf(&self) -> UART_CLK_PODFR {
        UART_CLK_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Selector for the UART clock multiplexor"]
    #[inline]
    pub fn uart_clk_sel(&self) -> UART_CLK_SELR {
        UART_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Divider for usdhc1 clock podf. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn usdhc1_podf(&self) -> USDHC1_PODFR {
        USDHC1_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Divider for usdhc2 clock. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn usdhc2_podf(&self) -> USDHC2_PODFR {
        USDHC2_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:27 - Divider for trace clock. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn trace_podf(&self) -> TRACE_PODFR {
        TRACE_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4786944 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Divider for uart clock podf."]
    #[inline]
    pub fn uart_clk_podf(&mut self) -> _UART_CLK_PODFW {
        _UART_CLK_PODFW { w: self }
    }
    #[doc = "Bit 6 - Selector for the UART clock multiplexor"]
    #[inline]
    pub fn uart_clk_sel(&mut self) -> _UART_CLK_SELW {
        _UART_CLK_SELW { w: self }
    }
    #[doc = "Bits 11:13 - Divider for usdhc1 clock podf. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn usdhc1_podf(&mut self) -> _USDHC1_PODFW {
        _USDHC1_PODFW { w: self }
    }
    #[doc = "Bits 16:18 - Divider for usdhc2 clock. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn usdhc2_podf(&mut self) -> _USDHC2_PODFW {
        _USDHC2_PODFW { w: self }
    }
    #[doc = "Bits 25:27 - Divider for trace clock. Divider should be updated when output clock is gated."]
    #[inline]
    pub fn trace_podf(&mut self) -> _TRACE_PODFW {
        _TRACE_PODFW { w: self }
    }
}
