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
#[doc = "Possible values of the field `OUTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTMODER {
    #[doc = "Asserted while counter is active"]
    OUTMODE_0,
    #[doc = "Clear OFLAG output on successful compare"]
    OUTMODE_1,
    #[doc = "Set OFLAG output on successful compare"]
    OUTMODE_2,
    #[doc = "Toggle OFLAG output on successful compare"]
    OUTMODE_3,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    OUTMODE_4,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    OUTMODE_5,
    #[doc = "Set on compare, cleared on counter rollover"]
    OUTMODE_6,
    #[doc = "Enable gated clock output while counter is active"]
    OUTMODE_7,
}
impl OUTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTMODER::OUTMODE_0 => 0,
            OUTMODER::OUTMODE_1 => 1,
            OUTMODER::OUTMODE_2 => 2,
            OUTMODER::OUTMODE_3 => 3,
            OUTMODER::OUTMODE_4 => 4,
            OUTMODER::OUTMODE_5 => 5,
            OUTMODER::OUTMODE_6 => 6,
            OUTMODER::OUTMODE_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTMODER {
        match value {
            0 => OUTMODER::OUTMODE_0,
            1 => OUTMODER::OUTMODE_1,
            2 => OUTMODER::OUTMODE_2,
            3 => OUTMODER::OUTMODE_3,
            4 => OUTMODER::OUTMODE_4,
            5 => OUTMODER::OUTMODE_5,
            6 => OUTMODER::OUTMODE_6,
            7 => OUTMODER::OUTMODE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTMODE_0`"]
    #[inline]
    pub fn is_outmode_0(&self) -> bool {
        *self == OUTMODER::OUTMODE_0
    }
    #[doc = "Checks if the value of the field is `OUTMODE_1`"]
    #[inline]
    pub fn is_outmode_1(&self) -> bool {
        *self == OUTMODER::OUTMODE_1
    }
    #[doc = "Checks if the value of the field is `OUTMODE_2`"]
    #[inline]
    pub fn is_outmode_2(&self) -> bool {
        *self == OUTMODER::OUTMODE_2
    }
    #[doc = "Checks if the value of the field is `OUTMODE_3`"]
    #[inline]
    pub fn is_outmode_3(&self) -> bool {
        *self == OUTMODER::OUTMODE_3
    }
    #[doc = "Checks if the value of the field is `OUTMODE_4`"]
    #[inline]
    pub fn is_outmode_4(&self) -> bool {
        *self == OUTMODER::OUTMODE_4
    }
    #[doc = "Checks if the value of the field is `OUTMODE_5`"]
    #[inline]
    pub fn is_outmode_5(&self) -> bool {
        *self == OUTMODER::OUTMODE_5
    }
    #[doc = "Checks if the value of the field is `OUTMODE_6`"]
    #[inline]
    pub fn is_outmode_6(&self) -> bool {
        *self == OUTMODER::OUTMODE_6
    }
    #[doc = "Checks if the value of the field is `OUTMODE_7`"]
    #[inline]
    pub fn is_outmode_7(&self) -> bool {
        *self == OUTMODER::OUTMODE_7
    }
}
#[doc = "Possible values of the field `COINIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COINITR {
    #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    COINIT_0,
    #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
    COINIT_1,
}
impl COINITR {
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
            COINITR::COINIT_0 => false,
            COINITR::COINIT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COINITR {
        match value {
            false => COINITR::COINIT_0,
            true => COINITR::COINIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `COINIT_0`"]
    #[inline]
    pub fn is_coinit_0(&self) -> bool {
        *self == COINITR::COINIT_0
    }
    #[doc = "Checks if the value of the field is `COINIT_1`"]
    #[inline]
    pub fn is_coinit_1(&self) -> bool {
        *self == COINITR::COINIT_1
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Count up."]
    DIR_0,
    #[doc = "Count down."]
    DIR_1,
}
impl DIRR {
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
            DIRR::DIR_0 => false,
            DIRR::DIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::DIR_0,
            true => DIRR::DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIR_0`"]
    #[inline]
    pub fn is_dir_0(&self) -> bool {
        *self == DIRR::DIR_0
    }
    #[doc = "Checks if the value of the field is `DIR_1`"]
    #[inline]
    pub fn is_dir_1(&self) -> bool {
        *self == DIRR::DIR_1
    }
}
#[doc = "Possible values of the field `LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENGTHR {
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    LENGTH_0,
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    LENGTH_1,
}
impl LENGTHR {
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
            LENGTHR::LENGTH_0 => false,
            LENGTHR::LENGTH_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LENGTHR {
        match value {
            false => LENGTHR::LENGTH_0,
            true => LENGTHR::LENGTH_1,
        }
    }
    #[doc = "Checks if the value of the field is `LENGTH_0`"]
    #[inline]
    pub fn is_length_0(&self) -> bool {
        *self == LENGTHR::LENGTH_0
    }
    #[doc = "Checks if the value of the field is `LENGTH_1`"]
    #[inline]
    pub fn is_length_1(&self) -> bool {
        *self == LENGTHR::LENGTH_1
    }
}
#[doc = "Possible values of the field `ONCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONCER {
    #[doc = "Count repeatedly."]
    ONCE_0,
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    ONCE_1,
}
impl ONCER {
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
            ONCER::ONCE_0 => false,
            ONCER::ONCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONCER {
        match value {
            false => ONCER::ONCE_0,
            true => ONCER::ONCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONCE_0`"]
    #[inline]
    pub fn is_once_0(&self) -> bool {
        *self == ONCER::ONCE_0
    }
    #[doc = "Checks if the value of the field is `ONCE_1`"]
    #[inline]
    pub fn is_once_1(&self) -> bool {
        *self == ONCER::ONCE_1
    }
}
#[doc = "Possible values of the field `SCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSR {
    #[doc = "Counter 0 input pin"]
    SCS_0,
    #[doc = "Counter 1 input pin"]
    SCS_1,
    #[doc = "Counter 2 input pin"]
    SCS_2,
    #[doc = "Counter 3 input pin"]
    SCS_3,
}
impl SCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCSR::SCS_0 => 0,
            SCSR::SCS_1 => 1,
            SCSR::SCS_2 => 2,
            SCSR::SCS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCSR {
        match value {
            0 => SCSR::SCS_0,
            1 => SCSR::SCS_1,
            2 => SCSR::SCS_2,
            3 => SCSR::SCS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SCS_0`"]
    #[inline]
    pub fn is_scs_0(&self) -> bool {
        *self == SCSR::SCS_0
    }
    #[doc = "Checks if the value of the field is `SCS_1`"]
    #[inline]
    pub fn is_scs_1(&self) -> bool {
        *self == SCSR::SCS_1
    }
    #[doc = "Checks if the value of the field is `SCS_2`"]
    #[inline]
    pub fn is_scs_2(&self) -> bool {
        *self == SCSR::SCS_2
    }
    #[doc = "Checks if the value of the field is `SCS_3`"]
    #[inline]
    pub fn is_scs_3(&self) -> bool {
        *self == SCSR::SCS_3
    }
}
#[doc = "Possible values of the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSR {
    #[doc = "Counter 0 input pin"]
    PCS_0,
    #[doc = "Counter 1 input pin"]
    PCS_1,
    #[doc = "Counter 2 input pin"]
    PCS_2,
    #[doc = "Counter 3 input pin"]
    PCS_3,
    #[doc = "Counter 0 output"]
    PCS_4,
    #[doc = "Counter 1 output"]
    PCS_5,
    #[doc = "Counter 2 output"]
    PCS_6,
    #[doc = "Counter 3 output"]
    PCS_7,
    #[doc = "IP bus clock divide by 1 prescaler"]
    PCS_8,
    #[doc = "IP bus clock divide by 2 prescaler"]
    PCS_9,
    #[doc = "IP bus clock divide by 4 prescaler"]
    PCS_10,
    #[doc = "IP bus clock divide by 8 prescaler"]
    PCS_11,
    #[doc = "IP bus clock divide by 16 prescaler"]
    PCS_12,
    #[doc = "IP bus clock divide by 32 prescaler"]
    PCS_13,
    #[doc = "IP bus clock divide by 64 prescaler"]
    PCS_14,
    #[doc = "IP bus clock divide by 128 prescaler"]
    PCS_15,
}
impl PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSR::PCS_0 => 0,
            PCSR::PCS_1 => 1,
            PCSR::PCS_2 => 2,
            PCSR::PCS_3 => 3,
            PCSR::PCS_4 => 4,
            PCSR::PCS_5 => 5,
            PCSR::PCS_6 => 6,
            PCSR::PCS_7 => 7,
            PCSR::PCS_8 => 8,
            PCSR::PCS_9 => 9,
            PCSR::PCS_10 => 10,
            PCSR::PCS_11 => 11,
            PCSR::PCS_12 => 12,
            PCSR::PCS_13 => 13,
            PCSR::PCS_14 => 14,
            PCSR::PCS_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSR {
        match value {
            0 => PCSR::PCS_0,
            1 => PCSR::PCS_1,
            2 => PCSR::PCS_2,
            3 => PCSR::PCS_3,
            4 => PCSR::PCS_4,
            5 => PCSR::PCS_5,
            6 => PCSR::PCS_6,
            7 => PCSR::PCS_7,
            8 => PCSR::PCS_8,
            9 => PCSR::PCS_9,
            10 => PCSR::PCS_10,
            11 => PCSR::PCS_11,
            12 => PCSR::PCS_12,
            13 => PCSR::PCS_13,
            14 => PCSR::PCS_14,
            15 => PCSR::PCS_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCS_0`"]
    #[inline]
    pub fn is_pcs_0(&self) -> bool {
        *self == PCSR::PCS_0
    }
    #[doc = "Checks if the value of the field is `PCS_1`"]
    #[inline]
    pub fn is_pcs_1(&self) -> bool {
        *self == PCSR::PCS_1
    }
    #[doc = "Checks if the value of the field is `PCS_2`"]
    #[inline]
    pub fn is_pcs_2(&self) -> bool {
        *self == PCSR::PCS_2
    }
    #[doc = "Checks if the value of the field is `PCS_3`"]
    #[inline]
    pub fn is_pcs_3(&self) -> bool {
        *self == PCSR::PCS_3
    }
    #[doc = "Checks if the value of the field is `PCS_4`"]
    #[inline]
    pub fn is_pcs_4(&self) -> bool {
        *self == PCSR::PCS_4
    }
    #[doc = "Checks if the value of the field is `PCS_5`"]
    #[inline]
    pub fn is_pcs_5(&self) -> bool {
        *self == PCSR::PCS_5
    }
    #[doc = "Checks if the value of the field is `PCS_6`"]
    #[inline]
    pub fn is_pcs_6(&self) -> bool {
        *self == PCSR::PCS_6
    }
    #[doc = "Checks if the value of the field is `PCS_7`"]
    #[inline]
    pub fn is_pcs_7(&self) -> bool {
        *self == PCSR::PCS_7
    }
    #[doc = "Checks if the value of the field is `PCS_8`"]
    #[inline]
    pub fn is_pcs_8(&self) -> bool {
        *self == PCSR::PCS_8
    }
    #[doc = "Checks if the value of the field is `PCS_9`"]
    #[inline]
    pub fn is_pcs_9(&self) -> bool {
        *self == PCSR::PCS_9
    }
    #[doc = "Checks if the value of the field is `PCS_10`"]
    #[inline]
    pub fn is_pcs_10(&self) -> bool {
        *self == PCSR::PCS_10
    }
    #[doc = "Checks if the value of the field is `PCS_11`"]
    #[inline]
    pub fn is_pcs_11(&self) -> bool {
        *self == PCSR::PCS_11
    }
    #[doc = "Checks if the value of the field is `PCS_12`"]
    #[inline]
    pub fn is_pcs_12(&self) -> bool {
        *self == PCSR::PCS_12
    }
    #[doc = "Checks if the value of the field is `PCS_13`"]
    #[inline]
    pub fn is_pcs_13(&self) -> bool {
        *self == PCSR::PCS_13
    }
    #[doc = "Checks if the value of the field is `PCS_14`"]
    #[inline]
    pub fn is_pcs_14(&self) -> bool {
        *self == PCSR::PCS_14
    }
    #[doc = "Checks if the value of the field is `PCS_15`"]
    #[inline]
    pub fn is_pcs_15(&self) -> bool {
        *self == PCSR::PCS_15
    }
}
#[doc = "Possible values of the field `CM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMR {
    #[doc = "No operation"]
    CM_0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL[IPS] = 0. Falling edges are counted when SCTRL[IPS] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL[IPS]."]
    CM_1,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    CM_2,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    CM_3,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    CM_4,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL[IPS] = 0. Falling edges are counted when SCTRL[IPS] = 1."]
    CM_5,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    CM_6,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CM_7,
}
impl CMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMR::CM_0 => 0,
            CMR::CM_1 => 1,
            CMR::CM_2 => 2,
            CMR::CM_3 => 3,
            CMR::CM_4 => 4,
            CMR::CM_5 => 5,
            CMR::CM_6 => 6,
            CMR::CM_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMR {
        match value {
            0 => CMR::CM_0,
            1 => CMR::CM_1,
            2 => CMR::CM_2,
            3 => CMR::CM_3,
            4 => CMR::CM_4,
            5 => CMR::CM_5,
            6 => CMR::CM_6,
            7 => CMR::CM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline]
    pub fn is_cm_0(&self) -> bool {
        *self == CMR::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_1`"]
    #[inline]
    pub fn is_cm_1(&self) -> bool {
        *self == CMR::CM_1
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline]
    pub fn is_cm_2(&self) -> bool {
        *self == CMR::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline]
    pub fn is_cm_3(&self) -> bool {
        *self == CMR::CM_3
    }
    #[doc = "Checks if the value of the field is `CM_4`"]
    #[inline]
    pub fn is_cm_4(&self) -> bool {
        *self == CMR::CM_4
    }
    #[doc = "Checks if the value of the field is `CM_5`"]
    #[inline]
    pub fn is_cm_5(&self) -> bool {
        *self == CMR::CM_5
    }
    #[doc = "Checks if the value of the field is `CM_6`"]
    #[inline]
    pub fn is_cm_6(&self) -> bool {
        *self == CMR::CM_6
    }
    #[doc = "Checks if the value of the field is `CM_7`"]
    #[inline]
    pub fn is_cm_7(&self) -> bool {
        *self == CMR::CM_7
    }
}
#[doc = "Values that can be written to the field `OUTMODE`"]
pub enum OUTMODEW {
    #[doc = "Asserted while counter is active"]
    OUTMODE_0,
    #[doc = "Clear OFLAG output on successful compare"]
    OUTMODE_1,
    #[doc = "Set OFLAG output on successful compare"]
    OUTMODE_2,
    #[doc = "Toggle OFLAG output on successful compare"]
    OUTMODE_3,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    OUTMODE_4,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    OUTMODE_5,
    #[doc = "Set on compare, cleared on counter rollover"]
    OUTMODE_6,
    #[doc = "Enable gated clock output while counter is active"]
    OUTMODE_7,
}
impl OUTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTMODEW::OUTMODE_0 => 0,
            OUTMODEW::OUTMODE_1 => 1,
            OUTMODEW::OUTMODE_2 => 2,
            OUTMODEW::OUTMODE_3 => 3,
            OUTMODEW::OUTMODE_4 => 4,
            OUTMODEW::OUTMODE_5 => 5,
            OUTMODEW::OUTMODE_6 => 6,
            OUTMODEW::OUTMODE_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Asserted while counter is active"]
    #[inline]
    pub fn outmode_0(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_0)
    }
    #[doc = "Clear OFLAG output on successful compare"]
    #[inline]
    pub fn outmode_1(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_1)
    }
    #[doc = "Set OFLAG output on successful compare"]
    #[inline]
    pub fn outmode_2(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_2)
    }
    #[doc = "Toggle OFLAG output on successful compare"]
    #[inline]
    pub fn outmode_3(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_3)
    }
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    #[inline]
    pub fn outmode_4(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_4)
    }
    #[doc = "Set on compare, cleared on secondary source input edge"]
    #[inline]
    pub fn outmode_5(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_5)
    }
    #[doc = "Set on compare, cleared on counter rollover"]
    #[inline]
    pub fn outmode_6(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_6)
    }
    #[doc = "Enable gated clock output while counter is active"]
    #[inline]
    pub fn outmode_7(self) -> &'a mut W {
        self.variant(OUTMODEW::OUTMODE_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COINIT`"]
pub enum COINITW {
    #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    COINIT_0,
    #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
    COINIT_1,
}
impl COINITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COINITW::COINIT_0 => false,
            COINITW::COINIT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COINITW<'a> {
    w: &'a mut W,
}
impl<'a> _COINITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COINITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    #[inline]
    pub fn coinit_0(self) -> &'a mut W {
        self.variant(COINITW::COINIT_0)
    }
    #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
    #[inline]
    pub fn coinit_1(self) -> &'a mut W {
        self.variant(COINITW::COINIT_1)
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
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Count up."]
    DIR_0,
    #[doc = "Count down."]
    DIR_1,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRW::DIR_0 => false,
            DIRW::DIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Count up."]
    #[inline]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRW::DIR_0)
    }
    #[doc = "Count down."]
    #[inline]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRW::DIR_1)
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
#[doc = "Values that can be written to the field `LENGTH`"]
pub enum LENGTHW {
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    LENGTH_0,
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    LENGTH_1,
}
impl LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LENGTHW::LENGTH_0 => false,
            LENGTHW::LENGTH_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LENGTHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    #[inline]
    pub fn length_0(self) -> &'a mut W {
        self.variant(LENGTHW::LENGTH_0)
    }
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    #[inline]
    pub fn length_1(self) -> &'a mut W {
        self.variant(LENGTHW::LENGTH_1)
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
#[doc = "Values that can be written to the field `ONCE`"]
pub enum ONCEW {
    #[doc = "Count repeatedly."]
    ONCE_0,
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    ONCE_1,
}
impl ONCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONCEW::ONCE_0 => false,
            ONCEW::ONCE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONCEW<'a> {
    w: &'a mut W,
}
impl<'a> _ONCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Count repeatedly."]
    #[inline]
    pub fn once_0(self) -> &'a mut W {
        self.variant(ONCEW::ONCE_0)
    }
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    #[inline]
    pub fn once_1(self) -> &'a mut W {
        self.variant(ONCEW::ONCE_1)
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
#[doc = "Values that can be written to the field `SCS`"]
pub enum SCSW {
    #[doc = "Counter 0 input pin"]
    SCS_0,
    #[doc = "Counter 1 input pin"]
    SCS_1,
    #[doc = "Counter 2 input pin"]
    SCS_2,
    #[doc = "Counter 3 input pin"]
    SCS_3,
}
impl SCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCSW::SCS_0 => 0,
            SCSW::SCS_1 => 1,
            SCSW::SCS_2 => 2,
            SCSW::SCS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCSW<'a> {
    w: &'a mut W,
}
impl<'a> _SCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counter 0 input pin"]
    #[inline]
    pub fn scs_0(self) -> &'a mut W {
        self.variant(SCSW::SCS_0)
    }
    #[doc = "Counter 1 input pin"]
    #[inline]
    pub fn scs_1(self) -> &'a mut W {
        self.variant(SCSW::SCS_1)
    }
    #[doc = "Counter 2 input pin"]
    #[inline]
    pub fn scs_2(self) -> &'a mut W {
        self.variant(SCSW::SCS_2)
    }
    #[doc = "Counter 3 input pin"]
    #[inline]
    pub fn scs_3(self) -> &'a mut W {
        self.variant(SCSW::SCS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS`"]
pub enum PCSW {
    #[doc = "Counter 0 input pin"]
    PCS_0,
    #[doc = "Counter 1 input pin"]
    PCS_1,
    #[doc = "Counter 2 input pin"]
    PCS_2,
    #[doc = "Counter 3 input pin"]
    PCS_3,
    #[doc = "Counter 0 output"]
    PCS_4,
    #[doc = "Counter 1 output"]
    PCS_5,
    #[doc = "Counter 2 output"]
    PCS_6,
    #[doc = "Counter 3 output"]
    PCS_7,
    #[doc = "IP bus clock divide by 1 prescaler"]
    PCS_8,
    #[doc = "IP bus clock divide by 2 prescaler"]
    PCS_9,
    #[doc = "IP bus clock divide by 4 prescaler"]
    PCS_10,
    #[doc = "IP bus clock divide by 8 prescaler"]
    PCS_11,
    #[doc = "IP bus clock divide by 16 prescaler"]
    PCS_12,
    #[doc = "IP bus clock divide by 32 prescaler"]
    PCS_13,
    #[doc = "IP bus clock divide by 64 prescaler"]
    PCS_14,
    #[doc = "IP bus clock divide by 128 prescaler"]
    PCS_15,
}
impl PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSW::PCS_0 => 0,
            PCSW::PCS_1 => 1,
            PCSW::PCS_2 => 2,
            PCSW::PCS_3 => 3,
            PCSW::PCS_4 => 4,
            PCSW::PCS_5 => 5,
            PCSW::PCS_6 => 6,
            PCSW::PCS_7 => 7,
            PCSW::PCS_8 => 8,
            PCSW::PCS_9 => 9,
            PCSW::PCS_10 => 10,
            PCSW::PCS_11 => 11,
            PCSW::PCS_12 => 12,
            PCSW::PCS_13 => 13,
            PCSW::PCS_14 => 14,
            PCSW::PCS_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counter 0 input pin"]
    #[inline]
    pub fn pcs_0(self) -> &'a mut W {
        self.variant(PCSW::PCS_0)
    }
    #[doc = "Counter 1 input pin"]
    #[inline]
    pub fn pcs_1(self) -> &'a mut W {
        self.variant(PCSW::PCS_1)
    }
    #[doc = "Counter 2 input pin"]
    #[inline]
    pub fn pcs_2(self) -> &'a mut W {
        self.variant(PCSW::PCS_2)
    }
    #[doc = "Counter 3 input pin"]
    #[inline]
    pub fn pcs_3(self) -> &'a mut W {
        self.variant(PCSW::PCS_3)
    }
    #[doc = "Counter 0 output"]
    #[inline]
    pub fn pcs_4(self) -> &'a mut W {
        self.variant(PCSW::PCS_4)
    }
    #[doc = "Counter 1 output"]
    #[inline]
    pub fn pcs_5(self) -> &'a mut W {
        self.variant(PCSW::PCS_5)
    }
    #[doc = "Counter 2 output"]
    #[inline]
    pub fn pcs_6(self) -> &'a mut W {
        self.variant(PCSW::PCS_6)
    }
    #[doc = "Counter 3 output"]
    #[inline]
    pub fn pcs_7(self) -> &'a mut W {
        self.variant(PCSW::PCS_7)
    }
    #[doc = "IP bus clock divide by 1 prescaler"]
    #[inline]
    pub fn pcs_8(self) -> &'a mut W {
        self.variant(PCSW::PCS_8)
    }
    #[doc = "IP bus clock divide by 2 prescaler"]
    #[inline]
    pub fn pcs_9(self) -> &'a mut W {
        self.variant(PCSW::PCS_9)
    }
    #[doc = "IP bus clock divide by 4 prescaler"]
    #[inline]
    pub fn pcs_10(self) -> &'a mut W {
        self.variant(PCSW::PCS_10)
    }
    #[doc = "IP bus clock divide by 8 prescaler"]
    #[inline]
    pub fn pcs_11(self) -> &'a mut W {
        self.variant(PCSW::PCS_11)
    }
    #[doc = "IP bus clock divide by 16 prescaler"]
    #[inline]
    pub fn pcs_12(self) -> &'a mut W {
        self.variant(PCSW::PCS_12)
    }
    #[doc = "IP bus clock divide by 32 prescaler"]
    #[inline]
    pub fn pcs_13(self) -> &'a mut W {
        self.variant(PCSW::PCS_13)
    }
    #[doc = "IP bus clock divide by 64 prescaler"]
    #[inline]
    pub fn pcs_14(self) -> &'a mut W {
        self.variant(PCSW::PCS_14)
    }
    #[doc = "IP bus clock divide by 128 prescaler"]
    #[inline]
    pub fn pcs_15(self) -> &'a mut W {
        self.variant(PCSW::PCS_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CM`"]
pub enum CMW {
    #[doc = "No operation"]
    CM_0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL[IPS] = 0. Falling edges are counted when SCTRL[IPS] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL[IPS]."]
    CM_1,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    CM_2,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    CM_3,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    CM_4,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL[IPS] = 0. Falling edges are counted when SCTRL[IPS] = 1."]
    CM_5,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    CM_6,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CM_7,
}
impl CMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMW::CM_0 => 0,
            CMW::CM_1 => 1,
            CMW::CM_2 => 2,
            CMW::CM_3 => 3,
            CMW::CM_4 => 4,
            CMW::CM_5 => 5,
            CMW::CM_6 => 6,
            CMW::CM_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CMW::CM_0)
    }
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL[IPS] = 0. Falling edges are counted when SCTRL[IPS] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL[IPS]."]
    #[inline]
    pub fn cm_1(self) -> &'a mut W {
        self.variant(CMW::CM_1)
    }
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    #[inline]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CMW::CM_2)
    }
    #[doc = "Count rising edges of primary source while secondary input high active"]
    #[inline]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CMW::CM_3)
    }
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    #[inline]
    pub fn cm_4(self) -> &'a mut W {
        self.variant(CMW::CM_4)
    }
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL[IPS] = 0. Falling edges are counted when SCTRL[IPS] = 1."]
    #[inline]
    pub fn cm_5(self) -> &'a mut W {
        self.variant(CMW::CM_5)
    }
    #[doc = "Edge of secondary source triggers primary count until compare"]
    #[inline]
    pub fn cm_6(self) -> &'a mut W {
        self.variant(CMW::CM_6)
    }
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    #[inline]
    pub fn cm_7(self) -> &'a mut W {
        self.variant(CMW::CM_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline]
    pub fn outmode(&self) -> OUTMODER {
        OUTMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline]
    pub fn coinit(&self) -> COINITR {
        COINITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline]
    pub fn length(&self) -> LENGTHR {
        LENGTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline]
    pub fn once(&self) -> ONCER {
        ONCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline]
    pub fn scs(&self) -> SCSR {
        SCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline]
    pub fn pcs(&self) -> PCSR {
        PCSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline]
    pub fn cm(&self) -> CMR {
        CMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline]
    pub fn outmode(&mut self) -> _OUTMODEW {
        _OUTMODEW { w: self }
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline]
    pub fn coinit(&mut self) -> _COINITW {
        _COINITW { w: self }
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline]
    pub fn length(&mut self) -> _LENGTHW {
        _LENGTHW { w: self }
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline]
    pub fn once(&mut self) -> _ONCEW {
        _ONCEW { w: self }
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline]
    pub fn scs(&mut self) -> _SCSW {
        _SCSW { w: self }
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline]
    pub fn cm(&mut self) -> _CMW {
        _CMW { w: self }
    }
}
