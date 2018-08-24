#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPTDCR {
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
#[doc = "Possible values of the field `SRTCR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTCR_ENR {
    #[doc = "SRTC rollover is disabled."]
    SRTCR_EN_0,
    #[doc = "SRTC rollover is enabled."]
    SRTCR_EN_1,
}
impl SRTCR_ENR {
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
            SRTCR_ENR::SRTCR_EN_0 => false,
            SRTCR_ENR::SRTCR_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRTCR_ENR {
        match value {
            false => SRTCR_ENR::SRTCR_EN_0,
            true => SRTCR_ENR::SRTCR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTCR_EN_0`"]
    #[inline]
    pub fn is_srtcr_en_0(&self) -> bool {
        *self == SRTCR_ENR::SRTCR_EN_0
    }
    #[doc = "Checks if the value of the field is `SRTCR_EN_1`"]
    #[inline]
    pub fn is_srtcr_en_1(&self) -> bool {
        *self == SRTCR_ENR::SRTCR_EN_1
    }
}
#[doc = "Possible values of the field `MCR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCR_ENR {
    #[doc = "MC rollover is disabled."]
    MCR_EN_0,
    #[doc = "MC rollover is enabled."]
    MCR_EN_1,
}
impl MCR_ENR {
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
            MCR_ENR::MCR_EN_0 => false,
            MCR_ENR::MCR_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCR_ENR {
        match value {
            false => MCR_ENR::MCR_EN_0,
            true => MCR_ENR::MCR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCR_EN_0`"]
    #[inline]
    pub fn is_mcr_en_0(&self) -> bool {
        *self == MCR_ENR::MCR_EN_0
    }
    #[doc = "Checks if the value of the field is `MCR_EN_1`"]
    #[inline]
    pub fn is_mcr_en_1(&self) -> bool {
        *self == MCR_ENR::MCR_EN_1
    }
}
#[doc = "Possible values of the field `ET1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET1_ENR {
    #[doc = "External tamper 1 is disabled."]
    ET1_EN_0,
    #[doc = "External tamper 1 is enabled."]
    ET1_EN_1,
}
impl ET1_ENR {
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
            ET1_ENR::ET1_EN_0 => false,
            ET1_ENR::ET1_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET1_ENR {
        match value {
            false => ET1_ENR::ET1_EN_0,
            true => ET1_ENR::ET1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ET1_EN_0`"]
    #[inline]
    pub fn is_et1_en_0(&self) -> bool {
        *self == ET1_ENR::ET1_EN_0
    }
    #[doc = "Checks if the value of the field is `ET1_EN_1`"]
    #[inline]
    pub fn is_et1_en_1(&self) -> bool {
        *self == ET1_ENR::ET1_EN_1
    }
}
#[doc = "Possible values of the field `ET1P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET1PR {
    #[doc = "External tamper 1 is active low."]
    ET1P_0,
    #[doc = "External tamper 1 is active high."]
    ET1P_1,
}
impl ET1PR {
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
            ET1PR::ET1P_0 => false,
            ET1PR::ET1P_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ET1PR {
        match value {
            false => ET1PR::ET1P_0,
            true => ET1PR::ET1P_1,
        }
    }
    #[doc = "Checks if the value of the field is `ET1P_0`"]
    #[inline]
    pub fn is_et1p_0(&self) -> bool {
        *self == ET1PR::ET1P_0
    }
    #[doc = "Checks if the value of the field is `ET1P_1`"]
    #[inline]
    pub fn is_et1p_1(&self) -> bool {
        *self == ET1PR::ET1P_1
    }
}
#[doc = r" Value of the field"]
pub struct PFD_OBSERVR {
    bits: bool,
}
impl PFD_OBSERVR {
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
pub struct POR_OBSERVR {
    bits: bool,
}
impl POR_OBSERVR {
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
#[doc = "Possible values of the field `OSCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCBR {
    #[doc = "Normal SRTC clock oscillator not bypassed."]
    OSCB_0,
    #[doc = "Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
    OSCB_1,
}
impl OSCBR {
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
            OSCBR::OSCB_0 => false,
            OSCBR::OSCB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCBR {
        match value {
            false => OSCBR::OSCB_0,
            true => OSCBR::OSCB_1,
        }
    }
    #[doc = "Checks if the value of the field is `OSCB_0`"]
    #[inline]
    pub fn is_oscb_0(&self) -> bool {
        *self == OSCBR::OSCB_0
    }
    #[doc = "Checks if the value of the field is `OSCB_1`"]
    #[inline]
    pub fn is_oscb_1(&self) -> bool {
        *self == OSCBR::OSCB_1
    }
}
#[doc = "Values that can be written to the field `SRTCR_EN`"]
pub enum SRTCR_ENW {
    #[doc = "SRTC rollover is disabled."]
    SRTCR_EN_0,
    #[doc = "SRTC rollover is enabled."]
    SRTCR_EN_1,
}
impl SRTCR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRTCR_ENW::SRTCR_EN_0 => false,
            SRTCR_ENW::SRTCR_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRTCR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRTCR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRTCR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRTC rollover is disabled."]
    #[inline]
    pub fn srtcr_en_0(self) -> &'a mut W {
        self.variant(SRTCR_ENW::SRTCR_EN_0)
    }
    #[doc = "SRTC rollover is enabled."]
    #[inline]
    pub fn srtcr_en_1(self) -> &'a mut W {
        self.variant(SRTCR_ENW::SRTCR_EN_1)
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
#[doc = "Values that can be written to the field `MCR_EN`"]
pub enum MCR_ENW {
    #[doc = "MC rollover is disabled."]
    MCR_EN_0,
    #[doc = "MC rollover is enabled."]
    MCR_EN_1,
}
impl MCR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCR_ENW::MCR_EN_0 => false,
            MCR_ENW::MCR_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MC rollover is disabled."]
    #[inline]
    pub fn mcr_en_0(self) -> &'a mut W {
        self.variant(MCR_ENW::MCR_EN_0)
    }
    #[doc = "MC rollover is enabled."]
    #[inline]
    pub fn mcr_en_1(self) -> &'a mut W {
        self.variant(MCR_ENW::MCR_EN_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ET1_EN`"]
pub enum ET1_ENW {
    #[doc = "External tamper 1 is disabled."]
    ET1_EN_0,
    #[doc = "External tamper 1 is enabled."]
    ET1_EN_1,
}
impl ET1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET1_ENW::ET1_EN_0 => false,
            ET1_ENW::ET1_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ET1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External tamper 1 is disabled."]
    #[inline]
    pub fn et1_en_0(self) -> &'a mut W {
        self.variant(ET1_ENW::ET1_EN_0)
    }
    #[doc = "External tamper 1 is enabled."]
    #[inline]
    pub fn et1_en_1(self) -> &'a mut W {
        self.variant(ET1_ENW::ET1_EN_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ET1P`"]
pub enum ET1PW {
    #[doc = "External tamper 1 is active low."]
    ET1P_0,
    #[doc = "External tamper 1 is active high."]
    ET1P_1,
}
impl ET1PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ET1PW::ET1P_0 => false,
            ET1PW::ET1P_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ET1PW<'a> {
    w: &'a mut W,
}
impl<'a> _ET1PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ET1PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External tamper 1 is active low."]
    #[inline]
    pub fn et1p_0(self) -> &'a mut W {
        self.variant(ET1PW::ET1P_0)
    }
    #[doc = "External tamper 1 is active high."]
    #[inline]
    pub fn et1p_1(self) -> &'a mut W {
        self.variant(ET1PW::ET1P_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PFD_OBSERVW<'a> {
    w: &'a mut W,
}
impl<'a> _PFD_OBSERVW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POR_OBSERVW<'a> {
    w: &'a mut W,
}
impl<'a> _POR_OBSERVW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCB`"]
pub enum OSCBW {
    #[doc = "Normal SRTC clock oscillator not bypassed."]
    OSCB_0,
    #[doc = "Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
    OSCB_1,
}
impl OSCBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCBW::OSCB_0 => false,
            OSCBW::OSCB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCBW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal SRTC clock oscillator not bypassed."]
    #[inline]
    pub fn oscb_0(self) -> &'a mut W {
        self.variant(OSCBW::OSCB_0)
    }
    #[doc = "Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
    #[inline]
    pub fn oscb_1(self) -> &'a mut W {
        self.variant(OSCBW::OSCB_1)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 1 - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline]
    pub fn srtcr_en(&self) -> SRTCR_ENR {
        SRTCR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline]
    pub fn mcr_en(&self) -> MCR_ENR {
        MCR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[inline]
    pub fn et1_en(&self) -> ET1_ENR {
        ET1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[inline]
    pub fn et1p(&self) -> ET1PR {
        ET1PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline]
    pub fn pfd_observ(&self) -> PFD_OBSERVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFD_OBSERVR { bits }
    }
    #[doc = "Bit 15 - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline]
    pub fn por_observ(&self) -> POR_OBSERVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POR_OBSERVR { bits }
    }
    #[doc = "Bit 28 - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline]
    pub fn oscb(&self) -> OSCBR {
        OSCBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 1 - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline]
    pub fn srtcr_en(&mut self) -> _SRTCR_ENW {
        _SRTCR_ENW { w: self }
    }
    #[doc = "Bit 2 - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline]
    pub fn mcr_en(&mut self) -> _MCR_ENW {
        _MCR_ENW { w: self }
    }
    #[doc = "Bit 9 - External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[inline]
    pub fn et1_en(&mut self) -> _ET1_ENW {
        _ET1_ENW { w: self }
    }
    #[doc = "Bit 11 - External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[inline]
    pub fn et1p(&mut self) -> _ET1PW {
        _ET1PW { w: self }
    }
    #[doc = "Bit 14 - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline]
    pub fn pfd_observ(&mut self) -> _PFD_OBSERVW {
        _PFD_OBSERVW { w: self }
    }
    #[doc = "Bit 15 - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline]
    pub fn por_observ(&mut self) -> _POR_OBSERVW {
        _POR_OBSERVW { w: self }
    }
    #[doc = "Bit 28 - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline]
    pub fn oscb(&mut self) -> _OSCBW {
        _OSCBW { w: self }
    }
}
