#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIGEON_1_2 {
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
#[doc = "Possible values of the field `SIG_LOGIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIG_LOGICR {
    #[doc = "No logic operation"]
    DIS,
    #[doc = "sigout = sig_another AND this_sig"]
    AND,
    #[doc = "sigout = sig_another OR this_sig"]
    OR,
    #[doc = "mask = sig_another AND other_masks"]
    MASK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIG_LOGICR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIG_LOGICR::DIS => 0,
            SIG_LOGICR::AND => 1,
            SIG_LOGICR::OR => 2,
            SIG_LOGICR::MASK => 3,
            SIG_LOGICR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIG_LOGICR {
        match value {
            0 => SIG_LOGICR::DIS,
            1 => SIG_LOGICR::AND,
            2 => SIG_LOGICR::OR,
            3 => SIG_LOGICR::MASK,
            i => SIG_LOGICR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == SIG_LOGICR::DIS
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline]
    pub fn is_and(&self) -> bool {
        *self == SIG_LOGICR::AND
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline]
    pub fn is_or(&self) -> bool {
        *self == SIG_LOGICR::OR
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline]
    pub fn is_mask(&self) -> bool {
        *self == SIG_LOGICR::MASK
    }
}
#[doc = "Possible values of the field `SIG_ANOTHER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIG_ANOTHERR {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIG_ANOTHERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIG_ANOTHERR::CLEAR_USING_MASK => 0,
            SIG_ANOTHERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIG_ANOTHERR {
        match value {
            0 => SIG_ANOTHERR::CLEAR_USING_MASK,
            i => SIG_ANOTHERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_USING_MASK`"]
    #[inline]
    pub fn is_clear_using_mask(&self) -> bool {
        *self == SIG_ANOTHERR::CLEAR_USING_MASK
    }
}
#[doc = "Values that can be written to the field `SIG_LOGIC`"]
pub enum SIG_LOGICW {
    #[doc = "No logic operation"]
    DIS,
    #[doc = "sigout = sig_another AND this_sig"]
    AND,
    #[doc = "sigout = sig_another OR this_sig"]
    OR,
    #[doc = "mask = sig_another AND other_masks"]
    MASK,
}
impl SIG_LOGICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIG_LOGICW::DIS => 0,
            SIG_LOGICW::AND => 1,
            SIG_LOGICW::OR => 2,
            SIG_LOGICW::MASK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIG_LOGICW<'a> {
    w: &'a mut W,
}
impl<'a> _SIG_LOGICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIG_LOGICW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No logic operation"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(SIG_LOGICW::DIS)
    }
    #[doc = "sigout = sig_another AND this_sig"]
    #[inline]
    pub fn and(self) -> &'a mut W {
        self.variant(SIG_LOGICW::AND)
    }
    #[doc = "sigout = sig_another OR this_sig"]
    #[inline]
    pub fn or(self) -> &'a mut W {
        self.variant(SIG_LOGICW::OR)
    }
    #[doc = "mask = sig_another AND other_masks"]
    #[inline]
    pub fn mask(self) -> &'a mut W {
        self.variant(SIG_LOGICW::MASK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIG_ANOTHER`"]
pub enum SIG_ANOTHERW {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK,
}
impl SIG_ANOTHERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIG_ANOTHERW::CLEAR_USING_MASK => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIG_ANOTHERW<'a> {
    w: &'a mut W,
}
impl<'a> _SIG_ANOTHERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIG_ANOTHERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Keep active until mask off"]
    #[inline]
    pub fn clear_using_mask(self) -> &'a mut W {
        self.variant(SIG_ANOTHERW::CLEAR_USING_MASK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline]
    pub fn sig_logic(&self) -> SIG_LOGICR {
        SIG_LOGICR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:8 - Select another signal for logic operation or as mask or counter tick event"]
    #[inline]
    pub fn sig_another(&self) -> SIG_ANOTHERR {
        SIG_ANOTHERR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline]
    pub fn sig_logic(&mut self) -> _SIG_LOGICW {
        _SIG_LOGICW { w: self }
    }
    #[doc = "Bits 4:8 - Select another signal for logic operation or as mask or counter tick event"]
    #[inline]
    pub fn sig_another(&mut self) -> _SIG_ANOTHERW {
        _SIG_ANOTHERW { w: self }
    }
}
