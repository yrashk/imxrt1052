#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CGPR {
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
#[doc = "Possible values of the field `PMIC_DELAY_SCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMIC_DELAY_SCALERR {
    #[doc = "clock is not divided"]
    PMIC_DELAY_SCALER_0,
    #[doc = "clock is divided /8"]
    PMIC_DELAY_SCALER_1,
}
impl PMIC_DELAY_SCALERR {
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
            PMIC_DELAY_SCALERR::PMIC_DELAY_SCALER_0 => false,
            PMIC_DELAY_SCALERR::PMIC_DELAY_SCALER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMIC_DELAY_SCALERR {
        match value {
            false => PMIC_DELAY_SCALERR::PMIC_DELAY_SCALER_0,
            true => PMIC_DELAY_SCALERR::PMIC_DELAY_SCALER_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMIC_DELAY_SCALER_0`"]
    #[inline]
    pub fn is_pmic_delay_scaler_0(&self) -> bool {
        *self == PMIC_DELAY_SCALERR::PMIC_DELAY_SCALER_0
    }
    #[doc = "Checks if the value of the field is `PMIC_DELAY_SCALER_1`"]
    #[inline]
    pub fn is_pmic_delay_scaler_1(&self) -> bool {
        *self == PMIC_DELAY_SCALERR::PMIC_DELAY_SCALER_1
    }
}
#[doc = "Possible values of the field `EFUSE_PROG_SUPPLY_GATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFUSE_PROG_SUPPLY_GATER {
    #[doc = "fuse programing supply voltage is gated off to the efuse module"]
    EFUSE_PROG_SUPPLY_GATE_0,
    #[doc = "allow fuse programing."]
    EFUSE_PROG_SUPPLY_GATE_1,
}
impl EFUSE_PROG_SUPPLY_GATER {
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
            EFUSE_PROG_SUPPLY_GATER::EFUSE_PROG_SUPPLY_GATE_0 => false,
            EFUSE_PROG_SUPPLY_GATER::EFUSE_PROG_SUPPLY_GATE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EFUSE_PROG_SUPPLY_GATER {
        match value {
            false => EFUSE_PROG_SUPPLY_GATER::EFUSE_PROG_SUPPLY_GATE_0,
            true => EFUSE_PROG_SUPPLY_GATER::EFUSE_PROG_SUPPLY_GATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EFUSE_PROG_SUPPLY_GATE_0`"]
    #[inline]
    pub fn is_efuse_prog_supply_gate_0(&self) -> bool {
        *self == EFUSE_PROG_SUPPLY_GATER::EFUSE_PROG_SUPPLY_GATE_0
    }
    #[doc = "Checks if the value of the field is `EFUSE_PROG_SUPPLY_GATE_1`"]
    #[inline]
    pub fn is_efuse_prog_supply_gate_1(&self) -> bool {
        *self == EFUSE_PROG_SUPPLY_GATER::EFUSE_PROG_SUPPLY_GATE_1
    }
}
#[doc = "Possible values of the field `SYS_MEM_DS_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_MEM_DS_CTRLR {
    #[doc = "Disable memory DS mode always"]
    SYS_MEM_DS_CTRL_0,
    #[doc = "Enable memory (outside ARM platform) DS mode when system STOP and PLL are disabled"]
    SYS_MEM_DS_CTRL_1,
    #[doc = "enable memory (outside ARM platform) DS mode when system is in STOP mode"]
    SYS_MEM_DS_CTRL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYS_MEM_DS_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_0 => 0,
            SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_1 => 1,
            SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_2 => 2,
            SYS_MEM_DS_CTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYS_MEM_DS_CTRLR {
        match value {
            0 => SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_0,
            1 => SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_1,
            2 => SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_2,
            i => SYS_MEM_DS_CTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_0`"]
    #[inline]
    pub fn is_sys_mem_ds_ctrl_0(&self) -> bool {
        *self == SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_0
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_1`"]
    #[inline]
    pub fn is_sys_mem_ds_ctrl_1(&self) -> bool {
        *self == SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_1
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_2`"]
    #[inline]
    pub fn is_sys_mem_ds_ctrl_2(&self) -> bool {
        *self == SYS_MEM_DS_CTRLR::SYS_MEM_DS_CTRL_2
    }
}
#[doc = "Possible values of the field `FPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPLR {
    #[doc = "Engage PLL enable default way."]
    FPL_0,
    #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    FPL_1,
}
impl FPLR {
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
            FPLR::FPL_0 => false,
            FPLR::FPL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPLR {
        match value {
            false => FPLR::FPL_0,
            true => FPLR::FPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FPL_0`"]
    #[inline]
    pub fn is_fpl_0(&self) -> bool {
        *self == FPLR::FPL_0
    }
    #[doc = "Checks if the value of the field is `FPL_1`"]
    #[inline]
    pub fn is_fpl_1(&self) -> bool {
        *self == FPLR::FPL_1
    }
}
#[doc = "Possible values of the field `INT_MEM_CLK_LPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_MEM_CLK_LPMR {
    #[doc = "Disable the clock to the ARM platform memories when entering Low Power Mode"]
    INT_MEM_CLK_LPM_0,
    #[doc = "Keep the clocks to the ARM platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    INT_MEM_CLK_LPM_1,
}
impl INT_MEM_CLK_LPMR {
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
            INT_MEM_CLK_LPMR::INT_MEM_CLK_LPM_0 => false,
            INT_MEM_CLK_LPMR::INT_MEM_CLK_LPM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_MEM_CLK_LPMR {
        match value {
            false => INT_MEM_CLK_LPMR::INT_MEM_CLK_LPM_0,
            true => INT_MEM_CLK_LPMR::INT_MEM_CLK_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_MEM_CLK_LPM_0`"]
    #[inline]
    pub fn is_int_mem_clk_lpm_0(&self) -> bool {
        *self == INT_MEM_CLK_LPMR::INT_MEM_CLK_LPM_0
    }
    #[doc = "Checks if the value of the field is `INT_MEM_CLK_LPM_1`"]
    #[inline]
    pub fn is_int_mem_clk_lpm_1(&self) -> bool {
        *self == INT_MEM_CLK_LPMR::INT_MEM_CLK_LPM_1
    }
}
#[doc = "Values that can be written to the field `PMIC_DELAY_SCALER`"]
pub enum PMIC_DELAY_SCALERW {
    #[doc = "clock is not divided"]
    PMIC_DELAY_SCALER_0,
    #[doc = "clock is divided /8"]
    PMIC_DELAY_SCALER_1,
}
impl PMIC_DELAY_SCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMIC_DELAY_SCALERW::PMIC_DELAY_SCALER_0 => false,
            PMIC_DELAY_SCALERW::PMIC_DELAY_SCALER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMIC_DELAY_SCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PMIC_DELAY_SCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMIC_DELAY_SCALERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clock is not divided"]
    #[inline]
    pub fn pmic_delay_scaler_0(self) -> &'a mut W {
        self.variant(PMIC_DELAY_SCALERW::PMIC_DELAY_SCALER_0)
    }
    #[doc = "clock is divided /8"]
    #[inline]
    pub fn pmic_delay_scaler_1(self) -> &'a mut W {
        self.variant(PMIC_DELAY_SCALERW::PMIC_DELAY_SCALER_1)
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
#[doc = "Values that can be written to the field `EFUSE_PROG_SUPPLY_GATE`"]
pub enum EFUSE_PROG_SUPPLY_GATEW {
    #[doc = "fuse programing supply voltage is gated off to the efuse module"]
    EFUSE_PROG_SUPPLY_GATE_0,
    #[doc = "allow fuse programing."]
    EFUSE_PROG_SUPPLY_GATE_1,
}
impl EFUSE_PROG_SUPPLY_GATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EFUSE_PROG_SUPPLY_GATEW::EFUSE_PROG_SUPPLY_GATE_0 => false,
            EFUSE_PROG_SUPPLY_GATEW::EFUSE_PROG_SUPPLY_GATE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EFUSE_PROG_SUPPLY_GATEW<'a> {
    w: &'a mut W,
}
impl<'a> _EFUSE_PROG_SUPPLY_GATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EFUSE_PROG_SUPPLY_GATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fuse programing supply voltage is gated off to the efuse module"]
    #[inline]
    pub fn efuse_prog_supply_gate_0(self) -> &'a mut W {
        self.variant(EFUSE_PROG_SUPPLY_GATEW::EFUSE_PROG_SUPPLY_GATE_0)
    }
    #[doc = "allow fuse programing."]
    #[inline]
    pub fn efuse_prog_supply_gate_1(self) -> &'a mut W {
        self.variant(EFUSE_PROG_SUPPLY_GATEW::EFUSE_PROG_SUPPLY_GATE_1)
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
#[doc = "Values that can be written to the field `SYS_MEM_DS_CTRL`"]
pub enum SYS_MEM_DS_CTRLW {
    #[doc = "Disable memory DS mode always"]
    SYS_MEM_DS_CTRL_0,
    #[doc = "Enable memory (outside ARM platform) DS mode when system STOP and PLL are disabled"]
    SYS_MEM_DS_CTRL_1,
    #[doc = "enable memory (outside ARM platform) DS mode when system is in STOP mode"]
    SYS_MEM_DS_CTRL_2,
}
impl SYS_MEM_DS_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYS_MEM_DS_CTRLW::SYS_MEM_DS_CTRL_0 => 0,
            SYS_MEM_DS_CTRLW::SYS_MEM_DS_CTRL_1 => 1,
            SYS_MEM_DS_CTRLW::SYS_MEM_DS_CTRL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYS_MEM_DS_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_DS_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYS_MEM_DS_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable memory DS mode always"]
    #[inline]
    pub fn sys_mem_ds_ctrl_0(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRLW::SYS_MEM_DS_CTRL_0)
    }
    #[doc = "Enable memory (outside ARM platform) DS mode when system STOP and PLL are disabled"]
    #[inline]
    pub fn sys_mem_ds_ctrl_1(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRLW::SYS_MEM_DS_CTRL_1)
    }
    #[doc = "enable memory (outside ARM platform) DS mode when system is in STOP mode"]
    #[inline]
    pub fn sys_mem_ds_ctrl_2(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRLW::SYS_MEM_DS_CTRL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FPL`"]
pub enum FPLW {
    #[doc = "Engage PLL enable default way."]
    FPL_0,
    #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    FPL_1,
}
impl FPLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPLW::FPL_0 => false,
            FPLW::FPL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPLW<'a> {
    w: &'a mut W,
}
impl<'a> _FPLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Engage PLL enable default way."]
    #[inline]
    pub fn fpl_0(self) -> &'a mut W {
        self.variant(FPLW::FPL_0)
    }
    #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    #[inline]
    pub fn fpl_1(self) -> &'a mut W {
        self.variant(FPLW::FPL_1)
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
#[doc = "Values that can be written to the field `INT_MEM_CLK_LPM`"]
pub enum INT_MEM_CLK_LPMW {
    #[doc = "Disable the clock to the ARM platform memories when entering Low Power Mode"]
    INT_MEM_CLK_LPM_0,
    #[doc = "Keep the clocks to the ARM platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    INT_MEM_CLK_LPM_1,
}
impl INT_MEM_CLK_LPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_MEM_CLK_LPMW::INT_MEM_CLK_LPM_0 => false,
            INT_MEM_CLK_LPMW::INT_MEM_CLK_LPM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_MEM_CLK_LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_MEM_CLK_LPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_MEM_CLK_LPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the clock to the ARM platform memories when entering Low Power Mode"]
    #[inline]
    pub fn int_mem_clk_lpm_0(self) -> &'a mut W {
        self.variant(INT_MEM_CLK_LPMW::INT_MEM_CLK_LPM_0)
    }
    #[doc = "Keep the clocks to the ARM platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    #[inline]
    pub fn int_mem_clk_lpm_1(self) -> &'a mut W {
        self.variant(INT_MEM_CLK_LPMW::INT_MEM_CLK_LPM_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline]
    pub fn pmic_delay_scaler(&self) -> PMIC_DELAY_SCALERR {
        PMIC_DELAY_SCALERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Defines the value of the output signal cgpr_dout[4]. Gate of program supply for efuse programing"]
    #[inline]
    pub fn efuse_prog_supply_gate(&self) -> EFUSE_PROG_SUPPLY_GATER {
        EFUSE_PROG_SUPPLY_GATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - System memory DS control"]
    #[inline]
    pub fn sys_mem_ds_ctrl(&self) -> SYS_MEM_DS_CTRLR {
        SYS_MEM_DS_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Fast PLL enable."]
    #[inline]
    pub fn fpl(&self) -> FPLR {
        FPLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Control for the Deep Sleep signal to the ARM Platform memories with additional control logic based on the ARM WFI signal"]
    #[inline]
    pub fn int_mem_clk_lpm(&self) -> INT_MEM_CLK_LPMR {
        INT_MEM_CLK_LPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65122 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline]
    pub fn pmic_delay_scaler(&mut self) -> _PMIC_DELAY_SCALERW {
        _PMIC_DELAY_SCALERW { w: self }
    }
    #[doc = "Bit 4 - Defines the value of the output signal cgpr_dout[4]. Gate of program supply for efuse programing"]
    #[inline]
    pub fn efuse_prog_supply_gate(&mut self) -> _EFUSE_PROG_SUPPLY_GATEW {
        _EFUSE_PROG_SUPPLY_GATEW { w: self }
    }
    #[doc = "Bits 14:15 - System memory DS control"]
    #[inline]
    pub fn sys_mem_ds_ctrl(&mut self) -> _SYS_MEM_DS_CTRLW {
        _SYS_MEM_DS_CTRLW { w: self }
    }
    #[doc = "Bit 16 - Fast PLL enable."]
    #[inline]
    pub fn fpl(&mut self) -> _FPLW {
        _FPLW { w: self }
    }
    #[doc = "Bit 17 - Control for the Deep Sleep signal to the ARM Platform memories with additional control logic based on the ARM WFI signal"]
    #[inline]
    pub fn int_mem_clk_lpm(&mut self) -> _INT_MEM_CLK_LPMW {
        _INT_MEM_CLK_LPMW { w: self }
    }
}
