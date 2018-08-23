#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRL2 {
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
#[doc = "Possible values of the field `UPDHLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDHLDR {
    #[doc = "Disable updates of hold registers on rising edge of TRIGGER"]
    UPDHLD_0,
    #[doc = "Enable updates of hold registers on rising edge of TRIGGER"]
    UPDHLD_1,
}
impl UPDHLDR {
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
            UPDHLDR::UPDHLD_0 => false,
            UPDHLDR::UPDHLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPDHLDR {
        match value {
            false => UPDHLDR::UPDHLD_0,
            true => UPDHLDR::UPDHLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDHLD_0`"]
    #[inline]
    pub fn is_updhld_0(&self) -> bool {
        *self == UPDHLDR::UPDHLD_0
    }
    #[doc = "Checks if the value of the field is `UPDHLD_1`"]
    #[inline]
    pub fn is_updhld_1(&self) -> bool {
        *self == UPDHLDR::UPDHLD_1
    }
}
#[doc = "Possible values of the field `UPDPOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDPOSR {
    #[doc = "No action for POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    UPDPOS_0,
    #[doc = "Clear POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    UPDPOS_1,
}
impl UPDPOSR {
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
            UPDPOSR::UPDPOS_0 => false,
            UPDPOSR::UPDPOS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPDPOSR {
        match value {
            false => UPDPOSR::UPDPOS_0,
            true => UPDPOSR::UPDPOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDPOS_0`"]
    #[inline]
    pub fn is_updpos_0(&self) -> bool {
        *self == UPDPOSR::UPDPOS_0
    }
    #[doc = "Checks if the value of the field is `UPDPOS_1`"]
    #[inline]
    pub fn is_updpos_1(&self) -> bool {
        *self == UPDPOSR::UPDPOS_1
    }
}
#[doc = "Possible values of the field `MOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODR {
    #[doc = "Disable modulo counting"]
    MOD_0,
    #[doc = "Enable modulo counting"]
    MOD_1,
}
impl MODR {
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
            MODR::MOD_0 => false,
            MODR::MOD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODR {
        match value {
            false => MODR::MOD_0,
            true => MODR::MOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_0`"]
    #[inline]
    pub fn is_mod_0(&self) -> bool {
        *self == MODR::MOD_0
    }
    #[doc = "Checks if the value of the field is `MOD_1`"]
    #[inline]
    pub fn is_mod_1(&self) -> bool {
        *self == MODR::MOD_1
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Last count was in the down direction"]
    DIR_0,
    #[doc = "Last count was in the up direction"]
    DIR_1,
}
impl DIRR {
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
            DIRR::DIR_0 => false,
            DIRR::DIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::DIR_0,
            true => DIRR::DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIR_0`"]
    #[inline]
    pub fn is_dir_0(&self) -> bool {
        *self == DIRR::DIR_0
    }
    #[doc = "Checks if the value of the field is `DIR_1`"]
    #[inline]
    pub fn is_dir_1(&self) -> bool {
        *self == DIRR::DIR_1
    }
}
#[doc = "Possible values of the field `RUIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUIER {
    #[doc = "Roll-under interrupt is disabled"]
    RUIE_0,
    #[doc = "Roll-under interrupt is enabled"]
    RUIE_1,
}
impl RUIER {
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
            RUIER::RUIE_0 => false,
            RUIER::RUIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUIER {
        match value {
            false => RUIER::RUIE_0,
            true => RUIER::RUIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUIE_0`"]
    #[inline]
    pub fn is_ruie_0(&self) -> bool {
        *self == RUIER::RUIE_0
    }
    #[doc = "Checks if the value of the field is `RUIE_1`"]
    #[inline]
    pub fn is_ruie_1(&self) -> bool {
        *self == RUIER::RUIE_1
    }
}
#[doc = "Possible values of the field `RUIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUIRQR {
    #[doc = "No roll-under has occurred"]
    RUIRQ_0,
    #[doc = "Roll-under has occurred"]
    RUIRQ_1,
}
impl RUIRQR {
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
            RUIRQR::RUIRQ_0 => false,
            RUIRQR::RUIRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUIRQR {
        match value {
            false => RUIRQR::RUIRQ_0,
            true => RUIRQR::RUIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUIRQ_0`"]
    #[inline]
    pub fn is_ruirq_0(&self) -> bool {
        *self == RUIRQR::RUIRQ_0
    }
    #[doc = "Checks if the value of the field is `RUIRQ_1`"]
    #[inline]
    pub fn is_ruirq_1(&self) -> bool {
        *self == RUIRQR::RUIRQ_1
    }
}
#[doc = "Possible values of the field `ROIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROIER {
    #[doc = "Roll-over interrupt is disabled"]
    ROIE_0,
    #[doc = "Roll-over interrupt is enabled"]
    ROIE_1,
}
impl ROIER {
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
            ROIER::ROIE_0 => false,
            ROIER::ROIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROIER {
        match value {
            false => ROIER::ROIE_0,
            true => ROIER::ROIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROIE_0`"]
    #[inline]
    pub fn is_roie_0(&self) -> bool {
        *self == ROIER::ROIE_0
    }
    #[doc = "Checks if the value of the field is `ROIE_1`"]
    #[inline]
    pub fn is_roie_1(&self) -> bool {
        *self == ROIER::ROIE_1
    }
}
#[doc = "Possible values of the field `ROIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROIRQR {
    #[doc = "No roll-over has occurred"]
    ROIRQ_0,
    #[doc = "Roll-over has occurred"]
    ROIRQ_1,
}
impl ROIRQR {
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
            ROIRQR::ROIRQ_0 => false,
            ROIRQR::ROIRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROIRQR {
        match value {
            false => ROIRQR::ROIRQ_0,
            true => ROIRQR::ROIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROIRQ_0`"]
    #[inline]
    pub fn is_roirq_0(&self) -> bool {
        *self == ROIRQR::ROIRQ_0
    }
    #[doc = "Checks if the value of the field is `ROIRQ_1`"]
    #[inline]
    pub fn is_roirq_1(&self) -> bool {
        *self == ROIRQR::ROIRQ_1
    }
}
#[doc = "Possible values of the field `REVMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVMODR {
    #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)."]
    REVMOD_0,
    #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)."]
    REVMOD_1,
}
impl REVMODR {
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
            REVMODR::REVMOD_0 => false,
            REVMODR::REVMOD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REVMODR {
        match value {
            false => REVMODR::REVMOD_0,
            true => REVMODR::REVMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `REVMOD_0`"]
    #[inline]
    pub fn is_revmod_0(&self) -> bool {
        *self == REVMODR::REVMOD_0
    }
    #[doc = "Checks if the value of the field is `REVMOD_1`"]
    #[inline]
    pub fn is_revmod_1(&self) -> bool {
        *self == REVMODR::REVMOD_1
    }
}
#[doc = "Possible values of the field `OUTCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCTLR {
    #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the compare value (COMP)."]
    OUTCTL_0,
    #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read."]
    OUTCTL_1,
}
impl OUTCTLR {
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
            OUTCTLR::OUTCTL_0 => false,
            OUTCTLR::OUTCTL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTCTLR {
        match value {
            false => OUTCTLR::OUTCTL_0,
            true => OUTCTLR::OUTCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUTCTL_0`"]
    #[inline]
    pub fn is_outctl_0(&self) -> bool {
        *self == OUTCTLR::OUTCTL_0
    }
    #[doc = "Checks if the value of the field is `OUTCTL_1`"]
    #[inline]
    pub fn is_outctl_1(&self) -> bool {
        *self == OUTCTLR::OUTCTL_1
    }
}
#[doc = "Possible values of the field `SABIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABIER {
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt disabled."]
    SABIE_0,
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt enabled."]
    SABIE_1,
}
impl SABIER {
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
            SABIER::SABIE_0 => false,
            SABIER::SABIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SABIER {
        match value {
            false => SABIER::SABIE_0,
            true => SABIER::SABIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SABIE_0`"]
    #[inline]
    pub fn is_sabie_0(&self) -> bool {
        *self == SABIER::SABIE_0
    }
    #[doc = "Checks if the value of the field is `SABIE_1`"]
    #[inline]
    pub fn is_sabie_1(&self) -> bool {
        *self == SABIER::SABIE_1
    }
}
#[doc = "Possible values of the field `SABIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABIRQR {
    #[doc = "No simultaneous change of PHASEA and PHASEB has occurred."]
    SABIRQ_0,
    #[doc = "A simultaneous change of PHASEA and PHASEB has occurred."]
    SABIRQ_1,
}
impl SABIRQR {
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
            SABIRQR::SABIRQ_0 => false,
            SABIRQR::SABIRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SABIRQR {
        match value {
            false => SABIRQR::SABIRQ_0,
            true => SABIRQR::SABIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SABIRQ_0`"]
    #[inline]
    pub fn is_sabirq_0(&self) -> bool {
        *self == SABIRQR::SABIRQ_0
    }
    #[doc = "Checks if the value of the field is `SABIRQ_1`"]
    #[inline]
    pub fn is_sabirq_1(&self) -> bool {
        *self == SABIRQR::SABIRQ_1
    }
}
#[doc = "Values that can be written to the field `UPDHLD`"]
pub enum UPDHLDW {
    #[doc = "Disable updates of hold registers on rising edge of TRIGGER"]
    UPDHLD_0,
    #[doc = "Enable updates of hold registers on rising edge of TRIGGER"]
    UPDHLD_1,
}
impl UPDHLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPDHLDW::UPDHLD_0 => false,
            UPDHLDW::UPDHLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDHLDW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDHLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDHLDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable updates of hold registers on rising edge of TRIGGER"]
    #[inline]
    pub fn updhld_0(self) -> &'a mut W {
        self.variant(UPDHLDW::UPDHLD_0)
    }
    #[doc = "Enable updates of hold registers on rising edge of TRIGGER"]
    #[inline]
    pub fn updhld_1(self) -> &'a mut W {
        self.variant(UPDHLDW::UPDHLD_1)
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
#[doc = "Values that can be written to the field `UPDPOS`"]
pub enum UPDPOSW {
    #[doc = "No action for POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    UPDPOS_0,
    #[doc = "Clear POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    UPDPOS_1,
}
impl UPDPOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPDPOSW::UPDPOS_0 => false,
            UPDPOSW::UPDPOS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDPOSW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDPOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDPOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action for POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    #[inline]
    pub fn updpos_0(self) -> &'a mut W {
        self.variant(UPDPOSW::UPDPOS_0)
    }
    #[doc = "Clear POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    #[inline]
    pub fn updpos_1(self) -> &'a mut W {
        self.variant(UPDPOSW::UPDPOS_1)
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
#[doc = "Values that can be written to the field `MOD`"]
pub enum MODW {
    #[doc = "Disable modulo counting"]
    MOD_0,
    #[doc = "Enable modulo counting"]
    MOD_1,
}
impl MODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODW::MOD_0 => false,
            MODW::MOD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODW<'a> {
    w: &'a mut W,
}
impl<'a> _MODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable modulo counting"]
    #[inline]
    pub fn mod_0(self) -> &'a mut W {
        self.variant(MODW::MOD_0)
    }
    #[doc = "Enable modulo counting"]
    #[inline]
    pub fn mod_1(self) -> &'a mut W {
        self.variant(MODW::MOD_1)
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
#[doc = "Values that can be written to the field `RUIE`"]
pub enum RUIEW {
    #[doc = "Roll-under interrupt is disabled"]
    RUIE_0,
    #[doc = "Roll-under interrupt is enabled"]
    RUIE_1,
}
impl RUIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RUIEW::RUIE_0 => false,
            RUIEW::RUIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RUIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Roll-under interrupt is disabled"]
    #[inline]
    pub fn ruie_0(self) -> &'a mut W {
        self.variant(RUIEW::RUIE_0)
    }
    #[doc = "Roll-under interrupt is enabled"]
    #[inline]
    pub fn ruie_1(self) -> &'a mut W {
        self.variant(RUIEW::RUIE_1)
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
#[doc = "Values that can be written to the field `RUIRQ`"]
pub enum RUIRQW {
    #[doc = "No roll-under has occurred"]
    RUIRQ_0,
    #[doc = "Roll-under has occurred"]
    RUIRQ_1,
}
impl RUIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RUIRQW::RUIRQ_0 => false,
            RUIRQW::RUIRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RUIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No roll-under has occurred"]
    #[inline]
    pub fn ruirq_0(self) -> &'a mut W {
        self.variant(RUIRQW::RUIRQ_0)
    }
    #[doc = "Roll-under has occurred"]
    #[inline]
    pub fn ruirq_1(self) -> &'a mut W {
        self.variant(RUIRQW::RUIRQ_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROIE`"]
pub enum ROIEW {
    #[doc = "Roll-over interrupt is disabled"]
    ROIE_0,
    #[doc = "Roll-over interrupt is enabled"]
    ROIE_1,
}
impl ROIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROIEW::ROIE_0 => false,
            ROIEW::ROIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ROIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Roll-over interrupt is disabled"]
    #[inline]
    pub fn roie_0(self) -> &'a mut W {
        self.variant(ROIEW::ROIE_0)
    }
    #[doc = "Roll-over interrupt is enabled"]
    #[inline]
    pub fn roie_1(self) -> &'a mut W {
        self.variant(ROIEW::ROIE_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROIRQ`"]
pub enum ROIRQW {
    #[doc = "No roll-over has occurred"]
    ROIRQ_0,
    #[doc = "Roll-over has occurred"]
    ROIRQ_1,
}
impl ROIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROIRQW::ROIRQ_0 => false,
            ROIRQW::ROIRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ROIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No roll-over has occurred"]
    #[inline]
    pub fn roirq_0(self) -> &'a mut W {
        self.variant(ROIRQW::ROIRQ_0)
    }
    #[doc = "Roll-over has occurred"]
    #[inline]
    pub fn roirq_1(self) -> &'a mut W {
        self.variant(ROIRQW::ROIRQ_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REVMOD`"]
pub enum REVMODW {
    #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)."]
    REVMOD_0,
    #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)."]
    REVMOD_1,
}
impl REVMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REVMODW::REVMOD_0 => false,
            REVMODW::REVMOD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REVMODW<'a> {
    w: &'a mut W,
}
impl<'a> _REVMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REVMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)."]
    #[inline]
    pub fn revmod_0(self) -> &'a mut W {
        self.variant(REVMODW::REVMOD_0)
    }
    #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)."]
    #[inline]
    pub fn revmod_1(self) -> &'a mut W {
        self.variant(REVMODW::REVMOD_1)
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
#[doc = "Values that can be written to the field `OUTCTL`"]
pub enum OUTCTLW {
    #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the compare value (COMP)."]
    OUTCTL_0,
    #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read."]
    OUTCTL_1,
}
impl OUTCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUTCTLW::OUTCTL_0 => false,
            OUTCTLW::OUTCTL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTCTLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the compare value (COMP)."]
    #[inline]
    pub fn outctl_0(self) -> &'a mut W {
        self.variant(OUTCTLW::OUTCTL_0)
    }
    #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read."]
    #[inline]
    pub fn outctl_1(self) -> &'a mut W {
        self.variant(OUTCTLW::OUTCTL_1)
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
#[doc = "Values that can be written to the field `SABIE`"]
pub enum SABIEW {
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt disabled."]
    SABIE_0,
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt enabled."]
    SABIE_1,
}
impl SABIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SABIEW::SABIE_0 => false,
            SABIEW::SABIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SABIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SABIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SABIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt disabled."]
    #[inline]
    pub fn sabie_0(self) -> &'a mut W {
        self.variant(SABIEW::SABIE_0)
    }
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt enabled."]
    #[inline]
    pub fn sabie_1(self) -> &'a mut W {
        self.variant(SABIEW::SABIE_1)
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
#[doc = "Values that can be written to the field `SABIRQ`"]
pub enum SABIRQW {
    #[doc = "No simultaneous change of PHASEA and PHASEB has occurred."]
    SABIRQ_0,
    #[doc = "A simultaneous change of PHASEA and PHASEB has occurred."]
    SABIRQ_1,
}
impl SABIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SABIRQW::SABIRQ_0 => false,
            SABIRQW::SABIRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SABIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SABIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SABIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No simultaneous change of PHASEA and PHASEB has occurred."]
    #[inline]
    pub fn sabirq_0(self) -> &'a mut W {
        self.variant(SABIRQW::SABIRQ_0)
    }
    #[doc = "A simultaneous change of PHASEA and PHASEB has occurred."]
    #[inline]
    pub fn sabirq_1(self) -> &'a mut W {
        self.variant(SABIRQW::SABIRQ_1)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Update Hold Registers"]
    #[inline]
    pub fn updhld(&self) -> UPDHLDR {
        UPDHLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Update Position Registers"]
    #[inline]
    pub fn updpos(&self) -> UPDPOSR {
        UPDPOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Enable Modulo Counting"]
    #[inline]
    pub fn mod_(&self) -> MODR {
        MODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Count Direction Flag"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Roll-under Interrupt Enable"]
    #[inline]
    pub fn ruie(&self) -> RUIER {
        RUIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Roll-under Interrupt Request"]
    #[inline]
    pub fn ruirq(&self) -> RUIRQR {
        RUIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Roll-over Interrupt Enable"]
    #[inline]
    pub fn roie(&self) -> ROIER {
        ROIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Roll-over Interrupt Request"]
    #[inline]
    pub fn roirq(&self) -> ROIRQR {
        ROIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Revolution Counter Modulus Enable"]
    #[inline]
    pub fn revmod(&self) -> REVMODR {
        REVMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Output Control"]
    #[inline]
    pub fn outctl(&self) -> OUTCTLR {
        OUTCTLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[inline]
    pub fn sabie(&self) -> SABIER {
        SABIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[inline]
    pub fn sabirq(&self) -> SABIRQR {
        SABIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Update Hold Registers"]
    #[inline]
    pub fn updhld(&mut self) -> _UPDHLDW {
        _UPDHLDW { w: self }
    }
    #[doc = "Bit 1 - Update Position Registers"]
    #[inline]
    pub fn updpos(&mut self) -> _UPDPOSW {
        _UPDPOSW { w: self }
    }
    #[doc = "Bit 2 - Enable Modulo Counting"]
    #[inline]
    pub fn mod_(&mut self) -> _MODW {
        _MODW { w: self }
    }
    #[doc = "Bit 4 - Roll-under Interrupt Enable"]
    #[inline]
    pub fn ruie(&mut self) -> _RUIEW {
        _RUIEW { w: self }
    }
    #[doc = "Bit 5 - Roll-under Interrupt Request"]
    #[inline]
    pub fn ruirq(&mut self) -> _RUIRQW {
        _RUIRQW { w: self }
    }
    #[doc = "Bit 6 - Roll-over Interrupt Enable"]
    #[inline]
    pub fn roie(&mut self) -> _ROIEW {
        _ROIEW { w: self }
    }
    #[doc = "Bit 7 - Roll-over Interrupt Request"]
    #[inline]
    pub fn roirq(&mut self) -> _ROIRQW {
        _ROIRQW { w: self }
    }
    #[doc = "Bit 8 - Revolution Counter Modulus Enable"]
    #[inline]
    pub fn revmod(&mut self) -> _REVMODW {
        _REVMODW { w: self }
    }
    #[doc = "Bit 9 - Output Control"]
    #[inline]
    pub fn outctl(&mut self) -> _OUTCTLW {
        _OUTCTLW { w: self }
    }
    #[doc = "Bit 10 - Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[inline]
    pub fn sabie(&mut self) -> _SABIEW {
        _SABIEW { w: self }
    }
    #[doc = "Bit 11 - Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[inline]
    pub fn sabirq(&mut self) -> _SABIRQW {
        _SABIRQW { w: self }
    }
}
