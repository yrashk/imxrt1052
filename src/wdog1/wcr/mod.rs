#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::WCR {
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
#[doc = "Possible values of the field `WDZST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDZSTR {
    #[doc = "Continue timer operation (Default)."]
    WDZST_0,
    #[doc = "Suspend the watchdog timer."]
    WDZST_1,
}
impl WDZSTR {
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
            WDZSTR::WDZST_0 => false,
            WDZSTR::WDZST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDZSTR {
        match value {
            false => WDZSTR::WDZST_0,
            true => WDZSTR::WDZST_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDZST_0`"]
    #[inline]
    pub fn is_wdzst_0(&self) -> bool {
        *self == WDZSTR::WDZST_0
    }
    #[doc = "Checks if the value of the field is `WDZST_1`"]
    #[inline]
    pub fn is_wdzst_1(&self) -> bool {
        *self == WDZSTR::WDZST_1
    }
}
#[doc = "Possible values of the field `WDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDBGR {
    #[doc = "Continue WDOG timer operation (Default)."]
    WDBG_0,
    #[doc = "Suspend the watchdog timer."]
    WDBG_1,
}
impl WDBGR {
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
            WDBGR::WDBG_0 => false,
            WDBGR::WDBG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDBGR {
        match value {
            false => WDBGR::WDBG_0,
            true => WDBGR::WDBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDBG_0`"]
    #[inline]
    pub fn is_wdbg_0(&self) -> bool {
        *self == WDBGR::WDBG_0
    }
    #[doc = "Checks if the value of the field is `WDBG_1`"]
    #[inline]
    pub fn is_wdbg_1(&self) -> bool {
        *self == WDBGR::WDBG_1
    }
}
#[doc = "Possible values of the field `WDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDER {
    #[doc = "Disable the Watchdog (Default)."]
    WDE_0,
    #[doc = "Enable the Watchdog."]
    WDE_1,
}
impl WDER {
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
            WDER::WDE_0 => false,
            WDER::WDE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDER {
        match value {
            false => WDER::WDE_0,
            true => WDER::WDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDE_0`"]
    #[inline]
    pub fn is_wde_0(&self) -> bool {
        *self == WDER::WDE_0
    }
    #[doc = "Checks if the value of the field is `WDE_1`"]
    #[inline]
    pub fn is_wde_1(&self) -> bool {
        *self == WDER::WDE_1
    }
}
#[doc = "Possible values of the field `WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTR {
    #[doc = "no description available"]
    WDT_0,
    #[doc = "no description available"]
    WDT_1,
}
impl WDTR {
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
            WDTR::WDT_0 => false,
            WDTR::WDT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTR {
        match value {
            false => WDTR::WDT_0,
            true => WDTR::WDT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDT_0`"]
    #[inline]
    pub fn is_wdt_0(&self) -> bool {
        *self == WDTR::WDT_0
    }
    #[doc = "Checks if the value of the field is `WDT_1`"]
    #[inline]
    pub fn is_wdt_1(&self) -> bool {
        *self == WDTR::WDT_1
    }
}
#[doc = "Possible values of the field `SRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSR {
    #[doc = "Assert system reset signal."]
    SRS_0,
    #[doc = "No effect on the system (Default)."]
    SRS_1,
}
impl SRSR {
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
            SRSR::SRS_0 => false,
            SRSR::SRS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSR {
        match value {
            false => SRSR::SRS_0,
            true => SRSR::SRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRS_0`"]
    #[inline]
    pub fn is_srs_0(&self) -> bool {
        *self == SRSR::SRS_0
    }
    #[doc = "Checks if the value of the field is `SRS_1`"]
    #[inline]
    pub fn is_srs_1(&self) -> bool {
        *self == SRSR::SRS_1
    }
}
#[doc = "Possible values of the field `WDA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDAR {
    #[doc = "no description available"]
    WDA_0,
    #[doc = "No effect on system (Default)."]
    WDA_1,
}
impl WDAR {
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
            WDAR::WDA_0 => false,
            WDAR::WDA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDAR {
        match value {
            false => WDAR::WDA_0,
            true => WDAR::WDA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDA_0`"]
    #[inline]
    pub fn is_wda_0(&self) -> bool {
        *self == WDAR::WDA_0
    }
    #[doc = "Checks if the value of the field is `WDA_1`"]
    #[inline]
    pub fn is_wda_1(&self) -> bool {
        *self == WDAR::WDA_1
    }
}
#[doc = "Possible values of the field `SRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRER {
    #[doc = "using original way to generate software reset (default)"]
    SRE_0,
    #[doc = "using new way to generate software reset."]
    SRE_1,
}
impl SRER {
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
            SRER::SRE_0 => false,
            SRER::SRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRER {
        match value {
            false => SRER::SRE_0,
            true => SRER::SRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRE_0`"]
    #[inline]
    pub fn is_sre_0(&self) -> bool {
        *self == SRER::SRE_0
    }
    #[doc = "Checks if the value of the field is `SRE_1`"]
    #[inline]
    pub fn is_sre_1(&self) -> bool {
        *self == SRER::SRE_1
    }
}
#[doc = "Possible values of the field `WDW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDWR {
    #[doc = "Continue WDOG timer operation (Default)."]
    WDW_0,
    #[doc = "Suspend WDOG timer operation."]
    WDW_1,
}
impl WDWR {
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
            WDWR::WDW_0 => false,
            WDWR::WDW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDWR {
        match value {
            false => WDWR::WDW_0,
            true => WDWR::WDW_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDW_0`"]
    #[inline]
    pub fn is_wdw_0(&self) -> bool {
        *self == WDWR::WDW_0
    }
    #[doc = "Checks if the value of the field is `WDW_1`"]
    #[inline]
    pub fn is_wdw_1(&self) -> bool {
        *self == WDWR::WDW_1
    }
}
#[doc = "Possible values of the field `WT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTR {
    #[doc = "- 0.5 Seconds (Default)."]
    WT_0,
    #[doc = "- 1.0 Seconds."]
    WT_1,
    #[doc = "- 1.5 Seconds."]
    WT_2,
    #[doc = "- 2.0 Seconds."]
    WT_3,
    #[doc = "- 128 Seconds."]
    WT_255,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WTR::WT_0 => 0,
            WTR::WT_1 => 1,
            WTR::WT_2 => 2,
            WTR::WT_3 => 3,
            WTR::WT_255 => 255,
            WTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WTR {
        match value {
            0 => WTR::WT_0,
            1 => WTR::WT_1,
            2 => WTR::WT_2,
            3 => WTR::WT_3,
            255 => WTR::WT_255,
            i => WTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WT_0`"]
    #[inline]
    pub fn is_wt_0(&self) -> bool {
        *self == WTR::WT_0
    }
    #[doc = "Checks if the value of the field is `WT_1`"]
    #[inline]
    pub fn is_wt_1(&self) -> bool {
        *self == WTR::WT_1
    }
    #[doc = "Checks if the value of the field is `WT_2`"]
    #[inline]
    pub fn is_wt_2(&self) -> bool {
        *self == WTR::WT_2
    }
    #[doc = "Checks if the value of the field is `WT_3`"]
    #[inline]
    pub fn is_wt_3(&self) -> bool {
        *self == WTR::WT_3
    }
    #[doc = "Checks if the value of the field is `WT_255`"]
    #[inline]
    pub fn is_wt_255(&self) -> bool {
        *self == WTR::WT_255
    }
}
#[doc = "Values that can be written to the field `WDZST`"]
pub enum WDZSTW {
    #[doc = "Continue timer operation (Default)."]
    WDZST_0,
    #[doc = "Suspend the watchdog timer."]
    WDZST_1,
}
impl WDZSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDZSTW::WDZST_0 => false,
            WDZSTW::WDZST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDZSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDZSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDZSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continue timer operation (Default)."]
    #[inline]
    pub fn wdzst_0(self) -> &'a mut W {
        self.variant(WDZSTW::WDZST_0)
    }
    #[doc = "Suspend the watchdog timer."]
    #[inline]
    pub fn wdzst_1(self) -> &'a mut W {
        self.variant(WDZSTW::WDZST_1)
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
#[doc = "Values that can be written to the field `WDBG`"]
pub enum WDBGW {
    #[doc = "Continue WDOG timer operation (Default)."]
    WDBG_0,
    #[doc = "Suspend the watchdog timer."]
    WDBG_1,
}
impl WDBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDBGW::WDBG_0 => false,
            WDBGW::WDBG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDBGW<'a> {
    w: &'a mut W,
}
impl<'a> _WDBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continue WDOG timer operation (Default)."]
    #[inline]
    pub fn wdbg_0(self) -> &'a mut W {
        self.variant(WDBGW::WDBG_0)
    }
    #[doc = "Suspend the watchdog timer."]
    #[inline]
    pub fn wdbg_1(self) -> &'a mut W {
        self.variant(WDBGW::WDBG_1)
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
#[doc = "Values that can be written to the field `WDE`"]
pub enum WDEW {
    #[doc = "Disable the Watchdog (Default)."]
    WDE_0,
    #[doc = "Enable the Watchdog."]
    WDE_1,
}
impl WDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDEW::WDE_0 => false,
            WDEW::WDE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDEW<'a> {
    w: &'a mut W,
}
impl<'a> _WDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the Watchdog (Default)."]
    #[inline]
    pub fn wde_0(self) -> &'a mut W {
        self.variant(WDEW::WDE_0)
    }
    #[doc = "Enable the Watchdog."]
    #[inline]
    pub fn wde_1(self) -> &'a mut W {
        self.variant(WDEW::WDE_1)
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
#[doc = "Values that can be written to the field `WDT`"]
pub enum WDTW {
    #[doc = "no description available"]
    WDT_0,
    #[doc = "no description available"]
    WDT_1,
}
impl WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTW::WDT_0 => false,
            WDTW::WDT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn wdt_0(self) -> &'a mut W {
        self.variant(WDTW::WDT_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn wdt_1(self) -> &'a mut W {
        self.variant(WDTW::WDT_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRS`"]
pub enum SRSW {
    #[doc = "Assert system reset signal."]
    SRS_0,
    #[doc = "No effect on the system (Default)."]
    SRS_1,
}
impl SRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRSW::SRS_0 => false,
            SRSW::SRS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRSW<'a> {
    w: &'a mut W,
}
impl<'a> _SRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert system reset signal."]
    #[inline]
    pub fn srs_0(self) -> &'a mut W {
        self.variant(SRSW::SRS_0)
    }
    #[doc = "No effect on the system (Default)."]
    #[inline]
    pub fn srs_1(self) -> &'a mut W {
        self.variant(SRSW::SRS_1)
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
#[doc = "Values that can be written to the field `WDA`"]
pub enum WDAW {
    #[doc = "no description available"]
    WDA_0,
    #[doc = "No effect on system (Default)."]
    WDA_1,
}
impl WDAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDAW::WDA_0 => false,
            WDAW::WDA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDAW<'a> {
    w: &'a mut W,
}
impl<'a> _WDAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn wda_0(self) -> &'a mut W {
        self.variant(WDAW::WDA_0)
    }
    #[doc = "No effect on system (Default)."]
    #[inline]
    pub fn wda_1(self) -> &'a mut W {
        self.variant(WDAW::WDA_1)
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
#[doc = "Values that can be written to the field `SRE`"]
pub enum SREW {
    #[doc = "using original way to generate software reset (default)"]
    SRE_0,
    #[doc = "using new way to generate software reset."]
    SRE_1,
}
impl SREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SREW::SRE_0 => false,
            SREW::SRE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SREW<'a> {
    w: &'a mut W,
}
impl<'a> _SREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "using original way to generate software reset (default)"]
    #[inline]
    pub fn sre_0(self) -> &'a mut W {
        self.variant(SREW::SRE_0)
    }
    #[doc = "using new way to generate software reset."]
    #[inline]
    pub fn sre_1(self) -> &'a mut W {
        self.variant(SREW::SRE_1)
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
#[doc = "Values that can be written to the field `WDW`"]
pub enum WDWW {
    #[doc = "Continue WDOG timer operation (Default)."]
    WDW_0,
    #[doc = "Suspend WDOG timer operation."]
    WDW_1,
}
impl WDWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDWW::WDW_0 => false,
            WDWW::WDW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDWW<'a> {
    w: &'a mut W,
}
impl<'a> _WDWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continue WDOG timer operation (Default)."]
    #[inline]
    pub fn wdw_0(self) -> &'a mut W {
        self.variant(WDWW::WDW_0)
    }
    #[doc = "Suspend WDOG timer operation."]
    #[inline]
    pub fn wdw_1(self) -> &'a mut W {
        self.variant(WDWW::WDW_1)
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
#[doc = "Values that can be written to the field `WT`"]
pub enum WTW {
    #[doc = "- 0.5 Seconds (Default)."]
    WT_0,
    #[doc = "- 1.0 Seconds."]
    WT_1,
    #[doc = "- 1.5 Seconds."]
    WT_2,
    #[doc = "- 2.0 Seconds."]
    WT_3,
    #[doc = "- 128 Seconds."]
    WT_255,
}
impl WTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WTW::WT_0 => 0,
            WTW::WT_1 => 1,
            WTW::WT_2 => 2,
            WTW::WT_3 => 3,
            WTW::WT_255 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTW<'a> {
    w: &'a mut W,
}
impl<'a> _WTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "- 0.5 Seconds (Default)."]
    #[inline]
    pub fn wt_0(self) -> &'a mut W {
        self.variant(WTW::WT_0)
    }
    #[doc = "- 1.0 Seconds."]
    #[inline]
    pub fn wt_1(self) -> &'a mut W {
        self.variant(WTW::WT_1)
    }
    #[doc = "- 1.5 Seconds."]
    #[inline]
    pub fn wt_2(self) -> &'a mut W {
        self.variant(WTW::WT_2)
    }
    #[doc = "- 2.0 Seconds."]
    #[inline]
    pub fn wt_3(self) -> &'a mut W {
        self.variant(WTW::WT_3)
    }
    #[doc = "- 128 Seconds."]
    #[inline]
    pub fn wt_255(self) -> &'a mut W {
        self.variant(WTW::WT_255)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - WDZST"]
    #[inline]
    pub fn wdzst(&self) -> WDZSTR {
        WDZSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - WDBG"]
    #[inline]
    pub fn wdbg(&self) -> WDBGR {
        WDBGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - WDE"]
    #[inline]
    pub fn wde(&self) -> WDER {
        WDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - WDT"]
    #[inline]
    pub fn wdt(&self) -> WDTR {
        WDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - SRS"]
    #[inline]
    pub fn srs(&self) -> SRSR {
        SRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - WDA"]
    #[inline]
    pub fn wda(&self) -> WDAR {
        WDAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - software reset extension, an option way to generate software reset"]
    #[inline]
    pub fn sre(&self) -> SRER {
        SRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - WDW"]
    #[inline]
    pub fn wdw(&self) -> WDWR {
        WDWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:15 - WT"]
    #[inline]
    pub fn wt(&self) -> WTR {
        WTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 48 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - WDZST"]
    #[inline]
    pub fn wdzst(&mut self) -> _WDZSTW {
        _WDZSTW { w: self }
    }
    #[doc = "Bit 1 - WDBG"]
    #[inline]
    pub fn wdbg(&mut self) -> _WDBGW {
        _WDBGW { w: self }
    }
    #[doc = "Bit 2 - WDE"]
    #[inline]
    pub fn wde(&mut self) -> _WDEW {
        _WDEW { w: self }
    }
    #[doc = "Bit 3 - WDT"]
    #[inline]
    pub fn wdt(&mut self) -> _WDTW {
        _WDTW { w: self }
    }
    #[doc = "Bit 4 - SRS"]
    #[inline]
    pub fn srs(&mut self) -> _SRSW {
        _SRSW { w: self }
    }
    #[doc = "Bit 5 - WDA"]
    #[inline]
    pub fn wda(&mut self) -> _WDAW {
        _WDAW { w: self }
    }
    #[doc = "Bit 6 - software reset extension, an option way to generate software reset"]
    #[inline]
    pub fn sre(&mut self) -> _SREW {
        _SREW { w: self }
    }
    #[doc = "Bit 7 - WDW"]
    #[inline]
    pub fn wdw(&mut self) -> _WDWW {
        _WDWW { w: self }
    }
    #[doc = "Bits 8:15 - WT"]
    #[inline]
    pub fn wt(&mut self) -> _WTW {
        _WTW { w: self }
    }
}
