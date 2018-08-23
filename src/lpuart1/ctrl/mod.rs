#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `PT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTR {
    #[doc = "Even parity."]
    PT_0,
    #[doc = "Odd parity."]
    PT_1,
}
impl PTR {
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
            PTR::PT_0 => false,
            PTR::PT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTR {
        match value {
            false => PTR::PT_0,
            true => PTR::PT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PT_0`"]
    #[inline]
    pub fn is_pt_0(&self) -> bool {
        *self == PTR::PT_0
    }
    #[doc = "Checks if the value of the field is `PT_1`"]
    #[inline]
    pub fn is_pt_1(&self) -> bool {
        *self == PTR::PT_1
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "No hardware parity generation or checking."]
    PE_0,
    #[doc = "Parity enabled."]
    PE_1,
}
impl PER {
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
            PER::PE_0 => false,
            PER::PE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::PE_0,
            true => PER::PE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PE_0`"]
    #[inline]
    pub fn is_pe_0(&self) -> bool {
        *self == PER::PE_0
    }
    #[doc = "Checks if the value of the field is `PE_1`"]
    #[inline]
    pub fn is_pe_1(&self) -> bool {
        *self == PER::PE_1
    }
}
#[doc = "Possible values of the field `ILT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILTR {
    #[doc = "Idle character bit count starts after start bit."]
    ILT_0,
    #[doc = "Idle character bit count starts after stop bit."]
    ILT_1,
}
impl ILTR {
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
            ILTR::ILT_0 => false,
            ILTR::ILT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILTR {
        match value {
            false => ILTR::ILT_0,
            true => ILTR::ILT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ILT_0`"]
    #[inline]
    pub fn is_ilt_0(&self) -> bool {
        *self == ILTR::ILT_0
    }
    #[doc = "Checks if the value of the field is `ILT_1`"]
    #[inline]
    pub fn is_ilt_1(&self) -> bool {
        *self == ILTR::ILT_1
    }
}
#[doc = "Possible values of the field `WAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKER {
    #[doc = "Configures RWU for idle-line wakeup."]
    WAKE_0,
    #[doc = "Configures RWU with address-mark wakeup."]
    WAKE_1,
}
impl WAKER {
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
            WAKER::WAKE_0 => false,
            WAKER::WAKE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKER {
        match value {
            false => WAKER::WAKE_0,
            true => WAKER::WAKE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_0`"]
    #[inline]
    pub fn is_wake_0(&self) -> bool {
        *self == WAKER::WAKE_0
    }
    #[doc = "Checks if the value of the field is `WAKE_1`"]
    #[inline]
    pub fn is_wake_1(&self) -> bool {
        *self == WAKER::WAKE_1
    }
}
#[doc = "Possible values of the field `M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR {
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    M_0,
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    M_1,
}
impl MR {
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
            MR::M_0 => false,
            MR::M_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR {
        match value {
            false => MR::M_0,
            true => MR::M_1,
        }
    }
    #[doc = "Checks if the value of the field is `M_0`"]
    #[inline]
    pub fn is_m_0(&self) -> bool {
        *self == MR::M_0
    }
    #[doc = "Checks if the value of the field is `M_1`"]
    #[inline]
    pub fn is_m_1(&self) -> bool {
        *self == MR::M_1
    }
}
#[doc = "Possible values of the field `RSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSRCR {
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    RSRC_0,
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    RSRC_1,
}
impl RSRCR {
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
            RSRCR::RSRC_0 => false,
            RSRCR::RSRC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSRCR {
        match value {
            false => RSRCR::RSRC_0,
            true => RSRCR::RSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSRC_0`"]
    #[inline]
    pub fn is_rsrc_0(&self) -> bool {
        *self == RSRCR::RSRC_0
    }
    #[doc = "Checks if the value of the field is `RSRC_1`"]
    #[inline]
    pub fn is_rsrc_1(&self) -> bool {
        *self == RSRCR::RSRC_1
    }
}
#[doc = "Possible values of the field `DOZEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEENR {
    #[doc = "LPUART is enabled in Doze mode."]
    DOZEEN_0,
    #[doc = "LPUART is disabled in Doze mode."]
    DOZEEN_1,
}
impl DOZEENR {
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
            DOZEENR::DOZEEN_0 => false,
            DOZEENR::DOZEEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZEENR {
        match value {
            false => DOZEENR::DOZEEN_0,
            true => DOZEENR::DOZEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEEN_0`"]
    #[inline]
    pub fn is_dozeen_0(&self) -> bool {
        *self == DOZEENR::DOZEEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEEN_1`"]
    #[inline]
    pub fn is_dozeen_1(&self) -> bool {
        *self == DOZEENR::DOZEEN_1
    }
}
#[doc = "Possible values of the field `LOOPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSR {
    #[doc = "Normal operation - RXD and TXD use separate pins."]
    LOOPS_0,
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    LOOPS_1,
}
impl LOOPSR {
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
            LOOPSR::LOOPS_0 => false,
            LOOPSR::LOOPS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPSR {
        match value {
            false => LOOPSR::LOOPS_0,
            true => LOOPSR::LOOPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOOPS_0`"]
    #[inline]
    pub fn is_loops_0(&self) -> bool {
        *self == LOOPSR::LOOPS_0
    }
    #[doc = "Checks if the value of the field is `LOOPS_1`"]
    #[inline]
    pub fn is_loops_1(&self) -> bool {
        *self == LOOPSR::LOOPS_1
    }
}
#[doc = "Possible values of the field `IDLECFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECFGR {
    #[doc = "1 idle character"]
    IDLECFG_0,
    #[doc = "2 idle characters"]
    IDLECFG_1,
    #[doc = "4 idle characters"]
    IDLECFG_2,
    #[doc = "8 idle characters"]
    IDLECFG_3,
    #[doc = "16 idle characters"]
    IDLECFG_4,
    #[doc = "32 idle characters"]
    IDLECFG_5,
    #[doc = "64 idle characters"]
    IDLECFG_6,
    #[doc = "128 idle characters"]
    IDLECFG_7,
}
impl IDLECFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECFGR::IDLECFG_0 => 0,
            IDLECFGR::IDLECFG_1 => 1,
            IDLECFGR::IDLECFG_2 => 2,
            IDLECFGR::IDLECFG_3 => 3,
            IDLECFGR::IDLECFG_4 => 4,
            IDLECFGR::IDLECFG_5 => 5,
            IDLECFGR::IDLECFG_6 => 6,
            IDLECFGR::IDLECFG_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECFGR {
        match value {
            0 => IDLECFGR::IDLECFG_0,
            1 => IDLECFGR::IDLECFG_1,
            2 => IDLECFGR::IDLECFG_2,
            3 => IDLECFGR::IDLECFG_3,
            4 => IDLECFGR::IDLECFG_4,
            5 => IDLECFGR::IDLECFG_5,
            6 => IDLECFGR::IDLECFG_6,
            7 => IDLECFGR::IDLECFG_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLECFG_0`"]
    #[inline]
    pub fn is_idlecfg_0(&self) -> bool {
        *self == IDLECFGR::IDLECFG_0
    }
    #[doc = "Checks if the value of the field is `IDLECFG_1`"]
    #[inline]
    pub fn is_idlecfg_1(&self) -> bool {
        *self == IDLECFGR::IDLECFG_1
    }
    #[doc = "Checks if the value of the field is `IDLECFG_2`"]
    #[inline]
    pub fn is_idlecfg_2(&self) -> bool {
        *self == IDLECFGR::IDLECFG_2
    }
    #[doc = "Checks if the value of the field is `IDLECFG_3`"]
    #[inline]
    pub fn is_idlecfg_3(&self) -> bool {
        *self == IDLECFGR::IDLECFG_3
    }
    #[doc = "Checks if the value of the field is `IDLECFG_4`"]
    #[inline]
    pub fn is_idlecfg_4(&self) -> bool {
        *self == IDLECFGR::IDLECFG_4
    }
    #[doc = "Checks if the value of the field is `IDLECFG_5`"]
    #[inline]
    pub fn is_idlecfg_5(&self) -> bool {
        *self == IDLECFGR::IDLECFG_5
    }
    #[doc = "Checks if the value of the field is `IDLECFG_6`"]
    #[inline]
    pub fn is_idlecfg_6(&self) -> bool {
        *self == IDLECFGR::IDLECFG_6
    }
    #[doc = "Checks if the value of the field is `IDLECFG_7`"]
    #[inline]
    pub fn is_idlecfg_7(&self) -> bool {
        *self == IDLECFGR::IDLECFG_7
    }
}
#[doc = "Possible values of the field `M7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7R {
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
    M7_0,
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    M7_1,
}
impl M7R {
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
            M7R::M7_0 => false,
            M7R::M7_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M7R {
        match value {
            false => M7R::M7_0,
            true => M7R::M7_1,
        }
    }
    #[doc = "Checks if the value of the field is `M7_0`"]
    #[inline]
    pub fn is_m7_0(&self) -> bool {
        *self == M7R::M7_0
    }
    #[doc = "Checks if the value of the field is `M7_1`"]
    #[inline]
    pub fn is_m7_1(&self) -> bool {
        *self == M7R::M7_1
    }
}
#[doc = "Possible values of the field `MA2IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA2IER {
    #[doc = "MA2F interrupt disabled"]
    MA2IE_0,
    #[doc = "MA2F interrupt enabled"]
    MA2IE_1,
}
impl MA2IER {
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
            MA2IER::MA2IE_0 => false,
            MA2IER::MA2IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA2IER {
        match value {
            false => MA2IER::MA2IE_0,
            true => MA2IER::MA2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MA2IE_0`"]
    #[inline]
    pub fn is_ma2ie_0(&self) -> bool {
        *self == MA2IER::MA2IE_0
    }
    #[doc = "Checks if the value of the field is `MA2IE_1`"]
    #[inline]
    pub fn is_ma2ie_1(&self) -> bool {
        *self == MA2IER::MA2IE_1
    }
}
#[doc = "Possible values of the field `MA1IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA1IER {
    #[doc = "MA1F interrupt disabled"]
    MA1IE_0,
    #[doc = "MA1F interrupt enabled"]
    MA1IE_1,
}
impl MA1IER {
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
            MA1IER::MA1IE_0 => false,
            MA1IER::MA1IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA1IER {
        match value {
            false => MA1IER::MA1IE_0,
            true => MA1IER::MA1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MA1IE_0`"]
    #[inline]
    pub fn is_ma1ie_0(&self) -> bool {
        *self == MA1IER::MA1IE_0
    }
    #[doc = "Checks if the value of the field is `MA1IE_1`"]
    #[inline]
    pub fn is_ma1ie_1(&self) -> bool {
        *self == MA1IER::MA1IE_1
    }
}
#[doc = "Possible values of the field `SBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKR {
    #[doc = "Normal transmitter operation."]
    SBK_0,
    #[doc = "Queue break character(s) to be sent."]
    SBK_1,
}
impl SBKR {
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
            SBKR::SBK_0 => false,
            SBKR::SBK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBKR {
        match value {
            false => SBKR::SBK_0,
            true => SBKR::SBK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBK_0`"]
    #[inline]
    pub fn is_sbk_0(&self) -> bool {
        *self == SBKR::SBK_0
    }
    #[doc = "Checks if the value of the field is `SBK_1`"]
    #[inline]
    pub fn is_sbk_1(&self) -> bool {
        *self == SBKR::SBK_1
    }
}
#[doc = "Possible values of the field `RWU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUR {
    #[doc = "Normal receiver operation."]
    RWU_0,
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    RWU_1,
}
impl RWUR {
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
            RWUR::RWU_0 => false,
            RWUR::RWU_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWUR {
        match value {
            false => RWUR::RWU_0,
            true => RWUR::RWU_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWU_0`"]
    #[inline]
    pub fn is_rwu_0(&self) -> bool {
        *self == RWUR::RWU_0
    }
    #[doc = "Checks if the value of the field is `RWU_1`"]
    #[inline]
    pub fn is_rwu_1(&self) -> bool {
        *self == RWUR::RWU_1
    }
}
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "Receiver disabled."]
    RE_0,
    #[doc = "Receiver enabled."]
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
#[doc = "Possible values of the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TER {
    #[doc = "Transmitter disabled."]
    TE_0,
    #[doc = "Transmitter enabled."]
    TE_1,
}
impl TER {
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
            TER::TE_0 => false,
            TER::TE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TER {
        match value {
            false => TER::TE_0,
            true => TER::TE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TE_0`"]
    #[inline]
    pub fn is_te_0(&self) -> bool {
        *self == TER::TE_0
    }
    #[doc = "Checks if the value of the field is `TE_1`"]
    #[inline]
    pub fn is_te_1(&self) -> bool {
        *self == TER::TE_1
    }
}
#[doc = "Possible values of the field `ILIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIER {
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    ILIE_0,
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    ILIE_1,
}
impl ILIER {
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
            ILIER::ILIE_0 => false,
            ILIER::ILIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIER {
        match value {
            false => ILIER::ILIE_0,
            true => ILIER::ILIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ILIE_0`"]
    #[inline]
    pub fn is_ilie_0(&self) -> bool {
        *self == ILIER::ILIE_0
    }
    #[doc = "Checks if the value of the field is `ILIE_1`"]
    #[inline]
    pub fn is_ilie_1(&self) -> bool {
        *self == ILIER::ILIE_1
    }
}
#[doc = "Possible values of the field `RIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIER {
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    RIE_0,
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    RIE_1,
}
impl RIER {
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
            RIER::RIE_0 => false,
            RIER::RIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIER {
        match value {
            false => RIER::RIE_0,
            true => RIER::RIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIE_0`"]
    #[inline]
    pub fn is_rie_0(&self) -> bool {
        *self == RIER::RIE_0
    }
    #[doc = "Checks if the value of the field is `RIE_1`"]
    #[inline]
    pub fn is_rie_1(&self) -> bool {
        *self == RIER::RIE_1
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    TCIE_0,
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    TCIE_1,
}
impl TCIER {
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
            TCIER::TCIE_0 => false,
            TCIER::TCIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::TCIE_0,
            true => TCIER::TCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCIE_0`"]
    #[inline]
    pub fn is_tcie_0(&self) -> bool {
        *self == TCIER::TCIE_0
    }
    #[doc = "Checks if the value of the field is `TCIE_1`"]
    #[inline]
    pub fn is_tcie_1(&self) -> bool {
        *self == TCIER::TCIE_1
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    TIE_0,
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    TIE_1,
}
impl TIER {
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
            TIER::TIE_0 => false,
            TIER::TIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::TIE_0,
            true => TIER::TIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE_0`"]
    #[inline]
    pub fn is_tie_0(&self) -> bool {
        *self == TIER::TIE_0
    }
    #[doc = "Checks if the value of the field is `TIE_1`"]
    #[inline]
    pub fn is_tie_1(&self) -> bool {
        *self == TIER::TIE_1
    }
}
#[doc = "Possible values of the field `PEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIER {
    #[doc = "PF interrupts disabled; use polling)."]
    PEIE_0,
    #[doc = "Hardware interrupt requested when PF is set."]
    PEIE_1,
}
impl PEIER {
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
            PEIER::PEIE_0 => false,
            PEIER::PEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEIER {
        match value {
            false => PEIER::PEIE_0,
            true => PEIER::PEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PEIE_0`"]
    #[inline]
    pub fn is_peie_0(&self) -> bool {
        *self == PEIER::PEIE_0
    }
    #[doc = "Checks if the value of the field is `PEIE_1`"]
    #[inline]
    pub fn is_peie_1(&self) -> bool {
        *self == PEIER::PEIE_1
    }
}
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "FE interrupts disabled; use polling."]
    FEIE_0,
    #[doc = "Hardware interrupt requested when FE is set."]
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
#[doc = "Possible values of the field `NEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEIER {
    #[doc = "NF interrupts disabled; use polling."]
    NEIE_0,
    #[doc = "Hardware interrupt requested when NF is set."]
    NEIE_1,
}
impl NEIER {
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
            NEIER::NEIE_0 => false,
            NEIER::NEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEIER {
        match value {
            false => NEIER::NEIE_0,
            true => NEIER::NEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `NEIE_0`"]
    #[inline]
    pub fn is_neie_0(&self) -> bool {
        *self == NEIER::NEIE_0
    }
    #[doc = "Checks if the value of the field is `NEIE_1`"]
    #[inline]
    pub fn is_neie_1(&self) -> bool {
        *self == NEIER::NEIE_1
    }
}
#[doc = "Possible values of the field `ORIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORIER {
    #[doc = "OR interrupts disabled; use polling."]
    ORIE_0,
    #[doc = "Hardware interrupt requested when OR is set."]
    ORIE_1,
}
impl ORIER {
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
            ORIER::ORIE_0 => false,
            ORIER::ORIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORIER {
        match value {
            false => ORIER::ORIE_0,
            true => ORIER::ORIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ORIE_0`"]
    #[inline]
    pub fn is_orie_0(&self) -> bool {
        *self == ORIER::ORIE_0
    }
    #[doc = "Checks if the value of the field is `ORIE_1`"]
    #[inline]
    pub fn is_orie_1(&self) -> bool {
        *self == ORIER::ORIE_1
    }
}
#[doc = "Possible values of the field `TXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINVR {
    #[doc = "Transmit data not inverted."]
    TXINV_0,
    #[doc = "Transmit data inverted."]
    TXINV_1,
}
impl TXINVR {
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
            TXINVR::TXINV_0 => false,
            TXINVR::TXINV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXINVR {
        match value {
            false => TXINVR::TXINV_0,
            true => TXINVR::TXINV_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXINV_0`"]
    #[inline]
    pub fn is_txinv_0(&self) -> bool {
        *self == TXINVR::TXINV_0
    }
    #[doc = "Checks if the value of the field is `TXINV_1`"]
    #[inline]
    pub fn is_txinv_1(&self) -> bool {
        *self == TXINVR::TXINV_1
    }
}
#[doc = "Possible values of the field `TXDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIRR {
    #[doc = "TXD pin is an input in single-wire mode."]
    TXDIR_0,
    #[doc = "TXD pin is an output in single-wire mode."]
    TXDIR_1,
}
impl TXDIRR {
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
            TXDIRR::TXDIR_0 => false,
            TXDIRR::TXDIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDIRR {
        match value {
            false => TXDIRR::TXDIR_0,
            true => TXDIRR::TXDIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDIR_0`"]
    #[inline]
    pub fn is_txdir_0(&self) -> bool {
        *self == TXDIRR::TXDIR_0
    }
    #[doc = "Checks if the value of the field is `TXDIR_1`"]
    #[inline]
    pub fn is_txdir_1(&self) -> bool {
        *self == TXDIRR::TXDIR_1
    }
}
#[doc = r" Value of the field"]
pub struct R9T8R {
    bits: bool,
}
impl R9T8R {
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
pub struct R8T9R {
    bits: bool,
}
impl R8T9R {
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
#[doc = "Values that can be written to the field `PT`"]
pub enum PTW {
    #[doc = "Even parity."]
    PT_0,
    #[doc = "Odd parity."]
    PT_1,
}
impl PTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTW::PT_0 => false,
            PTW::PT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTW<'a> {
    w: &'a mut W,
}
impl<'a> _PTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity."]
    #[inline]
    pub fn pt_0(self) -> &'a mut W {
        self.variant(PTW::PT_0)
    }
    #[doc = "Odd parity."]
    #[inline]
    pub fn pt_1(self) -> &'a mut W {
        self.variant(PTW::PT_1)
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
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "No hardware parity generation or checking."]
    PE_0,
    #[doc = "Parity enabled."]
    PE_1,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::PE_0 => false,
            PEW::PE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No hardware parity generation or checking."]
    #[inline]
    pub fn pe_0(self) -> &'a mut W {
        self.variant(PEW::PE_0)
    }
    #[doc = "Parity enabled."]
    #[inline]
    pub fn pe_1(self) -> &'a mut W {
        self.variant(PEW::PE_1)
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
#[doc = "Values that can be written to the field `ILT`"]
pub enum ILTW {
    #[doc = "Idle character bit count starts after start bit."]
    ILT_0,
    #[doc = "Idle character bit count starts after stop bit."]
    ILT_1,
}
impl ILTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ILTW::ILT_0 => false,
            ILTW::ILT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ILTW<'a> {
    w: &'a mut W,
}
impl<'a> _ILTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ILTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle character bit count starts after start bit."]
    #[inline]
    pub fn ilt_0(self) -> &'a mut W {
        self.variant(ILTW::ILT_0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline]
    pub fn ilt_1(self) -> &'a mut W {
        self.variant(ILTW::ILT_1)
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
#[doc = "Values that can be written to the field `WAKE`"]
pub enum WAKEW {
    #[doc = "Configures RWU for idle-line wakeup."]
    WAKE_0,
    #[doc = "Configures RWU with address-mark wakeup."]
    WAKE_1,
}
impl WAKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEW::WAKE_0 => false,
            WAKEW::WAKE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configures RWU for idle-line wakeup."]
    #[inline]
    pub fn wake_0(self) -> &'a mut W {
        self.variant(WAKEW::WAKE_0)
    }
    #[doc = "Configures RWU with address-mark wakeup."]
    #[inline]
    pub fn wake_1(self) -> &'a mut W {
        self.variant(WAKEW::WAKE_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M`"]
pub enum MW {
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    M_0,
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    M_1,
}
impl MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MW::M_0 => false,
            MW::M_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MW<'a> {
    w: &'a mut W,
}
impl<'a> _MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    #[inline]
    pub fn m_0(self) -> &'a mut W {
        self.variant(MW::M_0)
    }
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    #[inline]
    pub fn m_1(self) -> &'a mut W {
        self.variant(MW::M_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSRC`"]
pub enum RSRCW {
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    RSRC_0,
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    RSRC_1,
}
impl RSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSRCW::RSRC_0 => false,
            RSRCW::RSRC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    #[inline]
    pub fn rsrc_0(self) -> &'a mut W {
        self.variant(RSRCW::RSRC_0)
    }
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    #[inline]
    pub fn rsrc_1(self) -> &'a mut W {
        self.variant(RSRCW::RSRC_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOZEEN`"]
pub enum DOZEENW {
    #[doc = "LPUART is enabled in Doze mode."]
    DOZEEN_0,
    #[doc = "LPUART is disabled in Doze mode."]
    DOZEEN_1,
}
impl DOZEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZEENW::DOZEEN_0 => false,
            DOZEENW::DOZEEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZEENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPUART is enabled in Doze mode."]
    #[inline]
    pub fn dozeen_0(self) -> &'a mut W {
        self.variant(DOZEENW::DOZEEN_0)
    }
    #[doc = "LPUART is disabled in Doze mode."]
    #[inline]
    pub fn dozeen_1(self) -> &'a mut W {
        self.variant(DOZEENW::DOZEEN_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOOPS`"]
pub enum LOOPSW {
    #[doc = "Normal operation - RXD and TXD use separate pins."]
    LOOPS_0,
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    LOOPS_1,
}
impl LOOPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPSW::LOOPS_0 => false,
            LOOPSW::LOOPS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPSW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation - RXD and TXD use separate pins."]
    #[inline]
    pub fn loops_0(self) -> &'a mut W {
        self.variant(LOOPSW::LOOPS_0)
    }
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    #[inline]
    pub fn loops_1(self) -> &'a mut W {
        self.variant(LOOPSW::LOOPS_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLECFG`"]
pub enum IDLECFGW {
    #[doc = "1 idle character"]
    IDLECFG_0,
    #[doc = "2 idle characters"]
    IDLECFG_1,
    #[doc = "4 idle characters"]
    IDLECFG_2,
    #[doc = "8 idle characters"]
    IDLECFG_3,
    #[doc = "16 idle characters"]
    IDLECFG_4,
    #[doc = "32 idle characters"]
    IDLECFG_5,
    #[doc = "64 idle characters"]
    IDLECFG_6,
    #[doc = "128 idle characters"]
    IDLECFG_7,
}
impl IDLECFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECFGW::IDLECFG_0 => 0,
            IDLECFGW::IDLECFG_1 => 1,
            IDLECFGW::IDLECFG_2 => 2,
            IDLECFGW::IDLECFG_3 => 3,
            IDLECFGW::IDLECFG_4 => 4,
            IDLECFGW::IDLECFG_5 => 5,
            IDLECFGW::IDLECFG_6 => 6,
            IDLECFGW::IDLECFG_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECFGW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 idle character"]
    #[inline]
    pub fn idlecfg_0(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_0)
    }
    #[doc = "2 idle characters"]
    #[inline]
    pub fn idlecfg_1(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_1)
    }
    #[doc = "4 idle characters"]
    #[inline]
    pub fn idlecfg_2(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_2)
    }
    #[doc = "8 idle characters"]
    #[inline]
    pub fn idlecfg_3(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_3)
    }
    #[doc = "16 idle characters"]
    #[inline]
    pub fn idlecfg_4(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_4)
    }
    #[doc = "32 idle characters"]
    #[inline]
    pub fn idlecfg_5(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_5)
    }
    #[doc = "64 idle characters"]
    #[inline]
    pub fn idlecfg_6(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_6)
    }
    #[doc = "128 idle characters"]
    #[inline]
    pub fn idlecfg_7(self) -> &'a mut W {
        self.variant(IDLECFGW::IDLECFG_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M7`"]
pub enum M7W {
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
    M7_0,
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    M7_1,
}
impl M7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M7W::M7_0 => false,
            M7W::M7_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7W<'a> {
    w: &'a mut W,
}
impl<'a> _M7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
    #[inline]
    pub fn m7_0(self) -> &'a mut W {
        self.variant(M7W::M7_0)
    }
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    #[inline]
    pub fn m7_1(self) -> &'a mut W {
        self.variant(M7W::M7_1)
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
#[doc = "Values that can be written to the field `MA2IE`"]
pub enum MA2IEW {
    #[doc = "MA2F interrupt disabled"]
    MA2IE_0,
    #[doc = "MA2F interrupt enabled"]
    MA2IE_1,
}
impl MA2IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA2IEW::MA2IE_0 => false,
            MA2IEW::MA2IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA2IEW<'a> {
    w: &'a mut W,
}
impl<'a> _MA2IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA2IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MA2F interrupt disabled"]
    #[inline]
    pub fn ma2ie_0(self) -> &'a mut W {
        self.variant(MA2IEW::MA2IE_0)
    }
    #[doc = "MA2F interrupt enabled"]
    #[inline]
    pub fn ma2ie_1(self) -> &'a mut W {
        self.variant(MA2IEW::MA2IE_1)
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
#[doc = "Values that can be written to the field `MA1IE`"]
pub enum MA1IEW {
    #[doc = "MA1F interrupt disabled"]
    MA1IE_0,
    #[doc = "MA1F interrupt enabled"]
    MA1IE_1,
}
impl MA1IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA1IEW::MA1IE_0 => false,
            MA1IEW::MA1IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _MA1IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA1IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MA1F interrupt disabled"]
    #[inline]
    pub fn ma1ie_0(self) -> &'a mut W {
        self.variant(MA1IEW::MA1IE_0)
    }
    #[doc = "MA1F interrupt enabled"]
    #[inline]
    pub fn ma1ie_1(self) -> &'a mut W {
        self.variant(MA1IEW::MA1IE_1)
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
#[doc = "Values that can be written to the field `SBK`"]
pub enum SBKW {
    #[doc = "Normal transmitter operation."]
    SBK_0,
    #[doc = "Queue break character(s) to be sent."]
    SBK_1,
}
impl SBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBKW::SBK_0 => false,
            SBKW::SBK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBKW<'a> {
    w: &'a mut W,
}
impl<'a> _SBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal transmitter operation."]
    #[inline]
    pub fn sbk_0(self) -> &'a mut W {
        self.variant(SBKW::SBK_0)
    }
    #[doc = "Queue break character(s) to be sent."]
    #[inline]
    pub fn sbk_1(self) -> &'a mut W {
        self.variant(SBKW::SBK_1)
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
#[doc = "Values that can be written to the field `RWU`"]
pub enum RWUW {
    #[doc = "Normal receiver operation."]
    RWU_0,
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    RWU_1,
}
impl RWUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWUW::RWU_0 => false,
            RWUW::RWU_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWUW<'a> {
    w: &'a mut W,
}
impl<'a> _RWUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal receiver operation."]
    #[inline]
    pub fn rwu_0(self) -> &'a mut W {
        self.variant(RWUW::RWU_0)
    }
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    #[inline]
    pub fn rwu_1(self) -> &'a mut W {
        self.variant(RWUW::RWU_1)
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
#[doc = "Values that can be written to the field `RE`"]
pub enum REW {
    #[doc = "Receiver disabled."]
    RE_0,
    #[doc = "Receiver enabled."]
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
    #[doc = "Receiver disabled."]
    #[inline]
    pub fn re_0(self) -> &'a mut W {
        self.variant(REW::RE_0)
    }
    #[doc = "Receiver enabled."]
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TE`"]
pub enum TEW {
    #[doc = "Transmitter disabled."]
    TE_0,
    #[doc = "Transmitter enabled."]
    TE_1,
}
impl TEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEW::TE_0 => false,
            TEW::TE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitter disabled."]
    #[inline]
    pub fn te_0(self) -> &'a mut W {
        self.variant(TEW::TE_0)
    }
    #[doc = "Transmitter enabled."]
    #[inline]
    pub fn te_1(self) -> &'a mut W {
        self.variant(TEW::TE_1)
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
#[doc = "Values that can be written to the field `ILIE`"]
pub enum ILIEW {
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    ILIE_0,
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    ILIE_1,
}
impl ILIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ILIEW::ILIE_0 => false,
            ILIEW::ILIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ILIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ILIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ILIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    #[inline]
    pub fn ilie_0(self) -> &'a mut W {
        self.variant(ILIEW::ILIE_0)
    }
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    #[inline]
    pub fn ilie_1(self) -> &'a mut W {
        self.variant(ILIEW::ILIE_1)
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
#[doc = "Values that can be written to the field `RIE`"]
pub enum RIEW {
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    RIE_0,
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    RIE_1,
}
impl RIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIEW::RIE_0 => false,
            RIEW::RIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    #[inline]
    pub fn rie_0(self) -> &'a mut W {
        self.variant(RIEW::RIE_0)
    }
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    #[inline]
    pub fn rie_1(self) -> &'a mut W {
        self.variant(RIEW::RIE_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    TCIE_0,
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    TCIE_1,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::TCIE_0 => false,
            TCIEW::TCIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    #[inline]
    pub fn tcie_0(self) -> &'a mut W {
        self.variant(TCIEW::TCIE_0)
    }
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    #[inline]
    pub fn tcie_1(self) -> &'a mut W {
        self.variant(TCIEW::TCIE_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    TIE_0,
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    TIE_1,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::TIE_0 => false,
            TIEW::TIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    #[inline]
    pub fn tie_0(self) -> &'a mut W {
        self.variant(TIEW::TIE_0)
    }
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    #[inline]
    pub fn tie_1(self) -> &'a mut W {
        self.variant(TIEW::TIE_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEIE`"]
pub enum PEIEW {
    #[doc = "PF interrupts disabled; use polling)."]
    PEIE_0,
    #[doc = "Hardware interrupt requested when PF is set."]
    PEIE_1,
}
impl PEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEIEW::PEIE_0 => false,
            PEIEW::PEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline]
    pub fn peie_0(self) -> &'a mut W {
        self.variant(PEIEW::PEIE_0)
    }
    #[doc = "Hardware interrupt requested when PF is set."]
    #[inline]
    pub fn peie_1(self) -> &'a mut W {
        self.variant(PEIEW::PEIE_1)
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
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "FE interrupts disabled; use polling."]
    FEIE_0,
    #[doc = "Hardware interrupt requested when FE is set."]
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
    #[doc = "FE interrupts disabled; use polling."]
    #[inline]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIEW::FEIE_0)
    }
    #[doc = "Hardware interrupt requested when FE is set."]
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NEIE`"]
pub enum NEIEW {
    #[doc = "NF interrupts disabled; use polling."]
    NEIE_0,
    #[doc = "Hardware interrupt requested when NF is set."]
    NEIE_1,
}
impl NEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NEIEW::NEIE_0 => false,
            NEIEW::NEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NF interrupts disabled; use polling."]
    #[inline]
    pub fn neie_0(self) -> &'a mut W {
        self.variant(NEIEW::NEIE_0)
    }
    #[doc = "Hardware interrupt requested when NF is set."]
    #[inline]
    pub fn neie_1(self) -> &'a mut W {
        self.variant(NEIEW::NEIE_1)
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
#[doc = "Values that can be written to the field `ORIE`"]
pub enum ORIEW {
    #[doc = "OR interrupts disabled; use polling."]
    ORIE_0,
    #[doc = "Hardware interrupt requested when OR is set."]
    ORIE_1,
}
impl ORIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORIEW::ORIE_0 => false,
            ORIEW::ORIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ORIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OR interrupts disabled; use polling."]
    #[inline]
    pub fn orie_0(self) -> &'a mut W {
        self.variant(ORIEW::ORIE_0)
    }
    #[doc = "Hardware interrupt requested when OR is set."]
    #[inline]
    pub fn orie_1(self) -> &'a mut W {
        self.variant(ORIEW::ORIE_1)
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
#[doc = "Values that can be written to the field `TXINV`"]
pub enum TXINVW {
    #[doc = "Transmit data not inverted."]
    TXINV_0,
    #[doc = "Transmit data inverted."]
    TXINV_1,
}
impl TXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXINVW::TXINV_0 => false,
            TXINVW::TXINV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit data not inverted."]
    #[inline]
    pub fn txinv_0(self) -> &'a mut W {
        self.variant(TXINVW::TXINV_0)
    }
    #[doc = "Transmit data inverted."]
    #[inline]
    pub fn txinv_1(self) -> &'a mut W {
        self.variant(TXINVW::TXINV_1)
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
#[doc = "Values that can be written to the field `TXDIR`"]
pub enum TXDIRW {
    #[doc = "TXD pin is an input in single-wire mode."]
    TXDIR_0,
    #[doc = "TXD pin is an output in single-wire mode."]
    TXDIR_1,
}
impl TXDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDIRW::TXDIR_0 => false,
            TXDIRW::TXDIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXD pin is an input in single-wire mode."]
    #[inline]
    pub fn txdir_0(self) -> &'a mut W {
        self.variant(TXDIRW::TXDIR_0)
    }
    #[doc = "TXD pin is an output in single-wire mode."]
    #[inline]
    pub fn txdir_1(self) -> &'a mut W {
        self.variant(TXDIRW::TXDIR_1)
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
#[doc = r" Proxy"]
pub struct _R9T8W<'a> {
    w: &'a mut W,
}
impl<'a> _R9T8W<'a> {
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
#[doc = r" Proxy"]
pub struct _R8T9W<'a> {
    w: &'a mut W,
}
impl<'a> _R8T9W<'a> {
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
    #[doc = "Bit 0 - Parity Type"]
    #[inline]
    pub fn pt(&self) -> PTR {
        PTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline]
    pub fn ilt(&self) -> ILTR {
        ILTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline]
    pub fn wake(&self) -> WAKER {
        WAKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline]
    pub fn m(&self) -> MR {
        MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline]
    pub fn rsrc(&self) -> RSRCR {
        RSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline]
    pub fn dozeen(&self) -> DOZEENR {
        DOZEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline]
    pub fn loops(&self) -> LOOPSR {
        LOOPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline]
    pub fn idlecfg(&self) -> IDLECFGR {
        IDLECFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline]
    pub fn m7(&self) -> M7R {
        M7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline]
    pub fn ma2ie(&self) -> MA2IER {
        MA2IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline]
    pub fn ma1ie(&self) -> MA1IER {
        MA1IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline]
    pub fn sbk(&self) -> SBKR {
        SBKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline]
    pub fn rwu(&self) -> RWUR {
        RWUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline]
    pub fn te(&self) -> TER {
        TER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline]
    pub fn ilie(&self) -> ILIER {
        ILIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline]
    pub fn rie(&self) -> RIER {
        RIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn peie(&self) -> PEIER {
        PEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline]
    pub fn neie(&self) -> NEIER {
        NEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline]
    pub fn orie(&self) -> ORIER {
        ORIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline]
    pub fn txinv(&self) -> TXINVR {
        TXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline]
    pub fn txdir(&self) -> TXDIRR {
        TXDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline]
    pub fn r9t8(&self) -> R9T8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R9T8R { bits }
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline]
    pub fn r8t9(&self) -> R8T9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R8T9R { bits }
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
    #[doc = "Bit 0 - Parity Type"]
    #[inline]
    pub fn pt(&mut self) -> _PTW {
        _PTW { w: self }
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline]
    pub fn ilt(&mut self) -> _ILTW {
        _ILTW { w: self }
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline]
    pub fn wake(&mut self) -> _WAKEW {
        _WAKEW { w: self }
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline]
    pub fn m(&mut self) -> _MW {
        _MW { w: self }
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline]
    pub fn rsrc(&mut self) -> _RSRCW {
        _RSRCW { w: self }
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline]
    pub fn dozeen(&mut self) -> _DOZEENW {
        _DOZEENW { w: self }
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline]
    pub fn loops(&mut self) -> _LOOPSW {
        _LOOPSW { w: self }
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline]
    pub fn idlecfg(&mut self) -> _IDLECFGW {
        _IDLECFGW { w: self }
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline]
    pub fn m7(&mut self) -> _M7W {
        _M7W { w: self }
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline]
    pub fn ma2ie(&mut self) -> _MA2IEW {
        _MA2IEW { w: self }
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline]
    pub fn ma1ie(&mut self) -> _MA1IEW {
        _MA1IEW { w: self }
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline]
    pub fn sbk(&mut self) -> _SBKW {
        _SBKW { w: self }
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline]
    pub fn rwu(&mut self) -> _RWUW {
        _RWUW { w: self }
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline]
    pub fn ilie(&mut self) -> _ILIEW {
        _ILIEW { w: self }
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn peie(&mut self) -> _PEIEW {
        _PEIEW { w: self }
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline]
    pub fn neie(&mut self) -> _NEIEW {
        _NEIEW { w: self }
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline]
    pub fn orie(&mut self) -> _ORIEW {
        _ORIEW { w: self }
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline]
    pub fn txinv(&mut self) -> _TXINVW {
        _TXINVW { w: self }
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline]
    pub fn txdir(&mut self) -> _TXDIRW {
        _TXDIRW { w: self }
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline]
    pub fn r9t8(&mut self) -> _R9T8W {
        _R9T8W { w: self }
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline]
    pub fn r8t9(&mut self) -> _R8T9W {
        _R8T9W { w: self }
    }
}
