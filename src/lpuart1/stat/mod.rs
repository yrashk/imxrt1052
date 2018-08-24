#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `MA2F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA2FR {
    #[doc = "Received data is not equal to MA2"]
    MA2F_0,
    #[doc = "Received data is equal to MA2"]
    MA2F_1,
}
impl MA2FR {
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
            MA2FR::MA2F_0 => false,
            MA2FR::MA2F_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA2FR {
        match value {
            false => MA2FR::MA2F_0,
            true => MA2FR::MA2F_1,
        }
    }
    #[doc = "Checks if the value of the field is `MA2F_0`"]
    #[inline]
    pub fn is_ma2f_0(&self) -> bool {
        *self == MA2FR::MA2F_0
    }
    #[doc = "Checks if the value of the field is `MA2F_1`"]
    #[inline]
    pub fn is_ma2f_1(&self) -> bool {
        *self == MA2FR::MA2F_1
    }
}
#[doc = "Possible values of the field `MA1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA1FR {
    #[doc = "Received data is not equal to MA1"]
    MA1F_0,
    #[doc = "Received data is equal to MA1"]
    MA1F_1,
}
impl MA1FR {
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
            MA1FR::MA1F_0 => false,
            MA1FR::MA1F_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA1FR {
        match value {
            false => MA1FR::MA1F_0,
            true => MA1FR::MA1F_1,
        }
    }
    #[doc = "Checks if the value of the field is `MA1F_0`"]
    #[inline]
    pub fn is_ma1f_0(&self) -> bool {
        *self == MA1FR::MA1F_0
    }
    #[doc = "Checks if the value of the field is `MA1F_1`"]
    #[inline]
    pub fn is_ma1f_1(&self) -> bool {
        *self == MA1FR::MA1F_1
    }
}
#[doc = "Possible values of the field `PF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFR {
    #[doc = "No parity error."]
    PF_0,
    #[doc = "Parity error."]
    PF_1,
}
impl PFR {
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
            PFR::PF_0 => false,
            PFR::PF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFR {
        match value {
            false => PFR::PF_0,
            true => PFR::PF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PF_0`"]
    #[inline]
    pub fn is_pf_0(&self) -> bool {
        *self == PFR::PF_0
    }
    #[doc = "Checks if the value of the field is `PF_1`"]
    #[inline]
    pub fn is_pf_1(&self) -> bool {
        *self == PFR::PF_1
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    FE_0,
    #[doc = "Framing error."]
    FE_1,
}
impl FER {
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
            FER::FE_0 => false,
            FER::FE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER {
        match value {
            false => FER::FE_0,
            true => FER::FE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FE_0`"]
    #[inline]
    pub fn is_fe_0(&self) -> bool {
        *self == FER::FE_0
    }
    #[doc = "Checks if the value of the field is `FE_1`"]
    #[inline]
    pub fn is_fe_1(&self) -> bool {
        *self == FER::FE_1
    }
}
#[doc = "Possible values of the field `NF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFR {
    #[doc = "No noise detected."]
    NF_0,
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    NF_1,
}
impl NFR {
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
            NFR::NF_0 => false,
            NFR::NF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NFR {
        match value {
            false => NFR::NF_0,
            true => NFR::NF_1,
        }
    }
    #[doc = "Checks if the value of the field is `NF_0`"]
    #[inline]
    pub fn is_nf_0(&self) -> bool {
        *self == NFR::NF_0
    }
    #[doc = "Checks if the value of the field is `NF_1`"]
    #[inline]
    pub fn is_nf_1(&self) -> bool {
        *self == NFR::NF_1
    }
}
#[doc = "Possible values of the field `OR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORR {
    #[doc = "No overrun."]
    OR_0,
    #[doc = "Receive overrun (new LPUART data lost)."]
    OR_1,
}
impl ORR {
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
            ORR::OR_0 => false,
            ORR::OR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORR {
        match value {
            false => ORR::OR_0,
            true => ORR::OR_1,
        }
    }
    #[doc = "Checks if the value of the field is `OR_0`"]
    #[inline]
    pub fn is_or_0(&self) -> bool {
        *self == ORR::OR_0
    }
    #[doc = "Checks if the value of the field is `OR_1`"]
    #[inline]
    pub fn is_or_1(&self) -> bool {
        *self == ORR::OR_1
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "No idle line detected."]
    IDLE_0,
    #[doc = "Idle line was detected."]
    IDLE_1,
}
impl IDLER {
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
            IDLER::IDLE_0 => false,
            IDLER::IDLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLER {
        match value {
            false => IDLER::IDLE_0,
            true => IDLER::IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_0`"]
    #[inline]
    pub fn is_idle_0(&self) -> bool {
        *self == IDLER::IDLE_0
    }
    #[doc = "Checks if the value of the field is `IDLE_1`"]
    #[inline]
    pub fn is_idle_1(&self) -> bool {
        *self == IDLER::IDLE_1
    }
}
#[doc = "Possible values of the field `RDRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRFR {
    #[doc = "Receive data buffer empty."]
    RDRF_0,
    #[doc = "Receive data buffer full."]
    RDRF_1,
}
impl RDRFR {
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
            RDRFR::RDRF_0 => false,
            RDRFR::RDRF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDRFR {
        match value {
            false => RDRFR::RDRF_0,
            true => RDRFR::RDRF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDRF_0`"]
    #[inline]
    pub fn is_rdrf_0(&self) -> bool {
        *self == RDRFR::RDRF_0
    }
    #[doc = "Checks if the value of the field is `RDRF_1`"]
    #[inline]
    pub fn is_rdrf_1(&self) -> bool {
        *self == RDRFR::RDRF_1
    }
}
#[doc = "Possible values of the field `TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR {
    #[doc = "Transmitter active (sending data, a preamble, or a break)."]
    TC_0,
    #[doc = "Transmitter idle (transmission activity complete)."]
    TC_1,
}
impl TCR {
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
            TCR::TC_0 => false,
            TCR::TC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCR {
        match value {
            false => TCR::TC_0,
            true => TCR::TC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TC_0`"]
    #[inline]
    pub fn is_tc_0(&self) -> bool {
        *self == TCR::TC_0
    }
    #[doc = "Checks if the value of the field is `TC_1`"]
    #[inline]
    pub fn is_tc_1(&self) -> bool {
        *self == TCR::TC_1
    }
}
#[doc = "Possible values of the field `TDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRER {
    #[doc = "Transmit data buffer full."]
    TDRE_0,
    #[doc = "Transmit data buffer empty."]
    TDRE_1,
}
impl TDRER {
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
            TDRER::TDRE_0 => false,
            TDRER::TDRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRER {
        match value {
            false => TDRER::TDRE_0,
            true => TDRER::TDRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDRE_0`"]
    #[inline]
    pub fn is_tdre_0(&self) -> bool {
        *self == TDRER::TDRE_0
    }
    #[doc = "Checks if the value of the field is `TDRE_1`"]
    #[inline]
    pub fn is_tdre_1(&self) -> bool {
        *self == TDRER::TDRE_1
    }
}
#[doc = "Possible values of the field `RAF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAFR {
    #[doc = "LPUART receiver idle waiting for a start bit."]
    RAF_0,
    #[doc = "LPUART receiver active (RXD input not idle)."]
    RAF_1,
}
impl RAFR {
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
            RAFR::RAF_0 => false,
            RAFR::RAF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAFR {
        match value {
            false => RAFR::RAF_0,
            true => RAFR::RAF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RAF_0`"]
    #[inline]
    pub fn is_raf_0(&self) -> bool {
        *self == RAFR::RAF_0
    }
    #[doc = "Checks if the value of the field is `RAF_1`"]
    #[inline]
    pub fn is_raf_1(&self) -> bool {
        *self == RAFR::RAF_1
    }
}
#[doc = "Possible values of the field `LBKDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDER {
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    LBKDE_0,
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    LBKDE_1,
}
impl LBKDER {
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
            LBKDER::LBKDE_0 => false,
            LBKDER::LBKDE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDER {
        match value {
            false => LBKDER::LBKDE_0,
            true => LBKDER::LBKDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LBKDE_0`"]
    #[inline]
    pub fn is_lbkde_0(&self) -> bool {
        *self == LBKDER::LBKDE_0
    }
    #[doc = "Checks if the value of the field is `LBKDE_1`"]
    #[inline]
    pub fn is_lbkde_1(&self) -> bool {
        *self == LBKDER::LBKDE_1
    }
}
#[doc = "Possible values of the field `BRK13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK13R {
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    BRK13_0,
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    BRK13_1,
}
impl BRK13R {
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
            BRK13R::BRK13_0 => false,
            BRK13R::BRK13_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRK13R {
        match value {
            false => BRK13R::BRK13_0,
            true => BRK13R::BRK13_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRK13_0`"]
    #[inline]
    pub fn is_brk13_0(&self) -> bool {
        *self == BRK13R::BRK13_0
    }
    #[doc = "Checks if the value of the field is `BRK13_1`"]
    #[inline]
    pub fn is_brk13_1(&self) -> bool {
        *self == BRK13R::BRK13_1
    }
}
#[doc = "Possible values of the field `RWUID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUIDR {
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    RWUID_0,
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    RWUID_1,
}
impl RWUIDR {
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
            RWUIDR::RWUID_0 => false,
            RWUIDR::RWUID_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWUIDR {
        match value {
            false => RWUIDR::RWUID_0,
            true => RWUIDR::RWUID_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWUID_0`"]
    #[inline]
    pub fn is_rwuid_0(&self) -> bool {
        *self == RWUIDR::RWUID_0
    }
    #[doc = "Checks if the value of the field is `RWUID_1`"]
    #[inline]
    pub fn is_rwuid_1(&self) -> bool {
        *self == RWUIDR::RWUID_1
    }
}
#[doc = "Possible values of the field `RXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINVR {
    #[doc = "Receive data not inverted."]
    RXINV_0,
    #[doc = "Receive data inverted."]
    RXINV_1,
}
impl RXINVR {
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
            RXINVR::RXINV_0 => false,
            RXINVR::RXINV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXINVR {
        match value {
            false => RXINVR::RXINV_0,
            true => RXINVR::RXINV_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXINV_0`"]
    #[inline]
    pub fn is_rxinv_0(&self) -> bool {
        *self == RXINVR::RXINV_0
    }
    #[doc = "Checks if the value of the field is `RXINV_1`"]
    #[inline]
    pub fn is_rxinv_1(&self) -> bool {
        *self == RXINVR::RXINV_1
    }
}
#[doc = "Possible values of the field `MSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBFR {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    MSBF_0,
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL[M], CTRL[PE] and BAUD[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL[M] and CTRL[PE]."]
    MSBF_1,
}
impl MSBFR {
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
            MSBFR::MSBF_0 => false,
            MSBFR::MSBF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBFR {
        match value {
            false => MSBFR::MSBF_0,
            true => MSBFR::MSBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSBF_0`"]
    #[inline]
    pub fn is_msbf_0(&self) -> bool {
        *self == MSBFR::MSBF_0
    }
    #[doc = "Checks if the value of the field is `MSBF_1`"]
    #[inline]
    pub fn is_msbf_1(&self) -> bool {
        *self == MSBFR::MSBF_1
    }
}
#[doc = "Possible values of the field `RXEDGIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIFR {
    #[doc = "No active edge on the receive pin has occurred."]
    RXEDGIF_0,
    #[doc = "An active edge on the receive pin has occurred."]
    RXEDGIF_1,
}
impl RXEDGIFR {
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
            RXEDGIFR::RXEDGIF_0 => false,
            RXEDGIFR::RXEDGIF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEDGIFR {
        match value {
            false => RXEDGIFR::RXEDGIF_0,
            true => RXEDGIFR::RXEDGIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEDGIF_0`"]
    #[inline]
    pub fn is_rxedgif_0(&self) -> bool {
        *self == RXEDGIFR::RXEDGIF_0
    }
    #[doc = "Checks if the value of the field is `RXEDGIF_1`"]
    #[inline]
    pub fn is_rxedgif_1(&self) -> bool {
        *self == RXEDGIFR::RXEDGIF_1
    }
}
#[doc = "Possible values of the field `LBKDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIFR {
    #[doc = "No LIN break character has been detected."]
    LBKDIF_0,
    #[doc = "LIN break character has been detected."]
    LBKDIF_1,
}
impl LBKDIFR {
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
            LBKDIFR::LBKDIF_0 => false,
            LBKDIFR::LBKDIF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDIFR {
        match value {
            false => LBKDIFR::LBKDIF_0,
            true => LBKDIFR::LBKDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LBKDIF_0`"]
    #[inline]
    pub fn is_lbkdif_0(&self) -> bool {
        *self == LBKDIFR::LBKDIF_0
    }
    #[doc = "Checks if the value of the field is `LBKDIF_1`"]
    #[inline]
    pub fn is_lbkdif_1(&self) -> bool {
        *self == LBKDIFR::LBKDIF_1
    }
}
#[doc = "Values that can be written to the field `MA2F`"]
pub enum MA2FW {
    #[doc = "Received data is not equal to MA2"]
    MA2F_0,
    #[doc = "Received data is equal to MA2"]
    MA2F_1,
}
impl MA2FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA2FW::MA2F_0 => false,
            MA2FW::MA2F_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA2FW<'a> {
    w: &'a mut W,
}
impl<'a> _MA2FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA2FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data is not equal to MA2"]
    #[inline]
    pub fn ma2f_0(self) -> &'a mut W {
        self.variant(MA2FW::MA2F_0)
    }
    #[doc = "Received data is equal to MA2"]
    #[inline]
    pub fn ma2f_1(self) -> &'a mut W {
        self.variant(MA2FW::MA2F_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MA1F`"]
pub enum MA1FW {
    #[doc = "Received data is not equal to MA1"]
    MA1F_0,
    #[doc = "Received data is equal to MA1"]
    MA1F_1,
}
impl MA1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA1FW::MA1F_0 => false,
            MA1FW::MA1F_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA1FW<'a> {
    w: &'a mut W,
}
impl<'a> _MA1FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA1FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data is not equal to MA1"]
    #[inline]
    pub fn ma1f_0(self) -> &'a mut W {
        self.variant(MA1FW::MA1F_0)
    }
    #[doc = "Received data is equal to MA1"]
    #[inline]
    pub fn ma1f_1(self) -> &'a mut W {
        self.variant(MA1FW::MA1F_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PF`"]
pub enum PFW {
    #[doc = "No parity error."]
    PF_0,
    #[doc = "Parity error."]
    PF_1,
}
impl PFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFW::PF_0 => false,
            PFW::PF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFW<'a> {
    w: &'a mut W,
}
impl<'a> _PFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error."]
    #[inline]
    pub fn pf_0(self) -> &'a mut W {
        self.variant(PFW::PF_0)
    }
    #[doc = "Parity error."]
    #[inline]
    pub fn pf_1(self) -> &'a mut W {
        self.variant(PFW::PF_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE`"]
pub enum FEW {
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    FE_0,
    #[doc = "Framing error."]
    FE_1,
}
impl FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEW::FE_0 => false,
            FEW::FE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    #[inline]
    pub fn fe_0(self) -> &'a mut W {
        self.variant(FEW::FE_0)
    }
    #[doc = "Framing error."]
    #[inline]
    pub fn fe_1(self) -> &'a mut W {
        self.variant(FEW::FE_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NF`"]
pub enum NFW {
    #[doc = "No noise detected."]
    NF_0,
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    NF_1,
}
impl NFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NFW::NF_0 => false,
            NFW::NF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NFW<'a> {
    w: &'a mut W,
}
impl<'a> _NFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No noise detected."]
    #[inline]
    pub fn nf_0(self) -> &'a mut W {
        self.variant(NFW::NF_0)
    }
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    #[inline]
    pub fn nf_1(self) -> &'a mut W {
        self.variant(NFW::NF_1)
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
#[doc = "Values that can be written to the field `OR`"]
pub enum ORW {
    #[doc = "No overrun."]
    OR_0,
    #[doc = "Receive overrun (new LPUART data lost)."]
    OR_1,
}
impl ORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORW::OR_0 => false,
            ORW::OR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORW<'a> {
    w: &'a mut W,
}
impl<'a> _ORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun."]
    #[inline]
    pub fn or_0(self) -> &'a mut W {
        self.variant(ORW::OR_0)
    }
    #[doc = "Receive overrun (new LPUART data lost)."]
    #[inline]
    pub fn or_1(self) -> &'a mut W {
        self.variant(ORW::OR_1)
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
#[doc = "Values that can be written to the field `IDLE`"]
pub enum IDLEW {
    #[doc = "No idle line detected."]
    IDLE_0,
    #[doc = "Idle line was detected."]
    IDLE_1,
}
impl IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLEW::IDLE_0 => false,
            IDLEW::IDLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No idle line detected."]
    #[inline]
    pub fn idle_0(self) -> &'a mut W {
        self.variant(IDLEW::IDLE_0)
    }
    #[doc = "Idle line was detected."]
    #[inline]
    pub fn idle_1(self) -> &'a mut W {
        self.variant(IDLEW::IDLE_1)
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
#[doc = "Values that can be written to the field `LBKDE`"]
pub enum LBKDEW {
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    LBKDE_0,
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    LBKDE_1,
}
impl LBKDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDEW::LBKDE_0 => false,
            LBKDEW::LBKDE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    #[inline]
    pub fn lbkde_0(self) -> &'a mut W {
        self.variant(LBKDEW::LBKDE_0)
    }
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    #[inline]
    pub fn lbkde_1(self) -> &'a mut W {
        self.variant(LBKDEW::LBKDE_1)
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
#[doc = "Values that can be written to the field `BRK13`"]
pub enum BRK13W {
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    BRK13_0,
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    BRK13_1,
}
impl BRK13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRK13W::BRK13_0 => false,
            BRK13W::BRK13_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRK13W<'a> {
    w: &'a mut W,
}
impl<'a> _BRK13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRK13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    #[inline]
    pub fn brk13_0(self) -> &'a mut W {
        self.variant(BRK13W::BRK13_0)
    }
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    #[inline]
    pub fn brk13_1(self) -> &'a mut W {
        self.variant(BRK13W::BRK13_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RWUID`"]
pub enum RWUIDW {
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    RWUID_0,
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    RWUID_1,
}
impl RWUIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWUIDW::RWUID_0 => false,
            RWUIDW::RWUID_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWUIDW<'a> {
    w: &'a mut W,
}
impl<'a> _RWUIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWUIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    #[inline]
    pub fn rwuid_0(self) -> &'a mut W {
        self.variant(RWUIDW::RWUID_0)
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    #[inline]
    pub fn rwuid_1(self) -> &'a mut W {
        self.variant(RWUIDW::RWUID_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXINV`"]
pub enum RXINVW {
    #[doc = "Receive data not inverted."]
    RXINV_0,
    #[doc = "Receive data inverted."]
    RXINV_1,
}
impl RXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXINVW::RXINV_0 => false,
            RXINVW::RXINV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive data not inverted."]
    #[inline]
    pub fn rxinv_0(self) -> &'a mut W {
        self.variant(RXINVW::RXINV_0)
    }
    #[doc = "Receive data inverted."]
    #[inline]
    pub fn rxinv_1(self) -> &'a mut W {
        self.variant(RXINVW::RXINV_1)
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
#[doc = "Values that can be written to the field `MSBF`"]
pub enum MSBFW {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    MSBF_0,
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL[M], CTRL[PE] and BAUD[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL[M] and CTRL[PE]."]
    MSBF_1,
}
impl MSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBFW::MSBF_0 => false,
            MSBFW::MSBF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline]
    pub fn msbf_0(self) -> &'a mut W {
        self.variant(MSBFW::MSBF_0)
    }
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL[M], CTRL[PE] and BAUD[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL[M] and CTRL[PE]."]
    #[inline]
    pub fn msbf_1(self) -> &'a mut W {
        self.variant(MSBFW::MSBF_1)
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
#[doc = "Values that can be written to the field `RXEDGIF`"]
pub enum RXEDGIFW {
    #[doc = "No active edge on the receive pin has occurred."]
    RXEDGIF_0,
    #[doc = "An active edge on the receive pin has occurred."]
    RXEDGIF_1,
}
impl RXEDGIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEDGIFW::RXEDGIF_0 => false,
            RXEDGIFW::RXEDGIF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEDGIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEDGIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEDGIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline]
    pub fn rxedgif_0(self) -> &'a mut W {
        self.variant(RXEDGIFW::RXEDGIF_0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline]
    pub fn rxedgif_1(self) -> &'a mut W {
        self.variant(RXEDGIFW::RXEDGIF_1)
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
#[doc = "Values that can be written to the field `LBKDIF`"]
pub enum LBKDIFW {
    #[doc = "No LIN break character has been detected."]
    LBKDIF_0,
    #[doc = "LIN break character has been detected."]
    LBKDIF_1,
}
impl LBKDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDIFW::LBKDIF_0 => false,
            LBKDIFW::LBKDIF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No LIN break character has been detected."]
    #[inline]
    pub fn lbkdif_0(self) -> &'a mut W {
        self.variant(LBKDIFW::LBKDIF_0)
    }
    #[doc = "LIN break character has been detected."]
    #[inline]
    pub fn lbkdif_1(self) -> &'a mut W {
        self.variant(LBKDIFW::LBKDIF_1)
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
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline]
    pub fn ma2f(&self) -> MA2FR {
        MA2FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline]
    pub fn ma1f(&self) -> MA1FR {
        MA1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline]
    pub fn pf(&self) -> PFR {
        PFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline]
    pub fn fe(&self) -> FER {
        FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline]
    pub fn nf(&self) -> NFR {
        NFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline]
    pub fn or(&self) -> ORR {
        ORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Receive Data Register Full Flag"]
    #[inline]
    pub fn rdrf(&self) -> RDRFR {
        RDRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Transmission Complete Flag"]
    #[inline]
    pub fn tc(&self) -> TCR {
        TCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit Data Register Empty Flag"]
    #[inline]
    pub fn tdre(&self) -> TDRER {
        TDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Receiver Active Flag"]
    #[inline]
    pub fn raf(&self) -> RAFR {
        RAFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline]
    pub fn lbkde(&self) -> LBKDER {
        LBKDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline]
    pub fn brk13(&self) -> BRK13R {
        BRK13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline]
    pub fn rwuid(&self) -> RWUIDR {
        RWUIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline]
    pub fn rxinv(&self) -> RXINVR {
        RXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline]
    pub fn msbf(&self) -> MSBFR {
        MSBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline]
    pub fn rxedgif(&self) -> RXEDGIFR {
        RXEDGIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline]
    pub fn lbkdif(&self) -> LBKDIFR {
        LBKDIFR::_from({
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
        W { bits: 12582912 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline]
    pub fn ma2f(&mut self) -> _MA2FW {
        _MA2FW { w: self }
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline]
    pub fn ma1f(&mut self) -> _MA1FW {
        _MA1FW { w: self }
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline]
    pub fn pf(&mut self) -> _PFW {
        _PFW { w: self }
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline]
    pub fn fe(&mut self) -> _FEW {
        _FEW { w: self }
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline]
    pub fn nf(&mut self) -> _NFW {
        _NFW { w: self }
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline]
    pub fn or(&mut self) -> _ORW {
        _ORW { w: self }
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline]
    pub fn idle(&mut self) -> _IDLEW {
        _IDLEW { w: self }
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline]
    pub fn lbkde(&mut self) -> _LBKDEW {
        _LBKDEW { w: self }
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline]
    pub fn brk13(&mut self) -> _BRK13W {
        _BRK13W { w: self }
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline]
    pub fn rwuid(&mut self) -> _RWUIDW {
        _RWUIDW { w: self }
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline]
    pub fn rxinv(&mut self) -> _RXINVW {
        _RXINVW { w: self }
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline]
    pub fn msbf(&mut self) -> _MSBFW {
        _MSBFW { w: self }
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline]
    pub fn rxedgif(&mut self) -> _RXEDGIFW {
        _RXEDGIFW { w: self }
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline]
    pub fn lbkdif(&mut self) -> _LBKDIFW {
        _LBKDIFW { w: self }
    }
}
