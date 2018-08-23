#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CS2CDR {
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
#[doc = "Possible values of the field `SAI2_CLK_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_CLK_PODFR {
    #[doc = "divide by 1"]
    SAI2_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    SAI2_CLK_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAI2_CLK_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI2_CLK_PODFR::SAI2_CLK_PODF_0 => 0,
            SAI2_CLK_PODFR::SAI2_CLK_PODF_63 => 63,
            SAI2_CLK_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI2_CLK_PODFR {
        match value {
            0 => SAI2_CLK_PODFR::SAI2_CLK_PODF_0,
            63 => SAI2_CLK_PODFR::SAI2_CLK_PODF_63,
            i => SAI2_CLK_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PODF_0`"]
    #[inline]
    pub fn is_sai2_clk_podf_0(&self) -> bool {
        *self == SAI2_CLK_PODFR::SAI2_CLK_PODF_0
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PODF_63`"]
    #[inline]
    pub fn is_sai2_clk_podf_63(&self) -> bool {
        *self == SAI2_CLK_PODFR::SAI2_CLK_PODF_63
    }
}
#[doc = "Possible values of the field `SAI2_CLK_PRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_CLK_PREDR {
    #[doc = "divide by 1"]
    SAI2_CLK_PRED_0,
    #[doc = "divide by 2"]
    SAI2_CLK_PRED_1,
    #[doc = "divide by 3"]
    SAI2_CLK_PRED_2,
    #[doc = "divide by 4"]
    SAI2_CLK_PRED_3,
    #[doc = "divide by 5"]
    SAI2_CLK_PRED_4,
    #[doc = "divide by 6"]
    SAI2_CLK_PRED_5,
    #[doc = "divide by 7"]
    SAI2_CLK_PRED_6,
    #[doc = "divide by 8"]
    SAI2_CLK_PRED_7,
}
impl SAI2_CLK_PREDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAI2_CLK_PREDR::SAI2_CLK_PRED_0 => 0,
            SAI2_CLK_PREDR::SAI2_CLK_PRED_1 => 1,
            SAI2_CLK_PREDR::SAI2_CLK_PRED_2 => 2,
            SAI2_CLK_PREDR::SAI2_CLK_PRED_3 => 3,
            SAI2_CLK_PREDR::SAI2_CLK_PRED_4 => 4,
            SAI2_CLK_PREDR::SAI2_CLK_PRED_5 => 5,
            SAI2_CLK_PREDR::SAI2_CLK_PRED_6 => 6,
            SAI2_CLK_PREDR::SAI2_CLK_PRED_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAI2_CLK_PREDR {
        match value {
            0 => SAI2_CLK_PREDR::SAI2_CLK_PRED_0,
            1 => SAI2_CLK_PREDR::SAI2_CLK_PRED_1,
            2 => SAI2_CLK_PREDR::SAI2_CLK_PRED_2,
            3 => SAI2_CLK_PREDR::SAI2_CLK_PRED_3,
            4 => SAI2_CLK_PREDR::SAI2_CLK_PRED_4,
            5 => SAI2_CLK_PREDR::SAI2_CLK_PRED_5,
            6 => SAI2_CLK_PREDR::SAI2_CLK_PRED_6,
            7 => SAI2_CLK_PREDR::SAI2_CLK_PRED_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_0`"]
    #[inline]
    pub fn is_sai2_clk_pred_0(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_0
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_1`"]
    #[inline]
    pub fn is_sai2_clk_pred_1(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_1
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_2`"]
    #[inline]
    pub fn is_sai2_clk_pred_2(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_2
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_3`"]
    #[inline]
    pub fn is_sai2_clk_pred_3(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_3
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_4`"]
    #[inline]
    pub fn is_sai2_clk_pred_4(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_4
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_5`"]
    #[inline]
    pub fn is_sai2_clk_pred_5(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_5
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_6`"]
    #[inline]
    pub fn is_sai2_clk_pred_6(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_6
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_7`"]
    #[inline]
    pub fn is_sai2_clk_pred_7(&self) -> bool {
        *self == SAI2_CLK_PREDR::SAI2_CLK_PRED_7
    }
}
#[doc = "Values that can be written to the field `SAI2_CLK_PODF`"]
pub enum SAI2_CLK_PODFW {
    #[doc = "divide by 1"]
    SAI2_CLK_PODF_0,
    #[doc = "divide by 2^6"]
    SAI2_CLK_PODF_63,
}
impl SAI2_CLK_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI2_CLK_PODFW::SAI2_CLK_PODF_0 => 0,
            SAI2_CLK_PODFW::SAI2_CLK_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI2_CLK_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2_CLK_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2_CLK_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn sai2_clk_podf_0(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODFW::SAI2_CLK_PODF_0)
    }
    #[doc = "divide by 2^6"]
    #[inline]
    pub fn sai2_clk_podf_63(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODFW::SAI2_CLK_PODF_63)
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
#[doc = "Values that can be written to the field `SAI2_CLK_PRED`"]
pub enum SAI2_CLK_PREDW {
    #[doc = "divide by 1"]
    SAI2_CLK_PRED_0,
    #[doc = "divide by 2"]
    SAI2_CLK_PRED_1,
    #[doc = "divide by 3"]
    SAI2_CLK_PRED_2,
    #[doc = "divide by 4"]
    SAI2_CLK_PRED_3,
    #[doc = "divide by 5"]
    SAI2_CLK_PRED_4,
    #[doc = "divide by 6"]
    SAI2_CLK_PRED_5,
    #[doc = "divide by 7"]
    SAI2_CLK_PRED_6,
    #[doc = "divide by 8"]
    SAI2_CLK_PRED_7,
}
impl SAI2_CLK_PREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAI2_CLK_PREDW::SAI2_CLK_PRED_0 => 0,
            SAI2_CLK_PREDW::SAI2_CLK_PRED_1 => 1,
            SAI2_CLK_PREDW::SAI2_CLK_PRED_2 => 2,
            SAI2_CLK_PREDW::SAI2_CLK_PRED_3 => 3,
            SAI2_CLK_PREDW::SAI2_CLK_PRED_4 => 4,
            SAI2_CLK_PREDW::SAI2_CLK_PRED_5 => 5,
            SAI2_CLK_PREDW::SAI2_CLK_PRED_6 => 6,
            SAI2_CLK_PREDW::SAI2_CLK_PRED_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAI2_CLK_PREDW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2_CLK_PREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2_CLK_PREDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide by 1"]
    #[inline]
    pub fn sai2_clk_pred_0(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline]
    pub fn sai2_clk_pred_1(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline]
    pub fn sai2_clk_pred_2(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_2)
    }
    #[doc = "divide by 4"]
    #[inline]
    pub fn sai2_clk_pred_3(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_3)
    }
    #[doc = "divide by 5"]
    #[inline]
    pub fn sai2_clk_pred_4(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_4)
    }
    #[doc = "divide by 6"]
    #[inline]
    pub fn sai2_clk_pred_5(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_5)
    }
    #[doc = "divide by 7"]
    #[inline]
    pub fn sai2_clk_pred_6(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_6)
    }
    #[doc = "divide by 8"]
    #[inline]
    pub fn sai2_clk_pred_7(self) -> &'a mut W {
        self.variant(SAI2_CLK_PREDW::SAI2_CLK_PRED_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:5 - Divider for sai2 clock podf"]
    #[inline]
    pub fn sai2_clk_podf(&self) -> SAI2_CLK_PODFR {
        SAI2_CLK_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - Divider for sai2 clock pred.Divider should be updated when output clock is gated."]
    #[inline]
    pub fn sai2_clk_pred(&self) -> SAI2_CLK_PREDR {
        SAI2_CLK_PREDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 210625 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Divider for sai2 clock podf"]
    #[inline]
    pub fn sai2_clk_podf(&mut self) -> _SAI2_CLK_PODFW {
        _SAI2_CLK_PODFW { w: self }
    }
    #[doc = "Bits 6:8 - Divider for sai2 clock pred.Divider should be updated when output clock is gated."]
    #[inline]
    pub fn sai2_clk_pred(&mut self) -> _SAI2_CLK_PREDW {
        _SAI2_CLK_PREDW { w: self }
    }
}
