#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCSR {
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
#[doc = "Possible values of the field `TDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRER {
    #[doc = "DMA request is disabled"]
    TDRE_0,
    #[doc = "DMA request is enabled"]
    TDRE_1,
}
impl TDRER {
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
            TDRER::TDRE_0 => false,
            TDRER::TDRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRER {
        match value {
            false => TDRER::TDRE_0,
            true => TDRER::TDRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDRE_0`"]
    #[inline]
    pub fn is_tdre_0(&self) -> bool {
        *self == TDRER::TDRE_0
    }
    #[doc = "Checks if the value of the field is `TDRE_1`"]
    #[inline]
    pub fn is_tdre_1(&self) -> bool {
        *self == TDRER::TDRE_1
    }
}
#[doc = "Possible values of the field `TMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMODER {
    #[doc = "Timer Channel is disabled."]
    TMODE_0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMODE_1,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMODE_2,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMODE_3,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMODE_4,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMODE_5,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMODE_6,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMODE_7,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMODE_9,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMODE_10,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMODE_14,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMODE_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMODER::TMODE_0 => 0,
            TMODER::TMODE_1 => 1,
            TMODER::TMODE_2 => 2,
            TMODER::TMODE_3 => 3,
            TMODER::TMODE_4 => 4,
            TMODER::TMODE_5 => 5,
            TMODER::TMODE_6 => 6,
            TMODER::TMODE_7 => 7,
            TMODER::TMODE_9 => 9,
            TMODER::TMODE_10 => 10,
            TMODER::TMODE_14 => 14,
            TMODER::TMODE_15 => 15,
            TMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMODER {
        match value {
            0 => TMODER::TMODE_0,
            1 => TMODER::TMODE_1,
            2 => TMODER::TMODE_2,
            3 => TMODER::TMODE_3,
            4 => TMODER::TMODE_4,
            5 => TMODER::TMODE_5,
            6 => TMODER::TMODE_6,
            7 => TMODER::TMODE_7,
            9 => TMODER::TMODE_9,
            10 => TMODER::TMODE_10,
            14 => TMODER::TMODE_14,
            15 => TMODER::TMODE_15,
            i => TMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMODE_0`"]
    #[inline]
    pub fn is_tmode_0(&self) -> bool {
        *self == TMODER::TMODE_0
    }
    #[doc = "Checks if the value of the field is `TMODE_1`"]
    #[inline]
    pub fn is_tmode_1(&self) -> bool {
        *self == TMODER::TMODE_1
    }
    #[doc = "Checks if the value of the field is `TMODE_2`"]
    #[inline]
    pub fn is_tmode_2(&self) -> bool {
        *self == TMODER::TMODE_2
    }
    #[doc = "Checks if the value of the field is `TMODE_3`"]
    #[inline]
    pub fn is_tmode_3(&self) -> bool {
        *self == TMODER::TMODE_3
    }
    #[doc = "Checks if the value of the field is `TMODE_4`"]
    #[inline]
    pub fn is_tmode_4(&self) -> bool {
        *self == TMODER::TMODE_4
    }
    #[doc = "Checks if the value of the field is `TMODE_5`"]
    #[inline]
    pub fn is_tmode_5(&self) -> bool {
        *self == TMODER::TMODE_5
    }
    #[doc = "Checks if the value of the field is `TMODE_6`"]
    #[inline]
    pub fn is_tmode_6(&self) -> bool {
        *self == TMODER::TMODE_6
    }
    #[doc = "Checks if the value of the field is `TMODE_7`"]
    #[inline]
    pub fn is_tmode_7(&self) -> bool {
        *self == TMODER::TMODE_7
    }
    #[doc = "Checks if the value of the field is `TMODE_9`"]
    #[inline]
    pub fn is_tmode_9(&self) -> bool {
        *self == TMODER::TMODE_9
    }
    #[doc = "Checks if the value of the field is `TMODE_10`"]
    #[inline]
    pub fn is_tmode_10(&self) -> bool {
        *self == TMODER::TMODE_10
    }
    #[doc = "Checks if the value of the field is `TMODE_14`"]
    #[inline]
    pub fn is_tmode_14(&self) -> bool {
        *self == TMODER::TMODE_14
    }
    #[doc = "Checks if the value of the field is `TMODE_15`"]
    #[inline]
    pub fn is_tmode_15(&self) -> bool {
        *self == TMODER::TMODE_15
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Interrupt is disabled"]
    TIE_0,
    #[doc = "Interrupt is enabled"]
    TIE_1,
}
impl TIER {
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
            TIER::TIE_0 => false,
            TIER::TIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::TIE_0,
            true => TIER::TIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE_0`"]
    #[inline]
    pub fn is_tie_0(&self) -> bool {
        *self == TIER::TIE_0
    }
    #[doc = "Checks if the value of the field is `TIE_1`"]
    #[inline]
    pub fn is_tie_1(&self) -> bool {
        *self == TIER::TIE_1
    }
}
#[doc = "Possible values of the field `TF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFR {
    #[doc = "Input Capture or Output Compare has not occurred."]
    TF_0,
    #[doc = "Input Capture or Output Compare has occurred."]
    TF_1,
}
impl TFR {
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
            TFR::TF_0 => false,
            TFR::TF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFR {
        match value {
            false => TFR::TF_0,
            true => TFR::TF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF_0`"]
    #[inline]
    pub fn is_tf_0(&self) -> bool {
        *self == TFR::TF_0
    }
    #[doc = "Checks if the value of the field is `TF_1`"]
    #[inline]
    pub fn is_tf_1(&self) -> bool {
        *self == TFR::TF_1
    }
}
#[doc = "Possible values of the field `TPWC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPWCR {
    #[doc = "Pulse width is one 1588-clock cycle."]
    TPWC_0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    TPWC_1,
    #[doc = "Pulse width is three 1588-clock cycles."]
    TPWC_2,
    #[doc = "Pulse width is four 1588-clock cycles."]
    TPWC_3,
    #[doc = "Pulse width is 32 1588-clock cycles."]
    TPWC_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TPWCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPWCR::TPWC_0 => 0,
            TPWCR::TPWC_1 => 1,
            TPWCR::TPWC_2 => 2,
            TPWCR::TPWC_3 => 3,
            TPWCR::TPWC_31 => 31,
            TPWCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPWCR {
        match value {
            0 => TPWCR::TPWC_0,
            1 => TPWCR::TPWC_1,
            2 => TPWCR::TPWC_2,
            3 => TPWCR::TPWC_3,
            31 => TPWCR::TPWC_31,
            i => TPWCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TPWC_0`"]
    #[inline]
    pub fn is_tpwc_0(&self) -> bool {
        *self == TPWCR::TPWC_0
    }
    #[doc = "Checks if the value of the field is `TPWC_1`"]
    #[inline]
    pub fn is_tpwc_1(&self) -> bool {
        *self == TPWCR::TPWC_1
    }
    #[doc = "Checks if the value of the field is `TPWC_2`"]
    #[inline]
    pub fn is_tpwc_2(&self) -> bool {
        *self == TPWCR::TPWC_2
    }
    #[doc = "Checks if the value of the field is `TPWC_3`"]
    #[inline]
    pub fn is_tpwc_3(&self) -> bool {
        *self == TPWCR::TPWC_3
    }
    #[doc = "Checks if the value of the field is `TPWC_31`"]
    #[inline]
    pub fn is_tpwc_31(&self) -> bool {
        *self == TPWCR::TPWC_31
    }
}
#[doc = "Values that can be written to the field `TDRE`"]
pub enum TDREW {
    #[doc = "DMA request is disabled"]
    TDRE_0,
    #[doc = "DMA request is enabled"]
    TDRE_1,
}
impl TDREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDREW::TDRE_0 => false,
            TDREW::TDRE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDREW<'a> {
    w: &'a mut W,
}
impl<'a> _TDREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request is disabled"]
    #[inline]
    pub fn tdre_0(self) -> &'a mut W {
        self.variant(TDREW::TDRE_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline]
    pub fn tdre_1(self) -> &'a mut W {
        self.variant(TDREW::TDRE_1)
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
#[doc = "Values that can be written to the field `TMODE`"]
pub enum TMODEW {
    #[doc = "Timer Channel is disabled."]
    TMODE_0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMODE_1,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMODE_2,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMODE_3,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMODE_4,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMODE_5,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMODE_6,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMODE_7,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMODE_9,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMODE_10,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMODE_14,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMODE_15,
}
impl TMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMODEW::TMODE_0 => 0,
            TMODEW::TMODE_1 => 1,
            TMODEW::TMODE_2 => 2,
            TMODEW::TMODE_3 => 3,
            TMODEW::TMODE_4 => 4,
            TMODEW::TMODE_5 => 5,
            TMODEW::TMODE_6 => 6,
            TMODEW::TMODE_7 => 7,
            TMODEW::TMODE_9 => 9,
            TMODEW::TMODE_10 => 10,
            TMODEW::TMODE_14 => 14,
            TMODEW::TMODE_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer Channel is disabled."]
    #[inline]
    pub fn tmode_0(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_0)
    }
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    #[inline]
    pub fn tmode_1(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_1)
    }
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    #[inline]
    pub fn tmode_2(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_2)
    }
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    #[inline]
    pub fn tmode_3(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_3)
    }
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    #[inline]
    pub fn tmode_4(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_4)
    }
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    #[inline]
    pub fn tmode_5(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_5)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    #[inline]
    pub fn tmode_6(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_6)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    #[inline]
    pub fn tmode_7(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_7)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    #[inline]
    pub fn tmode_9(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_9)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    #[inline]
    pub fn tmode_10(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_10)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    #[inline]
    pub fn tmode_14(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_14)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    #[inline]
    pub fn tmode_15(self) -> &'a mut W {
        self.variant(TMODEW::TMODE_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Interrupt is disabled"]
    TIE_0,
    #[doc = "Interrupt is enabled"]
    TIE_1,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::TIE_0 => false,
            TIEW::TIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline]
    pub fn tie_0(self) -> &'a mut W {
        self.variant(TIEW::TIE_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline]
    pub fn tie_1(self) -> &'a mut W {
        self.variant(TIEW::TIE_1)
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
#[doc = "Values that can be written to the field `TF`"]
pub enum TFW {
    #[doc = "Input Capture or Output Compare has not occurred."]
    TF_0,
    #[doc = "Input Capture or Output Compare has occurred."]
    TF_1,
}
impl TFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFW::TF_0 => false,
            TFW::TF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFW<'a> {
    w: &'a mut W,
}
impl<'a> _TFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input Capture or Output Compare has not occurred."]
    #[inline]
    pub fn tf_0(self) -> &'a mut W {
        self.variant(TFW::TF_0)
    }
    #[doc = "Input Capture or Output Compare has occurred."]
    #[inline]
    pub fn tf_1(self) -> &'a mut W {
        self.variant(TFW::TF_1)
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
#[doc = "Values that can be written to the field `TPWC`"]
pub enum TPWCW {
    #[doc = "Pulse width is one 1588-clock cycle."]
    TPWC_0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    TPWC_1,
    #[doc = "Pulse width is three 1588-clock cycles."]
    TPWC_2,
    #[doc = "Pulse width is four 1588-clock cycles."]
    TPWC_3,
    #[doc = "Pulse width is 32 1588-clock cycles."]
    TPWC_31,
}
impl TPWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPWCW::TPWC_0 => 0,
            TPWCW::TPWC_1 => 1,
            TPWCW::TPWC_2 => 2,
            TPWCW::TPWC_3 => 3,
            TPWCW::TPWC_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPWCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPWCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Pulse width is one 1588-clock cycle."]
    #[inline]
    pub fn tpwc_0(self) -> &'a mut W {
        self.variant(TPWCW::TPWC_0)
    }
    #[doc = "Pulse width is two 1588-clock cycles."]
    #[inline]
    pub fn tpwc_1(self) -> &'a mut W {
        self.variant(TPWCW::TPWC_1)
    }
    #[doc = "Pulse width is three 1588-clock cycles."]
    #[inline]
    pub fn tpwc_2(self) -> &'a mut W {
        self.variant(TPWCW::TPWC_2)
    }
    #[doc = "Pulse width is four 1588-clock cycles."]
    #[inline]
    pub fn tpwc_3(self) -> &'a mut W {
        self.variant(TPWCW::TPWC_3)
    }
    #[doc = "Pulse width is 32 1588-clock cycles."]
    #[inline]
    pub fn tpwc_31(self) -> &'a mut W {
        self.variant(TPWCW::TPWC_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline]
    pub fn tdre(&self) -> TDRER {
        TDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline]
    pub fn tmode(&self) -> TMODER {
        TMODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline]
    pub fn tf(&self) -> TFR {
        TFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:15 - Timer PulseWidth Control"]
    #[inline]
    pub fn tpwc(&self) -> TPWCR {
        TPWCR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline]
    pub fn tdre(&mut self) -> _TDREW {
        _TDREW { w: self }
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline]
    pub fn tmode(&mut self) -> _TMODEW {
        _TMODEW { w: self }
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline]
    pub fn tf(&mut self) -> _TFW {
        _TFW { w: self }
    }
    #[doc = "Bits 11:15 - Timer PulseWidth Control"]
    #[inline]
    pub fn tpwc(&mut self) -> _TPWCW {
        _TPWCW { w: self }
    }
}
