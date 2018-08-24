#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPTIMER0CTRL {
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
#[doc = r" Value of the field"]
pub struct GPTCNTR {
    bits: u32,
}
impl GPTCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `GPTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPTMODER {
    #[doc = "One Shot Mode"]
    GPTMODE_0,
    #[doc = "Repeat Mode"]
    GPTMODE_1,
}
impl GPTMODER {
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
            GPTMODER::GPTMODE_0 => false,
            GPTMODER::GPTMODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPTMODER {
        match value {
            false => GPTMODER::GPTMODE_0,
            true => GPTMODER::GPTMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTMODE_0`"]
    #[inline]
    pub fn is_gptmode_0(&self) -> bool {
        *self == GPTMODER::GPTMODE_0
    }
    #[doc = "Checks if the value of the field is `GPTMODE_1`"]
    #[inline]
    pub fn is_gptmode_1(&self) -> bool {
        *self == GPTMODER::GPTMODE_1
    }
}
#[doc = "Possible values of the field `GPTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPTRSTR {
    #[doc = "No action"]
    GPTRST_0,
    #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
    GPTRST_1,
}
impl GPTRSTR {
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
            GPTRSTR::GPTRST_0 => false,
            GPTRSTR::GPTRST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPTRSTR {
        match value {
            false => GPTRSTR::GPTRST_0,
            true => GPTRSTR::GPTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTRST_0`"]
    #[inline]
    pub fn is_gptrst_0(&self) -> bool {
        *self == GPTRSTR::GPTRST_0
    }
    #[doc = "Checks if the value of the field is `GPTRST_1`"]
    #[inline]
    pub fn is_gptrst_1(&self) -> bool {
        *self == GPTRSTR::GPTRST_1
    }
}
#[doc = "Possible values of the field `GPTRUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPTRUNR {
    #[doc = "Stop counting"]
    GPTRUN_0,
    #[doc = "Run"]
    GPTRUN_1,
}
impl GPTRUNR {
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
            GPTRUNR::GPTRUN_0 => false,
            GPTRUNR::GPTRUN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPTRUNR {
        match value {
            false => GPTRUNR::GPTRUN_0,
            true => GPTRUNR::GPTRUN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTRUN_0`"]
    #[inline]
    pub fn is_gptrun_0(&self) -> bool {
        *self == GPTRUNR::GPTRUN_0
    }
    #[doc = "Checks if the value of the field is `GPTRUN_1`"]
    #[inline]
    pub fn is_gptrun_1(&self) -> bool {
        *self == GPTRUNR::GPTRUN_1
    }
}
#[doc = r" Proxy"]
pub struct _GPTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPTCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPTMODE`"]
pub enum GPTMODEW {
    #[doc = "One Shot Mode"]
    GPTMODE_0,
    #[doc = "Repeat Mode"]
    GPTMODE_1,
}
impl GPTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPTMODEW::GPTMODE_0 => false,
            GPTMODEW::GPTMODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _GPTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPTMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One Shot Mode"]
    #[inline]
    pub fn gptmode_0(self) -> &'a mut W {
        self.variant(GPTMODEW::GPTMODE_0)
    }
    #[doc = "Repeat Mode"]
    #[inline]
    pub fn gptmode_1(self) -> &'a mut W {
        self.variant(GPTMODEW::GPTMODE_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPTRST`"]
pub enum GPTRSTW {
    #[doc = "No action"]
    GPTRST_0,
    #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
    GPTRST_1,
}
impl GPTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPTRSTW::GPTRST_0 => false,
            GPTRSTW::GPTRST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn gptrst_0(self) -> &'a mut W {
        self.variant(GPTRSTW::GPTRST_0)
    }
    #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
    #[inline]
    pub fn gptrst_1(self) -> &'a mut W {
        self.variant(GPTRSTW::GPTRST_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPTRUN`"]
pub enum GPTRUNW {
    #[doc = "Stop counting"]
    GPTRUN_0,
    #[doc = "Run"]
    GPTRUN_1,
}
impl GPTRUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPTRUNW::GPTRUN_0 => false,
            GPTRUNW::GPTRUN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPTRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPTRUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPTRUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop counting"]
    #[inline]
    pub fn gptrun_0(self) -> &'a mut W {
        self.variant(GPTRUNW::GPTRUN_0)
    }
    #[doc = "Run"]
    #[inline]
    pub fn gptrun_1(self) -> &'a mut W {
        self.variant(GPTRUNW::GPTRUN_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:23 - General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline]
    pub fn gptcnt(&self) -> GPTCNTR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        GPTCNTR { bits }
    }
    #[doc = "Bit 24 - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again"]
    #[inline]
    pub fn gptmode(&self) -> GPTMODER {
        GPTMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - General Purpose Timer Reset"]
    #[inline]
    pub fn gptrst(&self) -> GPTRSTR {
        GPTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline]
    pub fn gptrun(&self) -> GPTRUNR {
        GPTRUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:23 - General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline]
    pub fn gptcnt(&mut self) -> _GPTCNTW {
        _GPTCNTW { w: self }
    }
    #[doc = "Bit 24 - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again"]
    #[inline]
    pub fn gptmode(&mut self) -> _GPTMODEW {
        _GPTMODEW { w: self }
    }
    #[doc = "Bit 30 - General Purpose Timer Reset"]
    #[inline]
    pub fn gptrst(&mut self) -> _GPTRSTW {
        _GPTRSTW { w: self }
    }
    #[doc = "Bit 31 - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline]
    pub fn gptrun(&mut self) -> _GPTRUNW {
        _GPTRUNW { w: self }
    }
}
