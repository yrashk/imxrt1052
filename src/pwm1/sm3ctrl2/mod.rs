#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM3CTRL2 {
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
#[doc = "Possible values of the field `CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SELR {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    CLK_SEL_0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    CLK_SEL_1,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    CLK_SEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_SELR::CLK_SEL_0 => 0,
            CLK_SELR::CLK_SEL_1 => 1,
            CLK_SELR::CLK_SEL_2 => 2,
            CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_SELR {
        match value {
            0 => CLK_SELR::CLK_SEL_0,
            1 => CLK_SELR::CLK_SEL_1,
            2 => CLK_SELR::CLK_SEL_2,
            i => CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_SEL_0`"]
    #[inline]
    pub fn is_clk_sel_0(&self) -> bool {
        *self == CLK_SELR::CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLK_SEL_1`"]
    #[inline]
    pub fn is_clk_sel_1(&self) -> bool {
        *self == CLK_SELR::CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `CLK_SEL_2`"]
    #[inline]
    pub fn is_clk_sel_2(&self) -> bool {
        *self == CLK_SELR::CLK_SEL_2
    }
}
#[doc = "Possible values of the field `RELOAD_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOAD_SELR {
    #[doc = "The local RELOAD signal is used to reload registers."]
    RELOAD_SEL_0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
    RELOAD_SEL_1,
}
impl RELOAD_SELR {
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
            RELOAD_SELR::RELOAD_SEL_0 => false,
            RELOAD_SELR::RELOAD_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RELOAD_SELR {
        match value {
            false => RELOAD_SELR::RELOAD_SEL_0,
            true => RELOAD_SELR::RELOAD_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD_SEL_0`"]
    #[inline]
    pub fn is_reload_sel_0(&self) -> bool {
        *self == RELOAD_SELR::RELOAD_SEL_0
    }
    #[doc = "Checks if the value of the field is `RELOAD_SEL_1`"]
    #[inline]
    pub fn is_reload_sel_1(&self) -> bool {
        *self == RELOAD_SELR::RELOAD_SEL_1
    }
}
#[doc = "Possible values of the field `FORCE_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_SELR {
    #[doc = "The local force signal, CTRL2[FORCE], from this submodule is used to force updates."]
    FORCE_SEL_0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_1,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    FORCE_SEL_2,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_3,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    FORCE_SEL_4,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_5,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    FORCE_SEL_6,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    FORCE_SEL_7,
}
impl FORCE_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORCE_SELR::FORCE_SEL_0 => 0,
            FORCE_SELR::FORCE_SEL_1 => 1,
            FORCE_SELR::FORCE_SEL_2 => 2,
            FORCE_SELR::FORCE_SEL_3 => 3,
            FORCE_SELR::FORCE_SEL_4 => 4,
            FORCE_SELR::FORCE_SEL_5 => 5,
            FORCE_SELR::FORCE_SEL_6 => 6,
            FORCE_SELR::FORCE_SEL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORCE_SELR {
        match value {
            0 => FORCE_SELR::FORCE_SEL_0,
            1 => FORCE_SELR::FORCE_SEL_1,
            2 => FORCE_SELR::FORCE_SEL_2,
            3 => FORCE_SELR::FORCE_SEL_3,
            4 => FORCE_SELR::FORCE_SEL_4,
            5 => FORCE_SELR::FORCE_SEL_5,
            6 => FORCE_SELR::FORCE_SEL_6,
            7 => FORCE_SELR::FORCE_SEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_0`"]
    #[inline]
    pub fn is_force_sel_0(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_0
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_1`"]
    #[inline]
    pub fn is_force_sel_1(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_1
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_2`"]
    #[inline]
    pub fn is_force_sel_2(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_2
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_3`"]
    #[inline]
    pub fn is_force_sel_3(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_3
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_4`"]
    #[inline]
    pub fn is_force_sel_4(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_4
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_5`"]
    #[inline]
    pub fn is_force_sel_5(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_5
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_6`"]
    #[inline]
    pub fn is_force_sel_6(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_6
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_7`"]
    #[inline]
    pub fn is_force_sel_7(&self) -> bool {
        *self == FORCE_SELR::FORCE_SEL_7
    }
}
#[doc = "Possible values of the field `FRCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRCENR {
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    FRCEN_0,
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    FRCEN_1,
}
impl FRCENR {
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
            FRCENR::FRCEN_0 => false,
            FRCENR::FRCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRCENR {
        match value {
            false => FRCENR::FRCEN_0,
            true => FRCENR::FRCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRCEN_0`"]
    #[inline]
    pub fn is_frcen_0(&self) -> bool {
        *self == FRCENR::FRCEN_0
    }
    #[doc = "Checks if the value of the field is `FRCEN_1`"]
    #[inline]
    pub fn is_frcen_1(&self) -> bool {
        *self == FRCENR::FRCEN_1
    }
}
#[doc = "Possible values of the field `INIT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_SELR {
    #[doc = "Local sync (PWM_X) causes initialization."]
    INIT_SEL_0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    INIT_SEL_1,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    INIT_SEL_2,
    #[doc = "EXT_SYNC causes initialization."]
    INIT_SEL_3,
}
impl INIT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INIT_SELR::INIT_SEL_0 => 0,
            INIT_SELR::INIT_SEL_1 => 1,
            INIT_SELR::INIT_SEL_2 => 2,
            INIT_SELR::INIT_SEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INIT_SELR {
        match value {
            0 => INIT_SELR::INIT_SEL_0,
            1 => INIT_SELR::INIT_SEL_1,
            2 => INIT_SELR::INIT_SEL_2,
            3 => INIT_SELR::INIT_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_0`"]
    #[inline]
    pub fn is_init_sel_0(&self) -> bool {
        *self == INIT_SELR::INIT_SEL_0
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_1`"]
    #[inline]
    pub fn is_init_sel_1(&self) -> bool {
        *self == INIT_SELR::INIT_SEL_1
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_2`"]
    #[inline]
    pub fn is_init_sel_2(&self) -> bool {
        *self == INIT_SELR::INIT_SEL_2
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_3`"]
    #[inline]
    pub fn is_init_sel_3(&self) -> bool {
        *self == INIT_SELR::INIT_SEL_3
    }
}
#[doc = r" Value of the field"]
pub struct PWMX_INITR {
    bits: bool,
}
impl PWMX_INITR {
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
pub struct PWM45_INITR {
    bits: bool,
}
impl PWM45_INITR {
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
pub struct PWM23_INITR {
    bits: bool,
}
impl PWM23_INITR {
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
#[doc = "Possible values of the field `INDEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INDEPR {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    INDEP_0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEP_1,
}
impl INDEPR {
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
            INDEPR::INDEP_0 => false,
            INDEPR::INDEP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INDEPR {
        match value {
            false => INDEPR::INDEP_0,
            true => INDEPR::INDEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `INDEP_0`"]
    #[inline]
    pub fn is_indep_0(&self) -> bool {
        *self == INDEPR::INDEP_0
    }
    #[doc = "Checks if the value of the field is `INDEP_1`"]
    #[inline]
    pub fn is_indep_1(&self) -> bool {
        *self == INDEPR::INDEP_1
    }
}
#[doc = r" Value of the field"]
pub struct WAITENR {
    bits: bool,
}
impl WAITENR {
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
pub struct DBGENR {
    bits: bool,
}
impl DBGENR {
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
#[doc = "Values that can be written to the field `CLK_SEL`"]
pub enum CLK_SELW {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    CLK_SEL_0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    CLK_SEL_1,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    CLK_SEL_2,
}
impl CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_SELW::CLK_SEL_0 => 0,
            CLK_SELW::CLK_SEL_1 => 1,
            CLK_SELW::CLK_SEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    #[inline]
    pub fn clk_sel_0(self) -> &'a mut W {
        self.variant(CLK_SELW::CLK_SEL_0)
    }
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    #[inline]
    pub fn clk_sel_1(self) -> &'a mut W {
        self.variant(CLK_SELW::CLK_SEL_1)
    }
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    #[inline]
    pub fn clk_sel_2(self) -> &'a mut W {
        self.variant(CLK_SELW::CLK_SEL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RELOAD_SEL`"]
pub enum RELOAD_SELW {
    #[doc = "The local RELOAD signal is used to reload registers."]
    RELOAD_SEL_0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
    RELOAD_SEL_1,
}
impl RELOAD_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RELOAD_SELW::RELOAD_SEL_0 => false,
            RELOAD_SELW::RELOAD_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RELOAD_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOAD_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RELOAD_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The local RELOAD signal is used to reload registers."]
    #[inline]
    pub fn reload_sel_0(self) -> &'a mut W {
        self.variant(RELOAD_SELW::RELOAD_SEL_0)
    }
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
    #[inline]
    pub fn reload_sel_1(self) -> &'a mut W {
        self.variant(RELOAD_SELW::RELOAD_SEL_1)
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
#[doc = "Values that can be written to the field `FORCE_SEL`"]
pub enum FORCE_SELW {
    #[doc = "The local force signal, CTRL2[FORCE], from this submodule is used to force updates."]
    FORCE_SEL_0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_1,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    FORCE_SEL_2,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_3,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    FORCE_SEL_4,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_5,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    FORCE_SEL_6,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    FORCE_SEL_7,
}
impl FORCE_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORCE_SELW::FORCE_SEL_0 => 0,
            FORCE_SELW::FORCE_SEL_1 => 1,
            FORCE_SELW::FORCE_SEL_2 => 2,
            FORCE_SELW::FORCE_SEL_3 => 3,
            FORCE_SELW::FORCE_SEL_4 => 4,
            FORCE_SELW::FORCE_SEL_5 => 5,
            FORCE_SELW::FORCE_SEL_6 => 6,
            FORCE_SELW::FORCE_SEL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCE_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCE_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The local force signal, CTRL2[FORCE], from this submodule is used to force updates."]
    #[inline]
    pub fn force_sel_0(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_0)
    }
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline]
    pub fn force_sel_1(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_1)
    }
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    #[inline]
    pub fn force_sel_2(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_2)
    }
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline]
    pub fn force_sel_3(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_3)
    }
    #[doc = "The local sync signal from this submodule is used to force updates."]
    #[inline]
    pub fn force_sel_4(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_4)
    }
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline]
    pub fn force_sel_5(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_5)
    }
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    #[inline]
    pub fn force_sel_6(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_6)
    }
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    #[inline]
    pub fn force_sel_7(self) -> &'a mut W {
        self.variant(FORCE_SELW::FORCE_SEL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FORCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEW<'a> {
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
#[doc = "Values that can be written to the field `FRCEN`"]
pub enum FRCENW {
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    FRCEN_0,
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    FRCEN_1,
}
impl FRCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRCENW::FRCEN_0 => false,
            FRCENW::FRCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    #[inline]
    pub fn frcen_0(self) -> &'a mut W {
        self.variant(FRCENW::FRCEN_0)
    }
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    #[inline]
    pub fn frcen_1(self) -> &'a mut W {
        self.variant(FRCENW::FRCEN_1)
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
#[doc = "Values that can be written to the field `INIT_SEL`"]
pub enum INIT_SELW {
    #[doc = "Local sync (PWM_X) causes initialization."]
    INIT_SEL_0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    INIT_SEL_1,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    INIT_SEL_2,
    #[doc = "EXT_SYNC causes initialization."]
    INIT_SEL_3,
}
impl INIT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INIT_SELW::INIT_SEL_0 => 0,
            INIT_SELW::INIT_SEL_1 => 1,
            INIT_SELW::INIT_SEL_2 => 2,
            INIT_SELW::INIT_SEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INIT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INIT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INIT_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Local sync (PWM_X) causes initialization."]
    #[inline]
    pub fn init_sel_0(self) -> &'a mut W {
        self.variant(INIT_SELW::INIT_SEL_0)
    }
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    #[inline]
    pub fn init_sel_1(self) -> &'a mut W {
        self.variant(INIT_SELW::INIT_SEL_1)
    }
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    #[inline]
    pub fn init_sel_2(self) -> &'a mut W {
        self.variant(INIT_SELW::INIT_SEL_2)
    }
    #[doc = "EXT_SYNC causes initialization."]
    #[inline]
    pub fn init_sel_3(self) -> &'a mut W {
        self.variant(INIT_SELW::INIT_SEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWMX_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMX_INITW<'a> {
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
#[doc = r" Proxy"]
pub struct _PWM45_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM45_INITW<'a> {
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
#[doc = r" Proxy"]
pub struct _PWM23_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM23_INITW<'a> {
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
#[doc = "Values that can be written to the field `INDEP`"]
pub enum INDEPW {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    INDEP_0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEP_1,
}
impl INDEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INDEPW::INDEP_0 => false,
            INDEPW::INDEP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INDEPW<'a> {
    w: &'a mut W,
}
impl<'a> _INDEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INDEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    #[inline]
    pub fn indep_0(self) -> &'a mut W {
        self.variant(INDEPW::INDEP_0)
    }
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    #[inline]
    pub fn indep_1(self) -> &'a mut W {
        self.variant(INDEPW::INDEP_1)
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
#[doc = r" Proxy"]
pub struct _WAITENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITENW<'a> {
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
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
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
    #[doc = "Bits 0:1 - Clock Source Select"]
    #[inline]
    pub fn clk_sel(&self) -> CLK_SELR {
        CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 2 - Reload Source Select"]
    #[inline]
    pub fn reload_sel(&self) -> RELOAD_SELR {
        RELOAD_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 3:5 - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline]
    pub fn force_sel(&self) -> FORCE_SELR {
        FORCE_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 7 - FRCEN"]
    #[inline]
    pub fn frcen(&self) -> FRCENR {
        FRCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:9 - Initialization Control Select"]
    #[inline]
    pub fn init_sel(&self) -> INIT_SELR {
        INIT_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 10 - PWM_X Initial Value"]
    #[inline]
    pub fn pwmx_init(&self) -> PWMX_INITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PWMX_INITR { bits }
    }
    #[doc = "Bit 11 - PWM45 Initial Value"]
    #[inline]
    pub fn pwm45_init(&self) -> PWM45_INITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PWM45_INITR { bits }
    }
    #[doc = "Bit 12 - PWM23 Initial Value"]
    #[inline]
    pub fn pwm23_init(&self) -> PWM23_INITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PWM23_INITR { bits }
    }
    #[doc = "Bit 13 - Independent or Complementary Pair Operation"]
    #[inline]
    pub fn indep(&self) -> INDEPR {
        INDEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - WAIT Enable"]
    #[inline]
    pub fn waiten(&self) -> WAITENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        WAITENR { bits }
    }
    #[doc = "Bit 15 - Debug Enable"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DBGENR { bits }
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
    #[doc = "Bits 0:1 - Clock Source Select"]
    #[inline]
    pub fn clk_sel(&mut self) -> _CLK_SELW {
        _CLK_SELW { w: self }
    }
    #[doc = "Bit 2 - Reload Source Select"]
    #[inline]
    pub fn reload_sel(&mut self) -> _RELOAD_SELW {
        _RELOAD_SELW { w: self }
    }
    #[doc = "Bits 3:5 - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline]
    pub fn force_sel(&mut self) -> _FORCE_SELW {
        _FORCE_SELW { w: self }
    }
    #[doc = "Bit 6 - Force Initialization"]
    #[inline]
    pub fn force(&mut self) -> _FORCEW {
        _FORCEW { w: self }
    }
    #[doc = "Bit 7 - FRCEN"]
    #[inline]
    pub fn frcen(&mut self) -> _FRCENW {
        _FRCENW { w: self }
    }
    #[doc = "Bits 8:9 - Initialization Control Select"]
    #[inline]
    pub fn init_sel(&mut self) -> _INIT_SELW {
        _INIT_SELW { w: self }
    }
    #[doc = "Bit 10 - PWM_X Initial Value"]
    #[inline]
    pub fn pwmx_init(&mut self) -> _PWMX_INITW {
        _PWMX_INITW { w: self }
    }
    #[doc = "Bit 11 - PWM45 Initial Value"]
    #[inline]
    pub fn pwm45_init(&mut self) -> _PWM45_INITW {
        _PWM45_INITW { w: self }
    }
    #[doc = "Bit 12 - PWM23 Initial Value"]
    #[inline]
    pub fn pwm23_init(&mut self) -> _PWM23_INITW {
        _PWM23_INITW { w: self }
    }
    #[doc = "Bit 13 - Independent or Complementary Pair Operation"]
    #[inline]
    pub fn indep(&mut self) -> _INDEPW {
        _INDEPW { w: self }
    }
    #[doc = "Bit 14 - WAIT Enable"]
    #[inline]
    pub fn waiten(&mut self) -> _WAITENW {
        _WAITENW { w: self }
    }
    #[doc = "Bit 15 - Debug Enable"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
}
