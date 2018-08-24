#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DACCR {
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
pub struct VOSELR {
    bits: u8,
}
impl VOSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `VRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRSELR {
    #[doc = "Vin1 is selected as resistor ladder network supply reference."]
    VRSEL_0,
    #[doc = "Vin2 is selected as resistor ladder network supply reference."]
    VRSEL_1,
}
impl VRSELR {
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
            VRSELR::VRSEL_0 => false,
            VRSELR::VRSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VRSELR {
        match value {
            false => VRSELR::VRSEL_0,
            true => VRSELR::VRSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VRSEL_0`"]
    #[inline]
    pub fn is_vrsel_0(&self) -> bool {
        *self == VRSELR::VRSEL_0
    }
    #[doc = "Checks if the value of the field is `VRSEL_1`"]
    #[inline]
    pub fn is_vrsel_1(&self) -> bool {
        *self == VRSELR::VRSEL_1
    }
}
#[doc = "Possible values of the field `DACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACENR {
    #[doc = "DAC is disabled."]
    DACEN_0,
    #[doc = "DAC is enabled."]
    DACEN_1,
}
impl DACENR {
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
            DACENR::DACEN_0 => false,
            DACENR::DACEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACENR {
        match value {
            false => DACENR::DACEN_0,
            true => DACENR::DACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACEN_0`"]
    #[inline]
    pub fn is_dacen_0(&self) -> bool {
        *self == DACENR::DACEN_0
    }
    #[doc = "Checks if the value of the field is `DACEN_1`"]
    #[inline]
    pub fn is_dacen_1(&self) -> bool {
        *self == DACENR::DACEN_1
    }
}
#[doc = r" Proxy"]
pub struct _VOSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VOSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VRSEL`"]
pub enum VRSELW {
    #[doc = "Vin1 is selected as resistor ladder network supply reference."]
    VRSEL_0,
    #[doc = "Vin2 is selected as resistor ladder network supply reference."]
    VRSEL_1,
}
impl VRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VRSELW::VRSEL_0 => false,
            VRSELW::VRSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Vin1 is selected as resistor ladder network supply reference."]
    #[inline]
    pub fn vrsel_0(self) -> &'a mut W {
        self.variant(VRSELW::VRSEL_0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference."]
    #[inline]
    pub fn vrsel_1(self) -> &'a mut W {
        self.variant(VRSELW::VRSEL_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DACEN`"]
pub enum DACENW {
    #[doc = "DAC is disabled."]
    DACEN_0,
    #[doc = "DAC is enabled."]
    DACEN_1,
}
impl DACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACENW::DACEN_0 => false,
            DACENW::DACEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC is disabled."]
    #[inline]
    pub fn dacen_0(self) -> &'a mut W {
        self.variant(DACENW::DACEN_0)
    }
    #[doc = "DAC is enabled."]
    #[inline]
    pub fn dacen_1(self) -> &'a mut W {
        self.variant(DACENW::DACEN_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline]
    pub fn vosel(&self) -> VOSELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        VOSELR { bits }
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline]
    pub fn vrsel(&self) -> VRSELR {
        VRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline]
    pub fn dacen(&self) -> DACENR {
        DACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline]
    pub fn vosel(&mut self) -> _VOSELW {
        _VOSELW { w: self }
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline]
    pub fn vrsel(&mut self) -> _VRSELW {
        _VRSELW { w: self }
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline]
    pub fn dacen(&mut self) -> _DACENW {
        _DACENW { w: self }
    }
}
