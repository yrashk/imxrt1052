#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL_SYS_SS {
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
pub struct STEPR {
    bits: u16,
}
impl STEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Spread spectrum modulation disabled"]
    ENABLE_0,
    #[doc = "Soread spectrum modulation enabled"]
    ENABLE_1,
}
impl ENABLER {
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
            ENABLER::ENABLE_0 => false,
            ENABLER::ENABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::ENABLE_0,
            true => ENABLER::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLER::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLER::ENABLE_1
    }
}
#[doc = r" Value of the field"]
pub struct STOPR {
    bits: u16,
}
impl STOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _STEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "Spread spectrum modulation disabled"]
    ENABLE_0,
    #[doc = "Soread spectrum modulation enabled"]
    ENABLE_1,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::ENABLE_0 => false,
            ENABLEW::ENABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Spread spectrum modulation disabled"]
    #[inline]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_0)
    }
    #[doc = "Soread spectrum modulation enabled"]
    #[inline]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:14 - Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM[B]*24MHz."]
    #[inline]
    pub fn step(&self) -> STEPR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        STEPR { bits }
    }
    #[doc = "Bit 15 - Enable bit"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM[B]*24MHz."]
    #[inline]
    pub fn stop(&self) -> STOPR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        STOPR { bits }
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
    #[doc = "Bits 0:14 - Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM[B]*24MHz."]
    #[inline]
    pub fn step(&mut self) -> _STEPW {
        _STEPW { w: self }
    }
    #[doc = "Bit 15 - Enable bit"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 16:31 - Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM[B]*24MHz."]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
}
