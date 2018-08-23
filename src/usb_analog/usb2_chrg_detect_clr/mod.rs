#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB2_CHRG_DETECT_CLR {
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
#[doc = "Possible values of the field `CHK_CONTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHK_CONTACTR {
    #[doc = "Do not check the contact of USB plug."]
    NO_CHECK,
    #[doc = "Check whether the USB plug has been in contact with each other"]
    CHECK,
}
impl CHK_CONTACTR {
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
            CHK_CONTACTR::NO_CHECK => false,
            CHK_CONTACTR::CHECK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHK_CONTACTR {
        match value {
            false => CHK_CONTACTR::NO_CHECK,
            true => CHK_CONTACTR::CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHECK`"]
    #[inline]
    pub fn is_no_check(&self) -> bool {
        *self == CHK_CONTACTR::NO_CHECK
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline]
    pub fn is_check(&self) -> bool {
        *self == CHK_CONTACTR::CHECK
    }
}
#[doc = "Possible values of the field `CHK_CHRG_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHK_CHRG_BR {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK,
}
impl CHK_CHRG_BR {
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
            CHK_CHRG_BR::CHECK => false,
            CHK_CHRG_BR::NO_CHECK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHK_CHRG_BR {
        match value {
            false => CHK_CHRG_BR::CHECK,
            true => CHK_CHRG_BR::NO_CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline]
    pub fn is_check(&self) -> bool {
        *self == CHK_CHRG_BR::CHECK
    }
    #[doc = "Checks if the value of the field is `NO_CHECK`"]
    #[inline]
    pub fn is_no_check(&self) -> bool {
        *self == CHK_CHRG_BR::NO_CHECK
    }
}
#[doc = "Possible values of the field `EN_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_BR {
    #[doc = "Enable the charger detector."]
    ENABLE,
    #[doc = "Disable the charger detector."]
    DISABLE,
}
impl EN_BR {
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
            EN_BR::ENABLE => false,
            EN_BR::DISABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_BR {
        match value {
            false => EN_BR::ENABLE,
            true => EN_BR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == EN_BR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EN_BR::DISABLE
    }
}
#[doc = "Values that can be written to the field `CHK_CONTACT`"]
pub enum CHK_CONTACTW {
    #[doc = "Do not check the contact of USB plug."]
    NO_CHECK,
    #[doc = "Check whether the USB plug has been in contact with each other"]
    CHECK,
}
impl CHK_CONTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHK_CONTACTW::NO_CHECK => false,
            CHK_CONTACTW::CHECK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHK_CONTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _CHK_CONTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHK_CONTACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not check the contact of USB plug."]
    #[inline]
    pub fn no_check(self) -> &'a mut W {
        self.variant(CHK_CONTACTW::NO_CHECK)
    }
    #[doc = "Check whether the USB plug has been in contact with each other"]
    #[inline]
    pub fn check(self) -> &'a mut W {
        self.variant(CHK_CONTACTW::CHECK)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHK_CHRG_B`"]
pub enum CHK_CHRG_BW {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK,
}
impl CHK_CHRG_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHK_CHRG_BW::CHECK => false,
            CHK_CHRG_BW::NO_CHECK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHK_CHRG_BW<'a> {
    w: &'a mut W,
}
impl<'a> _CHK_CHRG_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHK_CHRG_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    #[inline]
    pub fn check(self) -> &'a mut W {
        self.variant(CHK_CHRG_BW::CHECK)
    }
    #[doc = "Do not check whether a charger is connected to the USB port."]
    #[inline]
    pub fn no_check(self) -> &'a mut W {
        self.variant(CHK_CHRG_BW::NO_CHECK)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_B`"]
pub enum EN_BW {
    #[doc = "Enable the charger detector."]
    ENABLE,
    #[doc = "Disable the charger detector."]
    DISABLE,
}
impl EN_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_BW::ENABLE => false,
            EN_BW::DISABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_BW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the charger detector."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_BW::ENABLE)
    }
    #[doc = "Disable the charger detector."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_BW::DISABLE)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 18 - Check the contact of USB plug"]
    #[inline]
    pub fn chk_contact(&self) -> CHK_CONTACTR {
        CHK_CONTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Check the charger connection"]
    #[inline]
    pub fn chk_chrg_b(&self) -> CHK_CHRG_BR {
        CHK_CHRG_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Control the charger detector."]
    #[inline]
    pub fn en_b(&self) -> EN_BR {
        EN_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 18 - Check the contact of USB plug"]
    #[inline]
    pub fn chk_contact(&mut self) -> _CHK_CONTACTW {
        _CHK_CONTACTW { w: self }
    }
    #[doc = "Bit 19 - Check the charger connection"]
    #[inline]
    pub fn chk_chrg_b(&mut self) -> _CHK_CHRG_BW {
        _CHK_CHRG_BW { w: self }
    }
    #[doc = "Bit 20 - Control the charger detector."]
    #[inline]
    pub fn en_b(&mut self) -> _EN_BW {
        _EN_BW { w: self }
    }
}
