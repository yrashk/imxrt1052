#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PR {
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
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "Divide by 1"]
    PRESCALER_0,
    #[doc = "Divide by 2"]
    PRESCALER_1,
    #[doc = "Divide by 4096"]
    PRESCALER_4095,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PRESCALERR::PRESCALER_0 => 0,
            PRESCALERR::PRESCALER_1 => 1,
            PRESCALERR::PRESCALER_4095 => 4095,
            PRESCALERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PRESCALERR {
        match value {
            0 => PRESCALERR::PRESCALER_0,
            1 => PRESCALERR::PRESCALER_1,
            4095 => PRESCALERR::PRESCALER_4095,
            i => PRESCALERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER_0`"]
    #[inline]
    pub fn is_prescaler_0(&self) -> bool {
        *self == PRESCALERR::PRESCALER_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER_1`"]
    #[inline]
    pub fn is_prescaler_1(&self) -> bool {
        *self == PRESCALERR::PRESCALER_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER_4095`"]
    #[inline]
    pub fn is_prescaler_4095(&self) -> bool {
        *self == PRESCALERR::PRESCALER_4095
    }
}
#[doc = "Possible values of the field `PRESCALER24M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER24MR {
    #[doc = "Divide by 1"]
    PRESCALER24M_0,
    #[doc = "Divide by 2"]
    PRESCALER24M_1,
    #[doc = "Divide by 16"]
    PRESCALER24M_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCALER24MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER24MR::PRESCALER24M_0 => 0,
            PRESCALER24MR::PRESCALER24M_1 => 1,
            PRESCALER24MR::PRESCALER24M_15 => 15,
            PRESCALER24MR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER24MR {
        match value {
            0 => PRESCALER24MR::PRESCALER24M_0,
            1 => PRESCALER24MR::PRESCALER24M_1,
            15 => PRESCALER24MR::PRESCALER24M_15,
            i => PRESCALER24MR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_0`"]
    #[inline]
    pub fn is_prescaler24m_0(&self) -> bool {
        *self == PRESCALER24MR::PRESCALER24M_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_1`"]
    #[inline]
    pub fn is_prescaler24m_1(&self) -> bool {
        *self == PRESCALER24MR::PRESCALER24M_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_15`"]
    #[inline]
    pub fn is_prescaler24m_15(&self) -> bool {
        *self == PRESCALER24MR::PRESCALER24M_15
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
pub enum PRESCALERW {
    #[doc = "Divide by 1"]
    PRESCALER_0,
    #[doc = "Divide by 2"]
    PRESCALER_1,
    #[doc = "Divide by 4096"]
    PRESCALER_4095,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            PRESCALERW::PRESCALER_0 => 0,
            PRESCALERW::PRESCALER_1 => 1,
            PRESCALERW::PRESCALER_4095 => 4095,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn prescaler_0(self) -> &'a mut W {
        self.variant(PRESCALERW::PRESCALER_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn prescaler_1(self) -> &'a mut W {
        self.variant(PRESCALERW::PRESCALER_1)
    }
    #[doc = "Divide by 4096"]
    #[inline]
    pub fn prescaler_4095(self) -> &'a mut W {
        self.variant(PRESCALERW::PRESCALER_4095)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER24M`"]
pub enum PRESCALER24MW {
    #[doc = "Divide by 1"]
    PRESCALER24M_0,
    #[doc = "Divide by 2"]
    PRESCALER24M_1,
    #[doc = "Divide by 16"]
    PRESCALER24M_15,
}
impl PRESCALER24MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALER24MW::PRESCALER24M_0 => 0,
            PRESCALER24MW::PRESCALER24M_1 => 1,
            PRESCALER24MW::PRESCALER24M_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALER24MW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALER24MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALER24MW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn prescaler24m_0(self) -> &'a mut W {
        self.variant(PRESCALER24MW::PRESCALER24M_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn prescaler24m_1(self) -> &'a mut W {
        self.variant(PRESCALER24MW::PRESCALER24M_1)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn prescaler24m_15(self) -> &'a mut W {
        self.variant(PRESCALER24MW::PRESCALER24M_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:11 - Prescaler bits"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 12:15 - Prescaler bits"]
    #[inline]
    pub fn prescaler24m(&self) -> PRESCALER24MR {
        PRESCALER24MR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:11 - Prescaler bits"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
    #[doc = "Bits 12:15 - Prescaler bits"]
    #[inline]
    pub fn prescaler24m(&mut self) -> _PRESCALER24MW {
        _PRESCALER24MW { w: self }
    }
}
