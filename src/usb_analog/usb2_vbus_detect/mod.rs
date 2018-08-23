#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB2_VBUS_DETECT {
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
#[doc = "Possible values of the field `VBUSVALID_THRESH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_THRESHR {
    #[doc = "4.0V"]
    _4V0,
    #[doc = "4.1V"]
    _4V1,
    #[doc = "4.2V"]
    _4V2,
    #[doc = "4.3V"]
    _4V3,
    #[doc = "4.4V (default)"]
    _4V4,
    #[doc = "4.5V"]
    _4V5,
    #[doc = "4.6V"]
    _4V6,
    #[doc = "4.7V"]
    _4V7,
}
impl VBUSVALID_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VBUSVALID_THRESHR::_4V0 => 0,
            VBUSVALID_THRESHR::_4V1 => 1,
            VBUSVALID_THRESHR::_4V2 => 2,
            VBUSVALID_THRESHR::_4V3 => 3,
            VBUSVALID_THRESHR::_4V4 => 4,
            VBUSVALID_THRESHR::_4V5 => 5,
            VBUSVALID_THRESHR::_4V6 => 6,
            VBUSVALID_THRESHR::_4V7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VBUSVALID_THRESHR {
        match value {
            0 => VBUSVALID_THRESHR::_4V0,
            1 => VBUSVALID_THRESHR::_4V1,
            2 => VBUSVALID_THRESHR::_4V2,
            3 => VBUSVALID_THRESHR::_4V3,
            4 => VBUSVALID_THRESHR::_4V4,
            5 => VBUSVALID_THRESHR::_4V5,
            6 => VBUSVALID_THRESHR::_4V6,
            7 => VBUSVALID_THRESHR::_4V7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4V0`"]
    #[inline]
    pub fn is_4v0(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V0
    }
    #[doc = "Checks if the value of the field is `_4V1`"]
    #[inline]
    pub fn is_4v1(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V1
    }
    #[doc = "Checks if the value of the field is `_4V2`"]
    #[inline]
    pub fn is_4v2(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V2
    }
    #[doc = "Checks if the value of the field is `_4V3`"]
    #[inline]
    pub fn is_4v3(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V3
    }
    #[doc = "Checks if the value of the field is `_4V4`"]
    #[inline]
    pub fn is_4v4(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V4
    }
    #[doc = "Checks if the value of the field is `_4V5`"]
    #[inline]
    pub fn is_4v5(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V5
    }
    #[doc = "Checks if the value of the field is `_4V6`"]
    #[inline]
    pub fn is_4v6(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V6
    }
    #[doc = "Checks if the value of the field is `_4V7`"]
    #[inline]
    pub fn is_4v7(&self) -> bool {
        *self == VBUSVALID_THRESHR::_4V7
    }
}
#[doc = r" Value of the field"]
pub struct VBUSVALID_PWRUP_CMPSR {
    bits: bool,
}
impl VBUSVALID_PWRUP_CMPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DISCHARGE_VBUSR {
    bits: bool,
}
impl DISCHARGE_VBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHARGE_VBUSR {
    bits: bool,
}
impl CHARGE_VBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `VBUSVALID_THRESH`"]
pub enum VBUSVALID_THRESHW {
    #[doc = "4.0V"]
    _4V0,
    #[doc = "4.1V"]
    _4V1,
    #[doc = "4.2V"]
    _4V2,
    #[doc = "4.3V"]
    _4V3,
    #[doc = "4.4V (default)"]
    _4V4,
    #[doc = "4.5V"]
    _4V5,
    #[doc = "4.6V"]
    _4V6,
    #[doc = "4.7V"]
    _4V7,
}
impl VBUSVALID_THRESHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VBUSVALID_THRESHW::_4V0 => 0,
            VBUSVALID_THRESHW::_4V1 => 1,
            VBUSVALID_THRESHW::_4V2 => 2,
            VBUSVALID_THRESHW::_4V3 => 3,
            VBUSVALID_THRESHW::_4V4 => 4,
            VBUSVALID_THRESHW::_4V5 => 5,
            VBUSVALID_THRESHW::_4V6 => 6,
            VBUSVALID_THRESHW::_4V7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUSVALID_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSVALID_THRESHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUSVALID_THRESHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4.0V"]
    #[inline]
    pub fn _4v0(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V0)
    }
    #[doc = "4.1V"]
    #[inline]
    pub fn _4v1(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V1)
    }
    #[doc = "4.2V"]
    #[inline]
    pub fn _4v2(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V2)
    }
    #[doc = "4.3V"]
    #[inline]
    pub fn _4v3(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V3)
    }
    #[doc = "4.4V (default)"]
    #[inline]
    pub fn _4v4(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V4)
    }
    #[doc = "4.5V"]
    #[inline]
    pub fn _4v5(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V5)
    }
    #[doc = "4.6V"]
    #[inline]
    pub fn _4v6(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V6)
    }
    #[doc = "4.7V"]
    #[inline]
    pub fn _4v7(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::_4V7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBUSVALID_PWRUP_CMPSW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSVALID_PWRUP_CMPSW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISCHARGE_VBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCHARGE_VBUSW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHARGE_VBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _CHARGE_VBUSW<'a> {
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:2 - Set the threshold for the VBUSVALID comparator"]
    #[inline]
    pub fn vbusvalid_thresh(&self) -> VBUSVALID_THRESHR {
        VBUSVALID_THRESHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Powers up comparators for vbus_valid detector."]
    #[inline]
    pub fn vbusvalid_pwrup_cmps(&self) -> VBUSVALID_PWRUP_CMPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSVALID_PWRUP_CMPSR { bits }
    }
    #[doc = "Bit 26 - USB OTG discharge VBUS."]
    #[inline]
    pub fn discharge_vbus(&self) -> DISCHARGE_VBUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISCHARGE_VBUSR { bits }
    }
    #[doc = "Bit 27 - USB OTG charge VBUS."]
    #[inline]
    pub fn charge_vbus(&self) -> CHARGE_VBUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHARGE_VBUSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1048580 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Set the threshold for the VBUSVALID comparator"]
    #[inline]
    pub fn vbusvalid_thresh(&mut self) -> _VBUSVALID_THRESHW {
        _VBUSVALID_THRESHW { w: self }
    }
    #[doc = "Bit 20 - Powers up comparators for vbus_valid detector."]
    #[inline]
    pub fn vbusvalid_pwrup_cmps(&mut self) -> _VBUSVALID_PWRUP_CMPSW {
        _VBUSVALID_PWRUP_CMPSW { w: self }
    }
    #[doc = "Bit 26 - USB OTG discharge VBUS."]
    #[inline]
    pub fn discharge_vbus(&mut self) -> _DISCHARGE_VBUSW {
        _DISCHARGE_VBUSW { w: self }
    }
    #[doc = "Bit 27 - USB OTG charge VBUS."]
    #[inline]
    pub fn charge_vbus(&mut self) -> _CHARGE_VBUSW {
        _CHARGE_VBUSW { w: self }
    }
}
