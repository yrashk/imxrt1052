#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMCR2 {
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
pub struct SRRCR {
    bits: u8,
}
impl SRRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REF2REFR {
    bits: u8,
}
impl REF2REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ACT2ACTR {
    bits: u8,
}
impl ACT2ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ITO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITOR {
    #[doc = "IDLE timeout period is 256*Prescale period."]
    ITO_0,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_1,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_2,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_3,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_4,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_5,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_6,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_7,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_8,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ITOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ITOR::ITO_0 => 0,
            ITOR::ITO_1 => 1,
            ITOR::ITO_2 => 2,
            ITOR::ITO_3 => 3,
            ITOR::ITO_4 => 4,
            ITOR::ITO_5 => 5,
            ITOR::ITO_6 => 6,
            ITOR::ITO_7 => 7,
            ITOR::ITO_8 => 8,
            ITOR::ITO_9 => 9,
            ITOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ITOR {
        match value {
            0 => ITOR::ITO_0,
            1 => ITOR::ITO_1,
            2 => ITOR::ITO_2,
            3 => ITOR::ITO_3,
            4 => ITOR::ITO_4,
            5 => ITOR::ITO_5,
            6 => ITOR::ITO_6,
            7 => ITOR::ITO_7,
            8 => ITOR::ITO_8,
            9 => ITOR::ITO_9,
            i => ITOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ITO_0`"]
    #[inline]
    pub fn is_ito_0(&self) -> bool {
        *self == ITOR::ITO_0
    }
    #[doc = "Checks if the value of the field is `ITO_1`"]
    #[inline]
    pub fn is_ito_1(&self) -> bool {
        *self == ITOR::ITO_1
    }
    #[doc = "Checks if the value of the field is `ITO_2`"]
    #[inline]
    pub fn is_ito_2(&self) -> bool {
        *self == ITOR::ITO_2
    }
    #[doc = "Checks if the value of the field is `ITO_3`"]
    #[inline]
    pub fn is_ito_3(&self) -> bool {
        *self == ITOR::ITO_3
    }
    #[doc = "Checks if the value of the field is `ITO_4`"]
    #[inline]
    pub fn is_ito_4(&self) -> bool {
        *self == ITOR::ITO_4
    }
    #[doc = "Checks if the value of the field is `ITO_5`"]
    #[inline]
    pub fn is_ito_5(&self) -> bool {
        *self == ITOR::ITO_5
    }
    #[doc = "Checks if the value of the field is `ITO_6`"]
    #[inline]
    pub fn is_ito_6(&self) -> bool {
        *self == ITOR::ITO_6
    }
    #[doc = "Checks if the value of the field is `ITO_7`"]
    #[inline]
    pub fn is_ito_7(&self) -> bool {
        *self == ITOR::ITO_7
    }
    #[doc = "Checks if the value of the field is `ITO_8`"]
    #[inline]
    pub fn is_ito_8(&self) -> bool {
        *self == ITOR::ITO_8
    }
    #[doc = "Checks if the value of the field is `ITO_9`"]
    #[inline]
    pub fn is_ito_9(&self) -> bool {
        *self == ITOR::ITO_9
    }
}
#[doc = r" Proxy"]
pub struct _SRRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REF2REFW<'a> {
    w: &'a mut W,
}
impl<'a> _REF2REFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACT2ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _ACT2ACTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ITO`"]
pub enum ITOW {
    #[doc = "IDLE timeout period is 256*Prescale period."]
    ITO_0,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_1,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_2,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_3,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_4,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_5,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_6,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_7,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_8,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    ITO_9,
}
impl ITOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ITOW::ITO_0 => 0,
            ITOW::ITO_1 => 1,
            ITOW::ITO_2 => 2,
            ITOW::ITO_3 => 3,
            ITOW::ITO_4 => 4,
            ITOW::ITO_5 => 5,
            ITOW::ITO_6 => 6,
            ITOW::ITO_7 => 7,
            ITOW::ITO_8 => 8,
            ITOW::ITO_9 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITOW<'a> {
    w: &'a mut W,
}
impl<'a> _ITOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IDLE timeout period is 256*Prescale period."]
    #[inline]
    pub fn ito_0(self) -> &'a mut W {
        self.variant(ITOW::ITO_0)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_1(self) -> &'a mut W {
        self.variant(ITOW::ITO_1)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_2(self) -> &'a mut W {
        self.variant(ITOW::ITO_2)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_3(self) -> &'a mut W {
        self.variant(ITOW::ITO_3)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_4(self) -> &'a mut W {
        self.variant(ITOW::ITO_4)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_5(self) -> &'a mut W {
        self.variant(ITOW::ITO_5)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_6(self) -> &'a mut W {
        self.variant(ITOW::ITO_6)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_7(self) -> &'a mut W {
        self.variant(ITOW::ITO_7)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_8(self) -> &'a mut W {
        self.variant(ITOW::ITO_8)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline]
    pub fn ito_9(self) -> &'a mut W {
        self.variant(ITOW::ITO_9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - Self Refresh Recovery time"]
    #[inline]
    pub fn srrc(&self) -> SRRCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRRCR { bits }
    }
    #[doc = "Bits 8:15 - Refresh to Refresh wait time"]
    #[inline]
    pub fn ref2ref(&self) -> REF2REFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REF2REFR { bits }
    }
    #[doc = "Bits 16:23 - ACT to ACT wait time"]
    #[inline]
    pub fn act2act(&self) -> ACT2ACTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACT2ACTR { bits }
    }
    #[doc = "Bits 24:31 - SDRAM Idle timeout"]
    #[inline]
    pub fn ito(&self) -> ITOR {
        ITOR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147487470 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Self Refresh Recovery time"]
    #[inline]
    pub fn srrc(&mut self) -> _SRRCW {
        _SRRCW { w: self }
    }
    #[doc = "Bits 8:15 - Refresh to Refresh wait time"]
    #[inline]
    pub fn ref2ref(&mut self) -> _REF2REFW {
        _REF2REFW { w: self }
    }
    #[doc = "Bits 16:23 - ACT to ACT wait time"]
    #[inline]
    pub fn act2act(&mut self) -> _ACT2ACTW {
        _ACT2ACTW { w: self }
    }
    #[doc = "Bits 24:31 - SDRAM Idle timeout"]
    #[inline]
    pub fn ito(&mut self) -> _ITOW {
        _ITOW { w: self }
    }
}
