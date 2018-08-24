#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM1FRCTRL {
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
#[doc = "Possible values of the field `FRAC1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC1_ENR {
    #[doc = "Disable fractional cycle length for the PWM period."]
    FRAC1_EN_0,
    #[doc = "Enable fractional cycle length for the PWM period."]
    FRAC1_EN_1,
}
impl FRAC1_ENR {
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
            FRAC1_ENR::FRAC1_EN_0 => false,
            FRAC1_ENR::FRAC1_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAC1_ENR {
        match value {
            false => FRAC1_ENR::FRAC1_EN_0,
            true => FRAC1_ENR::FRAC1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC1_EN_0`"]
    #[inline]
    pub fn is_frac1_en_0(&self) -> bool {
        *self == FRAC1_ENR::FRAC1_EN_0
    }
    #[doc = "Checks if the value of the field is `FRAC1_EN_1`"]
    #[inline]
    pub fn is_frac1_en_1(&self) -> bool {
        *self == FRAC1_ENR::FRAC1_EN_1
    }
}
#[doc = "Possible values of the field `FRAC23_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC23_ENR {
    #[doc = "Disable fractional cycle placement for PWM_A."]
    FRAC23_EN_0,
    #[doc = "Enable fractional cycle placement for PWM_A."]
    FRAC23_EN_1,
}
impl FRAC23_ENR {
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
            FRAC23_ENR::FRAC23_EN_0 => false,
            FRAC23_ENR::FRAC23_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAC23_ENR {
        match value {
            false => FRAC23_ENR::FRAC23_EN_0,
            true => FRAC23_ENR::FRAC23_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC23_EN_0`"]
    #[inline]
    pub fn is_frac23_en_0(&self) -> bool {
        *self == FRAC23_ENR::FRAC23_EN_0
    }
    #[doc = "Checks if the value of the field is `FRAC23_EN_1`"]
    #[inline]
    pub fn is_frac23_en_1(&self) -> bool {
        *self == FRAC23_ENR::FRAC23_EN_1
    }
}
#[doc = "Possible values of the field `FRAC45_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC45_ENR {
    #[doc = "Disable fractional cycle placement for PWM_B."]
    FRAC45_EN_0,
    #[doc = "Enable fractional cycle placement for PWM_B."]
    FRAC45_EN_1,
}
impl FRAC45_ENR {
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
            FRAC45_ENR::FRAC45_EN_0 => false,
            FRAC45_ENR::FRAC45_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAC45_ENR {
        match value {
            false => FRAC45_ENR::FRAC45_EN_0,
            true => FRAC45_ENR::FRAC45_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC45_EN_0`"]
    #[inline]
    pub fn is_frac45_en_0(&self) -> bool {
        *self == FRAC45_ENR::FRAC45_EN_0
    }
    #[doc = "Checks if the value of the field is `FRAC45_EN_1`"]
    #[inline]
    pub fn is_frac45_en_1(&self) -> bool {
        *self == FRAC45_ENR::FRAC45_EN_1
    }
}
#[doc = "Possible values of the field `FRAC_PU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC_PUR {
    #[doc = "Turn off fractional delay logic."]
    FRAC_PU_0,
    #[doc = "Power up fractional delay logic."]
    FRAC_PU_1,
}
impl FRAC_PUR {
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
            FRAC_PUR::FRAC_PU_0 => false,
            FRAC_PUR::FRAC_PU_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAC_PUR {
        match value {
            false => FRAC_PUR::FRAC_PU_0,
            true => FRAC_PUR::FRAC_PU_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC_PU_0`"]
    #[inline]
    pub fn is_frac_pu_0(&self) -> bool {
        *self == FRAC_PUR::FRAC_PU_0
    }
    #[doc = "Checks if the value of the field is `FRAC_PU_1`"]
    #[inline]
    pub fn is_frac_pu_1(&self) -> bool {
        *self == FRAC_PUR::FRAC_PU_1
    }
}
#[doc = r" Value of the field"]
pub struct TESTR {
    bits: bool,
}
impl TESTR {
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
#[doc = "Values that can be written to the field `FRAC1_EN`"]
pub enum FRAC1_ENW {
    #[doc = "Disable fractional cycle length for the PWM period."]
    FRAC1_EN_0,
    #[doc = "Enable fractional cycle length for the PWM period."]
    FRAC1_EN_1,
}
impl FRAC1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAC1_ENW::FRAC1_EN_0 => false,
            FRAC1_ENW::FRAC1_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAC1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAC1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAC1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable fractional cycle length for the PWM period."]
    #[inline]
    pub fn frac1_en_0(self) -> &'a mut W {
        self.variant(FRAC1_ENW::FRAC1_EN_0)
    }
    #[doc = "Enable fractional cycle length for the PWM period."]
    #[inline]
    pub fn frac1_en_1(self) -> &'a mut W {
        self.variant(FRAC1_ENW::FRAC1_EN_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRAC23_EN`"]
pub enum FRAC23_ENW {
    #[doc = "Disable fractional cycle placement for PWM_A."]
    FRAC23_EN_0,
    #[doc = "Enable fractional cycle placement for PWM_A."]
    FRAC23_EN_1,
}
impl FRAC23_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAC23_ENW::FRAC23_EN_0 => false,
            FRAC23_ENW::FRAC23_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAC23_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAC23_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAC23_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable fractional cycle placement for PWM_A."]
    #[inline]
    pub fn frac23_en_0(self) -> &'a mut W {
        self.variant(FRAC23_ENW::FRAC23_EN_0)
    }
    #[doc = "Enable fractional cycle placement for PWM_A."]
    #[inline]
    pub fn frac23_en_1(self) -> &'a mut W {
        self.variant(FRAC23_ENW::FRAC23_EN_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRAC45_EN`"]
pub enum FRAC45_ENW {
    #[doc = "Disable fractional cycle placement for PWM_B."]
    FRAC45_EN_0,
    #[doc = "Enable fractional cycle placement for PWM_B."]
    FRAC45_EN_1,
}
impl FRAC45_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAC45_ENW::FRAC45_EN_0 => false,
            FRAC45_ENW::FRAC45_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAC45_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAC45_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAC45_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable fractional cycle placement for PWM_B."]
    #[inline]
    pub fn frac45_en_0(self) -> &'a mut W {
        self.variant(FRAC45_ENW::FRAC45_EN_0)
    }
    #[doc = "Enable fractional cycle placement for PWM_B."]
    #[inline]
    pub fn frac45_en_1(self) -> &'a mut W {
        self.variant(FRAC45_ENW::FRAC45_EN_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRAC_PU`"]
pub enum FRAC_PUW {
    #[doc = "Turn off fractional delay logic."]
    FRAC_PU_0,
    #[doc = "Power up fractional delay logic."]
    FRAC_PU_1,
}
impl FRAC_PUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAC_PUW::FRAC_PU_0 => false,
            FRAC_PUW::FRAC_PU_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAC_PUW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAC_PUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAC_PUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Turn off fractional delay logic."]
    #[inline]
    pub fn frac_pu_0(self) -> &'a mut W {
        self.variant(FRAC_PUW::FRAC_PU_0)
    }
    #[doc = "Power up fractional delay logic."]
    #[inline]
    pub fn frac_pu_1(self) -> &'a mut W {
        self.variant(FRAC_PUW::FRAC_PU_1)
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
    #[doc = "Bit 1 - Fractional Cycle PWM Period Enable"]
    #[inline]
    pub fn frac1_en(&self) -> FRAC1_ENR {
        FRAC1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Fractional Cycle Placement Enable for PWM_A"]
    #[inline]
    pub fn frac23_en(&self) -> FRAC23_ENR {
        FRAC23_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Fractional Cycle Placement Enable for PWM_B"]
    #[inline]
    pub fn frac45_en(&self) -> FRAC45_ENR {
        FRAC45_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Fractional Delay Circuit Power Up"]
    #[inline]
    pub fn frac_pu(&self) -> FRAC_PUR {
        FRAC_PUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Test Status Bit"]
    #[inline]
    pub fn test(&self) -> TESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TESTR { bits }
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
    #[doc = "Bit 1 - Fractional Cycle PWM Period Enable"]
    #[inline]
    pub fn frac1_en(&mut self) -> _FRAC1_ENW {
        _FRAC1_ENW { w: self }
    }
    #[doc = "Bit 2 - Fractional Cycle Placement Enable for PWM_A"]
    #[inline]
    pub fn frac23_en(&mut self) -> _FRAC23_ENW {
        _FRAC23_ENW { w: self }
    }
    #[doc = "Bit 4 - Fractional Cycle Placement Enable for PWM_B"]
    #[inline]
    pub fn frac45_en(&mut self) -> _FRAC45_ENW {
        _FRAC45_ENW { w: self }
    }
    #[doc = "Bit 8 - Fractional Delay Circuit Power Up"]
    #[inline]
    pub fn frac_pu(&mut self) -> _FRAC_PUW {
        _FRAC_PUW { w: self }
    }
}
