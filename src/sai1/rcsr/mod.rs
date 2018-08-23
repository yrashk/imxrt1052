#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCSR {
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
#[doc = "Possible values of the field `FRDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDER {
    #[doc = "Disables the DMA request."]
    FRDE_0,
    #[doc = "Enables the DMA request."]
    FRDE_1,
}
impl FRDER {
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
            FRDER::FRDE_0 => false,
            FRDER::FRDE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRDER {
        match value {
            false => FRDER::FRDE_0,
            true => FRDER::FRDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRDE_0`"]
    #[inline]
    pub fn is_frde_0(&self) -> bool {
        *self == FRDER::FRDE_0
    }
    #[doc = "Checks if the value of the field is `FRDE_1`"]
    #[inline]
    pub fn is_frde_1(&self) -> bool {
        *self == FRDER::FRDE_1
    }
}
#[doc = "Possible values of the field `FWDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWDER {
    #[doc = "Disables the DMA request."]
    FWDE_0,
    #[doc = "Enables the DMA request."]
    FWDE_1,
}
impl FWDER {
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
            FWDER::FWDE_0 => false,
            FWDER::FWDE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWDER {
        match value {
            false => FWDER::FWDE_0,
            true => FWDER::FWDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWDE_0`"]
    #[inline]
    pub fn is_fwde_0(&self) -> bool {
        *self == FWDER::FWDE_0
    }
    #[doc = "Checks if the value of the field is `FWDE_1`"]
    #[inline]
    pub fn is_fwde_1(&self) -> bool {
        *self == FWDER::FWDE_1
    }
}
#[doc = "Possible values of the field `FRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRIER {
    #[doc = "Disables the interrupt."]
    FRIE_0,
    #[doc = "Enables the interrupt."]
    FRIE_1,
}
impl FRIER {
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
            FRIER::FRIE_0 => false,
            FRIER::FRIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRIER {
        match value {
            false => FRIER::FRIE_0,
            true => FRIER::FRIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRIE_0`"]
    #[inline]
    pub fn is_frie_0(&self) -> bool {
        *self == FRIER::FRIE_0
    }
    #[doc = "Checks if the value of the field is `FRIE_1`"]
    #[inline]
    pub fn is_frie_1(&self) -> bool {
        *self == FRIER::FRIE_1
    }
}
#[doc = "Possible values of the field `FWIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWIER {
    #[doc = "Disables the interrupt."]
    FWIE_0,
    #[doc = "Enables the interrupt."]
    FWIE_1,
}
impl FWIER {
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
            FWIER::FWIE_0 => false,
            FWIER::FWIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWIER {
        match value {
            false => FWIER::FWIE_0,
            true => FWIER::FWIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWIE_0`"]
    #[inline]
    pub fn is_fwie_0(&self) -> bool {
        *self == FWIER::FWIE_0
    }
    #[doc = "Checks if the value of the field is `FWIE_1`"]
    #[inline]
    pub fn is_fwie_1(&self) -> bool {
        *self == FWIER::FWIE_1
    }
}
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "Disables the interrupt."]
    FEIE_0,
    #[doc = "Enables the interrupt."]
    FEIE_1,
}
impl FEIER {
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
            FEIER::FEIE_0 => false,
            FEIER::FEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIER {
        match value {
            false => FEIER::FEIE_0,
            true => FEIER::FEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEIE_0`"]
    #[inline]
    pub fn is_feie_0(&self) -> bool {
        *self == FEIER::FEIE_0
    }
    #[doc = "Checks if the value of the field is `FEIE_1`"]
    #[inline]
    pub fn is_feie_1(&self) -> bool {
        *self == FEIER::FEIE_1
    }
}
#[doc = "Possible values of the field `SEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIER {
    #[doc = "Disables interrupt."]
    SEIE_0,
    #[doc = "Enables interrupt."]
    SEIE_1,
}
impl SEIER {
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
            SEIER::SEIE_0 => false,
            SEIER::SEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEIER {
        match value {
            false => SEIER::SEIE_0,
            true => SEIER::SEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEIE_0`"]
    #[inline]
    pub fn is_seie_0(&self) -> bool {
        *self == SEIER::SEIE_0
    }
    #[doc = "Checks if the value of the field is `SEIE_1`"]
    #[inline]
    pub fn is_seie_1(&self) -> bool {
        *self == SEIER::SEIE_1
    }
}
#[doc = "Possible values of the field `WSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSIER {
    #[doc = "Disables interrupt."]
    WSIE_0,
    #[doc = "Enables interrupt."]
    WSIE_1,
}
impl WSIER {
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
            WSIER::WSIE_0 => false,
            WSIER::WSIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSIER {
        match value {
            false => WSIER::WSIE_0,
            true => WSIER::WSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WSIE_0`"]
    #[inline]
    pub fn is_wsie_0(&self) -> bool {
        *self == WSIER::WSIE_0
    }
    #[doc = "Checks if the value of the field is `WSIE_1`"]
    #[inline]
    pub fn is_wsie_1(&self) -> bool {
        *self == WSIER::WSIE_1
    }
}
#[doc = "Possible values of the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFR {
    #[doc = "Receive FIFO watermark not reached."]
    FRF_0,
    #[doc = "Receive FIFO watermark has been reached."]
    FRF_1,
}
impl FRFR {
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
            FRFR::FRF_0 => false,
            FRFR::FRF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRFR {
        match value {
            false => FRFR::FRF_0,
            true => FRFR::FRF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRF_0`"]
    #[inline]
    pub fn is_frf_0(&self) -> bool {
        *self == FRFR::FRF_0
    }
    #[doc = "Checks if the value of the field is `FRF_1`"]
    #[inline]
    pub fn is_frf_1(&self) -> bool {
        *self == FRFR::FRF_1
    }
}
#[doc = "Possible values of the field `FWF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWFR {
    #[doc = "No enabled receive FIFO is full."]
    FWF_0,
    #[doc = "Enabled receive FIFO is full."]
    FWF_1,
}
impl FWFR {
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
            FWFR::FWF_0 => false,
            FWFR::FWF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWFR {
        match value {
            false => FWFR::FWF_0,
            true => FWFR::FWF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWF_0`"]
    #[inline]
    pub fn is_fwf_0(&self) -> bool {
        *self == FWFR::FWF_0
    }
    #[doc = "Checks if the value of the field is `FWF_1`"]
    #[inline]
    pub fn is_fwf_1(&self) -> bool {
        *self == FWFR::FWF_1
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "Receive overflow not detected."]
    FEF_0,
    #[doc = "Receive overflow detected."]
    FEF_1,
}
impl FEFR {
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
            FEFR::FEF_0 => false,
            FEFR::FEF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEFR {
        match value {
            false => FEFR::FEF_0,
            true => FEFR::FEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEF_0`"]
    #[inline]
    pub fn is_fef_0(&self) -> bool {
        *self == FEFR::FEF_0
    }
    #[doc = "Checks if the value of the field is `FEF_1`"]
    #[inline]
    pub fn is_fef_1(&self) -> bool {
        *self == FEFR::FEF_1
    }
}
#[doc = "Possible values of the field `SEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEFR {
    #[doc = "Sync error not detected."]
    SEF_0,
    #[doc = "Frame sync error detected."]
    SEF_1,
}
impl SEFR {
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
            SEFR::SEF_0 => false,
            SEFR::SEF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEFR {
        match value {
            false => SEFR::SEF_0,
            true => SEFR::SEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEF_0`"]
    #[inline]
    pub fn is_sef_0(&self) -> bool {
        *self == SEFR::SEF_0
    }
    #[doc = "Checks if the value of the field is `SEF_1`"]
    #[inline]
    pub fn is_sef_1(&self) -> bool {
        *self == SEFR::SEF_1
    }
}
#[doc = "Possible values of the field `WSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSFR {
    #[doc = "Start of word not detected."]
    WSF_0,
    #[doc = "Start of word detected."]
    WSF_1,
}
impl WSFR {
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
            WSFR::WSF_0 => false,
            WSFR::WSF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSFR {
        match value {
            false => WSFR::WSF_0,
            true => WSFR::WSF_1,
        }
    }
    #[doc = "Checks if the value of the field is `WSF_0`"]
    #[inline]
    pub fn is_wsf_0(&self) -> bool {
        *self == WSFR::WSF_0
    }
    #[doc = "Checks if the value of the field is `WSF_1`"]
    #[inline]
    pub fn is_wsf_1(&self) -> bool {
        *self == WSFR::WSF_1
    }
}
#[doc = "Possible values of the field `SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR {
    #[doc = "No effect."]
    SR_0,
    #[doc = "Software reset."]
    SR_1,
}
impl SRR {
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
            SRR::SR_0 => false,
            SRR::SR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRR {
        match value {
            false => SRR::SR_0,
            true => SRR::SR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SR_0`"]
    #[inline]
    pub fn is_sr_0(&self) -> bool {
        *self == SRR::SR_0
    }
    #[doc = "Checks if the value of the field is `SR_1`"]
    #[inline]
    pub fn is_sr_1(&self) -> bool {
        *self == SRR::SR_1
    }
}
#[doc = "Possible values of the field `BCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCER {
    #[doc = "Receive bit clock is disabled."]
    BCE_0,
    #[doc = "Receive bit clock is enabled."]
    BCE_1,
}
impl BCER {
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
            BCER::BCE_0 => false,
            BCER::BCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCER {
        match value {
            false => BCER::BCE_0,
            true => BCER::BCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCE_0`"]
    #[inline]
    pub fn is_bce_0(&self) -> bool {
        *self == BCER::BCE_0
    }
    #[doc = "Checks if the value of the field is `BCE_1`"]
    #[inline]
    pub fn is_bce_1(&self) -> bool {
        *self == BCER::BCE_1
    }
}
#[doc = "Possible values of the field `DBGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGER {
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    DBGE_0,
    #[doc = "Receiver is enabled in Debug mode."]
    DBGE_1,
}
impl DBGER {
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
            DBGER::DBGE_0 => false,
            DBGER::DBGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGER {
        match value {
            false => DBGER::DBGE_0,
            true => DBGER::DBGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGE_0`"]
    #[inline]
    pub fn is_dbge_0(&self) -> bool {
        *self == DBGER::DBGE_0
    }
    #[doc = "Checks if the value of the field is `DBGE_1`"]
    #[inline]
    pub fn is_dbge_1(&self) -> bool {
        *self == DBGER::DBGE_1
    }
}
#[doc = "Possible values of the field `STOPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPER {
    #[doc = "Receiver disabled in Stop mode."]
    STOPE_0,
    #[doc = "Receiver enabled in Stop mode."]
    STOPE_1,
}
impl STOPER {
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
            STOPER::STOPE_0 => false,
            STOPER::STOPE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPER {
        match value {
            false => STOPER::STOPE_0,
            true => STOPER::STOPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOPE_0`"]
    #[inline]
    pub fn is_stope_0(&self) -> bool {
        *self == STOPER::STOPE_0
    }
    #[doc = "Checks if the value of the field is `STOPE_1`"]
    #[inline]
    pub fn is_stope_1(&self) -> bool {
        *self == STOPER::STOPE_1
    }
}
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "Receiver is disabled."]
    RE_0,
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    RE_1,
}
impl RER {
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
            RER::RE_0 => false,
            RER::RE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RER {
        match value {
            false => RER::RE_0,
            true => RER::RE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RE_0`"]
    #[inline]
    pub fn is_re_0(&self) -> bool {
        *self == RER::RE_0
    }
    #[doc = "Checks if the value of the field is `RE_1`"]
    #[inline]
    pub fn is_re_1(&self) -> bool {
        *self == RER::RE_1
    }
}
#[doc = "Values that can be written to the field `FRDE`"]
pub enum FRDEW {
    #[doc = "Disables the DMA request."]
    FRDE_0,
    #[doc = "Enables the DMA request."]
    FRDE_1,
}
impl FRDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRDEW::FRDE_0 => false,
            FRDEW::FRDE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRDEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline]
    pub fn frde_0(self) -> &'a mut W {
        self.variant(FRDEW::FRDE_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline]
    pub fn frde_1(self) -> &'a mut W {
        self.variant(FRDEW::FRDE_1)
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
#[doc = "Values that can be written to the field `FWDE`"]
pub enum FWDEW {
    #[doc = "Disables the DMA request."]
    FWDE_0,
    #[doc = "Enables the DMA request."]
    FWDE_1,
}
impl FWDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWDEW::FWDE_0 => false,
            FWDEW::FWDE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWDEW<'a> {
    w: &'a mut W,
}
impl<'a> _FWDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline]
    pub fn fwde_0(self) -> &'a mut W {
        self.variant(FWDEW::FWDE_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline]
    pub fn fwde_1(self) -> &'a mut W {
        self.variant(FWDEW::FWDE_1)
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
#[doc = "Values that can be written to the field `FRIE`"]
pub enum FRIEW {
    #[doc = "Disables the interrupt."]
    FRIE_0,
    #[doc = "Enables the interrupt."]
    FRIE_1,
}
impl FRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRIEW::FRIE_0 => false,
            FRIEW::FRIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline]
    pub fn frie_0(self) -> &'a mut W {
        self.variant(FRIEW::FRIE_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline]
    pub fn frie_1(self) -> &'a mut W {
        self.variant(FRIEW::FRIE_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FWIE`"]
pub enum FWIEW {
    #[doc = "Disables the interrupt."]
    FWIE_0,
    #[doc = "Enables the interrupt."]
    FWIE_1,
}
impl FWIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWIEW::FWIE_0 => false,
            FWIEW::FWIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FWIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline]
    pub fn fwie_0(self) -> &'a mut W {
        self.variant(FWIEW::FWIE_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline]
    pub fn fwie_1(self) -> &'a mut W {
        self.variant(FWIEW::FWIE_1)
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
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "Disables the interrupt."]
    FEIE_0,
    #[doc = "Enables the interrupt."]
    FEIE_1,
}
impl FEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEIEW::FEIE_0 => false,
            FEIEW::FEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIEW::FEIE_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline]
    pub fn feie_1(self) -> &'a mut W {
        self.variant(FEIEW::FEIE_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEIE`"]
pub enum SEIEW {
    #[doc = "Disables interrupt."]
    SEIE_0,
    #[doc = "Enables interrupt."]
    SEIE_1,
}
impl SEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEIEW::SEIE_0 => false,
            SEIEW::SEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline]
    pub fn seie_0(self) -> &'a mut W {
        self.variant(SEIEW::SEIE_0)
    }
    #[doc = "Enables interrupt."]
    #[inline]
    pub fn seie_1(self) -> &'a mut W {
        self.variant(SEIEW::SEIE_1)
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
#[doc = "Values that can be written to the field `WSIE`"]
pub enum WSIEW {
    #[doc = "Disables interrupt."]
    WSIE_0,
    #[doc = "Enables interrupt."]
    WSIE_1,
}
impl WSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSIEW::WSIE_0 => false,
            WSIEW::WSIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline]
    pub fn wsie_0(self) -> &'a mut W {
        self.variant(WSIEW::WSIE_0)
    }
    #[doc = "Enables interrupt."]
    #[inline]
    pub fn wsie_1(self) -> &'a mut W {
        self.variant(WSIEW::WSIE_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "Receive overflow not detected."]
    FEF_0,
    #[doc = "Receive overflow detected."]
    FEF_1,
}
impl FEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEFW::FEF_0 => false,
            FEFW::FEF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _FEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive overflow not detected."]
    #[inline]
    pub fn fef_0(self) -> &'a mut W {
        self.variant(FEFW::FEF_0)
    }
    #[doc = "Receive overflow detected."]
    #[inline]
    pub fn fef_1(self) -> &'a mut W {
        self.variant(FEFW::FEF_1)
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
#[doc = "Values that can be written to the field `SEF`"]
pub enum SEFW {
    #[doc = "Sync error not detected."]
    SEF_0,
    #[doc = "Frame sync error detected."]
    SEF_1,
}
impl SEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEFW::SEF_0 => false,
            SEFW::SEF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEFW<'a> {
    w: &'a mut W,
}
impl<'a> _SEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sync error not detected."]
    #[inline]
    pub fn sef_0(self) -> &'a mut W {
        self.variant(SEFW::SEF_0)
    }
    #[doc = "Frame sync error detected."]
    #[inline]
    pub fn sef_1(self) -> &'a mut W {
        self.variant(SEFW::SEF_1)
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
#[doc = "Values that can be written to the field `WSF`"]
pub enum WSFW {
    #[doc = "Start of word not detected."]
    WSF_0,
    #[doc = "Start of word detected."]
    WSF_1,
}
impl WSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSFW::WSF_0 => false,
            WSFW::WSF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSFW<'a> {
    w: &'a mut W,
}
impl<'a> _WSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start of word not detected."]
    #[inline]
    pub fn wsf_0(self) -> &'a mut W {
        self.variant(WSFW::WSF_0)
    }
    #[doc = "Start of word detected."]
    #[inline]
    pub fn wsf_1(self) -> &'a mut W {
        self.variant(WSFW::WSF_1)
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
#[doc = "Values that can be written to the field `SR`"]
pub enum SRW {
    #[doc = "No effect."]
    SR_0,
    #[doc = "Software reset."]
    SR_1,
}
impl SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRW::SR_0 => false,
            SRW::SR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn sr_0(self) -> &'a mut W {
        self.variant(SRW::SR_0)
    }
    #[doc = "Software reset."]
    #[inline]
    pub fn sr_1(self) -> &'a mut W {
        self.variant(SRW::SR_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FR`"]
pub enum FRW {
    #[doc = "No effect."]
    FR_0,
    #[doc = "FIFO reset."]
    FR_1,
}
impl FRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRW::FR_0 => false,
            FRW::FR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn fr_0(self) -> &'a mut W {
        self.variant(FRW::FR_0)
    }
    #[doc = "FIFO reset."]
    #[inline]
    pub fn fr_1(self) -> &'a mut W {
        self.variant(FRW::FR_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCE`"]
pub enum BCEW {
    #[doc = "Receive bit clock is disabled."]
    BCE_0,
    #[doc = "Receive bit clock is enabled."]
    BCE_1,
}
impl BCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCEW::BCE_0 => false,
            BCEW::BCE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCEW<'a> {
    w: &'a mut W,
}
impl<'a> _BCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive bit clock is disabled."]
    #[inline]
    pub fn bce_0(self) -> &'a mut W {
        self.variant(BCEW::BCE_0)
    }
    #[doc = "Receive bit clock is enabled."]
    #[inline]
    pub fn bce_1(self) -> &'a mut W {
        self.variant(BCEW::BCE_1)
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
#[doc = "Values that can be written to the field `DBGE`"]
pub enum DBGEW {
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    DBGE_0,
    #[doc = "Receiver is enabled in Debug mode."]
    DBGE_1,
}
impl DBGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGEW::DBGE_0 => false,
            DBGEW::DBGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    #[inline]
    pub fn dbge_0(self) -> &'a mut W {
        self.variant(DBGEW::DBGE_0)
    }
    #[doc = "Receiver is enabled in Debug mode."]
    #[inline]
    pub fn dbge_1(self) -> &'a mut W {
        self.variant(DBGEW::DBGE_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOPE`"]
pub enum STOPEW {
    #[doc = "Receiver disabled in Stop mode."]
    STOPE_0,
    #[doc = "Receiver enabled in Stop mode."]
    STOPE_1,
}
impl STOPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPEW::STOPE_0 => false,
            STOPEW::STOPE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPEW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver disabled in Stop mode."]
    #[inline]
    pub fn stope_0(self) -> &'a mut W {
        self.variant(STOPEW::STOPE_0)
    }
    #[doc = "Receiver enabled in Stop mode."]
    #[inline]
    pub fn stope_1(self) -> &'a mut W {
        self.variant(STOPEW::STOPE_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RE`"]
pub enum REW {
    #[doc = "Receiver is disabled."]
    RE_0,
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    RE_1,
}
impl REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REW::RE_0 => false,
            REW::RE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver is disabled."]
    #[inline]
    pub fn re_0(self) -> &'a mut W {
        self.variant(REW::RE_0)
    }
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    #[inline]
    pub fn re_1(self) -> &'a mut W {
        self.variant(REW::RE_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline]
    pub fn frde(&self) -> FRDER {
        FRDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline]
    pub fn fwde(&self) -> FWDER {
        FWDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline]
    pub fn frie(&self) -> FRIER {
        FRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline]
    pub fn fwie(&self) -> FWIER {
        FWIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline]
    pub fn seie(&self) -> SEIER {
        SEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline]
    pub fn wsie(&self) -> WSIER {
        WSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - FIFO Request Flag"]
    #[inline]
    pub fn frf(&self) -> FRFR {
        FRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - FIFO Warning Flag"]
    #[inline]
    pub fn fwf(&self) -> FWFR {
        FWFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline]
    pub fn sef(&self) -> SEFR {
        SEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline]
    pub fn wsf(&self) -> WSFR {
        WSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline]
    pub fn sr(&self) -> SRR {
        SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline]
    pub fn bce(&self) -> BCER {
        BCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline]
    pub fn dbge(&self) -> DBGER {
        DBGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline]
    pub fn stope(&self) -> STOPER {
        STOPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline]
    pub fn frde(&mut self) -> _FRDEW {
        _FRDEW { w: self }
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline]
    pub fn fwde(&mut self) -> _FWDEW {
        _FWDEW { w: self }
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline]
    pub fn frie(&mut self) -> _FRIEW {
        _FRIEW { w: self }
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline]
    pub fn fwie(&mut self) -> _FWIEW {
        _FWIEW { w: self }
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline]
    pub fn seie(&mut self) -> _SEIEW {
        _SEIEW { w: self }
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline]
    pub fn wsie(&mut self) -> _WSIEW {
        _WSIEW { w: self }
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline]
    pub fn sef(&mut self) -> _SEFW {
        _SEFW { w: self }
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline]
    pub fn wsf(&mut self) -> _WSFW {
        _WSFW { w: self }
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline]
    pub fn fr(&mut self) -> _FRW {
        _FRW { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline]
    pub fn bce(&mut self) -> _BCEW {
        _BCEW { w: self }
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline]
    pub fn dbge(&mut self) -> _DBGEW {
        _DBGEW { w: self }
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline]
    pub fn stope(&mut self) -> _STOPEW {
        _STOPEW { w: self }
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
}
