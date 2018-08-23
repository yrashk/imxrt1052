#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMCR3 {
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
#[doc = r" Value of the field"]
pub struct RENR {
    bits: bool,
}
impl RENR {
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
#[doc = "Possible values of the field `REBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REBLR {
    #[doc = "1"]
    REBL_0,
    #[doc = "2"]
    REBL_1,
    #[doc = "3"]
    REBL_2,
    #[doc = "4"]
    REBL_3,
    #[doc = "5"]
    REBL_4,
    #[doc = "6"]
    REBL_5,
    #[doc = "7"]
    REBL_6,
    #[doc = "8"]
    REBL_7,
}
impl REBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REBLR::REBL_0 => 0,
            REBLR::REBL_1 => 1,
            REBLR::REBL_2 => 2,
            REBLR::REBL_3 => 3,
            REBLR::REBL_4 => 4,
            REBLR::REBL_5 => 5,
            REBLR::REBL_6 => 6,
            REBLR::REBL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REBLR {
        match value {
            0 => REBLR::REBL_0,
            1 => REBLR::REBL_1,
            2 => REBLR::REBL_2,
            3 => REBLR::REBL_3,
            4 => REBLR::REBL_4,
            5 => REBLR::REBL_5,
            6 => REBLR::REBL_6,
            7 => REBLR::REBL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REBL_0`"]
    #[inline]
    pub fn is_rebl_0(&self) -> bool {
        *self == REBLR::REBL_0
    }
    #[doc = "Checks if the value of the field is `REBL_1`"]
    #[inline]
    pub fn is_rebl_1(&self) -> bool {
        *self == REBLR::REBL_1
    }
    #[doc = "Checks if the value of the field is `REBL_2`"]
    #[inline]
    pub fn is_rebl_2(&self) -> bool {
        *self == REBLR::REBL_2
    }
    #[doc = "Checks if the value of the field is `REBL_3`"]
    #[inline]
    pub fn is_rebl_3(&self) -> bool {
        *self == REBLR::REBL_3
    }
    #[doc = "Checks if the value of the field is `REBL_4`"]
    #[inline]
    pub fn is_rebl_4(&self) -> bool {
        *self == REBLR::REBL_4
    }
    #[doc = "Checks if the value of the field is `REBL_5`"]
    #[inline]
    pub fn is_rebl_5(&self) -> bool {
        *self == REBLR::REBL_5
    }
    #[doc = "Checks if the value of the field is `REBL_6`"]
    #[inline]
    pub fn is_rebl_6(&self) -> bool {
        *self == REBLR::REBL_6
    }
    #[doc = "Checks if the value of the field is `REBL_7`"]
    #[inline]
    pub fn is_rebl_7(&self) -> bool {
        *self == REBLR::REBL_7
    }
}
#[doc = "Possible values of the field `PRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER {
    #[doc = "256*16 cycle"]
    PRESCALE_0,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_1,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_2,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_3,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_4,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_5,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_6,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_7,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_8,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
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
            PRESCALER::PRESCALE_8 => 8,
            PRESCALER::PRESCALE_9 => 9,
            PRESCALER::_Reserved(bits) => bits,
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
            8 => PRESCALER::PRESCALE_8,
            9 => PRESCALER::PRESCALE_9,
            i => PRESCALER::_Reserved(i),
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
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline]
    pub fn is_prescale_8(&self) -> bool {
        *self == PRESCALER::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_9`"]
    #[inline]
    pub fn is_prescale_9(&self) -> bool {
        *self == PRESCALER::PRESCALE_9
    }
}
#[doc = "Possible values of the field `RT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR {
    #[doc = "256*Prescaler period"]
    RT_0,
    #[doc = "RT*Prescaler period"]
    RT_1,
    #[doc = "RT*Prescaler period"]
    RT_2,
    #[doc = "RT*Prescaler period"]
    RT_3,
    #[doc = "RT*Prescaler period"]
    RT_4,
    #[doc = "RT*Prescaler period"]
    RT_5,
    #[doc = "RT*Prescaler period"]
    RT_6,
    #[doc = "RT*Prescaler period"]
    RT_7,
    #[doc = "RT*Prescaler period"]
    RT_8,
    #[doc = "RT*Prescaler period"]
    RT_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTR::RT_0 => 0,
            RTR::RT_1 => 1,
            RTR::RT_2 => 2,
            RTR::RT_3 => 3,
            RTR::RT_4 => 4,
            RTR::RT_5 => 5,
            RTR::RT_6 => 6,
            RTR::RT_7 => 7,
            RTR::RT_8 => 8,
            RTR::RT_9 => 9,
            RTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTR {
        match value {
            0 => RTR::RT_0,
            1 => RTR::RT_1,
            2 => RTR::RT_2,
            3 => RTR::RT_3,
            4 => RTR::RT_4,
            5 => RTR::RT_5,
            6 => RTR::RT_6,
            7 => RTR::RT_7,
            8 => RTR::RT_8,
            9 => RTR::RT_9,
            i => RTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RT_0`"]
    #[inline]
    pub fn is_rt_0(&self) -> bool {
        *self == RTR::RT_0
    }
    #[doc = "Checks if the value of the field is `RT_1`"]
    #[inline]
    pub fn is_rt_1(&self) -> bool {
        *self == RTR::RT_1
    }
    #[doc = "Checks if the value of the field is `RT_2`"]
    #[inline]
    pub fn is_rt_2(&self) -> bool {
        *self == RTR::RT_2
    }
    #[doc = "Checks if the value of the field is `RT_3`"]
    #[inline]
    pub fn is_rt_3(&self) -> bool {
        *self == RTR::RT_3
    }
    #[doc = "Checks if the value of the field is `RT_4`"]
    #[inline]
    pub fn is_rt_4(&self) -> bool {
        *self == RTR::RT_4
    }
    #[doc = "Checks if the value of the field is `RT_5`"]
    #[inline]
    pub fn is_rt_5(&self) -> bool {
        *self == RTR::RT_5
    }
    #[doc = "Checks if the value of the field is `RT_6`"]
    #[inline]
    pub fn is_rt_6(&self) -> bool {
        *self == RTR::RT_6
    }
    #[doc = "Checks if the value of the field is `RT_7`"]
    #[inline]
    pub fn is_rt_7(&self) -> bool {
        *self == RTR::RT_7
    }
    #[doc = "Checks if the value of the field is `RT_8`"]
    #[inline]
    pub fn is_rt_8(&self) -> bool {
        *self == RTR::RT_8
    }
    #[doc = "Checks if the value of the field is `RT_9`"]
    #[inline]
    pub fn is_rt_9(&self) -> bool {
        *self == RTR::RT_9
    }
}
#[doc = "Possible values of the field `UT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTR {
    #[doc = "256*Prescaler period"]
    UT_0,
    #[doc = "UT*Prescaler period"]
    UT_1,
    #[doc = "UT*Prescaler period"]
    UT_2,
    #[doc = "UT*Prescaler period"]
    UT_3,
    #[doc = "UT*Prescaler period"]
    UT_4,
    #[doc = "UT*Prescaler period"]
    UT_5,
    #[doc = "UT*Prescaler period"]
    UT_6,
    #[doc = "UT*Prescaler period"]
    UT_7,
    #[doc = "UT*Prescaler period"]
    UT_8,
    #[doc = "UT*Prescaler period"]
    UT_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UTR::UT_0 => 0,
            UTR::UT_1 => 1,
            UTR::UT_2 => 2,
            UTR::UT_3 => 3,
            UTR::UT_4 => 4,
            UTR::UT_5 => 5,
            UTR::UT_6 => 6,
            UTR::UT_7 => 7,
            UTR::UT_8 => 8,
            UTR::UT_9 => 9,
            UTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UTR {
        match value {
            0 => UTR::UT_0,
            1 => UTR::UT_1,
            2 => UTR::UT_2,
            3 => UTR::UT_3,
            4 => UTR::UT_4,
            5 => UTR::UT_5,
            6 => UTR::UT_6,
            7 => UTR::UT_7,
            8 => UTR::UT_8,
            9 => UTR::UT_9,
            i => UTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UT_0`"]
    #[inline]
    pub fn is_ut_0(&self) -> bool {
        *self == UTR::UT_0
    }
    #[doc = "Checks if the value of the field is `UT_1`"]
    #[inline]
    pub fn is_ut_1(&self) -> bool {
        *self == UTR::UT_1
    }
    #[doc = "Checks if the value of the field is `UT_2`"]
    #[inline]
    pub fn is_ut_2(&self) -> bool {
        *self == UTR::UT_2
    }
    #[doc = "Checks if the value of the field is `UT_3`"]
    #[inline]
    pub fn is_ut_3(&self) -> bool {
        *self == UTR::UT_3
    }
    #[doc = "Checks if the value of the field is `UT_4`"]
    #[inline]
    pub fn is_ut_4(&self) -> bool {
        *self == UTR::UT_4
    }
    #[doc = "Checks if the value of the field is `UT_5`"]
    #[inline]
    pub fn is_ut_5(&self) -> bool {
        *self == UTR::UT_5
    }
    #[doc = "Checks if the value of the field is `UT_6`"]
    #[inline]
    pub fn is_ut_6(&self) -> bool {
        *self == UTR::UT_6
    }
    #[doc = "Checks if the value of the field is `UT_7`"]
    #[inline]
    pub fn is_ut_7(&self) -> bool {
        *self == UTR::UT_7
    }
    #[doc = "Checks if the value of the field is `UT_8`"]
    #[inline]
    pub fn is_ut_8(&self) -> bool {
        *self == UTR::UT_8
    }
    #[doc = "Checks if the value of the field is `UT_9`"]
    #[inline]
    pub fn is_ut_9(&self) -> bool {
        *self == UTR::UT_9
    }
}
#[doc = r" Proxy"]
pub struct _RENW<'a> {
    w: &'a mut W,
}
impl<'a> _RENW<'a> {
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
#[doc = "Values that can be written to the field `REBL`"]
pub enum REBLW {
    #[doc = "1"]
    REBL_0,
    #[doc = "2"]
    REBL_1,
    #[doc = "3"]
    REBL_2,
    #[doc = "4"]
    REBL_3,
    #[doc = "5"]
    REBL_4,
    #[doc = "6"]
    REBL_5,
    #[doc = "7"]
    REBL_6,
    #[doc = "8"]
    REBL_7,
}
impl REBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REBLW::REBL_0 => 0,
            REBLW::REBL_1 => 1,
            REBLW::REBL_2 => 2,
            REBLW::REBL_3 => 3,
            REBLW::REBL_4 => 4,
            REBLW::REBL_5 => 5,
            REBLW::REBL_6 => 6,
            REBLW::REBL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REBLW<'a> {
    w: &'a mut W,
}
impl<'a> _REBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REBLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn rebl_0(self) -> &'a mut W {
        self.variant(REBLW::REBL_0)
    }
    #[doc = "2"]
    #[inline]
    pub fn rebl_1(self) -> &'a mut W {
        self.variant(REBLW::REBL_1)
    }
    #[doc = "3"]
    #[inline]
    pub fn rebl_2(self) -> &'a mut W {
        self.variant(REBLW::REBL_2)
    }
    #[doc = "4"]
    #[inline]
    pub fn rebl_3(self) -> &'a mut W {
        self.variant(REBLW::REBL_3)
    }
    #[doc = "5"]
    #[inline]
    pub fn rebl_4(self) -> &'a mut W {
        self.variant(REBLW::REBL_4)
    }
    #[doc = "6"]
    #[inline]
    pub fn rebl_5(self) -> &'a mut W {
        self.variant(REBLW::REBL_5)
    }
    #[doc = "7"]
    #[inline]
    pub fn rebl_6(self) -> &'a mut W {
        self.variant(REBLW::REBL_6)
    }
    #[doc = "8"]
    #[inline]
    pub fn rebl_7(self) -> &'a mut W {
        self.variant(REBLW::REBL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALE`"]
pub enum PRESCALEW {
    #[doc = "256*16 cycle"]
    PRESCALE_0,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_1,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_2,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_3,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_4,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_5,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_6,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_7,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_8,
    #[doc = "PRESCALE*16 cycle"]
    PRESCALE_9,
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
            PRESCALEW::PRESCALE_8 => 8,
            PRESCALEW::PRESCALE_9 => 9,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "256*16 cycle"]
    #[inline]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_0)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_1)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_2)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_3(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_3)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_4(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_4)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_5(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_5)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_6(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_6)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_7(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_7)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_8)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline]
    pub fn prescale_9(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RT`"]
pub enum RTW {
    #[doc = "256*Prescaler period"]
    RT_0,
    #[doc = "RT*Prescaler period"]
    RT_1,
    #[doc = "RT*Prescaler period"]
    RT_2,
    #[doc = "RT*Prescaler period"]
    RT_3,
    #[doc = "RT*Prescaler period"]
    RT_4,
    #[doc = "RT*Prescaler period"]
    RT_5,
    #[doc = "RT*Prescaler period"]
    RT_6,
    #[doc = "RT*Prescaler period"]
    RT_7,
    #[doc = "RT*Prescaler period"]
    RT_8,
    #[doc = "RT*Prescaler period"]
    RT_9,
}
impl RTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTW::RT_0 => 0,
            RTW::RT_1 => 1,
            RTW::RT_2 => 2,
            RTW::RT_3 => 3,
            RTW::RT_4 => 4,
            RTW::RT_5 => 5,
            RTW::RT_6 => 6,
            RTW::RT_7 => 7,
            RTW::RT_8 => 8,
            RTW::RT_9 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "256*Prescaler period"]
    #[inline]
    pub fn rt_0(self) -> &'a mut W {
        self.variant(RTW::RT_0)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_1(self) -> &'a mut W {
        self.variant(RTW::RT_1)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_2(self) -> &'a mut W {
        self.variant(RTW::RT_2)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_3(self) -> &'a mut W {
        self.variant(RTW::RT_3)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_4(self) -> &'a mut W {
        self.variant(RTW::RT_4)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_5(self) -> &'a mut W {
        self.variant(RTW::RT_5)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_6(self) -> &'a mut W {
        self.variant(RTW::RT_6)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_7(self) -> &'a mut W {
        self.variant(RTW::RT_7)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_8(self) -> &'a mut W {
        self.variant(RTW::RT_8)
    }
    #[doc = "RT*Prescaler period"]
    #[inline]
    pub fn rt_9(self) -> &'a mut W {
        self.variant(RTW::RT_9)
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
#[doc = "Values that can be written to the field `UT`"]
pub enum UTW {
    #[doc = "256*Prescaler period"]
    UT_0,
    #[doc = "UT*Prescaler period"]
    UT_1,
    #[doc = "UT*Prescaler period"]
    UT_2,
    #[doc = "UT*Prescaler period"]
    UT_3,
    #[doc = "UT*Prescaler period"]
    UT_4,
    #[doc = "UT*Prescaler period"]
    UT_5,
    #[doc = "UT*Prescaler period"]
    UT_6,
    #[doc = "UT*Prescaler period"]
    UT_7,
    #[doc = "UT*Prescaler period"]
    UT_8,
    #[doc = "UT*Prescaler period"]
    UT_9,
}
impl UTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UTW::UT_0 => 0,
            UTW::UT_1 => 1,
            UTW::UT_2 => 2,
            UTW::UT_3 => 3,
            UTW::UT_4 => 4,
            UTW::UT_5 => 5,
            UTW::UT_6 => 6,
            UTW::UT_7 => 7,
            UTW::UT_8 => 8,
            UTW::UT_9 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UTW<'a> {
    w: &'a mut W,
}
impl<'a> _UTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "256*Prescaler period"]
    #[inline]
    pub fn ut_0(self) -> &'a mut W {
        self.variant(UTW::UT_0)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_1(self) -> &'a mut W {
        self.variant(UTW::UT_1)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_2(self) -> &'a mut W {
        self.variant(UTW::UT_2)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_3(self) -> &'a mut W {
        self.variant(UTW::UT_3)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_4(self) -> &'a mut W {
        self.variant(UTW::UT_4)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_5(self) -> &'a mut W {
        self.variant(UTW::UT_5)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_6(self) -> &'a mut W {
        self.variant(UTW::UT_6)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_7(self) -> &'a mut W {
        self.variant(UTW::UT_7)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_8(self) -> &'a mut W {
        self.variant(UTW::UT_8)
    }
    #[doc = "UT*Prescaler period"]
    #[inline]
    pub fn ut_9(self) -> &'a mut W {
        self.variant(UTW::UT_9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - Refresh enable"]
    #[inline]
    pub fn ren(&self) -> RENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RENR { bits }
    }
    #[doc = "Bits 1:3 - Refresh burst length"]
    #[inline]
    pub fn rebl(&self) -> REBLR {
        REBLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Prescaler timer period"]
    #[inline]
    pub fn prescale(&self) -> PRESCALER {
        PRESCALER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Refresh timer period"]
    #[inline]
    pub fn rt(&self) -> RTR {
        RTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Refresh urgent threshold"]
    #[inline]
    pub fn ut(&self) -> UTR {
        UTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1082163200 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Refresh enable"]
    #[inline]
    pub fn ren(&mut self) -> _RENW {
        _RENW { w: self }
    }
    #[doc = "Bits 1:3 - Refresh burst length"]
    #[inline]
    pub fn rebl(&mut self) -> _REBLW {
        _REBLW { w: self }
    }
    #[doc = "Bits 8:15 - Prescaler timer period"]
    #[inline]
    pub fn prescale(&mut self) -> _PRESCALEW {
        _PRESCALEW { w: self }
    }
    #[doc = "Bits 16:23 - Refresh timer period"]
    #[inline]
    pub fn rt(&mut self) -> _RTW {
        _RTW { w: self }
    }
    #[doc = "Bits 24:31 - Refresh urgent threshold"]
    #[inline]
    pub fn ut(&mut self) -> _UTW {
        _UTW { w: self }
    }
}
