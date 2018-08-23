#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
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
#[doc = "Possible values of the field `CMPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPIER {
    #[doc = "Compare interrupt is disabled"]
    CMPIE_0,
    #[doc = "Compare interrupt is enabled"]
    CMPIE_1,
}
impl CMPIER {
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
            CMPIER::CMPIE_0 => false,
            CMPIER::CMPIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPIER {
        match value {
            false => CMPIER::CMPIE_0,
            true => CMPIER::CMPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMPIE_0`"]
    #[inline]
    pub fn is_cmpie_0(&self) -> bool {
        *self == CMPIER::CMPIE_0
    }
    #[doc = "Checks if the value of the field is `CMPIE_1`"]
    #[inline]
    pub fn is_cmpie_1(&self) -> bool {
        *self == CMPIER::CMPIE_1
    }
}
#[doc = "Possible values of the field `CMPIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPIRQR {
    #[doc = "No match has occurred"]
    CMPIRQ_0,
    #[doc = "COMP match has occurred"]
    CMPIRQ_1,
}
impl CMPIRQR {
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
            CMPIRQR::CMPIRQ_0 => false,
            CMPIRQR::CMPIRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPIRQR {
        match value {
            false => CMPIRQR::CMPIRQ_0,
            true => CMPIRQR::CMPIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMPIRQ_0`"]
    #[inline]
    pub fn is_cmpirq_0(&self) -> bool {
        *self == CMPIRQR::CMPIRQ_0
    }
    #[doc = "Checks if the value of the field is `CMPIRQ_1`"]
    #[inline]
    pub fn is_cmpirq_1(&self) -> bool {
        *self == CMPIRQR::CMPIRQ_1
    }
}
#[doc = "Possible values of the field `WDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDER {
    #[doc = "Watchdog timer is disabled"]
    WDE_0,
    #[doc = "Watchdog timer is enabled"]
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
#[doc = "Possible values of the field `DIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIER {
    #[doc = "Watchdog timer interrupt is disabled"]
    DIE_0,
    #[doc = "Watchdog timer interrupt is enabled"]
    DIE_1,
}
impl DIER {
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
            DIER::DIE_0 => false,
            DIER::DIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIER {
        match value {
            false => DIER::DIE_0,
            true => DIER::DIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIE_0`"]
    #[inline]
    pub fn is_die_0(&self) -> bool {
        *self == DIER::DIE_0
    }
    #[doc = "Checks if the value of the field is `DIE_1`"]
    #[inline]
    pub fn is_die_1(&self) -> bool {
        *self == DIER::DIE_1
    }
}
#[doc = "Possible values of the field `DIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRQR {
    #[doc = "No interrupt has occurred"]
    DIRQ_0,
    #[doc = "Watchdog timeout interrupt has occurred"]
    DIRQ_1,
}
impl DIRQR {
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
            DIRQR::DIRQ_0 => false,
            DIRQR::DIRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRQR {
        match value {
            false => DIRQR::DIRQ_0,
            true => DIRQR::DIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRQ_0`"]
    #[inline]
    pub fn is_dirq_0(&self) -> bool {
        *self == DIRQR::DIRQ_0
    }
    #[doc = "Checks if the value of the field is `DIRQ_1`"]
    #[inline]
    pub fn is_dirq_1(&self) -> bool {
        *self == DIRQR::DIRQ_1
    }
}
#[doc = "Possible values of the field `XNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNER {
    #[doc = "Use positive transition edge of INDEX pulse"]
    XNE_0,
    #[doc = "Use negative transition edge of INDEX pulse"]
    XNE_1,
}
impl XNER {
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
            XNER::XNE_0 => false,
            XNER::XNE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XNER {
        match value {
            false => XNER::XNE_0,
            true => XNER::XNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNE_0`"]
    #[inline]
    pub fn is_xne_0(&self) -> bool {
        *self == XNER::XNE_0
    }
    #[doc = "Checks if the value of the field is `XNE_1`"]
    #[inline]
    pub fn is_xne_1(&self) -> bool {
        *self == XNER::XNE_1
    }
}
#[doc = "Possible values of the field `XIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIPR {
    #[doc = "No action"]
    XIP_0,
    #[doc = "INDEX pulse initializes the position counter"]
    XIP_1,
}
impl XIPR {
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
            XIPR::XIP_0 => false,
            XIPR::XIP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XIPR {
        match value {
            false => XIPR::XIP_0,
            true => XIPR::XIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIP_0`"]
    #[inline]
    pub fn is_xip_0(&self) -> bool {
        *self == XIPR::XIP_0
    }
    #[doc = "Checks if the value of the field is `XIP_1`"]
    #[inline]
    pub fn is_xip_1(&self) -> bool {
        *self == XIPR::XIP_1
    }
}
#[doc = "Possible values of the field `XIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIER {
    #[doc = "INDEX pulse interrupt is disabled"]
    XIE_0,
    #[doc = "INDEX pulse interrupt is enabled"]
    XIE_1,
}
impl XIER {
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
            XIER::XIE_0 => false,
            XIER::XIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XIER {
        match value {
            false => XIER::XIE_0,
            true => XIER::XIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIE_0`"]
    #[inline]
    pub fn is_xie_0(&self) -> bool {
        *self == XIER::XIE_0
    }
    #[doc = "Checks if the value of the field is `XIE_1`"]
    #[inline]
    pub fn is_xie_1(&self) -> bool {
        *self == XIER::XIE_1
    }
}
#[doc = "Possible values of the field `XIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIRQR {
    #[doc = "No interrupt has occurred"]
    XIRQ_0,
    #[doc = "INDEX pulse interrupt has occurred"]
    XIRQ_1,
}
impl XIRQR {
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
            XIRQR::XIRQ_0 => false,
            XIRQR::XIRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XIRQR {
        match value {
            false => XIRQR::XIRQ_0,
            true => XIRQR::XIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIRQ_0`"]
    #[inline]
    pub fn is_xirq_0(&self) -> bool {
        *self == XIRQR::XIRQ_0
    }
    #[doc = "Checks if the value of the field is `XIRQ_1`"]
    #[inline]
    pub fn is_xirq_1(&self) -> bool {
        *self == XIRQR::XIRQ_1
    }
}
#[doc = "Possible values of the field `PH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PH1R {
    #[doc = "Use standard quadrature decoder where PHASEA and PHASEB represent a two phase quadrature signal."]
    PH1_0,
    #[doc = "Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction. If CTRL[REV] = 0, PHASEB = 0, then count up If CTRL[REV] = 0, PHASEB = 1, then count down If CTRL[REV] = 1, PHASEB = 0, then count down If CTRL[REV] = 1, PHASEB = 1, then count up"]
    PH1_1,
}
impl PH1R {
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
            PH1R::PH1_0 => false,
            PH1R::PH1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PH1R {
        match value {
            false => PH1R::PH1_0,
            true => PH1R::PH1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PH1_0`"]
    #[inline]
    pub fn is_ph1_0(&self) -> bool {
        *self == PH1R::PH1_0
    }
    #[doc = "Checks if the value of the field is `PH1_1`"]
    #[inline]
    pub fn is_ph1_1(&self) -> bool {
        *self == PH1R::PH1_1
    }
}
#[doc = "Possible values of the field `REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVR {
    #[doc = "Count normally"]
    REV_0,
    #[doc = "Count in the reverse direction"]
    REV_1,
}
impl REVR {
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
            REVR::REV_0 => false,
            REVR::REV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REVR {
        match value {
            false => REVR::REV_0,
            true => REVR::REV_1,
        }
    }
    #[doc = "Checks if the value of the field is `REV_0`"]
    #[inline]
    pub fn is_rev_0(&self) -> bool {
        *self == REVR::REV_0
    }
    #[doc = "Checks if the value of the field is `REV_1`"]
    #[inline]
    pub fn is_rev_1(&self) -> bool {
        *self == REVR::REV_1
    }
}
#[doc = "Possible values of the field `HNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HNER {
    #[doc = "Use positive going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_0,
    #[doc = "Use negative going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_1,
}
impl HNER {
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
            HNER::HNE_0 => false,
            HNER::HNE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HNER {
        match value {
            false => HNER::HNE_0,
            true => HNER::HNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HNE_0`"]
    #[inline]
    pub fn is_hne_0(&self) -> bool {
        *self == HNER::HNE_0
    }
    #[doc = "Checks if the value of the field is `HNE_1`"]
    #[inline]
    pub fn is_hne_1(&self) -> bool {
        *self == HNER::HNE_1
    }
}
#[doc = "Possible values of the field `HIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIPR {
    #[doc = "No action"]
    HIP_0,
    #[doc = "HOME signal initializes the position counter"]
    HIP_1,
}
impl HIPR {
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
            HIPR::HIP_0 => false,
            HIPR::HIP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIPR {
        match value {
            false => HIPR::HIP_0,
            true => HIPR::HIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIP_0`"]
    #[inline]
    pub fn is_hip_0(&self) -> bool {
        *self == HIPR::HIP_0
    }
    #[doc = "Checks if the value of the field is `HIP_1`"]
    #[inline]
    pub fn is_hip_1(&self) -> bool {
        *self == HIPR::HIP_1
    }
}
#[doc = "Possible values of the field `HIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIER {
    #[doc = "Disable HOME interrupts"]
    HIE_0,
    #[doc = "Enable HOME interrupts"]
    HIE_1,
}
impl HIER {
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
            HIER::HIE_0 => false,
            HIER::HIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIER {
        match value {
            false => HIER::HIE_0,
            true => HIER::HIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIE_0`"]
    #[inline]
    pub fn is_hie_0(&self) -> bool {
        *self == HIER::HIE_0
    }
    #[doc = "Checks if the value of the field is `HIE_1`"]
    #[inline]
    pub fn is_hie_1(&self) -> bool {
        *self == HIER::HIE_1
    }
}
#[doc = "Possible values of the field `HIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRQR {
    #[doc = "No interrupt"]
    HIRQ_0,
    #[doc = "HOME signal transition interrupt request"]
    HIRQ_1,
}
impl HIRQR {
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
            HIRQR::HIRQ_0 => false,
            HIRQR::HIRQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIRQR {
        match value {
            false => HIRQR::HIRQ_0,
            true => HIRQR::HIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIRQ_0`"]
    #[inline]
    pub fn is_hirq_0(&self) -> bool {
        *self == HIRQR::HIRQ_0
    }
    #[doc = "Checks if the value of the field is `HIRQ_1`"]
    #[inline]
    pub fn is_hirq_1(&self) -> bool {
        *self == HIRQR::HIRQ_1
    }
}
#[doc = "Values that can be written to the field `CMPIE`"]
pub enum CMPIEW {
    #[doc = "Compare interrupt is disabled"]
    CMPIE_0,
    #[doc = "Compare interrupt is enabled"]
    CMPIE_1,
}
impl CMPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMPIEW::CMPIE_0 => false,
            CMPIEW::CMPIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare interrupt is disabled"]
    #[inline]
    pub fn cmpie_0(self) -> &'a mut W {
        self.variant(CMPIEW::CMPIE_0)
    }
    #[doc = "Compare interrupt is enabled"]
    #[inline]
    pub fn cmpie_1(self) -> &'a mut W {
        self.variant(CMPIEW::CMPIE_1)
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
#[doc = "Values that can be written to the field `CMPIRQ`"]
pub enum CMPIRQW {
    #[doc = "No match has occurred"]
    CMPIRQ_0,
    #[doc = "COMP match has occurred"]
    CMPIRQ_1,
}
impl CMPIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMPIRQW::CMPIRQ_0 => false,
            CMPIRQW::CMPIRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No match has occurred"]
    #[inline]
    pub fn cmpirq_0(self) -> &'a mut W {
        self.variant(CMPIRQW::CMPIRQ_0)
    }
    #[doc = "COMP match has occurred"]
    #[inline]
    pub fn cmpirq_1(self) -> &'a mut W {
        self.variant(CMPIRQW::CMPIRQ_1)
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
    #[doc = "Watchdog timer is disabled"]
    WDE_0,
    #[doc = "Watchdog timer is enabled"]
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
    #[doc = "Watchdog timer is disabled"]
    #[inline]
    pub fn wde_0(self) -> &'a mut W {
        self.variant(WDEW::WDE_0)
    }
    #[doc = "Watchdog timer is enabled"]
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
#[doc = "Values that can be written to the field `DIE`"]
pub enum DIEW {
    #[doc = "Watchdog timer interrupt is disabled"]
    DIE_0,
    #[doc = "Watchdog timer interrupt is enabled"]
    DIE_1,
}
impl DIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIEW::DIE_0 => false,
            DIEW::DIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog timer interrupt is disabled"]
    #[inline]
    pub fn die_0(self) -> &'a mut W {
        self.variant(DIEW::DIE_0)
    }
    #[doc = "Watchdog timer interrupt is enabled"]
    #[inline]
    pub fn die_1(self) -> &'a mut W {
        self.variant(DIEW::DIE_1)
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
#[doc = "Values that can be written to the field `DIRQ`"]
pub enum DIRQW {
    #[doc = "No interrupt has occurred"]
    DIRQ_0,
    #[doc = "Watchdog timeout interrupt has occurred"]
    DIRQ_1,
}
impl DIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRQW::DIRQ_0 => false,
            DIRQW::DIRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt has occurred"]
    #[inline]
    pub fn dirq_0(self) -> &'a mut W {
        self.variant(DIRQW::DIRQ_0)
    }
    #[doc = "Watchdog timeout interrupt has occurred"]
    #[inline]
    pub fn dirq_1(self) -> &'a mut W {
        self.variant(DIRQW::DIRQ_1)
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
#[doc = "Values that can be written to the field `XNE`"]
pub enum XNEW {
    #[doc = "Use positive transition edge of INDEX pulse"]
    XNE_0,
    #[doc = "Use negative transition edge of INDEX pulse"]
    XNE_1,
}
impl XNEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XNEW::XNE_0 => false,
            XNEW::XNE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XNEW<'a> {
    w: &'a mut W,
}
impl<'a> _XNEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XNEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use positive transition edge of INDEX pulse"]
    #[inline]
    pub fn xne_0(self) -> &'a mut W {
        self.variant(XNEW::XNE_0)
    }
    #[doc = "Use negative transition edge of INDEX pulse"]
    #[inline]
    pub fn xne_1(self) -> &'a mut W {
        self.variant(XNEW::XNE_1)
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
#[doc = "Values that can be written to the field `XIP`"]
pub enum XIPW {
    #[doc = "No action"]
    XIP_0,
    #[doc = "INDEX pulse initializes the position counter"]
    XIP_1,
}
impl XIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XIPW::XIP_0 => false,
            XIPW::XIP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XIPW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn xip_0(self) -> &'a mut W {
        self.variant(XIPW::XIP_0)
    }
    #[doc = "INDEX pulse initializes the position counter"]
    #[inline]
    pub fn xip_1(self) -> &'a mut W {
        self.variant(XIPW::XIP_1)
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
#[doc = "Values that can be written to the field `XIE`"]
pub enum XIEW {
    #[doc = "INDEX pulse interrupt is disabled"]
    XIE_0,
    #[doc = "INDEX pulse interrupt is enabled"]
    XIE_1,
}
impl XIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XIEW::XIE_0 => false,
            XIEW::XIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XIEW<'a> {
    w: &'a mut W,
}
impl<'a> _XIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INDEX pulse interrupt is disabled"]
    #[inline]
    pub fn xie_0(self) -> &'a mut W {
        self.variant(XIEW::XIE_0)
    }
    #[doc = "INDEX pulse interrupt is enabled"]
    #[inline]
    pub fn xie_1(self) -> &'a mut W {
        self.variant(XIEW::XIE_1)
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
#[doc = "Values that can be written to the field `XIRQ`"]
pub enum XIRQW {
    #[doc = "No interrupt has occurred"]
    XIRQ_0,
    #[doc = "INDEX pulse interrupt has occurred"]
    XIRQ_1,
}
impl XIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XIRQW::XIRQ_0 => false,
            XIRQW::XIRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _XIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt has occurred"]
    #[inline]
    pub fn xirq_0(self) -> &'a mut W {
        self.variant(XIRQW::XIRQ_0)
    }
    #[doc = "INDEX pulse interrupt has occurred"]
    #[inline]
    pub fn xirq_1(self) -> &'a mut W {
        self.variant(XIRQW::XIRQ_1)
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
#[doc = "Values that can be written to the field `PH1`"]
pub enum PH1W {
    #[doc = "Use standard quadrature decoder where PHASEA and PHASEB represent a two phase quadrature signal."]
    PH1_0,
    #[doc = "Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction. If CTRL[REV] = 0, PHASEB = 0, then count up If CTRL[REV] = 0, PHASEB = 1, then count down If CTRL[REV] = 1, PHASEB = 0, then count down If CTRL[REV] = 1, PHASEB = 1, then count up"]
    PH1_1,
}
impl PH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PH1W::PH1_0 => false,
            PH1W::PH1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PH1W<'a> {
    w: &'a mut W,
}
impl<'a> _PH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use standard quadrature decoder where PHASEA and PHASEB represent a two phase quadrature signal."]
    #[inline]
    pub fn ph1_0(self) -> &'a mut W {
        self.variant(PH1W::PH1_0)
    }
    #[doc = "Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction. If CTRL[REV] = 0, PHASEB = 0, then count up If CTRL[REV] = 0, PHASEB = 1, then count down If CTRL[REV] = 1, PHASEB = 0, then count down If CTRL[REV] = 1, PHASEB = 1, then count up"]
    #[inline]
    pub fn ph1_1(self) -> &'a mut W {
        self.variant(PH1W::PH1_1)
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
#[doc = "Values that can be written to the field `REV`"]
pub enum REVW {
    #[doc = "Count normally"]
    REV_0,
    #[doc = "Count in the reverse direction"]
    REV_1,
}
impl REVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REVW::REV_0 => false,
            REVW::REV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REVW<'a> {
    w: &'a mut W,
}
impl<'a> _REVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Count normally"]
    #[inline]
    pub fn rev_0(self) -> &'a mut W {
        self.variant(REVW::REV_0)
    }
    #[doc = "Count in the reverse direction"]
    #[inline]
    pub fn rev_1(self) -> &'a mut W {
        self.variant(REVW::REV_1)
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
#[doc = "Values that can be written to the field `SWIP`"]
pub enum SWIPW {
    #[doc = "No action"]
    SWIP_0,
    #[doc = "Initialize position counter"]
    SWIP_1,
}
impl SWIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWIPW::SWIP_0 => false,
            SWIPW::SWIP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWIPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn swip_0(self) -> &'a mut W {
        self.variant(SWIPW::SWIP_0)
    }
    #[doc = "Initialize position counter"]
    #[inline]
    pub fn swip_1(self) -> &'a mut W {
        self.variant(SWIPW::SWIP_1)
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
#[doc = "Values that can be written to the field `HNE`"]
pub enum HNEW {
    #[doc = "Use positive going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_0,
    #[doc = "Use negative going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_1,
}
impl HNEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HNEW::HNE_0 => false,
            HNEW::HNE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HNEW<'a> {
    w: &'a mut W,
}
impl<'a> _HNEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HNEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use positive going edge-to-trigger initialization of position counters UPOS and LPOS"]
    #[inline]
    pub fn hne_0(self) -> &'a mut W {
        self.variant(HNEW::HNE_0)
    }
    #[doc = "Use negative going edge-to-trigger initialization of position counters UPOS and LPOS"]
    #[inline]
    pub fn hne_1(self) -> &'a mut W {
        self.variant(HNEW::HNE_1)
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
#[doc = "Values that can be written to the field `HIP`"]
pub enum HIPW {
    #[doc = "No action"]
    HIP_0,
    #[doc = "HOME signal initializes the position counter"]
    HIP_1,
}
impl HIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIPW::HIP_0 => false,
            HIPW::HIP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIPW<'a> {
    w: &'a mut W,
}
impl<'a> _HIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn hip_0(self) -> &'a mut W {
        self.variant(HIPW::HIP_0)
    }
    #[doc = "HOME signal initializes the position counter"]
    #[inline]
    pub fn hip_1(self) -> &'a mut W {
        self.variant(HIPW::HIP_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIE`"]
pub enum HIEW {
    #[doc = "Disable HOME interrupts"]
    HIE_0,
    #[doc = "Enable HOME interrupts"]
    HIE_1,
}
impl HIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIEW::HIE_0 => false,
            HIEW::HIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable HOME interrupts"]
    #[inline]
    pub fn hie_0(self) -> &'a mut W {
        self.variant(HIEW::HIE_0)
    }
    #[doc = "Enable HOME interrupts"]
    #[inline]
    pub fn hie_1(self) -> &'a mut W {
        self.variant(HIEW::HIE_1)
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
#[doc = "Values that can be written to the field `HIRQ`"]
pub enum HIRQW {
    #[doc = "No interrupt"]
    HIRQ_0,
    #[doc = "HOME signal transition interrupt request"]
    HIRQ_1,
}
impl HIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIRQW::HIRQ_0 => false,
            HIRQW::HIRQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _HIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn hirq_0(self) -> &'a mut W {
        self.variant(HIRQW::HIRQ_0)
    }
    #[doc = "HOME signal transition interrupt request"]
    #[inline]
    pub fn hirq_1(self) -> &'a mut W {
        self.variant(HIRQW::HIRQ_1)
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
    #[doc = "Bit 0 - Compare Interrupt Enable"]
    #[inline]
    pub fn cmpie(&self) -> CMPIER {
        CMPIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Compare Interrupt Request"]
    #[inline]
    pub fn cmpirq(&self) -> CMPIRQR {
        CMPIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Watchdog Enable"]
    #[inline]
    pub fn wde(&self) -> WDER {
        WDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Watchdog Timeout Interrupt Enable"]
    #[inline]
    pub fn die(&self) -> DIER {
        DIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Watchdog Timeout Interrupt Request"]
    #[inline]
    pub fn dirq(&self) -> DIRQR {
        DIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Use Negative Edge of INDEX Pulse"]
    #[inline]
    pub fn xne(&self) -> XNER {
        XNER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline]
    pub fn xip(&self) -> XIPR {
        XIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - INDEX Pulse Interrupt Enable"]
    #[inline]
    pub fn xie(&self) -> XIER {
        XIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - INDEX Pulse Interrupt Request"]
    #[inline]
    pub fn xirq(&self) -> XIRQR {
        XIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Enable Signal Phase Count Mode"]
    #[inline]
    pub fn ph1(&self) -> PH1R {
        PH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Enable Reverse Direction Counting"]
    #[inline]
    pub fn rev(&self) -> REVR {
        REVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Use Negative Edge of HOME Input"]
    #[inline]
    pub fn hne(&self) -> HNER {
        HNER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[inline]
    pub fn hip(&self) -> HIPR {
        HIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - HOME Interrupt Enable"]
    #[inline]
    pub fn hie(&self) -> HIER {
        HIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - HOME Signal Transition Interrupt Request"]
    #[inline]
    pub fn hirq(&self) -> HIRQR {
        HIRQR::_from({
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
    #[doc = "Bit 0 - Compare Interrupt Enable"]
    #[inline]
    pub fn cmpie(&mut self) -> _CMPIEW {
        _CMPIEW { w: self }
    }
    #[doc = "Bit 1 - Compare Interrupt Request"]
    #[inline]
    pub fn cmpirq(&mut self) -> _CMPIRQW {
        _CMPIRQW { w: self }
    }
    #[doc = "Bit 2 - Watchdog Enable"]
    #[inline]
    pub fn wde(&mut self) -> _WDEW {
        _WDEW { w: self }
    }
    #[doc = "Bit 3 - Watchdog Timeout Interrupt Enable"]
    #[inline]
    pub fn die(&mut self) -> _DIEW {
        _DIEW { w: self }
    }
    #[doc = "Bit 4 - Watchdog Timeout Interrupt Request"]
    #[inline]
    pub fn dirq(&mut self) -> _DIRQW {
        _DIRQW { w: self }
    }
    #[doc = "Bit 5 - Use Negative Edge of INDEX Pulse"]
    #[inline]
    pub fn xne(&mut self) -> _XNEW {
        _XNEW { w: self }
    }
    #[doc = "Bit 6 - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline]
    pub fn xip(&mut self) -> _XIPW {
        _XIPW { w: self }
    }
    #[doc = "Bit 7 - INDEX Pulse Interrupt Enable"]
    #[inline]
    pub fn xie(&mut self) -> _XIEW {
        _XIEW { w: self }
    }
    #[doc = "Bit 8 - INDEX Pulse Interrupt Request"]
    #[inline]
    pub fn xirq(&mut self) -> _XIRQW {
        _XIRQW { w: self }
    }
    #[doc = "Bit 9 - Enable Signal Phase Count Mode"]
    #[inline]
    pub fn ph1(&mut self) -> _PH1W {
        _PH1W { w: self }
    }
    #[doc = "Bit 10 - Enable Reverse Direction Counting"]
    #[inline]
    pub fn rev(&mut self) -> _REVW {
        _REVW { w: self }
    }
    #[doc = "Bit 11 - Software Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline]
    pub fn swip(&mut self) -> _SWIPW {
        _SWIPW { w: self }
    }
    #[doc = "Bit 12 - Use Negative Edge of HOME Input"]
    #[inline]
    pub fn hne(&mut self) -> _HNEW {
        _HNEW { w: self }
    }
    #[doc = "Bit 13 - Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[inline]
    pub fn hip(&mut self) -> _HIPW {
        _HIPW { w: self }
    }
    #[doc = "Bit 14 - HOME Interrupt Enable"]
    #[inline]
    pub fn hie(&mut self) -> _HIEW {
        _HIEW { w: self }
    }
    #[doc = "Bit 15 - HOME Signal Transition Interrupt Request"]
    #[inline]
    pub fn hirq(&mut self) -> _HIRQW {
        _HIRQW { w: self }
    }
}
