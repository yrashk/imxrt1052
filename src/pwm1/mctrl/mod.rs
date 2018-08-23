#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::MCTRL {
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
#[doc = "Possible values of the field `LDOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOKR {
    #[doc = "Do not load new values."]
    LDOK_0,
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    LDOK_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LDOKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDOKR::LDOK_0 => 0,
            LDOKR::LDOK_1 => 1,
            LDOKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDOKR {
        match value {
            0 => LDOKR::LDOK_0,
            1 => LDOKR::LDOK_1,
            i => LDOKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LDOK_0`"]
    #[inline]
    pub fn is_ldok_0(&self) -> bool {
        *self == LDOKR::LDOK_0
    }
    #[doc = "Checks if the value of the field is `LDOK_1`"]
    #[inline]
    pub fn is_ldok_1(&self) -> bool {
        *self == LDOKR::LDOK_1
    }
}
#[doc = "Possible values of the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNR {
    #[doc = "PWM generator is disabled in the corresponding submodule."]
    RUN_0,
    #[doc = "PWM generator is enabled in the corresponding submodule."]
    RUN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RUNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RUNR::RUN_0 => 0,
            RUNR::RUN_1 => 1,
            RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RUNR {
        match value {
            0 => RUNR::RUN_0,
            1 => RUNR::RUN_1,
            i => RUNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RUN_0`"]
    #[inline]
    pub fn is_run_0(&self) -> bool {
        *self == RUNR::RUN_0
    }
    #[doc = "Checks if the value of the field is `RUN_1`"]
    #[inline]
    pub fn is_run_1(&self) -> bool {
        *self == RUNR::RUN_1
    }
}
#[doc = "Possible values of the field `IPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPOLR {
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    IPOL_0,
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    IPOL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IPOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IPOLR::IPOL_0 => 0,
            IPOLR::IPOL_1 => 1,
            IPOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IPOLR {
        match value {
            0 => IPOLR::IPOL_0,
            1 => IPOLR::IPOL_1,
            i => IPOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IPOL_0`"]
    #[inline]
    pub fn is_ipol_0(&self) -> bool {
        *self == IPOLR::IPOL_0
    }
    #[doc = "Checks if the value of the field is `IPOL_1`"]
    #[inline]
    pub fn is_ipol_1(&self) -> bool {
        *self == IPOLR::IPOL_1
    }
}
#[doc = "Values that can be written to the field `LDOK`"]
pub enum LDOKW {
    #[doc = "Do not load new values."]
    LDOK_0,
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    LDOK_1,
}
impl LDOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDOKW::LDOK_0 => 0,
            LDOKW::LDOK_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDOKW<'a> {
    w: &'a mut W,
}
impl<'a> _LDOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDOKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Do not load new values."]
    #[inline]
    pub fn ldok_0(self) -> &'a mut W {
        self.variant(LDOKW::LDOK_0)
    }
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    #[inline]
    pub fn ldok_1(self) -> &'a mut W {
        self.variant(LDOKW::LDOK_1)
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
#[doc = r" Proxy"]
pub struct _CLDOKW<'a> {
    w: &'a mut W,
}
impl<'a> _CLDOKW<'a> {
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
#[doc = "Values that can be written to the field `RUN`"]
pub enum RUNW {
    #[doc = "PWM generator is disabled in the corresponding submodule."]
    RUN_0,
    #[doc = "PWM generator is enabled in the corresponding submodule."]
    RUN_1,
}
impl RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RUNW::RUN_0 => 0,
            RUNW::RUN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM generator is disabled in the corresponding submodule."]
    #[inline]
    pub fn run_0(self) -> &'a mut W {
        self.variant(RUNW::RUN_0)
    }
    #[doc = "PWM generator is enabled in the corresponding submodule."]
    #[inline]
    pub fn run_1(self) -> &'a mut W {
        self.variant(RUNW::RUN_1)
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
#[doc = "Values that can be written to the field `IPOL`"]
pub enum IPOLW {
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    IPOL_0,
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    IPOL_1,
}
impl IPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IPOLW::IPOL_0 => 0,
            IPOLW::IPOL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _IPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    #[inline]
    pub fn ipol_0(self) -> &'a mut W {
        self.variant(IPOLW::IPOL_0)
    }
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    #[inline]
    pub fn ipol_1(self) -> &'a mut W {
        self.variant(IPOLW::IPOL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Load Okay"]
    #[inline]
    pub fn ldok(&self) -> LDOKR {
        LDOKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:11 - Run"]
    #[inline]
    pub fn run(&self) -> RUNR {
        RUNR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 12:15 - Current Polarity"]
    #[inline]
    pub fn ipol(&self) -> IPOLR {
        IPOLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Load Okay"]
    #[inline]
    pub fn ldok(&mut self) -> _LDOKW {
        _LDOKW { w: self }
    }
    #[doc = "Bits 4:7 - Clear Load Okay"]
    #[inline]
    pub fn cldok(&mut self) -> _CLDOKW {
        _CLDOKW { w: self }
    }
    #[doc = "Bits 8:11 - Run"]
    #[inline]
    pub fn run(&mut self) -> _RUNW {
        _RUNW { w: self }
    }
    #[doc = "Bits 12:15 - Current Polarity"]
    #[inline]
    pub fn ipol(&mut self) -> _IPOLW {
        _IPOLW { w: self }
    }
}
