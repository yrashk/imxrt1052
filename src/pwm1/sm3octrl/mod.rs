#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM3OCTRL {
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
#[doc = "Possible values of the field `PWMXFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMXFSR {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMXFS_0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMXFS_1,
    #[doc = "Output is tristated."]
    PWMXFS_2,
    #[doc = "Output is tristated."]
    PWMXFS_3,
}
impl PWMXFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMXFSR::PWMXFS_0 => 0,
            PWMXFSR::PWMXFS_1 => 1,
            PWMXFSR::PWMXFS_2 => 2,
            PWMXFSR::PWMXFS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMXFSR {
        match value {
            0 => PWMXFSR::PWMXFS_0,
            1 => PWMXFSR::PWMXFS_1,
            2 => PWMXFSR::PWMXFS_2,
            3 => PWMXFSR::PWMXFS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMXFS_0`"]
    #[inline]
    pub fn is_pwmxfs_0(&self) -> bool {
        *self == PWMXFSR::PWMXFS_0
    }
    #[doc = "Checks if the value of the field is `PWMXFS_1`"]
    #[inline]
    pub fn is_pwmxfs_1(&self) -> bool {
        *self == PWMXFSR::PWMXFS_1
    }
    #[doc = "Checks if the value of the field is `PWMXFS_2`"]
    #[inline]
    pub fn is_pwmxfs_2(&self) -> bool {
        *self == PWMXFSR::PWMXFS_2
    }
    #[doc = "Checks if the value of the field is `PWMXFS_3`"]
    #[inline]
    pub fn is_pwmxfs_3(&self) -> bool {
        *self == PWMXFSR::PWMXFS_3
    }
}
#[doc = "Possible values of the field `PWMBFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMBFSR {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMBFS_0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMBFS_1,
    #[doc = "Output is tristated."]
    PWMBFS_2,
    #[doc = "Output is tristated."]
    PWMBFS_3,
}
impl PWMBFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMBFSR::PWMBFS_0 => 0,
            PWMBFSR::PWMBFS_1 => 1,
            PWMBFSR::PWMBFS_2 => 2,
            PWMBFSR::PWMBFS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMBFSR {
        match value {
            0 => PWMBFSR::PWMBFS_0,
            1 => PWMBFSR::PWMBFS_1,
            2 => PWMBFSR::PWMBFS_2,
            3 => PWMBFSR::PWMBFS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMBFS_0`"]
    #[inline]
    pub fn is_pwmbfs_0(&self) -> bool {
        *self == PWMBFSR::PWMBFS_0
    }
    #[doc = "Checks if the value of the field is `PWMBFS_1`"]
    #[inline]
    pub fn is_pwmbfs_1(&self) -> bool {
        *self == PWMBFSR::PWMBFS_1
    }
    #[doc = "Checks if the value of the field is `PWMBFS_2`"]
    #[inline]
    pub fn is_pwmbfs_2(&self) -> bool {
        *self == PWMBFSR::PWMBFS_2
    }
    #[doc = "Checks if the value of the field is `PWMBFS_3`"]
    #[inline]
    pub fn is_pwmbfs_3(&self) -> bool {
        *self == PWMBFSR::PWMBFS_3
    }
}
#[doc = "Possible values of the field `PWMAFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMAFSR {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMAFS_0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMAFS_1,
    #[doc = "Output is tristated."]
    PWMAFS_2,
    #[doc = "Output is tristated."]
    PWMAFS_3,
}
impl PWMAFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMAFSR::PWMAFS_0 => 0,
            PWMAFSR::PWMAFS_1 => 1,
            PWMAFSR::PWMAFS_2 => 2,
            PWMAFSR::PWMAFS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMAFSR {
        match value {
            0 => PWMAFSR::PWMAFS_0,
            1 => PWMAFSR::PWMAFS_1,
            2 => PWMAFSR::PWMAFS_2,
            3 => PWMAFSR::PWMAFS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMAFS_0`"]
    #[inline]
    pub fn is_pwmafs_0(&self) -> bool {
        *self == PWMAFSR::PWMAFS_0
    }
    #[doc = "Checks if the value of the field is `PWMAFS_1`"]
    #[inline]
    pub fn is_pwmafs_1(&self) -> bool {
        *self == PWMAFSR::PWMAFS_1
    }
    #[doc = "Checks if the value of the field is `PWMAFS_2`"]
    #[inline]
    pub fn is_pwmafs_2(&self) -> bool {
        *self == PWMAFSR::PWMAFS_2
    }
    #[doc = "Checks if the value of the field is `PWMAFS_3`"]
    #[inline]
    pub fn is_pwmafs_3(&self) -> bool {
        *self == PWMAFSR::PWMAFS_3
    }
}
#[doc = "Possible values of the field `POLX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLXR {
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    POLX_0,
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    POLX_1,
}
impl POLXR {
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
            POLXR::POLX_0 => false,
            POLXR::POLX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POLXR {
        match value {
            false => POLXR::POLX_0,
            true => POLXR::POLX_1,
        }
    }
    #[doc = "Checks if the value of the field is `POLX_0`"]
    #[inline]
    pub fn is_polx_0(&self) -> bool {
        *self == POLXR::POLX_0
    }
    #[doc = "Checks if the value of the field is `POLX_1`"]
    #[inline]
    pub fn is_polx_1(&self) -> bool {
        *self == POLXR::POLX_1
    }
}
#[doc = "Possible values of the field `POLB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLBR {
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    POLB_0,
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    POLB_1,
}
impl POLBR {
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
            POLBR::POLB_0 => false,
            POLBR::POLB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POLBR {
        match value {
            false => POLBR::POLB_0,
            true => POLBR::POLB_1,
        }
    }
    #[doc = "Checks if the value of the field is `POLB_0`"]
    #[inline]
    pub fn is_polb_0(&self) -> bool {
        *self == POLBR::POLB_0
    }
    #[doc = "Checks if the value of the field is `POLB_1`"]
    #[inline]
    pub fn is_polb_1(&self) -> bool {
        *self == POLBR::POLB_1
    }
}
#[doc = "Possible values of the field `POLA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLAR {
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    POLA_0,
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    POLA_1,
}
impl POLAR {
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
            POLAR::POLA_0 => false,
            POLAR::POLA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POLAR {
        match value {
            false => POLAR::POLA_0,
            true => POLAR::POLA_1,
        }
    }
    #[doc = "Checks if the value of the field is `POLA_0`"]
    #[inline]
    pub fn is_pola_0(&self) -> bool {
        *self == POLAR::POLA_0
    }
    #[doc = "Checks if the value of the field is `POLA_1`"]
    #[inline]
    pub fn is_pola_1(&self) -> bool {
        *self == POLAR::POLA_1
    }
}
#[doc = r" Value of the field"]
pub struct PWMX_INR {
    bits: bool,
}
impl PWMX_INR {
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
pub struct PWMB_INR {
    bits: bool,
}
impl PWMB_INR {
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
pub struct PWMA_INR {
    bits: bool,
}
impl PWMA_INR {
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
#[doc = "Values that can be written to the field `PWMXFS`"]
pub enum PWMXFSW {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMXFS_0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMXFS_1,
    #[doc = "Output is tristated."]
    PWMXFS_2,
    #[doc = "Output is tristated."]
    PWMXFS_3,
}
impl PWMXFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMXFSW::PWMXFS_0 => 0,
            PWMXFSW::PWMXFS_1 => 1,
            PWMXFSW::PWMXFS_2 => 2,
            PWMXFSW::PWMXFS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMXFSW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMXFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMXFSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline]
    pub fn pwmxfs_0(self) -> &'a mut W {
        self.variant(PWMXFSW::PWMXFS_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline]
    pub fn pwmxfs_1(self) -> &'a mut W {
        self.variant(PWMXFSW::PWMXFS_1)
    }
    #[doc = "Output is tristated."]
    #[inline]
    pub fn pwmxfs_2(self) -> &'a mut W {
        self.variant(PWMXFSW::PWMXFS_2)
    }
    #[doc = "Output is tristated."]
    #[inline]
    pub fn pwmxfs_3(self) -> &'a mut W {
        self.variant(PWMXFSW::PWMXFS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMBFS`"]
pub enum PWMBFSW {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMBFS_0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMBFS_1,
    #[doc = "Output is tristated."]
    PWMBFS_2,
    #[doc = "Output is tristated."]
    PWMBFS_3,
}
impl PWMBFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMBFSW::PWMBFS_0 => 0,
            PWMBFSW::PWMBFS_1 => 1,
            PWMBFSW::PWMBFS_2 => 2,
            PWMBFSW::PWMBFS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMBFSW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMBFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMBFSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline]
    pub fn pwmbfs_0(self) -> &'a mut W {
        self.variant(PWMBFSW::PWMBFS_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline]
    pub fn pwmbfs_1(self) -> &'a mut W {
        self.variant(PWMBFSW::PWMBFS_1)
    }
    #[doc = "Output is tristated."]
    #[inline]
    pub fn pwmbfs_2(self) -> &'a mut W {
        self.variant(PWMBFSW::PWMBFS_2)
    }
    #[doc = "Output is tristated."]
    #[inline]
    pub fn pwmbfs_3(self) -> &'a mut W {
        self.variant(PWMBFSW::PWMBFS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMAFS`"]
pub enum PWMAFSW {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMAFS_0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMAFS_1,
    #[doc = "Output is tristated."]
    PWMAFS_2,
    #[doc = "Output is tristated."]
    PWMAFS_3,
}
impl PWMAFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMAFSW::PWMAFS_0 => 0,
            PWMAFSW::PWMAFS_1 => 1,
            PWMAFSW::PWMAFS_2 => 2,
            PWMAFSW::PWMAFS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMAFSW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMAFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMAFSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline]
    pub fn pwmafs_0(self) -> &'a mut W {
        self.variant(PWMAFSW::PWMAFS_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline]
    pub fn pwmafs_1(self) -> &'a mut W {
        self.variant(PWMAFSW::PWMAFS_1)
    }
    #[doc = "Output is tristated."]
    #[inline]
    pub fn pwmafs_2(self) -> &'a mut W {
        self.variant(PWMAFSW::PWMAFS_2)
    }
    #[doc = "Output is tristated."]
    #[inline]
    pub fn pwmafs_3(self) -> &'a mut W {
        self.variant(PWMAFSW::PWMAFS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POLX`"]
pub enum POLXW {
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    POLX_0,
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    POLX_1,
}
impl POLXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POLXW::POLX_0 => false,
            POLXW::POLX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POLXW<'a> {
    w: &'a mut W,
}
impl<'a> _POLXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POLXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    #[inline]
    pub fn polx_0(self) -> &'a mut W {
        self.variant(POLXW::POLX_0)
    }
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    #[inline]
    pub fn polx_1(self) -> &'a mut W {
        self.variant(POLXW::POLX_1)
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
#[doc = "Values that can be written to the field `POLB`"]
pub enum POLBW {
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    POLB_0,
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    POLB_1,
}
impl POLBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POLBW::POLB_0 => false,
            POLBW::POLB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POLBW<'a> {
    w: &'a mut W,
}
impl<'a> _POLBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POLBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    #[inline]
    pub fn polb_0(self) -> &'a mut W {
        self.variant(POLBW::POLB_0)
    }
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    #[inline]
    pub fn polb_1(self) -> &'a mut W {
        self.variant(POLBW::POLB_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POLA`"]
pub enum POLAW {
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    POLA_0,
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    POLA_1,
}
impl POLAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POLAW::POLA_0 => false,
            POLAW::POLA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POLAW<'a> {
    w: &'a mut W,
}
impl<'a> _POLAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POLAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    #[inline]
    pub fn pola_0(self) -> &'a mut W {
        self.variant(POLAW::POLA_0)
    }
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    #[inline]
    pub fn pola_1(self) -> &'a mut W {
        self.variant(POLAW::POLA_1)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - PWM_X Fault State"]
    #[inline]
    pub fn pwmxfs(&self) -> PWMXFSR {
        PWMXFSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - PWM_B Fault State"]
    #[inline]
    pub fn pwmbfs(&self) -> PWMBFSR {
        PWMBFSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - PWM_A Fault State"]
    #[inline]
    pub fn pwmafs(&self) -> PWMAFSR {
        PWMAFSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - PWM_X Output Polarity"]
    #[inline]
    pub fn polx(&self) -> POLXR {
        POLXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - PWM_B Output Polarity"]
    #[inline]
    pub fn polb(&self) -> POLBR {
        POLBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - PWM_A Output Polarity"]
    #[inline]
    pub fn pola(&self) -> POLAR {
        POLAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - PWM_X Input"]
    #[inline]
    pub fn pwmx_in(&self) -> PWMX_INR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PWMX_INR { bits }
    }
    #[doc = "Bit 14 - PWM_B Input"]
    #[inline]
    pub fn pwmb_in(&self) -> PWMB_INR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PWMB_INR { bits }
    }
    #[doc = "Bit 15 - PWM_A Input"]
    #[inline]
    pub fn pwma_in(&self) -> PWMA_INR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PWMA_INR { bits }
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
    #[doc = "Bits 0:1 - PWM_X Fault State"]
    #[inline]
    pub fn pwmxfs(&mut self) -> _PWMXFSW {
        _PWMXFSW { w: self }
    }
    #[doc = "Bits 2:3 - PWM_B Fault State"]
    #[inline]
    pub fn pwmbfs(&mut self) -> _PWMBFSW {
        _PWMBFSW { w: self }
    }
    #[doc = "Bits 4:5 - PWM_A Fault State"]
    #[inline]
    pub fn pwmafs(&mut self) -> _PWMAFSW {
        _PWMAFSW { w: self }
    }
    #[doc = "Bit 8 - PWM_X Output Polarity"]
    #[inline]
    pub fn polx(&mut self) -> _POLXW {
        _POLXW { w: self }
    }
    #[doc = "Bit 9 - PWM_B Output Polarity"]
    #[inline]
    pub fn polb(&mut self) -> _POLBW {
        _POLBW { w: self }
    }
    #[doc = "Bit 10 - PWM_A Output Polarity"]
    #[inline]
    pub fn pola(&mut self) -> _POLAW {
        _POLAW { w: self }
    }
}
