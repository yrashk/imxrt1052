#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR2 {
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
#[doc = "Possible values of the field `L2_MEM_EN_POWERSAVING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L2_MEM_EN_POWERSAVINGR {
    #[doc = "none memory power saving features enabled, SHUTDOWN/DEEPSLEEP/LIGHTSLEEP will have no effect"]
    L2_MEM_EN_POWERSAVING_0,
    #[doc = "memory power saving features enabled, set SHUTDOWN/DEEPSLEEP/LIGHTSLEEP(priority high to low) to enable power saving levels"]
    L2_MEM_EN_POWERSAVING_1,
}
impl L2_MEM_EN_POWERSAVINGR {
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
            L2_MEM_EN_POWERSAVINGR::L2_MEM_EN_POWERSAVING_0 => false,
            L2_MEM_EN_POWERSAVINGR::L2_MEM_EN_POWERSAVING_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L2_MEM_EN_POWERSAVINGR {
        match value {
            false => L2_MEM_EN_POWERSAVINGR::L2_MEM_EN_POWERSAVING_0,
            true => L2_MEM_EN_POWERSAVINGR::L2_MEM_EN_POWERSAVING_1,
        }
    }
    #[doc = "Checks if the value of the field is `L2_MEM_EN_POWERSAVING_0`"]
    #[inline]
    pub fn is_l2_mem_en_powersaving_0(&self) -> bool {
        *self == L2_MEM_EN_POWERSAVINGR::L2_MEM_EN_POWERSAVING_0
    }
    #[doc = "Checks if the value of the field is `L2_MEM_EN_POWERSAVING_1`"]
    #[inline]
    pub fn is_l2_mem_en_powersaving_1(&self) -> bool {
        *self == L2_MEM_EN_POWERSAVINGR::L2_MEM_EN_POWERSAVING_1
    }
}
#[doc = "Possible values of the field `L2_MEM_DEEPSLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L2_MEM_DEEPSLEEPR {
    #[doc = "no force sleep control supported, memory deep sleep mode only entered when whole system in stop mode"]
    L2_MEM_DEEPSLEEP_0,
    #[doc = "force memory into deep sleep mode"]
    L2_MEM_DEEPSLEEP_1,
}
impl L2_MEM_DEEPSLEEPR {
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
            L2_MEM_DEEPSLEEPR::L2_MEM_DEEPSLEEP_0 => false,
            L2_MEM_DEEPSLEEPR::L2_MEM_DEEPSLEEP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L2_MEM_DEEPSLEEPR {
        match value {
            false => L2_MEM_DEEPSLEEPR::L2_MEM_DEEPSLEEP_0,
            true => L2_MEM_DEEPSLEEPR::L2_MEM_DEEPSLEEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L2_MEM_DEEPSLEEP_0`"]
    #[inline]
    pub fn is_l2_mem_deepsleep_0(&self) -> bool {
        *self == L2_MEM_DEEPSLEEPR::L2_MEM_DEEPSLEEP_0
    }
    #[doc = "Checks if the value of the field is `L2_MEM_DEEPSLEEP_1`"]
    #[inline]
    pub fn is_l2_mem_deepsleep_1(&self) -> bool {
        *self == L2_MEM_DEEPSLEEPR::L2_MEM_DEEPSLEEP_1
    }
}
#[doc = "Possible values of the field `MQS_CLK_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MQS_CLK_DIVR {
    #[doc = "mclk frequency = hmclk frequency"]
    MQS_CLK_DIV_0,
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    MQS_CLK_DIV_1,
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    MQS_CLK_DIV_2,
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    MQS_CLK_DIV_255,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MQS_CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MQS_CLK_DIVR::MQS_CLK_DIV_0 => 0,
            MQS_CLK_DIVR::MQS_CLK_DIV_1 => 1,
            MQS_CLK_DIVR::MQS_CLK_DIV_2 => 2,
            MQS_CLK_DIVR::MQS_CLK_DIV_255 => 255,
            MQS_CLK_DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MQS_CLK_DIVR {
        match value {
            0 => MQS_CLK_DIVR::MQS_CLK_DIV_0,
            1 => MQS_CLK_DIVR::MQS_CLK_DIV_1,
            2 => MQS_CLK_DIVR::MQS_CLK_DIV_2,
            255 => MQS_CLK_DIVR::MQS_CLK_DIV_255,
            i => MQS_CLK_DIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MQS_CLK_DIV_0`"]
    #[inline]
    pub fn is_mqs_clk_div_0(&self) -> bool {
        *self == MQS_CLK_DIVR::MQS_CLK_DIV_0
    }
    #[doc = "Checks if the value of the field is `MQS_CLK_DIV_1`"]
    #[inline]
    pub fn is_mqs_clk_div_1(&self) -> bool {
        *self == MQS_CLK_DIVR::MQS_CLK_DIV_1
    }
    #[doc = "Checks if the value of the field is `MQS_CLK_DIV_2`"]
    #[inline]
    pub fn is_mqs_clk_div_2(&self) -> bool {
        *self == MQS_CLK_DIVR::MQS_CLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MQS_CLK_DIV_255`"]
    #[inline]
    pub fn is_mqs_clk_div_255(&self) -> bool {
        *self == MQS_CLK_DIVR::MQS_CLK_DIV_255
    }
}
#[doc = "Possible values of the field `MQS_SW_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MQS_SW_RSTR {
    #[doc = "Exit software reset for MQS"]
    MQS_SW_RST_0,
    #[doc = "Enable software reset for MQS"]
    MQS_SW_RST_1,
}
impl MQS_SW_RSTR {
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
            MQS_SW_RSTR::MQS_SW_RST_0 => false,
            MQS_SW_RSTR::MQS_SW_RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MQS_SW_RSTR {
        match value {
            false => MQS_SW_RSTR::MQS_SW_RST_0,
            true => MQS_SW_RSTR::MQS_SW_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `MQS_SW_RST_0`"]
    #[inline]
    pub fn is_mqs_sw_rst_0(&self) -> bool {
        *self == MQS_SW_RSTR::MQS_SW_RST_0
    }
    #[doc = "Checks if the value of the field is `MQS_SW_RST_1`"]
    #[inline]
    pub fn is_mqs_sw_rst_1(&self) -> bool {
        *self == MQS_SW_RSTR::MQS_SW_RST_1
    }
}
#[doc = "Possible values of the field `MQS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MQS_ENR {
    #[doc = "Disable MQS"]
    MQS_EN_0,
    #[doc = "Enable MQS"]
    MQS_EN_1,
}
impl MQS_ENR {
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
            MQS_ENR::MQS_EN_0 => false,
            MQS_ENR::MQS_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MQS_ENR {
        match value {
            false => MQS_ENR::MQS_EN_0,
            true => MQS_ENR::MQS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MQS_EN_0`"]
    #[inline]
    pub fn is_mqs_en_0(&self) -> bool {
        *self == MQS_ENR::MQS_EN_0
    }
    #[doc = "Checks if the value of the field is `MQS_EN_1`"]
    #[inline]
    pub fn is_mqs_en_1(&self) -> bool {
        *self == MQS_ENR::MQS_EN_1
    }
}
#[doc = "Possible values of the field `MQS_OVERSAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MQS_OVERSAMPLER {
    #[doc = "32"]
    MQS_OVERSAMPLE_0,
    #[doc = "64"]
    MQS_OVERSAMPLE_1,
}
impl MQS_OVERSAMPLER {
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
            MQS_OVERSAMPLER::MQS_OVERSAMPLE_0 => false,
            MQS_OVERSAMPLER::MQS_OVERSAMPLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MQS_OVERSAMPLER {
        match value {
            false => MQS_OVERSAMPLER::MQS_OVERSAMPLE_0,
            true => MQS_OVERSAMPLER::MQS_OVERSAMPLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MQS_OVERSAMPLE_0`"]
    #[inline]
    pub fn is_mqs_oversample_0(&self) -> bool {
        *self == MQS_OVERSAMPLER::MQS_OVERSAMPLE_0
    }
    #[doc = "Checks if the value of the field is `MQS_OVERSAMPLE_1`"]
    #[inline]
    pub fn is_mqs_oversample_1(&self) -> bool {
        *self == MQS_OVERSAMPLER::MQS_OVERSAMPLE_1
    }
}
#[doc = "Possible values of the field `QTIMER1_TMR_CNTS_FREEZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TMR_CNTS_FREEZER {
    #[doc = "timer counter work normally"]
    QTIMER1_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER1_TMR_CNTS_FREEZE_1,
}
impl QTIMER1_TMR_CNTS_FREEZER {
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
            QTIMER1_TMR_CNTS_FREEZER::QTIMER1_TMR_CNTS_FREEZE_0 => false,
            QTIMER1_TMR_CNTS_FREEZER::QTIMER1_TMR_CNTS_FREEZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER1_TMR_CNTS_FREEZER {
        match value {
            false => QTIMER1_TMR_CNTS_FREEZER::QTIMER1_TMR_CNTS_FREEZE_0,
            true => QTIMER1_TMR_CNTS_FREEZER::QTIMER1_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TMR_CNTS_FREEZE_0`"]
    #[inline]
    pub fn is_qtimer1_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER1_TMR_CNTS_FREEZER::QTIMER1_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TMR_CNTS_FREEZE_1`"]
    #[inline]
    pub fn is_qtimer1_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER1_TMR_CNTS_FREEZER::QTIMER1_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Possible values of the field `QTIMER2_TMR_CNTS_FREEZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TMR_CNTS_FREEZER {
    #[doc = "timer counter work normally"]
    QTIMER2_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER2_TMR_CNTS_FREEZE_1,
}
impl QTIMER2_TMR_CNTS_FREEZER {
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
            QTIMER2_TMR_CNTS_FREEZER::QTIMER2_TMR_CNTS_FREEZE_0 => false,
            QTIMER2_TMR_CNTS_FREEZER::QTIMER2_TMR_CNTS_FREEZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER2_TMR_CNTS_FREEZER {
        match value {
            false => QTIMER2_TMR_CNTS_FREEZER::QTIMER2_TMR_CNTS_FREEZE_0,
            true => QTIMER2_TMR_CNTS_FREEZER::QTIMER2_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TMR_CNTS_FREEZE_0`"]
    #[inline]
    pub fn is_qtimer2_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER2_TMR_CNTS_FREEZER::QTIMER2_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TMR_CNTS_FREEZE_1`"]
    #[inline]
    pub fn is_qtimer2_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER2_TMR_CNTS_FREEZER::QTIMER2_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Possible values of the field `QTIMER3_TMR_CNTS_FREEZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TMR_CNTS_FREEZER {
    #[doc = "timer counter work normally"]
    QTIMER3_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER3_TMR_CNTS_FREEZE_1,
}
impl QTIMER3_TMR_CNTS_FREEZER {
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
            QTIMER3_TMR_CNTS_FREEZER::QTIMER3_TMR_CNTS_FREEZE_0 => false,
            QTIMER3_TMR_CNTS_FREEZER::QTIMER3_TMR_CNTS_FREEZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER3_TMR_CNTS_FREEZER {
        match value {
            false => QTIMER3_TMR_CNTS_FREEZER::QTIMER3_TMR_CNTS_FREEZE_0,
            true => QTIMER3_TMR_CNTS_FREEZER::QTIMER3_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TMR_CNTS_FREEZE_0`"]
    #[inline]
    pub fn is_qtimer3_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER3_TMR_CNTS_FREEZER::QTIMER3_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TMR_CNTS_FREEZE_1`"]
    #[inline]
    pub fn is_qtimer3_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER3_TMR_CNTS_FREEZER::QTIMER3_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Possible values of the field `QTIMER4_TMR_CNTS_FREEZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TMR_CNTS_FREEZER {
    #[doc = "timer counter work normally"]
    QTIMER4_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER4_TMR_CNTS_FREEZE_1,
}
impl QTIMER4_TMR_CNTS_FREEZER {
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
            QTIMER4_TMR_CNTS_FREEZER::QTIMER4_TMR_CNTS_FREEZE_0 => false,
            QTIMER4_TMR_CNTS_FREEZER::QTIMER4_TMR_CNTS_FREEZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QTIMER4_TMR_CNTS_FREEZER {
        match value {
            false => QTIMER4_TMR_CNTS_FREEZER::QTIMER4_TMR_CNTS_FREEZE_0,
            true => QTIMER4_TMR_CNTS_FREEZER::QTIMER4_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TMR_CNTS_FREEZE_0`"]
    #[inline]
    pub fn is_qtimer4_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER4_TMR_CNTS_FREEZER::QTIMER4_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TMR_CNTS_FREEZE_1`"]
    #[inline]
    pub fn is_qtimer4_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER4_TMR_CNTS_FREEZER::QTIMER4_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Values that can be written to the field `L2_MEM_EN_POWERSAVING`"]
pub enum L2_MEM_EN_POWERSAVINGW {
    #[doc = "none memory power saving features enabled, SHUTDOWN/DEEPSLEEP/LIGHTSLEEP will have no effect"]
    L2_MEM_EN_POWERSAVING_0,
    #[doc = "memory power saving features enabled, set SHUTDOWN/DEEPSLEEP/LIGHTSLEEP(priority high to low) to enable power saving levels"]
    L2_MEM_EN_POWERSAVING_1,
}
impl L2_MEM_EN_POWERSAVINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L2_MEM_EN_POWERSAVINGW::L2_MEM_EN_POWERSAVING_0 => false,
            L2_MEM_EN_POWERSAVINGW::L2_MEM_EN_POWERSAVING_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L2_MEM_EN_POWERSAVINGW<'a> {
    w: &'a mut W,
}
impl<'a> _L2_MEM_EN_POWERSAVINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L2_MEM_EN_POWERSAVINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "none memory power saving features enabled, SHUTDOWN/DEEPSLEEP/LIGHTSLEEP will have no effect"]
    #[inline]
    pub fn l2_mem_en_powersaving_0(self) -> &'a mut W {
        self.variant(L2_MEM_EN_POWERSAVINGW::L2_MEM_EN_POWERSAVING_0)
    }
    #[doc = "memory power saving features enabled, set SHUTDOWN/DEEPSLEEP/LIGHTSLEEP(priority high to low) to enable power saving levels"]
    #[inline]
    pub fn l2_mem_en_powersaving_1(self) -> &'a mut W {
        self.variant(L2_MEM_EN_POWERSAVINGW::L2_MEM_EN_POWERSAVING_1)
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
#[doc = "Values that can be written to the field `L2_MEM_DEEPSLEEP`"]
pub enum L2_MEM_DEEPSLEEPW {
    #[doc = "no force sleep control supported, memory deep sleep mode only entered when whole system in stop mode"]
    L2_MEM_DEEPSLEEP_0,
    #[doc = "force memory into deep sleep mode"]
    L2_MEM_DEEPSLEEP_1,
}
impl L2_MEM_DEEPSLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L2_MEM_DEEPSLEEPW::L2_MEM_DEEPSLEEP_0 => false,
            L2_MEM_DEEPSLEEPW::L2_MEM_DEEPSLEEP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L2_MEM_DEEPSLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _L2_MEM_DEEPSLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L2_MEM_DEEPSLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no force sleep control supported, memory deep sleep mode only entered when whole system in stop mode"]
    #[inline]
    pub fn l2_mem_deepsleep_0(self) -> &'a mut W {
        self.variant(L2_MEM_DEEPSLEEPW::L2_MEM_DEEPSLEEP_0)
    }
    #[doc = "force memory into deep sleep mode"]
    #[inline]
    pub fn l2_mem_deepsleep_1(self) -> &'a mut W {
        self.variant(L2_MEM_DEEPSLEEPW::L2_MEM_DEEPSLEEP_1)
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
#[doc = "Values that can be written to the field `MQS_CLK_DIV`"]
pub enum MQS_CLK_DIVW {
    #[doc = "mclk frequency = hmclk frequency"]
    MQS_CLK_DIV_0,
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    MQS_CLK_DIV_1,
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    MQS_CLK_DIV_2,
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    MQS_CLK_DIV_255,
}
impl MQS_CLK_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MQS_CLK_DIVW::MQS_CLK_DIV_0 => 0,
            MQS_CLK_DIVW::MQS_CLK_DIV_1 => 1,
            MQS_CLK_DIVW::MQS_CLK_DIV_2 => 2,
            MQS_CLK_DIVW::MQS_CLK_DIV_255 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MQS_CLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _MQS_CLK_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MQS_CLK_DIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "mclk frequency = hmclk frequency"]
    #[inline]
    pub fn mqs_clk_div_0(self) -> &'a mut W {
        self.variant(MQS_CLK_DIVW::MQS_CLK_DIV_0)
    }
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    #[inline]
    pub fn mqs_clk_div_1(self) -> &'a mut W {
        self.variant(MQS_CLK_DIVW::MQS_CLK_DIV_1)
    }
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    #[inline]
    pub fn mqs_clk_div_2(self) -> &'a mut W {
        self.variant(MQS_CLK_DIVW::MQS_CLK_DIV_2)
    }
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    #[inline]
    pub fn mqs_clk_div_255(self) -> &'a mut W {
        self.variant(MQS_CLK_DIVW::MQS_CLK_DIV_255)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MQS_SW_RST`"]
pub enum MQS_SW_RSTW {
    #[doc = "Exit software reset for MQS"]
    MQS_SW_RST_0,
    #[doc = "Enable software reset for MQS"]
    MQS_SW_RST_1,
}
impl MQS_SW_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MQS_SW_RSTW::MQS_SW_RST_0 => false,
            MQS_SW_RSTW::MQS_SW_RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MQS_SW_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MQS_SW_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MQS_SW_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exit software reset for MQS"]
    #[inline]
    pub fn mqs_sw_rst_0(self) -> &'a mut W {
        self.variant(MQS_SW_RSTW::MQS_SW_RST_0)
    }
    #[doc = "Enable software reset for MQS"]
    #[inline]
    pub fn mqs_sw_rst_1(self) -> &'a mut W {
        self.variant(MQS_SW_RSTW::MQS_SW_RST_1)
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
#[doc = "Values that can be written to the field `MQS_EN`"]
pub enum MQS_ENW {
    #[doc = "Disable MQS"]
    MQS_EN_0,
    #[doc = "Enable MQS"]
    MQS_EN_1,
}
impl MQS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MQS_ENW::MQS_EN_0 => false,
            MQS_ENW::MQS_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MQS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MQS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MQS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable MQS"]
    #[inline]
    pub fn mqs_en_0(self) -> &'a mut W {
        self.variant(MQS_ENW::MQS_EN_0)
    }
    #[doc = "Enable MQS"]
    #[inline]
    pub fn mqs_en_1(self) -> &'a mut W {
        self.variant(MQS_ENW::MQS_EN_1)
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
#[doc = "Values that can be written to the field `MQS_OVERSAMPLE`"]
pub enum MQS_OVERSAMPLEW {
    #[doc = "32"]
    MQS_OVERSAMPLE_0,
    #[doc = "64"]
    MQS_OVERSAMPLE_1,
}
impl MQS_OVERSAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MQS_OVERSAMPLEW::MQS_OVERSAMPLE_0 => false,
            MQS_OVERSAMPLEW::MQS_OVERSAMPLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MQS_OVERSAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MQS_OVERSAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MQS_OVERSAMPLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "32"]
    #[inline]
    pub fn mqs_oversample_0(self) -> &'a mut W {
        self.variant(MQS_OVERSAMPLEW::MQS_OVERSAMPLE_0)
    }
    #[doc = "64"]
    #[inline]
    pub fn mqs_oversample_1(self) -> &'a mut W {
        self.variant(MQS_OVERSAMPLEW::MQS_OVERSAMPLE_1)
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
#[doc = "Values that can be written to the field `QTIMER1_TMR_CNTS_FREEZE`"]
pub enum QTIMER1_TMR_CNTS_FREEZEW {
    #[doc = "timer counter work normally"]
    QTIMER1_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER1_TMR_CNTS_FREEZE_1,
}
impl QTIMER1_TMR_CNTS_FREEZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER1_TMR_CNTS_FREEZEW::QTIMER1_TMR_CNTS_FREEZE_0 => false,
            QTIMER1_TMR_CNTS_FREEZEW::QTIMER1_TMR_CNTS_FREEZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER1_TMR_CNTS_FREEZEW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER1_TMR_CNTS_FREEZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER1_TMR_CNTS_FREEZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline]
    pub fn qtimer1_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER1_TMR_CNTS_FREEZEW::QTIMER1_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline]
    pub fn qtimer1_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER1_TMR_CNTS_FREEZEW::QTIMER1_TMR_CNTS_FREEZE_1)
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
#[doc = "Values that can be written to the field `QTIMER2_TMR_CNTS_FREEZE`"]
pub enum QTIMER2_TMR_CNTS_FREEZEW {
    #[doc = "timer counter work normally"]
    QTIMER2_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER2_TMR_CNTS_FREEZE_1,
}
impl QTIMER2_TMR_CNTS_FREEZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER2_TMR_CNTS_FREEZEW::QTIMER2_TMR_CNTS_FREEZE_0 => false,
            QTIMER2_TMR_CNTS_FREEZEW::QTIMER2_TMR_CNTS_FREEZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER2_TMR_CNTS_FREEZEW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER2_TMR_CNTS_FREEZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER2_TMR_CNTS_FREEZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline]
    pub fn qtimer2_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER2_TMR_CNTS_FREEZEW::QTIMER2_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline]
    pub fn qtimer2_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER2_TMR_CNTS_FREEZEW::QTIMER2_TMR_CNTS_FREEZE_1)
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
#[doc = "Values that can be written to the field `QTIMER3_TMR_CNTS_FREEZE`"]
pub enum QTIMER3_TMR_CNTS_FREEZEW {
    #[doc = "timer counter work normally"]
    QTIMER3_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER3_TMR_CNTS_FREEZE_1,
}
impl QTIMER3_TMR_CNTS_FREEZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER3_TMR_CNTS_FREEZEW::QTIMER3_TMR_CNTS_FREEZE_0 => false,
            QTIMER3_TMR_CNTS_FREEZEW::QTIMER3_TMR_CNTS_FREEZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER3_TMR_CNTS_FREEZEW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER3_TMR_CNTS_FREEZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER3_TMR_CNTS_FREEZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline]
    pub fn qtimer3_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER3_TMR_CNTS_FREEZEW::QTIMER3_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline]
    pub fn qtimer3_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER3_TMR_CNTS_FREEZEW::QTIMER3_TMR_CNTS_FREEZE_1)
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
#[doc = "Values that can be written to the field `QTIMER4_TMR_CNTS_FREEZE`"]
pub enum QTIMER4_TMR_CNTS_FREEZEW {
    #[doc = "timer counter work normally"]
    QTIMER4_TMR_CNTS_FREEZE_0,
    #[doc = "reset counter and ouput flags"]
    QTIMER4_TMR_CNTS_FREEZE_1,
}
impl QTIMER4_TMR_CNTS_FREEZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QTIMER4_TMR_CNTS_FREEZEW::QTIMER4_TMR_CNTS_FREEZE_0 => false,
            QTIMER4_TMR_CNTS_FREEZEW::QTIMER4_TMR_CNTS_FREEZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QTIMER4_TMR_CNTS_FREEZEW<'a> {
    w: &'a mut W,
}
impl<'a> _QTIMER4_TMR_CNTS_FREEZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QTIMER4_TMR_CNTS_FREEZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline]
    pub fn qtimer4_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER4_TMR_CNTS_FREEZEW::QTIMER4_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline]
    pub fn qtimer4_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER4_TMR_CNTS_FREEZEW::QTIMER4_TMR_CNTS_FREEZE_1)
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
    #[doc = "Bit 12 - enable power saving features on L2 memory"]
    #[inline]
    pub fn l2_mem_en_powersaving(&self) -> L2_MEM_EN_POWERSAVINGR {
        L2_MEM_EN_POWERSAVINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)"]
    #[inline]
    pub fn l2_mem_deepsleep(&self) -> L2_MEM_DEEPSLEEPR {
        L2_MEM_DEEPSLEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency."]
    #[inline]
    pub fn mqs_clk_div(&self) -> MQS_CLK_DIVR {
        MQS_CLK_DIVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - MQS software reset"]
    #[inline]
    pub fn mqs_sw_rst(&self) -> MQS_SW_RSTR {
        MQS_SW_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - MQS enable."]
    #[inline]
    pub fn mqs_en(&self) -> MQS_ENR {
        MQS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Used to control the PWM oversampling rate compared with mclk."]
    #[inline]
    pub fn mqs_oversample(&self) -> MQS_OVERSAMPLER {
        MQS_OVERSAMPLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - QTIMER1 timer counter freeze"]
    #[inline]
    pub fn qtimer1_tmr_cnts_freeze(&self) -> QTIMER1_TMR_CNTS_FREEZER {
        QTIMER1_TMR_CNTS_FREEZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - QTIMER2 timer counter freeze"]
    #[inline]
    pub fn qtimer2_tmr_cnts_freeze(&self) -> QTIMER2_TMR_CNTS_FREEZER {
        QTIMER2_TMR_CNTS_FREEZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - QTIMER3 timer counter freeze"]
    #[inline]
    pub fn qtimer3_tmr_cnts_freeze(&self) -> QTIMER3_TMR_CNTS_FREEZER {
        QTIMER3_TMR_CNTS_FREEZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - QTIMER4 timer counter freeze"]
    #[inline]
    pub fn qtimer4_tmr_cnts_freeze(&self) -> QTIMER4_TMR_CNTS_FREEZER {
        QTIMER4_TMR_CNTS_FREEZER::_from({
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
    #[doc = "Bit 12 - enable power saving features on L2 memory"]
    #[inline]
    pub fn l2_mem_en_powersaving(&mut self) -> _L2_MEM_EN_POWERSAVINGW {
        _L2_MEM_EN_POWERSAVINGW { w: self }
    }
    #[doc = "Bit 14 - control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)"]
    #[inline]
    pub fn l2_mem_deepsleep(&mut self) -> _L2_MEM_DEEPSLEEPW {
        _L2_MEM_DEEPSLEEPW { w: self }
    }
    #[doc = "Bits 16:23 - Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency."]
    #[inline]
    pub fn mqs_clk_div(&mut self) -> _MQS_CLK_DIVW {
        _MQS_CLK_DIVW { w: self }
    }
    #[doc = "Bit 24 - MQS software reset"]
    #[inline]
    pub fn mqs_sw_rst(&mut self) -> _MQS_SW_RSTW {
        _MQS_SW_RSTW { w: self }
    }
    #[doc = "Bit 25 - MQS enable."]
    #[inline]
    pub fn mqs_en(&mut self) -> _MQS_ENW {
        _MQS_ENW { w: self }
    }
    #[doc = "Bit 26 - Used to control the PWM oversampling rate compared with mclk."]
    #[inline]
    pub fn mqs_oversample(&mut self) -> _MQS_OVERSAMPLEW {
        _MQS_OVERSAMPLEW { w: self }
    }
    #[doc = "Bit 28 - QTIMER1 timer counter freeze"]
    #[inline]
    pub fn qtimer1_tmr_cnts_freeze(&mut self) -> _QTIMER1_TMR_CNTS_FREEZEW {
        _QTIMER1_TMR_CNTS_FREEZEW { w: self }
    }
    #[doc = "Bit 29 - QTIMER2 timer counter freeze"]
    #[inline]
    pub fn qtimer2_tmr_cnts_freeze(&mut self) -> _QTIMER2_TMR_CNTS_FREEZEW {
        _QTIMER2_TMR_CNTS_FREEZEW { w: self }
    }
    #[doc = "Bit 30 - QTIMER3 timer counter freeze"]
    #[inline]
    pub fn qtimer3_tmr_cnts_freeze(&mut self) -> _QTIMER3_TMR_CNTS_FREEZEW {
        _QTIMER3_TMR_CNTS_FREEZEW { w: self }
    }
    #[doc = "Bit 31 - QTIMER4 timer counter freeze"]
    #[inline]
    pub fn qtimer4_tmr_cnts_freeze(&mut self) -> _QTIMER4_TMR_CNTS_FREEZEW {
        _QTIMER4_TMR_CNTS_FREEZEW { w: self }
    }
}
