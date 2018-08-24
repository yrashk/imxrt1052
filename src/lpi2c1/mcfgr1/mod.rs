#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCFGR1 {
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
#[doc = "Possible values of the field `PRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER {
    #[doc = "Divide by 1"]
    PRESCALE_0,
    #[doc = "Divide by 2"]
    PRESCALE_1,
    #[doc = "Divide by 4"]
    PRESCALE_2,
    #[doc = "Divide by 8"]
    PRESCALE_3,
    #[doc = "Divide by 16"]
    PRESCALE_4,
    #[doc = "Divide by 32"]
    PRESCALE_5,
    #[doc = "Divide by 64"]
    PRESCALE_6,
    #[doc = "Divide by 128"]
    PRESCALE_7,
}
impl PRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER::PRESCALE_0 => 0,
            PRESCALER::PRESCALE_1 => 1,
            PRESCALER::PRESCALE_2 => 2,
            PRESCALER::PRESCALE_3 => 3,
            PRESCALER::PRESCALE_4 => 4,
            PRESCALER::PRESCALE_5 => 5,
            PRESCALER::PRESCALE_6 => 6,
            PRESCALER::PRESCALE_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER {
        match value {
            0 => PRESCALER::PRESCALE_0,
            1 => PRESCALER::PRESCALE_1,
            2 => PRESCALER::PRESCALE_2,
            3 => PRESCALER::PRESCALE_3,
            4 => PRESCALER::PRESCALE_4,
            5 => PRESCALER::PRESCALE_5,
            6 => PRESCALER::PRESCALE_6,
            7 => PRESCALER::PRESCALE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline]
    pub fn is_prescale_0(&self) -> bool {
        *self == PRESCALER::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline]
    pub fn is_prescale_1(&self) -> bool {
        *self == PRESCALER::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline]
    pub fn is_prescale_2(&self) -> bool {
        *self == PRESCALER::PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PRESCALE_3`"]
    #[inline]
    pub fn is_prescale_3(&self) -> bool {
        *self == PRESCALER::PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `PRESCALE_4`"]
    #[inline]
    pub fn is_prescale_4(&self) -> bool {
        *self == PRESCALER::PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `PRESCALE_5`"]
    #[inline]
    pub fn is_prescale_5(&self) -> bool {
        *self == PRESCALER::PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `PRESCALE_6`"]
    #[inline]
    pub fn is_prescale_6(&self) -> bool {
        *self == PRESCALER::PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `PRESCALE_7`"]
    #[inline]
    pub fn is_prescale_7(&self) -> bool {
        *self == PRESCALER::PRESCALE_7
    }
}
#[doc = "Possible values of the field `AUTOSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOSTOPR {
    #[doc = "No effect"]
    AUTOSTOP_0,
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
    AUTOSTOP_1,
}
impl AUTOSTOPR {
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
            AUTOSTOPR::AUTOSTOP_0 => false,
            AUTOSTOPR::AUTOSTOP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOSTOPR {
        match value {
            false => AUTOSTOPR::AUTOSTOP_0,
            true => AUTOSTOPR::AUTOSTOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOSTOP_0`"]
    #[inline]
    pub fn is_autostop_0(&self) -> bool {
        *self == AUTOSTOPR::AUTOSTOP_0
    }
    #[doc = "Checks if the value of the field is `AUTOSTOP_1`"]
    #[inline]
    pub fn is_autostop_1(&self) -> bool {
        *self == AUTOSTOPR::AUTOSTOP_1
    }
}
#[doc = "Possible values of the field `IGNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACKR {
    #[doc = "LPI2C Master will receive ACK and NACK normally"]
    IGNACK_0,
    #[doc = "LPI2C Master will treat a received NACK as if it (NACK) was an ACK"]
    IGNACK_1,
}
impl IGNACKR {
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
            IGNACKR::IGNACK_0 => false,
            IGNACKR::IGNACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IGNACKR {
        match value {
            false => IGNACKR::IGNACK_0,
            true => IGNACKR::IGNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `IGNACK_0`"]
    #[inline]
    pub fn is_ignack_0(&self) -> bool {
        *self == IGNACKR::IGNACK_0
    }
    #[doc = "Checks if the value of the field is `IGNACK_1`"]
    #[inline]
    pub fn is_ignack_1(&self) -> bool {
        *self == IGNACKR::IGNACK_1
    }
}
#[doc = "Possible values of the field `TIMECFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMECFGR {
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout"]
    TIMECFG_0,
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout"]
    TIMECFG_1,
}
impl TIMECFGR {
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
            TIMECFGR::TIMECFG_0 => false,
            TIMECFGR::TIMECFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMECFGR {
        match value {
            false => TIMECFGR::TIMECFG_0,
            true => TIMECFGR::TIMECFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIMECFG_0`"]
    #[inline]
    pub fn is_timecfg_0(&self) -> bool {
        *self == TIMECFGR::TIMECFG_0
    }
    #[doc = "Checks if the value of the field is `TIMECFG_1`"]
    #[inline]
    pub fn is_timecfg_1(&self) -> bool {
        *self == TIMECFGR::TIMECFG_1
    }
}
#[doc = "Possible values of the field `MATCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCFGR {
    #[doc = "Match is disabled"]
    MATCFG_0,
    #[doc = "Match is enabled (1st data word equals MATCH0 OR MATCH1)"]
    MATCFG_2,
    #[doc = "Match is enabled (any data word equals MATCH0 OR MATCH1)"]
    MATCFG_3,
    #[doc = "Match is enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)"]
    MATCFG_4,
    #[doc = "Match is enabled (any data word equals MATCH0 AND next data word equals MATCH1)"]
    MATCFG_5,
    #[doc = "Match is enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    MATCFG_6,
    #[doc = "Match is enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    MATCFG_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MATCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MATCFGR::MATCFG_0 => 0,
            MATCFGR::MATCFG_2 => 2,
            MATCFGR::MATCFG_3 => 3,
            MATCFGR::MATCFG_4 => 4,
            MATCFGR::MATCFG_5 => 5,
            MATCFGR::MATCFG_6 => 6,
            MATCFGR::MATCFG_7 => 7,
            MATCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MATCFGR {
        match value {
            0 => MATCFGR::MATCFG_0,
            2 => MATCFGR::MATCFG_2,
            3 => MATCFGR::MATCFG_3,
            4 => MATCFGR::MATCFG_4,
            5 => MATCFGR::MATCFG_5,
            6 => MATCFGR::MATCFG_6,
            7 => MATCFGR::MATCFG_7,
            i => MATCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MATCFG_0`"]
    #[inline]
    pub fn is_matcfg_0(&self) -> bool {
        *self == MATCFGR::MATCFG_0
    }
    #[doc = "Checks if the value of the field is `MATCFG_2`"]
    #[inline]
    pub fn is_matcfg_2(&self) -> bool {
        *self == MATCFGR::MATCFG_2
    }
    #[doc = "Checks if the value of the field is `MATCFG_3`"]
    #[inline]
    pub fn is_matcfg_3(&self) -> bool {
        *self == MATCFGR::MATCFG_3
    }
    #[doc = "Checks if the value of the field is `MATCFG_4`"]
    #[inline]
    pub fn is_matcfg_4(&self) -> bool {
        *self == MATCFGR::MATCFG_4
    }
    #[doc = "Checks if the value of the field is `MATCFG_5`"]
    #[inline]
    pub fn is_matcfg_5(&self) -> bool {
        *self == MATCFGR::MATCFG_5
    }
    #[doc = "Checks if the value of the field is `MATCFG_6`"]
    #[inline]
    pub fn is_matcfg_6(&self) -> bool {
        *self == MATCFGR::MATCFG_6
    }
    #[doc = "Checks if the value of the field is `MATCFG_7`"]
    #[inline]
    pub fn is_matcfg_7(&self) -> bool {
        *self == MATCFGR::MATCFG_7
    }
}
#[doc = "Possible values of the field `PINCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCFGR {
    #[doc = "2-pin open drain mode"]
    PINCFG_0,
    #[doc = "2-pin output only mode (ultra-fast mode)"]
    PINCFG_1,
    #[doc = "2-pin push-pull mode"]
    PINCFG_2,
    #[doc = "4-pin push-pull mode"]
    PINCFG_3,
    #[doc = "2-pin open drain mode with separate LPI2C slave"]
    PINCFG_4,
    #[doc = "2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    PINCFG_5,
    #[doc = "2-pin push-pull mode with separate LPI2C slave"]
    PINCFG_6,
    #[doc = "4-pin push-pull mode (inverted outputs)"]
    PINCFG_7,
}
impl PINCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINCFGR::PINCFG_0 => 0,
            PINCFGR::PINCFG_1 => 1,
            PINCFGR::PINCFG_2 => 2,
            PINCFGR::PINCFG_3 => 3,
            PINCFGR::PINCFG_4 => 4,
            PINCFGR::PINCFG_5 => 5,
            PINCFGR::PINCFG_6 => 6,
            PINCFGR::PINCFG_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINCFGR {
        match value {
            0 => PINCFGR::PINCFG_0,
            1 => PINCFGR::PINCFG_1,
            2 => PINCFGR::PINCFG_2,
            3 => PINCFGR::PINCFG_3,
            4 => PINCFGR::PINCFG_4,
            5 => PINCFGR::PINCFG_5,
            6 => PINCFGR::PINCFG_6,
            7 => PINCFGR::PINCFG_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PINCFG_0`"]
    #[inline]
    pub fn is_pincfg_0(&self) -> bool {
        *self == PINCFGR::PINCFG_0
    }
    #[doc = "Checks if the value of the field is `PINCFG_1`"]
    #[inline]
    pub fn is_pincfg_1(&self) -> bool {
        *self == PINCFGR::PINCFG_1
    }
    #[doc = "Checks if the value of the field is `PINCFG_2`"]
    #[inline]
    pub fn is_pincfg_2(&self) -> bool {
        *self == PINCFGR::PINCFG_2
    }
    #[doc = "Checks if the value of the field is `PINCFG_3`"]
    #[inline]
    pub fn is_pincfg_3(&self) -> bool {
        *self == PINCFGR::PINCFG_3
    }
    #[doc = "Checks if the value of the field is `PINCFG_4`"]
    #[inline]
    pub fn is_pincfg_4(&self) -> bool {
        *self == PINCFGR::PINCFG_4
    }
    #[doc = "Checks if the value of the field is `PINCFG_5`"]
    #[inline]
    pub fn is_pincfg_5(&self) -> bool {
        *self == PINCFGR::PINCFG_5
    }
    #[doc = "Checks if the value of the field is `PINCFG_6`"]
    #[inline]
    pub fn is_pincfg_6(&self) -> bool {
        *self == PINCFGR::PINCFG_6
    }
    #[doc = "Checks if the value of the field is `PINCFG_7`"]
    #[inline]
    pub fn is_pincfg_7(&self) -> bool {
        *self == PINCFGR::PINCFG_7
    }
}
#[doc = "Values that can be written to the field `PRESCALE`"]
pub enum PRESCALEW {
    #[doc = "Divide by 1"]
    PRESCALE_0,
    #[doc = "Divide by 2"]
    PRESCALE_1,
    #[doc = "Divide by 4"]
    PRESCALE_2,
    #[doc = "Divide by 8"]
    PRESCALE_3,
    #[doc = "Divide by 16"]
    PRESCALE_4,
    #[doc = "Divide by 32"]
    PRESCALE_5,
    #[doc = "Divide by 64"]
    PRESCALE_6,
    #[doc = "Divide by 128"]
    PRESCALE_7,
}
impl PRESCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALEW::PRESCALE_0 => 0,
            PRESCALEW::PRESCALE_1 => 1,
            PRESCALEW::PRESCALE_2 => 2,
            PRESCALEW::PRESCALE_3 => 3,
            PRESCALEW::PRESCALE_4 => 4,
            PRESCALEW::PRESCALE_5 => 5,
            PRESCALEW::PRESCALE_6 => 6,
            PRESCALEW::PRESCALE_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_1)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_2)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn prescale_3(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_3)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn prescale_4(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_4)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn prescale_5(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_5)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn prescale_6(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_6)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn prescale_7(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTOSTOP`"]
pub enum AUTOSTOPW {
    #[doc = "No effect"]
    AUTOSTOP_0,
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
    AUTOSTOP_1,
}
impl AUTOSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOSTOPW::AUTOSTOP_0 => false,
            AUTOSTOPW::AUTOSTOP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn autostop_0(self) -> &'a mut W {
        self.variant(AUTOSTOPW::AUTOSTOP_0)
    }
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
    #[inline]
    pub fn autostop_1(self) -> &'a mut W {
        self.variant(AUTOSTOPW::AUTOSTOP_1)
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
#[doc = "Values that can be written to the field `IGNACK`"]
pub enum IGNACKW {
    #[doc = "LPI2C Master will receive ACK and NACK normally"]
    IGNACK_0,
    #[doc = "LPI2C Master will treat a received NACK as if it (NACK) was an ACK"]
    IGNACK_1,
}
impl IGNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IGNACKW::IGNACK_0 => false,
            IGNACKW::IGNACK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IGNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IGNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPI2C Master will receive ACK and NACK normally"]
    #[inline]
    pub fn ignack_0(self) -> &'a mut W {
        self.variant(IGNACKW::IGNACK_0)
    }
    #[doc = "LPI2C Master will treat a received NACK as if it (NACK) was an ACK"]
    #[inline]
    pub fn ignack_1(self) -> &'a mut W {
        self.variant(IGNACKW::IGNACK_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMECFG`"]
pub enum TIMECFGW {
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout"]
    TIMECFG_0,
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout"]
    TIMECFG_1,
}
impl TIMECFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMECFGW::TIMECFG_0 => false,
            TIMECFGW::TIMECFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMECFGW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMECFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMECFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout"]
    #[inline]
    pub fn timecfg_0(self) -> &'a mut W {
        self.variant(TIMECFGW::TIMECFG_0)
    }
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout"]
    #[inline]
    pub fn timecfg_1(self) -> &'a mut W {
        self.variant(TIMECFGW::TIMECFG_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MATCFG`"]
pub enum MATCFGW {
    #[doc = "Match is disabled"]
    MATCFG_0,
    #[doc = "Match is enabled (1st data word equals MATCH0 OR MATCH1)"]
    MATCFG_2,
    #[doc = "Match is enabled (any data word equals MATCH0 OR MATCH1)"]
    MATCFG_3,
    #[doc = "Match is enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)"]
    MATCFG_4,
    #[doc = "Match is enabled (any data word equals MATCH0 AND next data word equals MATCH1)"]
    MATCFG_5,
    #[doc = "Match is enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    MATCFG_6,
    #[doc = "Match is enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    MATCFG_7,
}
impl MATCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MATCFGW::MATCFG_0 => 0,
            MATCFGW::MATCFG_2 => 2,
            MATCFGW::MATCFG_3 => 3,
            MATCFGW::MATCFG_4 => 4,
            MATCFGW::MATCFG_5 => 5,
            MATCFGW::MATCFG_6 => 6,
            MATCFGW::MATCFG_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Match is disabled"]
    #[inline]
    pub fn matcfg_0(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_0)
    }
    #[doc = "Match is enabled (1st data word equals MATCH0 OR MATCH1)"]
    #[inline]
    pub fn matcfg_2(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_2)
    }
    #[doc = "Match is enabled (any data word equals MATCH0 OR MATCH1)"]
    #[inline]
    pub fn matcfg_3(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_3)
    }
    #[doc = "Match is enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)"]
    #[inline]
    pub fn matcfg_4(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_4)
    }
    #[doc = "Match is enabled (any data word equals MATCH0 AND next data word equals MATCH1)"]
    #[inline]
    pub fn matcfg_5(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_5)
    }
    #[doc = "Match is enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    #[inline]
    pub fn matcfg_6(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_6)
    }
    #[doc = "Match is enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    #[inline]
    pub fn matcfg_7(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINCFG`"]
pub enum PINCFGW {
    #[doc = "2-pin open drain mode"]
    PINCFG_0,
    #[doc = "2-pin output only mode (ultra-fast mode)"]
    PINCFG_1,
    #[doc = "2-pin push-pull mode"]
    PINCFG_2,
    #[doc = "4-pin push-pull mode"]
    PINCFG_3,
    #[doc = "2-pin open drain mode with separate LPI2C slave"]
    PINCFG_4,
    #[doc = "2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    PINCFG_5,
    #[doc = "2-pin push-pull mode with separate LPI2C slave"]
    PINCFG_6,
    #[doc = "4-pin push-pull mode (inverted outputs)"]
    PINCFG_7,
}
impl PINCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINCFGW::PINCFG_0 => 0,
            PINCFGW::PINCFG_1 => 1,
            PINCFGW::PINCFG_2 => 2,
            PINCFGW::PINCFG_3 => 3,
            PINCFGW::PINCFG_4 => 4,
            PINCFGW::PINCFG_5 => 5,
            PINCFGW::PINCFG_6 => 6,
            PINCFGW::PINCFG_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2-pin open drain mode"]
    #[inline]
    pub fn pincfg_0(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_0)
    }
    #[doc = "2-pin output only mode (ultra-fast mode)"]
    #[inline]
    pub fn pincfg_1(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_1)
    }
    #[doc = "2-pin push-pull mode"]
    #[inline]
    pub fn pincfg_2(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_2)
    }
    #[doc = "4-pin push-pull mode"]
    #[inline]
    pub fn pincfg_3(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_3)
    }
    #[doc = "2-pin open drain mode with separate LPI2C slave"]
    #[inline]
    pub fn pincfg_4(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_4)
    }
    #[doc = "2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    #[inline]
    pub fn pincfg_5(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_5)
    }
    #[doc = "2-pin push-pull mode with separate LPI2C slave"]
    #[inline]
    pub fn pincfg_6(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_6)
    }
    #[doc = "4-pin push-pull mode (inverted outputs)"]
    #[inline]
    pub fn pincfg_7(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline]
    pub fn prescale(&self) -> PRESCALER {
        PRESCALER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline]
    pub fn autostop(&self) -> AUTOSTOPR {
        AUTOSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline]
    pub fn ignack(&self) -> IGNACKR {
        IGNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline]
    pub fn timecfg(&self) -> TIMECFGR {
        TIMECFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&self) -> MATCFGR {
        MATCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&self) -> PINCFGR {
        PINCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline]
    pub fn prescale(&mut self) -> _PRESCALEW {
        _PRESCALEW { w: self }
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline]
    pub fn autostop(&mut self) -> _AUTOSTOPW {
        _AUTOSTOPW { w: self }
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline]
    pub fn ignack(&mut self) -> _IGNACKW {
        _IGNACKW { w: self }
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline]
    pub fn timecfg(&mut self) -> _TIMECFGW {
        _TIMECFGW { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&mut self) -> _MATCFGW {
        _MATCFGW { w: self }
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&mut self) -> _PINCFGW {
        _PINCFGW { w: self }
    }
}
