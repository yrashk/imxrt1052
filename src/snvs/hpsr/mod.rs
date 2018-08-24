#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPSR {
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
#[doc = "Possible values of the field `HPTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPTAR {
    #[doc = "No time alarm interrupt occurred."]
    HPTA_0,
    #[doc = "A time alarm interrupt occurred."]
    HPTA_1,
}
impl HPTAR {
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
            HPTAR::HPTA_0 => false,
            HPTAR::HPTA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPTAR {
        match value {
            false => HPTAR::HPTA_0,
            true => HPTAR::HPTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPTA_0`"]
    #[inline]
    pub fn is_hpta_0(&self) -> bool {
        *self == HPTAR::HPTA_0
    }
    #[doc = "Checks if the value of the field is `HPTA_1`"]
    #[inline]
    pub fn is_hpta_1(&self) -> bool {
        *self == HPTAR::HPTA_1
    }
}
#[doc = "Possible values of the field `PI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIR {
    #[doc = "No periodic interrupt occurred."]
    PI_0,
    #[doc = "A periodic interrupt occurred."]
    PI_1,
}
impl PIR {
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
            PIR::PI_0 => false,
            PIR::PI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIR {
        match value {
            false => PIR::PI_0,
            true => PIR::PI_1,
        }
    }
    #[doc = "Checks if the value of the field is `PI_0`"]
    #[inline]
    pub fn is_pi_0(&self) -> bool {
        *self == PIR::PI_0
    }
    #[doc = "Checks if the value of the field is `PI_1`"]
    #[inline]
    pub fn is_pi_1(&self) -> bool {
        *self == PIR::PI_1
    }
}
#[doc = r" Value of the field"]
pub struct LPDISR {
    bits: bool,
}
impl LPDISR {
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
pub struct BTNR {
    bits: bool,
}
impl BTNR {
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
pub struct BIR {
    bits: bool,
}
impl BIR {
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
#[doc = "Possible values of the field `SSM_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_STATER {
    #[doc = "Init"]
    SSM_STATE_0,
    #[doc = "Hard Fail"]
    SSM_STATE_1,
    #[doc = "Soft Fail"]
    SSM_STATE_3,
    #[doc = "Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)"]
    SSM_STATE_8,
    #[doc = "Check"]
    SSM_STATE_9,
    #[doc = "Non-Secure"]
    SSM_STATE_11,
    #[doc = "Trusted"]
    SSM_STATE_13,
    #[doc = "Secure"]
    SSM_STATE_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSM_STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSM_STATER::SSM_STATE_0 => 0,
            SSM_STATER::SSM_STATE_1 => 1,
            SSM_STATER::SSM_STATE_3 => 3,
            SSM_STATER::SSM_STATE_8 => 8,
            SSM_STATER::SSM_STATE_9 => 9,
            SSM_STATER::SSM_STATE_11 => 11,
            SSM_STATER::SSM_STATE_13 => 13,
            SSM_STATER::SSM_STATE_15 => 15,
            SSM_STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSM_STATER {
        match value {
            0 => SSM_STATER::SSM_STATE_0,
            1 => SSM_STATER::SSM_STATE_1,
            3 => SSM_STATER::SSM_STATE_3,
            8 => SSM_STATER::SSM_STATE_8,
            9 => SSM_STATER::SSM_STATE_9,
            11 => SSM_STATER::SSM_STATE_11,
            13 => SSM_STATER::SSM_STATE_13,
            15 => SSM_STATER::SSM_STATE_15,
            i => SSM_STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_0`"]
    #[inline]
    pub fn is_ssm_state_0(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_0
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_1`"]
    #[inline]
    pub fn is_ssm_state_1(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_1
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_3`"]
    #[inline]
    pub fn is_ssm_state_3(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_3
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_8`"]
    #[inline]
    pub fn is_ssm_state_8(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_8
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_9`"]
    #[inline]
    pub fn is_ssm_state_9(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_9
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_11`"]
    #[inline]
    pub fn is_ssm_state_11(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_11
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_13`"]
    #[inline]
    pub fn is_ssm_state_13(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_13
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_15`"]
    #[inline]
    pub fn is_ssm_state_15(&self) -> bool {
        *self == SSM_STATER::SSM_STATE_15
    }
}
#[doc = "Possible values of the field `SYS_SECURITY_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_SECURITY_CFGR {
    #[doc = "Fab Configuration - the default configuration of newly fabricated chips"]
    SYS_SECURITY_CFG_0,
    #[doc = "Open Configuration - the configuration after NXP-programmable fuses have been blown"]
    SYS_SECURITY_CFG_1,
    #[doc = "Closed Configuration - the configuration after OEM-programmable fuses have been blown"]
    SYS_SECURITY_CFG_3,
    #[doc = "Field Return Configuration - the configuration of chips that are returned to NXP for analysis"]
    SYS_SECURITY_CFG_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYS_SECURITY_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYS_SECURITY_CFGR::SYS_SECURITY_CFG_0 => 0,
            SYS_SECURITY_CFGR::SYS_SECURITY_CFG_1 => 1,
            SYS_SECURITY_CFGR::SYS_SECURITY_CFG_3 => 3,
            SYS_SECURITY_CFGR::SYS_SECURITY_CFG_7 => 7,
            SYS_SECURITY_CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYS_SECURITY_CFGR {
        match value {
            0 => SYS_SECURITY_CFGR::SYS_SECURITY_CFG_0,
            1 => SYS_SECURITY_CFGR::SYS_SECURITY_CFG_1,
            3 => SYS_SECURITY_CFGR::SYS_SECURITY_CFG_3,
            7 => SYS_SECURITY_CFGR::SYS_SECURITY_CFG_7,
            i => SYS_SECURITY_CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYS_SECURITY_CFG_0`"]
    #[inline]
    pub fn is_sys_security_cfg_0(&self) -> bool {
        *self == SYS_SECURITY_CFGR::SYS_SECURITY_CFG_0
    }
    #[doc = "Checks if the value of the field is `SYS_SECURITY_CFG_1`"]
    #[inline]
    pub fn is_sys_security_cfg_1(&self) -> bool {
        *self == SYS_SECURITY_CFGR::SYS_SECURITY_CFG_1
    }
    #[doc = "Checks if the value of the field is `SYS_SECURITY_CFG_3`"]
    #[inline]
    pub fn is_sys_security_cfg_3(&self) -> bool {
        *self == SYS_SECURITY_CFGR::SYS_SECURITY_CFG_3
    }
    #[doc = "Checks if the value of the field is `SYS_SECURITY_CFG_7`"]
    #[inline]
    pub fn is_sys_security_cfg_7(&self) -> bool {
        *self == SYS_SECURITY_CFGR::SYS_SECURITY_CFG_7
    }
}
#[doc = r" Value of the field"]
pub struct SYS_SECURE_BOOTR {
    bits: bool,
}
impl SYS_SECURE_BOOTR {
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
pub struct OTPMK_SYNDROMER {
    bits: u16,
}
impl OTPMK_SYNDROMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `OTPMK_ZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTPMK_ZEROR {
    #[doc = "The OTPMK is not zero."]
    OTPMK_ZERO_0,
    #[doc = "The OTPMK is zero."]
    OTPMK_ZERO_1,
}
impl OTPMK_ZEROR {
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
            OTPMK_ZEROR::OTPMK_ZERO_0 => false,
            OTPMK_ZEROR::OTPMK_ZERO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTPMK_ZEROR {
        match value {
            false => OTPMK_ZEROR::OTPMK_ZERO_0,
            true => OTPMK_ZEROR::OTPMK_ZERO_1,
        }
    }
    #[doc = "Checks if the value of the field is `OTPMK_ZERO_0`"]
    #[inline]
    pub fn is_otpmk_zero_0(&self) -> bool {
        *self == OTPMK_ZEROR::OTPMK_ZERO_0
    }
    #[doc = "Checks if the value of the field is `OTPMK_ZERO_1`"]
    #[inline]
    pub fn is_otpmk_zero_1(&self) -> bool {
        *self == OTPMK_ZEROR::OTPMK_ZERO_1
    }
}
#[doc = "Possible values of the field `ZMK_ZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_ZEROR {
    #[doc = "The ZMK is not zero."]
    ZMK_ZERO_0,
    #[doc = "The ZMK is zero."]
    ZMK_ZERO_1,
}
impl ZMK_ZEROR {
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
            ZMK_ZEROR::ZMK_ZERO_0 => false,
            ZMK_ZEROR::ZMK_ZERO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZMK_ZEROR {
        match value {
            false => ZMK_ZEROR::ZMK_ZERO_0,
            true => ZMK_ZEROR::ZMK_ZERO_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_ZERO_0`"]
    #[inline]
    pub fn is_zmk_zero_0(&self) -> bool {
        *self == ZMK_ZEROR::ZMK_ZERO_0
    }
    #[doc = "Checks if the value of the field is `ZMK_ZERO_1`"]
    #[inline]
    pub fn is_zmk_zero_1(&self) -> bool {
        *self == ZMK_ZEROR::ZMK_ZERO_1
    }
}
#[doc = "Values that can be written to the field `HPTA`"]
pub enum HPTAW {
    #[doc = "No time alarm interrupt occurred."]
    HPTA_0,
    #[doc = "A time alarm interrupt occurred."]
    HPTA_1,
}
impl HPTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPTAW::HPTA_0 => false,
            HPTAW::HPTA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPTAW<'a> {
    w: &'a mut W,
}
impl<'a> _HPTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time alarm interrupt occurred."]
    #[inline]
    pub fn hpta_0(self) -> &'a mut W {
        self.variant(HPTAW::HPTA_0)
    }
    #[doc = "A time alarm interrupt occurred."]
    #[inline]
    pub fn hpta_1(self) -> &'a mut W {
        self.variant(HPTAW::HPTA_1)
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
#[doc = "Values that can be written to the field `PI`"]
pub enum PIW {
    #[doc = "No periodic interrupt occurred."]
    PI_0,
    #[doc = "A periodic interrupt occurred."]
    PI_1,
}
impl PIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIW::PI_0 => false,
            PIW::PI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIW<'a> {
    w: &'a mut W,
}
impl<'a> _PIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No periodic interrupt occurred."]
    #[inline]
    pub fn pi_0(self) -> &'a mut W {
        self.variant(PIW::PI_0)
    }
    #[doc = "A periodic interrupt occurred."]
    #[inline]
    pub fn pi_1(self) -> &'a mut W {
        self.variant(PIW::PI_1)
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
#[doc = r" Proxy"]
pub struct _BIW<'a> {
    w: &'a mut W,
}
impl<'a> _BIW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline]
    pub fn hpta(&self) -> HPTAR {
        HPTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline]
    pub fn pi(&self) -> PIR {
        PIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    #[inline]
    pub fn lpdis(&self) -> LPDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPDISR { bits }
    }
    #[doc = "Bit 6 - Button Value of the BTN input"]
    #[inline]
    pub fn btn(&self) -> BTNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BTNR { bits }
    }
    #[doc = "Bit 7 - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline]
    pub fn bi(&self) -> BIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BIR { bits }
    }
    #[doc = "Bits 8:11 - System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    #[inline]
    pub fn ssm_state(&self) -> SSM_STATER {
        SSM_STATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - System Security Configuration This field indicates the security configuration of SNVS, defined as follows:"]
    #[inline]
    pub fn sys_security_cfg(&self) -> SYS_SECURITY_CFGR {
        SYS_SECURITY_CFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM."]
    #[inline]
    pub fn sys_secure_boot(&self) -> SYS_SECURE_BOOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_SECURE_BOOTR { bits }
    }
    #[doc = "Bits 16:24 - One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    #[inline]
    pub fn otpmk_syndrome(&self) -> OTPMK_SYNDROMER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OTPMK_SYNDROMER { bits }
    }
    #[doc = "Bit 27 - One Time Programmable Master Key is Equal to Zero"]
    #[inline]
    pub fn otpmk_zero(&self) -> OTPMK_ZEROR {
        OTPMK_ZEROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Zeroizable Master Key is Equal to Zero"]
    #[inline]
    pub fn zmk_zero(&self) -> ZMK_ZEROR {
        ZMK_ZEROR::_from({
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
        W { bits: 2147528704 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline]
    pub fn hpta(&mut self) -> _HPTAW {
        _HPTAW { w: self }
    }
    #[doc = "Bit 1 - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline]
    pub fn pi(&mut self) -> _PIW {
        _PIW { w: self }
    }
    #[doc = "Bit 7 - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline]
    pub fn bi(&mut self) -> _BIW {
        _BIW { w: self }
    }
}
