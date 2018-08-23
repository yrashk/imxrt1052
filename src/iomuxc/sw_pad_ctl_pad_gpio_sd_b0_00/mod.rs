#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SW_PAD_CTL_PAD_GPIO_SD_B0_00 {
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
#[doc = "Possible values of the field `SRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRER {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE,
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
            SRER::SRE_0_SLOW_SLEW_RATE => false,
            SRER::SRE_1_FAST_SLEW_RATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRER {
        match value {
            false => SRER::SRE_0_SLOW_SLEW_RATE,
            true => SRER::SRE_1_FAST_SLEW_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `SRE_0_SLOW_SLEW_RATE`"]
    #[inline]
    pub fn is_sre_0_slow_slew_rate(&self) -> bool {
        *self == SRER::SRE_0_SLOW_SLEW_RATE
    }
    #[doc = "Checks if the value of the field is `SRE_1_FAST_SLEW_RATE`"]
    #[inline]
    pub fn is_sre_1_fast_slew_rate(&self) -> bool {
        *self == SRER::SRE_1_FAST_SLEW_RATE
    }
}
#[doc = "Possible values of the field `DSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSER {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED,
    #[doc = "R0(260 Ohm @ 3.3V, 150 Ohm@1.8V, 240 Ohm for DDR)"]
    DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR,
    #[doc = "R0/2"]
    DSE_2_R0_2,
    #[doc = "R0/3"]
    DSE_3_R0_3,
    #[doc = "R0/4"]
    DSE_4_R0_4,
    #[doc = "R0/5"]
    DSE_5_R0_5,
    #[doc = "R0/6"]
    DSE_6_R0_6,
    #[doc = "R0/7"]
    DSE_7_R0_7,
}
impl DSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSER::DSE_0_OUTPUT_DRIVER_DISABLED => 0,
            DSER::DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR => 1,
            DSER::DSE_2_R0_2 => 2,
            DSER::DSE_3_R0_3 => 3,
            DSER::DSE_4_R0_4 => 4,
            DSER::DSE_5_R0_5 => 5,
            DSER::DSE_6_R0_6 => 6,
            DSER::DSE_7_R0_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSER {
        match value {
            0 => DSER::DSE_0_OUTPUT_DRIVER_DISABLED,
            1 => DSER::DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR,
            2 => DSER::DSE_2_R0_2,
            3 => DSER::DSE_3_R0_3,
            4 => DSER::DSE_4_R0_4,
            5 => DSER::DSE_5_R0_5,
            6 => DSER::DSE_6_R0_6,
            7 => DSER::DSE_7_R0_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DSE_0_OUTPUT_DRIVER_DISABLED`"]
    #[inline]
    pub fn is_dse_0_output_driver_disabled(&self) -> bool {
        *self == DSER::DSE_0_OUTPUT_DRIVER_DISABLED
    }
    #[doc = "Checks if the value of the field is `DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR`"]
    #[inline]
    pub fn is_dse_1_r0_260_ohm_3_3v_150_ohm_1_8v_240_ohm_for_ddr(&self) -> bool {
        *self == DSER::DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR
    }
    #[doc = "Checks if the value of the field is `DSE_2_R0_2`"]
    #[inline]
    pub fn is_dse_2_r0_2(&self) -> bool {
        *self == DSER::DSE_2_R0_2
    }
    #[doc = "Checks if the value of the field is `DSE_3_R0_3`"]
    #[inline]
    pub fn is_dse_3_r0_3(&self) -> bool {
        *self == DSER::DSE_3_R0_3
    }
    #[doc = "Checks if the value of the field is `DSE_4_R0_4`"]
    #[inline]
    pub fn is_dse_4_r0_4(&self) -> bool {
        *self == DSER::DSE_4_R0_4
    }
    #[doc = "Checks if the value of the field is `DSE_5_R0_5`"]
    #[inline]
    pub fn is_dse_5_r0_5(&self) -> bool {
        *self == DSER::DSE_5_R0_5
    }
    #[doc = "Checks if the value of the field is `DSE_6_R0_6`"]
    #[inline]
    pub fn is_dse_6_r0_6(&self) -> bool {
        *self == DSER::DSE_6_R0_6
    }
    #[doc = "Checks if the value of the field is `DSE_7_R0_7`"]
    #[inline]
    pub fn is_dse_7_r0_7(&self) -> bool {
        *self == DSER::DSE_7_R0_7
    }
}
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ,
    #[doc = "medium(100MHz)"]
    SPEED_2_MEDIUM_100MHZ,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ,
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::SPEED_0_LOW_50MHZ => 0,
            SPEEDR::SPEED_1_MEDIUM_100MHZ => 1,
            SPEEDR::SPEED_2_MEDIUM_100MHZ => 2,
            SPEEDR::SPEED_3_MAX_200MHZ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::SPEED_0_LOW_50MHZ,
            1 => SPEEDR::SPEED_1_MEDIUM_100MHZ,
            2 => SPEEDR::SPEED_2_MEDIUM_100MHZ,
            3 => SPEEDR::SPEED_3_MAX_200MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPEED_0_LOW_50MHZ`"]
    #[inline]
    pub fn is_speed_0_low_50mhz(&self) -> bool {
        *self == SPEEDR::SPEED_0_LOW_50MHZ
    }
    #[doc = "Checks if the value of the field is `SPEED_1_MEDIUM_100MHZ`"]
    #[inline]
    pub fn is_speed_1_medium_100mhz(&self) -> bool {
        *self == SPEEDR::SPEED_1_MEDIUM_100MHZ
    }
    #[doc = "Checks if the value of the field is `SPEED_2_MEDIUM_100MHZ`"]
    #[inline]
    pub fn is_speed_2_medium_100mhz(&self) -> bool {
        *self == SPEEDR::SPEED_2_MEDIUM_100MHZ
    }
    #[doc = "Checks if the value of the field is `SPEED_3_MAX_200MHZ`"]
    #[inline]
    pub fn is_speed_3_max_200mhz(&self) -> bool {
        *self == SPEEDR::SPEED_3_MAX_200MHZ
    }
}
#[doc = "Possible values of the field `ODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODER {
    #[doc = "Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED,
    #[doc = "Open Drain Enabled"]
    ODE_1_OPEN_DRAIN_ENABLED,
}
impl ODER {
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
            ODER::ODE_0_OPEN_DRAIN_DISABLED => false,
            ODER::ODE_1_OPEN_DRAIN_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODER {
        match value {
            false => ODER::ODE_0_OPEN_DRAIN_DISABLED,
            true => ODER::ODE_1_OPEN_DRAIN_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ODE_0_OPEN_DRAIN_DISABLED`"]
    #[inline]
    pub fn is_ode_0_open_drain_disabled(&self) -> bool {
        *self == ODER::ODE_0_OPEN_DRAIN_DISABLED
    }
    #[doc = "Checks if the value of the field is `ODE_1_OPEN_DRAIN_ENABLED`"]
    #[inline]
    pub fn is_ode_1_open_drain_enabled(&self) -> bool {
        *self == ODER::ODE_1_OPEN_DRAIN_ENABLED
    }
}
#[doc = "Possible values of the field `PKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKER {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED,
}
impl PKER {
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
            PKER::PKE_0_PULL_KEEPER_DISABLED => false,
            PKER::PKE_1_PULL_KEEPER_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PKER {
        match value {
            false => PKER::PKE_0_PULL_KEEPER_DISABLED,
            true => PKER::PKE_1_PULL_KEEPER_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PKE_0_PULL_KEEPER_DISABLED`"]
    #[inline]
    pub fn is_pke_0_pull_keeper_disabled(&self) -> bool {
        *self == PKER::PKE_0_PULL_KEEPER_DISABLED
    }
    #[doc = "Checks if the value of the field is `PKE_1_PULL_KEEPER_ENABLED`"]
    #[inline]
    pub fn is_pke_1_pull_keeper_enabled(&self) -> bool {
        *self == PKER::PKE_1_PULL_KEEPER_ENABLED
    }
}
#[doc = "Possible values of the field `PUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUER {
    #[doc = "Keeper"]
    PUE_0_KEEPER,
    #[doc = "Pull"]
    PUE_1_PULL,
}
impl PUER {
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
            PUER::PUE_0_KEEPER => false,
            PUER::PUE_1_PULL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUER {
        match value {
            false => PUER::PUE_0_KEEPER,
            true => PUER::PUE_1_PULL,
        }
    }
    #[doc = "Checks if the value of the field is `PUE_0_KEEPER`"]
    #[inline]
    pub fn is_pue_0_keeper(&self) -> bool {
        *self == PUER::PUE_0_KEEPER
    }
    #[doc = "Checks if the value of the field is `PUE_1_PULL`"]
    #[inline]
    pub fn is_pue_1_pull(&self) -> bool {
        *self == PUER::PUE_1_PULL
    }
}
#[doc = "Possible values of the field `PUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSR {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP,
}
impl PUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PUSR::PUS_0_100K_OHM_PULL_DOWN => 0,
            PUSR::PUS_1_47K_OHM_PULL_UP => 1,
            PUSR::PUS_2_100K_OHM_PULL_UP => 2,
            PUSR::PUS_3_22K_OHM_PULL_UP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PUSR {
        match value {
            0 => PUSR::PUS_0_100K_OHM_PULL_DOWN,
            1 => PUSR::PUS_1_47K_OHM_PULL_UP,
            2 => PUSR::PUS_2_100K_OHM_PULL_UP,
            3 => PUSR::PUS_3_22K_OHM_PULL_UP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PUS_0_100K_OHM_PULL_DOWN`"]
    #[inline]
    pub fn is_pus_0_100k_ohm_pull_down(&self) -> bool {
        *self == PUSR::PUS_0_100K_OHM_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PUS_1_47K_OHM_PULL_UP`"]
    #[inline]
    pub fn is_pus_1_47k_ohm_pull_up(&self) -> bool {
        *self == PUSR::PUS_1_47K_OHM_PULL_UP
    }
    #[doc = "Checks if the value of the field is `PUS_2_100K_OHM_PULL_UP`"]
    #[inline]
    pub fn is_pus_2_100k_ohm_pull_up(&self) -> bool {
        *self == PUSR::PUS_2_100K_OHM_PULL_UP
    }
    #[doc = "Checks if the value of the field is `PUS_3_22K_OHM_PULL_UP`"]
    #[inline]
    pub fn is_pus_3_22k_ohm_pull_up(&self) -> bool {
        *self == PUSR::PUS_3_22K_OHM_PULL_UP
    }
}
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED,
    #[doc = "Hysteresis Enabled"]
    HYS_1_HYSTERESIS_ENABLED,
}
impl HYSR {
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
            HYSR::HYS_0_HYSTERESIS_DISABLED => false,
            HYSR::HYS_1_HYSTERESIS_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSR {
        match value {
            false => HYSR::HYS_0_HYSTERESIS_DISABLED,
            true => HYSR::HYS_1_HYSTERESIS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `HYS_0_HYSTERESIS_DISABLED`"]
    #[inline]
    pub fn is_hys_0_hysteresis_disabled(&self) -> bool {
        *self == HYSR::HYS_0_HYSTERESIS_DISABLED
    }
    #[doc = "Checks if the value of the field is `HYS_1_HYSTERESIS_ENABLED`"]
    #[inline]
    pub fn is_hys_1_hysteresis_enabled(&self) -> bool {
        *self == HYSR::HYS_1_HYSTERESIS_ENABLED
    }
}
#[doc = "Values that can be written to the field `SRE`"]
pub enum SREW {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE,
}
impl SREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SREW::SRE_0_SLOW_SLEW_RATE => false,
            SREW::SRE_1_FAST_SLEW_RATE => true,
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
    #[doc = "Slow Slew Rate"]
    #[inline]
    pub fn sre_0_slow_slew_rate(self) -> &'a mut W {
        self.variant(SREW::SRE_0_SLOW_SLEW_RATE)
    }
    #[doc = "Fast Slew Rate"]
    #[inline]
    pub fn sre_1_fast_slew_rate(self) -> &'a mut W {
        self.variant(SREW::SRE_1_FAST_SLEW_RATE)
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
#[doc = "Values that can be written to the field `DSE`"]
pub enum DSEW {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED,
    #[doc = "R0(260 Ohm @ 3.3V, 150 Ohm@1.8V, 240 Ohm for DDR)"]
    DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR,
    #[doc = "R0/2"]
    DSE_2_R0_2,
    #[doc = "R0/3"]
    DSE_3_R0_3,
    #[doc = "R0/4"]
    DSE_4_R0_4,
    #[doc = "R0/5"]
    DSE_5_R0_5,
    #[doc = "R0/6"]
    DSE_6_R0_6,
    #[doc = "R0/7"]
    DSE_7_R0_7,
}
impl DSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSEW::DSE_0_OUTPUT_DRIVER_DISABLED => 0,
            DSEW::DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR => 1,
            DSEW::DSE_2_R0_2 => 2,
            DSEW::DSE_3_R0_3 => 3,
            DSEW::DSE_4_R0_4 => 4,
            DSEW::DSE_5_R0_5 => 5,
            DSEW::DSE_6_R0_6 => 6,
            DSEW::DSE_7_R0_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "output driver disabled;"]
    #[inline]
    pub fn dse_0_output_driver_disabled(self) -> &'a mut W {
        self.variant(DSEW::DSE_0_OUTPUT_DRIVER_DISABLED)
    }
    #[doc = "R0(260 Ohm @ 3.3V, 150 Ohm@1.8V, 240 Ohm for DDR)"]
    #[inline]
    pub fn dse_1_r0_260_ohm_3_3v_150_ohm_1_8v_240_ohm_for_ddr(self) -> &'a mut W {
        self.variant(DSEW::DSE_1_R0_260_OHM_3_3V_150_OHM_1_8V_240_OHM_FOR_DDR)
    }
    #[doc = "R0/2"]
    #[inline]
    pub fn dse_2_r0_2(self) -> &'a mut W {
        self.variant(DSEW::DSE_2_R0_2)
    }
    #[doc = "R0/3"]
    #[inline]
    pub fn dse_3_r0_3(self) -> &'a mut W {
        self.variant(DSEW::DSE_3_R0_3)
    }
    #[doc = "R0/4"]
    #[inline]
    pub fn dse_4_r0_4(self) -> &'a mut W {
        self.variant(DSEW::DSE_4_R0_4)
    }
    #[doc = "R0/5"]
    #[inline]
    pub fn dse_5_r0_5(self) -> &'a mut W {
        self.variant(DSEW::DSE_5_R0_5)
    }
    #[doc = "R0/6"]
    #[inline]
    pub fn dse_6_r0_6(self) -> &'a mut W {
        self.variant(DSEW::DSE_6_R0_6)
    }
    #[doc = "R0/7"]
    #[inline]
    pub fn dse_7_r0_7(self) -> &'a mut W {
        self.variant(DSEW::DSE_7_R0_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPEED`"]
pub enum SPEEDW {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ,
    #[doc = "medium(100MHz)"]
    SPEED_2_MEDIUM_100MHZ,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ,
}
impl SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPEEDW::SPEED_0_LOW_50MHZ => 0,
            SPEEDW::SPEED_1_MEDIUM_100MHZ => 1,
            SPEEDW::SPEED_2_MEDIUM_100MHZ => 2,
            SPEEDW::SPEED_3_MAX_200MHZ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEEDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "low(50MHz)"]
    #[inline]
    pub fn speed_0_low_50mhz(self) -> &'a mut W {
        self.variant(SPEEDW::SPEED_0_LOW_50MHZ)
    }
    #[doc = "medium(100MHz)"]
    #[inline]
    pub fn speed_1_medium_100mhz(self) -> &'a mut W {
        self.variant(SPEEDW::SPEED_1_MEDIUM_100MHZ)
    }
    #[doc = "medium(100MHz)"]
    #[inline]
    pub fn speed_2_medium_100mhz(self) -> &'a mut W {
        self.variant(SPEEDW::SPEED_2_MEDIUM_100MHZ)
    }
    #[doc = "max(200MHz)"]
    #[inline]
    pub fn speed_3_max_200mhz(self) -> &'a mut W {
        self.variant(SPEEDW::SPEED_3_MAX_200MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ODE`"]
pub enum ODEW {
    #[doc = "Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED,
    #[doc = "Open Drain Enabled"]
    ODE_1_OPEN_DRAIN_ENABLED,
}
impl ODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODEW::ODE_0_OPEN_DRAIN_DISABLED => false,
            ODEW::ODE_1_OPEN_DRAIN_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Open Drain Disabled"]
    #[inline]
    pub fn ode_0_open_drain_disabled(self) -> &'a mut W {
        self.variant(ODEW::ODE_0_OPEN_DRAIN_DISABLED)
    }
    #[doc = "Open Drain Enabled"]
    #[inline]
    pub fn ode_1_open_drain_enabled(self) -> &'a mut W {
        self.variant(ODEW::ODE_1_OPEN_DRAIN_ENABLED)
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
#[doc = "Values that can be written to the field `PKE`"]
pub enum PKEW {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED,
}
impl PKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PKEW::PKE_0_PULL_KEEPER_DISABLED => false,
            PKEW::PKE_1_PULL_KEEPER_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pull/Keeper Disabled"]
    #[inline]
    pub fn pke_0_pull_keeper_disabled(self) -> &'a mut W {
        self.variant(PKEW::PKE_0_PULL_KEEPER_DISABLED)
    }
    #[doc = "Pull/Keeper Enabled"]
    #[inline]
    pub fn pke_1_pull_keeper_enabled(self) -> &'a mut W {
        self.variant(PKEW::PKE_1_PULL_KEEPER_ENABLED)
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
#[doc = "Values that can be written to the field `PUE`"]
pub enum PUEW {
    #[doc = "Keeper"]
    PUE_0_KEEPER,
    #[doc = "Pull"]
    PUE_1_PULL,
}
impl PUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUEW::PUE_0_KEEPER => false,
            PUEW::PUE_1_PULL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUEW<'a> {
    w: &'a mut W,
}
impl<'a> _PUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Keeper"]
    #[inline]
    pub fn pue_0_keeper(self) -> &'a mut W {
        self.variant(PUEW::PUE_0_KEEPER)
    }
    #[doc = "Pull"]
    #[inline]
    pub fn pue_1_pull(self) -> &'a mut W {
        self.variant(PUEW::PUE_1_PULL)
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
#[doc = "Values that can be written to the field `PUS`"]
pub enum PUSW {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP,
}
impl PUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PUSW::PUS_0_100K_OHM_PULL_DOWN => 0,
            PUSW::PUS_1_47K_OHM_PULL_UP => 1,
            PUSW::PUS_2_100K_OHM_PULL_UP => 2,
            PUSW::PUS_3_22K_OHM_PULL_UP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUSW<'a> {
    w: &'a mut W,
}
impl<'a> _PUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "100K Ohm Pull Down"]
    #[inline]
    pub fn pus_0_100k_ohm_pull_down(self) -> &'a mut W {
        self.variant(PUSW::PUS_0_100K_OHM_PULL_DOWN)
    }
    #[doc = "47K Ohm Pull Up"]
    #[inline]
    pub fn pus_1_47k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUSW::PUS_1_47K_OHM_PULL_UP)
    }
    #[doc = "100K Ohm Pull Up"]
    #[inline]
    pub fn pus_2_100k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUSW::PUS_2_100K_OHM_PULL_UP)
    }
    #[doc = "22K Ohm Pull Up"]
    #[inline]
    pub fn pus_3_22k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUSW::PUS_3_22K_OHM_PULL_UP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HYS`"]
pub enum HYSW {
    #[doc = "Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED,
    #[doc = "Hysteresis Enabled"]
    HYS_1_HYSTERESIS_ENABLED,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSW::HYS_0_HYSTERESIS_DISABLED => false,
            HYSW::HYS_1_HYSTERESIS_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hysteresis Disabled"]
    #[inline]
    pub fn hys_0_hysteresis_disabled(self) -> &'a mut W {
        self.variant(HYSW::HYS_0_HYSTERESIS_DISABLED)
    }
    #[doc = "Hysteresis Enabled"]
    #[inline]
    pub fn hys_1_hysteresis_enabled(self) -> &'a mut W {
        self.variant(HYSW::HYS_1_HYSTERESIS_ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Slew Rate Field"]
    #[inline]
    pub fn sre(&self) -> SRER {
        SRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Drive Strength Field"]
    #[inline]
    pub fn dse(&self) -> DSER {
        DSER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Speed Field"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Open Drain Enable Field"]
    #[inline]
    pub fn ode(&self) -> ODER {
        ODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Pull / Keep Enable Field"]
    #[inline]
    pub fn pke(&self) -> PKER {
        PKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Pull / Keep Select Field"]
    #[inline]
    pub fn pue(&self) -> PUER {
        PUER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Pull Up / Down Config. Field"]
    #[inline]
    pub fn pus(&self) -> PUSR {
        PUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Hyst. Enable Field"]
    #[inline]
    pub fn hys(&self) -> HYSR {
        HYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4272 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Slew Rate Field"]
    #[inline]
    pub fn sre(&mut self) -> _SREW {
        _SREW { w: self }
    }
    #[doc = "Bits 3:5 - Drive Strength Field"]
    #[inline]
    pub fn dse(&mut self) -> _DSEW {
        _DSEW { w: self }
    }
    #[doc = "Bits 6:7 - Speed Field"]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
    #[doc = "Bit 11 - Open Drain Enable Field"]
    #[inline]
    pub fn ode(&mut self) -> _ODEW {
        _ODEW { w: self }
    }
    #[doc = "Bit 12 - Pull / Keep Enable Field"]
    #[inline]
    pub fn pke(&mut self) -> _PKEW {
        _PKEW { w: self }
    }
    #[doc = "Bit 13 - Pull / Keep Select Field"]
    #[inline]
    pub fn pue(&mut self) -> _PUEW {
        _PUEW { w: self }
    }
    #[doc = "Bits 14:15 - Pull Up / Down Config. Field"]
    #[inline]
    pub fn pus(&mut self) -> _PUSW {
        _PUSW { w: self }
    }
    #[doc = "Bit 16 - Hyst. Enable Field"]
    #[inline]
    pub fn hys(&mut self) -> _HYSW {
        _HYSW { w: self }
    }
}
