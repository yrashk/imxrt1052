#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `FRZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZR {
    #[doc = "Timers continue to run in Debug mode."]
    FRZ_0,
    #[doc = "Timers are stopped in Debug mode."]
    FRZ_1,
}
impl FRZR {
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
            FRZR::FRZ_0 => false,
            FRZR::FRZ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRZR {
        match value {
            false => FRZR::FRZ_0,
            true => FRZR::FRZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRZ_0`"]
    #[inline]
    pub fn is_frz_0(&self) -> bool {
        *self == FRZR::FRZ_0
    }
    #[doc = "Checks if the value of the field is `FRZ_1`"]
    #[inline]
    pub fn is_frz_1(&self) -> bool {
        *self == FRZR::FRZ_1
    }
}
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Clock for standard PIT timers is enabled."]
    MDIS_0,
    #[doc = "Clock for standard PIT timers is disabled."]
    MDIS_1,
}
impl MDISR {
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
            MDISR::MDIS_0 => false,
            MDISR::MDIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDISR {
        match value {
            false => MDISR::MDIS_0,
            true => MDISR::MDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDIS_0`"]
    #[inline]
    pub fn is_mdis_0(&self) -> bool {
        *self == MDISR::MDIS_0
    }
    #[doc = "Checks if the value of the field is `MDIS_1`"]
    #[inline]
    pub fn is_mdis_1(&self) -> bool {
        *self == MDISR::MDIS_1
    }
}
#[doc = "Values that can be written to the field `FRZ`"]
pub enum FRZW {
    #[doc = "Timers continue to run in Debug mode."]
    FRZ_0,
    #[doc = "Timers are stopped in Debug mode."]
    FRZ_1,
}
impl FRZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRZW::FRZ_0 => false,
            FRZW::FRZ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timers continue to run in Debug mode."]
    #[inline]
    pub fn frz_0(self) -> &'a mut W {
        self.variant(FRZW::FRZ_0)
    }
    #[doc = "Timers are stopped in Debug mode."]
    #[inline]
    pub fn frz_1(self) -> &'a mut W {
        self.variant(FRZW::FRZ_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIS`"]
pub enum MDISW {
    #[doc = "Clock for standard PIT timers is enabled."]
    MDIS_0,
    #[doc = "Clock for standard PIT timers is disabled."]
    MDIS_1,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::MDIS_0 => false,
            MDISW::MDIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock for standard PIT timers is enabled."]
    #[inline]
    pub fn mdis_0(self) -> &'a mut W {
        self.variant(MDISW::MDIS_0)
    }
    #[doc = "Clock for standard PIT timers is disabled."]
    #[inline]
    pub fn mdis_1(self) -> &'a mut W {
        self.variant(MDISW::MDIS_1)
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
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Freeze"]
    #[inline]
    pub fn frz(&self) -> FRZR {
        FRZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Module Disable - (PIT section)"]
    #[inline]
    pub fn mdis(&self) -> MDISR {
        MDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Freeze"]
    #[inline]
    pub fn frz(&mut self) -> _FRZW {
        _FRZW { w: self }
    }
    #[doc = "Bit 1 - Module Disable - (PIT section)"]
    #[inline]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
}
