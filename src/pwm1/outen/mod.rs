#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::OUTEN {
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
#[doc = "Possible values of the field `PWMX_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMX_ENR {
    #[doc = "PWM_X output disabled."]
    PWMX_EN_0,
    #[doc = "PWM_X output enabled."]
    PWMX_EN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWMX_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMX_ENR::PWMX_EN_0 => 0,
            PWMX_ENR::PWMX_EN_1 => 1,
            PWMX_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMX_ENR {
        match value {
            0 => PWMX_ENR::PWMX_EN_0,
            1 => PWMX_ENR::PWMX_EN_1,
            i => PWMX_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWMX_EN_0`"]
    #[inline]
    pub fn is_pwmx_en_0(&self) -> bool {
        *self == PWMX_ENR::PWMX_EN_0
    }
    #[doc = "Checks if the value of the field is `PWMX_EN_1`"]
    #[inline]
    pub fn is_pwmx_en_1(&self) -> bool {
        *self == PWMX_ENR::PWMX_EN_1
    }
}
#[doc = "Possible values of the field `PWMB_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMB_ENR {
    #[doc = "PWM_B output disabled."]
    PWMB_EN_0,
    #[doc = "PWM_B output enabled."]
    PWMB_EN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWMB_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMB_ENR::PWMB_EN_0 => 0,
            PWMB_ENR::PWMB_EN_1 => 1,
            PWMB_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMB_ENR {
        match value {
            0 => PWMB_ENR::PWMB_EN_0,
            1 => PWMB_ENR::PWMB_EN_1,
            i => PWMB_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWMB_EN_0`"]
    #[inline]
    pub fn is_pwmb_en_0(&self) -> bool {
        *self == PWMB_ENR::PWMB_EN_0
    }
    #[doc = "Checks if the value of the field is `PWMB_EN_1`"]
    #[inline]
    pub fn is_pwmb_en_1(&self) -> bool {
        *self == PWMB_ENR::PWMB_EN_1
    }
}
#[doc = "Possible values of the field `PWMA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMA_ENR {
    #[doc = "PWM_A output disabled."]
    PWMA_EN_0,
    #[doc = "PWM_A output enabled."]
    PWMA_EN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWMA_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMA_ENR::PWMA_EN_0 => 0,
            PWMA_ENR::PWMA_EN_1 => 1,
            PWMA_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMA_ENR {
        match value {
            0 => PWMA_ENR::PWMA_EN_0,
            1 => PWMA_ENR::PWMA_EN_1,
            i => PWMA_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWMA_EN_0`"]
    #[inline]
    pub fn is_pwma_en_0(&self) -> bool {
        *self == PWMA_ENR::PWMA_EN_0
    }
    #[doc = "Checks if the value of the field is `PWMA_EN_1`"]
    #[inline]
    pub fn is_pwma_en_1(&self) -> bool {
        *self == PWMA_ENR::PWMA_EN_1
    }
}
#[doc = "Values that can be written to the field `PWMX_EN`"]
pub enum PWMX_ENW {
    #[doc = "PWM_X output disabled."]
    PWMX_EN_0,
    #[doc = "PWM_X output enabled."]
    PWMX_EN_1,
}
impl PWMX_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMX_ENW::PWMX_EN_0 => 0,
            PWMX_ENW::PWMX_EN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMX_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMX_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM_X output disabled."]
    #[inline]
    pub fn pwmx_en_0(self) -> &'a mut W {
        self.variant(PWMX_ENW::PWMX_EN_0)
    }
    #[doc = "PWM_X output enabled."]
    #[inline]
    pub fn pwmx_en_1(self) -> &'a mut W {
        self.variant(PWMX_ENW::PWMX_EN_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMB_EN`"]
pub enum PWMB_ENW {
    #[doc = "PWM_B output disabled."]
    PWMB_EN_0,
    #[doc = "PWM_B output enabled."]
    PWMB_EN_1,
}
impl PWMB_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMB_ENW::PWMB_EN_0 => 0,
            PWMB_ENW::PWMB_EN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMB_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMB_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMB_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM_B output disabled."]
    #[inline]
    pub fn pwmb_en_0(self) -> &'a mut W {
        self.variant(PWMB_ENW::PWMB_EN_0)
    }
    #[doc = "PWM_B output enabled."]
    #[inline]
    pub fn pwmb_en_1(self) -> &'a mut W {
        self.variant(PWMB_ENW::PWMB_EN_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMA_EN`"]
pub enum PWMA_ENW {
    #[doc = "PWM_A output disabled."]
    PWMA_EN_0,
    #[doc = "PWM_A output enabled."]
    PWMA_EN_1,
}
impl PWMA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMA_ENW::PWMA_EN_0 => 0,
            PWMA_ENW::PWMA_EN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMA_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM_A output disabled."]
    #[inline]
    pub fn pwma_en_0(self) -> &'a mut W {
        self.variant(PWMA_ENW::PWMA_EN_0)
    }
    #[doc = "PWM_A output enabled."]
    #[inline]
    pub fn pwma_en_1(self) -> &'a mut W {
        self.variant(PWMA_ENW::PWMA_EN_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - PWM_X Output Enables"]
    #[inline]
    pub fn pwmx_en(&self) -> PWMX_ENR {
        PWMX_ENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:7 - PWM_B Output Enables"]
    #[inline]
    pub fn pwmb_en(&self) -> PWMB_ENR {
        PWMB_ENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:11 - PWM_A Output Enables"]
    #[inline]
    pub fn pwma_en(&self) -> PWMA_ENR {
        PWMA_ENR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - PWM_X Output Enables"]
    #[inline]
    pub fn pwmx_en(&mut self) -> _PWMX_ENW {
        _PWMX_ENW { w: self }
    }
    #[doc = "Bits 4:7 - PWM_B Output Enables"]
    #[inline]
    pub fn pwmb_en(&mut self) -> _PWMB_ENW {
        _PWMB_ENW { w: self }
    }
    #[doc = "Bits 8:11 - PWM_A Output Enables"]
    #[inline]
    pub fn pwma_en(&mut self) -> _PWMA_ENW {
        _PWMA_ENW { w: self }
    }
}
