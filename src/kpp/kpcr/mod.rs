#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::KPCR {
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
#[doc = "Possible values of the field `KRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KRER {
    #[doc = "Row is not included in the keypad key press detect."]
    KRE_0,
    #[doc = "Row is included in the keypad key press detect."]
    KRE_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KRER::KRE_0 => 0,
            KRER::KRE_1 => 1,
            KRER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KRER {
        match value {
            0 => KRER::KRE_0,
            1 => KRER::KRE_1,
            i => KRER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KRE_0`"]
    #[inline]
    pub fn is_kre_0(&self) -> bool {
        *self == KRER::KRE_0
    }
    #[doc = "Checks if the value of the field is `KRE_1`"]
    #[inline]
    pub fn is_kre_1(&self) -> bool {
        *self == KRER::KRE_1
    }
}
#[doc = "Possible values of the field `KCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KCOR {
    #[doc = "Column strobe output is totem pole drive."]
    TOTEM_POLE,
    #[doc = "Column strobe output is open drain."]
    OPEN_DRAIN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KCOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KCOR::TOTEM_POLE => 0,
            KCOR::OPEN_DRAIN => 1,
            KCOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KCOR {
        match value {
            0 => KCOR::TOTEM_POLE,
            1 => KCOR::OPEN_DRAIN,
            i => KCOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TOTEM_POLE`"]
    #[inline]
    pub fn is_totem_pole(&self) -> bool {
        *self == KCOR::TOTEM_POLE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == KCOR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `KRE`"]
pub enum KREW {
    #[doc = "Row is not included in the keypad key press detect."]
    KRE_0,
    #[doc = "Row is included in the keypad key press detect."]
    KRE_1,
}
impl KREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KREW::KRE_0 => 0,
            KREW::KRE_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KREW<'a> {
    w: &'a mut W,
}
impl<'a> _KREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Row is not included in the keypad key press detect."]
    #[inline]
    pub fn kre_0(self) -> &'a mut W {
        self.variant(KREW::KRE_0)
    }
    #[doc = "Row is included in the keypad key press detect."]
    #[inline]
    pub fn kre_1(self) -> &'a mut W {
        self.variant(KREW::KRE_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KCO`"]
pub enum KCOW {
    #[doc = "Column strobe output is totem pole drive."]
    TOTEM_POLE,
    #[doc = "Column strobe output is open drain."]
    OPEN_DRAIN,
}
impl KCOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KCOW::TOTEM_POLE => 0,
            KCOW::OPEN_DRAIN => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KCOW<'a> {
    w: &'a mut W,
}
impl<'a> _KCOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KCOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Column strobe output is totem pole drive."]
    #[inline]
    pub fn totem_pole(self) -> &'a mut W {
        self.variant(KCOW::TOTEM_POLE)
    }
    #[doc = "Column strobe output is open drain."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(KCOW::OPEN_DRAIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:7 - Keypad Row Enable"]
    #[inline]
    pub fn kre(&self) -> KRER {
        KRER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:15 - Keypad Column Strobe Open-Drain Enable"]
    #[inline]
    pub fn kco(&self) -> KCOR {
        KCOR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Keypad Row Enable"]
    #[inline]
    pub fn kre(&mut self) -> _KREW {
        _KREW { w: self }
    }
    #[doc = "Bits 8:15 - Keypad Column Strobe Open-Drain Enable"]
    #[inline]
    pub fn kco(&mut self) -> _KCOW {
        _KCOW { w: self }
    }
}
