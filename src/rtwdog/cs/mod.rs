#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CS {
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
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "Watchdog disabled in chip stop mode."]
    STOP_0,
    #[doc = "Watchdog enabled in chip stop mode."]
    STOP_1,
}
impl STOPR {
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
            STOPR::STOP_0 => false,
            STOPR::STOP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPR {
        match value {
            false => STOPR::STOP_0,
            true => STOPR::STOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_0`"]
    #[inline]
    pub fn is_stop_0(&self) -> bool {
        *self == STOPR::STOP_0
    }
    #[doc = "Checks if the value of the field is `STOP_1`"]
    #[inline]
    pub fn is_stop_1(&self) -> bool {
        *self == STOPR::STOP_1
    }
}
#[doc = "Possible values of the field `WAIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITR {
    #[doc = "Watchdog disabled in chip wait mode."]
    WAIT_0,
    #[doc = "Watchdog enabled in chip wait mode."]
    WAIT_1,
}
impl WAITR {
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
            WAITR::WAIT_0 => false,
            WAITR::WAIT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITR {
        match value {
            false => WAITR::WAIT_0,
            true => WAITR::WAIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT_0`"]
    #[inline]
    pub fn is_wait_0(&self) -> bool {
        *self == WAITR::WAIT_0
    }
    #[doc = "Checks if the value of the field is `WAIT_1`"]
    #[inline]
    pub fn is_wait_1(&self) -> bool {
        *self == WAITR::WAIT_1
    }
}
#[doc = "Possible values of the field `DBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGR {
    #[doc = "Watchdog disabled in chip debug mode."]
    DBG_0,
    #[doc = "Watchdog enabled in chip debug mode."]
    DBG_1,
}
impl DBGR {
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
            DBGR::DBG_0 => false,
            DBGR::DBG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGR {
        match value {
            false => DBGR::DBG_0,
            true => DBGR::DBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_0`"]
    #[inline]
    pub fn is_dbg_0(&self) -> bool {
        *self == DBGR::DBG_0
    }
    #[doc = "Checks if the value of the field is `DBG_1`"]
    #[inline]
    pub fn is_dbg_1(&self) -> bool {
        *self == DBGR::DBG_1
    }
}
#[doc = "Possible values of the field `TST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTR {
    #[doc = "Watchdog test mode disabled."]
    TST_0,
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    TST_1,
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT[CNTLOW] is compared with TOVAL[TOVALLOW]."]
    TST_2,
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT[CNTHIGH] is compared with TOVAL[TOVALHIGH]."]
    TST_3,
}
impl TSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTR::TST_0 => 0,
            TSTR::TST_1 => 1,
            TSTR::TST_2 => 2,
            TSTR::TST_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTR {
        match value {
            0 => TSTR::TST_0,
            1 => TSTR::TST_1,
            2 => TSTR::TST_2,
            3 => TSTR::TST_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TST_0`"]
    #[inline]
    pub fn is_tst_0(&self) -> bool {
        *self == TSTR::TST_0
    }
    #[doc = "Checks if the value of the field is `TST_1`"]
    #[inline]
    pub fn is_tst_1(&self) -> bool {
        *self == TSTR::TST_1
    }
    #[doc = "Checks if the value of the field is `TST_2`"]
    #[inline]
    pub fn is_tst_2(&self) -> bool {
        *self == TSTR::TST_2
    }
    #[doc = "Checks if the value of the field is `TST_3`"]
    #[inline]
    pub fn is_tst_3(&self) -> bool {
        *self == TSTR::TST_3
    }
}
#[doc = "Possible values of the field `UPDATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATER {
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    UPDATE_0,
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    UPDATE_1,
}
impl UPDATER {
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
            UPDATER::UPDATE_0 => false,
            UPDATER::UPDATE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPDATER {
        match value {
            false => UPDATER::UPDATE_0,
            true => UPDATER::UPDATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE_0`"]
    #[inline]
    pub fn is_update_0(&self) -> bool {
        *self == UPDATER::UPDATE_0
    }
    #[doc = "Checks if the value of the field is `UPDATE_1`"]
    #[inline]
    pub fn is_update_1(&self) -> bool {
        *self == UPDATER::UPDATE_1
    }
}
#[doc = "Possible values of the field `INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTR {
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    INT_0,
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch."]
    INT_1,
}
impl INTR {
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
            INTR::INT_0 => false,
            INTR::INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTR {
        match value {
            false => INTR::INT_0,
            true => INTR::INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_0`"]
    #[inline]
    pub fn is_int_0(&self) -> bool {
        *self == INTR::INT_0
    }
    #[doc = "Checks if the value of the field is `INT_1`"]
    #[inline]
    pub fn is_int_1(&self) -> bool {
        *self == INTR::INT_1
    }
}
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Watchdog disabled."]
    EN_0,
    #[doc = "Watchdog enabled."]
    EN_1,
}
impl ENR {
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
            ENR::EN_0 => false,
            ENR::EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::EN_0,
            true => ENR::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline]
    pub fn is_en_0(&self) -> bool {
        *self == ENR::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline]
    pub fn is_en_1(&self) -> bool {
        *self == ENR::EN_1
    }
}
#[doc = "Possible values of the field `CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKR {
    #[doc = "Bus clock"]
    CLK_0,
    #[doc = "LPO clock"]
    CLK_1,
    #[doc = "INTCLK (internal clock)"]
    CLK_2,
    #[doc = "ERCLK (external reference clock)"]
    CLK_3,
}
impl CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKR::CLK_0 => 0,
            CLKR::CLK_1 => 1,
            CLKR::CLK_2 => 2,
            CLKR::CLK_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKR {
        match value {
            0 => CLKR::CLK_0,
            1 => CLKR::CLK_1,
            2 => CLKR::CLK_2,
            3 => CLKR::CLK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_0`"]
    #[inline]
    pub fn is_clk_0(&self) -> bool {
        *self == CLKR::CLK_0
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline]
    pub fn is_clk_1(&self) -> bool {
        *self == CLKR::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline]
    pub fn is_clk_2(&self) -> bool {
        *self == CLKR::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_3`"]
    #[inline]
    pub fn is_clk_3(&self) -> bool {
        *self == CLKR::CLK_3
    }
}
#[doc = "Possible values of the field `RCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCSR {
    #[doc = "Reconfiguring WDOG."]
    RCS_0,
    #[doc = "Reconfiguration is successful."]
    RCS_1,
}
impl RCSR {
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
            RCSR::RCS_0 => false,
            RCSR::RCS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCSR {
        match value {
            false => RCSR::RCS_0,
            true => RCSR::RCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RCS_0`"]
    #[inline]
    pub fn is_rcs_0(&self) -> bool {
        *self == RCSR::RCS_0
    }
    #[doc = "Checks if the value of the field is `RCS_1`"]
    #[inline]
    pub fn is_rcs_1(&self) -> bool {
        *self == RCSR::RCS_1
    }
}
#[doc = "Possible values of the field `ULK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULKR {
    #[doc = "WDOG is locked."]
    ULK_0,
    #[doc = "WDOG is unlocked."]
    ULK_1,
}
impl ULKR {
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
            ULKR::ULK_0 => false,
            ULKR::ULK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULKR {
        match value {
            false => ULKR::ULK_0,
            true => ULKR::ULK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ULK_0`"]
    #[inline]
    pub fn is_ulk_0(&self) -> bool {
        *self == ULKR::ULK_0
    }
    #[doc = "Checks if the value of the field is `ULK_1`"]
    #[inline]
    pub fn is_ulk_1(&self) -> bool {
        *self == ULKR::ULK_1
    }
}
#[doc = "Possible values of the field `PRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESR {
    #[doc = "256 prescaler disabled."]
    PRES_0,
    #[doc = "256 prescaler enabled."]
    PRES_1,
}
impl PRESR {
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
            PRESR::PRES_0 => false,
            PRESR::PRES_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESR {
        match value {
            false => PRESR::PRES_0,
            true => PRESR::PRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRES_0`"]
    #[inline]
    pub fn is_pres_0(&self) -> bool {
        *self == PRESR::PRES_0
    }
    #[doc = "Checks if the value of the field is `PRES_1`"]
    #[inline]
    pub fn is_pres_1(&self) -> bool {
        *self == PRESR::PRES_1
    }
}
#[doc = "Possible values of the field `CMD32EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD32ENR {
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    CMD32EN_0,
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    CMD32EN_1,
}
impl CMD32ENR {
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
            CMD32ENR::CMD32EN_0 => false,
            CMD32ENR::CMD32EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD32ENR {
        match value {
            false => CMD32ENR::CMD32EN_0,
            true => CMD32ENR::CMD32EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMD32EN_0`"]
    #[inline]
    pub fn is_cmd32en_0(&self) -> bool {
        *self == CMD32ENR::CMD32EN_0
    }
    #[doc = "Checks if the value of the field is `CMD32EN_1`"]
    #[inline]
    pub fn is_cmd32en_1(&self) -> bool {
        *self == CMD32ENR::CMD32EN_1
    }
}
#[doc = "Possible values of the field `FLG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLGR {
    #[doc = "No interrupt occurred."]
    FLG_0,
    #[doc = "An interrupt occurred."]
    FLG_1,
}
impl FLGR {
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
            FLGR::FLG_0 => false,
            FLGR::FLG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLGR {
        match value {
            false => FLGR::FLG_0,
            true => FLGR::FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLG_0`"]
    #[inline]
    pub fn is_flg_0(&self) -> bool {
        *self == FLGR::FLG_0
    }
    #[doc = "Checks if the value of the field is `FLG_1`"]
    #[inline]
    pub fn is_flg_1(&self) -> bool {
        *self == FLGR::FLG_1
    }
}
#[doc = "Possible values of the field `WIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINR {
    #[doc = "Window mode disabled."]
    WIN_0,
    #[doc = "Window mode enabled."]
    WIN_1,
}
impl WINR {
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
            WINR::WIN_0 => false,
            WINR::WIN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WINR {
        match value {
            false => WINR::WIN_0,
            true => WINR::WIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIN_0`"]
    #[inline]
    pub fn is_win_0(&self) -> bool {
        *self == WINR::WIN_0
    }
    #[doc = "Checks if the value of the field is `WIN_1`"]
    #[inline]
    pub fn is_win_1(&self) -> bool {
        *self == WINR::WIN_1
    }
}
#[doc = "Values that can be written to the field `STOP`"]
pub enum STOPW {
    #[doc = "Watchdog disabled in chip stop mode."]
    STOP_0,
    #[doc = "Watchdog enabled in chip stop mode."]
    STOP_1,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPW::STOP_0 => false,
            STOPW::STOP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled in chip stop mode."]
    #[inline]
    pub fn stop_0(self) -> &'a mut W {
        self.variant(STOPW::STOP_0)
    }
    #[doc = "Watchdog enabled in chip stop mode."]
    #[inline]
    pub fn stop_1(self) -> &'a mut W {
        self.variant(STOPW::STOP_1)
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
#[doc = "Values that can be written to the field `WAIT`"]
pub enum WAITW {
    #[doc = "Watchdog disabled in chip wait mode."]
    WAIT_0,
    #[doc = "Watchdog enabled in chip wait mode."]
    WAIT_1,
}
impl WAITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAITW::WAIT_0 => false,
            WAITW::WAIT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled in chip wait mode."]
    #[inline]
    pub fn wait_0(self) -> &'a mut W {
        self.variant(WAITW::WAIT_0)
    }
    #[doc = "Watchdog enabled in chip wait mode."]
    #[inline]
    pub fn wait_1(self) -> &'a mut W {
        self.variant(WAITW::WAIT_1)
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
#[doc = "Values that can be written to the field `DBG`"]
pub enum DBGW {
    #[doc = "Watchdog disabled in chip debug mode."]
    DBG_0,
    #[doc = "Watchdog enabled in chip debug mode."]
    DBG_1,
}
impl DBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGW::DBG_0 => false,
            DBGW::DBG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled in chip debug mode."]
    #[inline]
    pub fn dbg_0(self) -> &'a mut W {
        self.variant(DBGW::DBG_0)
    }
    #[doc = "Watchdog enabled in chip debug mode."]
    #[inline]
    pub fn dbg_1(self) -> &'a mut W {
        self.variant(DBGW::DBG_1)
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
#[doc = "Values that can be written to the field `TST`"]
pub enum TSTW {
    #[doc = "Watchdog test mode disabled."]
    TST_0,
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    TST_1,
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT[CNTLOW] is compared with TOVAL[TOVALLOW]."]
    TST_2,
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT[CNTHIGH] is compared with TOVAL[TOVALHIGH]."]
    TST_3,
}
impl TSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTW::TST_0 => 0,
            TSTW::TST_1 => 1,
            TSTW::TST_2 => 2,
            TSTW::TST_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Watchdog test mode disabled."]
    #[inline]
    pub fn tst_0(self) -> &'a mut W {
        self.variant(TSTW::TST_0)
    }
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    #[inline]
    pub fn tst_1(self) -> &'a mut W {
        self.variant(TSTW::TST_1)
    }
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT[CNTLOW] is compared with TOVAL[TOVALLOW]."]
    #[inline]
    pub fn tst_2(self) -> &'a mut W {
        self.variant(TSTW::TST_2)
    }
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT[CNTHIGH] is compared with TOVAL[TOVALHIGH]."]
    #[inline]
    pub fn tst_3(self) -> &'a mut W {
        self.variant(TSTW::TST_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UPDATE`"]
pub enum UPDATEW {
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    UPDATE_0,
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    UPDATE_1,
}
impl UPDATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPDATEW::UPDATE_0 => false,
            UPDATEW::UPDATE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    #[inline]
    pub fn update_0(self) -> &'a mut W {
        self.variant(UPDATEW::UPDATE_0)
    }
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    #[inline]
    pub fn update_1(self) -> &'a mut W {
        self.variant(UPDATEW::UPDATE_1)
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
#[doc = "Values that can be written to the field `INT`"]
pub enum INTW {
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    INT_0,
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch."]
    INT_1,
}
impl INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTW::INT_0 => false,
            INTW::INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    #[inline]
    pub fn int_0(self) -> &'a mut W {
        self.variant(INTW::INT_0)
    }
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch."]
    #[inline]
    pub fn int_1(self) -> &'a mut W {
        self.variant(INTW::INT_1)
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
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Watchdog disabled."]
    EN_0,
    #[doc = "Watchdog enabled."]
    EN_1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::EN_0 => false,
            ENW::EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled."]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "Watchdog enabled."]
    #[inline]
    pub fn en_1(self) -> &'a mut W {
        self.variant(ENW::EN_1)
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
#[doc = "Values that can be written to the field `CLK`"]
pub enum CLKW {
    #[doc = "Bus clock"]
    CLK_0,
    #[doc = "LPO clock"]
    CLK_1,
    #[doc = "INTCLK (internal clock)"]
    CLK_2,
    #[doc = "ERCLK (external reference clock)"]
    CLK_3,
}
impl CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKW::CLK_0 => 0,
            CLKW::CLK_1 => 1,
            CLKW::CLK_2 => 2,
            CLKW::CLK_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bus clock"]
    #[inline]
    pub fn clk_0(self) -> &'a mut W {
        self.variant(CLKW::CLK_0)
    }
    #[doc = "LPO clock"]
    #[inline]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(CLKW::CLK_1)
    }
    #[doc = "INTCLK (internal clock)"]
    #[inline]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(CLKW::CLK_2)
    }
    #[doc = "ERCLK (external reference clock)"]
    #[inline]
    pub fn clk_3(self) -> &'a mut W {
        self.variant(CLKW::CLK_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRES`"]
pub enum PRESW {
    #[doc = "256 prescaler disabled."]
    PRES_0,
    #[doc = "256 prescaler enabled."]
    PRES_1,
}
impl PRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRESW::PRES_0 => false,
            PRESW::PRES_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "256 prescaler disabled."]
    #[inline]
    pub fn pres_0(self) -> &'a mut W {
        self.variant(PRESW::PRES_0)
    }
    #[doc = "256 prescaler enabled."]
    #[inline]
    pub fn pres_1(self) -> &'a mut W {
        self.variant(PRESW::PRES_1)
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
#[doc = "Values that can be written to the field `CMD32EN`"]
pub enum CMD32ENW {
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    CMD32EN_0,
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    CMD32EN_1,
}
impl CMD32ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD32ENW::CMD32EN_0 => false,
            CMD32ENW::CMD32EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD32ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD32ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD32ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    #[inline]
    pub fn cmd32en_0(self) -> &'a mut W {
        self.variant(CMD32ENW::CMD32EN_0)
    }
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    #[inline]
    pub fn cmd32en_1(self) -> &'a mut W {
        self.variant(CMD32ENW::CMD32EN_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLG`"]
pub enum FLGW {
    #[doc = "No interrupt occurred."]
    FLG_0,
    #[doc = "An interrupt occurred."]
    FLG_1,
}
impl FLGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLGW::FLG_0 => false,
            FLGW::FLG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLGW<'a> {
    w: &'a mut W,
}
impl<'a> _FLGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt occurred."]
    #[inline]
    pub fn flg_0(self) -> &'a mut W {
        self.variant(FLGW::FLG_0)
    }
    #[doc = "An interrupt occurred."]
    #[inline]
    pub fn flg_1(self) -> &'a mut W {
        self.variant(FLGW::FLG_1)
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
#[doc = "Values that can be written to the field `WIN`"]
pub enum WINW {
    #[doc = "Window mode disabled."]
    WIN_0,
    #[doc = "Window mode enabled."]
    WIN_1,
}
impl WINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WINW::WIN_0 => false,
            WINW::WIN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WINW<'a> {
    w: &'a mut W,
}
impl<'a> _WINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Window mode disabled."]
    #[inline]
    pub fn win_0(self) -> &'a mut W {
        self.variant(WINW::WIN_0)
    }
    #[doc = "Window mode enabled."]
    #[inline]
    pub fn win_1(self) -> &'a mut W {
        self.variant(WINW::WIN_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Stop Enable"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline]
    pub fn wait(&self) -> WAITR {
        WAITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline]
    pub fn dbg(&self) -> DBGR {
        DBGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline]
    pub fn tst(&self) -> TSTR {
        TSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline]
    pub fn update(&self) -> UPDATER {
        UPDATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline]
    pub fn int(&self) -> INTR {
        INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline]
    pub fn clk(&self) -> CLKR {
        CLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Reconfiguration Success"]
    #[inline]
    pub fn rcs(&self) -> RCSR {
        RCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Unlock status"]
    #[inline]
    pub fn ulk(&self) -> ULKR {
        ULKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline]
    pub fn pres(&self) -> PRESR {
        PRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline]
    pub fn cmd32en(&self) -> CMD32ENR {
        CMD32ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline]
    pub fn flg(&self) -> FLGR {
        FLGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline]
    pub fn win(&self) -> WINR {
        WINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 10624 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Stop Enable"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline]
    pub fn wait(&mut self) -> _WAITW {
        _WAITW { w: self }
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline]
    pub fn dbg(&mut self) -> _DBGW {
        _DBGW { w: self }
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline]
    pub fn tst(&mut self) -> _TSTW {
        _TSTW { w: self }
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline]
    pub fn update(&mut self) -> _UPDATEW {
        _UPDATEW { w: self }
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline]
    pub fn int(&mut self) -> _INTW {
        _INTW { w: self }
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline]
    pub fn clk(&mut self) -> _CLKW {
        _CLKW { w: self }
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline]
    pub fn pres(&mut self) -> _PRESW {
        _PRESW { w: self }
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline]
    pub fn cmd32en(&mut self) -> _CMD32ENW {
        _CMD32ENW { w: self }
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline]
    pub fn flg(&mut self) -> _FLGW {
        _FLGW { w: self }
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline]
    pub fn win(&mut self) -> _WINW {
        _WINW { w: self }
    }
}
