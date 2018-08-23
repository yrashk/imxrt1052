#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CR0 {
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
#[doc = "Possible values of the field `HYSTCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTCTRR {
    #[doc = "Level 0"]
    HYSTCTR_0,
    #[doc = "Level 1"]
    HYSTCTR_1,
    #[doc = "Level 2"]
    HYSTCTR_2,
    #[doc = "Level 3"]
    HYSTCTR_3,
}
impl HYSTCTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HYSTCTRR::HYSTCTR_0 => 0,
            HYSTCTRR::HYSTCTR_1 => 1,
            HYSTCTRR::HYSTCTR_2 => 2,
            HYSTCTRR::HYSTCTR_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HYSTCTRR {
        match value {
            0 => HYSTCTRR::HYSTCTR_0,
            1 => HYSTCTRR::HYSTCTR_1,
            2 => HYSTCTRR::HYSTCTR_2,
            3 => HYSTCTRR::HYSTCTR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_0`"]
    #[inline]
    pub fn is_hystctr_0(&self) -> bool {
        *self == HYSTCTRR::HYSTCTR_0
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_1`"]
    #[inline]
    pub fn is_hystctr_1(&self) -> bool {
        *self == HYSTCTRR::HYSTCTR_1
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_2`"]
    #[inline]
    pub fn is_hystctr_2(&self) -> bool {
        *self == HYSTCTRR::HYSTCTR_2
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_3`"]
    #[inline]
    pub fn is_hystctr_3(&self) -> bool {
        *self == HYSTCTRR::HYSTCTR_3
    }
}
#[doc = "Possible values of the field `FILTER_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_CNTR {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    FILTER_CNT_0,
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    FILTER_CNT_1,
    #[doc = "2 consecutive samples must agree."]
    FILTER_CNT_2,
    #[doc = "3 consecutive samples must agree."]
    FILTER_CNT_3,
    #[doc = "4 consecutive samples must agree."]
    FILTER_CNT_4,
    #[doc = "5 consecutive samples must agree."]
    FILTER_CNT_5,
    #[doc = "6 consecutive samples must agree."]
    FILTER_CNT_6,
    #[doc = "7 consecutive samples must agree."]
    FILTER_CNT_7,
}
impl FILTER_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTER_CNTR::FILTER_CNT_0 => 0,
            FILTER_CNTR::FILTER_CNT_1 => 1,
            FILTER_CNTR::FILTER_CNT_2 => 2,
            FILTER_CNTR::FILTER_CNT_3 => 3,
            FILTER_CNTR::FILTER_CNT_4 => 4,
            FILTER_CNTR::FILTER_CNT_5 => 5,
            FILTER_CNTR::FILTER_CNT_6 => 6,
            FILTER_CNTR::FILTER_CNT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTER_CNTR {
        match value {
            0 => FILTER_CNTR::FILTER_CNT_0,
            1 => FILTER_CNTR::FILTER_CNT_1,
            2 => FILTER_CNTR::FILTER_CNT_2,
            3 => FILTER_CNTR::FILTER_CNT_3,
            4 => FILTER_CNTR::FILTER_CNT_4,
            5 => FILTER_CNTR::FILTER_CNT_5,
            6 => FILTER_CNTR::FILTER_CNT_6,
            7 => FILTER_CNTR::FILTER_CNT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_0`"]
    #[inline]
    pub fn is_filter_cnt_0(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_0
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_1`"]
    #[inline]
    pub fn is_filter_cnt_1(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_1
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_2`"]
    #[inline]
    pub fn is_filter_cnt_2(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_2
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_3`"]
    #[inline]
    pub fn is_filter_cnt_3(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_3
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_4`"]
    #[inline]
    pub fn is_filter_cnt_4(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_4
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_5`"]
    #[inline]
    pub fn is_filter_cnt_5(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_5
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_6`"]
    #[inline]
    pub fn is_filter_cnt_6(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_6
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_7`"]
    #[inline]
    pub fn is_filter_cnt_7(&self) -> bool {
        *self == FILTER_CNTR::FILTER_CNT_7
    }
}
#[doc = "Values that can be written to the field `HYSTCTR`"]
pub enum HYSTCTRW {
    #[doc = "Level 0"]
    HYSTCTR_0,
    #[doc = "Level 1"]
    HYSTCTR_1,
    #[doc = "Level 2"]
    HYSTCTR_2,
    #[doc = "Level 3"]
    HYSTCTR_3,
}
impl HYSTCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HYSTCTRW::HYSTCTR_0 => 0,
            HYSTCTRW::HYSTCTR_1 => 1,
            HYSTCTRW::HYSTCTR_2 => 2,
            HYSTCTRW::HYSTCTR_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTCTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0"]
    #[inline]
    pub fn hystctr_0(self) -> &'a mut W {
        self.variant(HYSTCTRW::HYSTCTR_0)
    }
    #[doc = "Level 1"]
    #[inline]
    pub fn hystctr_1(self) -> &'a mut W {
        self.variant(HYSTCTRW::HYSTCTR_1)
    }
    #[doc = "Level 2"]
    #[inline]
    pub fn hystctr_2(self) -> &'a mut W {
        self.variant(HYSTCTRW::HYSTCTR_2)
    }
    #[doc = "Level 3"]
    #[inline]
    pub fn hystctr_3(self) -> &'a mut W {
        self.variant(HYSTCTRW::HYSTCTR_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTER_CNT`"]
pub enum FILTER_CNTW {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    FILTER_CNT_0,
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    FILTER_CNT_1,
    #[doc = "2 consecutive samples must agree."]
    FILTER_CNT_2,
    #[doc = "3 consecutive samples must agree."]
    FILTER_CNT_3,
    #[doc = "4 consecutive samples must agree."]
    FILTER_CNT_4,
    #[doc = "5 consecutive samples must agree."]
    FILTER_CNT_5,
    #[doc = "6 consecutive samples must agree."]
    FILTER_CNT_6,
    #[doc = "7 consecutive samples must agree."]
    FILTER_CNT_7,
}
impl FILTER_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTER_CNTW::FILTER_CNT_0 => 0,
            FILTER_CNTW::FILTER_CNT_1 => 1,
            FILTER_CNTW::FILTER_CNT_2 => 2,
            FILTER_CNTW::FILTER_CNT_3 => 3,
            FILTER_CNTW::FILTER_CNT_4 => 4,
            FILTER_CNTW::FILTER_CNT_5 => 5,
            FILTER_CNTW::FILTER_CNT_6 => 6,
            FILTER_CNTW::FILTER_CNT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTER_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTER_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTER_CNTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    #[inline]
    pub fn filter_cnt_0(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_0)
    }
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    #[inline]
    pub fn filter_cnt_1(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_1)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline]
    pub fn filter_cnt_2(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_2)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline]
    pub fn filter_cnt_3(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_3)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline]
    pub fn filter_cnt_4(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_4)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline]
    pub fn filter_cnt_5(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_5)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline]
    pub fn filter_cnt_6(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_6)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline]
    pub fn filter_cnt_7(self) -> &'a mut W {
        self.variant(FILTER_CNTW::FILTER_CNT_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline]
    pub fn hystctr(&self) -> HYSTCTRR {
        HYSTCTRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline]
    pub fn filter_cnt(&self) -> FILTER_CNTR {
        FILTER_CNTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline]
    pub fn hystctr(&mut self) -> _HYSTCTRW {
        _HYSTCTRW { w: self }
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline]
    pub fn filter_cnt(&mut self) -> _FILTER_CNTW {
        _FILTER_CNTW { w: self }
    }
}
