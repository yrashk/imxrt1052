#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLPCR {
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
#[doc = "Possible values of the field `LPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMR {
    #[doc = "Remain in run mode"]
    LPM_0,
    #[doc = "Transfer to wait mode"]
    LPM_1,
    #[doc = "Transfer to stop mode"]
    LPM_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPMR::LPM_0 => 0,
            LPMR::LPM_1 => 1,
            LPMR::LPM_2 => 2,
            LPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPMR {
        match value {
            0 => LPMR::LPM_0,
            1 => LPMR::LPM_1,
            2 => LPMR::LPM_2,
            i => LPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPM_0`"]
    #[inline]
    pub fn is_lpm_0(&self) -> bool {
        *self == LPMR::LPM_0
    }
    #[doc = "Checks if the value of the field is `LPM_1`"]
    #[inline]
    pub fn is_lpm_1(&self) -> bool {
        *self == LPMR::LPM_1
    }
    #[doc = "Checks if the value of the field is `LPM_2`"]
    #[inline]
    pub fn is_lpm_2(&self) -> bool {
        *self == LPMR::LPM_2
    }
}
#[doc = "Possible values of the field `ARM_CLK_DIS_ON_LPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_CLK_DIS_ON_LPMR {
    #[doc = "ARM clock enabled on wait mode."]
    ARM_CLK_DIS_ON_LPM_0,
    #[doc = "ARM clock disabled on wait mode. ."]
    ARM_CLK_DIS_ON_LPM_1,
}
impl ARM_CLK_DIS_ON_LPMR {
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
            ARM_CLK_DIS_ON_LPMR::ARM_CLK_DIS_ON_LPM_0 => false,
            ARM_CLK_DIS_ON_LPMR::ARM_CLK_DIS_ON_LPM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARM_CLK_DIS_ON_LPMR {
        match value {
            false => ARM_CLK_DIS_ON_LPMR::ARM_CLK_DIS_ON_LPM_0,
            true => ARM_CLK_DIS_ON_LPMR::ARM_CLK_DIS_ON_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_CLK_DIS_ON_LPM_0`"]
    #[inline]
    pub fn is_arm_clk_dis_on_lpm_0(&self) -> bool {
        *self == ARM_CLK_DIS_ON_LPMR::ARM_CLK_DIS_ON_LPM_0
    }
    #[doc = "Checks if the value of the field is `ARM_CLK_DIS_ON_LPM_1`"]
    #[inline]
    pub fn is_arm_clk_dis_on_lpm_1(&self) -> bool {
        *self == ARM_CLK_DIS_ON_LPMR::ARM_CLK_DIS_ON_LPM_1
    }
}
#[doc = "Possible values of the field `SBYOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBYOSR {
    #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    SBYOS_0,
    #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    SBYOS_1,
}
impl SBYOSR {
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
            SBYOSR::SBYOS_0 => false,
            SBYOSR::SBYOS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBYOSR {
        match value {
            false => SBYOSR::SBYOS_0,
            true => SBYOSR::SBYOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBYOS_0`"]
    #[inline]
    pub fn is_sbyos_0(&self) -> bool {
        *self == SBYOSR::SBYOS_0
    }
    #[doc = "Checks if the value of the field is `SBYOS_1`"]
    #[inline]
    pub fn is_sbyos_1(&self) -> bool {
        *self == SBYOSR::SBYOS_1
    }
}
#[doc = "Possible values of the field `DIS_REF_OSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_REF_OSCR {
    #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    DIS_REF_OSC_0,
    #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    DIS_REF_OSC_1,
}
impl DIS_REF_OSCR {
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
            DIS_REF_OSCR::DIS_REF_OSC_0 => false,
            DIS_REF_OSCR::DIS_REF_OSC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIS_REF_OSCR {
        match value {
            false => DIS_REF_OSCR::DIS_REF_OSC_0,
            true => DIS_REF_OSCR::DIS_REF_OSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_REF_OSC_0`"]
    #[inline]
    pub fn is_dis_ref_osc_0(&self) -> bool {
        *self == DIS_REF_OSCR::DIS_REF_OSC_0
    }
    #[doc = "Checks if the value of the field is `DIS_REF_OSC_1`"]
    #[inline]
    pub fn is_dis_ref_osc_1(&self) -> bool {
        *self == DIS_REF_OSCR::DIS_REF_OSC_1
    }
}
#[doc = "Possible values of the field `VSTBY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSTBYR {
    #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    VSTBY_0,
    #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    VSTBY_1,
}
impl VSTBYR {
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
            VSTBYR::VSTBY_0 => false,
            VSTBYR::VSTBY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VSTBYR {
        match value {
            false => VSTBYR::VSTBY_0,
            true => VSTBYR::VSTBY_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSTBY_0`"]
    #[inline]
    pub fn is_vstby_0(&self) -> bool {
        *self == VSTBYR::VSTBY_0
    }
    #[doc = "Checks if the value of the field is `VSTBY_1`"]
    #[inline]
    pub fn is_vstby_1(&self) -> bool {
        *self == VSTBYR::VSTBY_1
    }
}
#[doc = "Possible values of the field `STBY_COUNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBY_COUNTR {
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_0,
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_1,
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_2,
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_3,
}
impl STBY_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STBY_COUNTR::STBY_COUNT_0 => 0,
            STBY_COUNTR::STBY_COUNT_1 => 1,
            STBY_COUNTR::STBY_COUNT_2 => 2,
            STBY_COUNTR::STBY_COUNT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STBY_COUNTR {
        match value {
            0 => STBY_COUNTR::STBY_COUNT_0,
            1 => STBY_COUNTR::STBY_COUNT_1,
            2 => STBY_COUNTR::STBY_COUNT_2,
            3 => STBY_COUNTR::STBY_COUNT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_0`"]
    #[inline]
    pub fn is_stby_count_0(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_0
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_1`"]
    #[inline]
    pub fn is_stby_count_1(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_1
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_2`"]
    #[inline]
    pub fn is_stby_count_2(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_2
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_3`"]
    #[inline]
    pub fn is_stby_count_3(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_3
    }
}
#[doc = "Possible values of the field `COSC_PWRDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_PWRDOWNR {
    #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    COSC_PWRDOWN_0,
    #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    COSC_PWRDOWN_1,
}
impl COSC_PWRDOWNR {
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
            COSC_PWRDOWNR::COSC_PWRDOWN_0 => false,
            COSC_PWRDOWNR::COSC_PWRDOWN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSC_PWRDOWNR {
        match value {
            false => COSC_PWRDOWNR::COSC_PWRDOWN_0,
            true => COSC_PWRDOWNR::COSC_PWRDOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_0`"]
    #[inline]
    pub fn is_cosc_pwrdown_0(&self) -> bool {
        *self == COSC_PWRDOWNR::COSC_PWRDOWN_0
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_1`"]
    #[inline]
    pub fn is_cosc_pwrdown_1(&self) -> bool {
        *self == COSC_PWRDOWNR::COSC_PWRDOWN_1
    }
}
#[doc = r" Value of the field"]
pub struct BYPASS_LPM_HS1R {
    bits: bool,
}
impl BYPASS_LPM_HS1R {
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
pub struct BYPASS_LPM_HS0R {
    bits: bool,
}
impl BYPASS_LPM_HS0R {
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
#[doc = "Possible values of the field `MASK_CORE0_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_CORE0_WFIR {
    #[doc = "WFI of core0 is not masked"]
    MASK_CORE0_WFI_0,
    #[doc = "WFI of core0 is masked"]
    MASK_CORE0_WFI_1,
}
impl MASK_CORE0_WFIR {
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
            MASK_CORE0_WFIR::MASK_CORE0_WFI_0 => false,
            MASK_CORE0_WFIR::MASK_CORE0_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_CORE0_WFIR {
        match value {
            false => MASK_CORE0_WFIR::MASK_CORE0_WFI_0,
            true => MASK_CORE0_WFIR::MASK_CORE0_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_0`"]
    #[inline]
    pub fn is_mask_core0_wfi_0(&self) -> bool {
        *self == MASK_CORE0_WFIR::MASK_CORE0_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_1`"]
    #[inline]
    pub fn is_mask_core0_wfi_1(&self) -> bool {
        *self == MASK_CORE0_WFIR::MASK_CORE0_WFI_1
    }
}
#[doc = "Possible values of the field `MASK_SCU_IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_SCU_IDLER {
    #[doc = "SCU IDLE is not masked"]
    MASK_SCU_IDLE_0,
    #[doc = "SCU IDLE is masked"]
    MASK_SCU_IDLE_1,
}
impl MASK_SCU_IDLER {
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
            MASK_SCU_IDLER::MASK_SCU_IDLE_0 => false,
            MASK_SCU_IDLER::MASK_SCU_IDLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_SCU_IDLER {
        match value {
            false => MASK_SCU_IDLER::MASK_SCU_IDLE_0,
            true => MASK_SCU_IDLER::MASK_SCU_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_IDLE_0`"]
    #[inline]
    pub fn is_mask_scu_idle_0(&self) -> bool {
        *self == MASK_SCU_IDLER::MASK_SCU_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_IDLE_1`"]
    #[inline]
    pub fn is_mask_scu_idle_1(&self) -> bool {
        *self == MASK_SCU_IDLER::MASK_SCU_IDLE_1
    }
}
#[doc = "Possible values of the field `MASK_L2CC_IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_L2CC_IDLER {
    #[doc = "L2CC IDLE is not masked"]
    MASK_L2CC_IDLE_0,
    #[doc = "L2CC IDLE is masked"]
    MASK_L2CC_IDLE_1,
}
impl MASK_L2CC_IDLER {
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
            MASK_L2CC_IDLER::MASK_L2CC_IDLE_0 => false,
            MASK_L2CC_IDLER::MASK_L2CC_IDLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_L2CC_IDLER {
        match value {
            false => MASK_L2CC_IDLER::MASK_L2CC_IDLE_0,
            true => MASK_L2CC_IDLER::MASK_L2CC_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_IDLE_0`"]
    #[inline]
    pub fn is_mask_l2cc_idle_0(&self) -> bool {
        *self == MASK_L2CC_IDLER::MASK_L2CC_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_IDLE_1`"]
    #[inline]
    pub fn is_mask_l2cc_idle_1(&self) -> bool {
        *self == MASK_L2CC_IDLER::MASK_L2CC_IDLE_1
    }
}
#[doc = "Values that can be written to the field `LPM`"]
pub enum LPMW {
    #[doc = "Remain in run mode"]
    LPM_0,
    #[doc = "Transfer to wait mode"]
    LPM_1,
    #[doc = "Transfer to stop mode"]
    LPM_2,
}
impl LPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPMW::LPM_0 => 0,
            LPMW::LPM_1 => 1,
            LPMW::LPM_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Remain in run mode"]
    #[inline]
    pub fn lpm_0(self) -> &'a mut W {
        self.variant(LPMW::LPM_0)
    }
    #[doc = "Transfer to wait mode"]
    #[inline]
    pub fn lpm_1(self) -> &'a mut W {
        self.variant(LPMW::LPM_1)
    }
    #[doc = "Transfer to stop mode"]
    #[inline]
    pub fn lpm_2(self) -> &'a mut W {
        self.variant(LPMW::LPM_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARM_CLK_DIS_ON_LPM`"]
pub enum ARM_CLK_DIS_ON_LPMW {
    #[doc = "ARM clock enabled on wait mode."]
    ARM_CLK_DIS_ON_LPM_0,
    #[doc = "ARM clock disabled on wait mode. ."]
    ARM_CLK_DIS_ON_LPM_1,
}
impl ARM_CLK_DIS_ON_LPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARM_CLK_DIS_ON_LPMW::ARM_CLK_DIS_ON_LPM_0 => false,
            ARM_CLK_DIS_ON_LPMW::ARM_CLK_DIS_ON_LPM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARM_CLK_DIS_ON_LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _ARM_CLK_DIS_ON_LPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARM_CLK_DIS_ON_LPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ARM clock enabled on wait mode."]
    #[inline]
    pub fn arm_clk_dis_on_lpm_0(self) -> &'a mut W {
        self.variant(ARM_CLK_DIS_ON_LPMW::ARM_CLK_DIS_ON_LPM_0)
    }
    #[doc = "ARM clock disabled on wait mode. ."]
    #[inline]
    pub fn arm_clk_dis_on_lpm_1(self) -> &'a mut W {
        self.variant(ARM_CLK_DIS_ON_LPMW::ARM_CLK_DIS_ON_LPM_1)
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
#[doc = "Values that can be written to the field `SBYOS`"]
pub enum SBYOSW {
    #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    SBYOS_0,
    #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    SBYOS_1,
}
impl SBYOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBYOSW::SBYOS_0 => false,
            SBYOSW::SBYOS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBYOSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBYOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBYOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    #[inline]
    pub fn sbyos_0(self) -> &'a mut W {
        self.variant(SBYOSW::SBYOS_0)
    }
    #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    #[inline]
    pub fn sbyos_1(self) -> &'a mut W {
        self.variant(SBYOSW::SBYOS_1)
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
#[doc = "Values that can be written to the field `DIS_REF_OSC`"]
pub enum DIS_REF_OSCW {
    #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    DIS_REF_OSC_0,
    #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    DIS_REF_OSC_1,
}
impl DIS_REF_OSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIS_REF_OSCW::DIS_REF_OSC_0 => false,
            DIS_REF_OSCW::DIS_REF_OSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIS_REF_OSCW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_REF_OSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIS_REF_OSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    #[inline]
    pub fn dis_ref_osc_0(self) -> &'a mut W {
        self.variant(DIS_REF_OSCW::DIS_REF_OSC_0)
    }
    #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    #[inline]
    pub fn dis_ref_osc_1(self) -> &'a mut W {
        self.variant(DIS_REF_OSCW::DIS_REF_OSC_1)
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
#[doc = "Values that can be written to the field `VSTBY`"]
pub enum VSTBYW {
    #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    VSTBY_0,
    #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    VSTBY_1,
}
impl VSTBYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VSTBYW::VSTBY_0 => false,
            VSTBYW::VSTBY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VSTBYW<'a> {
    w: &'a mut W,
}
impl<'a> _VSTBYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VSTBYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    #[inline]
    pub fn vstby_0(self) -> &'a mut W {
        self.variant(VSTBYW::VSTBY_0)
    }
    #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    #[inline]
    pub fn vstby_1(self) -> &'a mut W {
        self.variant(VSTBYW::VSTBY_1)
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
#[doc = "Values that can be written to the field `STBY_COUNT`"]
pub enum STBY_COUNTW {
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_0,
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_1,
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_2,
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_3,
}
impl STBY_COUNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STBY_COUNTW::STBY_COUNT_0 => 0,
            STBY_COUNTW::STBY_COUNT_1 => 1,
            STBY_COUNTW::STBY_COUNT_2 => 2,
            STBY_COUNTW::STBY_COUNT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBY_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _STBY_COUNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBY_COUNTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline]
    pub fn stby_count_0(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_0)
    }
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline]
    pub fn stby_count_1(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_1)
    }
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline]
    pub fn stby_count_2(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_2)
    }
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline]
    pub fn stby_count_3(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COSC_PWRDOWN`"]
pub enum COSC_PWRDOWNW {
    #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    COSC_PWRDOWN_0,
    #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    COSC_PWRDOWN_1,
}
impl COSC_PWRDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSC_PWRDOWNW::COSC_PWRDOWN_0 => false,
            COSC_PWRDOWNW::COSC_PWRDOWN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSC_PWRDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _COSC_PWRDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSC_PWRDOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    #[inline]
    pub fn cosc_pwrdown_0(self) -> &'a mut W {
        self.variant(COSC_PWRDOWNW::COSC_PWRDOWN_0)
    }
    #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    #[inline]
    pub fn cosc_pwrdown_1(self) -> &'a mut W {
        self.variant(COSC_PWRDOWNW::COSC_PWRDOWN_1)
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
pub struct _BYPASS_LPM_HS1W<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_LPM_HS1W<'a> {
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
#[doc = r" Proxy"]
pub struct _BYPASS_LPM_HS0W<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_LPM_HS0W<'a> {
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
#[doc = "Values that can be written to the field `MASK_CORE0_WFI`"]
pub enum MASK_CORE0_WFIW {
    #[doc = "WFI of core0 is not masked"]
    MASK_CORE0_WFI_0,
    #[doc = "WFI of core0 is masked"]
    MASK_CORE0_WFI_1,
}
impl MASK_CORE0_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_CORE0_WFIW::MASK_CORE0_WFI_0 => false,
            MASK_CORE0_WFIW::MASK_CORE0_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_CORE0_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_CORE0_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_CORE0_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI of core0 is not masked"]
    #[inline]
    pub fn mask_core0_wfi_0(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFIW::MASK_CORE0_WFI_0)
    }
    #[doc = "WFI of core0 is masked"]
    #[inline]
    pub fn mask_core0_wfi_1(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFIW::MASK_CORE0_WFI_1)
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
#[doc = "Values that can be written to the field `MASK_SCU_IDLE`"]
pub enum MASK_SCU_IDLEW {
    #[doc = "SCU IDLE is not masked"]
    MASK_SCU_IDLE_0,
    #[doc = "SCU IDLE is masked"]
    MASK_SCU_IDLE_1,
}
impl MASK_SCU_IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_SCU_IDLEW::MASK_SCU_IDLE_0 => false,
            MASK_SCU_IDLEW::MASK_SCU_IDLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_SCU_IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_SCU_IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_SCU_IDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCU IDLE is not masked"]
    #[inline]
    pub fn mask_scu_idle_0(self) -> &'a mut W {
        self.variant(MASK_SCU_IDLEW::MASK_SCU_IDLE_0)
    }
    #[doc = "SCU IDLE is masked"]
    #[inline]
    pub fn mask_scu_idle_1(self) -> &'a mut W {
        self.variant(MASK_SCU_IDLEW::MASK_SCU_IDLE_1)
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
#[doc = "Values that can be written to the field `MASK_L2CC_IDLE`"]
pub enum MASK_L2CC_IDLEW {
    #[doc = "L2CC IDLE is not masked"]
    MASK_L2CC_IDLE_0,
    #[doc = "L2CC IDLE is masked"]
    MASK_L2CC_IDLE_1,
}
impl MASK_L2CC_IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_L2CC_IDLEW::MASK_L2CC_IDLE_0 => false,
            MASK_L2CC_IDLEW::MASK_L2CC_IDLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_L2CC_IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_L2CC_IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_L2CC_IDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "L2CC IDLE is not masked"]
    #[inline]
    pub fn mask_l2cc_idle_0(self) -> &'a mut W {
        self.variant(MASK_L2CC_IDLEW::MASK_L2CC_IDLE_0)
    }
    #[doc = "L2CC IDLE is masked"]
    #[inline]
    pub fn mask_l2cc_idle_1(self) -> &'a mut W {
        self.variant(MASK_L2CC_IDLEW::MASK_L2CC_IDLE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm(&self) -> LPMR {
        LPMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Define if ARM clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline]
    pub fn arm_clk_dis_on_lpm(&self) -> ARM_CLK_DIS_ON_LPMR {
        ARM_CLK_DIS_ON_LPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Standby clock oscillator bit"]
    #[inline]
    pub fn sbyos(&self) -> SBYOSR {
        SBYOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline]
    pub fn dis_ref_osc(&self) -> DIS_REF_OSCR {
        DIS_REF_OSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Voltage standby request bit"]
    #[inline]
    pub fn vstby(&self) -> VSTBYR {
        VSTBYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - Standby counter definition"]
    #[inline]
    pub fn stby_count(&self) -> STBY_COUNTR {
        STBY_COUNTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline]
    pub fn cosc_pwrdown(&self) -> COSC_PWRDOWNR {
        COSC_PWRDOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline]
    pub fn bypass_lpm_hs1(&self) -> BYPASS_LPM_HS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASS_LPM_HS1R { bits }
    }
    #[doc = "Bit 21 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline]
    pub fn bypass_lpm_hs0(&self) -> BYPASS_LPM_HS0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASS_LPM_HS0R { bits }
    }
    #[doc = "Bit 22 - Mask WFI of core0 for entering low power mode Assertion of all bits[27:22] will generate low power mode request"]
    #[inline]
    pub fn mask_core0_wfi(&self) -> MASK_CORE0_WFIR {
        MASK_CORE0_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Mask SCU IDLE for entering low power mode Assertion of all bits[27:22] will generate low power mode request"]
    #[inline]
    pub fn mask_scu_idle(&self) -> MASK_SCU_IDLER {
        MASK_SCU_IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Mask L2CC IDLE for entering low power mode"]
    #[inline]
    pub fn mask_l2cc_idle(&self) -> MASK_L2CC_IDLER {
        MASK_L2CC_IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 121 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm(&mut self) -> _LPMW {
        _LPMW { w: self }
    }
    #[doc = "Bit 5 - Define if ARM clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline]
    pub fn arm_clk_dis_on_lpm(&mut self) -> _ARM_CLK_DIS_ON_LPMW {
        _ARM_CLK_DIS_ON_LPMW { w: self }
    }
    #[doc = "Bit 6 - Standby clock oscillator bit"]
    #[inline]
    pub fn sbyos(&mut self) -> _SBYOSW {
        _SBYOSW { w: self }
    }
    #[doc = "Bit 7 - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline]
    pub fn dis_ref_osc(&mut self) -> _DIS_REF_OSCW {
        _DIS_REF_OSCW { w: self }
    }
    #[doc = "Bit 8 - Voltage standby request bit"]
    #[inline]
    pub fn vstby(&mut self) -> _VSTBYW {
        _VSTBYW { w: self }
    }
    #[doc = "Bits 9:10 - Standby counter definition"]
    #[inline]
    pub fn stby_count(&mut self) -> _STBY_COUNTW {
        _STBY_COUNTW { w: self }
    }
    #[doc = "Bit 11 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline]
    pub fn cosc_pwrdown(&mut self) -> _COSC_PWRDOWNW {
        _COSC_PWRDOWNW { w: self }
    }
    #[doc = "Bit 19 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline]
    pub fn bypass_lpm_hs1(&mut self) -> _BYPASS_LPM_HS1W {
        _BYPASS_LPM_HS1W { w: self }
    }
    #[doc = "Bit 21 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline]
    pub fn bypass_lpm_hs0(&mut self) -> _BYPASS_LPM_HS0W {
        _BYPASS_LPM_HS0W { w: self }
    }
    #[doc = "Bit 22 - Mask WFI of core0 for entering low power mode Assertion of all bits[27:22] will generate low power mode request"]
    #[inline]
    pub fn mask_core0_wfi(&mut self) -> _MASK_CORE0_WFIW {
        _MASK_CORE0_WFIW { w: self }
    }
    #[doc = "Bit 26 - Mask SCU IDLE for entering low power mode Assertion of all bits[27:22] will generate low power mode request"]
    #[inline]
    pub fn mask_scu_idle(&mut self) -> _MASK_SCU_IDLEW {
        _MASK_SCU_IDLEW { w: self }
    }
    #[doc = "Bit 27 - Mask L2CC IDLE for entering low power mode"]
    #[inline]
    pub fn mask_l2cc_idle(&mut self) -> _MASK_L2CC_IDLEW {
        _MASK_L2CC_IDLEW { w: self }
    }
}
