#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FSTS0 {
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
#[doc = "Possible values of the field `FFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLAGR {
    #[doc = "No fault on the FAULTx pin."]
    FFLAG_0,
    #[doc = "Fault on the FAULTx pin."]
    FFLAG_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FFLAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FFLAGR::FFLAG_0 => 0,
            FFLAGR::FFLAG_1 => 1,
            FFLAGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FFLAGR {
        match value {
            0 => FFLAGR::FFLAG_0,
            1 => FFLAGR::FFLAG_1,
            i => FFLAGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FFLAG_0`"]
    #[inline]
    pub fn is_fflag_0(&self) -> bool {
        *self == FFLAGR::FFLAG_0
    }
    #[doc = "Checks if the value of the field is `FFLAG_1`"]
    #[inline]
    pub fn is_fflag_1(&self) -> bool {
        *self == FFLAGR::FFLAG_1
    }
}
#[doc = "Possible values of the field `FFULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFULLR {
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
    FFULL_0,
    #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
    FFULL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FFULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FFULLR::FFULL_0 => 0,
            FFULLR::FFULL_1 => 1,
            FFULLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FFULLR {
        match value {
            0 => FFULLR::FFULL_0,
            1 => FFULLR::FFULL_1,
            i => FFULLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FFULL_0`"]
    #[inline]
    pub fn is_ffull_0(&self) -> bool {
        *self == FFULLR::FFULL_0
    }
    #[doc = "Checks if the value of the field is `FFULL_1`"]
    #[inline]
    pub fn is_ffull_1(&self) -> bool {
        *self == FFULLR::FFULL_1
    }
}
#[doc = r" Value of the field"]
pub struct FFPINR {
    bits: u8,
}
impl FFPINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FHALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FHALFR {
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    FHALF_0,
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    FHALF_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FHALFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FHALFR::FHALF_0 => 0,
            FHALFR::FHALF_1 => 1,
            FHALFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FHALFR {
        match value {
            0 => FHALFR::FHALF_0,
            1 => FHALFR::FHALF_1,
            i => FHALFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FHALF_0`"]
    #[inline]
    pub fn is_fhalf_0(&self) -> bool {
        *self == FHALFR::FHALF_0
    }
    #[doc = "Checks if the value of the field is `FHALF_1`"]
    #[inline]
    pub fn is_fhalf_1(&self) -> bool {
        *self == FHALFR::FHALF_1
    }
}
#[doc = "Values that can be written to the field `FFLAG`"]
pub enum FFLAGW {
    #[doc = "No fault on the FAULTx pin."]
    FFLAG_0,
    #[doc = "Fault on the FAULTx pin."]
    FFLAG_1,
}
impl FFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FFLAGW::FFLAG_0 => 0,
            FFLAGW::FFLAG_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _FFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFLAGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No fault on the FAULTx pin."]
    #[inline]
    pub fn fflag_0(self) -> &'a mut W {
        self.variant(FFLAGW::FFLAG_0)
    }
    #[doc = "Fault on the FAULTx pin."]
    #[inline]
    pub fn fflag_1(self) -> &'a mut W {
        self.variant(FFLAGW::FFLAG_1)
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
#[doc = "Values that can be written to the field `FFULL`"]
pub enum FFULLW {
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
    FFULL_0,
    #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
    FFULL_1,
}
impl FFULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FFULLW::FFULL_0 => 0,
            FFULLW::FFULL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFULLW<'a> {
    w: &'a mut W,
}
impl<'a> _FFULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFULLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
    #[inline]
    pub fn ffull_0(self) -> &'a mut W {
        self.variant(FFULLW::FFULL_0)
    }
    #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
    #[inline]
    pub fn ffull_1(self) -> &'a mut W {
        self.variant(FFULLW::FFULL_1)
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
#[doc = "Values that can be written to the field `FHALF`"]
pub enum FHALFW {
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    FHALF_0,
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    FHALF_1,
}
impl FHALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FHALFW::FHALF_0 => 0,
            FHALFW::FHALF_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FHALFW<'a> {
    w: &'a mut W,
}
impl<'a> _FHALFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FHALFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    #[inline]
    pub fn fhalf_0(self) -> &'a mut W {
        self.variant(FHALFW::FHALF_0)
    }
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    #[inline]
    pub fn fhalf_1(self) -> &'a mut W {
        self.variant(FHALFW::FHALF_1)
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
    #[doc = "Bits 0:3 - Fault Flags"]
    #[inline]
    pub fn fflag(&self) -> FFLAGR {
        FFLAGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:7 - Full Cycle"]
    #[inline]
    pub fn ffull(&self) -> FFULLR {
        FFULLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:11 - Filtered Fault Pins"]
    #[inline]
    pub fn ffpin(&self) -> FFPINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        FFPINR { bits }
    }
    #[doc = "Bits 12:15 - Half Cycle Fault Recovery"]
    #[inline]
    pub fn fhalf(&self) -> FHALFR {
        FHALFR::_from({
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
    #[doc = "Bits 0:3 - Fault Flags"]
    #[inline]
    pub fn fflag(&mut self) -> _FFLAGW {
        _FFLAGW { w: self }
    }
    #[doc = "Bits 4:7 - Full Cycle"]
    #[inline]
    pub fn ffull(&mut self) -> _FFULLW {
        _FFULLW { w: self }
    }
    #[doc = "Bits 12:15 - Half Cycle Fault Recovery"]
    #[inline]
    pub fn fhalf(&mut self) -> _FHALFW {
        _FHALFW { w: self }
    }
}
