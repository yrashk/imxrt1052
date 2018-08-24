#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HC0 {
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
#[doc = "Possible values of the field `ADCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHR {
    #[doc = "External channel selection from ADC_ETC"]
    ADCH_16,
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    ADCH_25,
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    ADCH_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCHR::ADCH_16 => 16,
            ADCHR::ADCH_25 => 25,
            ADCHR::ADCH_31 => 31,
            ADCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCHR {
        match value {
            16 => ADCHR::ADCH_16,
            25 => ADCHR::ADCH_25,
            31 => ADCHR::ADCH_31,
            i => ADCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCH_16`"]
    #[inline]
    pub fn is_adch_16(&self) -> bool {
        *self == ADCHR::ADCH_16
    }
    #[doc = "Checks if the value of the field is `ADCH_25`"]
    #[inline]
    pub fn is_adch_25(&self) -> bool {
        *self == ADCHR::ADCH_25
    }
    #[doc = "Checks if the value of the field is `ADCH_31`"]
    #[inline]
    pub fn is_adch_31(&self) -> bool {
        *self == ADCHR::ADCH_31
    }
}
#[doc = "Possible values of the field `AIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIENR {
    #[doc = "Conversion complete interrupt disabled"]
    AIEN_0,
    #[doc = "Conversion complete interrupt enabled"]
    AIEN_1,
}
impl AIENR {
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
            AIENR::AIEN_0 => false,
            AIENR::AIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIENR {
        match value {
            false => AIENR::AIEN_0,
            true => AIENR::AIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AIEN_0`"]
    #[inline]
    pub fn is_aien_0(&self) -> bool {
        *self == AIENR::AIEN_0
    }
    #[doc = "Checks if the value of the field is `AIEN_1`"]
    #[inline]
    pub fn is_aien_1(&self) -> bool {
        *self == AIENR::AIEN_1
    }
}
#[doc = "Values that can be written to the field `ADCH`"]
pub enum ADCHW {
    #[doc = "External channel selection from ADC_ETC"]
    ADCH_16,
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    ADCH_25,
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    ADCH_31,
}
impl ADCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCHW::ADCH_16 => 16,
            ADCHW::ADCH_25 => 25,
            ADCHW::ADCH_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External channel selection from ADC_ETC"]
    #[inline]
    pub fn adch_16(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_16)
    }
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    #[inline]
    pub fn adch_25(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_25)
    }
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    #[inline]
    pub fn adch_31(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AIEN`"]
pub enum AIENW {
    #[doc = "Conversion complete interrupt disabled"]
    AIEN_0,
    #[doc = "Conversion complete interrupt enabled"]
    AIEN_1,
}
impl AIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIENW::AIEN_0 => false,
            AIENW::AIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIENW<'a> {
    w: &'a mut W,
}
impl<'a> _AIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Conversion complete interrupt disabled"]
    #[inline]
    pub fn aien_0(self) -> &'a mut W {
        self.variant(AIENW::AIEN_0)
    }
    #[doc = "Conversion complete interrupt enabled"]
    #[inline]
    pub fn aien_1(self) -> &'a mut W {
        self.variant(AIENW::AIEN_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline]
    pub fn adch(&self) -> ADCHR {
        ADCHR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Conversion Complete Interrupt Enable/Disable Control"]
    #[inline]
    pub fn aien(&self) -> AIENR {
        AIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline]
    pub fn adch(&mut self) -> _ADCHW {
        _ADCHW { w: self }
    }
    #[doc = "Bit 7 - Conversion Complete Interrupt Enable/Disable Control"]
    #[inline]
    pub fn aien(&mut self) -> _AIENW {
        _AIENW { w: self }
    }
}
