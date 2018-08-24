#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCFG {
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
#[doc = "Possible values of the field `TSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTARTR {
    #[doc = "Start bit disabled"]
    TSTART_0,
    #[doc = "Start bit enabled"]
    TSTART_1,
}
impl TSTARTR {
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
            TSTARTR::TSTART_0 => false,
            TSTARTR::TSTART_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSTARTR {
        match value {
            false => TSTARTR::TSTART_0,
            true => TSTARTR::TSTART_1,
        }
    }
    #[doc = "Checks if the value of the field is `TSTART_0`"]
    #[inline]
    pub fn is_tstart_0(&self) -> bool {
        *self == TSTARTR::TSTART_0
    }
    #[doc = "Checks if the value of the field is `TSTART_1`"]
    #[inline]
    pub fn is_tstart_1(&self) -> bool {
        *self == TSTARTR::TSTART_1
    }
}
#[doc = "Possible values of the field `TSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTOPR {
    #[doc = "Stop bit disabled"]
    TSTOP_0,
    #[doc = "Stop bit is enabled on timer compare"]
    TSTOP_1,
    #[doc = "Stop bit is enabled on timer disable"]
    TSTOP_2,
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    TSTOP_3,
}
impl TSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTOPR::TSTOP_0 => 0,
            TSTOPR::TSTOP_1 => 1,
            TSTOPR::TSTOP_2 => 2,
            TSTOPR::TSTOP_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTOPR {
        match value {
            0 => TSTOPR::TSTOP_0,
            1 => TSTOPR::TSTOP_1,
            2 => TSTOPR::TSTOP_2,
            3 => TSTOPR::TSTOP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TSTOP_0`"]
    #[inline]
    pub fn is_tstop_0(&self) -> bool {
        *self == TSTOPR::TSTOP_0
    }
    #[doc = "Checks if the value of the field is `TSTOP_1`"]
    #[inline]
    pub fn is_tstop_1(&self) -> bool {
        *self == TSTOPR::TSTOP_1
    }
    #[doc = "Checks if the value of the field is `TSTOP_2`"]
    #[inline]
    pub fn is_tstop_2(&self) -> bool {
        *self == TSTOPR::TSTOP_2
    }
    #[doc = "Checks if the value of the field is `TSTOP_3`"]
    #[inline]
    pub fn is_tstop_3(&self) -> bool {
        *self == TSTOPR::TSTOP_3
    }
}
#[doc = "Possible values of the field `TIMENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMENAR {
    #[doc = "Timer always enabled"]
    TIMENA_0,
    #[doc = "Timer enabled on Timer N-1 enable"]
    TIMENA_1,
    #[doc = "Timer enabled on Trigger high"]
    TIMENA_2,
    #[doc = "Timer enabled on Trigger high and Pin high"]
    TIMENA_3,
    #[doc = "Timer enabled on Pin rising edge"]
    TIMENA_4,
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    TIMENA_5,
    #[doc = "Timer enabled on Trigger rising edge"]
    TIMENA_6,
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    TIMENA_7,
}
impl TIMENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMENAR::TIMENA_0 => 0,
            TIMENAR::TIMENA_1 => 1,
            TIMENAR::TIMENA_2 => 2,
            TIMENAR::TIMENA_3 => 3,
            TIMENAR::TIMENA_4 => 4,
            TIMENAR::TIMENA_5 => 5,
            TIMENAR::TIMENA_6 => 6,
            TIMENAR::TIMENA_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMENAR {
        match value {
            0 => TIMENAR::TIMENA_0,
            1 => TIMENAR::TIMENA_1,
            2 => TIMENAR::TIMENA_2,
            3 => TIMENAR::TIMENA_3,
            4 => TIMENAR::TIMENA_4,
            5 => TIMENAR::TIMENA_5,
            6 => TIMENAR::TIMENA_6,
            7 => TIMENAR::TIMENA_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMENA_0`"]
    #[inline]
    pub fn is_timena_0(&self) -> bool {
        *self == TIMENAR::TIMENA_0
    }
    #[doc = "Checks if the value of the field is `TIMENA_1`"]
    #[inline]
    pub fn is_timena_1(&self) -> bool {
        *self == TIMENAR::TIMENA_1
    }
    #[doc = "Checks if the value of the field is `TIMENA_2`"]
    #[inline]
    pub fn is_timena_2(&self) -> bool {
        *self == TIMENAR::TIMENA_2
    }
    #[doc = "Checks if the value of the field is `TIMENA_3`"]
    #[inline]
    pub fn is_timena_3(&self) -> bool {
        *self == TIMENAR::TIMENA_3
    }
    #[doc = "Checks if the value of the field is `TIMENA_4`"]
    #[inline]
    pub fn is_timena_4(&self) -> bool {
        *self == TIMENAR::TIMENA_4
    }
    #[doc = "Checks if the value of the field is `TIMENA_5`"]
    #[inline]
    pub fn is_timena_5(&self) -> bool {
        *self == TIMENAR::TIMENA_5
    }
    #[doc = "Checks if the value of the field is `TIMENA_6`"]
    #[inline]
    pub fn is_timena_6(&self) -> bool {
        *self == TIMENAR::TIMENA_6
    }
    #[doc = "Checks if the value of the field is `TIMENA_7`"]
    #[inline]
    pub fn is_timena_7(&self) -> bool {
        *self == TIMENAR::TIMENA_7
    }
}
#[doc = "Possible values of the field `TIMDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMDISR {
    #[doc = "Timer never disabled"]
    TIMDIS_0,
    #[doc = "Timer disabled on Timer N-1 disable"]
    TIMDIS_1,
    #[doc = "Timer disabled on Timer compare"]
    TIMDIS_2,
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    TIMDIS_3,
    #[doc = "Timer disabled on Pin rising or falling edge"]
    TIMDIS_4,
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    TIMDIS_5,
    #[doc = "Timer disabled on Trigger falling edge"]
    TIMDIS_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMDISR::TIMDIS_0 => 0,
            TIMDISR::TIMDIS_1 => 1,
            TIMDISR::TIMDIS_2 => 2,
            TIMDISR::TIMDIS_3 => 3,
            TIMDISR::TIMDIS_4 => 4,
            TIMDISR::TIMDIS_5 => 5,
            TIMDISR::TIMDIS_6 => 6,
            TIMDISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMDISR {
        match value {
            0 => TIMDISR::TIMDIS_0,
            1 => TIMDISR::TIMDIS_1,
            2 => TIMDISR::TIMDIS_2,
            3 => TIMDISR::TIMDIS_3,
            4 => TIMDISR::TIMDIS_4,
            5 => TIMDISR::TIMDIS_5,
            6 => TIMDISR::TIMDIS_6,
            i => TIMDISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMDIS_0`"]
    #[inline]
    pub fn is_timdis_0(&self) -> bool {
        *self == TIMDISR::TIMDIS_0
    }
    #[doc = "Checks if the value of the field is `TIMDIS_1`"]
    #[inline]
    pub fn is_timdis_1(&self) -> bool {
        *self == TIMDISR::TIMDIS_1
    }
    #[doc = "Checks if the value of the field is `TIMDIS_2`"]
    #[inline]
    pub fn is_timdis_2(&self) -> bool {
        *self == TIMDISR::TIMDIS_2
    }
    #[doc = "Checks if the value of the field is `TIMDIS_3`"]
    #[inline]
    pub fn is_timdis_3(&self) -> bool {
        *self == TIMDISR::TIMDIS_3
    }
    #[doc = "Checks if the value of the field is `TIMDIS_4`"]
    #[inline]
    pub fn is_timdis_4(&self) -> bool {
        *self == TIMDISR::TIMDIS_4
    }
    #[doc = "Checks if the value of the field is `TIMDIS_5`"]
    #[inline]
    pub fn is_timdis_5(&self) -> bool {
        *self == TIMDISR::TIMDIS_5
    }
    #[doc = "Checks if the value of the field is `TIMDIS_6`"]
    #[inline]
    pub fn is_timdis_6(&self) -> bool {
        *self == TIMDISR::TIMDIS_6
    }
}
#[doc = "Possible values of the field `TIMRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMRSTR {
    #[doc = "Timer never reset"]
    TIMRST_0,
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    TIMRST_2,
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    TIMRST_3,
    #[doc = "Timer reset on Timer Pin rising edge"]
    TIMRST_4,
    #[doc = "Timer reset on Trigger rising edge"]
    TIMRST_6,
    #[doc = "Timer reset on Trigger rising or falling edge"]
    TIMRST_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMRSTR::TIMRST_0 => 0,
            TIMRSTR::TIMRST_2 => 2,
            TIMRSTR::TIMRST_3 => 3,
            TIMRSTR::TIMRST_4 => 4,
            TIMRSTR::TIMRST_6 => 6,
            TIMRSTR::TIMRST_7 => 7,
            TIMRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMRSTR {
        match value {
            0 => TIMRSTR::TIMRST_0,
            2 => TIMRSTR::TIMRST_2,
            3 => TIMRSTR::TIMRST_3,
            4 => TIMRSTR::TIMRST_4,
            6 => TIMRSTR::TIMRST_6,
            7 => TIMRSTR::TIMRST_7,
            i => TIMRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMRST_0`"]
    #[inline]
    pub fn is_timrst_0(&self) -> bool {
        *self == TIMRSTR::TIMRST_0
    }
    #[doc = "Checks if the value of the field is `TIMRST_2`"]
    #[inline]
    pub fn is_timrst_2(&self) -> bool {
        *self == TIMRSTR::TIMRST_2
    }
    #[doc = "Checks if the value of the field is `TIMRST_3`"]
    #[inline]
    pub fn is_timrst_3(&self) -> bool {
        *self == TIMRSTR::TIMRST_3
    }
    #[doc = "Checks if the value of the field is `TIMRST_4`"]
    #[inline]
    pub fn is_timrst_4(&self) -> bool {
        *self == TIMRSTR::TIMRST_4
    }
    #[doc = "Checks if the value of the field is `TIMRST_6`"]
    #[inline]
    pub fn is_timrst_6(&self) -> bool {
        *self == TIMRSTR::TIMRST_6
    }
    #[doc = "Checks if the value of the field is `TIMRST_7`"]
    #[inline]
    pub fn is_timrst_7(&self) -> bool {
        *self == TIMRSTR::TIMRST_7
    }
}
#[doc = "Possible values of the field `TIMDEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMDECR {
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    TIMDEC_0,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    TIMDEC_1,
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    TIMDEC_2,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    TIMDEC_3,
}
impl TIMDECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMDECR::TIMDEC_0 => 0,
            TIMDECR::TIMDEC_1 => 1,
            TIMDECR::TIMDEC_2 => 2,
            TIMDECR::TIMDEC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMDECR {
        match value {
            0 => TIMDECR::TIMDEC_0,
            1 => TIMDECR::TIMDEC_1,
            2 => TIMDECR::TIMDEC_2,
            3 => TIMDECR::TIMDEC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMDEC_0`"]
    #[inline]
    pub fn is_timdec_0(&self) -> bool {
        *self == TIMDECR::TIMDEC_0
    }
    #[doc = "Checks if the value of the field is `TIMDEC_1`"]
    #[inline]
    pub fn is_timdec_1(&self) -> bool {
        *self == TIMDECR::TIMDEC_1
    }
    #[doc = "Checks if the value of the field is `TIMDEC_2`"]
    #[inline]
    pub fn is_timdec_2(&self) -> bool {
        *self == TIMDECR::TIMDEC_2
    }
    #[doc = "Checks if the value of the field is `TIMDEC_3`"]
    #[inline]
    pub fn is_timdec_3(&self) -> bool {
        *self == TIMDECR::TIMDEC_3
    }
}
#[doc = "Possible values of the field `TIMOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUTR {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    TIMOUT_0,
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    TIMOUT_1,
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    TIMOUT_2,
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    TIMOUT_3,
}
impl TIMOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMOUTR::TIMOUT_0 => 0,
            TIMOUTR::TIMOUT_1 => 1,
            TIMOUTR::TIMOUT_2 => 2,
            TIMOUTR::TIMOUT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMOUTR {
        match value {
            0 => TIMOUTR::TIMOUT_0,
            1 => TIMOUTR::TIMOUT_1,
            2 => TIMOUTR::TIMOUT_2,
            3 => TIMOUTR::TIMOUT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMOUT_0`"]
    #[inline]
    pub fn is_timout_0(&self) -> bool {
        *self == TIMOUTR::TIMOUT_0
    }
    #[doc = "Checks if the value of the field is `TIMOUT_1`"]
    #[inline]
    pub fn is_timout_1(&self) -> bool {
        *self == TIMOUTR::TIMOUT_1
    }
    #[doc = "Checks if the value of the field is `TIMOUT_2`"]
    #[inline]
    pub fn is_timout_2(&self) -> bool {
        *self == TIMOUTR::TIMOUT_2
    }
    #[doc = "Checks if the value of the field is `TIMOUT_3`"]
    #[inline]
    pub fn is_timout_3(&self) -> bool {
        *self == TIMOUTR::TIMOUT_3
    }
}
#[doc = "Values that can be written to the field `TSTART`"]
pub enum TSTARTW {
    #[doc = "Start bit disabled"]
    TSTART_0,
    #[doc = "Start bit enabled"]
    TSTART_1,
}
impl TSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSTARTW::TSTART_0 => false,
            TSTARTW::TSTART_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start bit disabled"]
    #[inline]
    pub fn tstart_0(self) -> &'a mut W {
        self.variant(TSTARTW::TSTART_0)
    }
    #[doc = "Start bit enabled"]
    #[inline]
    pub fn tstart_1(self) -> &'a mut W {
        self.variant(TSTARTW::TSTART_1)
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
#[doc = "Values that can be written to the field `TSTOP`"]
pub enum TSTOPW {
    #[doc = "Stop bit disabled"]
    TSTOP_0,
    #[doc = "Stop bit is enabled on timer compare"]
    TSTOP_1,
    #[doc = "Stop bit is enabled on timer disable"]
    TSTOP_2,
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    TSTOP_3,
}
impl TSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTOPW::TSTOP_0 => 0,
            TSTOPW::TSTOP_1 => 1,
            TSTOPW::TSTOP_2 => 2,
            TSTOPW::TSTOP_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTOPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Stop bit disabled"]
    #[inline]
    pub fn tstop_0(self) -> &'a mut W {
        self.variant(TSTOPW::TSTOP_0)
    }
    #[doc = "Stop bit is enabled on timer compare"]
    #[inline]
    pub fn tstop_1(self) -> &'a mut W {
        self.variant(TSTOPW::TSTOP_1)
    }
    #[doc = "Stop bit is enabled on timer disable"]
    #[inline]
    pub fn tstop_2(self) -> &'a mut W {
        self.variant(TSTOPW::TSTOP_2)
    }
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    #[inline]
    pub fn tstop_3(self) -> &'a mut W {
        self.variant(TSTOPW::TSTOP_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMENA`"]
pub enum TIMENAW {
    #[doc = "Timer always enabled"]
    TIMENA_0,
    #[doc = "Timer enabled on Timer N-1 enable"]
    TIMENA_1,
    #[doc = "Timer enabled on Trigger high"]
    TIMENA_2,
    #[doc = "Timer enabled on Trigger high and Pin high"]
    TIMENA_3,
    #[doc = "Timer enabled on Pin rising edge"]
    TIMENA_4,
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    TIMENA_5,
    #[doc = "Timer enabled on Trigger rising edge"]
    TIMENA_6,
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    TIMENA_7,
}
impl TIMENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMENAW::TIMENA_0 => 0,
            TIMENAW::TIMENA_1 => 1,
            TIMENAW::TIMENA_2 => 2,
            TIMENAW::TIMENA_3 => 3,
            TIMENAW::TIMENA_4 => 4,
            TIMENAW::TIMENA_5 => 5,
            TIMENAW::TIMENA_6 => 6,
            TIMENAW::TIMENA_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMENAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer always enabled"]
    #[inline]
    pub fn timena_0(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_0)
    }
    #[doc = "Timer enabled on Timer N-1 enable"]
    #[inline]
    pub fn timena_1(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_1)
    }
    #[doc = "Timer enabled on Trigger high"]
    #[inline]
    pub fn timena_2(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_2)
    }
    #[doc = "Timer enabled on Trigger high and Pin high"]
    #[inline]
    pub fn timena_3(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_3)
    }
    #[doc = "Timer enabled on Pin rising edge"]
    #[inline]
    pub fn timena_4(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_4)
    }
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    #[inline]
    pub fn timena_5(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_5)
    }
    #[doc = "Timer enabled on Trigger rising edge"]
    #[inline]
    pub fn timena_6(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_6)
    }
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    #[inline]
    pub fn timena_7(self) -> &'a mut W {
        self.variant(TIMENAW::TIMENA_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMDIS`"]
pub enum TIMDISW {
    #[doc = "Timer never disabled"]
    TIMDIS_0,
    #[doc = "Timer disabled on Timer N-1 disable"]
    TIMDIS_1,
    #[doc = "Timer disabled on Timer compare"]
    TIMDIS_2,
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    TIMDIS_3,
    #[doc = "Timer disabled on Pin rising or falling edge"]
    TIMDIS_4,
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    TIMDIS_5,
    #[doc = "Timer disabled on Trigger falling edge"]
    TIMDIS_6,
}
impl TIMDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMDISW::TIMDIS_0 => 0,
            TIMDISW::TIMDIS_1 => 1,
            TIMDISW::TIMDIS_2 => 2,
            TIMDISW::TIMDIS_3 => 3,
            TIMDISW::TIMDIS_4 => 4,
            TIMDISW::TIMDIS_5 => 5,
            TIMDISW::TIMDIS_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMDISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer never disabled"]
    #[inline]
    pub fn timdis_0(self) -> &'a mut W {
        self.variant(TIMDISW::TIMDIS_0)
    }
    #[doc = "Timer disabled on Timer N-1 disable"]
    #[inline]
    pub fn timdis_1(self) -> &'a mut W {
        self.variant(TIMDISW::TIMDIS_1)
    }
    #[doc = "Timer disabled on Timer compare"]
    #[inline]
    pub fn timdis_2(self) -> &'a mut W {
        self.variant(TIMDISW::TIMDIS_2)
    }
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    #[inline]
    pub fn timdis_3(self) -> &'a mut W {
        self.variant(TIMDISW::TIMDIS_3)
    }
    #[doc = "Timer disabled on Pin rising or falling edge"]
    #[inline]
    pub fn timdis_4(self) -> &'a mut W {
        self.variant(TIMDISW::TIMDIS_4)
    }
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    #[inline]
    pub fn timdis_5(self) -> &'a mut W {
        self.variant(TIMDISW::TIMDIS_5)
    }
    #[doc = "Timer disabled on Trigger falling edge"]
    #[inline]
    pub fn timdis_6(self) -> &'a mut W {
        self.variant(TIMDISW::TIMDIS_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMRST`"]
pub enum TIMRSTW {
    #[doc = "Timer never reset"]
    TIMRST_0,
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    TIMRST_2,
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    TIMRST_3,
    #[doc = "Timer reset on Timer Pin rising edge"]
    TIMRST_4,
    #[doc = "Timer reset on Trigger rising edge"]
    TIMRST_6,
    #[doc = "Timer reset on Trigger rising or falling edge"]
    TIMRST_7,
}
impl TIMRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMRSTW::TIMRST_0 => 0,
            TIMRSTW::TIMRST_2 => 2,
            TIMRSTW::TIMRST_3 => 3,
            TIMRSTW::TIMRST_4 => 4,
            TIMRSTW::TIMRST_6 => 6,
            TIMRSTW::TIMRST_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMRSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer never reset"]
    #[inline]
    pub fn timrst_0(self) -> &'a mut W {
        self.variant(TIMRSTW::TIMRST_0)
    }
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    #[inline]
    pub fn timrst_2(self) -> &'a mut W {
        self.variant(TIMRSTW::TIMRST_2)
    }
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    #[inline]
    pub fn timrst_3(self) -> &'a mut W {
        self.variant(TIMRSTW::TIMRST_3)
    }
    #[doc = "Timer reset on Timer Pin rising edge"]
    #[inline]
    pub fn timrst_4(self) -> &'a mut W {
        self.variant(TIMRSTW::TIMRST_4)
    }
    #[doc = "Timer reset on Trigger rising edge"]
    #[inline]
    pub fn timrst_6(self) -> &'a mut W {
        self.variant(TIMRSTW::TIMRST_6)
    }
    #[doc = "Timer reset on Trigger rising or falling edge"]
    #[inline]
    pub fn timrst_7(self) -> &'a mut W {
        self.variant(TIMRSTW::TIMRST_7)
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
#[doc = "Values that can be written to the field `TIMDEC`"]
pub enum TIMDECW {
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    TIMDEC_0,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    TIMDEC_1,
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    TIMDEC_2,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    TIMDEC_3,
}
impl TIMDECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMDECW::TIMDEC_0 => 0,
            TIMDECW::TIMDEC_1 => 1,
            TIMDECW::TIMDEC_2 => 2,
            TIMDECW::TIMDEC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMDECW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMDECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMDECW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    #[inline]
    pub fn timdec_0(self) -> &'a mut W {
        self.variant(TIMDECW::TIMDEC_0)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    #[inline]
    pub fn timdec_1(self) -> &'a mut W {
        self.variant(TIMDECW::TIMDEC_1)
    }
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    #[inline]
    pub fn timdec_2(self) -> &'a mut W {
        self.variant(TIMDECW::TIMDEC_2)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    #[inline]
    pub fn timdec_3(self) -> &'a mut W {
        self.variant(TIMDECW::TIMDEC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMOUT`"]
pub enum TIMOUTW {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    TIMOUT_0,
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    TIMOUT_1,
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    TIMOUT_2,
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    TIMOUT_3,
}
impl TIMOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMOUTW::TIMOUT_0 => 0,
            TIMOUTW::TIMOUT_1 => 1,
            TIMOUTW::TIMOUT_2 => 2,
            TIMOUTW::TIMOUT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    #[inline]
    pub fn timout_0(self) -> &'a mut W {
        self.variant(TIMOUTW::TIMOUT_0)
    }
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    #[inline]
    pub fn timout_1(self) -> &'a mut W {
        self.variant(TIMOUTW::TIMOUT_1)
    }
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    #[inline]
    pub fn timout_2(self) -> &'a mut W {
        self.variant(TIMOUTW::TIMOUT_2)
    }
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    #[inline]
    pub fn timout_3(self) -> &'a mut W {
        self.variant(TIMOUTW::TIMOUT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline]
    pub fn tstart(&self) -> TSTARTR {
        TSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline]
    pub fn tstop(&self) -> TSTOPR {
        TSTOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline]
    pub fn timena(&self) -> TIMENAR {
        TIMENAR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline]
    pub fn timdis(&self) -> TIMDISR {
        TIMDISR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline]
    pub fn timrst(&self) -> TIMRSTR {
        TIMRSTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline]
    pub fn timdec(&self) -> TIMDECR {
        TIMDECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline]
    pub fn timout(&self) -> TIMOUTR {
        TIMOUTR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline]
    pub fn tstart(&mut self) -> _TSTARTW {
        _TSTARTW { w: self }
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline]
    pub fn tstop(&mut self) -> _TSTOPW {
        _TSTOPW { w: self }
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline]
    pub fn timena(&mut self) -> _TIMENAW {
        _TIMENAW { w: self }
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline]
    pub fn timdis(&mut self) -> _TIMDISW {
        _TIMDISW { w: self }
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline]
    pub fn timrst(&mut self) -> _TIMRSTW {
        _TIMRSTW { w: self }
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline]
    pub fn timdec(&mut self) -> _TIMDECW {
        _TIMDECW { w: self }
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline]
    pub fn timout(&mut self) -> _TIMOUTW {
        _TIMOUTW { w: self }
    }
}
