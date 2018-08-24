#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM0TCTRL {
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
#[doc = "Possible values of the field `OUT_TRIG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT_TRIG_ENR {
    #[doc = "PWM_OUT_TRIGx will not set when the counter value matches the VALx value."]
    OUT_TRIG_EN_0,
    #[doc = "PWM_OUT_TRIGx will set when the counter value matches the VALx value."]
    OUT_TRIG_EN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUT_TRIG_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUT_TRIG_ENR::OUT_TRIG_EN_0 => 0,
            OUT_TRIG_ENR::OUT_TRIG_EN_1 => 1,
            OUT_TRIG_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUT_TRIG_ENR {
        match value {
            0 => OUT_TRIG_ENR::OUT_TRIG_EN_0,
            1 => OUT_TRIG_ENR::OUT_TRIG_EN_1,
            i => OUT_TRIG_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT_TRIG_EN_0`"]
    #[inline]
    pub fn is_out_trig_en_0(&self) -> bool {
        *self == OUT_TRIG_ENR::OUT_TRIG_EN_0
    }
    #[doc = "Checks if the value of the field is `OUT_TRIG_EN_1`"]
    #[inline]
    pub fn is_out_trig_en_1(&self) -> bool {
        *self == OUT_TRIG_ENR::OUT_TRIG_EN_1
    }
}
#[doc = "Possible values of the field `TRGFRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGFRQR {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL[LDFQ] being non-zero."]
    TRGFRQ_0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL[LDFQ] being non-zero."]
    TRGFRQ_1,
}
impl TRGFRQR {
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
            TRGFRQR::TRGFRQ_0 => false,
            TRGFRQR::TRGFRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGFRQR {
        match value {
            false => TRGFRQR::TRGFRQ_0,
            true => TRGFRQR::TRGFRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRGFRQ_0`"]
    #[inline]
    pub fn is_trgfrq_0(&self) -> bool {
        *self == TRGFRQR::TRGFRQ_0
    }
    #[doc = "Checks if the value of the field is `TRGFRQ_1`"]
    #[inline]
    pub fn is_trgfrq_1(&self) -> bool {
        *self == TRGFRQR::TRGFRQ_1
    }
}
#[doc = "Possible values of the field `PWBOT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWBOT1R {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
    PWBOT1_0,
    #[doc = "Route the PWMB output to the PWM_OUT_TRIG1 port."]
    PWBOT1_1,
}
impl PWBOT1R {
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
            PWBOT1R::PWBOT1_0 => false,
            PWBOT1R::PWBOT1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWBOT1R {
        match value {
            false => PWBOT1R::PWBOT1_0,
            true => PWBOT1R::PWBOT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWBOT1_0`"]
    #[inline]
    pub fn is_pwbot1_0(&self) -> bool {
        *self == PWBOT1R::PWBOT1_0
    }
    #[doc = "Checks if the value of the field is `PWBOT1_1`"]
    #[inline]
    pub fn is_pwbot1_1(&self) -> bool {
        *self == PWBOT1R::PWBOT1_1
    }
}
#[doc = "Possible values of the field `PWAOT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWAOT0R {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
    PWAOT0_0,
    #[doc = "Route the PWMA output to the PWM_OUT_TRIG0 port."]
    PWAOT0_1,
}
impl PWAOT0R {
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
            PWAOT0R::PWAOT0_0 => false,
            PWAOT0R::PWAOT0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWAOT0R {
        match value {
            false => PWAOT0R::PWAOT0_0,
            true => PWAOT0R::PWAOT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWAOT0_0`"]
    #[inline]
    pub fn is_pwaot0_0(&self) -> bool {
        *self == PWAOT0R::PWAOT0_0
    }
    #[doc = "Checks if the value of the field is `PWAOT0_1`"]
    #[inline]
    pub fn is_pwaot0_1(&self) -> bool {
        *self == PWAOT0R::PWAOT0_1
    }
}
#[doc = "Values that can be written to the field `OUT_TRIG_EN`"]
pub enum OUT_TRIG_ENW {
    #[doc = "PWM_OUT_TRIGx will not set when the counter value matches the VALx value."]
    OUT_TRIG_EN_0,
    #[doc = "PWM_OUT_TRIGx will set when the counter value matches the VALx value."]
    OUT_TRIG_EN_1,
}
impl OUT_TRIG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUT_TRIG_ENW::OUT_TRIG_EN_0 => 0,
            OUT_TRIG_ENW::OUT_TRIG_EN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT_TRIG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OUT_TRIG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT_TRIG_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM_OUT_TRIGx will not set when the counter value matches the VALx value."]
    #[inline]
    pub fn out_trig_en_0(self) -> &'a mut W {
        self.variant(OUT_TRIG_ENW::OUT_TRIG_EN_0)
    }
    #[doc = "PWM_OUT_TRIGx will set when the counter value matches the VALx value."]
    #[inline]
    pub fn out_trig_en_1(self) -> &'a mut W {
        self.variant(OUT_TRIG_ENW::OUT_TRIG_EN_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGFRQ`"]
pub enum TRGFRQW {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL[LDFQ] being non-zero."]
    TRGFRQ_0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL[LDFQ] being non-zero."]
    TRGFRQ_1,
}
impl TRGFRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGFRQW::TRGFRQ_0 => false,
            TRGFRQW::TRGFRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGFRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGFRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGFRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL[LDFQ] being non-zero."]
    #[inline]
    pub fn trgfrq_0(self) -> &'a mut W {
        self.variant(TRGFRQW::TRGFRQ_0)
    }
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL[LDFQ] being non-zero."]
    #[inline]
    pub fn trgfrq_1(self) -> &'a mut W {
        self.variant(TRGFRQW::TRGFRQ_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWBOT1`"]
pub enum PWBOT1W {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
    PWBOT1_0,
    #[doc = "Route the PWMB output to the PWM_OUT_TRIG1 port."]
    PWBOT1_1,
}
impl PWBOT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWBOT1W::PWBOT1_0 => false,
            PWBOT1W::PWBOT1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWBOT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWBOT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWBOT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
    #[inline]
    pub fn pwbot1_0(self) -> &'a mut W {
        self.variant(PWBOT1W::PWBOT1_0)
    }
    #[doc = "Route the PWMB output to the PWM_OUT_TRIG1 port."]
    #[inline]
    pub fn pwbot1_1(self) -> &'a mut W {
        self.variant(PWBOT1W::PWBOT1_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWAOT0`"]
pub enum PWAOT0W {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
    PWAOT0_0,
    #[doc = "Route the PWMA output to the PWM_OUT_TRIG0 port."]
    PWAOT0_1,
}
impl PWAOT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWAOT0W::PWAOT0_0 => false,
            PWAOT0W::PWAOT0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWAOT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWAOT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWAOT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
    #[inline]
    pub fn pwaot0_0(self) -> &'a mut W {
        self.variant(PWAOT0W::PWAOT0_0)
    }
    #[doc = "Route the PWMA output to the PWM_OUT_TRIG0 port."]
    #[inline]
    pub fn pwaot0_1(self) -> &'a mut W {
        self.variant(PWAOT0W::PWAOT0_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:5 - Output Trigger Enables"]
    #[inline]
    pub fn out_trig_en(&self) -> OUT_TRIG_ENR {
        OUT_TRIG_ENR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 12 - Trigger frequency"]
    #[inline]
    pub fn trgfrq(&self) -> TRGFRQR {
        TRGFRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Output Trigger 1 Source Select"]
    #[inline]
    pub fn pwbot1(&self) -> PWBOT1R {
        PWBOT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Output Trigger 0 Source Select"]
    #[inline]
    pub fn pwaot0(&self) -> PWAOT0R {
        PWAOT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    #[doc = "Bits 0:5 - Output Trigger Enables"]
    #[inline]
    pub fn out_trig_en(&mut self) -> _OUT_TRIG_ENW {
        _OUT_TRIG_ENW { w: self }
    }
    #[doc = "Bit 12 - Trigger frequency"]
    #[inline]
    pub fn trgfrq(&mut self) -> _TRGFRQW {
        _TRGFRQW { w: self }
    }
    #[doc = "Bit 14 - Output Trigger 1 Source Select"]
    #[inline]
    pub fn pwbot1(&mut self) -> _PWBOT1W {
        _PWBOT1W { w: self }
    }
    #[doc = "Bit 15 - Output Trigger 0 Source Select"]
    #[inline]
    pub fn pwaot0(&mut self) -> _PWAOT0W {
        _PWAOT0W { w: self }
    }
}
