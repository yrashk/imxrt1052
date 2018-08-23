#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SCTRL {
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
#[doc = "Possible values of the field `OEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OENR {
    #[doc = "The external pin is configured as an input."]
    OEN_0,
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OEN_1,
}
impl OENR {
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
            OENR::OEN_0 => false,
            OENR::OEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OENR {
        match value {
            false => OENR::OEN_0,
            true => OENR::OEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OEN_0`"]
    #[inline]
    pub fn is_oen_0(&self) -> bool {
        *self == OENR::OEN_0
    }
    #[doc = "Checks if the value of the field is `OEN_1`"]
    #[inline]
    pub fn is_oen_1(&self) -> bool {
        *self == OENR::OEN_1
    }
}
#[doc = "Possible values of the field `OPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPSR {
    #[doc = "True polarity."]
    OPS_0,
    #[doc = "Inverted polarity."]
    OPS_1,
}
impl OPSR {
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
            OPSR::OPS_0 => false,
            OPSR::OPS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OPSR {
        match value {
            false => OPSR::OPS_0,
            true => OPSR::OPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OPS_0`"]
    #[inline]
    pub fn is_ops_0(&self) -> bool {
        *self == OPSR::OPS_0
    }
    #[doc = "Checks if the value of the field is `OPS_1`"]
    #[inline]
    pub fn is_ops_1(&self) -> bool {
        *self == OPSR::OPS_1
    }
}
#[doc = r" Value of the field"]
pub struct VALR {
    bits: bool,
}
impl VALR {
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
pub struct EEOFR {
    bits: bool,
}
impl EEOFR {
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
pub struct MSTRR {
    bits: bool,
}
impl MSTRR {
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
#[doc = "Possible values of the field `CAPTURE_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE_MODER {
    #[doc = "Capture function is disabled"]
    CAPTURE_MODE_0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    CAPTURE_MODE_1,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    CAPTURE_MODE_2,
    #[doc = "Load capture register on both edges of input"]
    CAPTURE_MODE_3,
}
impl CAPTURE_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPTURE_MODER::CAPTURE_MODE_0 => 0,
            CAPTURE_MODER::CAPTURE_MODE_1 => 1,
            CAPTURE_MODER::CAPTURE_MODE_2 => 2,
            CAPTURE_MODER::CAPTURE_MODE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPTURE_MODER {
        match value {
            0 => CAPTURE_MODER::CAPTURE_MODE_0,
            1 => CAPTURE_MODER::CAPTURE_MODE_1,
            2 => CAPTURE_MODER::CAPTURE_MODE_2,
            3 => CAPTURE_MODER::CAPTURE_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_0`"]
    #[inline]
    pub fn is_capture_mode_0(&self) -> bool {
        *self == CAPTURE_MODER::CAPTURE_MODE_0
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_1`"]
    #[inline]
    pub fn is_capture_mode_1(&self) -> bool {
        *self == CAPTURE_MODER::CAPTURE_MODE_1
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_2`"]
    #[inline]
    pub fn is_capture_mode_2(&self) -> bool {
        *self == CAPTURE_MODER::CAPTURE_MODE_2
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_3`"]
    #[inline]
    pub fn is_capture_mode_3(&self) -> bool {
        *self == CAPTURE_MODER::CAPTURE_MODE_3
    }
}
#[doc = r" Value of the field"]
pub struct INPUTR {
    bits: bool,
}
impl INPUTR {
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
pub struct IPSR {
    bits: bool,
}
impl IPSR {
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
pub struct IEFIER {
    bits: bool,
}
impl IEFIER {
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
pub struct IEFR {
    bits: bool,
}
impl IEFR {
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
pub struct TOFIER {
    bits: bool,
}
impl TOFIER {
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
pub struct TOFR {
    bits: bool,
}
impl TOFR {
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
pub struct TCFIER {
    bits: bool,
}
impl TCFIER {
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
pub struct TCFR {
    bits: bool,
}
impl TCFR {
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
#[doc = "Values that can be written to the field `OEN`"]
pub enum OENW {
    #[doc = "The external pin is configured as an input."]
    OEN_0,
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OEN_1,
}
impl OENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OENW::OEN_0 => false,
            OENW::OEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OENW<'a> {
    w: &'a mut W,
}
impl<'a> _OENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The external pin is configured as an input."]
    #[inline]
    pub fn oen_0(self) -> &'a mut W {
        self.variant(OENW::OEN_0)
    }
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    #[inline]
    pub fn oen_1(self) -> &'a mut W {
        self.variant(OENW::OEN_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPS`"]
pub enum OPSW {
    #[doc = "True polarity."]
    OPS_0,
    #[doc = "Inverted polarity."]
    OPS_1,
}
impl OPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPSW::OPS_0 => false,
            OPSW::OPS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPSW<'a> {
    w: &'a mut W,
}
impl<'a> _OPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "True polarity."]
    #[inline]
    pub fn ops_0(self) -> &'a mut W {
        self.variant(OPSW::OPS_0)
    }
    #[doc = "Inverted polarity."]
    #[inline]
    pub fn ops_1(self) -> &'a mut W {
        self.variant(OPSW::OPS_1)
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
#[doc = r" Proxy"]
pub struct _FORCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEW<'a> {
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
#[doc = r" Proxy"]
pub struct _VALW<'a> {
    w: &'a mut W,
}
impl<'a> _VALW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EEOFW<'a> {
    w: &'a mut W,
}
impl<'a> _EEOFW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTRW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTURE_MODE`"]
pub enum CAPTURE_MODEW {
    #[doc = "Capture function is disabled"]
    CAPTURE_MODE_0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    CAPTURE_MODE_1,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    CAPTURE_MODE_2,
    #[doc = "Load capture register on both edges of input"]
    CAPTURE_MODE_3,
}
impl CAPTURE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPTURE_MODEW::CAPTURE_MODE_0 => 0,
            CAPTURE_MODEW::CAPTURE_MODE_1 => 1,
            CAPTURE_MODEW::CAPTURE_MODE_2 => 2,
            CAPTURE_MODEW::CAPTURE_MODE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTURE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTURE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTURE_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Capture function is disabled"]
    #[inline]
    pub fn capture_mode_0(self) -> &'a mut W {
        self.variant(CAPTURE_MODEW::CAPTURE_MODE_0)
    }
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    #[inline]
    pub fn capture_mode_1(self) -> &'a mut W {
        self.variant(CAPTURE_MODEW::CAPTURE_MODE_1)
    }
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    #[inline]
    pub fn capture_mode_2(self) -> &'a mut W {
        self.variant(CAPTURE_MODEW::CAPTURE_MODE_2)
    }
    #[doc = "Load capture register on both edges of input"]
    #[inline]
    pub fn capture_mode_3(self) -> &'a mut W {
        self.variant(CAPTURE_MODEW::CAPTURE_MODE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IPSW<'a> {
    w: &'a mut W,
}
impl<'a> _IPSW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IEFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IEFIEW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IEFW<'a> {
    w: &'a mut W,
}
impl<'a> _IEFW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TOFIEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TOFW<'a> {
    w: &'a mut W,
}
impl<'a> _TOFW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFIEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFW<'a> {
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
    #[doc = "Bit 0 - Output Enable"]
    #[inline]
    pub fn oen(&self) -> OENR {
        OENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Output Polarity Select"]
    #[inline]
    pub fn ops(&self) -> OPSR {
        OPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Forced OFLAG Value"]
    #[inline]
    pub fn val(&self) -> VALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        VALR { bits }
    }
    #[doc = "Bit 4 - Enable External OFLAG Force"]
    #[inline]
    pub fn eeof(&self) -> EEOFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        EEOFR { bits }
    }
    #[doc = "Bit 5 - Master Mode"]
    #[inline]
    pub fn mstr(&self) -> MSTRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        MSTRR { bits }
    }
    #[doc = "Bits 6:7 - Input Capture Mode"]
    #[inline]
    pub fn capture_mode(&self) -> CAPTURE_MODER {
        CAPTURE_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - External Input Signal"]
    #[inline]
    pub fn input(&self) -> INPUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        INPUTR { bits }
    }
    #[doc = "Bit 9 - Input Polarity Select"]
    #[inline]
    pub fn ips(&self) -> IPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        IPSR { bits }
    }
    #[doc = "Bit 10 - Input Edge Flag Interrupt Enable"]
    #[inline]
    pub fn iefie(&self) -> IEFIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        IEFIER { bits }
    }
    #[doc = "Bit 11 - Input Edge Flag"]
    #[inline]
    pub fn ief(&self) -> IEFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        IEFR { bits }
    }
    #[doc = "Bit 12 - Timer Overflow Flag Interrupt Enable"]
    #[inline]
    pub fn tofie(&self) -> TOFIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TOFIER { bits }
    }
    #[doc = "Bit 13 - Timer Overflow Flag"]
    #[inline]
    pub fn tof(&self) -> TOFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TOFR { bits }
    }
    #[doc = "Bit 14 - Timer Compare Flag Interrupt Enable"]
    #[inline]
    pub fn tcfie(&self) -> TCFIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCFIER { bits }
    }
    #[doc = "Bit 15 - Timer Compare Flag"]
    #[inline]
    pub fn tcf(&self) -> TCFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCFR { bits }
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
    #[doc = "Bit 0 - Output Enable"]
    #[inline]
    pub fn oen(&mut self) -> _OENW {
        _OENW { w: self }
    }
    #[doc = "Bit 1 - Output Polarity Select"]
    #[inline]
    pub fn ops(&mut self) -> _OPSW {
        _OPSW { w: self }
    }
    #[doc = "Bit 2 - Force OFLAG Output"]
    #[inline]
    pub fn force(&mut self) -> _FORCEW {
        _FORCEW { w: self }
    }
    #[doc = "Bit 3 - Forced OFLAG Value"]
    #[inline]
    pub fn val(&mut self) -> _VALW {
        _VALW { w: self }
    }
    #[doc = "Bit 4 - Enable External OFLAG Force"]
    #[inline]
    pub fn eeof(&mut self) -> _EEOFW {
        _EEOFW { w: self }
    }
    #[doc = "Bit 5 - Master Mode"]
    #[inline]
    pub fn mstr(&mut self) -> _MSTRW {
        _MSTRW { w: self }
    }
    #[doc = "Bits 6:7 - Input Capture Mode"]
    #[inline]
    pub fn capture_mode(&mut self) -> _CAPTURE_MODEW {
        _CAPTURE_MODEW { w: self }
    }
    #[doc = "Bit 9 - Input Polarity Select"]
    #[inline]
    pub fn ips(&mut self) -> _IPSW {
        _IPSW { w: self }
    }
    #[doc = "Bit 10 - Input Edge Flag Interrupt Enable"]
    #[inline]
    pub fn iefie(&mut self) -> _IEFIEW {
        _IEFIEW { w: self }
    }
    #[doc = "Bit 11 - Input Edge Flag"]
    #[inline]
    pub fn ief(&mut self) -> _IEFW {
        _IEFW { w: self }
    }
    #[doc = "Bit 12 - Timer Overflow Flag Interrupt Enable"]
    #[inline]
    pub fn tofie(&mut self) -> _TOFIEW {
        _TOFIEW { w: self }
    }
    #[doc = "Bit 13 - Timer Overflow Flag"]
    #[inline]
    pub fn tof(&mut self) -> _TOFW {
        _TOFW { w: self }
    }
    #[doc = "Bit 14 - Timer Compare Flag Interrupt Enable"]
    #[inline]
    pub fn tcfie(&mut self) -> _TCFIEW {
        _TCFIEW { w: self }
    }
    #[doc = "Bit 15 - Timer Compare Flag"]
    #[inline]
    pub fn tcf(&mut self) -> _TCFW {
        _TCFW { w: self }
    }
}
